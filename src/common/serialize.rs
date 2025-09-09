
#[must_use]
pub enum SerializeResult<Word, Error>
{
    Ok(Word),
    Done(Word),
    Empty,
    Err(Error),
}

pub trait Serialize
{
    type Word;
    type Error;

    fn pop(&mut self) -> SerializeResult<Self::Word, Self::Error>;
}