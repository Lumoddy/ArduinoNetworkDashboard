
pub trait WriteLongName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteLong<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteLong
    : super::ChildWrite
    + super::Write<
        i64,
        Next = Self::Parent>
    + super::WriteUnsigned<
        u64,
        Next = Self::Parent> { }

pub trait WriteLongListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteLongList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteLongList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [i64],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [u64],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteLongListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteLong<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteLongListContents
    : super::ChildWrite
    + super::WriteAppend<
        i64,
        Next = Self::Parent>
    + super::WriteAppendUnsigned<
        u64,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteLong<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }