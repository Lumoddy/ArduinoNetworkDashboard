
pub trait Visitor: Sized
{
    type Error;
    type Return;

    fn into_collect_overflow_err(self, capacity: usize) -> Self::Error;

    fn into_invalid_syntax_err(self, found: u8, expected: &'static str) -> Self::Error;

    fn into_invalid_value_err(self, found: &'static str, expected: &'static str) -> Self::Error;
}

pub trait ValueVisitor: Visitor
{
    fn u8(self) -> Result<(u8, Self), Self::Error>;

    fn i8(self) -> Result<(i8, Self), Self::Error>;

    fn u16(self) -> Result<(u16, Self), Self::Error>;

    fn i16(self) -> Result<(i16, Self), Self::Error>;

    fn u32(self) -> Result<(u32, Self), Self::Error>;

    fn i32(self) -> Result<(i32, Self), Self::Error>;

    fn u64(self) -> Result<(u64, Self), Self::Error>;

    fn i64(self) -> Result<(i64, Self), Self::Error>;

    fn value<T: FromSMF>(self) -> Result<(T, Self), Self::Error>
    {
        T::from_smf(self)
    }

    type SequenceVisitor: SequenceVisitor<
        Error = Self::Error,
        Return = Self>;

    fn sequence(self) -> Result<Self::SequenceVisitor, Self::Error>;

    fn end(self) -> Result<Self::Return, Self::Error>;
}

pub trait SequenceVisitor: Visitor
{
    fn next<const N: usize>(self) -> Result<Result<Self, Self::Return>, Self::Error>;
}

pub trait FromSMF: Sized
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>;
}

impl FromSMF for bool
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        Ok(match visitor.u8()?
        {
            (0, visitor) => (false, visitor),
            (_, visitor) => (true, visitor),
        })
    }
}

impl FromSMF for u8
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.u8()
    }
}

impl FromSMF for i8
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.i8()
    }
}

impl FromSMF for u16
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.u16()
    }
}

impl FromSMF for i16
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.i16()
    }
}

impl FromSMF for u32
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.u32()
    }
}

impl FromSMF for i32
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.i32()
    }
}

impl FromSMF for u64
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.u64()
    }
}

impl FromSMF for i64
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        visitor.i64()
    }
}

impl FromSMF for usize
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        Ok(match visitor.u16()?
        {
            (value, visitor) => (value as usize, visitor),
        })
    }
}

impl FromSMF for isize
{
    fn from_smf<V: ValueVisitor>(visitor: V) -> Result<(Self, V), V::Error>
    {
        Ok(match visitor.i16()?
        {
            (value, visitor) => (value as isize, visitor),
        })
    }
}