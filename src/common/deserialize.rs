
#[must_use]
pub enum DeserializeResult<Word, Error>
{
    Ok,
    Done,
    Full(Word),
    Err(Error, Word),
}

pub trait Deserialize
{
    type Word;
    type Error;

    fn push(&mut self) -> DeserializeResult<Self::Word, Self::Error>;
}