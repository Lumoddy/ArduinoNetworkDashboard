use super::WriteLong;

pub trait WriteLongArrayName
{
    type Parent;
    type Error;
    type ValueWriter: WriteLongArray<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteLongArray
{
    type Parent;
    type Error;

    fn write(self, array: &[i64]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[u64]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteLongArrayContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteLongArrayContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteLong<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteLong>::Parent,
                <Self::WriteElement as WriteLong>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteLong>::Parent,
                <Self::WriteElement as WriteLong>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}

pub trait WriteLongArrayListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteLongArrayList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteLongArrayList
{
    type Parent;
    type Error;

    fn write(self, array: &[&[i64]]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[&[u64]]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteLongArrayListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteLongArrayListContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteLongArray<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteLongArray>::Parent,
                <Self::WriteElement as WriteLongArray>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteLongArray>::Parent,
                <Self::WriteElement as WriteLongArray>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}