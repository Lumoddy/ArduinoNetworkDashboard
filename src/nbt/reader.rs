use super::{ElementType, Endian, Type};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataBuffered
{
    pub byte_count: usize,
}

pub trait ReadRaw
{
    type Error;

    unsafe fn discard<'s>(&'s mut self, bytes: usize)
        -> Result<(), Self::Error>;

    unsafe fn read_type<'s>(&'s mut self)
        -> Result<Result<Type, DataBuffered>, Self::Error>;

    unsafe fn read_type_or_end<'s>(&'s mut self)
        -> Result<Result<Option<Type>, DataBuffered>, Self::Error>;

    unsafe fn read_element_type<'s>(&'s mut self)
        -> Result<Result<ElementType, DataBuffered>, Self::Error>;

    unsafe fn read_end<'s>(&'s mut self)
        -> Result<Result<(), DataBuffered>, Self::Error>;

    unsafe fn read_len<'s>(&'s mut self)
        -> Result<u32, Self::Error>;

    unsafe fn read_bool<'s>(&'s mut self)
        -> Result<bool, Self::Error>;

    unsafe fn read_byte<'s>(&'s mut self)
        -> Result<i8, Self::Error>;

    unsafe fn read_ubyte<'s>(&'s mut self)
        -> Result<u8, Self::Error>;

    unsafe fn read_short<'s>(&'s mut self)
        -> Result<i16, Self::Error>;

    unsafe fn read_ushort<'s>(&'s mut self)
        -> Result<u16, Self::Error>;

    unsafe fn read_int<'s>(&'s mut self)
        -> Result<i32, Self::Error>;

    unsafe fn read_uint<'s>(&'s mut self)
        -> Result<u32, Self::Error>;

    unsafe fn read_long<'s>(&'s mut self)
        -> Result<i64, Self::Error>;

    unsafe fn read_ulong<'s>(&'s mut self)
        -> Result<u64, Self::Error>;
}

pub struct ClosureRawReader<E, F: FnMut() -> Result<u8, E>>
{
    _f: F,
    _endian: Endian,
    _buffer: heapless::Deque<u8, 1>,
}

impl<E, F: FnMut() -> Result<u8, E>> ClosureRawReader<E, F>
{
    pub const fn new(endian: Endian, f: F) -> Self
    {
        Self { _f: f, _endian: endian, _buffer: heapless::Deque::new() }
    }
}

impl<E, F: FnMut() -> Result<u8, E>> ReadRaw for ClosureRawReader<E, F>
{
    type Error = E;

    unsafe fn discard<'s>(&'s mut self, bytes: usize) -> Result<(), E>
    {
        for _ in 0..bytes
        {
            _ = if self._buffer.is_empty() { (self._f)()? }
            else { self._buffer.pop_back_unchecked() }
        }

        Ok(())
    }

    unsafe fn read_type<'s>(&'s mut self)
        -> Result<Result<Type, DataBuffered>, E>
    {
        match self._buffer.back()
        {
            Some(&byte) => match Type::try_from(byte)
            {
                Ok(tag) =>
                {
                    // SAFETY: Buffer not empty here.
                    unsafe { self._buffer.pop_back_unchecked(); }

                    Ok(Ok(tag))
                },
                Err(()) => Ok(Err(DataBuffered { byte_count: 1 }))
            },
            None =>
            {
                let byte = (self._f)()?;

                match Type::try_from(byte)
                {
                    Ok(tag) => Ok(Ok(tag)),
                    Err(()) =>
                    {
                        // SAFETY: Buffer empty here.
                        unsafe { self._buffer.push_front_unchecked(byte); }

                        Ok(Err(DataBuffered { byte_count: 1 }))
                    }
                }
            },
        }
    }

    unsafe fn read_type_or_end<'s>(&'s mut self)
        -> Result<Result<Option<Type>, DataBuffered>, E>
    {
        match self._buffer.back()
        {
            Some(&0) =>
            {
                // SAFETY: Buffer not empty here.
                unsafe { self._buffer.pop_back_unchecked(); }

                Ok(Ok(None))
            }
            Some(&byte) => match Type::try_from(byte)
            {
                Ok(tag) =>
                {
                    // SAFETY: Buffer not empty here.
                    unsafe { self._buffer.pop_back_unchecked(); }

                    Ok(Ok(Some(tag)))
                },
                Err(()) => Ok(Err(DataBuffered { byte_count: 1 }))
            },
            None =>
            {
                let byte = (self._f)()?;

                match Type::try_from(byte)
                {
                    Ok(tag) => Ok(Ok(Some(tag))),
                    Err(()) =>
                    {
                        // SAFETY: Buffer empty here.
                        unsafe { self._buffer.push_front_unchecked(byte); }

                        Ok(Err(DataBuffered { byte_count: 1 }))
                    }
                }
            },
        }
    }

    unsafe fn read_element_type<'s>(&'s mut self)
        -> Result<Result<ElementType, DataBuffered>, E>
    {
        match self._buffer.back()
        {
            Some(&byte) => match ElementType::try_from(byte)
            {
                Ok(tag) =>
                {
                    // SAFETY: Buffer not empty here.
                    unsafe { self._buffer.pop_back_unchecked(); }

                    Ok(Ok(tag))
                },
                Err(()) => Ok(Err(DataBuffered { byte_count: 1 }))
            },
            None =>
            {
                let byte = (self._f)()?;

                match ElementType::try_from(byte)
                {
                    Ok(tag) => Ok(Ok(tag)),
                    Err(()) =>
                    {
                        // SAFETY: Buffer empty here.
                        unsafe { self._buffer.push_front_unchecked(byte); }

                        Ok(Err(DataBuffered { byte_count: 1 }))
                    }
                }
            },
        }
    }

    unsafe fn read_end<'s>(&'s mut self)
        -> Result<Result<(), DataBuffered>, E>
    {
        match self._buffer.back()
        {
            Some(&byte) => match byte
            {
                0 =>
                {
                    // SAFETY: Buffer not empty here.
                    unsafe { self._buffer.pop_back_unchecked(); }

                    Ok(Ok(()))
                },
                _ => Ok(Err(DataBuffered { byte_count: 1 }))
            },
            None => match (self._f)()?
            {
                0 => Ok(Ok(())),
                byte =>
                {
                    // SAFETY: Buffer empty here.
                    unsafe { self._buffer.push_front_unchecked(byte); }

                    Ok(Err(DataBuffered { byte_count: 1 }))
                }
            },
        }
    }

    unsafe fn read_len<'s>(&'s mut self)
        -> Result<u32, E>
    {
        self.read_uint()
    }

    unsafe fn read_bool<'s>(&'s mut self)
        -> Result<bool, E>
    {
        Ok(self.read_ubyte()? != 0)
    }

    unsafe fn read_byte<'s>(&'s mut self)
        -> Result<i8, E>
    {
        self.read_ubyte().map(|x| x as i8)
    }

    unsafe fn read_ubyte<'s>(&'s mut self)
        -> Result<u8, E>
    {
        match self._buffer.pop_back()
        {
            Some(byte) => Ok(byte),
            None => Ok((self._f)()?),
        }
    }

    unsafe fn read_short<'s>(&'s mut self)
        -> Result<i16, E>
    {
        self.read_ushort().map(|x| x as i16)
    }

    unsafe fn read_ushort<'s>(&'s mut self)
        -> Result<u16, E>
    {
        Ok(match self._endian
        {
            Endian::Big => u16::from_be_bytes(
                [
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                ]),
            Endian::Little => u16::from_le_bytes(
                [
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                ]),
        })
    }

    unsafe fn read_int<'s>(&'s mut self)
        -> Result<i32, E>
    {
        self.read_uint().map(|x| x as i32)
    }

    unsafe fn read_uint<'s>(&'s mut self)
        -> Result<u32, E>
    {
        Ok(match self._endian
        {
            Endian::Big => u32::from_be_bytes(
                [
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                ]),
            Endian::Little => u32::from_le_bytes(
                [
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                ]),
        })
    }

    unsafe fn read_long<'s>(&'s mut self)
        -> Result<i64, E>
    {
        self.read_ulong().map(|x| x as i64)
    }

    unsafe fn read_ulong<'s>(&'s mut self)
        -> Result<u64, E>
    {
        Ok(match self._endian
        {
            Endian::Big => u64::from_be_bytes(
                [
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                ]),
            Endian::Little => u64::from_le_bytes(
                [
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                    self.read_ubyte()?,
                ]),
        })
    }
}
