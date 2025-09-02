use arduino_hal::pac;
use avr_device::interrupt::CriticalSection;

use super::{PinPortID, Scheduler, SchedulerAllocationOps, SchedulerTaskContext};

pub(super) struct _SchedulerState
{
    pub exint: pac::EXINT,
    pub portb: pac::PORTB,
    pub last_pinb: u8,
    pub portc: pac::PORTC,
    pub last_pinc: u8,
    pub portd: pac::PORTD,
    pub last_pind: u8,
    pub allocation: &'static mut dyn SchedulerAllocationOps,
}

pub(super) static mut _SCHEDULER: Option<_SchedulerState> = None;

#[avr_device::interrupt(atmega328p)]
unsafe fn PCINT0()
{
    let Some(scheduler) = &mut _SCHEDULER else { return };

    let current_value = scheduler.portb.pinb.read().bits();

    for i in (0..scheduler.allocation.len()).rev()
    {
        let (pin, _) = scheduler.allocation.index(i);
        let index: u8 = match pin
        {
            PinPortID::PB0 => 0,
            PinPortID::PB1 => 1,
            PinPortID::PB2 => 2,
            PinPortID::PB3 => 3,
            PinPortID::PB4 => 4,
            PinPortID::PB5 => 5,
            PinPortID::PB6 => 6,
            PinPortID::PB7 => 7,
            _ => continue,
        };

        if ((scheduler.last_pinb >> index) & 1)
            != ((current_value >> index) & 1)
        {
            let (_, task) = scheduler.allocation.remove(i);

            task(SchedulerTaskContext
            {
                scheduler: &Scheduler { _private: () },
                cs: CriticalSection::new(),
                pin_is_high: (current_value >> index) != 0,
            });
        }
    }

    scheduler.last_pinb = current_value;
}

#[avr_device::interrupt(atmega328p)]
unsafe fn PCINT1()
{
    let Some(scheduler) = &mut _SCHEDULER else { return };

    let current_value = scheduler.portc.pinc.read().bits();

    for i in (0..scheduler.allocation.len()).rev()
    {
        let (pin, _) = scheduler.allocation.index(i);
        let index: u8 = match pin
        {
            PinPortID::PC0 => 0,
            PinPortID::PC1 => 1,
            PinPortID::PC2 => 2,
            PinPortID::PC3 => 3,
            PinPortID::PC4 => 4,
            PinPortID::PC5 => 5,
            PinPortID::PC6 => 6,
            _ => continue,
        };

        if ((scheduler.last_pinc >> index) & 1)
            != ((current_value >> index) & 1)
        {
            let (_, task) = scheduler.allocation.remove(i);

            task(SchedulerTaskContext
            {
                scheduler: &Scheduler { _private: () },
                cs: CriticalSection::new(),
                pin_is_high: (current_value >> index) != 0,
            });
        }
    }

    scheduler.last_pinc = current_value;
}

#[avr_device::interrupt(atmega328p)]
unsafe fn PCINT2()
{
    let Some(scheduler) = &mut _SCHEDULER else { return };

    let current_value = scheduler.portd.pind.read().bits();

    for i in (0..scheduler.allocation.len()).rev()
    {
        let (pin, _) = scheduler.allocation.index(i);
        let index: u8 = match pin
        {
            PinPortID::PD0 => 0,
            PinPortID::PD1 => 1,
            PinPortID::PD2 => 2,
            PinPortID::PD3 => 3,
            PinPortID::PD4 => 4,
            PinPortID::PD5 => 5,
            PinPortID::PD6 => 6,
            PinPortID::PD7 => 7,
            _ => continue,
        };

        if ((scheduler.last_pind >> index) & 1)
            != ((current_value >> index) & 1)
        {
            let (_, task) = scheduler.allocation.remove(i);

            task(SchedulerTaskContext
            {
                scheduler: &Scheduler { _private: () },
                cs: CriticalSection::new(),
                pin_is_high: (current_value >> index) != 0,
            });
        }
    }

    scheduler.last_pind = current_value;
}