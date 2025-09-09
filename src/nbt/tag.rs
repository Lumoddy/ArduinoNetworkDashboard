
pub trait TypeMap
{
    type Byte;
    type Short;
    type Int;
    type Long;
    type Float;
    type Double;
    type ByteArray;
    type String;
    type List;
    type Compound;
    type IntArray;
    type LongArray;
}

impl TypeMap for ()
{
    type Byte = ();
    type Short = ();
    type Int = ();
    type Long = ();
    type Float = ();
    type Double = ();
    type ByteArray = ();
    type String = ();
    type List = ();
    type Compound = ();
    type IntArray = ();
    type LongArray = ();
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type
{
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeVariant<Map: TypeMap>
{
    Byte(Map::Byte),
    Short(Map::Short),
    Int(Map::Int),
    Long(Map::Long),
    Float(Map::Float),
    Double(Map::Double),
    ByteArray(Map::ByteArray),
    String(Map::String),
    List(Map::List),
    Compound(Map::Compound),
    IntArray(Map::IntArray),
    LongArray(Map::LongArray),
}

impl<Map: TypeMap> TypeVariant<Map>
{
    fn variant_type(&self) -> Type
    {
        match self
        {
            Self::Byte(_) => Type::Byte,
            Self::Short(_) => Type::Short,
            Self::Int(_) => Type::Int,
            Self::Long(_) => Type::Long,
            Self::Float(_) => Type::Float,
            Self::Double(_) => Type::Double,
            Self::ByteArray(_) => Type::ByteArray,
            Self::String(_) => Type::String,
            Self::List(_) => Type::List,
            Self::Compound(_) => Type::Compound,
            Self::IntArray(_) => Type::IntArray,
            Self::LongArray(_) => Type::LongArray,
        }
    }
}