use arduino_hal::{clock::Clock, pac::{tc1::tccr1b::CS1_A, TC1}, simple_pwm::Prescaler, DefaultClock};
use avr_device::interrupt::{self, CriticalSection};
use core::{cmp::Ordering, num::NonZero};

pub struct SchedulerTaskContext<'cs>
{
    pub scheduler: &'cs Scheduler,
    pub cs: CriticalSection<'cs>,
    pub cycles_since_init: u64,
}

pub struct SchedulerAllocation<const TASK_CAPACITY: usize>
{
    _tasks: heapless::BinaryHeap<
        _TaskEntry,
        heapless::binary_heap::Max,
        TASK_CAPACITY>,
}

impl<const TASK_CAPACITY: usize> SchedulerAllocation<TASK_CAPACITY>
{
    pub const fn new() -> Self { Self { _tasks: heapless::BinaryHeap::new() } }
}

impl<const TASK_CAPACITY: usize> Default for SchedulerAllocation<TASK_CAPACITY>
{
    fn default() -> Self { Self::new() }
}

pub trait SchedulerAllocationOps
{
    fn schedule_task_absolute(
        &mut self,
        priority: u8,
        cycles_after_init: u64,
        task: fn(SchedulerTaskContext))
        -> Result<(), fn(SchedulerTaskContext)>;

    fn next(&mut self, current_time: u64)
        -> Option<fn(SchedulerTaskContext)>;

    fn next_task_time(&self) -> Option<u64>;
}

impl<const TASK_CAPACITY: usize>
    SchedulerAllocationOps for SchedulerAllocation<TASK_CAPACITY>
{
    fn schedule_task_absolute(
        &mut self,
        priority: u8,
        cycles_after_init: u64,
        task: fn(SchedulerTaskContext))
        -> Result<(), fn(SchedulerTaskContext)>
    {
        match self._tasks.push(_TaskEntry { priority, cycles_after_init, task })
        {
            Ok(_) => Ok(()),
            Err(entry) => Err(entry.task),
        }
    }

    fn next(&mut self, current_time: u64) -> Option<fn(SchedulerTaskContext)>
    {
        let Some(time) = self.next_task_time() else { return None };

        if time > current_time { return None };

        let entry = unsafe { self._tasks.pop_unchecked() };

        return Some(entry.task);
    }

    fn next_task_time(&self) -> Option<u64>
    {
        let Some(entry) = self._tasks.peek() else { return None };

        return Some(entry.cycles_after_init);
    }

}

struct _TaskEntry
{
    pub priority: u8,
    pub cycles_after_init: u64,
    pub task: fn(SchedulerTaskContext),
}

impl PartialEq for _TaskEntry
{
    fn eq(&self, other: &Self) -> bool
    {
        self.priority == other.priority && self.cycles_after_init == other.cycles_after_init
    }
}

impl Eq for _TaskEntry { }

impl PartialOrd for _TaskEntry
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for _TaskEntry
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        match self.priority.cmp(&other.priority)
        {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }

        match self.cycles_after_init.cmp(&other.cycles_after_init)
        {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }

        return Ordering::Equal;
    }
}

pub struct Scheduler { _private: () }

