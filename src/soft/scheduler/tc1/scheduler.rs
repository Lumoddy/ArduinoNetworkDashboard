use arduino_hal::clock::Clock;
use arduino_hal::{pac, DefaultClock};
use arduino_hal::pac::tc1::tccr1b::CS1_A;
use arduino_hal::simple_pwm::Prescaler;
use avr_device::interrupt::{self, CriticalSection};

use crate::panic_payload;

use super::{SchedulerAllocationOps, _SchedulerState, PRESCALER, _SCHEDULER};

pub struct SchedulerTaskContext<'cs>
{
    pub scheduler: &'cs Scheduler,
    pub cs: CriticalSection<'cs>,
    pub cycles_since_init: u64,
}

pub struct SchedulerConfig<
    Allocation: 'static + ?Sized + SchedulerAllocationOps
        = dyn 'static + SchedulerAllocationOps>
{
    pub tc: pac::TC1,
    pub allocation: &'static mut Allocation,
}

pub struct Scheduler { pub(super) _private: () }

impl Scheduler
{
    pub fn init<Allocation: 'static + SchedulerAllocationOps>(
        config: SchedulerConfig<Allocation>)
        -> Result<Scheduler, SchedulerConfig<Allocation>>
    {
        unsafe
        {
            match _SCHEDULER
            {
                None =>
                {
                    interrupt::disable();

                    config.tc.tcnt1.write(|w| w.bits(0));
                    config.tc.tccr1a.write(|w| w
                        .wgm1().bits(0b__00));
                    config.tc.tccr1b.write(|w| w
                        .wgm1().bits(0b01__)
                        .cs1().variant(match PRESCALER
                        {
                            Prescaler::Direct => CS1_A::DIRECT,
                            Prescaler::Prescale8 => CS1_A::PRESCALE_8,
                            Prescaler::Prescale64 => CS1_A::PRESCALE_64,
                            Prescaler::Prescale256 => CS1_A::PRESCALE_256,
                            Prescaler::Prescale1024 => CS1_A::PRESCALE_1024,
                        }));
                    config.tc.timsk1.write(|w| w
                        .ocie1a().set_bit()
                        .toie1().clear_bit());
                    config.tc.tifr1.write(|w| w
                        .ocf1a().set_bit());

                    _SCHEDULER = Some(_SchedulerState
                    {
                        tc: config.tc,
                        cycle_counter: 0,
                        allocation: config.allocation,
                    });

                    interrupt::enable();

                    Ok(Scheduler { _private: () })
                },
                Some(_) => Err(config),
            }
        }
    }

    pub unsafe fn steal() -> Result<Scheduler, ()>
    {
        unsafe
        {
            match _SCHEDULER
            {
                None => Err(()),
                Some(_) => Ok(Scheduler { _private: () }),
            }
        }
    }

    pub const unsafe fn steal_copy(_: &Scheduler) -> Scheduler
    {
        Scheduler { _private: () }
    }

    pub fn can_schedule_task(&self) -> bool
    {
        interrupt::free(|_| unsafe
        {
            let Some(scheduler) = &mut _SCHEDULER
            else
            {
                panic_payload!(
                    str: b"TC1 Scheduler was initiated in an illegal way.");
            };

            scheduler.allocation.can_schedule_task()
        })
    }

    pub fn schedule_task_millis(
        &self,
        priority: u8,
        millisecond_delay: u64,
        task: fn(SchedulerTaskContext)) -> Result<(), ()>
    {
        self.schedule_task_cycles(
            priority,
            (DefaultClock::FREQ as u64 * millisecond_delay)
                / (1000 * match PRESCALER
                {
                    Prescaler::Direct => 1,
                    Prescaler::Prescale8 => 8,
                    Prescaler::Prescale64 => 64,
                    Prescaler::Prescale256 => 256,
                    Prescaler::Prescale1024 => 1024,
                }),
            task)
    }

    pub fn schedule_task_cycles(
        &self,
        priority: u8,
        cycle_delay: u64,
        task: fn(SchedulerTaskContext)) -> Result<(), ()>
    {
        interrupt::free(|_| unsafe
        {
            let Some(scheduler) = &mut _SCHEDULER
            else
            {
                panic_payload!(
                    str: b"TC1 Scheduler was initiated in an illegal way.");
            };

            self.schedule_task_absolute(
                priority,
                scheduler.cycle_counter
                    + scheduler.tc.tcnt1.read().bits() as u64
                    + cycle_delay,
                task)
        })
    }

    pub fn schedule_task_absolute(
        &self,
        priority: u8,
        cycles_after_init: u64,
        task: fn(SchedulerTaskContext)) -> Result<(), ()>
    {
        interrupt::free(|_| unsafe
        {
            let Some(scheduler) = &mut _SCHEDULER
            else
            {
                panic_payload!(
                    str: b"TC1 Scheduler was initiated in an illegal way.");
            };

            scheduler.allocation.push_task(priority, cycles_after_init, task)?;

            match cycles_after_init.checked_sub(scheduler.cycle_counter)
            {
                None =>
                {
                    scheduler.tc.timsk1.write(|w| w
                        .ocie1a().set_bit());
                    scheduler.tc.tifr1.write(|w| w
                        .ocf1a().set_bit());
                },
                Some(0x10000..=u64::MAX) => (),
                Some(delta_cycle_count) =>
                {
                    scheduler.tc.ocr1a.modify(|r, w|
                    {
                        scheduler.tc.tifr1.write(|w| w
                            .ocf1a().bit(false));

                        let before = r.bits();

                        if delta_cycle_count as u16 > before { return w };

                        let after = delta_cycle_count as u16;
                        w.bits(after);

                        let tcnt = scheduler.tc.tcnt1.read().bits();
                        scheduler.tc.tifr1.write(|w| w
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