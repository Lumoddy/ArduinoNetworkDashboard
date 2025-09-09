
pub trait WriteIntName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteInt<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteInt
    : super::ChildWrite
    + super::Write<
        i32,
        Next = Self::Parent>
    + super::WriteUnsigned<
        u32,
        Next = Self::Parent> { }

pub trait WriteIntListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteIntList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteIntList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [i32],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [u32],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteIntListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteInt<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteIntListContents
    : super::ChildWrite
    + super::WriteAppend<
        i32,
        Next = Self::Parent>
    + super::WriteAppendUnsigned<
        u32,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteInt<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }