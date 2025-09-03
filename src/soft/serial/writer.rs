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

            (self._ghost_pin, writer.buffer)
        })
    }

    pub fn start_write_now(&mut self) -> Result<(), WriteError>
    {
        interrupt::free(|_| unsafe
        {
            let (Some(writer), process_state_task)
                = &mut _READERS[self._state_index as usize]
            else { unreachable_payload!() };

            match writer.last_phase
            {
                _WriterPhase::Idle =>
                {
                    if !writer.scheduler.can_schedule_task()
                    { return Err(WriteError::TaskQueueFull) };

                    let Some(byte) = writer.buffer.pop_back()
                    else { return Err(WriteError::NothingToWrite) };

                    writer._set_pin(true);

                    writer.last_phase = _WriterPhase::StartBit(byte);

                    let Ok(()) = writer.scheduler.schedule_task_cycles(
                        0xFF,
                        writer.baud_cycles,
                        *process_state_task)
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

            !matches!(writer.last_phase, _WriterPhase::Idle)
        })
    }

    pub fn err(&self) -> Option<WriteError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(writer) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            if let _WriterPhase::Err(error) = writer.last_phase
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
                pin: pin.downgrade(),
                buffer: config.buffer,
                last_phase: _WriterPhase::Idle,
                scheduler: tc1::Scheduler::steal_copy(config.tc1),
                baudrate: config.baudrate,
                baud_cycles: DefaultClock::FREQ as u64
                    / (config.baudrate as u64 * match tc1::PRESCALER
                    {
                        Prescaler::Direct => 1,
                        Prescaler::Prescale8 => 8,
                        Prescaler::Prescale64 => 64,
                        Prescaler::Prescale256 => 256,
                        Prescaler::Prescale1024 => 1024,
                    }),
                high_is_one: config.inverse_voltage,
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
    Bit0(u8),
    Bit1(u8),
    Bit2(u8),
    Bit3(u8),
    Bit4(u8),
    Bit5(u8),
    Bit6(u8),
    Bit7,
    EndBit,
    Err(WriteError),
}

struct _WriterState
{
    pub pin: Pin<mode::Output, Dynamic>,
    pub buffer: &'static mut dyn SoftSerialWriterBufferOps,
    pub last_phase: _WriterPhase,
    pub scheduler: tc1::Scheduler,
    pub baudrate: u32,
    pub baud_cycles: u64,
    pub high_is_one: bool,
}

impl _WriterState
{
    fn _process_state(
        &mut self,
        context: tc1::SchedulerTaskContext,
        process_state_task: fn(tc1::SchedulerTaskContext))
    {
        match self.last_phase
        {
            _WriterPhase::Idle => (),
            _WriterPhase::Err(_) => (),
            _WriterPhase::StartBit(byte) =>
            {
                self._set_pin((byte & 1) != 0);

                self.last_phase = _WriterPhase::Bit0(byte);

                let Ok(()) = self.scheduler.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + self.baud_cycles,
                    process_state_task)
                else
                {
                    self.last_phase = _WriterPhase::Err(
                        WriteError::TaskQueueFull);
                    return;
                };
            },
            _WriterPhase::Bit0(byte)
            | _WriterPhase::Bit1(byte)
            | _WriterPhase::Bit2(byte)
            | _WriterPhase::Bit3(byte)
            | _WriterPhase::Bit4(byte)
            | _WriterPhase::Bit5(byte)
            | _WriterPhase::Bit6(byte) =>
            {
                let (mask, next_phase) = match self.last_phase
                {
                    _WriterPhase::Bit0(_) => (1 << 1, _WriterPhase::Bit1(byte)),
                    _WriterPhase::Bit1(_) => (1 << 2, _WriterPhase::Bit2(byte)),
                    _WriterPhase::Bit2(_) => (1 << 3, _WriterPhase::Bit3(byte)),
                    _WriterPhase::Bit3(_) => (1 << 4, _WriterPhase::Bit4(byte)),
                    _WriterPhase::Bit4(_) => (1 << 5, _WriterPhase::Bit5(byte)),
                    _WriterPhase::Bit5(_) => (1 << 6, _WriterPhase::Bit6(byte)),
                    _WriterPhase::Bit6(_) => (1 << 7, _WriterPhase::Bit7),
                    _ => unreachable_payload!(),
                };

                self._set_pin((byte & mask) != 0);

                self.last_phase = next_phase;

                let Ok(()) = self.scheduler.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + self.baud_cycles,
                    process_state_task)
                else
                {
                    self.last_phase = _WriterPhase::Err(
                        WriteError::TaskQueueFull);
                    return;
                };
            },
            _WriterPhase::Bit7 =>
            {
                self._set_pin(true);

                self.last_phase = _WriterPhase::EndBit;

                let Ok(()) = self.scheduler.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + self.baud_cycles,
                    process_state_task)
                else
                {
                    self.last_phase = _WriterPhase::Err(
                        WriteError::TaskQueueFull);
                    return;
                };
            },
            _WriterPhase::EndBit => match self.buffer.pop_back()
            {
                Some(next_byte) =>
                {
                    self._set_pin(true);

                    self.last_phase = _WriterPhase::StartBit(next_byte);

                    let Ok(()) = self.scheduler.schedule_task_absolute(
                        0xFF,
                        context.cycles_since_init + self.baud_cycles,
                        process_state_task)
                    else
                    {
                        self.last_phase = _WriterPhase::Err(
                            WriteError::TaskQueueFull);
                        return;
                    };
                },
                None =>
                {
                    self._set_pin(false);

                    self.last_phase = _WriterPhase::Idle;
                },
            },
        }
    }

    fn _set_pin(&mut self, one: bool)
    {
        if one == self.high_is_one
        { self.pin.set_high() }
        else
        { self.pin.set_low() }
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