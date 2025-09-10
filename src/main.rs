#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(abi_avr_interrupt)]

use arduino_hal::clock::Clock;
use arduino_hal::port::mode;
use arduino_hal::port::Pin;
use arduino_hal::port::PinOps;
use arduino_hal::prelude::_embedded_hal_serial_Read;
use arduino_hal::prelude::_embedded_hal_serial_Write;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::DefaultClock;
use nb::block;
use nbt::reader::ReadRaw;
use nbt::ElementType;
mod panic_handler;
mod nbt;

use core::convert::Infallible;

use nbt::writer::WriteRaw;
use nbt::Type;

#[arduino_hal::entry]
fn main() -> ! { process() }

pub enum InteractablePin<PIN: PinOps>
{
    DigitalInput(Pin<mode::Input<mode::PullUp>, PIN>),
    DigitalOutput(Pin<mode::Output, PIN>),
}

#[derive(Clone, Copy)]
enum ReadError
{
    FoundControlByte(u8),
    TimedOut,
    InvalidTag { message: &'static str, path: &'static str },
    StateError { message: &'static str, path: &'static str },
}

impl From<&mut ReadError> for ReadError
{
    fn from(value: &mut ReadError) -> Self { *value }
}

impl From<&ReadError> for ReadError
{
    fn from(value: &ReadError) -> Self { *value }
}

