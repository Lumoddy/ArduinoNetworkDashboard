use core::convert::Infallible;

#[must_use]
pub enum DeserializeResult<Output, SinkError, SerializeError>
{
    Ok(Output),
    SinkErr(SinkError),
    SerializeErr(SerializeError),
}

pub trait Deserialize
{
    type Output;
    type Word;
    type Error;

    fn sink<
        F: FnMut() -> Result<Self::Word, E>,
        E>(self, f: F) -> DeserializeResult<Self::Output, E, Self::Error>;

    fn sink_infallible<F: FnMut(u8)>(self, f: F)
        -> DeserializeResult<Self::Output, Infallible, Self::Error>;
}