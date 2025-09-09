
pub trait WriteStringName
{
    type Parent;
    type Error;
    type ValueWriter: WriteString<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteString
{
    type Parent;
    type Error;

    fn write(self, array: &str) -> Result<Self::Parent, Self::Error>;
}

pub trait WriteStringListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteStringList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteStringList
{
    type Parent;
    type Error;

    fn write(self, array: &[&str]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteStringListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteStringListContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteString<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteString>::Parent,
                <Self::WriteElement as WriteString>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteString>::Parent,
                <Self::WriteElement as WriteString>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}