fn process() -> !
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let (mut serial_reader, mut serial_writer) = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(9600)).split();

    let mut raw_serial_reader = unsafe { core::ptr::read(&serial_reader) };
    let mut raw_serial_writer = unsafe { core::ptr::read(&serial_writer) };

    pub enum InteractablePinId
    {
        D2, D3, D4, D5, D6, D7,
        D8, D9, D10, D11, D12, D13,
        A0, A1, A2, A3, A4, A5,
    }

    let mut interactive_d2 = InteractablePin::DigitalInput(pins.d2.into_pull_up_input());
    let mut interactive_d3 = InteractablePin::DigitalInput(pins.d3.into_pull_up_input());
    let mut interactive_d4 = InteractablePin::DigitalInput(pins.d4.into_pull_up_input());
    let mut interactive_d5 = InteractablePin::DigitalInput(pins.d5.into_pull_up_input());
    let mut interactive_d6 = InteractablePin::DigitalInput(pins.d6.into_pull_up_input());
    let mut interactive_d7 = InteractablePin::DigitalInput(pins.d7.into_pull_up_input());

    let mut interactive_d8 = InteractablePin::DigitalInput(pins.d8.into_pull_up_input());
    let mut interactive_d9 = InteractablePin::DigitalInput(pins.d9.into_pull_up_input());
    let mut interactive_d10 = InteractablePin::DigitalInput(pins.d10.into_pull_up_input());
    let mut interactive_d11 = InteractablePin::DigitalInput(pins.d11.into_pull_up_input());
    let mut interactive_d12 = InteractablePin::DigitalInput(pins.d12.into_pull_up_input());
    let mut interactive_d13 = InteractablePin::DigitalInput(pins.d13.into_pull_up_input());

    let mut interactive_a0 = InteractablePin::DigitalInput(pins.a0.into_pull_up_input());
    let mut interactive_a1 = InteractablePin::DigitalInput(pins.a1.into_pull_up_input());
    let mut interactive_a2 = InteractablePin::DigitalInput(pins.a2.into_pull_up_input());
    let mut interactive_a3 = InteractablePin::DigitalInput(pins.a3.into_pull_up_input());
    let mut interactive_a4 = InteractablePin::DigitalInput(pins.a4.into_pull_up_input());
    let mut interactive_a5 = InteractablePin::DigitalInput(pins.a5.into_pull_up_input());

    // https://symbl.cc/en/unicode-table/
    const CONTROL_BYTE: u8 = b'';
    const START_TEXT_BYTE: u8 = b'';

    let mut reader = nbt::reader::ClosureRawReader::<ReadError, _>::new(
        nbt::Endian::Little,
        ||
        {
            const MAX_ATTEMPTS: u32 = DefaultClock::FREQ / 1000;

            let mut read = ||
            {
                let mut attempt_limit = MAX_ATTEMPTS;

                loop
                {
                    match serial_reader.read()
                    {
                        Ok(byte) => break Ok(byte),
                        Err(nb::Error::WouldBlock) =>
                        {
                            if attempt_limit == 0
                            {
                                break Err(ReadError::TimedOut);
                            }

                            attempt_limit -= 1;
                        },
                    }
                }
            };

            match read()?
            {
                CONTROL_BYTE =>
                {
                    match read()?
                    {
                        CONTROL_BYTE => Ok(CONTROL_BYTE),
                        byte => Err(ReadError::FoundControlByte(byte)),
                    }
                }
                byte => Ok(byte),
            }
        });

    let mut writer = nbt::writer::ClosureRawWriter::<Infallible, _>::new(
        nbt::Endian::Little,
        |byte|
        {
            match byte
            {
                CONTROL_BYTE =>
                {
                    block!(serial_writer.write(CONTROL_BYTE))?;
                    block!(serial_writer.write(CONTROL_BYTE))?;
                }
                _ =>
                {
                    block!(serial_writer.write(byte))?;
                }
            }

            Ok(())
        });

    loop
    {
        match block!(raw_serial_reader.read()).unwrap_infallible()
        {
            CONTROL_BYTE =>
            {
                match block!(raw_serial_reader.read()).unwrap_infallible()
                {
                    START_TEXT_BYTE => (),
                    CONTROL_BYTE => continue,
                    _ =>
                    {
                        infallible_scope(|| unsafe
                        {
                            block!(raw_serial_writer.write(CONTROL_BYTE))?;
                            block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                            writer.write_type(Type::Compound)?;
                            writer.write_name("error")?;
                            {
                                writer.write_type(Type::String)?;
                                writer.write_name("type")?;
                                writer.write_string(
                                    "control-char")?;

                                writer.write_type(Type::String)?;
                                writer.write_name("message")?;
                                writer.write_string(
                                    "Invalid control char received.")?;

                                writer.write_end()?;
                            }

                            Ok(())
                        });
                        continue;
                    },
                }
            }
            _ => continue,
        }

        enum Response
        {
            GetConfig,
            GetPin
            {
                pin: &'static str,
                state: u8,
            },
        }

        let mut response: Option<Response> = None;

        if let Err(error) = try_scope(|| unsafe
        {
            match collect_type_and_name::<16>(&mut reader, "")
                .as_ref().map(|(tag, name)| (tag, name.as_str()))?
            {
                // MARK: /get-config
                (Type::Compound, "get-config") =>
                {
                    match collect_type_or_end_and_name::<8>(&mut reader, "get-config/")
                        .as_ref().map(|x| x.as_ref().map(|(tag, name)| (tag, name.as_str())))?
                    {
                        None => (),
                        _ => return Err(ReadError::InvalidTag
                        {
                            message: "Invalid field.",
                            path: "get-config/",
                        })
                    }

                    // MARK: .   +get-config
                    response = Some(Response::GetConfig);
                }
                // MARK: /set-pin
                (Type::Compound, "set-pin") =>
                {
                    let mut pin: Option<InteractablePinId> = None;
                    let mut state: Option<u8> = None;

                    match collect_type_or_end_and_name::<8>(&mut reader, "set-pin/")
                        .as_ref().map(|x| x.as_ref().map(|(tag, name)| (tag, name.as_str())))?
                    {
                        None => (),
                        // MARK: .   /pin
                        Some((Type::String, "pin")) => match collect_string::<8>(&mut reader, "")
                            .as_mut().map(|string| { string.make_ascii_uppercase(); string.as_str() })?
                        {
                            "D2" => pin = Some(InteractablePinId::D2),
                            "D3" => pin = Some(InteractablePinId::D3),
                            "D4" => pin = Some(InteractablePinId::D4),
                            "D5" => pin = Some(InteractablePinId::D5),
                            "D6" => pin = Some(InteractablePinId::D6),
                            "D7" => pin = Some(InteractablePinId::D7),
                            "D8" => pin = Some(InteractablePinId::D8),
                            "D9" => pin = Some(InteractablePinId::D9),
                            "D10" => pin = Some(InteractablePinId::D10),
                            "D11" => pin = Some(InteractablePinId::D11),
                            "D12" => pin = Some(InteractablePinId::D12),
                            "D13" => pin = Some(InteractablePinId::D13),
                            "A0" => pin = Some(InteractablePinId::A0),
                            "A1" => pin = Some(InteractablePinId::A1),
                            "A2" => pin = Some(InteractablePinId::A2),
                            "A3" => pin = Some(InteractablePinId::A3),
                            "A4" => pin = Some(InteractablePinId::A4),
                            "A5" => pin = Some(InteractablePinId::A5),
                            _ => return Err(ReadError::InvalidTag
                            {
                                message: "Invalid pin.",
                                path: "set-pin/pin",
                            })
                        }
                        // MARK: .   /state
                        Some((Type::Byte, "state")) =>
                        {
                            state = Some(reader.read_ubyte()?);
                        }
                        _ => return Err(ReadError::InvalidTag
                        {
                            message: "Invalid field.",
                            path: "set-pin/",
                        })
                    }

                    const INPUT_MODE_ERROR: ReadError = ReadError::InvalidTag
                    {
                        message: "Specified pin is in input mode.",
                        path: "set-pin",
                    };

                    // MARK: .   ->
                    match (pin, state)
                    {
                        (Some(InteractablePinId::D2), Some(state)) => match (&mut interactive_d2, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D3), Some(state)) => match (&mut interactive_d3, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D4), Some(state)) => match (&mut interactive_d4, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D5), Some(state)) => match (&mut interactive_d5, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D6), Some(state)) => match (&mut interactive_d6, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D7), Some(state)) => match (&mut interactive_d7, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D8), Some(state)) => match (&mut interactive_d8, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D9), Some(state)) => match (&mut interactive_d9, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D10), Some(state)) => match (&mut interactive_d10, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D11), Some(state)) => match (&mut interactive_d11, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D12), Some(state)) => match (&mut interactive_d12, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::D13), Some(state)) => match (&mut interactive_d13, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::A0), Some(state)) => match (&mut interactive_a0, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::A1), Some(state)) => match (&mut interactive_a1, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::A2), Some(state)) => match (&mut interactive_a2, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::A3), Some(state)) => match (&mut interactive_a3, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::A4), Some(state)) => match (&mut interactive_a4, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (Some(InteractablePinId::A5), Some(state)) => match (&mut interactive_a5, state)
                        {
                            (InteractablePin::DigitalOutput(pin), 0) => pin.set_low(),
                            (InteractablePin::DigitalOutput(pin), _) => pin.set_high(),
                            _ => return Err(INPUT_MODE_ERROR),
                        }
                        (None, _) => return Err(ReadError::InvalidTag
                        {
                            message: "Missing 'pin' field.",
                            path: "set-pin/",
                        }),
                        (_, None) => return Err(ReadError::InvalidTag
                        {
                            message: "Missing 'state' field.",
                            path: "set-pin/",
                        }),
                    }
                }
                // MARK: /get-pin
                (Type::Compound, "get-pin") =>
                {
                    let mut pin: Option<InteractablePinId> = None;

                    match collect_type_or_end_and_name::<8>(&mut reader, "get-pin/")
                        .as_ref().map(|x| x.as_ref().map(|(tag, name)| (tag, name.as_str())))?
                    {
                        None => (),
                        // MARK: .   /pin
                        Some((Type::String, "pin")) => match collect_string::<8>(&mut reader, "")
                            .as_mut().map(|string| { string.make_ascii_uppercase(); string.as_str() })?
                        {
                            "D2" => pin = Some(InteractablePinId::D2),
                            "D3" => pin = Some(InteractablePinId::D3),
                            "D4" => pin = Some(InteractablePinId::D4),
                            "D5" => pin = Some(InteractablePinId::D5),
                            "D6" => pin = Some(InteractablePinId::D6),
                            "D7" => pin = Some(InteractablePinId::D7),
                            "D8" => pin = Some(InteractablePinId::D8),
                            "D9" => pin = Some(InteractablePinId::D9),
                            "D10" => pin = Some(InteractablePinId::D10),
                            "D11" => pin = Some(InteractablePinId::D11),
                            "D12" => pin = Some(InteractablePinId::D12),
                            "D13" => pin = Some(InteractablePinId::D13),
                            "A0" => pin = Some(InteractablePinId::A0),
                            "A1" => pin = Some(InteractablePinId::A1),
                            "A2" => pin = Some(InteractablePinId::A2),
                            "A3" => pin = Some(InteractablePinId::A3),
                            "A4" => pin = Some(InteractablePinId::A4),
                            "A5" => pin = Some(InteractablePinId::A5),
                            _ => return Err(ReadError::InvalidTag
                            {
                                message: "Invalid pin.",
                                path: "get-pin/pin",
                            })
                        }
                        _ => return Err(ReadError::InvalidTag
                        {
                            message: "Invalid field.",
                            path: "get-pin/",
                        })
                    }

                    let state;

                    // MARK: .   ->
                    match pin
                    {
                        Some(InteractablePinId::D2) => match &interactive_d2
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D3) => match &interactive_d3
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D4) => match &interactive_d4
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D5) => match &interactive_d5
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D6) => match &interactive_d6
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D7) => match &interactive_d7
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D8) => match &interactive_d8
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D9) => match &interactive_d9
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D10) => match &interactive_d10
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D11) => match &interactive_d11
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D12) => match &interactive_d12
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::D13) => match &interactive_d13
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::A0) => match &interactive_a0
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::A1) => match &interactive_a1
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::A2) => match &interactive_a2
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::A3) => match &interactive_a3
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::A4) => match &interactive_a4
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        Some(InteractablePinId::A5) => match &interactive_a5
                        {
                            InteractablePin::DigitalInput(pin) => state = if pin.is_high() { 0xFF } else { 0 },
                            InteractablePin::DigitalOutput(pin) => state = if pin.is_set_high() { 0xFF } else { 0 },
                        }
                        None => return Err(ReadError::InvalidTag
                        {
                            message: "Missing 'pin' field.",
                            path: "get-pin/",
                        }),
                    }

                    // MARK: .   +get-pin
                    response = Some(Response::GetPin
                    {
                        pin: match pin
                        {
                            None => "",
                            Some(InteractablePinId::D2) => "D2",
                            Some(InteractablePinId::D3) => "D3",
                            Some(InteractablePinId::D4) => "D4",
                            Some(InteractablePinId::D5) => "D5",
                            Some(InteractablePinId::D6) => "D6",
                            Some(InteractablePinId::D7) => "D7",
                            Some(InteractablePinId::D8) => "D8",
                            Some(InteractablePinId::D9) => "D9",
                            Some(InteractablePinId::D10) => "D10",
                            Some(InteractablePinId::D11) => "D11",
                            Some(InteractablePinId::D12) => "D12",
                            Some(InteractablePinId::D13) => "D13",
                            Some(InteractablePinId::A0) => "A0",
                            Some(InteractablePinId::A1) => "A1",
                            Some(InteractablePinId::A2) => "A2",
                            Some(InteractablePinId::A3) => "A3",
                            Some(InteractablePinId::A4) => "A4",
                            Some(InteractablePinId::A5) => "A5",
                        },
                        state,
                    });
                }
                // MARK: /set-pin-mode
                (Type::Compound, "set-pin-mode") =>
                {
                    enum Mode
                    {
                        DigitalInput,
                        DigitalOutput,
                        AnalogInput,
                        AnalogOutput,
                    }

                    let mut pin: Option<InteractablePinId> = None;
                    let mut mode: Option<Mode> = None;

                    match collect_type_or_end_and_name::<8>(&mut reader, "set-pin-mode/")
                        .as_ref().map(|x| x.as_ref().map(|(tag, name)| (tag, name.as_str())))?
                    {
                        None => (),
                        // MARK: .   /pin
                        Some((Type::String, "pin")) => match collect_string::<8>(&mut reader, "")
                            .as_mut().map(|string| { string.make_ascii_uppercase(); string.as_str() })?
                        {
                            "D2" => pin = Some(InteractablePinId::D2),
                            "D3" => pin = Some(InteractablePinId::D3),
                            "D4" => pin = Some(InteractablePinId::D4),
                            "D5" => pin = Some(InteractablePinId::D5),
                            "D6" => pin = Some(InteractablePinId::D6),
                            "D7" => pin = Some(InteractablePinId::D7),
                            "D8" => pin = Some(InteractablePinId::D8),
                            "D9" => pin = Some(InteractablePinId::D9),
                            "D10" => pin = Some(InteractablePinId::D10),
                            "D11" => pin = Some(InteractablePinId::D11),
                            "D12" => pin = Some(InteractablePinId::D12),
                            "D13" => pin = Some(InteractablePinId::D13),
                            "A0" => pin = Some(InteractablePinId::A0),
                            "A1" => pin = Some(InteractablePinId::A1),
                            "A2" => pin = Some(InteractablePinId::A2),
                            "A3" => pin = Some(InteractablePinId::A3),
                            "A4" => pin = Some(InteractablePinId::A4),
                            "A5" => pin = Some(InteractablePinId::A5),
                            _ => return Err(ReadError::InvalidTag
                            {
                                message: "Invalid pin.",
                                path: "set-pin-mode/pin",
                            })
                        }
                        // MARK: .   /mode
                        Some((Type::String, "mode")) => match collect_string::<16>(&mut reader, "")
                            .as_mut().map(|string| { string.make_ascii_uppercase(); string.as_str() })?
                        {
                            "input" => mode = Some(Mode::DigitalInput),
                            "output" => mode = Some(Mode::DigitalOutput),
                            "digital-input" => mode = Some(Mode::DigitalInput),
                            "digital-output" => mode = Some(Mode::DigitalOutput),
                            "analog-input" => mode = Some(Mode::AnalogInput),
                            "analog-output" => mode = Some(Mode::AnalogOutput),
                            _ => return Err(ReadError::InvalidTag
                            {
                                message: "Invalid mode.",
                                path: "set-pin-mode/mode",
                            })
                        }
                        _ => return Err(ReadError::InvalidTag
                        {
                            message: "Invalid field.",
                            path: "set-pin-mode/",
                        })
                    }

                    const INPUT_ANALOG_ERROR: ReadError = ReadError::StateError
                    {
                        message: "This config does not support analog input.",
                        path: "set-pin-mode",
                    };
                    const OUTPUT_ANALOG_ERROR: ReadError = ReadError::StateError
                    {
                        message: "This config does not support analog output.",
                        path: "set-pin-mode",
                    };

                    // MARK: .   ->
                    match (pin, mode)
                    {
                        (Some(InteractablePinId::D2), Some(mode)) => match (&mut interactive_d2, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d2 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d2 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D3), Some(mode)) => match (&mut interactive_d3, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d3 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d3 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D4), Some(mode)) => match (&mut interactive_d4, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d4 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d4 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D5), Some(mode)) => match (&mut interactive_d5, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d5 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d5 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D6), Some(mode)) => match (&mut interactive_d6, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d6 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d6 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D7), Some(mode)) => match (&mut interactive_d7, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d7 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d7 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D8), Some(mode)) => match (&mut interactive_d8, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d8 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d8 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D9), Some(mode)) => match (&mut interactive_d9, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d9 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d9 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D10), Some(mode)) => match (&mut interactive_d10, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d10 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d10 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D11), Some(mode)) => match (&mut interactive_d11, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d11 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d11 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D12), Some(mode)) => match (&mut interactive_d12, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d12 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d12 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::D13), Some(mode)) => match (&mut interactive_d13, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_d13 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_d13 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::A0), Some(mode)) => match (&mut interactive_a0, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_a0 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_a0 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::A1), Some(mode)) => match (&mut interactive_a1, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_a1 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_a1 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::A2), Some(mode)) => match (&mut interactive_a2, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_a2 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_a2 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::A3), Some(mode)) => match (&mut interactive_a3, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_a3 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_a3 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::A4), Some(mode)) => match (&mut interactive_a4, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_a4 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_a4 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (Some(InteractablePinId::A5), Some(mode)) => match (&mut interactive_a5, mode)
                        {
                            (InteractablePin::DigitalInput(_), Mode::DigitalInput) => (),
                            (InteractablePin::DigitalInput(pin), Mode::DigitalOutput) => interactive_a5 = InteractablePin::DigitalOutput(core::ptr::read(pin).into_output()),
                            (InteractablePin::DigitalOutput(_), Mode::DigitalOutput) => (),
                            (InteractablePin::DigitalOutput(pin), Mode::DigitalInput) => interactive_a5 = InteractablePin::DigitalInput(core::ptr::read(pin).into_pull_up_input()),
                            (_, Mode::AnalogInput) => return Err(INPUT_ANALOG_ERROR),
                            (_, Mode::AnalogOutput) => return Err(OUTPUT_ANALOG_ERROR),
                        },
                        (None, _) => return Err(ReadError::InvalidTag
                        {
                            message: "Missing 'pin' field.",
                            path: "set-pin-mode/",
                        }),
                        (_, None) => return Err(ReadError::InvalidTag
                        {
                            message: "Missing 'state' field.",
                            path: "set-pin-mode/",
                        }),
                    }
                }
                _ => return Err(ReadError::InvalidTag
                {
                    message: "Invalid root tag.",
                    path: "",
                })
            }

            // MARK: Response
            if let Some(response) = response.take()
            {
                infallible_scope(||
                {
                    match response
                    {
                        Response::GetPin { pin, state } =>
                        {
                            block!(raw_serial_writer.write(CONTROL_BYTE))?;
                            block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                            writer.write_type(Type::Compound)?;
                            writer.write_name("+get-pin")?;
                            {
                                writer.write_type(Type::String)?;
                                writer.write_name("pin")?;
                                writer.write_string(pin)?;

                                writer.write_type(Type::Byte)?;
                                writer.write_name("state")?;
                                writer.write_ubyte(state)?;

                                writer.write_end()?;
                            }
                        },
                        Response::GetConfig =>
                        {
                            block!(raw_serial_writer.write(CONTROL_BYTE))?;
                            block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                            writer.write_type(Type::Compound)?;
                            writer.write_name("+get-config")?;
                            {
                                writer.write_type(Type::List)?;
                                writer.write_name("pins")?;
                                writer.write_element_type(ElementType::Compound)?;
                                writer.write_len(18)?;
                                {
                                    let mut write_pin = |
                                        name: &str,
                                        display: &str,
                                        modes: &[&str]| -> Result<(), Infallible>
                                    {
                                        writer.write_type(Type::String)?;
                                        writer.write_name("name")?;
                                        writer.write_string(name)?;

                                        writer.write_type(Type::String)?;
                                        writer.write_name("display")?;
                                        writer.write_string(display)?;

                                        writer.write_type(Type::List)?;
                                        writer.write_name("modes")?;
                                        writer.write_element_type(ElementType::String)?;
                                        writer.write_len(modes.len() as u32)?;
                                        for mode in modes
                                        {
                                            writer.write_string(mode)?;
                                        }

                                        Ok(())
                                    };

                                    write_pin("D2", "2", &["digital-input", "digital-output"])?;
                                    write_pin("D3", "~3", &["digital-input", "digital-output"])?;
                                    write_pin("D4", "4", &["digital-input", "digital-output"])?;
                                    write_pin("D5", "~5", &["digital-input", "digital-output"])?;
                                    write_pin("D6", "~6", &["digital-input", "digital-output"])?;
                                    write_pin("D7", "7", &["digital-input", "digital-output"])?;

                                    write_pin("D8", "8", &["digital-input", "digital-output"])?;
                                    write_pin("D9", "~9", &["digital-input", "digital-output"])?;
                                    write_pin("D10", "~10", &["digital-input", "digital-output"])?;
                                    write_pin("D11", "~11", &["digital-input", "digital-output"])?;
                                    write_pin("D12", "12", &["digital-input", "digital-output"])?;
                                    write_pin("D13", "13", &["digital-input", "digital-output"])?;

                                    write_pin("A0", "A0", &["digital-input", "digital-output"])?;
                                    write_pin("A1", "A1", &["digital-input", "digital-output"])?;
                                    write_pin("A2", "A2", &["digital-input", "digital-output"])?;
                                    write_pin("A3", "A3", &["digital-input", "digital-output"])?;
                                    write_pin("A4", "A4", &["digital-input", "digital-output"])?;
                                    write_pin("A5", "A5", &["digital-input", "digital-output"])?;

                                    writer.write_end()?;
                                }

                                writer.write_end()?;
                            }
                        }
                    }

                    Ok(())
                });
            }

            Ok(())
        })
        {
            infallible_scope(|| unsafe
            {
                match error
                {
                    ReadError::FoundControlByte(byte) =>
                    {
                        block!(raw_serial_writer.write(CONTROL_BYTE))?;
                        block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                        writer.write_type(Type::Compound)?;
                        writer.write_name("error")?;
                        {
                            writer.write_type(Type::String)?;
                            writer.write_name("type")?;
                            writer.write_string(
                                "control-char")?;

                            writer.write_type(Type::String)?;
                            writer.write_name("message")?;
                            writer.write_string(
                                "Control char received mid-message.")?;

                            writer.write_type(Type::Byte)?;
                            writer.write_name("byte")?;
                            writer.write_ubyte(byte)?;

                            writer.write_end()?;
                        }
                    },
                    ReadError::TimedOut =>
                    {
                        block!(raw_serial_writer.write(CONTROL_BYTE))?;
                        block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                        writer.write_type(Type::Compound)?;
                        writer.write_name("error")?;
                        {
                            writer.write_type(Type::String)?;
                            writer.write_name("type")?;
                            writer.write_string(
                                "timeout")?;

                            writer.write_type(Type::String)?;
                            writer.write_name("message")?;
                            writer.write_string(
                                "Message reading timed out.")?;

                            writer.write_end()?;
                        }
                    },
                    ReadError::InvalidTag { message, path } =>
                    {
                        block!(raw_serial_writer.write(CONTROL_BYTE))?;
                        block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                        writer.write_type(Type::Compound)?;
                        writer.write_name("error")?;
                        {
                            writer.write_type(Type::String)?;
                            writer.write_name("type")?;
                            writer.write_string(
                                "invalid-tag")?;

                            writer.write_type(Type::String)?;
                            writer.write_name("message")?;
                            writer.write_string(message)?;

                            writer.write_type(Type::String)?;
                            writer.write_name("path")?;
                            writer.write_string(path)?;

                            writer.write_end()?;
                        }
                    },
                    ReadError::StateError { message, path } =>
                    {
                        block!(raw_serial_writer.write(CONTROL_BYTE))?;
                        block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                        writer.write_type(Type::Compound)?;
                        writer.write_name("error")?;
                        {
                            writer.write_type(Type::String)?;
                            writer.write_name("type")?;
                            writer.write_string(
                                "invalid-command")?;

                            writer.write_type(Type::String)?;
                            writer.write_name("message")?;
                            writer.write_string(message)?;

                            writer.write_type(Type::String)?;
                            writer.write_name("path")?;
                            writer.write_string(path)?;

                            writer.write_end()?;
                        }
                    },
                }

                Ok(())
            });
            continue;
        }
    }
}

