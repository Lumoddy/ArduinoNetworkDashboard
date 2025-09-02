use arduino_hal::hal::port;
use arduino_hal::{pac, Peripherals};
use arduino_hal::pac::exint::eicra::{ISC0_A, ISC1_A};
use arduino_hal::port::{mode, Pin};
use avr_device::interrupt::{self, CriticalSection};

use crate::panic_payload;

use super::{SchedulerAllocationOps, _SchedulerState, _SCHEDULER};

pub struct SchedulerTaskContext<'cs>
{
    pub scheduler: &'cs Scheduler,
    pub cs: CriticalSection<'cs>,
    pub pin_is_high: bool,
}

pub trait StaticIntoPinID
{
    fn id() -> PinPortID;
}

pub trait IntoPinID
{
    fn id(&self) -> PinPortID;
}

macro_rules! pin_port_id_impl
{
    (
        $(#[$pins_attr:meta])*
        $vis:vis enum $name:ident
        {
            $($variant:ident),+ $(,)?
        }
    ) =>
    {
        $(#[$pins_attr])*
        $vis enum $name
        {
            $($variant),+
        }

        $(
            impl StaticIntoPinID for port::$variant
            {
                fn id() -> $name { $name::$variant }
            }
        )+
    };
}

pin_port_id_impl!
{
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PinPortID
    {
        PB0, PB1, PB2, PB3, PB4, PB5, PB6, PB7,
        PC0, PC1, PC2, PC3, PC4, PC5, PC6,
        PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7,
    }
}

impl<MODE: mode::Io, PIN: StaticIntoPinID> IntoPinID for Pin<MODE, PIN>
{
    fn id(&self) -> PinPortID { PIN::id() }
}

pub struct SchedulerConfig<
    Allocation: 'static + ?Sized + SchedulerAllocationOps
        = dyn 'static + SchedulerAllocationOps>
{
    pub exint: pac::EXINT,
    // pub portb: pac::PORTB, // Used by arduino_hal::pins!(), it would just be
    // pub portc: pac::PORTC, // inconvenient.
    // pub portd: pac::PORTD,
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
                    config.exint.eicra.write(|w| w
                        .isc0().variant(ISC0_A::VAL_0X01)
                        .isc1().variant(ISC1_A::VAL_0X01));
                    config.exint.eimsk.write(|w| w
                        .int0().set_bit()
                        .int1().set_bit());
                    config.exint.pcicr.write(|w| w
                        .pcie().bits(0b111));
                    config.exint.pcmsk0.write(|w| w
                        .pcint().bits(0));
                    config.exint.pcmsk1.write(|w| w
                        .pcint().bits(0));
                    config.exint.pcmsk2.write(|w| w
                        .pcint().bits(0));

                    let dp = Peripherals::steal();

                    let portb = dp.PORTB;
                    let portc = dp.PORTC;
                    let portd = dp.PORTD;

                    _SCHEDULER = Some(_SchedulerState
                    {
                        exint: config.exint,
                        last_pinb: portb.pinb.read().bits(),
                        portb,
                        last_pinc: portc.pinc.read().bits(),
                        portc,
                        last_pind: portd.pind.read().bits(),
                        portd,
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
                    str: b"EXINT Scheduler was initiated in an illegal way.");
            };

            scheduler.allocation.can_schedule_task()
        })
    }

    pub fn schedule_task(
        &self,
        pin: PinPortID,
        task: fn(SchedulerTaskContext)) -> Result<(), ()>
    {
        interrupt::free(|_| unsafe
        {
            let Some(scheduler) = &mut _SCHEDULER
            else
            {
                panic_payload!(
                    str: b"EXINT Scheduler was initiated in an illegal way.");
            };

            scheduler.allocation.schedule_task(pin, task)?;

            match pin
            {
                PinPortID::PB0
                | PinPortID::PB1
                | PinPortID::PB2
                | PinPortID::PB3
                | PinPortID::PB4
                | PinPortID::PB5
                | PinPortID::PB6
                | PinPortID::PB7 => scheduler.exint.pcmsk0.modify(|r, w| w
                    .pcint().bits(r.pcint().bits()
                        | (1 << match pin
                        {
                            PinPortID::PB0 => 0,
                            PinPortID::PB1 => 1,
                            PinPortID::PB2 => 2,
                            PinPortID::PB3 => 3,
                            PinPortID::PB4 => 4,
                            PinPortID::PB5 => 5,
                            PinPortID::PB6 => 6,
                            PinPortID::PB7 => 7,
                            _ => unreachable!(),
                        }))),

                PinPortID::PC0
                | PinPortID::PC1
                | PinPortID::PC2
                | PinPortID::PC3
                | PinPortID::PC4
                | PinPortID::PC5
                | PinPortID::PC6 => scheduler.exint.pcmsk1.modify(|r, w| w
                    .pcint().bits(r.pcint().bits()
                        | (1 << match pin
                        {
                            PinPortID::PC0 => 0,
                            PinPortID::PC1 => 1,
                            PinPortID::PC2 => 2,
                            PinPortID::PC3 => 3,
                            PinPortID::PC4 => 4,
                            PinPortID::PC5 => 5,
                            PinPortID::PC6 => 6,
                            _ => unreachable!(),
                        }))),

                PinPortID::PD0
                | PinPortID::PD1
                | PinPortID::PD2
                | PinPortID::PD3
                | PinPortID::PD4
                | PinPortID::PD5
                | PinPortID::PD6
                | PinPortID::PD7 => scheduler.exint.pcmsk2.modify(|r, w| w
                    .pcint().bits(r.pcint().bits()
                        | (1 << match pin
                        {
                            PinPortID::PD0 => 0,
                            PinPortID::PD1 => 1,
                            PinPortID::PD2 => 2,
                            PinPortID::PD3 => 3,
                            PinPortID::PD4 => 4,
                            PinPortID::PD5 => 5,
                            PinPortID::PD6 => 6,
                            PinPortID::PD7 => 7,
                            _ => unreachable!(),
                        }))),
            }

            Ok(())
        })
    }
}