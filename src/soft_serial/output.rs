use core::ptr;

use arduino_hal::clock::Clock;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::pac::tc1::tccr1b::CS1_A;
use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::DefaultClock;
use avr_device::interrupt;
use nb::block;

use crate::soft_serial::state::DynamicSerialState;
use crate::unreachable_payload;

use super::interrupts::CYCLE_COUNTER;
use super::state::{push_event, Event, EventEntry, EVENT_QUEUE, SERIAL_STATES};
use super::{ucycles, SERIAL_BUFFER_CAPACITY};

pub struct SerialOutput<PIN: PinOps<Dynamic = Dynamic>>
{
    pub(super) ghost_pin: Pin<mode::Output, PIN>,
    pub(super) state_index: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SerialOutputError
{
    TooManyParallel,
}

pub struct SerialOutputConfig<PIN: PinOps<Dynamic = Dynamic>>
{
    pub pin: Pin<mode::Output, PIN>,
    pub baudrate: u32,
    pub inverse_signal: bool,
}

impl<PIN: PinOps<Dynamic = Dynamic>> SerialOutput<PIN>
{
    pub fn init(config: SerialOutputConfig<PIN>)
        -> Result<SerialOutput<PIN>, SerialOutputConfig<PIN>>
    {
        interrupt::free(|_| unsafe
        {
            for (i, data) in SERIAL_STATES.iter_mut().enumerate()
            {
                let None = data else { continue };

                let high_is_one = config.inverse_signal;

                let result = SerialOutput
                {
                    ghost_pin: ptr::read(&config.pin),
                    state_index: i as u8,
                };

                *data = Some(DynamicSerialState::Output(SerialOutputData
                {
                    pin: config.pin.downgrade(),
                    buffer: heapless::Deque::new(),
                    byte: 0,
                    baud_cycles:
                        DefaultClock::FREQ as u64
                        / (config.baudrate as u64
                        * match crate::soft_serial::interrupts::PRESCALER
                        {
                            CS1_A::DIRECT => 1,
                            CS1_A::PRESCALE_8 => 8,
                            CS1_A::PRESCALE_64 => 64,
                            CS1_A::PRESCALE_256 => 256,
                            CS1_A::PRESCALE_1024 => 1024,
                            _ => unreachable_payload!(),
                        }),
                    high_is_one,
                    state: SerialOutputState::Idle,
                }));

                return Ok(result);
            }

            Err(config)
        })
    }

    pub fn err(&self) -> Option<SerialOutputError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(DynamicSerialState::Output(data))
                = &mut SERIAL_STATES[self.state_index as usize]
            else { unreachable_payload!() };

            if let SerialOutputState::Err(error) = data.state
            { return Some(error) };

            None
        })
    }

    pub fn take_err(&mut self) -> Option<SerialOutputError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(DynamicSerialState::Output(data))
                = &mut SERIAL_STATES[self.state_index as usize]
            else { unreachable_payload!() };

            if let SerialOutputState::Err(error) = data.state
            {
                data.state = SerialOutputState::Idle;
                return Some(error);
            }

            None
        })
    }

    pub fn take_pin(self) -> Pin<mode::Output, PIN>
    {
        unsafe { ptr::read(&self.ghost_pin) }
    }

    pub fn write_byte(&mut self, byte: u8) -> nb::Result<(), SerialOutputError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(DynamicSerialState::Output(data))
                = &mut SERIAL_STATES[self.state_index as usize]
            else { unreachable_payload!() };

            match data.state
            {
                SerialOutputState::Err(error) => Err(nb::Error::Other(error)),
                SerialOutputState::Active =>
                {
                    let Ok(()) = data.buffer.push_front(byte) else
                    { return Err(nb::Error::WouldBlock) };

                    Ok(())
                },
                SerialOutputState::Idle =>
                {
                    data.byte = byte;

                    let dp = arduino_hal::Peripherals::steal();

                    data.state = SerialOutputState::Active;

                    let Ok(()) = push_event(EventEntry
                    {
                        deadline:
                            CYCLE_COUNTER
                            + dp.TC1.tcnt1.read().bits() as ucycles,
                        event: Event::OutputStart
                        {
                            target: &mut SERIAL_STATES[self.state_index as usize],
                        },
                    })
                    else
                    {
                        let error = SerialOutputError::TooManyParallel;
                        data.state = SerialOutputState::Err(error);
                        return Err(nb::Error::Other(error));
                    };

                    Ok(())
                },
            }
        })
    }
}

impl<PIN: PinOps<Dynamic = Dynamic>> Drop for SerialOutput<PIN>
{
    fn drop(&mut self)
    {
        interrupt::free(|_| unsafe
        {
            SERIAL_STATES[self.state_index as usize] = None;
        })
    }
}

impl<PIN: PinOps<Dynamic = Dynamic>> ufmt::uWrite for SerialOutput<PIN>
{
    type Error = SerialOutputError;

    fn write_str(&mut self, s: &str) -> Result<(), SerialOutputError>
    {
        for byte in s.as_bytes()
        {
            if let Err(error) = block!(self.write_byte(*byte))
            { return Err(error) };
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum SerialOutputState
{
    Idle,
    Active,
    Err(SerialOutputError),
}

pub(super) struct SerialOutputData
{
    pub pin: Pin<mode::Output, Dynamic>,
    pub buffer: heapless::Deque<u8, SERIAL_BUFFER_CAPACITY>,
    pub byte: u8,
    pub baud_cycles: u64,
    pub high_is_one: bool,
    pub state: SerialOutputState,
}

impl SerialOutputData
{
    pub(super) fn set_pin_is_one(&mut self, one: bool)
    {
        if self.high_is_one == one
        {
            self.pin.set_high();
        }
        else
        {
            self.pin.set_low();
        }
    }
}