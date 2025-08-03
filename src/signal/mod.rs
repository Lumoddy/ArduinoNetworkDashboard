
mod decoder_int_8;
mod decoder_int_16;
mod decoder_int_32;
mod decoder_int_64;
mod decoder_size;
mod decoder_tuple;
mod decoder_vec;

// mod encoder_int_8;
// mod encoder_int_16;
// mod encoder_int_32;
// mod encoder_int_64;
// mod encoder_size;
// mod encoder_tuple;
// mod encoder_vec;

pub(crate) use core::convert::Infallible;
pub(crate) use core::mem::take;

pub(crate) mod decoder
{
    pub use super::decoder_int_8::*;
    pub use super::decoder_int_16::*;
    pub use super::decoder_int_32::*;
    pub use super::decoder_int_64::*;
    pub use super::decoder_size::*;
    pub use super::decoder_tuple::*;
    pub use super::decoder_vec::*;

    // pub use super::encoder_int_8::*;
    // pub use super::encoder_int_16::*;
    // pub use super::encoder_int_32::*;
    // pub use super::encoder_int_64::*;
    // pub use super::encoder_size::*;
    // pub use super::encoder_tuple::*;
    // pub use super::encoder_vec::*;

    pub trait Decodable<W> : Sized
    {
        type DecoderError;

        type Decoder: Decode<Self, W, Error = Self::DecoderError>;
    }

    pub trait Decode<T, W>
    {
        type Error;

        /// Writes a word to the decoder.
        /// - If an error occurs, it is returned.
        /// - If the decoder is not taking any more words, the word itself is
        /// returned
        /// - Otherwise the word was added to the decoder and the result of
        /// `is_done()` is returned.
        fn write(&mut self, word: W) -> Result<Result<bool, W>, Self::Error>;

        /// If this function returns `true`, the decoder is not taking any more
        /// words and it is safe to call `unwrap()` without it panicking.
        fn is_done(&self) -> bool;

        /// Consumes the decoder to produce the result value. Should only be
        /// called if `is_done()` returns `true`, otherwise this function will
        /// panic.
        fn unwrap(self) -> T;
    }

    pub trait Encodable<W> : Sized
    {
        type Encode: Encode<Self, W>;

        fn encode(self) -> impl Encode<Self, W>;
    }

    pub trait Encode<T, W>
    {
        /// If this function returns `true`, `read()` will return a word.
        fn is_done(&self) -> bool;

        /// Returns the next word of the encoded string unless the encoder has
        /// reached its end.
        fn read(&mut self) -> Option<W>;
    }
}