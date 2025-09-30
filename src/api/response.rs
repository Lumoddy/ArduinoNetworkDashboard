use crate::json::{self, ObjectTracer, ValueTracer, ArrayTracer, StringTracer};

use super::{InteractivePinID, PinMode};

pub enum ResponseMessage
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
    },
    SetPinOk
    {
        pin: InteractivePinID,
    },
    SetPinError
    {
        pin: InteractivePinID,
        message: &'static str,
    },
    GetPinModeOk
    {
        pin: InteractivePinID,
        mode: PinMode,
    },
    GetPinModeError
    {
        pin: InteractivePinID,
        message: &'static str,
    },
    SetPinModeOk
    {
        pin: InteractivePinID,
    },
    SetPinModeError
    {
        pin: InteractivePinID,
        message: &'static str,
    },
    PinChanged
    {
        pin: InteractivePinID,
        is_high: bool,
    },
    InvalidControlByteError
    {
        char_index: usize,
        byte: u8,
    },
    InvalidUTF8Error
    {
        char_index: usize,
    },
    InvalidSyntaxError
    {
        char_index: usize,
        found: char,
        expected: &'static str,
    },
    InvalidValueError
    {
        char_index: usize,
        found: &'static str,
        expected: &'static str,
    },
    FieldNotFoundError
    {
        char_index: usize,
        expected: &'static str,
    },
    DuplicateFieldError
    {
        char_index: usize,
        found: &'static str,
    },
    InvalidFieldError
    {
        char_index: usize,
    },
    NumberOverflowError
    {
        char_index: usize,
        size: usize,
    },
    StringOverflowError
    {
        char_index: usize,
        capacity: usize,
    },
    TimedOutError
    {
        char_index: usize,
    },
}

impl json::IntoJSON for ResponseMessage
{
    fn into_json<T: json::ValueTracer>(self, tracer: T)
        -> Result<T::Return, T::Error>
    {
        match self
        {
            Self::GetConfig =>
            {
                struct _PinMessage
                {
                    id: InteractivePinID,
                }

                impl json::IntoJSON for _PinMessage
                {
                    fn into_json<T: json::ValueTracer>(self, tracer: T)
                        -> Result<T::Return, T::Error>
                    {
                        tracer.object()?
                            .entry_key("path")?.str("+get-config")?
                            .entry_key("model")?.str(
                                "Arduino Uno")?
                            .entry_key("pins")?.str(
                                "Invalid control byte received.")?
                            .entry_key("modes")?.array()?
                                .element()?.str("digital-input")?
                                .element()?.str("digital-output")?
                                .end()?
                            .end()
                    }
                }

                tracer.object()?
                    .entry_key("path")?.str("+get-config")?
                    .entry_key("model")?.str(
                        "Arduino Uno")?
                    .entry_key("pins")?.array()?
                        .element()?.value(_PinMessage { id: InteractivePinID::D3 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D4 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D5 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D6 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D7 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D8 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D9 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D10 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D11 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D12 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::D13 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::A0 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::A1 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::A2 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::A3 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::A4 })?
                        .element()?.value(_PinMessage { id: InteractivePinID::A5 })?
                        .end()?
                    .end()
            },
            Self::GetPinOk { pin, is_high } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+get-pin")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .entry_key("is-high")?.bool(is_high)?
                    .end()
            },
            Self::GetPinError { pin, message } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+get-pin")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .entry_key("error")?.str(message)?
                    .end()
            },
            Self::SetPinOk { pin } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+set-pin")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .end()
            },
            Self::SetPinError { pin, message } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+set-pin")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .entry_key("error")?.str(message)?
                    .end()
            },
            Self::GetPinModeOk { pin, mode } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+get-pin-mode")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .entry_key("mode")?.value(mode)?
                    .end()
            },
            Self::GetPinModeError { pin, message } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+get-pin-mode")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .entry_key("error")?.str(message)?
                    .end()
            },
            Self::SetPinModeOk { pin } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+set-pin-mode")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .end()
            },
            Self::SetPinModeError { pin, message } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+set-pin-mode")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .entry_key("error")?.str(message)?
                    .end()
            },
            Self::PinChanged { pin, is_high } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("pin-changed")?
                    .entry_key("pin-id")?.number_u8(pin.into())?
                    .entry_key("is-high")?.bool(is_high)?
                    .end()
            },
            Self::InvalidControlByteError { char_index, byte: char } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("invalid-control")?
                    .entry_key("char")?.number_u8(char)?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::InvalidUTF8Error { char_index } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("invalid-utf8")?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::InvalidSyntaxError { char_index, found, expected } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("invalid-syntax")?
                    .entry_key("found")?.string()?.append(found)?.end()?
                    .entry_key("expected")?.str(expected)?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::InvalidValueError { char_index, found, expected } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("invalid-value")?
                    .entry_key("found")?.str(found)?
                    .entry_key("expected")?.str(expected)?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::FieldNotFoundError { char_index, expected } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("missing-field")?
                    .entry_key("expected")?.str(expected)?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::DuplicateFieldError { char_index, found } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("duplicate-field")?
                    .entry_key("found")?.str(found)?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::InvalidFieldError { char_index } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("invalid-field")?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::NumberOverflowError { char_index, size } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("number-overflow")?
                    .entry_key("size")?.number_usize(size)?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::StringOverflowError { char_index, capacity } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("string-overflow")?
                    .entry_key("capacity")?.number_usize(capacity)?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
            Self::TimedOutError { char_index } =>
            {
                tracer.object()?
                    .entry_key("path")?.str("+error")?
                    .entry_key("error")?.str("timed-out")?
                    .entry_key("index")?.number_usize(char_index)?
                    .end()
            },
        }
    }
}