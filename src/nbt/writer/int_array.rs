use super::WriteInt;

pub trait WriteIntArrayName
{
    type Parent;
    type Error;
    type ValueWriter: WriteIntArray<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteIntArray
{
    type Parent;
    type Error;

    fn write(self, array: &[i32]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[u32]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteIntArrayContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteIntArrayContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteInt<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteInt>::Parent,
                <Self::WriteElement as WriteInt>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteInt>::Parent,
                <Self::WriteElement as WriteInt>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}

pub trait WriteIntArrayListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteIntArrayList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteIntArrayList
{
    type Parent;
    type Error;

    fn write(self, array: &[&[i32]]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[&[u32]]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteIntArrayListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteIntArrayListContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteIntArray<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteIntArray>::Parent,
                <Self::WriteElement as WriteIntArray>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteIntArray>::Parent,
                <Self::WriteElement as WriteIntArray>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}