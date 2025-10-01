use super::{SequenceVisitor, ValueVisitor, Visitor};

pub enum ReaderError<E>
{
    Inner
    {
        byte_index: usize,
        error: E,
    },
    CollectOverflow
    {
        byte_index: usize,
        capacity: usize,
    },
    InvalidSyntax
    {
        byte_index: usize,
        found: u8,
        expected: &'static str,
    },
    InvalidValue
    {
        byte_index: usize,
        found: &'static str,
        expected: &'static str,
    },
}

pub struct Reader<F: FnMut() -> Result<u8, E>, E>
{
    _f: F,
    _byte_counter: usize,
}

impl<F: FnMut() -> Result<u8, E>, E> Reader<F, E>
{
    pub fn new(f: F) -> Self
    {
        Self { _f: f, _byte_counter: 0 }
    }
}

impl<F: FnMut() -> Result<u8, E>, E> Visitor for Reader<F, E>
{
    type Error = ReaderError<E>;
    type Return = Self;

    fn into_collect_overflow_err(self, capacity: usize) -> ReaderError<E>
    {
        ReaderError::CollectOverflow { byte_index: self._byte_counter, capacity }
    }

    fn into_invalid_syntax_err(self, found: u8, expected: &'static str) -> ReaderError<E>
    {
        ReaderError::InvalidSyntax { byte_index: self._byte_counter, found, expected }
    }

    fn into_invalid_value_err(self, found: &'static str, expected: &'static str) -> ReaderError<E>
    {
        ReaderError::InvalidValue { byte_index: self._byte_counter, found, expected }
    }
}

impl<F: FnMut() -> Result<u8, E>, E> ValueVisitor for Reader<F, E>
{
    fn u8(mut self) -> Result<(u8, Self), ReaderError<E>>
    {
        match (self._f)()
        {
            Ok(byte) =>
            {
                self._byte_counter += 1;
                Ok((byte, self))
            },
            Err(error) => Err(ReaderError::Inner
            {
                byte_index: self._byte_counter,
                error,
            }),
        }
    }

    fn i8(self) -> Result<(i8, Self), Self::Error>
    {
        Ok(match self.u8()? { (byte, reader) => (byte as i8, reader) })
    }

    fn u16(self) -> Result<(u16, Self), Self::Error>
    {
        let mut bytes = [0; 2];

        Ok(
        (
            u16::from_le_bytes(bytes),
            match match self
            .u8()? { (byte, reader) => { bytes[0] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[1] = byte; reader } }
        ))
    }

    fn i16(self) -> Result<(i16, Self), Self::Error>
    {
        Ok(match self.u16()? { (byte, reader) => (byte as i16, reader) })
    }

    fn u32(self) -> Result<(u32, Self), Self::Error>
    {
        let mut bytes = [0; 4];

        Ok(
        (
            u32::from_le_bytes(bytes),
            match match match match self
            .u8()? { (byte, reader) => { bytes[0] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[1] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[2] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[3] = byte; reader } }
        ))
    }

    fn i32(self) -> Result<(i32, Self), Self::Error>
    {
        Ok(match self.u32()? { (byte, reader) => (byte as i32, reader) })
    }

    fn u64(self) -> Result<(u64, Self), Self::Error>
    {
        let mut bytes = [0; 8];

        Ok(
        (
            u64::from_le_bytes(bytes),
            match match match match match match match match self
            .u8()? { (byte, reader) => { bytes[0] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[1] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[2] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[3] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[4] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[5] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[6] = byte; reader } }
            .u8()? { (byte, reader) => { bytes[7] = byte; reader } }
        ))
    }

    fn i64(self) -> Result<(i64, Self), Self::Error>
    {
        Ok(match self.u64()? { (byte, reader) => (byte as i64, reader) })
    }

    type SequenceVisitor = Self;

    fn sequence(self) -> Result<Self, ReaderError<E>>
    {
        Ok(self)
    }

    fn end(self) -> Result<Reader<F, E>, ReaderError<E>>
    {
        Ok(self)
    }
}

impl<F: FnMut() -> Result<u8, E>, E> SequenceVisitor for Reader<F, E>
{
    fn next<const N: usize>(self) -> Result<Result<Self, Self>, ReaderError<E>>
    {
        match self.u8()?
        {
            (0, reader) => Ok(Err(reader)),
            (1, reader) => Ok(Ok(reader)),
            (byte, reader) => Err(reader.into_invalid_syntax_err(byte, "sequence decider")),
        }
    }
}