use crate::json::{self, NumberVisitor, StringVisitor, Visitor};

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

impl json::IntoJSON for InteractivePinID
{
    fn into_json<T: json::ValueTracer>(self, tracer: T) -> Result<T::Return, T::Error>
    {
        tracer.number_u8(self.into())
    }
}

impl json::FromJSON for InteractivePinID
{
    fn from_json<V: json::ValueVisitor>(visitor: V) -> Result<(Self, V::Return), V::Error>
    {
        match visitor.value()?
        {
            json::TypedValueVisitor::Object(visitor) => return Err(
                visitor.into_invalid_value_err("object", "pin id")),
            json::TypedValueVisitor::Array(visitor) => return Err(
                visitor.into_invalid_value_err("array", "pin id")),
            json::TypedValueVisitor::String(visitor) => return Err(
                visitor.into_invalid_value_err("string", "pin id")),
            json::TypedValueVisitor::Number(visitor) =>
            {
                let (pin, visitor) = visitor.collect_u8()?;
                match pin.try_into()
                {
                    Ok(pin) => Ok((pin, visitor)),
                    Err(()) => return Err(
                        visitor.into_invalid_value_err("invalid pin", "pin id")),
                }
            },
            json::TypedValueVisitor::Boolean(visitor) => return Err(
                visitor.into_invalid_value_err("boolean", "pin id")),
            json::TypedValueVisitor::Null(visitor) => return Err(
                visitor.into_invalid_value_err("null", "pin id")),
        }
    }
}