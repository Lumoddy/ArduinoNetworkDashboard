
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Type
{
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl TryFrom<u8> for Type
{
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error>
    {
        match value
        {
            1 => Ok(Type::Byte),
            2 => Ok(Type::Short),
            3 => Ok(Type::Int),
            4 => Ok(Type::Long),
            5 => Ok(Type::Float),
            6 => Ok(Type::Double),
            7 => Ok(Type::ByteArray),
            8 => Ok(Type::String),
            9 => Ok(Type::List),
            10 => Ok(Type::Compound),
            11 => Ok(Type::IntArray),
            12 => Ok(Type::LongArray),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ElementType
{
    Empty = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl TryFrom<u8> for ElementType
{
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error>
    {
        match value
        {
            0 => Ok(ElementType::Empty),
            1 => Ok(ElementType::Byte),
            2 => Ok(ElementType::Short),
            3 => Ok(ElementType::Int),
            4 => Ok(ElementType::Long),
            5 => Ok(ElementType::Float),
            6 => Ok(ElementType::Double),
            7 => Ok(ElementType::ByteArray),
            8 => Ok(ElementType::String),
            9 => Ok(ElementType::List),
            10 => Ok(ElementType::Compound),
            11 => Ok(ElementType::IntArray),
            12 => Ok(ElementType::LongArray),
            _ => Err(()),
        }
    }
}