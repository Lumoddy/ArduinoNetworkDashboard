use super::*;

// MARK: u8
type EncoderU8 = IdentityEncoder<u8>;

type DecoderU8 = IdentityDecoder<u8>;

// MARK: i8
struct EncoderI8
{
    _state: EncoderU8,
}

impl Encoder for EncoderI8
{
    type Value = i8;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        self._state.read_word()
    }
}

impl From<i8> for EncoderI8
{
    fn from(value: i8) -> Self { Self { _state: (value as u8).into() } }
}

struct DecoderI8
{
    _state: DecoderU8,
}

impl DecoderI8
{
    pub fn new() -> Self { Self { _state: Default::default() } }
}

impl Decoder for DecoderI8
{
    type Value = i8;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
    {
        match self._state.write_word(word)
        {
            DecoderResult::Ok => DecoderResult::Ok,
            DecoderResult::Done(value) => DecoderResult::Done(value as i8),
            DecoderResult::Full(word) => DecoderResult::Full(word),
            DecoderResult::Err(error, word) => DecoderResult::Err(error, word),
        }
    }
}

impl Default for DecoderI8
{
    fn default() -> Self { Self::new() }
}

// MARK: u16
struct EncoderU16
{
    _state: heapless::LengthlessVecEncoder<EncoderU8, 2>,
}

impl Encoder for EncoderU16
{
    type Value = u16;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self> { self._state.read_word() }
}

impl From<u16> for EncoderU16
{
    fn from(value: u16) -> Self
    {
        Self
        {
            _state: ::heapless::Vec::from_slice(
                &value.to_le_bytes()).unwrap().into()
        }
    }
}

struct DecoderU16
{
    _state: heapless::LengthlessVecDecoder<DecoderU8, 2>,
}

impl Decoder for DecoderU16
{
    type Value = u16;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
    {
        match self._state.write_word(word)
        {
            DecoderResult::Ok => DecoderResult::Ok,
            DecoderResult::Done(value) => DecoderResult::Done(
                u16::from_le_bytes([value[0], value[1]])),
            DecoderResult::Full(word) => DecoderResult::Full(word),
            DecoderResult::Err(error, word) => DecoderResult::Err(error, word),
        }
    }
}

impl Default for DecoderU16
{
    fn default() -> Self
    {
        Self { _state: heapless::LengthlessVecDecoder::of_length(2) }
    }
}

// MARK: i16
struct EncoderI16
{
    _state: EncoderU16,
}

impl Encoder for EncoderI16
{
    type Value = i16;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        self._state.read_word()
    }
}

impl From<i16> for EncoderI16
{
    fn from(value: i16) -> Self { Self { _state: (value as u16).into() } }
}

struct DecoderI16
{
    _state: DecoderU16,
}

impl DecoderI16
{
    pub fn new() -> Self { Self { _state: Default::default() } }
}

impl Decoder for DecoderI16
{
    type Value = i16;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
    {
        match self._state.write_word(word)
        {
            DecoderResult::Ok => DecoderResult::Ok,
            DecoderResult::Done(value) => DecoderResult::Done(value as i16),
            DecoderResult::Full(word) => DecoderResult::Full(word),
            DecoderResult::Err(error, word) => DecoderResult::Err(error, word),
        }
    }
}

impl Default for DecoderI16
{
    fn default() -> Self { Self::new() }
}

// MARK: u32
struct EncoderU32
{
    _state: heapless::LengthlessVecEncoder<EncoderU8, 4>,
}

impl Encoder for EncoderU32
{
    type Value = u32;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self> { self._state.read_word() }
}

impl From<u32> for EncoderU32
{
    fn from(value: u32) -> Self
    {
        Self
        {
            _state: ::heapless::Vec::from_slice(
                &value.to_le_bytes()).unwrap().into()
        }
    }
}

struct DecoderU32
{
    _state: heapless::LengthlessVecDecoder<DecoderU8, 4>,
}

