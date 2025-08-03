use core::convert::Infallible;

use super::decoder::*;

#[derive(Default)]
pub enum DecoderU8
{
    #[default]
    Ready,
    Done(u8),
}

impl DecoderU8
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<u8> for DecoderU8
{
    fn read(&mut self) -> Option<u8>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<u8> for DecoderU8
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Done(u8::from_le_bytes([byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<u8> for DecoderU8
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<u8> for DecoderU8 { }
impl Decoder<u8> for DecoderU8 { }

impl TryDecodable for u8 { type Decoder = DecoderU8; }
impl Decodable for u8 { }

#[derive(Default)]
pub enum DecoderI8
{
    #[default]
    Ready,
    Done(i8),
}

impl DecoderI8
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<i8> for DecoderI8
{
    fn read(&mut self) -> Option<i8>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<i8> for DecoderI8
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Done(i8::from_le_bytes([byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<i8> for DecoderI8
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<i8> for DecoderI8 { }
impl Decoder<i8> for DecoderI8 { }

impl TryDecodable for i8 { type Decoder = DecoderI8; }
impl Decodable for i8 { }

#[derive(Default)]
pub enum DecoderU16
{
    #[default]
    Ready,
    Progress1(u8),
    Done(u16),
}

impl DecoderU16
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<u16> for DecoderU16
{
    fn read(&mut self) -> Option<u16>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<u16> for DecoderU16
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(())
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Done(u16::from_le_bytes([*b1, byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<u16> for DecoderU16
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<u16> for DecoderU16 { }
impl Decoder<u16> for DecoderU16 { }

impl TryDecodable for u16 { type Decoder = DecoderU16; }
impl Decodable for u16 { }

#[derive(Default)]
pub enum DecoderI16
{
    #[default]
    Ready,
    Progress1(u8),
    Done(i16),
}

impl DecoderI16
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<i16> for DecoderI16
{
    fn read(&mut self) -> Option<i16>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<i16> for DecoderI16
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(())
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Done(i16::from_le_bytes([*b1, byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<i16> for DecoderI16
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<i16> for DecoderI16 { }
impl Decoder<i16> for DecoderI16 { }

impl TryDecodable for i16 { type Decoder = DecoderI16; }
impl Decodable for i16 { }

#[derive(Default)]
pub enum DecoderU32
{
    #[default]
    Ready,
    Progress1(u8),
    Progress2(u8, u8),
    Progress3(u8, u8, u8),
    Done(u32),
}

impl DecoderU32
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<u32> for DecoderU32
{
    fn read(&mut self) -> Option<u32>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<u32> for DecoderU32
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(())
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Progress2(*b1, byte);
                Ok(())
            },
            Self::Progress2(b1, b2) =>
            {
                *self = Self::Progress3(*b1, *b2, byte);
                Ok(())
            },
            Self::Progress3(b1, b2, b3) =>
            {
                *self = Self::Done(u32::from_le_bytes([*b1, *b2, *b3, byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<u32> for DecoderU32
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<u32> for DecoderU32 { }
impl Decoder<u32> for DecoderU32 { }

impl TryDecodable for u32 { type Decoder = DecoderU32; }
impl Decodable for u32 { }

#[derive(Default)]
pub enum DecoderI32
{
    #[default]
    Ready,
    Progress1(u8),
    Progress2(u8, u8),
    Progress3(u8, u8, u8),
    Done(i32),
}

impl DecoderI32
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<i32> for DecoderI32
{
    fn read(&mut self) -> Option<i32>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<i32> for DecoderI32
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(())
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Progress2(*b1, byte);
                Ok(())
            },
            Self::Progress2(b1, b2) =>
            {
                *self = Self::Progress3(*b1, *b2, byte);
                Ok(())
            },
            Self::Progress3(b1, b2, b3) =>
            {
                *self = Self::Done(i32::from_le_bytes([*b1, *b2, *b3, byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<i32> for DecoderI32
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<i32> for DecoderI32 { }
impl Decoder<i32> for DecoderI32 { }

impl TryDecodable for i32 { type Decoder = DecoderI32; }
impl Decodable for i32 { }

#[derive(Default)]
pub enum DecoderU64
{
    #[default]
    Ready,
    Progress1(u8),
    Progress2(u8, u8),
    Progress3(u8, u8, u8),
    Progress4(u8, u8, u8, u8),
    Progress5(u8, u8, u8, u8, u8),
    Progress6(u8, u8, u8, u8, u8, u8),
    Progress7(u8, u8, u8, u8, u8, u8, u8),
    Done(u64),
}

impl DecoderU64
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<u64> for DecoderU64
{
    fn read(&mut self) -> Option<u64>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<u64> for DecoderU64
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(())
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Progress2(*b1, byte);
                Ok(())
            },
            Self::Progress2(b1, b2) =>
            {
                *self = Self::Progress3(*b1, *b2, byte);
                Ok(())
            },
            Self::Progress3(b1, b2, b3) =>
            {
                *self = Self::Progress4(*b1, *b2, *b3, byte);
                Ok(())
            },
            Self::Progress4(b1, b2, b3, b4) =>
            {
                *self = Self::Progress5(*b1, *b2, *b3, *b4, byte);
                Ok(())
            },
            Self::Progress5(b1, b2, b3, b4, b5) =>
            {
                *self = Self::Progress6(*b1, *b2, *b3, *b4, *b5, byte);
                Ok(())
            },
            Self::Progress6(b1, b2, b3, b4, b5, b6) =>
            {
                *self = Self::Progress7(*b1, *b2, *b3, *b4, *b5, *b6, byte);
                Ok(())
            },
            Self::Progress7(b1, b2, b3, b4, b5, b6, b7) =>
            {
                *self = Self::Done(u64::from_le_bytes([*b1, *b2, *b3, *b4, *b5, *b6, *b7, byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<u64> for DecoderU64
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<u64> for DecoderU64 { }
impl Decoder<u64> for DecoderU64 { }

impl TryDecodable for u64 { type Decoder = DecoderU64; }
impl Decodable for u64 { }

#[derive(Default)]
pub enum DecoderI64
{
    #[default]
    Ready,
    Progress1(u8),
    Progress2(u8, u8),
    Progress3(u8, u8, u8),
    Progress4(u8, u8, u8, u8),
    Progress5(u8, u8, u8, u8, u8),
    Progress6(u8, u8, u8, u8, u8, u8),
    Progress7(u8, u8, u8, u8, u8, u8, u8),
    Done(i64),
}

impl DecoderI64
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<i64> for DecoderI64
{
    fn read(&mut self) -> Option<i64>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<i64> for DecoderI64
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(())
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Progress2(*b1, byte);
                Ok(())
            },
            Self::Progress2(b1, b2) =>
            {
                *self = Self::Progress3(*b1, *b2, byte);
                Ok(())
            },
            Self::Progress3(b1, b2, b3) =>
            {
                *self = Self::Progress4(*b1, *b2, *b3, byte);
                Ok(())
            },
            Self::Progress4(b1, b2, b3, b4) =>
            {
                *self = Self::Progress5(*b1, *b2, *b3, *b4, byte);
                Ok(())
            },
            Self::Progress5(b1, b2, b3, b4, b5) =>
            {
                *self = Self::Progress6(*b1, *b2, *b3, *b4, *b5, byte);
                Ok(())
            },
            Self::Progress6(b1, b2, b3, b4, b5, b6) =>
            {
                *self = Self::Progress7(*b1, *b2, *b3, *b4, *b5, *b6, byte);
                Ok(())
            },
            Self::Progress7(b1, b2, b3, b4, b5, b6, b7) =>
            {
                *self = Self::Done(i64::from_le_bytes([*b1, *b2, *b3, *b4, *b5, *b6, *b7, byte]));
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<i64> for DecoderI64
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<i64> for DecoderI64 { }
impl Decoder<i64> for DecoderI64 { }

impl TryDecodable for i64 { type Decoder = DecoderI64; }
impl Decodable for i64 { }

#[derive(Default)]
pub enum DecoderUSize
{
    #[default]
    Ready,
    Progress1Long(u8),
    Done(usize),
}

impl DecoderUSize
{
    pub const fn new() -> Self { Self::Ready }
}

impl DecoderReader<usize> for DecoderUSize
{
    fn read(&mut self) -> Option<usize>
    {
        match self
        {
            Self::Done(result) => Some(*result),
            _ => None,
        }
    }
}

impl DecoderWriter<usize> for DecoderUSize
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                if byte == 0xFF
                { *self = Self::Progress1Long(byte); }
                else
                { *self = Self::Done(byte.into()); }
                Ok(())
            },
            Self::Progress1Long(b1) =>
            {
                *self = Self::Done(u16::from_le_bytes([*b1, byte]).into());
                Ok(())
            },
            _ => Err(byte),
        }
    }
}

impl TryDecoderWriter<usize> for DecoderUSize
{
    type Error = Infallible;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        Ok(self.write(byte))
    }
}

impl TryDecoder<usize> for DecoderUSize { }
impl Decoder<usize> for DecoderUSize { }

impl TryDecodable for usize { type Decoder = DecoderUSize; }
impl Decodable for usize { }