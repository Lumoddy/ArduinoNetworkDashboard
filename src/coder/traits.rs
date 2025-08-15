
pub enum EncoderResult<Word, Error>
{
    Ok(Word),
    Done(Word),
    Empty,
    Err(Error),
}

pub trait Encoder
{
    type Value;
    type Word;
    type Error;

    fn read_word(&mut self) -> EncoderResultOf<Self>;
}

pub type EncoderResultOf<Encoder>
where Encoder: self::Encoder
    = EncoderResult<Encoder::Word, Encoder::Error>;

pub enum DecoderResult<Value, Word, Error>
{
    Ok,
    Done(Value),
    Full(Word),
    Err(Error, Word),
}

pub trait Decoder
{
    type Value;
    type Word;
    type Error;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>;
}

pub type DecoderResultOf<Decoder>
where Decoder: self::Decoder
    = DecoderResult<Decoder::Value, Decoder::Word, Decoder::Error>;