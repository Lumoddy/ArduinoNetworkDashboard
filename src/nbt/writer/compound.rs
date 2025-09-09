use crate::nbt::Type;

pub trait WriteCompoundName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteCompound<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteCompound: super::ChildWrite + super::ErrorWrite
{
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
    : super::ChildWrite
    + super::WriteName<
        Next: WriteCompoundList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteCompoundList
    : super::ChildWrite
    + super::WriteLen<
        ChildNext: WriteCompoundListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteCompound<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteCompoundListContents
    : super::ChildWrite
    + super::WriteExtend<
        ChildWrite: WriteCompound<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }