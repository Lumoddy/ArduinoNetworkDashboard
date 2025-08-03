use super::decoder::*;
use super::Infallible;

#[derive(Default)]
pub enum DecodeU8
{
    #[default]
    Ready,
    Done(u8),
}

impl DecodeU8
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for u8
{
    type DecoderError = Infallible;

    type Decoder = DecodeU8;
}

impl Decode<u8, u8> for DecodeU8
{
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> Result<Result<bool, u8>, Infallible>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Done(u8::from_le_bytes([byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> u8
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}

#[derive(Default)]
pub enum DecodeI8
{
    #[default]
    Ready,
    Done(i8),
}

impl DecodeI8
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for i8
{
    type DecoderError = Infallible;

    type Decoder = DecodeI8;
}

impl Decode<i8, u8> for DecodeI8
{
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> Result<Result<bool, u8>, Infallible>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Done(i8::from_le_bytes([byte]));
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> i8
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}