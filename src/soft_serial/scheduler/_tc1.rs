use core::{any::Any, cell::Cell, cmp::Ordering, marker::PhantomData, ptr};
use arduino_hal::{clock, pac::{tc1::tccr1b, TC1}, DefaultClock};
use avr_device::interrupt::{self, CriticalSection, Mutex};
use crate::soft_serial::scheduler;

use super::*;

pub trait Prescaler { const CS: tccr1b::CS1_A; }

struct NoPrescaler { _private: () }
struct PrescaleBy8 { _private: () }
struct PrescaleBy64 { _private: () }
struct PrescaleBy256 { _private: () }
struct PrescaleBy1024 { _private: () }

impl Prescaler for NoPrescaler
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::DIRECT;
}
impl Prescaler for PrescaleBy8
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_8;
}
impl Prescaler for PrescaleBy64
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_64;
}
impl Prescaler for PrescaleBy256
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_256;
}
impl Prescaler for PrescaleBy1024
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_1024;
}

struct _TaskEntry<Task: SchedulerTask, Priority: Ord>
{
    pub priority: Priority,
    pub time: SchedulerCounterInt,
    pub task: Task,
}

impl<Task: SchedulerTask, Priority: Ord> PartialEq for _TaskEntry<Task, Priority>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.priority == other.priority && self.time == other.time
    }
}

impl<Task: SchedulerTask, Priority: Ord> Eq for _TaskEntry<Task, Priority> { }

impl<Task: SchedulerTask, Priority: Ord> PartialOrd for _TaskEntry<Task, Priority>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl<Task: SchedulerTask, Priority: Ord> Ord for _TaskEntry<Task, Priority>
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        match self.priority.cmp(&other.priority)
        {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }

        match self.time.cmp(&other.time)
        {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }

        return Ordering::Equal;
    }
}

pub struct Scheduler<
    Task: SchedulerTask,
    const CAPACITY: usize,
    Priority: Ord,
    Clock: clock::Clock = DefaultClock,
    Prescaler: self::Prescaler = PrescaleBy64>
{
    _tc1: TC1,
    _tasks: heapless::BinaryHeap<
        _TaskEntry<Task, Priority>,
        heapless::binary_heap::Min,
        CAPACITY>,
    _cycle_count_at_last_overflow: u64,
    _clock: PhantomData<Clock>,
    _prescaler: PhantomData<Prescaler>,
}

impl<
    Task: SchedulerTask,
    const CAPACITY: usize,
    Priority: Ord,
    Clock: clock::Clock,
    Prescaler: self::Prescaler> Scheduler<
        Task,
        CAPACITY,
        Priority,
        Clock,
        Prescaler>
{
    pub fn new(tc1: TC1) -> Self
    {
        Self
        {
            _tc1: tc1,
            _tasks: heapless::BinaryHeap::new(),
            _clock: PhantomData,
            _prescaler: PhantomData,
            _cycle_count_at_last_overflow: 0,
        }
    }

    pub fn schedule_task_millis(
        &mut self,
        priority: Priority,
        delay_in_milliseconds: u64,
        task: Task) -> Result<(), Task>
    {
        let tcnt = self._tc1.tcnt1.read().bits();
        let time = interrupt::free(|_|
            self._cycle_count_at_last_overflow
            + tcnt as u64
            + ((Clock::FREQ as u64 * delay_in_milliseconds) / 1000));

        self.schedule_task_absolute(priority, time, task)
    }

    pub fn schedule_task(
        &mut self,
        priority: Priority,
        delay_in_cycles: u64,
        task: Task) -> Result<(), Task>
    {
        let tcnt = self._tc1.tcnt1.read().bits();
        let time = interrupt::free(|_|
            self._cycle_count_at_last_overflow
            + tcnt as u64
            + delay_in_cycles);

        self.schedule_task_absolute(priority, time, task)
    }

    pub fn schedule_task_absolute(
        &mut self,
        priority: Priority,
        cycles_since_init: u64,
        task: Task) -> Result<(), Task>
    {
        match self._tasks.push(_TaskEntry
        {
            priority,
            time: cycles_since_init,
            task,
        })
        {
            Ok(()) =>
            {
                self._tc1.timsk1.write(|w| w
                    .ocie1a().set_bit());
                self._tc1.tccr1a.write(|w| w
                    .wgm1().bits(0b__00));
                self._tc1.tccr1b.write(|w| w
                    .wgm1().bits(0b01__)
                    .cs1().variant(Prescaler::CS));

                Ok(())
            },
            Err(entry) => Err(entry.task),
        }
    }
}

