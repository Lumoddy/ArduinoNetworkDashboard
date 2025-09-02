use core::{mem, ptr};

use arduino_hal::clock::Clock;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::{mode, PinOps, Pin};
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::DefaultClock;
use avr_device::interrupt;

use crate::soft::tc1;
use crate::unreachable_payload;

pub trait SoftSerialWriterBufferOps
{
    fn pop_back(&mut self) -> Option<u8>;
}

impl<const N: usize> SoftSerialWriterBufferOps for heapless::Deque<u8, N>
{
    fn pop_back(&mut self) -> Option<u8>
    {
        self.pop_back()
    }
}

pub struct SoftSerialWriter<PIN: PinOps<Dynamic = Dynamic>>
{
    _ghost_pin: Pin<mode::Output, PIN>,
    _state_index: u8,
}

#[derive(Clone, Copy)]
pub enum WriteError
{
    AlreadyWriting,
    NothingToWrite,
    TaskQueueFull,
}

impl<PIN: PinOps<Dynamic = Dynamic>> SoftSerialWriter<PIN>
{
    pub fn deconstruct(self)
        -> (Pin<mode::Output, PIN>, &'static mut dyn SoftSerialWriterBufferOps)
    {
        interrupt::free(|_| unsafe
        {
            let Some(writer) = mem::replace(
                &mut _READERS[self._state_index as usize].0,
                None) else { unreachable_payload!() };

            (self._ghost_pin, writer._buffer)
        })
    }

    pub fn start_write_now(&mut self) -> Result<(), WriteError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(writer) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            match writer._phase
            {
                _WriterPhase::Idle =>
                {
                    if !writer._scheduler.can_schedule_task()
                    { return Err(WriteError::TaskQueueFull) };

                    let Some(byte) = writer._buffer.pop_back()
                    else { return Err(WriteError::NothingToWrite) };

                    writer._set_pin(true);

                    writer._phase = _WriterPhase::StartBit(byte);

                    let Ok(()) = writer._scheduler.schedule_task_cycles(
                        0xFF,
                        writer._baud_cycles,
                        _READERS[self._state_index as usize].1)
                    else { unreachable_payload!() };

                    Ok(())
                },
                _ => Err(WriteError::AlreadyWriting),
            }
        })
    }

    pub fn is_writing(&self) -> bool
    {
        interrupt::free(|_| unsafe
        {
            let Some(writer) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            !matches!(writer._phase, _WriterPhase::Idle)
        })
    }

    pub fn err(&self) -> Option<WriteError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(writer) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            if let _WriterPhase::Err(error) = writer._phase
            { Some(error) }
            else
            { None }
        })
    }
}

