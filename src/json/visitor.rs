use super::conversion::FromJSON;

pub enum TypedValueVisitor<
    Object: ObjectVisitor,
    Array: ArrayVisitor,
    String: StringVisitor,
    Number: NumberVisitor,
    Boolean: Visitor,
    Null: Visitor>
{
    Object(Object),
    Array(Array),
    String(String),
    Number(Number),
    Boolean(Boolean),
    Null(Null),
}

pub trait Visitor: Sized
{
    type Error;
    type Return: Visitor<Error = Self::Error>;

    fn into_invalid_value_err(self, found: &'static str, expected: &'static str) -> Self::Error;

    fn into_field_not_found_err(self, expected: &'static str) -> Self::Error;

    fn into_duplicate_field_err(self, found: &'static str) -> Self::Error;

    fn into_invalid_field_err(self) -> Self::Error;
}

pub trait ValueVisitor: Visitor
{
    type ObjectVisitor: ObjectVisitor<
        Return = Self::Return,
        Error = Self::Error>;
    type ArrayVisitor: ArrayVisitor<
        Return = Self::Return,
        Error = Self::Error>;
    type StringVisitor: StringVisitor<
        Return = Self::Return,
        Error = Self::Error>;
    type NumberVisitor: NumberVisitor<
        Return = Self::Return,
        Error = Self::Error>;
    type BooleanVisitor: BooleanVisitor<
        Return = Self::Return,
        Error = Self::Error>;
    type NullVisitor: NullVisitor<
        Return = Self::Return,
        Error = Self::Error>;

    fn value(self) -> Result<
        TypedValueVisitor<
            Self::ObjectVisitor,
            Self::ArrayVisitor,
            Self::StringVisitor,
            Self::NumberVisitor,
            Self::BooleanVisitor,
            Self::NullVisitor>,
        Self::Error>;

    fn as_value<T: FromJSON>(self) -> Result<(T, Self::Return), Self::Error>
    {
        T::from_json(self)
    }
}

pub trait ObjectVisitor: Visitor
{
    type KeyVisitor: KeyVisitor<
        Return = Self,
        Error = Self::Error>;

    fn next(self) -> Result<Result<Self::KeyVisitor, Self::Return>, Self::Error>;
}

pub trait KeyVisitor: Visitor
{
    type ValueVisitor: ValueVisitor<
        Return = Self::Return,
        Error = Self::Error>;

    fn collect_key<const N: usize>(self)
        -> Result<(heapless::String<N>, Self::ValueVisitor), Self::Error>;
}

pub trait ArrayVisitor: Visitor
{
    type ElementVisitor: ValueVisitor<
        Return = Self,
        Error = Self::Error>;

    fn next(self) -> Result<Result<Self::ElementVisitor, Self::Return>, Self::Error>;
}

pub trait StringVisitor: Visitor
{
    fn next_char(self)
        -> Result<Result<(char, Self), Self::Return>, Self::Error>;

    fn collect_string<const N: usize>(self)
        -> Result<(heapless::String<N>, Self::Return), Self::Error>;
}

pub trait NumberVisitor: Visitor
{
    fn collect_u8(self) -> Result<(u8, Self::Return), Self::Error>;

    fn collect_i8(self) -> Result<(i8, Self::Return), Self::Error>;

    fn collect_u16(self) -> Result<(u16, Self::Return), Self::Error>;

    fn collect_i16(self) -> Result<(i16, Self::Return), Self::Error>;

    fn collect_u32(self) -> Result<(u32, Self::Return), Self::Error>;

    fn collect_i32(self) -> Result<(i32, Self::Return), Self::Error>;

    fn collect_u64(self) -> Result<(u64, Self::Return), Self::Error>;

    fn collect_i64(self) -> Result<(i64, Self::Return), Self::Error>;

    fn collect_usize(self) -> Result<(usize, Self::Return), Self::Error>;

    fn collect_isize(self) -> Result<(isize, Self::Return), Self::Error>;
}

pub trait BooleanVisitor: Visitor
{
    fn collect_bool(self) -> Result<(bool, Self::Return), Self::Error>;
}

pub trait NullVisitor: Visitor
{
    fn collect_null(self) -> Result<((), Self::Return), Self::Error>;
}