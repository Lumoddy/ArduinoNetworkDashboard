
pub trait WriteListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteList: super::ChildWrite + super::ErrorWrite
{
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
    : super::ChildWrite
    + super::WriteName<
        Next: WriteListList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteListList
    : super::ChildWrite
    + super::WriteLen<
        ChildNext: WriteListListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteList<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteListListContents
    : super::ChildWrite
    + super::WriteExtend<
        ChildWrite: WriteList<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }