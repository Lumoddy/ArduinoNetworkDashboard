
pub trait WriteByteName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteByte<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteByte
    : super::ChildWrite
    + super::Write<
        i8,
        Next = Self::Parent>
    + super::WriteUnsigned<
        u8,
        Next = Self::Parent> { }

pub trait WriteBoolName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteBool<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteBool
    : super::ChildWrite
    + super::Write<
        bool,
        Next = Self::Parent> { }

pub trait WriteByteListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteByteList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteByteList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [i8],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [u8],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteByteListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteByte<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteByteListContents
    : super::ChildWrite
    + super::WriteAppend<
        i8,
        Next = Self::Parent>
    + super::WriteAppendUnsigned<
        u8,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteByte<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }