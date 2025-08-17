use core::mem::replace;

use super::*;

enum _TupleEncoderState<T1, T2>
where T1: Encoder, T2: Encoder
{
    Ready(T1::Value, T2::Value),
    Phase1(T1, T2::Value),
    Phase2(T2),
    Done,
}

pub struct TupleEncoder<T1, T2, Word, Error>
where
    T1: Encoder<Word = Word, Error = Error> + From<T1::Value>,
    T2: Encoder<Word = Word, Error = Error> + From<T2::Value>
{
    _state: _TupleEncoderState<T1, T2>,
}

impl<T1, T2, Word, Error>
    Encoder for TupleEncoder<T1, T2, Word, Error>
where
    T1: Encoder<Word = Word, Error = Error> + From<T1::Value>,
    T2: Encoder<Word = Word, Error = Error> + From<T2::Value>
{
    type Value = (T1, T2);
    type Word = Word;
    type Error = Error;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        match &mut self._state
        {
            _TupleEncoderState::Ready(_, _) =>
            {
                match replace(&mut self._state, _TupleEncoderState::Done)
                {
                    _TupleEncoderState::Ready(t1, t2) =>
                    {
                        self._state = _TupleEncoderState::Phase1(t1.into(), t2);
                        self.read_word()
                    },
                    _ => unreachable!(),
                }
            },
            _TupleEncoderState::Phase1(encoder, _) => match encoder.read_word()
            {
                EncoderResult::Ok(word) => EncoderResult::Ok(word),
                EncoderResult::Done(word) =>
                {
                    match replace(&mut self._state, _TupleEncoderState::Done)
                    {
                        _TupleEncoderState::Ready(t1, t2) =>
                        {
                            self._state = _TupleEncoderState::Phase1(
                                t1.into(),
                                t2);
                            EncoderResult::Ok(word)
                        },
                        _ => unreachable!(),
                    }
                },
                EncoderResult::Empty => panic!(
                    "Inner encoder returned empty before returning done."),
                EncoderResult::Err(error) => EncoderResult::Err(error),
            },
            _TupleEncoderState::Phase2(encoder) => todo!(),
            _TupleEncoderState::Done => todo!(),
        }
    }
}