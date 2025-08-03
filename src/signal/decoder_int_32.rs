use super::decoder::*;
use super::Infallible;

#[derive(Default)]
pub enum DecodeU32
{
    #[default]
    Ready,
    Progress1(u8),
    Progress2(u8, u8),
    Progress3(u8, u8, u8),
    Done(u32),
}

impl DecodeU32
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for u32
{
    type DecoderError = Infallible;

    type Decoder = DecodeU32;
}

impl Decode<u32, u8> for DecodeU32
{
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> Result<Result<bool, u8>, Infallible>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(Ok(false))
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Progress2(*b1, byte);
                Ok(Ok(false))
            },
            Self::Progress2(b1, b2) =>
            {
                *self = Self::Progress3(*b1, *b2, byte);
                Ok(Ok(false))
            },
            Self::Progress3(b1, b2, b3) =>
            {
                *self = Self::Done(u32::from_le_bytes([*b1, *b2, *b3, byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> u32
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}

#[derive(Default)]
pub enum DecodeI32
{
    #[default]
    Ready,
    Progress1(u8),
    Progress2(u8, u8),
    Progress3(u8, u8, u8),
    Done(i32),
}

impl DecodeI32
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for i32
{
    type DecoderError = Infallible;

    type Decoder = DecodeI32;
}

impl Decode<i32, u8> for DecodeI32
{
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> Result<Result<bool, u8>, Infallible>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(byte);
                Ok(Ok(false))
            },
            Self::Progress1(b1) =>
            {
                *self = Self::Progress2(*b1, byte);
                Ok(Ok(false))
            },
            Self::Progress2(b1, b2) =>
            {
                *self = Self::Progress3(*b1, *b2, byte);
                Ok(Ok(false))
            },
            Self::Progress3(b1, b2, b3) =>
            {
                *self = Self::Done(i32::from_le_bytes([*b1, *b2, *b3, byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> i32
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}