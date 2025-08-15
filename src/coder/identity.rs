use core::{marker::PhantomData, mem::replace};
use super::*;

pub struct IdentityEncoder<Value>
{
    _value: Option<Value>,
}

impl<Value> Encoder for IdentityEncoder<Value>
{
    type Value = Value;
    type Word = Value;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        match replace(&mut self._value, None)
        {
            Some(value) => EncoderResult::Done(value),
            None => EncoderResult::Empty,
        }
    }
}

impl<Value> From<Value> for IdentityEncoder<Value>
{
    fn from(value: Value) -> Self
    {
        Self { _value: Some(value) }
    }
}

pub struct IdentityDecoder<Value>
{
    _is_done: bool,
    _marker: PhantomData<Value>,
}

impl<Value> Decoder for IdentityDecoder<Value>
{
    type Value = Value;
    type Word = Value;
    type Error = Infallible;

    fn write_word(&mut self, word: Value) -> DecoderResultOf<Self>
    {
        if self._is_done
        {
            DecoderResult::Done(word)
        }
        else
        {
            self._is_done = true;
            DecoderResult::Full(word)
        }
    }
}

impl<Value> Default for IdentityDecoder<Value>
{
    fn default() -> Self
    {
        Self { _is_done: false, _marker: Default::default() }
    }
}