fn infallible_scope(f: impl FnOnce() -> Result<(), Infallible>)
{
    f().unwrap_infallible()
}

fn try_scope<E>(f: impl FnOnce() -> Result<(), E>) -> Result<(), E>
{
    f()
}

unsafe fn collect_type_or_end_and_name<const N: usize>(
    reader: &mut impl ReadRaw<Error = ReadError>,
    error_path: &'static str)
    -> Result<Option<(Type, heapless::String<N>)>, ReadError>
{
    let tag = match reader.read_type_or_end()?
    {
        Ok(Some(tag)) => tag,
        Ok(None) => return Ok(None),
        Err(error) =>
        {
            reader.discard(error.byte_count)?;
            return Err(ReadError::InvalidTag
            {
                message: "Type is not valid.",
                path: error_path,
            });
        },
    };

    Ok(Some((tag, collect_name(reader, error_path)?)))
}

unsafe fn collect_type_and_name<const N: usize>(
    reader: &mut impl ReadRaw<Error = ReadError>,
    error_path: &'static str)
    -> Result<(Type, heapless::String<N>), ReadError>
{
    let tag = match reader.read_type()?
    {
        Ok(tag) => tag,
        Err(error) =>
        {
            reader.discard(error.byte_count)?;
            return Err(ReadError::InvalidTag
            {
                message: "Type is not valid.",
                path: error_path,
            });
        },
    };

    Ok((tag, collect_name(reader, error_path)?))
}

unsafe fn collect_name<const N: usize>(
    reader: &mut impl ReadRaw<Error = ReadError>,
    error_path: &'static str)
    -> Result<heapless::String<N>, ReadError>
{
    let len = reader.read_ushort()?;

    if len > N as u16
    {
        return Err(ReadError::InvalidTag
        {
            message: "Name is too long.",
            path: error_path,
        });
    }

    let mut bytes = heapless::Vec::<u8, N>::new();

    for _ in 0..len
    {
        let Ok(()) = bytes.push(reader.read_ubyte()?)
        else
        {
            return Err(ReadError::InvalidTag
            {
                message: "Name is too long.",
                path: error_path,
            });
        };
    }

    match heapless::String::from_utf8(bytes)
    {
        Ok(utf8) => Ok(utf8),
        Err(_) => return Err(ReadError::InvalidTag
        {
            message: "Name is not valid UTF-8.",
            path: error_path,
        }),
    }
}

unsafe fn collect_string<const N: usize>(
    reader: &mut impl ReadRaw<Error = ReadError>,
    error_path: &'static str)
    -> Result<heapless::String<N>, ReadError>
{
    let len = reader.read_ushort()?;

    if len > N as u16
    {
        return Err(ReadError::InvalidTag
        {
            message: "String is too long.",
            path: error_path,
        });
    }

    let mut bytes = heapless::Vec::<u8, N>::new();

    for _ in 0..len
    {
        let Ok(()) = bytes.push(reader.read_ubyte()?)
        else
        {
            return Err(ReadError::InvalidTag
            {
                message: "String is too long.",
                path: error_path,
            })
        };
    }

    match heapless::String::from_utf8(bytes)
    {
        Ok(utf8) => Ok(utf8),
        Err(_) => return Err(ReadError::InvalidTag
        {
            message: "String is not valid UTF-8.",
            path: error_path,
        }),
    }
}