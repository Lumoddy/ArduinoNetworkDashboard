use super::*;
use ::heapless::*;

// MARK: Length
enum _LengthEncoderState
{
    Ready(usize),
    Long([u8; 2]),
    SecondByte(u8),
    Done,
}

pub struct LengthEncoder
{
    _state: _LengthEncoderState,
}

impl Encoder for LengthEncoder
{
    type Value = usize;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        match self._state
        {
            _LengthEncoderState::Ready(length) =>
            {
                if length < 0xFF
                {
                    self._state = _LengthEncoderState::Done;
                    EncoderResult::Done(length as u8)
                }
                else
                {
                    self._state = _LengthEncoderState::Long(length.to_le_bytes());
                    EncoderResult::Ok(0xFF)
                }
            },
            _LengthEncoderState::Long(bytes) =>
            {
                self._state = _LengthEncoderState::SecondByte(bytes[1]);
                EncoderResult::Ok(bytes[0])
            },
            _LengthEncoderState::SecondByte(byte) =>
            {
                EncoderResult::Done(byte)
            },
            _LengthEncoderState::Done => EncoderResult::Empty,
        }
    }
}

impl From<usize> for LengthEncoder
{
    fn from(value: usize) -> Self
    {
        Self { _state: _LengthEncoderState::Ready(value) }
    }
}

enum _LengthDecoderState
{
    Ready,
    ReadyLong,
    FirstByte(u8),
    Done,
}

pub struct LengthDecoder
{
    _state: _LengthDecoderState,
}

impl Decoder for LengthDecoder
{
    type Value = usize;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: u8) -> DecoderResultOf<Self>
    {
        match self._state
        {
            _LengthDecoderState::Ready =>
            {
                if word == 0xFF
                {
                    self._state = _LengthDecoderState::ReadyLong;
                    DecoderResult::Ok
                }
                else
                {
                    self._state = _LengthDecoderState::Done;
                    DecoderResult::Done(word as usize)
                }
            },
            _LengthDecoderState::ReadyLong =>
            {
                self._state = _LengthDecoderState::FirstByte(word);
                DecoderResult::Ok
            },
            _LengthDecoderState::FirstByte(byte) =>
            {
                self._state = _LengthDecoderState::Done;
                DecoderResult::Done(usize::from_le_bytes([byte, word]))
            },
            _LengthDecoderState::Done => DecoderResult::Full(word),
        }
    }
}

impl Default for LengthDecoder
{
    fn default() -> Self
    {
        Self { _state: _LengthDecoderState::Ready }
    }
}

mod heapless
{
    use super::*;

    // MARK: LengthlessVec
    pub struct LengthlessVecEncoder<T, const N: usize>
    where T: Encoder
    {
        _current_encoder: Option<T>,
        _elements_left: Vec<T::Value, N>,
    }

    impl<T, const N: usize, Word, Error> Encoder for LengthlessVecEncoder<T, N>
    where T: Encoder<Word = Word, Error = Error> + From<T::Value>
    {
        type Value = Vec<T::Value, N>;
        type Word = Word;
        type Error = Error;

        fn read_word(&mut self) -> EncoderResultOf<Self>
        {
            match &mut self._current_encoder
            {
                None => match self._elements_left.pop()
                {
                    Some(element) =>
                    {
                        self._current_encoder = Some(element.into());
                        self.read_word()
                    },
                    None => EncoderResult::Empty,
                },
                Some(encoder) => match encoder.read_word()
                {
                    EncoderResult::Ok(word) => EncoderResult::Ok(word),
                    EncoderResult::Err(error) => EncoderResult::Err(error),
                    EncoderResult::Empty => panic!(
                        "Inner encoder returned empty before returning done."),
                    EncoderResult::Done(word) =>
                    {
                        self._current_encoder = None;
                        if self._elements_left.is_empty()
                        { EncoderResult::Done(word) }
                        else
                        { EncoderResult::Ok(word) }
                    },
                },
            }
        }
    }

    impl<T, const N: usize, Word, Error>
        From<Vec<T::Value, N>> for LengthlessVecEncoder<T, N>
    where T: Encoder<Word = Word, Error = Error> + From<T::Value>
    {
        fn from(value: Vec<T::Value, N>) -> Self
        {
            if value.len() == 0
            { panic!("Cannot encode empty array.") }

            Self { _current_encoder: None, _elements_left: value }
        }
    }

    pub struct LengthlessVecDecoder<T, const N: usize>
    where T: Decoder
    {
        _elements_so_far: Option<Vec<T::Value, N>>,
        _current_decoder: T,
        _target_len: usize,
    }

    impl<T, const N: usize, Word, Error> Decoder for LengthlessVecDecoder<T, N>
    where T: Decoder<Word = Word, Error = Error> + Default
    {
        type Value = Vec<T::Value, N>;
        type Word = Word;
        type Error = Error;

        fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
        {
            match &mut self._elements_so_far
            {
                None => DecoderResult::Full(word),
                Some(vec) =>
                {
                    match self._current_decoder.write_word(word)
                    {
                        DecoderResult::Ok => DecoderResult::Ok,
                        DecoderResult::Done(value) =>
                        {
                            match vec.push(value)
                            {
                                Ok(_) => (),
                                Err(_) => unreachable!(),
                            }

                            if vec.len() == self._target_len
                            {
                                DecoderResult::Done(
                                    self._elements_so_far.take().unwrap())
                            }
                            else
                            {
                                self._current_decoder = Default::default();
                                DecoderResult::Ok
                            }
                        },
                        DecoderResult::Full(_) => panic!(
                            "Inner encoder returned full before returning \
                             done."),
                        DecoderResult::Err(error, word) =>
                        {
                            DecoderResult::Err(error, word)
                        },
                    }
                },
            }
        }
    }

