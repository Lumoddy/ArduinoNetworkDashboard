
#[must_use]
pub enum DeserializeResult<Output, Word, Error>
{
    Ok,
    Done(Output),
    Full(Word),
    Err(Error, Word),
}

pub trait Deserialize
{
    type Output;
    type Word;
    type Error;

    fn push(&mut self, word: Self::Word)
        -> DeserializeResult<Self::Output, Self::Word, Self::Error>;
}