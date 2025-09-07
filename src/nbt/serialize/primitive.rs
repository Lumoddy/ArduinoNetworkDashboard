use super::payload::PayloadWrite;
use super::writer::WriteReturn;
use super::SerializerWriter;

pub trait PrimitiveSerializerWrite<Value>: WriteReturn
{
    fn value(self, value: Value) -> Result<Self::Return, Self::Error>;
}

impl<Value, E, F: FnMut(u8) -> Result<(), E>>
    PrimitiveSerializerWrite<Value> for SerializerWriter<E, F>
where Self: PayloadWrite<Value, Error = E, Return = Self>
{
    #[inline]
    fn value(self, value: Value) -> Result<Self, Self::Error>
    {
        self.write(value)
    }
}

pub trait PrimitiveSerializerWriteName<Value>: WriteReturn
{
    fn name(self, name: &str) -> Result<
        impl PrimitiveSerializerWrite<Value, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<Value, E, F: FnMut(u8) -> Result<(), E>>
    PrimitiveSerializerWriteName<Value> for SerializerWriter<E, F>
where Self: PrimitiveSerializerWrite<Value, Error = E, Return = Self>
{
    #[inline]
    fn name(self, name: &str) -> Result<
        impl PrimitiveSerializerWrite<Value, Error = E, Return = Self>,
        E>
    {
        self.write(name)
    }
}