use crate::nbt::Type;

pub trait WriteListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteList
{
    type Parent;
    type Error;

    fn empty(self) -> Result<Self::Parent, Self::Error>;

    type WriteByte: super::WriteByteList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_byte(self) -> Result<Self::WriteByte, Self::Error>;

    type WriteShort: super::WriteShortList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_short(self) -> Result<Self::WriteShort, Self::Error>;

    type WriteInt: super::WriteIntList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_int(self) -> Result<Self::WriteInt, Self::Error>;

    type WriteLong: super::WriteLongList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_long(self) -> Result<Self::WriteLong, Self::Error>;

    type WriteByteArray: super::WriteByteArrayList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_byte_array(self) -> Result<Self::WriteByteArray, Self::Error>;

    type WriteString: super::WriteStringList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_string(self) -> Result<Self::WriteString, Self::Error>;

    type WriteList: super::WriteListList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_list(self) -> Result<Self::WriteList, Self::Error>;

    type WriteCompound: super::WriteCompoundList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_compound(self) -> Result<Self::WriteCompound, Self::Error>;

    type WriteIntArray: super::WriteIntArrayList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_int_array(self) -> Result<Self::WriteIntArray, Self::Error>;

    type WriteLongArray: super::WriteLongArrayList<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn of_long_array(self) -> Result<Self::WriteLongArray, Self::Error>;
}

pub trait WriteListListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteListList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteListList
{
    type Parent;
    type Error;

    type WriteContents: WriteListListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteListListContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteList<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteList>::Parent,
                <Self::WriteElement as WriteList>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteList>::Parent,
                <Self::WriteElement as WriteList>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}

// MARK: impl

impl<W: super::WriteRaw> WriteList for W
{
    type Parent = W;
    type Error = W::Error;

    fn empty(mut self) -> Result<W, W::Error>
    {
        unsafe
        {
            self.write_end()?;
            self.write_length(0)?;
            Ok(self)
        }
    }

    type WriteByte = W;

    fn of_byte(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Byte).map(|_| self) } }

    type WriteShort = W;

    fn of_short(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Short).map(|_| self) } }

    type WriteInt = W;

    fn of_int(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Int).map(|_| self) } }

    type WriteLong = W;

    fn of_long(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Long).map(|_| self) } }

    type WriteByteArray = W;

    fn of_byte_array(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::ByteArray).map(|_| self) } }

    type WriteString = W;

    fn of_string(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::String).map(|_| self) } }

    type WriteList = W;

    fn of_list(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::List).map(|_| self) } }

    type WriteCompound = W;

    fn of_compound(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Compound).map(|_| self) } }

    type WriteIntArray = W;

    fn of_int_array(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::IntArray).map(|_| self) } }

    type WriteLongArray = W;

    fn of_long_array(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::LongArray).map(|_| self) } }
}

impl<W: super::WriteRaw> WriteListList for W
{
    type Parent = W;
    type Error = W::Error;

    type WriteContents = (usize, W);

    fn length(mut self, length: u32) -> Result<Self::WriteContents, Self::Error>
    { unsafe { self.write_length(length).map(|_| (length as usize, self)) } }
}