
pub trait WriteShortName
{
    type Parent;
    type Error;
    type ValueWriter: WriteShort<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteShort
{
    type Parent;
    type Error;

    fn write(self, short: i16) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, short: u16) -> Result<Self::Parent, Self::Error>;
}

pub trait WriteShortListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteShortList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteShortList
{
    type Parent;
    type Error;

    fn write(self, array: &[i16]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[u16]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteShortListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteShortListContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteShort<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteShort>::Parent,
                <Self::WriteElement as WriteShort>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteShort>::Parent,
                <Self::WriteElement as WriteShort>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}