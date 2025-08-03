use super::decoder::*;
use super::Infallible;

#[derive(Default)]
pub enum DecodeUSize
{
    #[default]
    Ready,
    ProgressLong(u8),
    Done(usize),
}

impl DecodeUSize
{
    pub const fn new() -> Self { Self::Ready }
}

impl Decodable<u8> for usize
{
    type DecoderError = Infallible;

    type Decoder = DecodeUSize;
}

impl Decode<usize, u8> for DecodeUSize
{
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> Result<Result<bool, u8>, Infallible>
    {
        match self
        {
            Self::Ready if byte == 0xFF =>
            {
                *self = Self::ProgressLong(byte);
                Ok(Ok(false))
            },
            Self::Ready =>
            {
                *self = Self::Done(byte.into());
                Ok(Ok(true))
            },
            Self::ProgressLong(b1) =>
            {
                *self = Self::Done(u16::from_le_bytes([*b1, byte]).into());
                Ok(Ok(true))
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> usize
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}