pub enum InitError<
    's,
    PIN: PinOps<Dynamic = Dynamic>,
    Buffer: 'static + SoftSerialWriterBufferOps + ?Sized
        = dyn 'static + SoftSerialWriterBufferOps>
{
    TooManyWriters(
        Pin<mode::Output, PIN>,
        WriterConfig<'s, Buffer>),
}

pub struct WriterConfig<
    's,
    Buffer: 'static + SoftSerialWriterBufferOps + ?Sized
        = dyn 'static + SoftSerialWriterBufferOps>
{
    pub baudrate: u32,
    pub tc1: &'s tc1::Scheduler,
    pub buffer: &'static mut Buffer,
    pub inverse_voltage: bool,
}

pub fn init<PIN: PinOps<Dynamic = Dynamic>>(
    pin: Pin<mode::Output, PIN>,
    config: WriterConfig)
    -> Result<SoftSerialWriter<PIN>, InitError<PIN>>
{
    unsafe
    {
        for (i, writer) in &mut _READERS.iter_mut().enumerate()
        {
            let None = writer.0 else { continue };

            let result = SoftSerialWriter
            {
                _ghost_pin: ptr::read(&raw const pin),
                _state_index: i as u8,
            };

            writer.0 = Some(_WriterState
            {
                _pin: pin.downgrade(),
                _buffer: config.buffer,
                _phase: _WriterPhase::Idle,
                _scheduler: tc1::Scheduler::steal_copy(config.tc1),
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

        Err(InitError::TooManyWriters(pin, config))
    }
}

enum _WriterPhase
{
    Idle,
    StartBit(u8),
    Bit(u8),
    EndBit,
    Err(WriteError),
}

struct _WriterState
{
    _pin: Pin<mode::Output, Dynamic>,
    _buffer: &'static mut dyn SoftSerialWriterBufferOps,
    _phase: _WriterPhase,
    _scheduler: tc1::Scheduler,
    _baudrate: u32,
    _baud_cycles: u64,
    _high_is_one: bool,
}

impl _WriterState
{
    fn _process_state(
        &mut self,
        context: tc1::SchedulerTaskContext,
        process_state_task: fn(tc1::SchedulerTaskContext))
    {
        match self._phase
        {
            _WriterPhase::Idle => (),
            _WriterPhase::StartBit(buffer) =>
            {
                self._set_pin((buffer & 0x1) != 0);

                self._phase = _WriterPhase::Bit(buffer.unbounded_shr(1) | 0x80);

                let Ok(()) = self._scheduler.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + self._baud_cycles,
                    process_state_task)
                else
                {
                    self._phase = _WriterPhase::Err(WriteError::TaskQueueFull);
                    return;
                };
            },
            _WriterPhase::Bit(buffer) =>
            {
                let next_buffer = buffer.unbounded_shr(1);

                self._set_pin((buffer & 0x1) != 0);

                if next_buffer == 0x1
                {
                    self._phase = _WriterPhase::EndBit;

                    let Ok(()) = self._scheduler.schedule_task_cycles(
                        0xFF,
                        self._baud_cycles,
                        process_state_task)
                    else
                    {
                        self._phase = _WriterPhase::Err(WriteError::TaskQueueFull);
                        return;
                    };
                }
                else
                {
                    self._phase = _WriterPhase::Bit(next_buffer);

                    let Ok(()) = self._scheduler.schedule_task_absolute(
                        0xFF,
                        context.cycles_since_init + self._baud_cycles,
                        process_state_task)
                    else
                    {
                        self._phase = _WriterPhase::Err(WriteError::TaskQueueFull);
                        return;
                    };
                }
            },
            _WriterPhase::EndBit =>
            {
                let Some(byte) = self._buffer.pop_back()
                else
                {
                    self._phase = _WriterPhase::Idle;

                    self._set_pin(false);
                    return
                };

                self._set_pin(true);

                self._phase = _WriterPhase::StartBit(byte);

                let Ok(()) = self._scheduler.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + self._baud_cycles,
                    process_state_task)
                else
                {
                    self._phase = _WriterPhase::Err(WriteError::TaskQueueFull);
                    return;
                };
            },
            _WriterPhase::Err(_) => (),
        }
    }

    fn _set_pin(&mut self, one: bool)
    {
        if one == self._high_is_one
        { self._pin.set_low() }
        else
        { self._pin.set_high() }
    }
}

static mut _READERS: [(Option<_WriterState>, fn(tc1::SchedulerTaskContext)); 4] =
[
    (None, |cs| unsafe
    {
        let Some(writer) = &mut _READERS[0].0
        else { return };

        writer._process_state(cs, _READERS[0].1);
    }),
    (None, |cs| unsafe
    {
        let Some(writer) = &mut _READERS[1].0
        else { return };

        writer._process_state(cs, _READERS[1].1);
    }),
    (None, |cs| unsafe
    {
        let Some(writer) = &mut _READERS[2].0
        else { return };

        writer._process_state(cs, _READERS[2].1);
    }),
    (None, |cs| unsafe
    {
        let Some(writer) = &mut _READERS[3].0
        else { return };

        writer._process_state(cs, _READERS[3].1);
    }),
];