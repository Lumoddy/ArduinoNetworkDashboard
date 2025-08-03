use super::decoder::*;
use core::mem::take;

#[derive(Default)]
pub enum DecoderTuple<
    T1: TryDecodable,
    T2: TryDecodable>
    where T1::Decoder: TryDecoder<T1> + Default,
        T2::Decoder: TryDecoder<T2> + Default
{
    #[default]
    Ready,
    Progress1(T1::Decoder),
    Progress2(T1, T2::Decoder),
    Done((T1, T2)),
}
impl<
    T1: TryDecodable,
    T2: TryDecodable> DecoderTuple<T1, T2>
    where T1::Decoder: TryDecoder<T1> + Default,
        T2::Decoder: TryDecoder<T2> + Default
{
    pub const fn new() -> Self { Self::Ready }
}

impl<
    T1: TryDecodable,
    T2: TryDecodable> DecoderReader<(T1, T2)>
    for DecoderTuple<T1, T2>
    where T1::Decoder: TryDecoder<T1> + Default,
        T2::Decoder: TryDecoder<T2> + Default
{
    fn read(&mut self) -> Option<(T1, T2)>
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

impl<
    T1: TryDecodable,
    T2: TryDecodable,
    E> TryDecoderWriter<(T1, T2)>
    for DecoderTuple<T1, T2>
    where T1::Decoder: TryDecoder<T1, Error = E> + Default,
        T2::Decoder: TryDecoder<T2, Error = E> + Default
{
    type Error = E;

    fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(Default::default());
                return self.try_write(byte);
            },
            Self::Progress1(decoder) =>
            {
                _ = decoder.try_write(byte)?;

                match decoder.read()
                {
                    Some(value) =>
                    {
                        *self = Self::Progress2(value, Default::default());
                        return self.try_write(byte);
                    },
                    None => return Ok(Ok(())),
                }
            },
            Self::Progress2(_, decoder) =>
            {
                _ = decoder.try_write(byte)?;

                match decoder.read()
                {
                    Some(value) => match take(self)
                    {
                        Self::Progress2(v1, _) =>
                        {
                            *self = Self::Done((v1, value));
                            return self.try_write(byte);
                        },
                        _ => unreachable!(),
                    },
                    None => Ok(Ok(())),
                }
            },
            _ => Ok(Err(byte)),
        }
    }
}

impl<
    T1: Decodable,
    T2: Decodable,
    E> DecoderWriter<(T1, T2)>
    for DecoderTuple<T1, T2>
    where T1::Decoder: Decoder<T1, Error = E> + Default,
        T2::Decoder: Decoder<T2, Error = E> + Default
{
    fn write(&mut self, byte: u8) -> Result<(), u8>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(Default::default());
                return self.write(byte);
            },
            Self::Progress1(decoder) =>
            {
                _ = decoder.write(byte);

                match decoder.read()
                {
                    Some(value) =>
                    {
                        *self = Self::Progress2(value, Default::default());
                        return self.write(byte);
                    },
                    None => return Ok(()),
                }
            },
            Self::Progress2(_, decoder) =>
            {
                _ = decoder.write(byte);

                match decoder.read()
                {
                    Some(value) => match take(self)
                    {
                        Self::Progress2(v1, _) =>
                        {
                            *self = Self::Done((v1, value));
                            return self.write(byte);
                        },
                        _ => unreachable!(),
                    },
                    None => Ok(()),
                }
            },
            _ => Err(byte),
        }
    }
}

impl<
    T1: TryDecodable,
    T2: TryDecodable,
    E> TryDecoder<(T1, T2)>
    for DecoderTuple<T1, T2>
    where T1::Decoder: TryDecoder<T1, Error = E> + Default,
        T2::Decoder: TryDecoder<T2, Error = E> + Default { }
impl<
    T1: Decodable,
    T2: Decodable,
    E> Decoder<(T1, T2)>
    for DecoderTuple<T1, T2>
    where T1::Decoder: Decoder<T1, Error = E> + Default,
        T2::Decoder: Decoder<T2, Error = E> + Default { }

impl<
    T1: TryDecodable,
    T2: TryDecodable,
    E> TryDecodable
    for (T1, T2)
    where T1::Decoder: TryDecoder<T1, Error = E> + Default,
        T2::Decoder: TryDecoder<T2, Error = E> + Default
{
    type Decoder = DecoderTuple<T1, T2>;
}
impl<
    T1: Decodable,
    T2: Decodable,
    E> Decodable
    for (T1, T2)
    where T1::Decoder: Decoder<T1, Error = E> + Default,
        T2::Decoder: Decoder<T2, Error = E> + Default { }