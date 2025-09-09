
pub trait WriteIntName
{
    type Parent;
    type Error;
    type ValueWriter: WriteInt<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteInt
{
    type Parent;
    type Error;

    fn write(self, int: i32) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, int: u32) -> Result<Self::Parent, Self::Error>;
}

pub trait WriteIntListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteIntList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteIntList
{
    type Parent;
    type Error;

    fn write(self, array: &[i32]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[u32]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteIntListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteIntListContents: Sized
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