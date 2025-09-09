
pub trait WriteByteArrayName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteByteArray<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteByteArray
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [i8],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [u8],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteByteArrayContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: super::WriteByte<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteByteArrayContents
    : super::ChildWrite
    + super::WriteAppend<
        i8,
        Next = Self::Parent>
    + super::WriteAppendUnsigned<
        u8,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: super::WriteByte<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteByteArrayListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteByteArrayList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteByteArrayList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [&'a [i8]],
        Next = Self::Parent>
    + for<'a> super::WriteUnsigned<
        &'a [&'a [u8]],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteByteArrayListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteByteArray<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteByteArrayListContents
    : super::ChildWrite
    + for<'a> super::WriteAppend<
        &'a [i8],
        Next = Self::Parent>
    + for<'a> super::WriteAppendUnsigned<
        &'a [u8],
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteByteArray<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }