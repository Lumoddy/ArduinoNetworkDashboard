#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(abi_avr_interrupt)]

use arduino_hal::clock::Clock;
use arduino_hal::hal::port;
use arduino_hal::prelude::_embedded_hal_serial_Read;
use arduino_hal::prelude::_embedded_hal_serial_Write;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::DefaultClock;
use interactive::PinDigitalInteraction;
use interactive::PinDigitalInteractionChanges;
use interactive::PinModeInteraction;
use nb::block;
use nbt::ReadRaw;
use nbt::WriteRaw;
mod panic_handler;
mod nbt;
mod interactive;

use core::convert::Infallible;
use core::panic::Location;

impl interactive::PinMode
{
    const AS_STRING_PREFERRED_CAPACITY: usize = 16;
}

impl TryFrom<&str> for interactive::PinMode
{
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error>
    {
        match value
        {
            "input" | "digital-input" => Ok(interactive::PinMode::DigitalInput),
            "output" | "digital-output" => Ok(interactive::PinMode::DigitalOutput),
            _ => Err(()),
        }
    }
}

impl From<interactive::PinMode> for &'static str
{
    fn from(value: interactive::PinMode) -> Self
    {
        match value
        {
            interactive::PinMode::DigitalInput => "digital-input",
            interactive::PinMode::DigitalOutput => "digital-output",
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! { process() }

#[derive(Clone, Copy)]
pub enum InteractivePinID
{
    D2, D3, D4, D5, D6, D7,
    D8, D9, D10, D11, D12, D13,
    A0, A1, A2, A3, A4, A5,
}

impl TryFrom<u8> for InteractivePinID
{
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error>
    {
        match value
        {
            0 => Ok(InteractivePinID::D2),
            1 => Ok(InteractivePinID::D3),
            2 => Ok(InteractivePinID::D4),
            3 => Ok(InteractivePinID::D5),
            4 => Ok(InteractivePinID::D6),
            5 => Ok(InteractivePinID::D7),
            6 => Ok(InteractivePinID::D8),
            7 => Ok(InteractivePinID::D9),
            8 => Ok(InteractivePinID::D10),
            9 => Ok(InteractivePinID::D11),
            10 => Ok(InteractivePinID::D12),
            11 => Ok(InteractivePinID::D13),
            12 => Ok(InteractivePinID::A0),
            13 => Ok(InteractivePinID::A1),
            14 => Ok(InteractivePinID::A2),
            15 => Ok(InteractivePinID::A3),
            16 => Ok(InteractivePinID::A4),
            17 => Ok(InteractivePinID::A5),
            _ => Err(()),
        }
    }
}

impl From<InteractivePinID> for u8
{
    fn from(value: InteractivePinID) -> Self
    {
        match value
        {
            InteractivePinID::D2 => 0,
            InteractivePinID::D3 => 1,
            InteractivePinID::D4 => 2,
            InteractivePinID::D5 => 3,
            InteractivePinID::D6 => 4,
            InteractivePinID::D7 => 5,
            InteractivePinID::D8 => 6,
            InteractivePinID::D9 => 7,
            InteractivePinID::D10 => 8,
            InteractivePinID::D11 => 9,
            InteractivePinID::D12 => 10,
            InteractivePinID::D13 => 11,
            InteractivePinID::A0 => 12,
            InteractivePinID::A1 => 13,
            InteractivePinID::A2 => 14,
            InteractivePinID::A3 => 15,
            InteractivePinID::A4 => 16,
            InteractivePinID::A5 => 17,
        }
    }
}

pub struct InteractivePins
{
    d2: interactive::Pin<port::PD2>,
    d3: interactive::Pin<port::PD3>,
    d4: interactive::Pin<port::PD4>,
    d5: interactive::Pin<port::PD5>,
    d6: interactive::Pin<port::PD6>,
    d7: interactive::Pin<port::PD7>,
    d8: interactive::Pin<port::PB0>,
    d9: interactive::Pin<port::PB1>,
    d10: interactive::Pin<port::PB2>,
    d11: interactive::Pin<port::PB3>,
    d12: interactive::Pin<port::PB4>,
    d13: interactive::Pin<port::PB5>,
    a0: interactive::Pin<port::PC0>,
    a1: interactive::Pin<port::PC1>,
    a2: interactive::Pin<port::PC2>,
    a3: interactive::Pin<port::PC3>,
    a4: interactive::Pin<port::PC4>,
    a5: interactive::Pin<port::PC5>,
}

pub struct PickedInteractivePin<'p>
{
    pub all_pins: &'p mut InteractivePins,
    pub pin_id: InteractivePinID,
}

impl<'p> interactive::PinDigitalInteraction for PickedInteractivePin<'p>
{
    fn get_pin_is_high(&self) -> Result<bool, interactive::PinGetPowerError>
    {
        match self.pin_id
        {
            InteractivePinID::D2 => self.all_pins.d2.get_pin_is_high(),
            InteractivePinID::D3 => self.all_pins.d3.get_pin_is_high(),
            InteractivePinID::D4 => self.all_pins.d4.get_pin_is_high(),
            InteractivePinID::D5 => self.all_pins.d5.get_pin_is_high(),
            InteractivePinID::D6 => self.all_pins.d6.get_pin_is_high(),
            InteractivePinID::D7 => self.all_pins.d7.get_pin_is_high(),
            InteractivePinID::D8 => self.all_pins.d8.get_pin_is_high(),
            InteractivePinID::D9 => self.all_pins.d9.get_pin_is_high(),
            InteractivePinID::D10 => self.all_pins.d10.get_pin_is_high(),
            InteractivePinID::D11 => self.all_pins.d11.get_pin_is_high(),
            InteractivePinID::D12 => self.all_pins.d12.get_pin_is_high(),
            InteractivePinID::D13 => self.all_pins.d13.get_pin_is_high(),
            InteractivePinID::A0 => self.all_pins.a0.get_pin_is_high(),
            InteractivePinID::A1 => self.all_pins.a1.get_pin_is_high(),
            InteractivePinID::A2 => self.all_pins.a2.get_pin_is_high(),
            InteractivePinID::A3 => self.all_pins.a3.get_pin_is_high(),
            InteractivePinID::A4 => self.all_pins.a4.get_pin_is_high(),
            InteractivePinID::A5 => self.all_pins.a5.get_pin_is_high(),
        }
    }

    fn set_pin_is_high(&mut self, power: bool) -> Result<(), interactive::PinSetPowerError>
    {
        match self.pin_id
        {
            InteractivePinID::D2 => self.all_pins.d2.set_pin_is_high(power),
            InteractivePinID::D3 => self.all_pins.d3.set_pin_is_high(power),
            InteractivePinID::D4 => self.all_pins.d4.set_pin_is_high(power),
            InteractivePinID::D5 => self.all_pins.d5.set_pin_is_high(power),
            InteractivePinID::D6 => self.all_pins.d6.set_pin_is_high(power),
            InteractivePinID::D7 => self.all_pins.d7.set_pin_is_high(power),
            InteractivePinID::D8 => self.all_pins.d8.set_pin_is_high(power),
            InteractivePinID::D9 => self.all_pins.d9.set_pin_is_high(power),
            InteractivePinID::D10 => self.all_pins.d10.set_pin_is_high(power),
            InteractivePinID::D11 => self.all_pins.d11.set_pin_is_high(power),
            InteractivePinID::D12 => self.all_pins.d12.set_pin_is_high(power),
            InteractivePinID::D13 => self.all_pins.d13.set_pin_is_high(power),
            InteractivePinID::A0 => self.all_pins.a0.set_pin_is_high(power),
            InteractivePinID::A1 => self.all_pins.a1.set_pin_is_high(power),
            InteractivePinID::A2 => self.all_pins.a2.set_pin_is_high(power),
            InteractivePinID::A3 => self.all_pins.a3.set_pin_is_high(power),
            InteractivePinID::A4 => self.all_pins.a4.set_pin_is_high(power),
            InteractivePinID::A5 => self.all_pins.a5.set_pin_is_high(power),
        }
    }
}

impl<'p> interactive::PinDigitalInteractionChanges for PickedInteractivePin<'p>
{
    fn detect_pin_change(&mut self) -> Result<Option<bool>, interactive::PinPowerChangeError>
    {
        match self.pin_id
        {
            InteractivePinID::D2 => self.all_pins.d2.detect_pin_change(),
            InteractivePinID::D3 => self.all_pins.d3.detect_pin_change(),
            InteractivePinID::D4 => self.all_pins.d4.detect_pin_change(),
            InteractivePinID::D5 => self.all_pins.d5.detect_pin_change(),
            InteractivePinID::D6 => self.all_pins.d6.detect_pin_change(),
            InteractivePinID::D7 => self.all_pins.d7.detect_pin_change(),
            InteractivePinID::D8 => self.all_pins.d8.detect_pin_change(),
            InteractivePinID::D9 => self.all_pins.d9.detect_pin_change(),
            InteractivePinID::D10 => self.all_pins.d10.detect_pin_change(),
            InteractivePinID::D11 => self.all_pins.d11.detect_pin_change(),
            InteractivePinID::D12 => self.all_pins.d12.detect_pin_change(),
            InteractivePinID::D13 => self.all_pins.d13.detect_pin_change(),
            InteractivePinID::A0 => self.all_pins.a0.detect_pin_change(),
            InteractivePinID::A1 => self.all_pins.a1.detect_pin_change(),
            InteractivePinID::A2 => self.all_pins.a2.detect_pin_change(),
            InteractivePinID::A3 => self.all_pins.a3.detect_pin_change(),
            InteractivePinID::A4 => self.all_pins.a4.detect_pin_change(),
            InteractivePinID::A5 => self.all_pins.a5.detect_pin_change(),
        }
    }
}

impl<'p> interactive::PinModeInteraction for PickedInteractivePin<'p>
{
    fn get_pin_mode(&self) -> Result<interactive::PinMode, interactive::PinGetModeError>
    {
        match self.pin_id
        {
            InteractivePinID::D2 => self.all_pins.d2.get_pin_mode(),
            InteractivePinID::D3 => self.all_pins.d3.get_pin_mode(),
            InteractivePinID::D4 => self.all_pins.d4.get_pin_mode(),
            InteractivePinID::D5 => self.all_pins.d5.get_pin_mode(),
            InteractivePinID::D6 => self.all_pins.d6.get_pin_mode(),
            InteractivePinID::D7 => self.all_pins.d7.get_pin_mode(),
            InteractivePinID::D8 => self.all_pins.d8.get_pin_mode(),
            InteractivePinID::D9 => self.all_pins.d9.get_pin_mode(),
            InteractivePinID::D10 => self.all_pins.d10.get_pin_mode(),
            InteractivePinID::D11 => self.all_pins.d11.get_pin_mode(),
            InteractivePinID::D12 => self.all_pins.d12.get_pin_mode(),
            InteractivePinID::D13 => self.all_pins.d13.get_pin_mode(),
            InteractivePinID::A0 => self.all_pins.a0.get_pin_mode(),
            InteractivePinID::A1 => self.all_pins.a1.get_pin_mode(),
            InteractivePinID::A2 => self.all_pins.a2.get_pin_mode(),
            InteractivePinID::A3 => self.all_pins.a3.get_pin_mode(),
            InteractivePinID::A4 => self.all_pins.a4.get_pin_mode(),
            InteractivePinID::A5 => self.all_pins.a5.get_pin_mode(),
        }
    }

    fn set_pin_mode(&mut self, mode: interactive::PinMode) -> Result<(), interactive::PinSetModeError>
    {
        match self.pin_id
        {
            InteractivePinID::D2 => self.all_pins.d2.set_pin_mode(mode),
            InteractivePinID::D3 => self.all_pins.d3.set_pin_mode(mode),
            InteractivePinID::D4 => self.all_pins.d4.set_pin_mode(mode),
            InteractivePinID::D5 => self.all_pins.d5.set_pin_mode(mode),
            InteractivePinID::D6 => self.all_pins.d6.set_pin_mode(mode),
            InteractivePinID::D7 => self.all_pins.d7.set_pin_mode(mode),
            InteractivePinID::D8 => self.all_pins.d8.set_pin_mode(mode),
            InteractivePinID::D9 => self.all_pins.d9.set_pin_mode(mode),
            InteractivePinID::D10 => self.all_pins.d10.set_pin_mode(mode),
            InteractivePinID::D11 => self.all_pins.d11.set_pin_mode(mode),
            InteractivePinID::D12 => self.all_pins.d12.set_pin_mode(mode),
            InteractivePinID::D13 => self.all_pins.d13.set_pin_mode(mode),
            InteractivePinID::A0 => self.all_pins.a0.set_pin_mode(mode),
            InteractivePinID::A1 => self.all_pins.a1.set_pin_mode(mode),
            InteractivePinID::A2 => self.all_pins.a2.set_pin_mode(mode),
            InteractivePinID::A3 => self.all_pins.a3.set_pin_mode(mode),
            InteractivePinID::A4 => self.all_pins.a4.set_pin_mode(mode),
            InteractivePinID::A5 => self.all_pins.a5.set_pin_mode(mode),
        }
    }
}

#[derive(Clone, Copy)]
enum ReadError
{
    FoundControlByte(u8),
    TimedOut,
    InvalidRequest
    {
        message: &'static str,
        path: &'static str,
        location: &'static Location<'static>,
    },
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

    let mut interactive_pins = InteractivePins
    {
        d2: pins.d2.into_pull_up_input().into(),
        d3: pins.d3.into_pull_up_input().into(),
        d4: pins.d4.into_pull_up_input().into(),
        d5: pins.d5.into_pull_up_input().into(),
        d6: pins.d6.into_pull_up_input().into(),
        d7: pins.d7.into_pull_up_input().into(),
        d8: pins.d8.into_pull_up_input().into(),
        d9: pins.d9.into_pull_up_input().into(),
        d10: pins.d10.into_pull_up_input().into(),
        d11: pins.d11.into_pull_up_input().into(),
        d12: pins.d12.into_pull_up_input().into(),
        d13: pins.d13.into_pull_up_input().into(),
        a0: pins.a0.into_pull_up_input().into(),
        a1: pins.a1.into_pull_up_input().into(),
        a2: pins.a2.into_pull_up_input().into(),
        a3: pins.a3.into_pull_up_input().into(),
        a4: pins.a4.into_pull_up_input().into(),
        a5: pins.a5.into_pull_up_input().into(),
    };

    // https://symbl.cc/en/unicode-table/
    const CONTROL_BYTE: u8 = b'';
    const START_TEXT_BYTE: u8 = b'';

    let mut reader = nbt::ClosureRawReader::<ReadError, _>::new(
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

    let mut writer = nbt::ClosureRawWriter::<Infallible, _>::new(
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
        unsafe
        {
            let mut write_change = |
                pin: InteractivePinID,
                now_is_high: bool|
            {
                infallible_scope(||
                {
                    block!(raw_serial_writer.write(CONTROL_BYTE))?;
                    block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                    writer.write_type(nbt::Type::Compound)?;
                    writer.write_name("pin-changed")?;
                    {
                        writer.write_type(nbt::Type::Byte)?;
                        writer.write_name("pin")?;
                        writer.write_ubyte(pin.into())?;

                        writer.write_type(nbt::Type::Byte)?;
                        writer.write_name("is-high")?;
                        writer.write_bool(now_is_high)?;

                        writer.write_end()?;
                    }

                    Ok(())
                });
            };

            let mut try_write_change = |pin: InteractivePinID|
            {
                if let Ok(Some(is_high)) = (PickedInteractivePin
                    {
                        all_pins: &mut interactive_pins,
                        pin_id: pin,
                    })
                    .detect_pin_change()
                {
                    write_change(pin, is_high)
                }
            };

            try_write_change(InteractivePinID::D2);
            try_write_change(InteractivePinID::D3);
            try_write_change(InteractivePinID::D4);
            try_write_change(InteractivePinID::D5);
            try_write_change(InteractivePinID::D6);
            try_write_change(InteractivePinID::D7);
            try_write_change(InteractivePinID::D8);
            try_write_change(InteractivePinID::D9);
            try_write_change(InteractivePinID::D10);
            try_write_change(InteractivePinID::D11);
            try_write_change(InteractivePinID::D12);
            try_write_change(InteractivePinID::D13);
            try_write_change(InteractivePinID::A0);
            try_write_change(InteractivePinID::A1);
            try_write_change(InteractivePinID::A2);
            try_write_change(InteractivePinID::A3);
            try_write_change(InteractivePinID::A4);
            try_write_change(InteractivePinID::A5);
        }

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
                            writer.write_type(nbt::Type::Compound)?;
                            writer.write_name("error")?;
                            {
                                writer.write_type(nbt::Type::String)?;
                                writer.write_name("type")?;
                                writer.write_string(
                                    "control-byte")?;

                                writer.write_type(nbt::Type::String)?;
                                writer.write_name("message")?;
                                writer.write_string(
                                    "Invalid control byte received.")?;

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
            GetPinOk
            {
                pin: InteractivePinID,
                is_high: bool,
            },
            GetPinError
            {
                pin: InteractivePinID,
                message: &'static str,
                location: &'static Location<'static>,
            },
            SetPinOk
            {
                pin: InteractivePinID,
            },
            SetPinError
            {
                pin: InteractivePinID,
                message: &'static str,
                location: &'static Location<'static>,
            },
            GetPinModeOk
            {
                pin: InteractivePinID,
                mode: &'static str,
            },
            GetPinModeError
            {
                pin: InteractivePinID,
                message: &'static str,
                location: &'static Location<'static>,
            },
            SetPinModeOk
            {
                pin: InteractivePinID,
            },
            SetPinModeError
            {
                pin: InteractivePinID,
                message: &'static str,
                location: &'static Location<'static>,
            },
        }

        match try_scope(|| unsafe
        {
            match collect_type_and_name::<16>(
                &mut reader,
                "",
                Location::caller())
                .as_ref().map(|(tag, name)| (tag, name.as_str()))?
            {
                // MARK: get-config
                (nbt::Type::Compound, "get-config") =>
                {
                    loop
                    {
                        match collect_type_or_end_and_name::<8>(
                            &mut reader,
                            "get-config/",
                            Location::caller())?
                            .as_ref().map(|(tag, name)| (tag, name.as_str()))
                        {
                            None => break,
                            _ => return Err(ReadError::InvalidRequest
                            {
                                message: "Invalid field.",
                                path: "get-config/",
                                location: Location::caller(),
                            })
                        }
                    }

                    Ok(Response::GetConfig)
                }
                // MARK: get-pin
                (nbt::Type::Compound, "get-pin") =>
                {
                    let mut pin: Option<InteractivePinID> = None;

                    loop
                    {
                        match collect_type_or_end_and_name::<8>(
                            &mut reader,
                            "get-pin/",
                            Location::caller())?
                            .as_ref().map(|(tag, name)| (tag, name.as_str()))
                        {
                            None => break,
                            Some((nbt::Type::Byte, "pin")) =>
                            {
                                match reader.read_ubyte()?.try_into()
                                {
                                    Ok(id) => pin = Some(id),
                                    Err(()) => return Err(ReadError::InvalidRequest
                                    {
                                        message: "Invalid pin.",
                                        path: "get-pin/pin",
                                        location: Location::caller(),
                                    })
                                }
                            },
                            _ => return Err(ReadError::InvalidRequest
                            {
                                message: "Invalid field.",
                                path: "get-pin/",
                                location: Location::caller(),
                            }),
                        }
                    }

                    match pin
                    {
                        Some(pin) =>
                        {
                            match (PickedInteractivePin
                            {
                                all_pins: &mut interactive_pins,
                                pin_id: pin,
                            })
                            .get_pin_is_high()
                            {
                                Ok(is_high) => Ok(Response::GetPinOk
                                {
                                    pin,
                                    is_high,
                                }),
                            }
                        },
                        None => return Err(ReadError::InvalidRequest
                        {
                            message: "Missing 'pin' field.",
                            path: "get-pin/",
                            location: Location::caller(),
                        }),
                    }
                }
                // MARK: set-pin
                (nbt::Type::Compound, "set-pin") =>
                {
                    let mut pin: Option<InteractivePinID> = None;
                    let mut is_high: Option<bool> = None;

                    loop
                    {
                        match collect_type_or_end_and_name::<8>(
                            &mut reader,
                            "set-pin/",
                            Location::caller())?
                            .as_ref().map(|(tag, name)| (tag, name.as_str()))
                        {
                            None => break,
                            Some((nbt::Type::Byte, "pin")) =>
                            {
                                match reader.read_ubyte()?.try_into()
                                {
                                    Ok(id) => pin = Some(id),
                                    Err(()) => return Err(ReadError::InvalidRequest
                                    {
                                        message: "Invalid pin.",
                                        path: "set-pin/pin",
                                        location: Location::caller(),
                                    })
                                }
                            },
                            Some((nbt::Type::Byte, "is-high")) =>
                            {
                                is_high = Some(reader.read_byte()? != 0);
                            },
                            _ => return Err(ReadError::InvalidRequest
                            {
                                message: "Invalid field.",
                                path: "set-pin/",
                                location: Location::caller(),
                            }),
                        }
                    }

                    match (pin, is_high)
                    {
                        (Some(pin), Some(is_high)) =>
                        {
                            match (PickedInteractivePin
                            {
                                all_pins: &mut interactive_pins,
                                pin_id: pin,
                            })
                            .set_pin_is_high(is_high)
                            {
                                Ok(()) => Ok(Response::SetPinOk
                                {
                                    pin,
                                }),
                                Err(interactive::PinSetPowerError::IsInput)
                                    => return Ok(Response::SetPinError
                                    {
                                        message: "Cannot set state of pin in input mode.",
                                        pin,
                                        location: Location::caller(),
                                    }),
                            }
                        },
                        (None, _) => return Err(ReadError::InvalidRequest
                        {
                            message: "Missing 'pin' field.",
                            path: "set-pin/",
                            location: Location::caller(),
                        }),
                        (_, None) => return Err(ReadError::InvalidRequest
                        {
                            message: "Missing 'is_high' field.",
                            path: "set-pin/",
                            location: Location::caller(),
                        }),
                    }
                }
                // MARK: get-pin-mode
                (nbt::Type::Compound, "get-pin-mode") =>
                {
                    let mut pin: Option<InteractivePinID> = None;

                    loop
                    {
                        match collect_type_or_end_and_name::<8>(
                            &mut reader,
                            "get-pin-mode/",
                            Location::caller())?
                            .as_ref().map(|(tag, name)| (tag, name.as_str()))
                        {
                            None => break,
                            Some((nbt::Type::Byte, "pin")) =>
                            {
                                match reader.read_ubyte()?.try_into()
                                {
                                    Ok(id) => pin = Some(id),
                                    Err(()) => return Err(ReadError::InvalidRequest
                                    {
                                        message: "Invalid pin.",
                                        path: "get-pin-mode/pin",
                                        location: Location::caller(),
                                    })
                                }
                            },
                            _ => return Err(ReadError::InvalidRequest
                            {
                                message: "Invalid field.",
                                path: "get-pin-mode/",
                                location: Location::caller(),
                            }),
                        }
                    }

                    match pin
                    {
                        Some(pin) =>
                        {
                            match (PickedInteractivePin
                            {
                                all_pins: &mut interactive_pins,
                                pin_id: pin,
                            })
                            .get_pin_mode()
                            {
                                Ok(mode) => Ok(Response::GetPinModeOk
                                {
                                    pin,
                                    mode: mode.into(),
                                }),
                            }
                        },
                        None => return Err(ReadError::InvalidRequest
                        {
                            message: "Missing 'pin' field.",
                            path: "get-pin-mode/",
                            location: Location::caller(),
                        }),
                    }
                }
                // MARK: set-pin-mode
                (nbt::Type::Compound, "set-pin-mode") =>
                {
                    let mut pin: Option<InteractivePinID> = None;
                    let mut mode: Option<interactive::PinMode> = None;

                    loop
                    {
                        match collect_type_or_end_and_name::<8>(
                            &mut reader,
                            "set-pin-mode/",
                            Location::caller())?
                            .as_ref().map(|(tag, name)| (tag, name.as_str()))
                        {
                            None => break,
                            Some((nbt::Type::Byte, "pin")) =>
                            {
                                match reader.read_ubyte()?.try_into()
                                {
                                    Ok(id) => pin = Some(id),
                                    Err(()) => return Err(ReadError::InvalidRequest
                                    {
                                        message: "Invalid pin.",
                                        path: "set-pin-mode/pin",
                                        location: Location::caller(),
                                    })
                                }
                            },
                            Some((nbt::Type::String, "mode")) =>
                            {
                                match collect_string::<{ interactive::PinMode::AS_STRING_PREFERRED_CAPACITY }>(
                                    &mut reader,
                                    "set-pin-mode/pin",
                                    Location::caller())?
                                    .as_str().try_into()
                                {
                                    Ok(id) => mode = Some(id),
                                    Err(()) => return Err(ReadError::InvalidRequest
                                    {
                                        message: "Invalid mode.",
                                        path: "set-pin-mode/mode",
                                        location: Location::caller(),
                                    })
                                }
                            },
                            _ => return Err(ReadError::InvalidRequest
                            {
                                message: "Invalid field.",
                                path: "set-pin-mode/",
                                location: Location::caller(),
                            }),
                        }
                    }

                    match (pin, mode)
                    {
                        (Some(pin), Some(mode)) =>
                        {
                            match (PickedInteractivePin
                            {
                                all_pins: &mut interactive_pins,
                                pin_id: pin,
                            })
                            .set_pin_mode(mode)
                            {
                                Ok(()) => Ok(Response::SetPinModeOk
                                {
                                    pin,
                                }),
                            }
                        },
                        (None, _) => return Err(ReadError::InvalidRequest
                        {
                            message: "Missing 'pin' field.",
                            path: "set-pin-mode/",
                            location: Location::caller(),
                        }),
                        (_, None) => return Err(ReadError::InvalidRequest
                        {
                            message: "Missing 'mode' field.",
                            path: "set-pin-mode/",
                            location: Location::caller(),
                        }),
                    }
                }
                _ => return Err(ReadError::InvalidRequest
                {
                    message: "Invalid root tag.",
                    path: "",
                    location: Location::caller(),
                })
            }
        })
        // MARK: Response
        {
            // MARK: GetConfig
            Ok(Response::GetConfig) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+get-config")?;
                {
                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("name")?;
                    writer.write_string("Arduino Uno")?;

                    writer.write_type(nbt::Type::List)?;
                    writer.write_name("pins")?;
                    writer.write_element_type(nbt::ElementType::Compound)?;
                    writer.write_len(18)?;
                    {
                        let mut write_pin = |
                            id: u8,
                            name: &str,
                            modes: &[&str]| -> Result<(), Infallible>
                        {
                            writer.write_type(nbt::Type::Byte)?;
                            writer.write_name("id")?;
                            writer.write_ubyte(id)?;

                            writer.write_type(nbt::Type::String)?;
                            writer.write_name("name")?;
                            writer.write_string(name)?;

                            writer.write_type(nbt::Type::List)?;
                            writer.write_name("modes")?;
                            writer.write_element_type(nbt::ElementType::String)?;
                            writer.write_len(modes.len() as u32)?;
                            for mode in modes
                            {
                                writer.write_string(mode)?;
                            }

                            writer.write_end()?;

                            Ok(())
                        };

                        write_pin(0, "D2", &["digital-input", "digital-output"])?;
                        write_pin(1, "D3", &["digital-input", "digital-output"])?;
                        write_pin(2, "D4", &["digital-input", "digital-output"])?;
                        write_pin(3, "D5", &["digital-input", "digital-output"])?;
                        write_pin(4, "D6", &["digital-input", "digital-output"])?;
                        write_pin(5, "D7", &["digital-input", "digital-output"])?;

                        write_pin(6, "D8", &["digital-input", "digital-output"])?;
                        write_pin(7, "D9", &["digital-input", "digital-output"])?;
                        write_pin(8, "D10", &["digital-input", "digital-output"])?;
                        write_pin(9, "D11", &["digital-input", "digital-output"])?;
                        write_pin(10, "D12", &["digital-input", "digital-output"])?;
                        write_pin(11, "D13", &["digital-input", "digital-output"])?;

                        write_pin(12, "A0", &["digital-input", "digital-output"])?;
                        write_pin(13, "A1", &["digital-input", "digital-output"])?;
                        write_pin(14, "A2", &["digital-input", "digital-output"])?;
                        write_pin(15, "A3", &["digital-input", "digital-output"])?;
                        write_pin(16, "A4", &["digital-input", "digital-output"])?;
                        write_pin(17, "A5", &["digital-input", "digital-output"])?;
                    }

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: GetPinOk
            Ok(Response::GetPinOk { pin, is_high }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+get-pin")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("is-high")?;
                    writer.write_bool(is_high)?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: GetPinError
            Ok(Response::GetPinError { pin, message, location }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+get-pin")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("error")?;
                    writer.write_string(message)?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("file")?;
                    writer.write_string(location.file())?;

                    writer.write_type(nbt::Type::Int)?;
                    writer.write_name("line")?;
                    writer.write_uint(location.line())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: SetPinOk
            Ok(Response::SetPinOk { pin }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+set-pin")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: SetPinError
            Ok(Response::SetPinError { pin, message, location }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+set-pin")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("error")?;
                    writer.write_string(message)?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("file")?;
                    writer.write_string(location.file())?;

                    writer.write_type(nbt::Type::Int)?;
                    writer.write_name("line")?;
                    writer.write_uint(location.line())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: GetPinModeOk
            Ok(Response::GetPinModeOk { pin, mode }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+get-pin-mode")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("mode")?;
                    writer.write_string(mode.into())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: GetPinModeError
            Ok(Response::GetPinModeError { pin, message, location }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+get-pin-mode")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("error")?;
                    writer.write_string(message)?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("file")?;
                    writer.write_string(location.file())?;

                    writer.write_type(nbt::Type::Int)?;
                    writer.write_name("line")?;
                    writer.write_uint(location.line())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: SetPinModeOk
            Ok(Response::SetPinModeOk { pin }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+set-pin-mode")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: SetPinModeError
            Ok(Response::SetPinModeError { pin, message, location }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("+set-pin-mode")?;
                {
                    writer.write_type(nbt::Type::Byte)?;
                    writer.write_name("pin")?;
                    writer.write_ubyte(pin.into())?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("error")?;
                    writer.write_string(message)?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("file")?;
                    writer.write_string(location.file())?;

                    writer.write_type(nbt::Type::Int)?;
                    writer.write_name("line")?;
                    writer.write_uint(location.line())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: TimedOut
            Err(ReadError::TimedOut) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("error")?;
                {
                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("type")?;
                    writer.write_string(
                        "timeout")?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("message")?;
                    writer.write_string(
                        "Timed out while reading command.")?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: FoundControlByte
            Err(ReadError::FoundControlByte(_)) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("error")?;
                {
                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("type")?;
                    writer.write_string(
                        "control-byte")?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("message")?;
                    writer.write_string(
                        "Found control byte while reading command.")?;

                    writer.write_end()?;
                }

                Ok(())
            }),
            // MARK: InvalidRequest
            Err(ReadError::InvalidRequest { message, path, location }) => infallible_scope(|| unsafe
            {
                block!(raw_serial_writer.write(CONTROL_BYTE))?;
                block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                writer.write_type(nbt::Type::Compound)?;
                writer.write_name("error")?;
                {
                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("type")?;
                    writer.write_string(
                        "invalid-request")?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("message")?;
                    writer.write_string(message)?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("path")?;
                    writer.write_string(path)?;

                    writer.write_type(nbt::Type::String)?;
                    writer.write_name("file")?;
                    writer.write_string(location.file())?;

                    writer.write_type(nbt::Type::Int)?;
                    writer.write_name("line")?;
                    writer.write_uint(location.line())?;

                    writer.write_end()?;
                }

                Ok(())
            }),
        }
    }
}

fn infallible_scope<T>(f: impl FnOnce() -> Result<T, Infallible>) -> T
{
    f().unwrap_infallible()
}

fn try_scope<T, E>(f: impl FnOnce() -> Result<T, E>) -> Result<T, E>
{
    f()
}

unsafe fn collect_type_or_end_and_name<const N: usize>(
    reader: &mut impl nbt::ReadRaw<Error = ReadError>,
    error_path: &'static str,
    error_location: &'static Location<'static>)
    -> Result<Option<(nbt::Type, heapless::String<N>)>, ReadError>
{
    let tag = match reader.read_type_or_end()?
    {
        Ok(Some(tag)) => tag,
        Ok(None) => return Ok(None),
        Err(error) =>
        {
            reader.discard(error.byte_count)?;
            return Err(ReadError::InvalidRequest
            {
                message: "Type is not valid.",
                path: error_path,
                location: error_location,
            });
        },
    };

    Ok(Some((tag, collect_name(reader, error_path, error_location)?)))
}

unsafe fn collect_type_and_name<const N: usize>(
    reader: &mut impl nbt::ReadRaw<Error = ReadError>,
    error_path: &'static str,
    error_location: &'static Location<'static>)
    -> Result<(nbt::Type, heapless::String<N>), ReadError>
{
    let tag = match reader.read_type()?
    {
        Ok(tag) => tag,
        Err(error) =>
        {
            reader.discard(error.byte_count)?;
            return Err(ReadError::InvalidRequest
            {
                message: "Type is not valid.",
                path: error_path,
                location: error_location,
            });
        },
    };

    Ok((tag, collect_name(reader, error_path, error_location)?))
}

unsafe fn collect_name<const N: usize>(
    reader: &mut impl nbt::ReadRaw<Error = ReadError>,
    error_path: &'static str,
    error_location: &'static Location<'static>)
    -> Result<heapless::String<N>, ReadError>
{
    let len = reader.read_ushort()?;

    if len > N as u16
    {
        return Err(ReadError::InvalidRequest
        {
            message: "Name is too long.",
            path: error_path,
            location: error_location,
        });
    }

    let mut bytes = heapless::Vec::<u8, N>::new();

    for _ in 0..len
    {
        let Ok(()) = bytes.push(reader.read_ubyte()?)
        else
        {
            return Err(ReadError::InvalidRequest
            {
                message: "Name is too long.",
                path: error_path,
                location: error_location,
            });
        };
    }

    match heapless::String::from_utf8(bytes)
    {
        Ok(utf8) => Ok(utf8),
        Err(_) => return Err(ReadError::InvalidRequest
        {
            message: "Name is not valid UTF-8.",
            path: error_path,
            location: error_location,
        }),
    }
}

unsafe fn collect_string<const N: usize>(
    reader: &mut impl nbt::ReadRaw<Error = ReadError>,
    error_path: &'static str,
    error_location: &'static Location<'static>)
    -> Result<heapless::String<N>, ReadError>
{
    let len = reader.read_ushort()?;

    if len > N as u16
    {
        return Err(ReadError::InvalidRequest
        {
            message: "String is too long.",
            path: error_path,
            location: error_location,
        });
    }

    let mut bytes = heapless::Vec::<u8, N>::new();

    for _ in 0..len
    {
        let Ok(()) = bytes.push(reader.read_ubyte()?)
        else
        {
            return Err(ReadError::InvalidRequest
            {
                message: "String is too long.",
                path: error_path,
                location: error_location,
            })
        };
    }

    match heapless::String::from_utf8(bytes)
    {
        Ok(utf8) => Ok(utf8),
        Err(_) => return Err(ReadError::InvalidRequest
        {
            message: "String is not valid UTF-8.",
            path: error_path,
            location: error_location,
        }),
    }
}