use super::writer::{Endian, SerializerWriter, Write, WriteReturn};

pub trait PayloadWrite<Value>: WriteReturn
{
    fn write(self, payload: Value) -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<i8> for SerializerWriter<E, F>
{
    #[inline]
    fn write(self, payload: i8) -> Result<Self::Return, E> { self.write(&payload) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&i8> for SerializerWriter<E, F>
{
    fn write(mut self, payload: &i8) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<i16> for SerializerWriter<E, F>
{
    #[inline]
    fn write(self, payload: i16) -> Result<Self::Return, E> { self.write(&payload) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&i16> for SerializerWriter<E, F>
{
    fn write(mut self, payload: &i16) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<i32> for SerializerWriter<E, F>
{
    #[inline]
    fn write(self, payload: i32) -> Result<Self::Return, E> { self.write(&payload) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&i32> for SerializerWriter<E, F>
{
    fn write(mut self, payload: &i32) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<i64> for SerializerWriter<E, F>
{
    #[inline]
    fn write(self, payload: i64) -> Result<Self::Return, E> { self.write(&payload) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&i64> for SerializerWriter<E, F>
{
    fn write(mut self, payload: &i64) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<f32> for SerializerWriter<E, F>
{
    #[inline]
    fn write(self, payload: f32) -> Result<Self::Return, E> { self.write(&payload) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&f32> for SerializerWriter<E, F>
{
    fn write(mut self, payload: &f32) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<f64> for SerializerWriter<E, F>
{
    #[inline]
    fn write(self, payload: f64) -> Result<Self::Return, E> { self.write(&payload) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&f64> for SerializerWriter<E, F>
{
    fn write(mut self, payload: &f64) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&str> for SerializerWriter<E, F>
where
    Self: for<'a> PayloadWrite<&'a i8, Error = E, Return = Self>
        + PayloadWrite<i32, Error = E, Return = Self>
{
    fn write(mut self, payload: &str) -> Result<Self, E>
    {
        self = self.write(payload.len() as i32)?;

        for byte in payload.as_bytes()
        { self = self.write(*byte as i8)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&[i8]> for SerializerWriter<E, F>
where
    Self: for<'a> PayloadWrite<&'a i8, Error = E, Return = Self>
        + PayloadWrite<i32, Error = E, Return = Self>
{
    fn write(mut self, payload: &[i8]) -> Result<Self, E>
    {
        self = self.write(payload.len() as i32)?;

        for element in payload
        { self = self.write(element)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&[i32]> for SerializerWriter<E, F>
where Self:
    for<'a> PayloadWrite<&'a i32, Error = E, Return = Self>
    + PayloadWrite<i32, Error = E, Return = Self>
{
    fn write(mut self, payload: &[i32]) -> Result<Self, E>
    {
        self = self.write(payload.len() as i32)?;

        for element in payload
        { self = self.write(element)? }

        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    PayloadWrite<&[i64]> for SerializerWriter<E, F>
where Self:
    for<'a> PayloadWrite<&'a i64, Error = E, Return = Self>
    + PayloadWrite<i32, Error = E, Return = Self>
{
    fn write(mut self, payload: &[i64]) -> Result<Self, E>
    {
        self = self.write(payload.len() as i32)?;

        for element in payload
        { self = self.write(element)? }

        Ok(self)
    }
}

pub trait ListPayloadWrite: Sized
{
    type Error;
    type ElementWriter: WriteReturn;

    fn write_list(
        self,
        length: u32,
        f: impl FnMut(u32, Self::ElementWriter) -> Result<
            <Self::ElementWriter as WriteReturn>::Return,
            <Self::ElementWriter as Write>::Error>)
        -> Result<Self, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    ListPayloadWrite for SerializerWriter<E, F>
where Self: PayloadWrite<i32, Error = E, Return = Self>
{
    type Error = E;
    type ElementWriter = Self;

    fn write_list(
        mut self,
        length: u32,
        mut f: impl FnMut(u32, Self) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self.write(length as i32)?;

        for i in 0..length
        { self = f(i, self)?; }

        Ok(self)
    }
}