use crate::smf::{self, IntoSMF, SequenceTracer, ValueTracer};

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
        byte_index: usize,
        byte: u8,
    },
    CollectOverflowError
    {
        byte_index: usize,
        capacity: usize,
    },
    InvalidSyntaxError
    {
        byte_index: usize,
        found: u8,
        expected: &'static str,
    },
    InvalidValueError
    {
        byte_index: usize,
        found: &'static str,
        expected: &'static str,
    },
    TimedOutError
    {
        byte_index: usize,
    },
}

impl smf::IntoSMF for ResponseMessage
{
    fn into_smf<T: smf::ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        match self
        {
            Self::GetConfig =>
            {
                struct _PinConfig(InteractivePinID);

                impl IntoSMF for _PinConfig
                {
                    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
                    {
                        tracer
                            .value(self.0)?
                            .value(self.0.name())?
                            .sequence()?
                                .next()?.value("digital-input")?.end()?
                                .next()?.value("digital-output")?.end()?
                            .end()
                    }
                }

                tracer
                    .u8(1)?
                    .value("Arduino Uno")?
                    .sequence()?
                        .next()?.value(_PinConfig(InteractivePinID::D2))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D3))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D4))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D5))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D6))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D7))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D8))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D9))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D10))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D11))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D12))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::D13))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::A0))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::A1))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::A2))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::A3))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::A4))?.end()?
                        .next()?.value(_PinConfig(InteractivePinID::A5))?.end()?
                    .end()
            },
            Self::GetPinOk { pin, is_high } => tracer
                .u8(2)?
                .value(pin)?
                .value(is_high),
            Self::GetPinError { pin, message } => tracer
                .u8(3)?
                .value(pin)?
                .value(message),
            Self::SetPinOk { pin } => tracer
                .u8(4)?
                .value(pin),
            Self::SetPinError { pin, message } => tracer
                .u8(5)?
                .value(pin)?
                .value(message),
            Self::GetPinModeOk { pin, mode } => tracer
                .u8(6)?
                .value(pin)?
                .value(mode),
            Self::GetPinModeError { pin, message } => tracer
                .u8(7)?
                .value(pin)?
                .value(message),
            Self::SetPinModeOk { pin } => tracer
                .u8(8)?
                .value(pin),
            Self::SetPinModeError { pin, message } => tracer
                .u8(9)?
                .value(pin)?
                .value(message),
            Self::PinChanged { pin, is_high } => tracer
                .u8(10)?
                .value(pin)?
                .value(is_high),
            Self::InvalidControlByteError { byte_index, byte } => tracer
                .u8(11)?
                .value(byte_index)?
                .value(byte),
            Self::CollectOverflowError { byte_index, capacity } => tracer
                .u8(12)?
                .value(byte_index)?
                .value(capacity),
            Self::InvalidSyntaxError { byte_index, found, expected } => tracer
                .u8(13)?
                .value(byte_index)?
                .value(found)?
                .value(expected),
            Self::InvalidValueError { byte_index, found, expected } => tracer
                .u8(14)?
                .value(byte_index)?
                .value(found)?
                .value(expected),
            Self::TimedOutError { byte_index } => tracer
                .u8(15)?
                .value(byte_index),
        }
    }
}