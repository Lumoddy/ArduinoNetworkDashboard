use super::decoder::*;
use core::mem::take;
use heapless::Vec;

#[derive(Default)]
pub enum DecoderVec<T: TryDecodable, const N: usize, E: Default>
    where T::Decoder: TryDecoder<T, Error = E> + Default
{
    #[default]
    Ready,
    ProgressLength(DecoderUSize),
    ProgressVec(Vec<T, N>, usize, T::Decoder),
    Done(Vec<T, N>),
}

impl<T: TryDecodable, const N: usize, E: Default> DecoderVec<T, N, E>
    where T::Decoder: TryDecoder<T, Error = E> + Default
{
    pub const fn new() -> Self { Self::Ready }
}

impl<T: TryDecodable, const N: usize, E: Default> DecoderReader<Vec<T, N>>
    for DecoderVec<T, N, E>
    where T::Decoder: TryDecoder<T, Error = E> + Default
{
    fn read(&mut self) -> Option<Vec<T, N>>
    {
        match self
        {
            Self::Done(_) => match take(self)
            {
                Self::Done(result) => Some(result),
                _ => unreachable!(),
            },
            _ => None,
        }
    }
}

impl<T: TryDecodable, const N: usize, E: Default> TryDecoderWriter<Vec<T, N>>
    for DecoderVec<T, N, E>
    where T::Decoder: TryDecoder<T, Error = E> + Default
{
    type Error = E;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::ProgressLength(Default::default());
                return self.try_write(byte);
            },
            Self::ProgressLength(decoder) =>
            {
                _ = decoder.write(byte);

                match decoder.read()
                {
                    Some(value) =>
                    {
                        *self = Self::ProgressVec(
                            Vec::new(),
                            value,
                            Default::default());
                        return self.try_write(byte);
                    },
                    None => return Ok(Ok(())),
                }
            },
            Self::ProgressVec(vec, length, decoder) =>
            {
                _ = decoder.try_write(byte)?;

                match decoder.read()
                {
                    Some(value) if vec.len() == *length => match vec.push(value)
                    {
                        Ok(_) => match take(self)
                        {
                            Self::ProgressVec(vec, _, _) =>
                            {
                                *self = Self::Done(vec);
                                return self.try_write(byte);
                            },
                            _ => unreachable!(),
                        },
                        Err(_) => Err(Default::default()),
                    },
                    Some(value) => match vec.push(value)
                    {
                        Ok(_) => match take(self)
                        {
                            Self::ProgressVec(vec, length, _) =>
                            {
                                *self = Self::ProgressVec(
                                    vec,
                                    length,
                                    Default::default());
                                return self.try_write(byte);
                            },
                            _ => unreachable!(),
                        },
                        Err(_) => Err(Default::default()),
                    },
                    None => Ok(Ok(())),
                }
            },
            _ => Ok(Err(byte)),
        }
    }
}

impl<T: TryDecodable, const N: usize, E: Default> TryDecoder<Vec<T, N>>
    for DecoderVec<T, N, E>
    where T::Decoder: TryDecoder<T, Error = E> + Default { }

impl<T: TryDecodable, const N: usize, E: Default> TryDecodable
    for Vec<T, N>
    where T::Decoder: TryDecoder<T, Error = E> + Default
{
    type Decoder = DecoderVec<T, N, E>;
}