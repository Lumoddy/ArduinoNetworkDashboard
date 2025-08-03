use super::decoder::*;
use super::Infallible;

#[derive(Default)]
pub enum DecodeU64
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

impl DecodeU64
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for u64
{
    type DecoderError = Infallible;

    type Decoder = DecodeU64;
}

impl Decode<u64, u8> for DecodeU64
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
                *self = Self::Progress4(*b1, *b2, *b3, byte);
                Ok(Ok(false))
            },
            Self::Progress4(b1, b2, b3, b4) =>
            {
                *self = Self::Progress5(*b1, *b2, *b3, *b4, byte);
                Ok(Ok(false))
            },
            Self::Progress5(b1, b2, b3, b4, b5) =>
            {
                *self = Self::Progress6(*b1, *b2, *b3, *b4, *b5, byte);
                Ok(Ok(false))
            },
            Self::Progress6(b1, b2, b3, b4, b5, b6) =>
            {
                *self = Self::Progress7(*b1, *b2, *b3, *b4, *b5, *b6, byte);
                Ok(Ok(false))
            },
            Self::Progress7(b1, b2, b3, b4, b5, b6, b7) =>
            {
                *self = Self::Done(
                    u64::from_le_bytes([*b1, *b2, *b3, *b4, *b5, *b6, *b7, byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> u64
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}

#[derive(Default)]
pub enum DecodeI64
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

impl DecodeI64
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for i64
{
    type DecoderError = Infallible;

    type Decoder = DecodeI64;
}

impl Decode<i64, u8> for DecodeI64
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
                *self = Self::Progress4(*b1, *b2, *b3, byte);
                Ok(Ok(false))
            },
            Self::Progress4(b1, b2, b3, b4) =>
            {
                *self = Self::Progress5(*b1, *b2, *b3, *b4, byte);
                Ok(Ok(false))
            },
            Self::Progress5(b1, b2, b3, b4, b5) =>
            {
                *self = Self::Progress6(*b1, *b2, *b3, *b4, *b5, byte);
                Ok(Ok(false))
            },
            Self::Progress6(b1, b2, b3, b4, b5, b6) =>
            {
                *self = Self::Progress7(*b1, *b2, *b3, *b4, *b5, *b6, byte);
                Ok(Ok(false))
            },
            Self::Progress7(b1, b2, b3, b4, b5, b6, b7) =>
            {
                *self = Self::Done(
                    i64::from_le_bytes([*b1, *b2, *b3, *b4, *b5, *b6, *b7, byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> i64
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}