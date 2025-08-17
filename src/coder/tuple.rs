use core::mem::replace;
use super::*;

// MARK: 2
enum _PairEncoderState<T0, T1>
where T0: Encoder, T1: Encoder
{
    Ready((T0::Value, T1::Value)),
    Phase1 { encoder: T0, rest: ((), T1::Value) },
    Phase2 { encoder: T1, rest: ((), ()) },
    Done,
}

pub struct PairEncoder<T0, T1, Word, Error>
where
    T0: Encoder<Word = Word, Error = Error> + From<T0::Value>,
    T1: Encoder<Word = Word, Error = Error> + From<T1::Value>
{
    _state: _PairEncoderState<T0, T1>,
}

impl<T0, T1, Word, Error>
    Encoder for PairEncoder<T0, T1, Word, Error>
where
    T0: Encoder<Word = Word, Error = Error> + From<T0::Value>,
    T1: Encoder<Word = Word, Error = Error> + From<T1::Value>
{
    type Value = (T0, T1);
    type Word = Word;
    type Error = Error;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        match replace(&mut self._state, _PairEncoderState::Done)
        {
            _PairEncoderState::Ready(value) =>
            {
                self._state = _PairEncoderState::Phase1
                {
                    encoder: value.0.into(),
                    rest: ((), value.1),
                };
                self.read_word()
            },
            _PairEncoderState::Phase1
            {
                mut encoder,
                rest,
            }
            => match encoder.read_word()
            {
                EncoderResult::Ok(word) =>
                {
                    self._state = _PairEncoderState::Phase1 { encoder, rest };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Done(word) =>
                {
                    self._state = _PairEncoderState::Phase2
                    {
                        encoder: rest.1.into(),
                        rest: ((), ()),
                    };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Empty => panic!(
                    "Inner encoder returned empty before returning done."),
                EncoderResult::Err(error) => EncoderResult::Err(error),
            },
            _PairEncoderState::Phase2
            {
                mut encoder,
                rest,
            }
            => match encoder.read_word()
            {
                EncoderResult::Ok(word) =>
                {
                    self._state = _PairEncoderState::Phase2 { encoder, rest };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Done(word) =>
                {
                    self._state = _PairEncoderState::Done;
                    EncoderResult::Done(word)
                },
                EncoderResult::Empty => panic!(
                    "Inner encoder returned empty before returning done."),
                EncoderResult::Err(error) => EncoderResult::Err(error),
            },
            _PairEncoderState::Done => EncoderResult::Empty,
        }
    }
}

// MARK: 3
enum _TripleEncoderState<T0, T1, T2>
where T0: Encoder, T1: Encoder, T2: Encoder
{
    Ready((T0::Value, T1::Value, T2::Value)),
    Phase1 { encoder: T0, rest: ((), T1::Value, T2::Value) },
    Phase2 { encoder: T1, rest: ((), (), T2::Value) },
    Phase3 { encoder: T2, rest: ((), (), ()) },
    Done,
}

pub struct TripleEncoder<T0, T1, T2, Word, Error>
where
    T0: Encoder<Word = Word, Error = Error> + From<T0::Value>,
    T1: Encoder<Word = Word, Error = Error> + From<T1::Value>,
    T2: Encoder<Word = Word, Error = Error> + From<T2::Value>
{
    _state: _TripleEncoderState<T0, T1, T2>,
}

impl<T0, T1, T2, Word, Error>
    Encoder for TripleEncoder<T0, T1, T2, Word, Error>
where
    T0: Encoder<Word = Word, Error = Error> + From<T0::Value>,
    T1: Encoder<Word = Word, Error = Error> + From<T1::Value>,
    T2: Encoder<Word = Word, Error = Error> + From<T2::Value>
{
    type Value = (T0, T1, T2);
    type Word = Word;
    type Error = Error;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        match replace(&mut self._state, _TripleEncoderState::Done)
        {
            _TripleEncoderState::Ready(value) =>
            {
                self._state = _TripleEncoderState::Phase1
                {
                    encoder: value.0.into(),
                    rest: ((), value.1, value.2),
                };
                self.read_word()
            },
            _TripleEncoderState::Phase1
            {
                mut encoder,
                rest,
            }
            => match encoder.read_word()
            {
                EncoderResult::Ok(word) =>
                {
                    self._state = _TripleEncoderState::Phase1 { encoder, rest };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Done(word) =>
                {
                    self._state = _TripleEncoderState::Phase2
                    {
                        encoder: rest.1.into(),
                        rest: ((), (), rest.2),
                    };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Empty => panic!(
                    "Inner encoder returned empty before returning done."),
                EncoderResult::Err(error) => EncoderResult::Err(error),
            },
            _TripleEncoderState::Phase2
            {
                mut encoder,
                rest,
            }
            => match encoder.read_word()
            {
                EncoderResult::Ok(word) =>
                {
                    self._state = _TripleEncoderState::Phase2 { encoder, rest };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Done(word) =>
                {
                    self._state = _TripleEncoderState::Phase3
                    {
                        encoder: rest.2.into(),
                        rest: ((), (), ()),
                    };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Empty => panic!(
                    "Inner encoder returned empty before returning done."),
                EncoderResult::Err(error) => EncoderResult::Err(error),
            },
            _TripleEncoderState::Phase3
            {
                mut encoder,
                rest,
            }
            => match encoder.read_word()
            {
                EncoderResult::Ok(word) =>
                {
                    self._state = _TripleEncoderState::Phase3 { encoder, rest };
                    EncoderResult::Ok(word)
                },
                EncoderResult::Done(word) =>
                {
                    self._state = _TripleEncoderState::Done;
                    EncoderResult::Done(word)
                },
                EncoderResult::Empty => panic!(
                    "Inner encoder returned empty before returning done."),
                EncoderResult::Err(error) => EncoderResult::Err(error),
            },
            _TripleEncoderState::Done => EncoderResult::Empty,
        }
    }
}