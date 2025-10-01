use super::{SequenceTracer, Tracer, ValueTracer};

pub enum WriterError<E>
{
    Inner
    {
        byte_index: usize,
        error: E,
    },
}

pub struct Writer<F: FnMut(u8) -> Result<(), E>, E>
{
    _f: F,
    _byte_counter: usize,
}

impl<F: FnMut(u8) -> Result<(), E>, E> Writer<F, E>
{
    pub fn new(f: F) -> Self { Self { _f: f, _byte_counter: 0 } }

    pub fn into_inner(self) -> F { self._f }
}

impl<F: FnMut(u8) -> Result<(), E>, E> Tracer for Writer<F, E>
{
    type Error = WriterError<E>;
    type Return = Self;
}

impl<F: FnMut(u8) -> Result<(), E>, E> ValueTracer for Writer<F, E>
{
    fn u8(mut self, value: u8) -> Result<Self, WriterError<E>>
    {
        match (self._f)(value)
        {
            Ok(()) =>
            {
                self._byte_counter += 1;
                Ok(self)
            },
            Err(error) => Err(WriterError::Inner
            {
                byte_index: self._byte_counter,
                error,
            }),
        }
    }

    fn i8(self, value: i8) -> Result<Self, WriterError<E>>
    {
        self.u8(value as u8)
    }

    fn u16(self, value: u16) -> Result<Self, WriterError<E>>
    {
        let bytes = value.to_le_bytes();
        Ok(self
            .u8(bytes[0])?
            .u8(bytes[1])?)
    }

    fn i16(self, value: i16) -> Result<Self, WriterError<E>>
    {
        self.u16(value as u16)
    }

    fn u32(self, value: u32) -> Result<Self, WriterError<E>>
    {
        let bytes = value.to_le_bytes();
        Ok(self
            .u8(bytes[0])?
            .u8(bytes[1])?
            .u8(bytes[2])?
            .u8(bytes[3])?)
    }

    fn i32(self, value: i32) -> Result<Self, WriterError<E>>
    {
        self.u32(value as u32)
    }

    fn u64(self, value: u64) -> Result<Self, WriterError<E>>
    {
        let bytes = value.to_le_bytes();
        Ok(self
            .u8(bytes[0])?
            .u8(bytes[1])?
            .u8(bytes[2])?
            .u8(bytes[3])?
            .u8(bytes[4])?
            .u8(bytes[5])?
            .u8(bytes[6])?
            .u8(bytes[7])?)
    }

    fn i64(self, value: i64) -> Result<Self, WriterError<E>>
    {
        self.u64(value as u64)
    }

    type SequenceTracer = Self;

    fn sequence(self) -> Result<Self, WriterError<E>>
    {
        Ok(self)
    }

    fn end(self) -> Result<Self, WriterError<E>>
    {
        Ok(self)
    }
}

impl<F: FnMut(u8) -> Result<(), E>, E> SequenceTracer for Writer<F, E>
{
    type ElementTracer = Self;

    fn next(self) -> Result<Self, WriterError<E>>
    {
        self.u8(1)
    }

    fn end(self) -> Result<Self, WriterError<E>>
    {
        self.u8(0)
    }
}