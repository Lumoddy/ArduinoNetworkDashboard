use arduino_hal::pac::{EXINT, TC1};
use arduino_hal::pac::tc1::tccr1b::CS1_A;
use avr_device::interrupt::{self, CriticalSection};

use crate::soft_serial::input::SerialInputState;
use crate::soft_serial::state::{push_event, DynamicSerialState};
use crate::unreachable_payload;

use super::input::SerialInputError;
use super::state::{Event, EventEntry, EVENT_QUEUE, SERIAL_STATES};
use super::ucycles;

pub const PRESCALER: CS1_A = CS1_A::PRESCALE_8;

pub static mut CYCLE_COUNTER: ucycles = 0;

pub fn init_service(timer: TC1, exint: EXINT)
{
    _ = timer;
    _ = exint;

    static mut ALREADY_INIT: bool = false;

    unsafe
    {
        if ALREADY_INIT { return };

        let dp = arduino_hal::Peripherals::steal();

        interrupt::disable();

        dp.EXINT.pcmsk0.write(|w| w
            .pcint().bits(0));
        dp.EXINT.pcmsk1.write(|w| w
            .pcint().bits(0));
        dp.EXINT.pcmsk2.write(|w| w
            .pcint().bits(0));
        dp.EXINT.pcicr.write(|w| w
            .pcie().bits(0b111));

        dp.TC1.timsk1.write(|w| w
            .ocie1a().set_bit());
        dp.TC1.tccr1a.write(|w| w
            .wgm1().bits(0b__00));
        dp.TC1.tccr1b.write(|w| w
            .wgm1().bits(0b01__)
            .cs1().variant(PRESCALER));
        dp.TC1.ocr1a.write(|w| w
            .bits(0xFFFF));
        dp.TC1.tifr1.write(|w| w
            .ocf1a().set_bit());

        ALREADY_INIT = true;

        interrupt::enable();
    }
}

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_COMPA()
{
    let dp = arduino_hal::Peripherals::steal();

    let elapsed = dp.TC1.ocr1a.read().bits();
    CYCLE_COUNTER = CYCLE_COUNTER.wrapping_add(elapsed as ucycles);

    match EVENT_QUEUE.peek()
    {
        None =>
        {
            dp.TC1.ocr1a.write(|w| w
                .bits(0xFFFF));
            dp.TC1.tifr1.write(|w| w
                .ocf1a().set_bit());
        },
        Some(entry) =>
        {
            match entry.deadline.checked_sub(CYCLE_COUNTER)
            {
                Some(0xFFFF..=ucycles::MAX) =>
                {
                    dp.TC1.ocr1a.write(|w| w
                        .bits(0xFFFF));
                    dp.TC1.tifr1.write(|w| w
                        .ocf1a().set_bit());
                },
                None | Some(0) =>
                {
                    EVENT_QUEUE.pop_unchecked().run(CriticalSection::new());

                    dp.TC1.ocr1a.write(|w| w
                        .bits(0));
                    dp.TC1.tifr1.write(|w|
                        w.ocf1a().clear_bit());
                }
                Some(cycles_until_deadline) =>
                {
                    let cycles_until_deadline = cycles_until_deadline as u16;

                    dp.TC1.ocr1a.write(|w| w
                        .bits(cycles_until_deadline));

                    if dp.TC1.tcnt1.read().bits() >= cycles_until_deadline
                    {
                        dp.TC1.tifr1.write(|w|
                            w.ocf1a().clear_bit());
                    }
                    else
                    {
                        dp.TC1.tifr1.write(|w|
                            w.ocf1a().set_bit());
                    }
                },
            }
        },
    }
}

#[avr_device::interrupt(atmega328p)]
unsafe fn PCINT0() { _pcint() }

#[avr_device::interrupt(atmega328p)]
unsafe fn PCINT1() { _pcint() }

#[avr_device::interrupt(atmega328p)]
unsafe fn PCINT2() { _pcint() }

unsafe fn _pcint()
{
    let dp = arduino_hal::Peripherals::steal();

    let tcnt = dp.TC1.tcnt1.read().bits() as ucycles;

    for state in &mut SERIAL_STATES
    {
        let target = state as *mut _;
        match state
        {
            Some(DynamicSerialState::Input(reader)) =>
            {
                let pin_is_one = reader.pin_is_one();

                if matches!(
                    (&reader.state, pin_is_one, reader.pin_was_one),
                    (SerialInputState::Idle, true, false))
                {
                    let Ok(()) = reader.buffer.push_front(b' ')
                    else { unreachable_payload!() };
                    let Ok(()) = reader.buffer.push_front(b'^')
                    else { unreachable_payload!() };

                    reader.state = SerialInputState::Active;

                    let Ok(()) = push_event(EventEntry
                    {
                        deadline:
                            CYCLE_COUNTER
                            + tcnt
                            + (reader.baud_cycles / 2),
                        event: Event::InputStart { target },
                    })
                    else
                    {
                        reader.state = SerialInputState::Err(
                            SerialInputError::TooManyParallel);
                        continue;
                    };
                }

                reader.pin_was_one = pin_is_one;
            },
            _ => (),
        }
    }
}