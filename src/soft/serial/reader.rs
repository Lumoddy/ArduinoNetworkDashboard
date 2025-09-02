use core::{mem, ptr};

use arduino_hal::clock::Clock;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::{mode, PinOps, Pin};
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::DefaultClock;
use avr_device::interrupt;

use crate::soft::exint::{self, StaticIntoPinID};
use crate::soft::tc1;
use crate::unreachable_payload;

pub trait SoftSerialReaderBufferOps
{
    fn push_front(&mut self, byte: u8) -> Result<(), u8>;
}

impl<const N: usize> SoftSerialReaderBufferOps for heapless::Deque<u8, N>
{
    fn push_front(&mut self, byte: u8) -> Result<(), u8>
    {
        self.push_front(byte)
    }
}

pub struct SoftSerialReader<PIN: PinOps<Dynamic = Dynamic>>
{
    _ghost_pin: Pin<mode::Input<mode::PullUp>, PIN>,
    _state_index: u8,
}

#[derive(Clone, Copy)]
pub enum ReadError
{
    AlreadyWriting,
    TaskQueueFull,
}

impl<PIN: PinOps<Dynamic = Dynamic> + StaticIntoPinID> SoftSerialReader<PIN>
{
    pub fn deconstruct<'s>(self)
        -> (Pin<mode::Input<mode::PullUp>, PIN>,
            &'static mut dyn SoftSerialReaderBufferOps)
    {
        interrupt::free(|_| unsafe
        {
            let Some(reader) = mem::replace(
                &mut _READERS[self._state_index as usize].0,
                None) else { unreachable_payload!() };

            (
                self._ghost_pin,
                reader._buffer,
            )
        })
    }

    pub fn start_read_now(&mut self) -> Result<(), ReadError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(reader) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            match reader._phase
            {
                _ReaderPhase::Idle =>
                {
                    reader._phase = _ReaderPhase::StartBit;

                    let Ok(()) = reader._tc1.schedule_task_cycles(
                        0xFF,
                        reader._baud_cycles / 2,
                        _READERS[self._state_index as usize].1)
                    else
                    {
                        reader._phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return Err(ReadError::TaskQueueFull);
                    };

                    Ok(())
                },
                _ => Err(ReadError::AlreadyWriting),
            }
        })
    }

    pub fn is_reading(&self) -> bool
    {
        interrupt::free(|_| unsafe
        {
            let Some(writer) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            !matches!(writer._phase, _ReaderPhase::Idle)
        })
    }

    pub fn err(&mut self) -> Option<ReadError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(reader) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            if let _ReaderPhase::Err(error) = reader._phase
            { Some(error) }
            else
            { None }
        })
    }
}

