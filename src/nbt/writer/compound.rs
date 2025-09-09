use crate::nbt::Type;

pub trait WriteCompoundName
{
    type Parent;
    type Error;
    type ValueWriter: WriteCompound<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteCompound
{
    type Parent;
    type Error;

    fn end(self) -> Result<Self::Parent, Self::Error>;

    type WriteBool: super::WriteBoolName<
        Error = Self::Error,
        Parent = Self>;

    fn bool(self) -> Result<Self::WriteBool, Self::Error>;

    type WriteByte: super::WriteByteName<
        Error = Self::Error,
        Parent = Self>;

    fn byte(self) -> Result<Self::WriteByte, Self::Error>;

    type WriteShort: super::WriteShortName<
        Error = Self::Error,
        Parent = Self>;

    fn short(self) -> Result<Self::WriteShort, Self::Error>;

    type WriteInt: super::WriteIntName<
        Error = Self::Error,
        Parent = Self>;

    fn int(self) -> Result<Self::WriteInt, Self::Error>;

    type WriteLong: super::WriteLongName<
        Error = Self::Error,
        Parent = Self>;

    fn long(self) -> Result<Self::WriteLong, Self::Error>;

    type WriteByteArray: super::WriteByteArrayName<
        Error = Self::Error,
        Parent = Self>;

    fn byte_array(self) -> Result<Self::WriteByteArray, Self::Error>;

    type WriteString: super::WriteStringName<
        Error = Self::Error,
        Parent = Self>;

    fn string(self) -> Result<Self::WriteString, Self::Error>;

    type WriteList: super::WriteListName<
        Error = Self::Error,
        Parent = Self>;

    fn list(self) -> Result<Self::WriteList, Self::Error>;

    type WriteCompound: super::WriteCompoundName<
        Error = Self::Error,
        Parent = Self>;

    fn compound(self) -> Result<Self::WriteCompound, Self::Error>;

    type WriteIntArray: super::WriteIntArrayName<
        Error = Self::Error,
        Parent = Self>;

    fn int_array(self) -> Result<Self::WriteIntArray, Self::Error>;

    type WriteLongArray: super::WriteLongArrayName<
        Error = Self::Error,
        Parent = Self>;

    fn long_array(self) -> Result<Self::WriteLongArray, Self::Error>;
}

pub trait WriteCompoundListName
{
    type Parent;
    type Error;
    type ValueWriter: WriteCompoundList<Error = Self::Error, Parent = Self::Parent>;

    fn name(self, name: &str) -> Result<Self::ValueWriter, Self::Error>;
}

pub trait WriteCompoundList
{
    type Parent;
    type Error;

    type WriteContents: WriteCompoundListContents<
        Error = Self::Error,
        Parent = Self::Parent>;

    fn length(self, length: u32) -> Result<Self::WriteContents, Self::Error>;
}

pub trait WriteCompoundListContents: Sized
{
    type Parent;
    type Error;

    type WriteElement: WriteCompound<Error = Self::Error>;

    fn map(
        self,
        f: impl FnMut(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteCompound>::Parent,
                <Self::WriteElement as WriteCompound>::Error>)
            -> Result<Self::Parent, Self::Error>;

    fn end(self) -> Result<Result<Self::Parent, Self>, Self::Error>;

    fn enter(
        self,
        f: impl FnOnce(Self::WriteElement)
            -> Result<
                <Self::WriteElement as WriteCompound>::Parent,
                <Self::WriteElement as WriteCompound>::Error>)
        -> Result<Result<Self, Self>, Self::Error>;
}

// MARK: impl

impl<W: super::WriteRaw> WriteCompound for W
{
    type Parent = W;
    type Error = W::Error;

    fn end(mut self) -> Result<W, W::Error>
    { unsafe { self.write_end().map(|_| self) } }

    type WriteBool = W;

    fn bool(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Byte).map(|_| self) } }

    type WriteByte = W;

    fn byte(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Byte).map(|_| self) } }

    type WriteShort = W;

    fn short(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Short).map(|_| self) } }

    type WriteInt = W;

    fn int(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Int).map(|_| self) } }

    type WriteLong = W;

    fn long(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Long).map(|_| self) } }

    type WriteByteArray = W;

    fn byte_array(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::ByteArray).map(|_| self) } }

    type WriteString = W;

    fn string(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::String).map(|_| self) } }

    type WriteList = W;

    fn list(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::List).map(|_| self) } }

    type WriteCompound = W;

    fn compound(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::Compound).map(|_| self) } }

    type WriteIntArray = W;

    fn int_array(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::IntArray).map(|_| self) } }

    type WriteLongArray = W;

    fn long_array(mut self) -> Result<W, W::Error>
    { unsafe { self.write_type(Type::LongArray).map(|_| self) } }
}

impl<W: super::WriteRaw> WriteCompoundList for W
{
    type Parent = W;
    type Error = W::Error;

    type WriteContents = (usize, W);

    fn length(mut self, length: u32) -> Result<Self::WriteContents, Self::Error>
    { unsafe { self.write_length(length).map(|_| (length as usize, self)) } }
}