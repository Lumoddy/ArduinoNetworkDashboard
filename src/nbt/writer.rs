use super::{ElementType, Endian, Type};

pub trait WriteRaw
{
    type Error;

    unsafe fn write_type<'s>(&'s mut self, tag: Type)
        -> Result<(), Self::Error>;

    unsafe fn write_element_type<'s>(&'s mut self, tag: ElementType)
        -> Result<(), Self::Error>;

    unsafe fn write_end<'s>(&'s mut self)
        -> Result<(), Self::Error>;

    unsafe fn write_len<'s>(&'s mut self, len: u32)
        -> Result<(), Self::Error>;

    unsafe fn write_name<'s>(&'s mut self, name: &str)
        -> Result<(), Self::Error>;

    unsafe fn write_bool<'s>(&'s mut self, bool: bool)
        -> Result<(), Self::Error>;

    unsafe fn write_byte<'s>(&'s mut self, byte: i8)
        -> Result<(), Self::Error>;

    unsafe fn write_ubyte<'s>(&'s mut self, byte: u8)
        -> Result<(), Self::Error>;

    unsafe fn write_short<'s>(&'s mut self, short: i16)
        -> Result<(), Self::Error>;

    unsafe fn write_ushort<'s>(&'s mut self, short: u16)
        -> Result<(), Self::Error>;

    unsafe fn write_int<'s>(&'s mut self, int: i32)
        -> Result<(), Self::Error>;

    unsafe fn write_uint<'s>(&'s mut self, int: u32)
        -> Result<(), Self::Error>;

    unsafe fn write_long<'s>(&'s mut self, long: i64)
        -> Result<(), Self::Error>;

    unsafe fn write_ulong<'s>(&'s mut self, long: u64)
        -> Result<(), Self::Error>;

    unsafe fn write_char<'s>(&'s mut self, char: char)
        -> Result<(), Self::Error>;

    unsafe fn write_string<'s>(&'s mut self, string: &str)
        -> Result<(), Self::Error>;

    unsafe fn write_byte_array<'s>(&'s mut self, array: &[i8])
        -> Result<(), Self::Error>;

    unsafe fn write_ubyte_array<'s>(&'s mut self, array: &[u8])
        -> Result<(), Self::Error>;

    unsafe fn write_short_array<'s>(&'s mut self, array: &[i16])
        -> Result<(), Self::Error>;

    unsafe fn write_ushort_array<'s>(&'s mut self, array: &[u16])
        -> Result<(), Self::Error>;

    unsafe fn write_int_array<'s>(&'s mut self, array: &[i32])
        -> Result<(), Self::Error>;

    unsafe fn write_uint_array<'s>(&'s mut self, array: &[u32])
        -> Result<(), Self::Error>;

    unsafe fn write_long_array<'s>(&'s mut self, array: &[i64])
        -> Result<(), Self::Error>;

    unsafe fn write_ulong_array<'s>(&'s mut self, array: &[u64])
        -> Result<(), Self::Error>;
}

pub struct ClosureRawWriter<E, F: FnMut(u8) -> Result<(), E>>
{
    _f: F,
    _endian: Endian,
}

impl<E, F: FnMut(u8) -> Result<(), E>> ClosureRawWriter<E, F>
{
    pub const fn new(endian: Endian, f: F) -> Self
    {
        Self { _f: f, _endian: endian }
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>> WriteRaw for ClosureRawWriter<E, F>
{
    type Error = E;

    unsafe fn write_type<'s>(&'s mut self, tag: Type)
        -> Result<(), E>
    {
        (self._f)(tag as u8)
    }

    unsafe fn write_element_type<'s>(&'s mut self, tag: ElementType)
        -> Result<(), E>
    {
        (self._f)(tag as u8)
    }

