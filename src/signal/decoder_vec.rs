use heapless::Vec;

use super::decoder::*;
use super::take;

#[derive(Default)]
pub enum DecodeVec<T, const N: usize, E>
    where T: Decodable<u8, DecoderError = E>,
        T::Decoder: Decode<T, u8> + Default,
        E: Default
{
    #[default]
    Ready,
    ProgressLength(DecodeUSize),
    ProgressVec(Vec<T, N>, usize, T::Decoder),
    Done(Vec<T, N>),
}

impl<T, const N: usize, E> DecodeVec<T, N, E>
    where T: Decodable<u8, DecoderError = E>,
        T::Decoder: Decode<T, u8> + Default,
        E: Default
{
    pub const fn new() -> Self { Self::Ready }
}

impl<T, const N: usize, E> Decodable<u8>
    for Vec<T, N>
    where T: Decodable<u8, DecoderError = E>,
        T::Decoder: Decode<T, u8> + Default,
        E: Default
{
    type DecoderError = E;

    type Decoder = DecodeVec<T, N, E>;
}

impl<T, const N: usize, E> Decode<Vec<T, N>, u8>
    for DecodeVec<T, N, E>
    where T: Decodable<u8, DecoderError = E>,
        T::Decoder: Decode<T, u8> + Default,
        E: Default
{
    type Error = E;

    fn write(&mut self, byte: u8) -> Result<Result<bool, u8>, E>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::ProgressLength(Default::default());
                self.write(byte)
            },
            Self::ProgressLength(decode) =>
            {
                match decode.write(byte)
                {
                    Ok(Ok(true)) => match take(self)
                    {
                        Self::ProgressLength(decode) => match decode.unwrap()
                        {
                            0 =>
                            {
                                *self = Self::Done(Vec::new());
                                Ok(Ok(true))
                            },
                            length =>
                            {
                                *self = Self::ProgressVec(
                                    Vec::new(),
                                    length,
                                    Default::default());
                                Ok(Ok(false))
                            },
                        },
                        _ => unreachable!(),
                    },
                    Ok(Ok(false)) => Ok(Ok(false)),
                    Ok(Err(_)) => panic!("Unexpected inner decoder state."),
                }
            },
            Self::ProgressVec(_, _, decode) =>
            {
                match decode.write(byte)
                {
                    Ok(Ok(true)) => match take(self)
                    {
                        Self::ProgressVec(mut vec, length, decode) =>
                        {
                            vec.push(decode.unwrap());
                            if vec.len() == length
                            {
                                *self = Self::Done(vec);
                                Ok(Ok(true))
                            }
                            else
                            {
                                *self = Self::ProgressVec(
                                    vec,
                                    length,
                                    Default::default());
                                Ok(Ok(false))
                            }
                        },
                        _ => unreachable!(),
                    },
                    Ok(Ok(false)) => Ok(Ok(false)),
                    Ok(Err(_)) => panic!("Unexpected inner decoder state."),
                    Err(error) => Err(error),
                }
            },
            Self::Done(_) => Ok(Err(byte)),
        }
    }

    fn is_done(&self) -> bool { matches!(self, Self::Done(_)) }

    fn unwrap(self) -> Vec<T, N>
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}