pub enum InitError<
    's,
    PIN: PinOps<Dynamic = Dynamic>,
    Buffer: 'static + ?Sized + SoftSerialReaderBufferOps
        = dyn 'static + SoftSerialReaderBufferOps>
{
    TooManyReaders(
        Pin<mode::Input<mode::PullUp>, PIN>,
        ReaderConfig<'s, Buffer>),
}

pub struct ReaderConfig<
    's,
    Buffer: 'static + ?Sized + SoftSerialReaderBufferOps
        = dyn 'static + SoftSerialReaderBufferOps>
{
    pub baudrate: u32,
    pub tc1: &'s tc1::Scheduler,
    pub exint: &'s exint::Scheduler,
    pub buffer: &'static mut Buffer,
    pub inverse_voltage: bool,
}

pub fn init<PIN: PinOps<Dynamic = Dynamic> + StaticIntoPinID>(
    pin: Pin<mode::Input<mode::PullUp>, PIN>,
    config: ReaderConfig)
    -> Result<SoftSerialReader<PIN>, InitError<PIN>>
{
    unsafe
    {
        for (i, reader) in &mut _READERS.iter_mut().enumerate()
        {
            let None = reader.0 else { continue };

            let result = SoftSerialReader
            {
                _ghost_pin: ptr::read(&raw const pin),
                _state_index: i as u8,
            };

            reader.0 = Some(_ReaderState
            {
                _pin: pin.downgrade(),
                _buffer: config.buffer,
                _phase: _ReaderPhase::Idle,
                _tc1: tc1::Scheduler::steal_copy(config.tc1),
                _exint: exint::Scheduler::steal_copy(config.exint),
                _baudrate: config.baudrate,
                _baud_cycles: DefaultClock::FREQ as u64
                    / (config.baudrate as u64 * match tc1::PRESCALER
                    {
                        Prescaler::Direct => 1,
                        Prescaler::Prescale8 => 8,
                        Prescaler::Prescale64 => 64,
                        Prescaler::Prescale256 => 256,
                        Prescaler::Prescale1024 => 1024,
                    }),
                _high_is_one: !config.inverse_voltage,
            });

            return Ok(result);
        }

        Err(InitError::TooManyReaders(pin, config))
    }
}

enum _ReaderPhase
{
    Idle,
    StartBit,
    Bit(u8),
    Err(ReadError)
}

struct _ReaderState
{
    _pin: Pin<mode::Input<mode::PullUp>, Dynamic>,
    _buffer: &'static mut dyn SoftSerialReaderBufferOps,
    _phase: _ReaderPhase,
    _tc1: tc1::Scheduler,
    _exint: exint::Scheduler,
    _baudrate: u32,
    _baud_cycles: u64,
    _high_is_one: bool,
}

impl _ReaderState
{
    fn _process_state(
        &mut self,
        context: tc1::SchedulerTaskContext,
        process_state_task: fn(tc1::SchedulerTaskContext))
    {
        match self._phase
        {
            _ReaderPhase::Idle => (),
            _ReaderPhase::StartBit =>
            {
                if self._pin_is_one()
                {
                    self._phase = _ReaderPhase::Bit(1);

                    let Ok(()) = self._tc1.schedule_task_absolute(
                        0xFF,
                        context.cycles_since_init + self._baud_cycles,
                        process_state_task)
                    else
                    {
                        self._phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return;
                    };
                }
                else
                {
                    self._phase = _ReaderPhase::Idle;
                }
            },
            _ReaderPhase::Bit(accumulate) =>
            {
                let mut next_accumulate = accumulate.unbounded_shl(1);

                if self._pin_is_one()
                {
                    next_accumulate |= 1;
                }

                if (accumulate & 0x80) == 0
                {
                    self._phase = _ReaderPhase::Bit(next_accumulate);

                    let Ok(()) = self._tc1.schedule_task_absolute(
                        0xFF,
                        context.cycles_since_init + self._baud_cycles,
                        process_state_task)
                    else
                    {
                        self._phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return;
                    };
                }
                else
                {
                    self._phase = _ReaderPhase::StartBit;
                    let Ok(()) = self._buffer.push_front(next_accumulate)
                    else
                    {
                        self._phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return;
                    };

                    let Ok(()) = self._tc1.schedule_task_absolute(
                        0xFF,
                        context.cycles_since_init + (self._baud_cycles * 2),
                        process_state_task)
                    else
                    {
                        self._phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return;
                    };
                }
            },
            _ReaderPhase::Err(_) => (),
        }
    }

    fn _pin_is_one(&self) -> bool
    {
        self._pin.is_high() == self._high_is_one
    }
}

static mut _READERS: [(Option<_ReaderState>, fn(tc1::SchedulerTaskContext)); 4] =
[
    (None, |cs| unsafe
    {
        let Some(reader) = &mut _READERS[0].0
        else { return };

        reader._process_state(cs, _READERS[0].1);
    }),
    (None, |cs| unsafe
    {
        let Some(reader) = &mut _READERS[1].0
        else { return };

        reader._process_state(cs, _READERS[1].1);
    }),
    (None, |cs| unsafe
    {
        let Some(reader) = &mut _READERS[2].0
        else { return };

        reader._process_state(cs, _READERS[2].1);
    }),
    (None, |cs| unsafe
    {
        let Some(reader) = &mut _READERS[3].0
        else { return };

        reader._process_state(cs, _READERS[3].1);
    }),
];