    unsafe fn write_end<'s>(&'s mut self)
        -> Result<(), E>
    {
        (self._f)(0)
    }

    unsafe fn write_len<'s>(&'s mut self, len: u32)
        -> Result<(), E>
    {
        self.write_uint(len)
    }

    unsafe fn write_name<'s>(&'s mut self, name: &str)
        -> Result<(), E>
    {
        self.write_string(name)
    }

    unsafe fn write_bool<'s>(&'s mut self, bool: bool)
        -> Result<(), E>
    {
        (self._f)(if bool { 1 } else { 0 })
    }

    unsafe fn write_byte<'s>(&'s mut self, byte: i8)
        -> Result<(), E>
    {
        self.write_ubyte(byte as u8)
    }

    unsafe fn write_ubyte<'s>(&'s mut self, byte: u8)
        -> Result<(), E>
    {
        (self._f)(byte)
    }

    unsafe fn write_short<'s>(&'s mut self, short: i16)
        -> Result<(), E>
    {
        self.write_ushort(short as u16)
    }

    unsafe fn write_ushort<'s>(&'s mut self, short: u16)
        -> Result<(), E>
    {
        for byte in match self._endian
        {
            Endian::Big => short.to_be_bytes(),
            Endian::Little => short.to_le_bytes(),
        }
        {
            (self._f)(byte)?
        }

        Ok(())
    }

    unsafe fn write_int<'s>(&'s mut self, int: i32)
        -> Result<(), E>
    {
        self.write_uint(int as u32)
    }

    unsafe fn write_uint<'s>(&'s mut self, int: u32)
        -> Result<(), E>
    {
        for byte in match self._endian
        {
            Endian::Big => int.to_be_bytes(),
            Endian::Little => int.to_le_bytes(),
        }
        {
            (self._f)(byte)?
        }

        Ok(())
    }

    unsafe fn write_long<'s>(&'s mut self, long: i64)
        -> Result<(), E>
    {
        self.write_ulong(long as u64)
    }

    unsafe fn write_ulong<'s>(&'s mut self, long: u64)
        -> Result<(), E>
    {
        for byte in match self._endian
        {
            Endian::Big => long.to_be_bytes(),
            Endian::Little => long.to_le_bytes(),
        }
        {
            (self._f)(byte)?
        }

        Ok(())
    }

    unsafe fn write_char<'s>(&'s mut self, char: char)
        -> Result<(), E>
    {
        for byte in char.encode_utf8(&mut [0u8; 4]).as_bytes()
        {
            (self._f)(*byte)?
        }

        Ok(())
    }

    unsafe fn write_string<'s>(&'s mut self, string: &str)
        -> Result<(), E>
    {
        self.write_ushort(string.len() as u16)?;

        for byte in string.as_bytes()
        {
            (self._f)(*byte)?
        }

        Ok(())
    }

    unsafe fn write_byte_array<'s>(&'s mut self, array: &[i8])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_byte(*element)?;
        }

        Ok(())
    }

    unsafe fn write_ubyte_array<'s>(&'s mut self, array: &[u8])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_ubyte(*element)?;
        }

        Ok(())
    }

    unsafe fn write_short_array<'s>(&'s mut self, array: &[i16])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_short(*element)?;
        }

        Ok(())
    }

    unsafe fn write_ushort_array<'s>(&'s mut self, array: &[u16])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_ushort(*element)?;
        }

        Ok(())
    }

    unsafe fn write_int_array<'s>(&'s mut self, array: &[i32])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_int(*element)?;
        }

        Ok(())
    }

    unsafe fn write_uint_array<'s>(&'s mut self, array: &[u32])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_uint(*element)?;
        }

        Ok(())
    }

    unsafe fn write_long_array<'s>(&'s mut self, array: &[i64])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_long(*element)?;
        }

        Ok(())
    }

    unsafe fn write_ulong_array<'s>(&'s mut self, array: &[u64])
        -> Result<(), E>
    {
        self.write_uint(array.len() as u32)?;

        for element in array
        {
            self.write_ulong(*element)?;
        }

        Ok(())
    }
}