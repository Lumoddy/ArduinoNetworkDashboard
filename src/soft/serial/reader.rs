use core::{mem, ptr};

use arduino_hal::clock::Clock;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::{mode, PinOps, Pin};
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::DefaultClock;
use avr_device::interrupt;

use crate::soft::exint::{self, IntoPinID, PinEdge, PinPortID, StaticIntoPinID};
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
    BufferFull,
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
                reader.buffer,
            )
        })
    }

    pub fn start_read_now(&mut self) -> Result<(), ReadError>
    {
        interrupt::free(|_| unsafe
        {
            let (Some(reader), process_state_task)
                = &mut _READERS[self._state_index as usize]
            else { unreachable_payload!() };

            match reader.next_phase
            {
                _ReaderPhase::Idle =>
                {
                    reader.next_phase = _ReaderPhase::StartBit;

                    let Ok(()) = reader.tc1.schedule_task_cycles(
                        0xFF,
                        reader.baud_cycles / 2,
                        *process_state_task)
                    else
                    {
                        reader.next_phase = _ReaderPhase::Err(
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

            !matches!(writer.next_phase, _ReaderPhase::Idle)
        })
    }

    pub fn err(&mut self) -> Option<ReadError>
    {
        interrupt::free(|_| unsafe
        {
            let Some(reader) = &mut _READERS[self._state_index as usize].0
            else { unreachable_payload!() };

            if let _ReaderPhase::Err(error) = reader.next_phase
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
    TaskQueueFull(
        Pin<mode::Input<mode::PullUp>, PIN>,
        ReaderConfig<'s, Buffer>),
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
        for (i, (reader, _)) in &mut _READERS.iter_mut().enumerate()
        {
            let None = reader else { continue };

            let result = SoftSerialReader
            {
                _ghost_pin: ptr::read(&raw const pin),
                _state_index: i as u8,
            };

            let high_is_one = config.inverse_voltage;

            let Ok(()) = config.exint.schedule_task(
                pin.id(),
                if high_is_one { PinEdge::LowToHigh }
                else { PinEdge::HighToLow },
                _read_pin_now_task)
            else { return Err(InitError::TaskQueueFull(pin, config)) };

            *reader = Some(_ReaderState
            {
                pin_id: pin.id(),
                pin: pin.downgrade(),
                buffer: config.buffer,
                next_phase: _ReaderPhase::Idle,
                tc1: tc1::Scheduler::steal_copy(config.tc1),
                exint: exint::Scheduler::steal_copy(config.exint),
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
                high_is_one,
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
    Bit0,
    Bit1(u8),
    Bit2(u8),
    Bit3(u8),
    Bit4(u8),
    Bit5(u8),
    Bit6(u8),
    Bit7(u8),
    Err(ReadError)
}

struct _ReaderState
{
    pub pin_id: PinPortID,
    pub pin: Pin<mode::Input<mode::PullUp>, Dynamic>,
    pub buffer: &'static mut dyn SoftSerialReaderBufferOps,
    pub next_phase: _ReaderPhase,
    pub tc1: tc1::Scheduler,
    pub exint: exint::Scheduler,
    pub baudrate: u32,
    pub baud_cycles: u64,
    pub high_is_one: bool,
}

impl _ReaderState
{
    fn _process_state(
        &mut self,
        context: tc1::SchedulerTaskContext,
        process_state_task: fn(tc1::SchedulerTaskContext))
    {
        match self.next_phase
        {
            _ReaderPhase::Idle => (),
            _ReaderPhase::Err(_) => (),
            _ReaderPhase::StartBit =>
            {
                self.buffer.push_front(b'1');

                if self._pin_is_one()
                {
                    self.buffer.push_front(b'^');

                    self.next_phase = _ReaderPhase::Bit0;

                    let Ok(()) = self.tc1.schedule_task_absolute(
                        0xFF,
                        context.cycles_since_init + self.baud_cycles,
                        process_state_task)
                    else
                    {
                        self.next_phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return;
                    };
                }
                else
                {
                    self.buffer.push_front(b'v');

                    self.next_phase = _ReaderPhase::Idle;

                    let Ok(()) = self.exint.schedule_task(
                        self.pin_id,
                        if self.high_is_one { PinEdge::LowToHigh }
                        else { PinEdge::HighToLow },
                        _read_pin_now_task)
                    else
                    {
                        self.next_phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return;
                    };
                }
            },
            _ReaderPhase::Bit0 =>
            {
                self.buffer.push_front(b'2');

                if self._pin_is_one()
                {
                    self.buffer.push_front(b'^');

                    self.next_phase = _ReaderPhase::Bit1(1);
                }
                else
                {
                    self.buffer.push_front(b'v');

                    self.next_phase = _ReaderPhase::Bit1(0);
                }

                let Ok(()) = self.tc1.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + self.baud_cycles,
                    process_state_task)
                else
                {
                    self.next_phase = _ReaderPhase::Err(
                        ReadError::TaskQueueFull);
                    return;
                };
            }
            _ReaderPhase::Bit1(byte)
            | _ReaderPhase::Bit2(byte)
            | _ReaderPhase::Bit3(byte)
            | _ReaderPhase::Bit4(byte)
            | _ReaderPhase::Bit5(byte)
            | _ReaderPhase::Bit6(byte) =>
            {
                self.buffer.push_front(b'2');

                if self._pin_is_one()
                {
                    self.buffer.push_front(b'^');

                    self.next_phase = match self.next_phase
                    {
                        _ReaderPhase::Bit1(_) => _ReaderPhase::Bit2(byte | (1 << 1)),
                        _ReaderPhase::Bit2(_) => _ReaderPhase::Bit3(byte | (1 << 2)),
                        _ReaderPhase::Bit3(_) => _ReaderPhase::Bit4(byte | (1 << 3)),
                        _ReaderPhase::Bit4(_) => _ReaderPhase::Bit5(byte | (1 << 4)),
                        _ReaderPhase::Bit5(_) => _ReaderPhase::Bit6(byte | (1 << 5)),
                        _ReaderPhase::Bit6(_) => _ReaderPhase::Bit7(byte | (1 << 6)),
                        _ => unreachable_payload!(),
                    };
                }
                else
                {
                    self.buffer.push_front(b'v');

                    self.next_phase = match self.next_phase
                    {
                        _ReaderPhase::Bit1(_) => _ReaderPhase::Bit2(byte),
                        _ReaderPhase::Bit2(_) => _ReaderPhase::Bit3(byte),
                        _ReaderPhase::Bit3(_) => _ReaderPhase::Bit4(byte),
                        _ReaderPhase::Bit4(_) => _ReaderPhase::Bit5(byte),
                        _ReaderPhase::Bit5(_) => _ReaderPhase::Bit6(byte),
                        _ReaderPhase::Bit6(_) => _ReaderPhase::Bit7(byte),
                        _ => unreachable_payload!(),
                    };
                }

                let Ok(()) = self.tc1.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + self.baud_cycles,
                    process_state_task)
                else
                {
                    self.next_phase = _ReaderPhase::Err(
                        ReadError::TaskQueueFull);
                    return;
                };
            },
            _ReaderPhase::Bit7(byte) =>
            {
                self.buffer.push_front(b'D');

                let final_byte;

                if self._pin_is_one()
                {
                    self.buffer.push_front(b'^');

                    final_byte = byte | (1 << 7);
                }
                else
                {
                    self.buffer.push_front(b'v');

                    final_byte = byte;
                }

                self.next_phase = _ReaderPhase::StartBit;

                let Ok(()) = self.buffer.push_front(final_byte)
                else
                {
                    self.next_phase = _ReaderPhase::Err(
                        ReadError::BufferFull);
                    return;
                };

                let Ok(()) = self.tc1.schedule_task_absolute(
                    0xFF,
                    context.cycles_since_init + (self.baud_cycles * 2),
                    process_state_task)
                else
                {
                    self.next_phase = _ReaderPhase::Err(
                        ReadError::TaskQueueFull);
                    return;
                };
            }
        }
    }

    fn _pin_is_one(&self) -> bool
    {
        self.pin.is_high() == self.high_is_one
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

fn _read_pin_now_task(context: exint::SchedulerTaskContext)
{
    unsafe
    {
        for reader in &mut _READERS.iter_mut()
        {
            let (Some(reader), process_state_task) = reader else { continue };

            if reader.high_is_one != context.pin_is_high { continue };

            if reader.pin_id != context.pin { continue };

            match reader.next_phase
            {
                _ReaderPhase::Idle =>
                {
                    reader.next_phase = _ReaderPhase::StartBit;

                    let Ok(()) = reader.tc1.schedule_task_cycles(
                        0xFF,
                        (reader.baud_cycles / 2) - match tc1::PRESCALER
                        {
                            Prescaler::Direct => todo!(),
                            Prescaler::Prescale8 => 80,
                            Prescaler::Prescale64 => todo!(),
                            Prescaler::Prescale256 => todo!(),
                            Prescaler::Prescale1024 => todo!(),
                        },
                        *process_state_task)
                    else
                    {
                        reader.next_phase = _ReaderPhase::Err(
                            ReadError::TaskQueueFull);
                        return;
                    };
                },
                _ => (),
            }
        }
    }
}