use crate::smf;

use super::{InteractivePinID, PinMode};

pub enum RequestMessage
{
    GetConfig,
    GetPin
    {
        pin: InteractivePinID,
    },
    SetPin
    {
        pin: InteractivePinID,
        is_high: bool,
    },
    GetPinMode
    {
        pin: InteractivePinID,
    },
    SetPinMode
    {
        pin: InteractivePinID,
        mode: PinMode,
    },
}

impl smf::FromSMF for RequestMessage
{
    fn from_smf<V: smf::ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        match visitor.u8()?
        {
            (1, visitor) => Ok((Self::GetConfig, visitor)),
            (2, visitor) =>
            {
                let pin;

                match match visitor
                .value()? { (value, reader) => { pin = value; reader } }
                {
                    reader => Ok((Self::GetPin { pin }, reader))
                }
            },
            (3, visitor) =>
            {
                let pin;
                let is_high;

                match match match visitor
                .value()? { (value, reader) => { pin = value; reader } }
                .value()? { (value, reader) => { is_high = value; reader } }
                {
                    reader => Ok((Self::SetPin { pin, is_high }, reader))
                }
            },
            (4, visitor) =>
            {
                let pin;

                match match visitor
                .value()? { (value, reader) => { pin = value; reader } }
                {
                    reader => Ok((Self::GetPinMode { pin }, reader))
                }
            },
            (5, visitor) =>
            {
                let pin;
                let mode;

                match match match visitor
                .value()? { (value, reader) => { pin = value; reader } }
                .value()? { (value, reader) => { mode = value; reader } }
                {
                    reader => Ok((Self::SetPinMode { pin, mode }, reader))
                }
            },
            (_, visitor) => return Err(visitor.into_invalid_value_err(
                "invalid",
                "valid request type id")),
        }
    }
}

impl smf::IntoSMF for RequestMessage
{
    fn into_smf<T: smf::ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        match self
        {
            Self::GetConfig => tracer
                .u8(1),
            Self::GetPin { pin } => tracer
                .u8(2)?
                .value(pin),
            Self::SetPin { pin, is_high } => tracer
                .u8(3)?
                .value(pin)?
                .value(is_high),
            Self::GetPinMode { pin } => tracer
                .u8(4)?
                .value(pin),
            Self::SetPinMode { pin, mode } => tracer
                .u8(5)?
                .value(pin)?
                .value(mode),
        }
    }
}