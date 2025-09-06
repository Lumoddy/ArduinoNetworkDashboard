use core::convert::Infallible;

#[must_use]
pub enum SerializeResult<DrainError, SerializeError>
{
    Ok,
    DrainErr(DrainError),
    SerializeErr(SerializeError),
}

pub trait Serialize
{
    type Word;
    type Error;

    fn drain<
        F: FnMut(Self::Word) -> Result<(), E>,
        E>(self, f: F) -> SerializeResult<E, Self::Error>;

    fn drain_infallible<F: FnMut(u8)>(self, f: F)
        -> SerializeResult<Infallible, Self::Error>;
}