impl<
    Task: SchedulerTask,
    const CAPACITY: usize,
    Priority: Ord,
    Clock: clock::Clock,
    Prescaler: self::Prescaler> TCSchedulerOps for Scheduler<
        Task,
        CAPACITY,
        Priority,
        Clock,
        Prescaler>
{
    fn timer_compa(&mut self)
    {
        self._tc1.timsk1.write(|w| w
            .ocie1a().clear_bit()
            .toie1().set_bit());

        loop
        {
            let Some(entry) = self._tasks.peek() else { break };

            let tcnt = self._tc1.tcnt1.read().bits();
            let cycles_since_init = interrupt::free(|_|
                self._cycle_count_at_last_overflow + tcnt as u64);
            if entry.time > cycles_since_init
            { break }

            let Some(entry) = self._tasks.pop() else { unreachable!() };

            entry.task.call(SchedulerTaskContext
            {
                cycles_since_init
            });
        }

        self._tc1.timsk1.write(|w| w
            .ocie1a().set_bit()
            .toie1().set_bit());
    }

    fn timer_ovf(&mut self)
    {
        self._cycle_count_at_last_overflow += 0x10000;
    }

    fn on_activate(&mut self)
    {
        self._tc1.tcnt1.write(|w| w.bits(0));
        self._tc1.tccr1a.write(|w| w
            .wgm1().bits(0b__00));
        self._tc1.tccr1b.write(|w| w
            .wgm1().bits(0b01__)
            .cs1().variant(Prescaler::CS));
        self._tc1.timsk1.write(|w| w
            .ocie1a().clear_bit()
            .toie1().set_bit());
    }

    fn on_deactivate(&mut self)
    {
        let tcnt = self._tc1.tcnt1.read().bits();
        self._cycle_count_at_last_overflow += tcnt as u64;
        self._tc1.tccr1b.write(|w| w
            .wgm1().bits(0b01__)
            .cs1().variant(tccr1b::CS1_A::NO_CLOCK));
        self._tc1.timsk1.write(|w| w
            .ocie1a().clear_bit()
            .toie1().clear_bit());
    }
}

pub fn set_active_scheduler<T: TCSchedulerOps>(scheduler: &'static mut T)
    -> Result<&'static mut T, &'static mut T>
{
    interrupt::free(|_| unsafe
    {
        match &mut _SCHEDULER
        {
            Some(_) => Err(scheduler),
            None =>
            {
                scheduler.on_activate();
                _SCHEDULER = Some(scheduler);

                // SAFETY:
                // This pattern was copied from an impl of `core::Any` so that
                // means its safe right? ...right?
                Ok(&mut *(
                    *_SCHEDULER.as_mut().unwrap()
                    as *mut dyn TCSchedulerOps
                    as *mut T))
            },
        }
    })
}

pub fn is_scheduler_active<T: TCSchedulerOps>(scheduler: &'static T) -> bool
{
    interrupt::free(|_| unsafe
    {
        let Some(existing) = &mut _SCHEDULER else { return false };

        ptr::eq(*existing, scheduler)
    })
}

pub fn get_active_scheduler()
    -> Result<Mutex<Cell<&'static mut dyn TCSchedulerOps>>, ()>
{
    interrupt::free(|_| unsafe
    {
        match &mut _SCHEDULER
        {
            // SAFETY:
            // Copies the reference, not the value. This is ok since Mutex
            // enforces only reading from one place.
            Some(scheduler) => Ok(Mutex::new(Cell::new(ptr::read(scheduler)))),
            None => Err(()),
        }
    })
}

static mut _SCHEDULER: Option<&'static mut dyn TCSchedulerOps> = None;

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_OVF()
{
    let Some(scheduler) = &mut _SCHEDULER else { return };
    scheduler.timer_ovf();
}

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_COMPA()
{
    let Some(scheduler) = &mut _SCHEDULER else { return };
    scheduler.timer_compa();
}