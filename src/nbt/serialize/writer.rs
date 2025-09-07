use core::convert::Infallible;

use crate::common::serialize::{Serialize, SerializeResult};

use super::CompoundSerializerWrite;

pub trait SerializeAsCompoundTag
{
    type Error;

    fn serialize<W: CompoundSerializerWrite>(self, writer: W)
        -> Result<Result<W::Parent, Self::Error>, W::Error>;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Endian
{
    Little,
    Big,
}

pub struct Serializer<Value: SerializeAsCompoundTag>
{
    _value: Value,
    _endian: Endian,
}

impl<Value: SerializeAsCompoundTag> Serializer<Value>
{
    pub const fn new(value: Value, endian: Endian) -> Self
    {
        Self { _value: value, _endian: endian }
    }
}

impl<Value: SerializeAsCompoundTag> Serialize for Serializer<Value>
{
    type Word = u8;
    type Error = Value::Error;

    fn drain<
        F: FnMut(u8) -> Result<(), E>,
        E>(self, f: F) -> SerializeResult<E, Value::Error>
    {
        match self._value.serialize(
            SerializerWriter { f, endian: self._endian })
        {
            Ok(Ok(_)) => SerializeResult::Ok,
            Ok(Err(error)) => SerializeResult::SerializeErr(error),
            Err(error) => SerializeResult::DrainErr(error),
        }
    }

    fn drain_infallible<F: FnMut(u8)>(self, f: F)
        -> SerializeResult<Infallible, Value::Error>
    {
        let mut f = f;
        let f = |byte| Ok::<(), Infallible>(f(byte));
        match self._value.serialize(
            SerializerWriter { f, endian: self._endian })
        {
            Ok(Ok(_)) => SerializeResult::Ok,
            Ok(Err(error)) => SerializeResult::SerializeErr(error),
        }
    }
}

pub(super) struct SerializerWriter<E, F: FnMut(u8) -> Result<(), E>>
{
    pub(super) f: F,
    pub(super) endian: Endian,
}

pub trait Write
{
    type Error;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    Write for SerializerWriter<E, F>
{
    type Error = E;
}

pub trait WriteReturn: Write
{
    type Return;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    WriteReturn for SerializerWriter<E, F>
{
    type Return = Self;
}