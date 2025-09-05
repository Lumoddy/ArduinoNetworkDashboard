use avr_device::interrupt::CriticalSection;

use crate::soft_serial::output::{SerialOutputError, SerialOutputState};
use crate::unreachable_payload;

use super::input::{SerialInputData, SerialInputError, SerialInputState};
use super::interrupts::CYCLE_COUNTER;
use super::output::SerialOutputData;
use super::{ucycles, SERIAL_PIN_CAPACITY};

#[derive(Clone, Copy)]
pub enum Event
{
    InputStart { target: *mut Option<DynamicSerialState> },
    InputBit0 { target: *mut Option<DynamicSerialState> },
    InputBit1 { target: *mut Option<DynamicSerialState> },
    InputBit2 { target: *mut Option<DynamicSerialState> },
    InputBit3 { target: *mut Option<DynamicSerialState> },
    InputBit4 { target: *mut Option<DynamicSerialState> },
    InputBit5 { target: *mut Option<DynamicSerialState> },
    InputBit6 { target: *mut Option<DynamicSerialState> },
    InputBit7 { target: *mut Option<DynamicSerialState> },
    InputEnd { target: *mut Option<DynamicSerialState> },

    OutputStart { target: *mut Option<DynamicSerialState> },
    OutputBit0 { target: *mut Option<DynamicSerialState> },
    OutputBit1 { target: *mut Option<DynamicSerialState> },
    OutputBit2 { target: *mut Option<DynamicSerialState> },
    OutputBit3 { target: *mut Option<DynamicSerialState> },
    OutputBit4 { target: *mut Option<DynamicSerialState> },
    OutputBit5 { target: *mut Option<DynamicSerialState> },
    OutputBit6 { target: *mut Option<DynamicSerialState> },
    OutputBit7 { target: *mut Option<DynamicSerialState> },
    OutputEnd { target: *mut Option<DynamicSerialState> },
    OutputContinuousStart { target: *mut Option<DynamicSerialState> },
}

pub enum DynamicSerialState
{
    Input(SerialInputData),
    Output(SerialOutputData),
}

pub struct EventEntry
{
    pub deadline: ucycles,
    pub event: Event,
}

impl PartialOrd for EventEntry
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
    {
        self.deadline.partial_cmp(&other.deadline)
    }
}

impl PartialEq for EventEntry
{
    fn eq(&self, other: &Self) -> bool { self.deadline == other.deadline }
}

impl Eq for EventEntry { }

impl Ord for EventEntry
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering
    {
        self.deadline.cmp(&other.deadline)
    }
}

