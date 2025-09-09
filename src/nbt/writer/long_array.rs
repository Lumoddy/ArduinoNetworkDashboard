
pub trait WriteLongArrayName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteLongArray<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteLongArray
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [i64],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [u64],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteLongArrayContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: super::WriteLong<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteLongArrayContents
    : super::ChildWrite
    + super::WriteAppend<
        i64,
        Next = Self::Parent>
    + super::WriteAppendUnsigned<
        u64,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: super::WriteLong<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteLongArrayListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteLongArrayList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteLongArrayList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [&'a [i64]],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [&'a [u64]],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteLongArrayListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteLongArray<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteLongArrayListContents
    : super::ChildWrite
    + for<'a> super::WriteAppend<
        &'a [i64],
        Next = Self::Parent>
    + for<'a> super::WriteAppendUnsigned<
        &'a [u64],
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteLongArray<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }