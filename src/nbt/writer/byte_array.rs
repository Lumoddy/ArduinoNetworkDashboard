use super::WriteByte;

pub trait WriteByteArrayName
{
    type Parent;
    type Error;
    type ValueWriter: WriteByteArray<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteByteArray
{
    type Parent;
    type Error;

    fn write(self, array: &[i8]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[u8]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteByteArrayContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteByteArrayContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteByte<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteByte>::Parent,
                <Self::WriteElement as WriteByte>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteByte>::Parent,
                <Self::WriteElement as WriteByte>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}

pub trait WriteByteArrayListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteByteArrayList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteByteArrayList
{
    type Parent;
    type Error;

    fn write(self, array: &[&[i8]]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[&[u8]]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteByteArrayListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteByteArrayListContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteByteArray<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteByteArray>::Parent,
                <Self::WriteElement as WriteByteArray>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteByteArray>::Parent,
                <Self::WriteElement as WriteByteArray>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}