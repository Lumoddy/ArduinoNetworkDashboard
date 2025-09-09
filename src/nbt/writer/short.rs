
pub trait WriteShortName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteShort<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteShort
    : super::ChildWrite
    + super::Write<
        i16,
        Next = Self::Parent>
    + super::WriteUnsigned<
        u16,
        Next = Self::Parent> { }

pub trait WriteShortListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteShortList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteShortList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [i16],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [u16],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteShortListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteShort<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteShortListContents
    : super::ChildWrite
    + super::WriteAppend<
        i16,
        Next = Self::Parent>
    + super::WriteAppendUnsigned<
        u16,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteShort<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }