use super::decoder::*;
use super::take;

#[derive(Default)]
pub enum DecodeTuple<T1, T2, E>
    where T1: Decodable<u8, DecoderError = E>,
        T2: Decodable<u8, DecoderError = E>,
        T1::Decoder: Decode<T1, u8> + Default,
        T2::Decoder: Decode<T2, u8> + Default
{
    #[default]
    Ready,
    Progress1(T1::Decoder),
    Progress2(T1, T2::Decoder),
    Done((T1, T2)),
}

impl<T1, T2, E> DecodeTuple<T1, T2, E>
    where T1: Decodable<u8, DecoderError = E>,
        T2: Decodable<u8, DecoderError = E>,
        T1::Decoder: Decode<T1, u8> + Default,
        T2::Decoder: Decode<T2, u8> + Default
{
    pub const fn new() -> Self { Self::Ready }
}

impl<T1, T2, E> Decodable<u8>
    for (T1, T2)
    where T1: Decodable<u8, DecoderError = E>,
        T2: Decodable<u8, DecoderError = E>,
        T1::Decoder: Decode<T1, u8> + Default,
        T2::Decoder: Decode<T2, u8> + Default
{
    type DecoderError = E;

    type Decoder = DecodeTuple<T1, T2, E>;
}

impl<T1, T2, E> Decode<(T1, T2), u8>
    for DecodeTuple<T1, T2, E>
    where T1: Decodable<u8, DecoderError = E>,
        T2: Decodable<u8, DecoderError = E>,
        T1::Decoder: Decode<T1, u8> + Default,
        T2::Decoder: Decode<T2, u8> + Default
{
    type Error = E;

    fn write(&mut self, byte: u8) -> Result<Result<bool, u8>, E>
    {
        match self
        {
            Self::Ready =>
            {
                *self = Self::Progress1(Default::default());
                self.write(byte)
            },
            Self::Progress1(decode) =>
            {
                match decode.write(byte)
                {
                    Ok(Ok(true)) => match take(self)
                    {
                        Self::Progress1(decode) =>
                        {
                            *self = Self::Progress2(
                                decode.unwrap(),
                                Default::default());
                            Ok(Ok(false))
                        },
                        _ => unreachable!(),
                    },
                    Ok(Ok(false)) => Ok(Ok(false)),
                    Ok(Err(_)) => panic!("Unexpected inner decoder state."),
                    Err(error) => Err(error),
                }
            },
            Self::Progress2(_, decode) =>
            {
                match decode.write(byte)
                {
                    Ok(Ok(true)) => match take(self)
                    {
                        Self::Progress2(v1, decode) =>
                        {
                            *self = Self::Done(
                                (v1, decode.unwrap()));
                            Ok(Ok(false))
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

    fn unwrap(self) -> (T1, T2)
    {
        match self
        {
            Self::Done(result) => result,
            _ => panic!("Cannot unwrap unfinished decoder."),
        }
    }
}