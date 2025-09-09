
pub trait WriteByteName
{
    type Parent;
    type Error;
    type ValueWriter: WriteByte<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteByte
{
    type Parent;
    type Error;

    fn write(self, byte: i8) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, byte: u8) -> Result<Self::Parent, Self::Error>;
}

pub trait WriteByteListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteByteList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteByteList
{
    type Parent;
    type Error;

    fn write(self, array: &[i8]) -> Result<Self::Parent, Self::Error>;

    fn write_unsigned(self, array: &[u8]) -> Result<Self::Parent, Self::Error>;

    type WriteContents: WriteByteListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteByteListContents: Sized
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

pub trait WriteBoolName
{
    type Parent;
    type Error;
    type ValueWriter: WriteBool<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteBool
{
    type Parent;
    type Error;

    fn write(self, bool: bool) -> Result<Self::Parent, Self::Error>;
}