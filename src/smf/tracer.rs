
pub trait Tracer: Sized
{
    type Error;
    type Return;
}

pub trait ValueTracer: Tracer
{
    fn u8(self, value: u8) -> Result<Self, Self::Error>;

    fn i8(self, value: i8) -> Result<Self, Self::Error>;

    fn u16(self, value: u16) -> Result<Self, Self::Error>;

    fn i16(self, value: i16) -> Result<Self, Self::Error>;

    fn u32(self, value: u32) -> Result<Self, Self::Error>;

    fn i32(self, value: i32) -> Result<Self, Self::Error>;

    fn u64(self, value: u64) -> Result<Self, Self::Error>;

    fn i64(self, value: i64) -> Result<Self, Self::Error>;

    fn value<T: IntoSMF>(self, value: T) -> Result<Self, Self::Error>
    {
        value.into_smf(self)
    }

    type SequenceTracer: SequenceTracer<
        Error = Self::Error,
        Return = Self>;

    fn sequence(self) -> Result<Self::SequenceTracer, Self::Error>;

    fn end(self) -> Result<Self::Return, Self::Error>;
}

pub trait SequenceTracer: Tracer
{
    type ElementTracer: ValueTracer<
        Error = Self::Error,
        Return = Self>;

    fn next(self) -> Result<Self::ElementTracer, Self::Error>;

    fn end(self) -> Result<Self::Return, Self::Error>;
}

pub trait IntoSMF: Sized
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>;
}

impl IntoSMF for bool
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.u8(if self { 1 } else { 0 })
    }
}

impl IntoSMF for u8
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.u8(self)
    }
}

impl IntoSMF for i8
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.i8(self)
    }
}

impl IntoSMF for u16
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.u16(self)
    }
}

impl IntoSMF for i16
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.i16(self)
    }
}

impl IntoSMF for u32
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.u32(self)
    }
}

impl IntoSMF for i32
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.i32(self)
    }
}

impl IntoSMF for u64
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.u64(self)
    }
}

impl IntoSMF for i64
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.i64(self)
    }
}

impl IntoSMF for usize
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.u16(self as u16)
    }
}

impl IntoSMF for isize
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.i16(self as i16)
    }
}

impl IntoSMF for &[u8]
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        let mut sequence = tracer.sequence()?;

        for &byte in self
        {
            sequence = sequence.next()?.u8(byte)?.end()?
        }

        sequence.end()
    }
}

impl IntoSMF for &str
{
    fn into_smf<T: ValueTracer>(self, tracer: T) -> Result<T, T::Error>
    {
        tracer.value(self.as_bytes())
    }
}