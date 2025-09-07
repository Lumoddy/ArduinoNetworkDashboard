use super::payload::PayloadWrite;
use super::writer::WriteReturn;
use super::{CompoundSerializerWrite, ListFromSerializerWrite, ListPayloadWrite, SerializerWriter, Write};

pub trait CompoundListSerializerWrite: ListFromSerializerWrite<
    ElementWriter:
        for<'a> CompoundSerializerWrite<Error = Self::Error>>
{
    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::ElementWriter) -> Result<
            <Self::ElementWriter as WriteReturn>::Return,
            <Self::ElementWriter as Write>::Error>)
        -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundListSerializerWrite for SerializerWriter<E, F>
where
    Self: ListPayloadWrite<Error = E, ElementWriter = Self>
        + for<'a> CompoundSerializerWrite<Error = E, Return = Self>
{
    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::ElementWriter) -> Result<Self, E>)
        -> Result<Self::Return, Self::Error>
    {
        self.write_list(length, f)
    }
}

pub trait CompoundListSerializerWriteName: WriteReturn
{
    fn name(self, name: &str) -> Result<
        impl CompoundListSerializerWrite<Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundListSerializerWriteName for SerializerWriter<E, F>
where Self: CompoundListSerializerWrite<Error = E, Return = Self>
{
    #[inline]
    fn name(self, name: &str) -> Result<
        impl CompoundListSerializerWrite<Error = E, Return = Self>,
        Self::Error>
    {
        self.write(name)
    }
}