impl EventEntry
{
    pub fn run(self, _: CriticalSection)
    {
        match self.event
        {
            Event::InputStart { target } => unsafe
            {
                match &mut *target
                {
                    Some(DynamicSerialState::Input(reader)) =>
                    {
                        let Ok(()) = reader.buffer.push_front(b' ')
                        else { unreachable_payload!() };
                        let Ok(()) = reader.buffer.push_front(b'S')
                        else { unreachable_payload!() };

                        if reader.pin_is_one()
                        {
                            let Ok(()) = reader.buffer.push_front(b'#')
                            else { unreachable_payload!() };

                            reader.byte = 0;

                            let Ok(()) = EVENT_QUEUE.push(EventEntry
                            {
                                deadline: self.deadline + reader.baud_cycles,
                                event: Event::InputBit0 { target },
                            })
                            else
                            {
                                reader.state = SerialInputState::Err(
                                    SerialInputError::TooManyParallel);
                                return;
                            };
                        }
                        else
                        {
                            let Ok(()) = reader.buffer.push_front(b'-')
                            else { unreachable_payload!() };

                            reader.state = SerialInputState::Idle;
                        }
                    },
                    _ => (),
                }
            },
            Event::InputBit0 { target }
            | Event::InputBit1 { target }
            | Event::InputBit2 { target }
            | Event::InputBit3 { target }
            | Event::InputBit4 { target }
            | Event::InputBit5 { target }
            | Event::InputBit6 { target }
            | Event::InputBit7 { target } => unsafe
            {
                match &mut *target
                {
                    Some(DynamicSerialState::Input(reader)) =>
                    {
                        let Ok(()) = reader.buffer.push_front(b' ')
                        else { unreachable_payload!() };
                        let Ok(()) = reader.buffer.push_front(match self.event
                        {
                            Event::InputBit0 { target: _ } => b'0',
                            Event::InputBit1 { target: _ } => b'1',
                            Event::InputBit2 { target: _ } => b'2',
                            Event::InputBit3 { target: _ } => b'3',
                            Event::InputBit4 { target: _ } => b'4',
                            Event::InputBit5 { target: _ } => b'5',
                            Event::InputBit6 { target: _ } => b'6',
                            Event::InputBit7 { target: _ } => b'7',
                            _ => unreachable_payload!(),
                        })
                        else { unreachable_payload!() };

                        if reader.pin_is_one()
                        {
                            let Ok(()) = reader.buffer.push_front(b'#')
                            else { unreachable_payload!() };

                            reader.byte |= match self.event
                            {
                                Event::InputBit0 { target: _ } => 1 << 0,
                                Event::InputBit1 { target: _ } => 1 << 1,
                                Event::InputBit2 { target: _ } => 1 << 2,
                                Event::InputBit3 { target: _ } => 1 << 3,
                                Event::InputBit4 { target: _ } => 1 << 4,
                                Event::InputBit5 { target: _ } => 1 << 5,
                                Event::InputBit6 { target: _ } => 1 << 6,
                                Event::InputBit7 { target: _ } => 1 << 7,
                                _ => unreachable_payload!(),
                            };
                        }
                        else
                        {
                            let Ok(()) = reader.buffer.push_front(b'-')
                            else { unreachable_payload!() };
                        }

                        let Ok(()) = EVENT_QUEUE.push(EventEntry
                        {
                            deadline: self.deadline + reader.baud_cycles,
                            event: match self.event
                            {
                                Event::InputBit0 { target }
                                    => Event::InputBit1 { target },
                                Event::InputBit1 { target }
                                    => Event::InputBit2 { target },
                                Event::InputBit2 { target }
                                    => Event::InputBit3 { target },
                                Event::InputBit3 { target }
                                    => Event::InputBit4 { target },
                                Event::InputBit4 { target }
                                    => Event::InputBit5 { target },
                                Event::InputBit5 { target }
                                    => Event::InputBit6 { target },
                                Event::InputBit6 { target }
                                    => Event::InputBit7 { target },
                                Event::InputBit7 { target }
                                    => Event::InputEnd { target },
                                _ => unreachable_payload!(),
                            },
                        })
                        else
                        {
                            reader.state = SerialInputState::Err(
                                SerialInputError::TooManyParallel);
                            return;
                        };
                    },
                    _ => (),
                }
            }
            Event::InputEnd { target } => unsafe
            {
                match &mut *target
                {
                    Some(DynamicSerialState::Input(reader)) =>
                    {
                        let Ok(()) = reader.buffer.push_front(b' ')
                        else { unreachable_payload!() };
                        let Ok(()) = reader.buffer.push_front(b'E')
                        else { unreachable_payload!() };

                        if !reader.pin_is_one()
                        {
                            let Ok(()) = reader.buffer.push_front(b'-')
                            else { unreachable_payload!() };

                            reader.state = SerialInputState::Err(
                                SerialInputError::IncompleteData);
                            return;
                        }

                        let Ok(()) = reader.buffer.push_front(b'#')
                        else { unreachable_payload!() };

                        if reader.buffer.is_full()
                        {
                            reader.buffer.pop_back_unchecked();
                        }

                        reader.buffer.push_front_unchecked(reader.byte);

                        let Ok(()) = EVENT_QUEUE.push(EventEntry
                        {
                            deadline: self.deadline + reader.baud_cycles,
                            event: Event::InputStart { target },
                        })
                        else
                        {
                            reader.state = SerialInputState::Err(
                                SerialInputError::TooManyParallel);
                            return;
                        };
                    },
                    _ => (),
                }
            }

            Event::OutputStart { target } => unsafe
            {
                match &mut *target
                {
                    Some(DynamicSerialState::Output(writer)) =>
                    {
                        writer.set_pin_is_one(true);

                        let Ok(()) = EVENT_QUEUE.push(EventEntry
                        {
                            deadline: self.deadline + writer.baud_cycles,
                            event: Event::OutputBit0 { target },
                        })
                        else
                        {
                            writer.state = SerialOutputState::Err(
                                SerialOutputError::TooManyParallel);
                            return;
                        };
                    },
                    _ => (),
                }
            },
            Event::OutputBit0 { target }
            | Event::OutputBit1 { target }
            | Event::OutputBit2 { target }
            | Event::OutputBit3 { target }
            | Event::OutputBit4 { target }
            | Event::OutputBit5 { target }
            | Event::OutputBit6 { target }
            | Event::OutputBit7 { target } => unsafe
            {
                match &mut *target
                {
                    Some(DynamicSerialState::Output(writer)) =>
                    {
                        let byte = writer.byte;
                        writer.set_pin_is_one(
                            (byte & match self.event
                            {
                                Event::OutputBit0 { target: _ } => 1 << 0,
                                Event::OutputBit1 { target: _ } => 1 << 1,
                                Event::OutputBit2 { target: _ } => 1 << 2,
                                Event::OutputBit3 { target: _ } => 1 << 3,
                                Event::OutputBit4 { target: _ } => 1 << 4,
                                Event::OutputBit5 { target: _ } => 1 << 5,
                                Event::OutputBit6 { target: _ } => 1 << 6,
                                Event::OutputBit7 { target: _ } => 1 << 7,
                                _ => unreachable_payload!(),
                            })
                            != 0);

                        let Ok(()) = EVENT_QUEUE.push(EventEntry
                        {
                            deadline: self.deadline + writer.baud_cycles,
                            event: match self.event
                            {
                                Event::OutputBit0 { target }
                                    => Event::OutputBit1 { target },
                                Event::OutputBit1 { target }
                                    => Event::OutputBit2 { target },
                                Event::OutputBit2 { target }
                                    => Event::OutputBit3 { target },
                                Event::OutputBit3 { target }
                                    => Event::OutputBit4 { target },
                                Event::OutputBit4 { target }
                                    => Event::OutputBit5 { target },
                                Event::OutputBit5 { target }
                                    => Event::OutputBit6 { target },
                                Event::OutputBit6 { target }
                                    => Event::OutputBit7 { target },
                                Event::OutputBit7 { target }
                                    => Event::OutputEnd { target },
                                _ => unreachable_payload!(),
                            },
                        })
                        else
                        {
                            writer.state = SerialOutputState::Err(
                                SerialOutputError::TooManyParallel);
                            return;
                        };
                    },
                    _ => (),
                }
            }
            Event::OutputEnd { target } => unsafe
            {
                match &mut *target
                {
                    Some(DynamicSerialState::Output(writer)) =>
                    {
                        writer.set_pin_is_one(true);

                        let Ok(()) = EVENT_QUEUE.push(EventEntry
                        {
                            deadline: self.deadline + writer.baud_cycles,
                            event: Event::OutputContinuousStart { target },
                        })
                        else
                        {
                            writer.state = SerialOutputState::Err(
                                SerialOutputError::TooManyParallel);
                            return;
                        };
                    },
                    _ => (),
                }
            },
            Event::OutputContinuousStart { target } => unsafe
            {
                match &mut *target
                {
                    Some(DynamicSerialState::Output(writer)) =>
                    {
                        if writer.buffer.is_empty()
                        {
                            writer.set_pin_is_one(false);

                            writer.state = SerialOutputState::Idle;
                        }
                        else
                        {
                            writer.set_pin_is_one(true);

                            writer.byte = writer.buffer.pop_back_unchecked();

                            let Ok(()) = EVENT_QUEUE.push(EventEntry
                            {
                                deadline: self.deadline + writer.baud_cycles,
                                event: Event::OutputBit0 { target },
                            })
                            else
                            {
                                writer.state = SerialOutputState::Err(
                                    SerialOutputError::TooManyParallel);
                                return;
                            };
                        }
                    },
                    _ => (),
                }
            },
        }
    }
}

