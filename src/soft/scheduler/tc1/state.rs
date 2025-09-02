use arduino_hal::pac;
use avr_device::interrupt::CriticalSection;

use super::{Scheduler, SchedulerAllocationOps, SchedulerTaskContext};

pub(super) struct _SchedulerState
{
    pub tc: pac::TC1,
    pub cycle_counter: u64,
    pub allocation: &'static mut dyn SchedulerAllocationOps,
}

pub(super) static mut _SCHEDULER: Option<_SchedulerState> = None;

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_COMPA()
{
    let Some(scheduler) = &mut _SCHEDULER else { return };

    let before_orc1a = scheduler.tc.ocr1a.read().bits();
    scheduler.cycle_counter += before_orc1a as u64;

    loop
    {
        let Some(task) = scheduler.allocation.next(scheduler.cycle_counter)
        else { break };

        task(SchedulerTaskContext
        {
            scheduler: &Scheduler { _private: () },
            cs: CriticalSection::new(),
            cycles_since_init: scheduler.cycle_counter,
        });
    }

    match scheduler.allocation.next_task_time()
        .map(|x| x.checked_sub(scheduler.cycle_counter))
    {
        None | Some(Some(0x10000..=u64::MAX)) =>
        {
            scheduler.tc.ocr1a.write(|w| w
                .bits(0xFFFF));
            scheduler.tc.tifr1.write(|w| w
                .ocf1a().clear_bit());
        },
        Some(None) =>
        {
            scheduler.tc.ocr1a.write(|w| w
                .bits(0xFFFF));
            scheduler.tc.tifr1.write(|w| w
                .ocf1a().set_bit());
        },
        Some(Some(delta_cycle_count)) =>
        {
            scheduler.tc.ocr1a.write(|w| w
                .bits(delta_cycle_count as u16));

            if scheduler.tc.tcnt1.read().bits() < delta_cycle_count as u16
            {
                scheduler.tc.tifr1.write(|w| w
                    .ocf1a().set_bit());
            }
        },
    }
}