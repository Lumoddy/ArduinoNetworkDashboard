
pub trait WriteIntArrayName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteIntArray<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteIntArray
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [i32],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [u32],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteIntArrayContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: super::WriteInt<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteIntArrayContents
    : super::ChildWrite
    + super::WriteAppend<
        i32,
        Next = Self::Parent>
    + super::WriteAppendUnsigned<
        u32,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: super::WriteInt<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteIntArrayListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteIntArrayList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteIntArrayList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [&'a [i32]],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [&'a [u32]],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteIntArrayListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteIntArray<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteIntArrayListContents
    : super::ChildWrite
    + for<'a> super::WriteAppend<
        &'a [i32],
        Next = Self::Parent>
    + for<'a> super::WriteAppendUnsigned<
        &'a [u32],
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteIntArray<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }