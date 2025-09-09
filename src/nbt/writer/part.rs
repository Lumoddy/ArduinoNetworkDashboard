
pub trait ErrorWrite
{
    type Error;
}

pub trait ChildWrite
{
    type Parent;
}

pub trait Write<Value>: ErrorWrite
{
    type Next;

    fn write(self, value: Value) -> Result<Self::Next, Self::Error>;
}

pub trait WriteUnsigned<Value>: ErrorWrite
{
    type Next;

    fn write_unsigned(self, value: Value) -> Result<Self::Next, Self::Error>;
}

pub trait WriteName: ErrorWrite
{
    type Next;

    fn name(self, name: &str) -> Result<Self::Next, Self::Error>;
}

pub trait WriteLen<Length = u32>: ErrorWrite
{
    type ChildNext;

    fn len(self, len: Length) -> Result<Self::ChildNext, Self::Error>;
}

pub trait WriteArray<Length = u32>: Sized + ErrorWrite
{
    type ChildWrite: ChildWrite<Parent = Self::Next> + ErrorWrite;
    type Next;

    fn each(
        self,
        len: Length,
        f: impl FnMut(Length, Self::ChildWrite)
            -> Result<
                <Self::ChildWrite as ChildWrite>::Parent,
                <Self::ChildWrite as ErrorWrite>::Error>)
        -> Result<Self::Next, Self::Error>;
}

pub trait WriteExtend<Index = u32>: Sized + ErrorWrite
{
    type ChildWrite: ChildWrite + ErrorWrite;
    type Next;

    fn each(
        self,
        f: impl FnMut(Index, Self::ChildWrite)
            -> Result<
                <Self::ChildWrite as ChildWrite>::Parent,
                <Self::ChildWrite as ErrorWrite>::Error>)
        -> Result<Self::Next, Self::Error>;

    fn end(self) -> Result<Result<Self::Next, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Index, Self::ChildWrite)
            -> Result<
                <Self::ChildWrite as ChildWrite>::Parent,
                <Self::ChildWrite as ErrorWrite>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}

pub trait WriteAppend<Value, Index = u32>
    : Sized
    + Write<
        Value,
        Next = Result<Self, Self>>
{
    type Next;

    fn map(
        self,
        f: impl FnMut(Index) -> Result<Value, Self::Error>)
        -> Result<<Self as WriteAppend<Value, Index>>::Next, Self::Error>;
}

pub trait WriteAppendUnsigned<Value, Index = u32>
    : Sized
    + WriteUnsigned<
        Value,
        Next = Result<Self, Self>>
{
    type Next;

    fn map_unsigned(
        self,
        f: impl FnMut(Index) -> Result<Value, Self::Error>)
        -> Result<<Self as WriteAppendUnsigned<Value, Index>>::Next, Self::Error>;
}