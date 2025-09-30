use super::IntoJSON;

pub trait Tracer: Sized
{
    type Error;
    type Return: Tracer<Error = Self::Error>;
}

pub trait ValueTracer: Tracer
{
    type ObjectTracer: ObjectTracer<
        Return = Self::Return,
        Error = Self::Error>;
    type ArrayTracer: ArrayTracer<
        Return = Self::Return,
        Error = Self::Error>;
    type StringTracer: StringTracer<
        Return = Self::Return,
        Error = Self::Error>;
    type NumberTracer: NumberTracer<
        Return = Self::Return,
        Error = Self::Error>;
    type BooleanTracer: BooleanTracer<
        Return = Self::Return,
        Error = Self::Error>;

    fn object(self) -> Result<Self::ObjectTracer, Self::Error>;

    fn array(self) -> Result<Self::ArrayTracer, Self::Error>;

    fn str(self, value: &str) -> Result<Self::Return, Self::Error>;

    fn string(self) -> Result<Self::StringTracer, Self::Error>;

    fn number(self) -> Result<Self::NumberTracer, Self::Error>;

    fn number_u8(self, value: u8) -> Result<Self::Return, Self::Error>;

    fn number_i8(self, value: i8) -> Result<Self::Return, Self::Error>;

    fn number_u16(self, value: u16) -> Result<Self::Return, Self::Error>;

    fn number_i16(self, value: i16) -> Result<Self::Return, Self::Error>;

    fn number_u32(self, value: u32) -> Result<Self::Return, Self::Error>;

    fn number_i32(self, value: i32) -> Result<Self::Return, Self::Error>;

    fn number_u64(self, value: u64) -> Result<Self::Return, Self::Error>;

    fn number_i64(self, value: i64) -> Result<Self::Return, Self::Error>;

    fn number_usize(self, value: usize) -> Result<Self::Return, Self::Error>;

    fn number_isize(self, value: isize) -> Result<Self::Return, Self::Error>;

    fn boolean(self) -> Result<Self::BooleanTracer, Self::Error>;

    fn bool(self, value: bool) -> Result<Self::Return, Self::Error>;

    fn null(self) -> Result<Self::Return, Self::Error>;

    fn value<T: IntoJSON>(self, value: T) -> Result<Self::Return, Self::Error>
    {
        value.into_json(self)
    }
}

pub trait ObjectTracer: Tracer
{
    type KeyTracer: KeyTracer<
        Return = Self,
        Error = Self::Error,
        ValueTracer: ValueTracer<
            Return = Self,
            Error = Self::Error>>;

    fn entry_key(self, value: &str) -> Result<<Self::KeyTracer as KeyTracer>::ValueTracer, Self::Error>;

    fn entry(self) -> Result<Self::KeyTracer, Self::Error>;

    fn end(self) -> Result<Self::Return, Self::Error>;
}

pub trait KeyTracer: Tracer
{
    type ValueTracer: ValueTracer<
        Return = Self::Return,
        Error = Self::Error>;

    fn key_append(self, char: char)
        -> Result<Self::ValueTracer, Self::Error>;

    fn key_over(self, chars: impl Iterator<Item = char>)
        -> Result<Self::ValueTracer, Self::Error>;
}

pub trait ArrayTracer: Tracer
{
    type ElementTracer: ValueTracer<
        Return = Self,
        Error = Self::Error>;

    fn element(self) -> Result<Self::ElementTracer, Self::Error>;

    fn end(self) -> Result<Self::Return, Self::Error>;
}

pub trait StringTracer: Tracer
{
    fn append(self, char: char)
        -> Result<Self, Self::Error>;

    fn over(self, chars: impl Iterator<Item = char>)
        -> Result<Self, Self::Error>;

    fn end(self) -> Result<Self::Return, Self::Error>;
}

pub trait NumberTracer: Tracer
{
    fn u8(self, value: u8) -> Result<Self::Return, Self::Error>;

    fn i8(self, value: i8) -> Result<Self::Return, Self::Error>;

    fn u16(self, value: u16) -> Result<Self::Return, Self::Error>;

    fn i16(self, value: i16) -> Result<Self::Return, Self::Error>;

    fn u32(self, value: u32) -> Result<Self::Return, Self::Error>;

    fn i32(self, value: i32) -> Result<Self::Return, Self::Error>;

    fn u64(self, value: u64) -> Result<Self::Return, Self::Error>;

    fn i64(self, value: i64) -> Result<Self::Return, Self::Error>;

    fn usize(self, value: usize) -> Result<Self::Return, Self::Error>;

    fn isize(self, value: isize) -> Result<Self::Return, Self::Error>;
}

pub trait BooleanTracer: Tracer
{
    fn bool(self, value: bool) -> Result<Self::Return, Self::Error>;
}