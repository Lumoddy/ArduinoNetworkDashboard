use super::payload::PayloadWrite;
use super::writer::WriteReturn;
use super::{ListFromSerializerWrite, ListPayloadWrite, PrimitiveSerializerWrite, SerializerWriter};

pub trait PrimitiveListListSerializerWrite<Element>: ListFromSerializerWrite<
    ElementWriter:
        for<'a> PrimitiveSerializerWrite<&'a Element, Error = Self::Error>>
{
    fn of(self, elements: &[&[Element]]) -> Result<Self::Return, Self::Error>;
}

impl<Element, E, F: FnMut(u8) -> Result<(), E>>
    PrimitiveListListSerializerWrite<Element> for SerializerWriter<E, F>
where
    Self: for<'a> PrimitiveSerializerWrite<&'a Element, Error = E, Return = Self>
        + for<'a> PayloadWrite<&'a Element>
{
    fn of(self, lists: &[&[Element]]) -> Result<Self::Return, Self::Error>
    {
        self.write_list(
            lists.len() as u32,
            |i, w| w.write_list(
                lists[i as usize].len() as u32,
                |ii, w| w.write(&(lists[i as usize])[ii as usize])))
    }
}

pub trait PrimitiveListListSerializerWriteName<Element>: WriteReturn
{
    fn name(self, name: &str) -> Result<
        impl PrimitiveListListSerializerWrite<Element, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<Element, E, F: FnMut(u8) -> Result<(), E>>
    PrimitiveListListSerializerWriteName<Element> for SerializerWriter<E, F>
where Self: PrimitiveListListSerializerWrite<Element, Error = E, Return = Self>
{
    #[inline]
    fn name(self, name: &str) -> Result<
        impl PrimitiveListListSerializerWrite<Element, Error = E, Return = Self>,
        Self::Error>
    {
        self.write(name)
    }
}