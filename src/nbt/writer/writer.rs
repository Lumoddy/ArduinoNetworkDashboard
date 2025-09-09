use crate::nbt::Type;

use super::{WriteCompound, WriteRaw};

pub const MAX_LIST_LENGTH: usize = 256;

pub enum Endian
{
    Big,
    Little,
}

pub enum ClosureWriterError<Closure>
{
    LengthOverflow,
    FromClosure(Closure),
}

impl<Closure> From<Closure> for ClosureWriterError<Closure>
{
    fn from(value: Closure) -> Self { Self::FromClosure(value) }
}

pub enum ClosureWriter { }

struct _InternalClosureWriter<E, F: FnMut(u8) -> Result<(), E>>
{
    _closure: F,
    _endian: Endian,
}

impl ClosureWriter
{
    pub fn new<E, F: FnMut(u8) -> Result<(), E>>(endian: Endian, f: F)
        -> impl WriteCompound<Error = ClosureWriterError<E>>
    {
        _InternalClosureWriter::<E, F> { _closure: f, _endian: endian }
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>> WriteRaw for _InternalClosureWriter<E, F>
{
    type Error = ClosureWriterError<E>;

    unsafe fn write_type(&mut self, value: Type)
        -> Result<(), Self::Error>
    {
        Ok((self._closure)(match value
        {
            Type::Byte => 1,
            Type::Short => 2,
            Type::Int => 3,
            Type::Long => 4,
            Type::Float => 5,
            Type::Double => 6,
            Type::ByteArray => 7,
            Type::String => 8,
            Type::List => 9,
            Type::Compound => 10,
            Type::IntArray => 11,
            Type::LongArray => 12,
        })?)
    }

    unsafe fn write_name(&mut self, name: &str)
        -> Result<(), Self::Error>
    { self.write_string(name) }

    unsafe fn write_end(&mut self)
        -> Result<(), Self::Error>
    { Ok((self._closure)(0)?) }

    unsafe fn write_bool(&mut self, value: bool)
        -> Result<(), Self::Error>
    { Ok((self._closure)(if value { 1 } else { 0 })?) }

    unsafe fn write_byte(&mut self, value: i8)
        -> Result<(), Self::Error>
    { self.write_byte_unsigned(value as u8) }

    unsafe fn write_byte_unsigned(&mut self, value: u8)
        -> Result<(), Self::Error>
    { Ok((self._closure)(value)?) }

    unsafe fn write_short(&mut self, value: i16)
        -> Result<(), Self::Error>
    { self.write_short_unsigned(value as u16) }

    unsafe fn write_short_unsigned(&mut self, value: u16)
        -> Result<(), Self::Error>
    {
        for byte in match self._endian
        {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
        { (self._closure)(byte)? }

        Ok(())
    }

    unsafe fn write_int(&mut self, value: i32)
        -> Result<(), Self::Error>
    { self.write_int_unsigned(value as u32) }

    unsafe fn write_int_unsigned(&mut self, value: u32)
        -> Result<(), Self::Error>
    {
        for byte in match self._endian
        {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
        { (self._closure)(byte)? }

        Ok(())
    }

    unsafe fn write_long(&mut self, value: i64)
        -> Result<(), Self::Error>
    { self.write_long_unsigned(value as u64) }

    unsafe fn write_long_unsigned(&mut self, value: u64)
        -> Result<(), Self::Error>
    {
        for byte in match self._endian
        {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
        { (self._closure)(byte)? }

        Ok(())
    }

    unsafe fn write_length(&mut self, value: u32)
        -> Result<(), Self::Error>
    {
        if value > usize::MAX as u32 || value as usize > MAX_LIST_LENGTH
        { return Err(ClosureWriterError::LengthOverflow) }

        self.write_int_unsigned(value)?;

        Ok(())
    }

    unsafe fn write_string(&mut self, value: &str)
        -> Result<(), Self::Error>
    {
        self.write_short_unsigned(value.len() as u16);

        for byte in value.as_bytes()
        { (self._closure)(*byte)? }

        Ok(())
    }
}