impl Scheduler
{
    pub fn init(tc: TC1, allocation: &'static mut dyn SchedulerAllocationOps)
        -> Result<Scheduler, (TC1, &'static mut dyn SchedulerAllocationOps)>
    {
        unsafe
        {
            match _SCHEDULER
            {
                None =>
                {
                    tc.tcnt1.write(|w| w.bits(0));
                    tc.tccr1a.write(|w| w
                        .wgm1().bits(0b__00));
                    tc.tccr1b.write(|w| w
                        .wgm1().bits(0b01__)
                        .cs1().variant(match _PRESCALER
                        {
                            Prescaler::Direct => CS1_A::DIRECT,
                            Prescaler::Prescale8 => CS1_A::PRESCALE_8,
                            Prescaler::Prescale64 => CS1_A::PRESCALE_64,
                            Prescaler::Prescale256 => CS1_A::PRESCALE_256,
                            Prescaler::Prescale1024 => CS1_A::PRESCALE_1024,
                        }));
                    tc.timsk1.write(|w| w
                        .ocie1a().set_bit()
                        .toie1().clear_bit());
                    tc.tifr1.write(|w| w
                        .ocf1a().set_bit());

                    _SCHEDULER = Some(_SchedulerState
                    {
                        _tc: tc,
                        _cycle_counter: 0,
                        _allocation: allocation,
                    });

                    interrupt::enable();

                    Ok(Scheduler { _private: () })
                },
                Some(_) => Err((tc, allocation)),
            }
        }
    }

    pub fn schedule_task_millis(
        &self,
        priority: u8,
        millisecond_delay: u64,
        task: fn(SchedulerTaskContext))
        -> Result<(), fn(SchedulerTaskContext)>
    {
        interrupt::free(|_| unsafe
        {
            let Some(scheduler) = &mut _SCHEDULER
            else
            {
                unreachable!(
                    "TC1 Scheduler was initiated in an illegal way.");
            };

            self.schedule_task_absolute(
                priority,
                scheduler._cycle_counter
                    + scheduler._tc.tcnt1.read().bits() as u64
                    + ((DefaultClock::FREQ as u64
                        * millisecond_delay)
                        / (1000 * match _PRESCALER
                        {
                            Prescaler::Direct => 1,
                            Prescaler::Prescale8 => 8,
                            Prescaler::Prescale64 => 64,
                            Prescaler::Prescale256 => 256,
                            Prescaler::Prescale1024 => 1024,
                        })),
                task)
        })
    }

    pub fn schedule_task_absolute(
        &self,
        priority: u8,
        cycles_after_init: u64,
        task: fn(SchedulerTaskContext))
        -> Result<(), fn(SchedulerTaskContext)>
    {
        interrupt::free(|_| unsafe
        {
            let Some(scheduler) = &mut _SCHEDULER
            else
            {
                unreachable!(
                    "TC1 Scheduler was initiated in an illegal way.");
            };

            scheduler._allocation.schedule_task_absolute(
                priority,
                cycles_after_init,
                task)?;

            match cycles_after_init.checked_sub(scheduler._cycle_counter)
            {
                None =>
                {
                    scheduler._tc.timsk1.write(|w| w
                        .ocie1a().set_bit());
                    scheduler._tc.tifr1.write(|w| w
                        .ocf1a().set_bit());
                },
                Some(0x10000..=u64::MAX) => (),
                Some(delta_cycle_count) =>
                {
                    scheduler._tc.ocr1a.modify(|r, w|
                    {
                        scheduler._tc.tifr1.write(|w| w
                            .ocf1a().bit(false));

                        let before = r.bits();

                        if delta_cycle_count as u16 > before { return w };

                        let after = delta_cycle_count as u16;
                        w.bits(after);

                        let tcnt = scheduler._tc.tcnt1.read().bits();
                        scheduler._tc.tifr1.write(|w| w
                            .ocf1a().bit(
                                if before < after
                                { before < tcnt && tcnt < after }
                                else
                                { after >= tcnt || tcnt >= before }));

                        w
                    });
                },
            }

            Ok(())
        })
    }
}

struct _SchedulerState
{
    _tc: TC1,
    _cycle_counter: u64,
    _allocation: &'static mut dyn SchedulerAllocationOps,
}

const _PRESCALER: Prescaler = Prescaler::Prescale8;
static mut _SCHEDULER: Option<_SchedulerState> = None;

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_COMPA()
{
    let Some(scheduler) = &mut _SCHEDULER else { return };

    let before_orc1a = scheduler._tc.ocr1a.read().bits();
    scheduler._cycle_counter += before_orc1a as u64;

    loop
    {
        let Some(task) = scheduler._allocation.next(scheduler._cycle_counter)
        else { break };

        task(SchedulerTaskContext
        {
            scheduler: &Scheduler { _private: () },
            cs: CriticalSection::new(),
            cycles_since_init: scheduler._cycle_counter,
        });
    }

    match scheduler._allocation.next_task_time()
        .map(|x| x.checked_sub(scheduler._cycle_counter))
    {
        None | Some(Some(0x10000..=u64::MAX)) =>
        {
            scheduler._tc.ocr1a.write(|w| w
                .bits(0xFFFF));
            scheduler._tc.tifr1.write(|w| w
                .ocf1a().clear_bit());
        },
        Some(None) =>
        {
            scheduler._tc.ocr1a.write(|w| w
                .bits(0xFFFF));
            scheduler._tc.tifr1.write(|w| w
                .ocf1a().set_bit());
        },
        Some(Some(delta_cycle_count)) =>
        {
            scheduler._tc.ocr1a.write(|w| w
                .bits(delta_cycle_count as u16));

            if scheduler._tc.tcnt1.read().bits() < delta_cycle_count as u16
            {
                scheduler._tc.tifr1.write(|w| w
                    .ocf1a().set_bit());
            }
        },
    }
}