pub unsafe fn push_event(entry: EventEntry) -> Result<(), EventEntry>
{
    if matches!(EVENT_QUEUE.peek(), Some(first_entry) if &entry >= first_entry)
    {
        EVENT_QUEUE.push(entry)
    }
    else
    {
        let dp = arduino_hal::Peripherals::steal();

        match entry.deadline.checked_sub(CYCLE_COUNTER)
        {
            Some(0xFFFF..=ucycles::MAX) =>
            {
                dp.TC1.ocr1a.write(|w| w
                    .bits(0xFFFF));
                dp.TC1.tifr1.write(|w| w
                    .ocf1a().set_bit());

                EVENT_QUEUE.push(entry)
            },
            None | Some(0) =>
            {
                entry.run(CriticalSection::new());

                dp.TC1.ocr1a.write(|w| w
                    .bits(0));
                dp.TC1.tifr1.write(|w|
                    w.ocf1a().clear_bit());

                Ok(())
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

                EVENT_QUEUE.push(entry)
            },
        }
    }
}

pub static mut EVENT_QUEUE: heapless::BinaryHeap<
    EventEntry,
    heapless::binary_heap::Min,
    SERIAL_PIN_CAPACITY> = heapless::BinaryHeap::new();

pub static mut SERIAL_STATES: [Option<DynamicSerialState>; SERIAL_PIN_CAPACITY]
    = [const { None }; SERIAL_PIN_CAPACITY];