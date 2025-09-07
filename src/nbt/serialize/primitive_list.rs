use super::payload::PayloadWrite;
use super::writer::WriteReturn;
use super::{ListFromSerializerWrite, ListPayloadWrite, PrimitiveSerializerWrite, SerializerWriter};

pub trait PrimitiveListSerializerWrite<Element>: ListFromSerializerWrite<
    ElementWriter:
        for<'a> PrimitiveSerializerWrite<&'a Element, Error = Self::Error>>
{
    fn of(self, elements: &[Element]) -> Result<Self::Return, Self::Error>;
}

impl<Element, E, F: FnMut(u8) -> Result<(), E>>
    PrimitiveListSerializerWrite<Element> for SerializerWriter<E, F>
where
    Self: ListPayloadWrite<Error = E, ElementWriter = Self>
        + for<'a> PrimitiveSerializerWrite<&'a Element, Error = E, Return = Self>
{
    fn of(self, elements: &[Element]) -> Result<Self::Return, Self::Error>
    {
        self.write_list(
            elements.len() as u32,
            |i, w| w.value(&elements[i as usize]))
    }
}

pub trait PrimitiveListSerializerWriteName<Element>: WriteReturn
{
    fn name(self, name: &str) -> Result<
        impl PrimitiveListSerializerWrite<Element, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<Element, E, F: FnMut(u8) -> Result<(), E>>
    PrimitiveListSerializerWriteName<Element> for SerializerWriter<E, F>
where Self: PrimitiveListSerializerWrite<Element, Error = E, Return = Self>
{
    #[inline]
    fn name(self, name: &str) -> Result<
        impl PrimitiveListSerializerWrite<Element, Error = E, Return = Self>,
        Self::Error>
    {
        self.write(name)
    }
}