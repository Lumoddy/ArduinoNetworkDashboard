use crate::json::{self, KeyVisitor, StringVisitor, ValueVisitor, Visitor, ObjectVisitor};

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

impl json::FromJSON for RequestMessage
{
    fn from_json<V: json::ValueVisitor>(visitor: V) -> Result<(Self, V::Return), V::Error>
    {
        let mut visitor = match visitor.value()?
        {
            json::TypedValueVisitor::Object(visitor) => visitor,
            json::TypedValueVisitor::Array(visitor) => return Err(
                visitor.into_invalid_value_err("array", "request object")),
            json::TypedValueVisitor::String(visitor) => return Err(
                visitor.into_invalid_value_err("string", "request object")),
            json::TypedValueVisitor::Number(visitor) => return Err(
                visitor.into_invalid_value_err("number", "request object")),
            json::TypedValueVisitor::Boolean(visitor) => return Err(
                visitor.into_invalid_value_err("boolean", "request object")),
            json::TypedValueVisitor::Null(visitor) => return Err(
                visitor.into_invalid_value_err("null", "request object")),
        };

        enum _Path
        {
            GetConfig,
            GetPin,
            SetPin,
            GetPinMode,
            SetPinMode,
        }

        impl json::FromJSON for _Path
        {
            fn from_json<V: ValueVisitor>(visitor: V) -> Result<(Self, V::Return), V::Error>
            {
                match visitor.value()?
                {
                    json::TypedValueVisitor::Object(visitor) => return Err(
                        visitor.into_invalid_value_err("object", "path")),
                    json::TypedValueVisitor::Array(visitor) => return Err(
                        visitor.into_invalid_value_err("array", "path")),
                    json::TypedValueVisitor::String(visitor) =>
                    {
                        let (pin, visitor) = visitor.collect_string::<16>()?;
                        match pin.as_str()
                        {
                            "get-config" => Ok((_Path::GetConfig, visitor)),
                            "get-pin" => Ok((_Path::GetPin, visitor)),
                            "set-pin" => Ok((_Path::SetPin, visitor)),
                            "get-pin-mode" => Ok((_Path::GetPinMode, visitor)),
                            "set-pin-mode" => Ok((_Path::SetPinMode, visitor)),
                            _ => return Err(
                                visitor.into_invalid_value_err("invalid path", "path")),
                        }
                    },
                    json::TypedValueVisitor::Number(visitor) => return Err(
                        visitor.into_invalid_value_err("number", "path")),
                    json::TypedValueVisitor::Boolean(visitor) => return Err(
                        visitor.into_invalid_value_err("boolean", "path")),
                    json::TypedValueVisitor::Null(visitor) => return Err(
                        visitor.into_invalid_value_err("null", "path")),
                }
            }
        }

        let mut path: Option<_Path> = None;
        let mut pin: Option<InteractivePinID> = None;
        let mut is_high: Option<bool> = None;
        let mut mode: Option<PinMode> = None;

        let visitor = loop
        {
            let (key, entry_visitor) = match visitor.next()?
            {
                Ok(visitor) => visitor.collect_key::<16>()?,
                Err(visitor) => break visitor,
            };
            match key.as_str()
            {
                "path" => match pin
                {
                    Some(_) => return Err(
                        entry_visitor.into_duplicate_field_err("path")),
                    None =>
                    {
                        let value;
                        (value, visitor) = entry_visitor.as_value()?;
                        path = Some(value);
                    },
                },
                "pin" => match pin
                {
                    Some(_) => return Err(
                        entry_visitor.into_duplicate_field_err("pin")),
                    None =>
                    {
                        let value;
                        (value, visitor) = entry_visitor.as_value()?;
                        pin = Some(value);
                    },
                },
                "is-high" => match is_high
                {
                    Some(_) => return Err(
                        entry_visitor.into_duplicate_field_err("is-high")),
                    None =>
                    {
                        let value;
                        (value, visitor) = entry_visitor.as_value()?;
                        is_high = Some(value);
                    },
                },
                "mode" => match is_high
                {
                    Some(_) => return Err(
                        entry_visitor.into_duplicate_field_err("mode")),
                    None =>
                    {
                        let value;
                        (value, visitor) = entry_visitor.as_value()?;
                        mode = Some(value);
                    },
                },
                _ => return Err(
                    entry_visitor.into_invalid_field_err()),
            }
        };

        Ok(
        (
            match path
            {
                None => return Err(
                    visitor.into_field_not_found_err("path")),
                Some(_Path::GetConfig) => RequestMessage::GetConfig,
                Some(_Path::GetPin) => RequestMessage::GetPin
                {
                    pin: match pin
                    {
                        Some(pin) => pin,
                        None => return Err(
                            visitor.into_field_not_found_err("pin")),
                    },
                },
                Some(_Path::SetPin) => RequestMessage::SetPin
                {
                    pin: match pin
                    {
                        Some(pin) => pin,
                        None => return Err(
                            visitor.into_field_not_found_err("pin")),
                    },
                    is_high: match is_high
                    {
                        Some(is_high) => is_high,
                        None => return Err(
                            visitor.into_field_not_found_err("is-high")),
                    }
                },
                Some(_Path::GetPinMode) => RequestMessage::GetPinMode
                {
                    pin: match pin
                    {
                        Some(pin) => pin,
                        None => return Err(
                            visitor.into_field_not_found_err("pin")),
                    },
                },
                Some(_Path::SetPinMode) => RequestMessage::SetPinMode
                {
                    pin: match pin
                    {
                        Some(pin) => pin,
                        None => return Err(
                            visitor.into_field_not_found_err("pin")),
                    },
                    mode: match mode
                    {
                        Some(mode) => mode,
                        None => return Err(
                            visitor.into_field_not_found_err("mode")),
                    }
                },
            },
            visitor,
        ))
    }
}