impl Decoder for DecoderU32
{
    type Value = u32;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
    {
        match self._state.write_word(word)
        {
            DecoderResult::Ok => DecoderResult::Ok,
            DecoderResult::Done(value) => DecoderResult::Done(
                u32::from_le_bytes([value[0], value[1], value[2], value[3]])),
            DecoderResult::Full(word) => DecoderResult::Full(word),
            DecoderResult::Err(error, word) => DecoderResult::Err(error, word),
        }
    }
}

impl Default for DecoderU32
{
    fn default() -> Self
    {
        Self { _state: heapless::LengthlessVecDecoder::of_length(4) }
    }
}

// MARK: i32
struct EncoderI32
{
    _state: EncoderU32,
}

impl Encoder for EncoderI32
{
    type Value = i32;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        self._state.read_word()
    }
}

impl From<i32> for EncoderI32
{
    fn from(value: i32) -> Self { Self { _state: (value as u32).into() } }
}

struct DecoderI32
{
    _state: DecoderU32,
}

impl DecoderI32
{
    pub fn new() -> Self { Self { _state: Default::default() } }
}

impl Decoder for DecoderI32
{
    type Value = i32;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
    {
        match self._state.write_word(word)
        {
            DecoderResult::Ok => DecoderResult::Ok,
            DecoderResult::Done(value) => DecoderResult::Done(value as i32),
            DecoderResult::Full(word) => DecoderResult::Full(word),
            DecoderResult::Err(error, word) => DecoderResult::Err(error, word),
        }
    }
}

impl Default for DecoderI32
{
    fn default() -> Self { Self::new() }
}

// MARK: u64
struct EncoderU64
{
    _state: heapless::LengthlessVecEncoder<EncoderU8, 8>,
}

impl Encoder for EncoderU64
{
    type Value = u64;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self> { self._state.read_word() }
}

impl From<u64> for EncoderU64
{
    fn from(value: u64) -> Self
    {
        Self
        {
            _state: ::heapless::Vec::from_slice(
                &value.to_le_bytes()).unwrap().into()
        }
    }
}

struct DecoderU64
{
    _state: heapless::LengthlessVecDecoder<DecoderU8, 8>,
}

impl Decoder for DecoderU64
{
    type Value = u64;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
    {
        match self._state.write_word(word)
        {
            DecoderResult::Ok => DecoderResult::Ok,
            DecoderResult::Done(value) => DecoderResult::Done(
                u64::from_le_bytes([
                    value[0], value[1], value[2], value[3],
                    value[4], value[5], value[6], value[7]])),
            DecoderResult::Full(word) => DecoderResult::Full(word),
            DecoderResult::Err(error, word) => DecoderResult::Err(error, word),
        }
    }
}

impl Default for DecoderU64
{
    fn default() -> Self
    {
        Self { _state: heapless::LengthlessVecDecoder::of_length(8) }
    }
}

// MARK: i64
struct EncoderI64
{
    _state: EncoderU64,
}

impl Encoder for EncoderI64
{
    type Value = i64;
    type Word = u8;
    type Error = Infallible;

    fn read_word(&mut self) -> EncoderResultOf<Self>
    {
        self._state.read_word()
    }
}

impl From<i64> for EncoderI64
{
    fn from(value: i64) -> Self { Self { _state: (value as u64).into() } }
}

struct DecoderI64
{
    _state: DecoderU64,
}

impl DecoderI64
{
    pub fn new() -> Self { Self { _state: Default::default() } }
}

impl Decoder for DecoderI64
{
    type Value = i64;
    type Word = u8;
    type Error = Infallible;

    fn write_word(&mut self, word: Self::Word) -> DecoderResultOf<Self>
    {
        match self._state.write_word(word)
        {
            DecoderResult::Ok => DecoderResult::Ok,
            DecoderResult::Done(value) => DecoderResult::Done(value as i64),
            DecoderResult::Full(word) => DecoderResult::Full(word),
            DecoderResult::Err(error, word) => DecoderResult::Err(error, word),
        }
    }
}

impl Default for DecoderI64
{
    fn default() -> Self { Self::new() }
}