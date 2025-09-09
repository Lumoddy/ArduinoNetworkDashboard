
pub trait WriteLongName
{
    type Parent;
    type Error;
    type ValueWriter: WriteLong<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteLong
{
    type Parent;
    type Error;

    fn write(self, long: i64) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, long: u64) -> Result<Self::Parent, Self::Error>;
}

pub trait WriteLongListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteLongList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteLongList
{
    type Parent;
    type Error;

    fn write(self, array: &[i64]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[u64]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteLongListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteLongListContents: Sized
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