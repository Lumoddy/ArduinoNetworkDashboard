use core::ptr;

use arduino_hal::clock::Clock;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::pac::tc1::tccr1b::CS1_A;
use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::DefaultClock;
use avr_device::interrupt;

use crate::unreachable_payload;

use super::pcint::PCINTPin;
use super::state::{DynamicSerialState, SERIAL_STATES};
use super::SERIAL_BUFFER_CAPACITY;

pub struct SerialInput<PIN: PinOps<Dynamic = Dynamic> + PCINTPin>
{
    pub(super) ghost_pin: Pin<mode::Input<mode::PullUp>, PIN>,
    pub(super) state_index: u8,
}

#[derive(Clone, Copy)]
pub enum SerialInputError
{
    TooManyParallel,
    IncompleteData,
}

pub struct SerialInputConfig<PIN: PinOps<Dynamic = Dynamic> + PCINTPin>
{
    pub pin: Pin<mode::Input<mode::PullUp>, PIN>,
    pub baudrate: u32,
    pub inverse_signal: bool,
}

impl<PIN: PinOps<Dynamic = Dynamic> + PCINTPin> SerialInput<PIN>
{
    pub fn init(config: SerialInputConfig<PIN>)
        -> Result<SerialInput<PIN>, SerialInputConfig<PIN>>
    {
        interrupt::free(|_| unsafe
        {
            PIN::enable_int();

            for (i, data) in SERIAL_STATES.iter_mut().enumerate()
            {
                let None = data else { continue };

                let high_is_one = config.inverse_signal;

                let result = SerialInput
                {
                    ghost_pin: ptr::read(&config.pin),
                    state_index: i as u8,
                };

                *data = Some(DynamicSerialState::Input(SerialInputData
                {
                    pin_was_one: config.pin.is_high() == high_is_one,
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
                    state: SerialInputState::Idle,
                }));

                return Ok(result);
            }

            Err(config)
        })
    }

    pub fn err(&self) -> Option<SerialInputError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(DynamicSerialState::Input(data))
                = &mut SERIAL_STATES[self.state_index as usize]
            else { unreachable_payload!() };

            if let SerialInputState::Err(error) = data.state
            { return Some(error) };

            None
        })
    }

    pub fn take_err(&mut self) -> Option<SerialInputError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(DynamicSerialState::Input(data))
                = &mut SERIAL_STATES[self.state_index as usize]
            else { unreachable_payload!() };

            if let SerialInputState::Err(error) = data.state
            {
                data.state = SerialInputState::Idle;
                return Some(error);
            }

            None
        })
    }

    pub fn take_pin(self) -> Pin<mode::Input<mode::PullUp>, PIN>
    {
        unsafe { ptr::read(&self.ghost_pin) }
    }

    pub fn read_byte(&mut self) -> nb::Result<u8, SerialInputError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(DynamicSerialState::Input(data))
                = &mut SERIAL_STATES[self.state_index as usize]
            else { unreachable_payload!() };

            if let SerialInputState::Err(error) = data.state
            { return Err(nb::Error::Other(error)) };

            if let Some(byte) = data.buffer.pop_back()
            { return Ok(byte) };

            Err(nb::Error::WouldBlock)
        })
    }

    pub fn continuous_read(&mut self) -> SerialInputBlockingIter
    {
        interrupt::free(|_| unsafe
        {
            SerialInputBlockingIter
            {
                _state: &mut SERIAL_STATES[self.state_index as usize],
            }
        })
    }
}

impl<PIN: PinOps<Dynamic = Dynamic> + PCINTPin> Drop for SerialInput<PIN>
{
    fn drop(&mut self)
    {
        interrupt::free(|_| unsafe
        {
            PIN::disable_int();

            SERIAL_STATES[self.state_index as usize] = None;
        })
    }
}

pub struct SerialInputBlockingIter
{
    _state: &'static mut Option<DynamicSerialState>,
}

impl Iterator for SerialInputBlockingIter
{
    type Item = u8;

    fn next(&mut self) -> Option<u8>
    {
        loop
        {
            let irq_flag = interrupt::disable_save();

            let Some(DynamicSerialState::Input(data))
                = core::hint::black_box::<&mut _>(self._state)
            else
            {
                unsafe { interrupt::restore(irq_flag) };
                return None;
            };

            if let Some(byte) = data.buffer.pop_back()
            {
                unsafe { interrupt::restore(irq_flag) };
                return Some(byte)
            }

            if let SerialInputState::Idle | SerialInputState::Err(_)
                = data.state
            {
                unsafe { interrupt::restore(irq_flag) };
                return None;
            }

            unsafe { interrupt::restore(irq_flag) };
        }
    }
}

pub(super) enum SerialInputState
{
    Idle,
    Active,
    Err(SerialInputError),
}

pub(super) struct SerialInputData
{
    pub pin: Pin<mode::Input<mode::PullUp>, Dynamic>,
    pub pin_was_one: bool,
    pub buffer: heapless::Deque<u8, SERIAL_BUFFER_CAPACITY>,
    pub byte: u8,
    pub baud_cycles: u64,
    pub high_is_one: bool,
    pub state: SerialInputState,
}

impl SerialInputData
{
    pub(super) fn pin_is_one(&self) -> bool
    {
        self.pin.is_high() == self.high_is_one
    }
}