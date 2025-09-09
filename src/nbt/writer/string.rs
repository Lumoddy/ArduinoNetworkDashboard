
pub trait WriteStringName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteString<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteString
    : super::ChildWrite
    + for<'a> super::Write<
        &'a str,
        Next = Self::Parent> { }

pub trait WriteStringListName
    : super::ChildWrite
    + super::WriteName<
        Next: WriteStringList<
            Error = Self::Error,
            Parent = Self::Parent>> { }

pub trait WriteStringList
    : super::ChildWrite
    + for<'a> super::Write<
        &'a [&'a str],
        Next = Self::Parent>
    + super::WriteLen<
        ChildNext: WriteStringListContents<
            Error = Self::Error,
            Parent = Self::Parent>>
    + super::WriteArray<
        ChildWrite: WriteString<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }

pub trait WriteStringListContents
    : super::ChildWrite
    + for<'a> super::WriteAppend<
        &'a str,
        Next = Self::Parent>
    + super::WriteExtend<
        ChildWrite: WriteString<
            Error = Self::Error,
            Parent = Self::Parent>,
        Next = Self::Parent> { }