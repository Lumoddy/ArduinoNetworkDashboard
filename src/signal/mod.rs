
mod decoder_int;
mod decoder_tuple;
mod decoder_vec;

pub(crate) mod decoder
{
    use core::convert::Infallible;

    pub use super::decoder_int::*;
    pub use super::decoder_tuple::*;
    pub use super::decoder_vec::*;

    pub trait TryDecodable: Sized
    {
        type Decoder: TryDecoder<Self>;
    }

    pub trait Decodable: TryDecodable
        where Self::Decoder: Decoder<Self> { }

    pub trait TryDecoderWriter<T>
    {
        type Error;

        fn try_write(&mut self, byte: u8) -> Result<Result<(), u8>, Self::Error>;
    }

    pub trait DecoderWriter<T>: TryDecoderWriter<T>
    {
        fn write(&mut self, byte: u8) -> Result<(), u8>;
    }

    pub trait DecoderReader<T>
    {
        fn read(&mut self) -> Option<T>;
    }

    pub trait TryDecoder<T>: TryDecoderWriter<T> + DecoderReader<T> { }
    pub trait Decoder<T>: DecoderWriter<T> + TryDecoder<T> { }
}