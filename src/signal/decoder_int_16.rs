use super::decoder::*;
use super::Infallible;

#[derive(Default)]
pub enum DecodeU16
{
    #[default]
    Ready,
    Progress1(u8),
    Done(u16),
}

impl DecodeU16
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for u16
{
    type DecoderError = Infallible;

    type Decoder = DecodeU16;
}

impl Decode<u16, u8> for DecodeU16
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
                *self = Self::Done(u16::from_le_bytes([*b1, byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> u16
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}

#[derive(Default)]
pub enum DecodeI16
{
    #[default]
    Ready,
    Progress1(u8),
    Done(i16),
}

impl DecodeI16
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for i16
{
    type DecoderError = Infallible;

    type Decoder = DecodeI16;
}

impl Decode<i16, u8> for DecodeI16
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
                *self = Self::Done(i16::from_le_bytes([*b1, byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> i16
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}