    impl<T, const N: usize, Word, Error> LengthlessVecDecoder<T, N>
    where T: Decoder<Word = Word, Error = Error> + Default
    {
        pub fn of_length(length: usize) -> Self
        {
            Self
            {
                _elements_so_far: Some(Vec::new()),
                _current_decoder: Default::default(),
                _target_len: length,
            }
        }
    }

    // MARK: Vec
    enum _VecEncoderState<T, const N: usize>
    where T: Encoder
    {
        Ready,
        Length(LengthEncoder),
        Vec(T),
    }

    pub struct VecEncoder<T, const N: usize>
    where T: Encoder
    {
        _elements_left: Vec<T::Value, N>,
        _state: _VecEncoderState<T, N>,
    }

    impl<T, const N: usize, Error> Encoder for VecEncoder<T, N>
    where T: Encoder<Word = u8, Error = Error> + From<T::Value>
    {
        type Value = Vec<T::Value, N>;
        type Word = u8;
        type Error = Error;

        fn read_word(&mut self) -> EncoderResultOf<Self>
        {
            match &mut self._state
            {
                _VecEncoderState::Length(encoder) =>
                {
                    match encoder.read_word()
                    {
                        EncoderResult::Ok(word) => EncoderResult::Ok(word),
                        EncoderResult::Done(word) =>
                        {
                            if self._elements_left.len() == 0
                            {
                                EncoderResult::Done(word)
                            }
                            else
                            {
                                self._elements_left = Vec::new();
                                self.read_word()
                            }
                        },
                        EncoderResult::Empty => panic!(
                            "Inner encoder returned empty before returning \
                             done."),
                    }
                },
                _VecEncoderState::Vec(current_encoder) =>
                {
                    match current_encoder.read_word()
                    {
                        EncoderResult::Ok(word) => EncoderResult::Ok(word),
                        EncoderResult::Err(error) => EncoderResult::Err(error),
                        EncoderResult::Empty => panic!(
                            "Inner encoder returned empty before returning \
                             done."),
                        EncoderResult::Done(word) =>
                        {
                            self._state = _VecEncoderState::Ready;
                            if self._elements_left.is_empty()
                            { EncoderResult::Done(word) }
                            else
                            { EncoderResult::Ok(word) }
                        },
                    }
                },
                _VecEncoderState::Ready => match self._elements_left.pop()
                {
                    Some(element) =>
                    {
                        self._state = _VecEncoderState::Vec(element.into());
                        self.read_word()
                    },
                    None => EncoderResult::Empty,
                },
            }
        }
    }

    impl<T, const N: usize, Error>
        From<Vec<T::Value, N>> for VecEncoder<T, N>
    where T: Encoder<Word = u8, Error = Error> + From<T::Value>
    {
        fn from(value: Vec<T::Value, N>) -> Self
        {
            if value.len() == 0
            { panic!("Cannot encode empty array.") }

            Self { _elements_left: value, _state: _VecEncoderState::Ready }
        }
    }

    enum _VecDecoderState<T, const N: usize>
    where T: Decoder
    {
        Length(LengthDecoder),
        Vec
        {
            elements_so_far: Option<Vec<T::Value, N>>,
            current_decoder: T,
            target_len: usize,
        },
    }

    pub struct VecDecoder<T, const N: usize>
    where T: Decoder
    {
        _state: _VecDecoderState<T, N>,
    }

    pub enum VecDecoderError<InnerError>
    {
        LengthLongerThanCapacity,
        Other(InnerError),
    }

    impl<T, const N: usize, Error> Decoder for VecDecoder<T, N>
    where T: Decoder<Word = u8, Error = Error> + Default
    {
        type Value = Vec<T::Value, N>;
        type Word = u8;
        type Error = VecDecoderError<Error>;

        fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
        {
            match &mut self._state
            {
                _VecDecoderState::Length(decoder) =>
                {
                    match decoder.write_word(word)
                    {
                        DecoderResult::Ok => DecoderResult::Ok,
                        DecoderResult::Done(value) =>
                        {
                            self._state = _VecDecoderState::Vec
                            {
                                elements_so_far: Some(Vec::new()),
                                current_decoder: Default::default(),
                                target_len: value,
                            };
                            DecoderResult::Ok
                        },
                        DecoderResult::Full(_) => panic!(
                            "Inner encoder returned full before returning \
                             done."),
                    }
                },
                _VecDecoderState::Vec
                {
                    elements_so_far,
                    current_decoder,
                    target_len,
                }
                => match elements_so_far
                {
                    None => DecoderResult::Full(word),
                    Some(vec) =>
                    {
                        match current_decoder.write_word(word)
                        {
                            DecoderResult::Ok => DecoderResult::Ok,
                            DecoderResult::Done(value) =>
                            {
                                match vec.push(value)
                                {
                                    Ok(_) => (),
                                    Err(_) => unreachable!(),
                                }

                                if vec.len() == *target_len
                                {
                                    DecoderResult::Done(
                                        elements_so_far.take().unwrap())
                                }
                                else
                                {
                                    *current_decoder = Default::default();
                                    DecoderResult::Ok
                                }
                            },
                            DecoderResult::Full(_) => panic!(
                                "Inner encoder returned full before returning \
                                done."),
                            DecoderResult::Err(error, word) =>
                            {
                                DecoderResult::Err(
                                    VecDecoderError::Other(error),
                                    word)
                            },
                        }
                    },
                },
            }
        }
    }

    impl<T, const N: usize, Error> Default for VecDecoder<T, N>
    where T: Decoder<Word = u8, Error = Error> + Default
    {
        fn default() -> Self
        {
            Self { _state: _VecDecoderState::Length(Default::default()) }
        }
    }
}