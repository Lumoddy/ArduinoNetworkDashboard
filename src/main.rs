#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(abi_avr_interrupt)]

mod panic_handler;
mod json;
mod api;

use api::InteractivePinID;
use api::InteractivePins;
use api::PinDigitalInteraction;
use api::PinDigitalInteractionChanges;
use api::PinModeInteraction;
use api::PinSetPowerError;
use api::RequestMessage;
use api::ResponseMessage;
use arduino_hal::clock::Clock;
use arduino_hal::prelude::_embedded_hal_serial_Read;
use arduino_hal::prelude::_embedded_hal_serial_Write;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::DefaultClock;
use json::IntoJSON;
use json::ObjectTracer;
use json::ReaderError;
use json::ValueTracer;
use nb::block;
mod interactive;

use core::convert::Infallible;

#[arduino_hal::entry]
fn main() -> ! { process() }

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
    const END_TEXT_BYTE: u8 = b'';

    #[derive(Clone, Copy)]
    enum _ReadError
    {
        FoundControlByte(u8),
        TimedOut,
    }

    #[derive(Clone, Copy)]
    enum _WriteError { }

    let mut read_byte = move ||
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
                            break Err(_ReadError::TimedOut);
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
                    byte => Err(_ReadError::FoundControlByte(byte)),
                }
            }
            byte => Ok(byte),
        }
    };

    fn read_json<T: json::FromJSON>(f: &mut impl FnMut() -> Result<u8, _ReadError>)
        -> Result<T, json::ReaderError<_ReadError>>
    {
        T::from_json(json::Reader::new(move || f())).map(|x| x.0)
    }

    let mut write_byte = move |byte|
    {
        if let CONTROL_BYTE = byte
        {
            match block!(serial_writer.write(CONTROL_BYTE))
            {
                Ok(()) => ()
            };
        }

        match block!(serial_writer.write(byte))
        {
            Ok(()) => ()
        };

        Ok::<(), _WriteError>(())
    };

    fn write_json<T: json::IntoJSON>(
        f: &mut impl FnMut(u8) -> Result<(), _WriteError>,
        value: T)
        -> Result<(), json::WriterError<_WriteError>>
    {
        T::into_json(value, json::Writer::new(move |byte| f(byte))).map(|_| ())
    }

    loop
    {
        let mut try_write_change = |pin: InteractivePinID|
        {
            if let Ok(Some(is_high)) = interactive_pins
                .borrow(pin)
                .detect_pin_change()
            {
                infallible_scope(||
                {
                    struct _Message
                    {
                        pin: InteractivePinID,
                        is_high: bool,
                    }

                    impl IntoJSON for _Message
                    {
                        fn into_json<T: json::ValueTracer>(self, tracer: T)
                            -> Result<T::Return, T::Error>
                        {
                            tracer.object()?
                                .entry_key("type")?.str("pin-changed")?
                                .entry_key("pin-id")?.number_u8(self.pin.into())?
                                .entry_key("is-high")?.bool(self.is_high)?
                                .end()
                        }
                    }

                    block!(raw_serial_writer.write(CONTROL_BYTE))?;
                    block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                    match write_json(&mut write_byte, _Message
                    {
                        pin,
                        is_high,
                    })
                    {
                        Ok(()) => (),
                    }
                    block!(raw_serial_writer.write(CONTROL_BYTE))?;
                    block!(raw_serial_writer.write(END_TEXT_BYTE))?;

                    Ok(())
                });
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

        match block!(raw_serial_reader.read()).unwrap_infallible()
        {
            CONTROL_BYTE =>
            {
                match block!(raw_serial_reader.read()).unwrap_infallible()
                {
                    START_TEXT_BYTE => (),
                    CONTROL_BYTE => continue,
                    byte =>
                    {
                        infallible_scope(||
                        {
                            block!(raw_serial_writer.write(CONTROL_BYTE))?;
                            block!(raw_serial_writer.write(START_TEXT_BYTE))?;
                            match write_json(
                                &mut write_byte,
                                ResponseMessage::InvalidControlByteError { char_index: 0, byte })
                            {
                                Ok(()) => (),
                            }
                            block!(raw_serial_writer.write(CONTROL_BYTE))?;
                            block!(raw_serial_writer.write(END_TEXT_BYTE))?;

                            Ok(())
                        });

                        continue;
                    },
                }
            }
            _ => continue,
        }

        let response = match read_json(&mut read_byte)
        {
            Ok(RequestMessage::GetConfig) =>
            {
                ResponseMessage::GetConfig
            },
            Ok(RequestMessage::GetPin { pin }) =>
            {
                match interactive_pins.borrow(pin).pin_is_high()
                {
                    Ok(is_high) => ResponseMessage::GetPinOk { pin, is_high }
                }
            },
            Ok(RequestMessage::SetPin { pin, is_high }) =>
            {
                match interactive_pins.borrow(pin).set_pin_is_high(is_high)
                {
                    Ok(()) => ResponseMessage::SetPinOk { pin },
                    Err(PinSetPowerError::IsInput) => ResponseMessage::SetPinError
                    {
                        pin,
                        message: "pin-not-output",
                    }
                }
            },
            Ok(RequestMessage::GetPinMode { pin }) =>
            {
                match interactive_pins.borrow(pin).pin_mode()
                {
                    Ok(mode) => ResponseMessage::GetPinModeOk { pin, mode }
                }
            },
            Ok(RequestMessage::SetPinMode { pin, mode }) =>
            {
                match interactive_pins.borrow(pin).set_pin_mode(mode)
                {
                    Ok(()) => ResponseMessage::SetPinModeOk { pin },
                }
            },
            Err(ReaderError::Source { char_index, error: _ReadError::FoundControlByte(byte) }) =>
            {
                ResponseMessage::InvalidControlByteError { char_index, byte }
            },
            Err(ReaderError::Source { char_index, error: _ReadError::TimedOut }) =>
            {
                ResponseMessage::TimedOutError { char_index }
            },
            Err(ReaderError::InvalidUTF8 { char_index }) =>
            {
                ResponseMessage::InvalidUTF8Error { char_index }
            },
            Err(ReaderError::InvalidSyntax { char_index, found, expected }) =>
            {
                ResponseMessage::InvalidSyntaxError { char_index, found, expected }
            },
            Err(ReaderError::InvalidValue { char_index, found, expected }) =>
            {
                ResponseMessage::InvalidValueError { char_index, found, expected }
            },
            Err(ReaderError::FieldNotFound { char_index, expected }) =>
            {
                ResponseMessage::FieldNotFoundError { char_index, expected }
            },
            Err(ReaderError::DuplicateField { char_index, found }) =>
            {
                ResponseMessage::DuplicateFieldError { char_index, found }
            },
            Err(ReaderError::InvalidField { char_index }) =>
            {
                ResponseMessage::InvalidFieldError { char_index }
            },
            Err(ReaderError::NumberOverflow { char_index, size }) =>
            {
                ResponseMessage::NumberOverflowError { char_index, size }
            },
            Err(ReaderError::StringOverflow { char_index, capacity }) =>
            {
                ResponseMessage::StringOverflowError { char_index, capacity }
            },
        };

        infallible_scope(||
        {
            block!(raw_serial_writer.write(CONTROL_BYTE))?;
            block!(raw_serial_writer.write(START_TEXT_BYTE))?;
            match write_json(&mut write_byte, response)
            {
                Ok(()) => (),
            }
            block!(raw_serial_writer.write(CONTROL_BYTE))?;
            block!(raw_serial_writer.write(END_TEXT_BYTE))?;

            Ok(())
        });
    }
}

fn infallible_scope<T>(f: impl FnOnce() -> Result<T, Infallible>) -> T
{
    f().unwrap_infallible()
}