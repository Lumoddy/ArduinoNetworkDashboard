use super::{CompoundListSerializerWriteName, ListPayloadWrite, PrimitiveListListSerializerWriteName, PrimitiveListSerializerWriteName, SerializerWriter, Write, WriteReturn};

pub trait ListTypePickerSerializerWrite: WriteReturn
{
    fn of_byte(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i8,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_short(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i16,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_int(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i32,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_long(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i64,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_float(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            f32,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_double(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            f64,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_byte_array(self) -> Result<
        impl PrimitiveListListSerializerWriteName<
            i8,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_string(self) -> Result<
        impl for<'a> PrimitiveListSerializerWriteName<
            &'a str,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_compound(self) -> Result<
        impl CompoundListSerializerWriteName<
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_int_array(self) -> Result<
        impl PrimitiveListListSerializerWriteName<
            i32,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn of_long_array(self) -> Result<
        impl PrimitiveListListSerializerWriteName<
            i64,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;
}

pub trait ListFromSerializerWrite: WriteReturn
{
    type ElementWriter: WriteReturn;

    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::ElementWriter) -> Result<
            <Self::ElementWriter as WriteReturn>::Return,
            <Self::ElementWriter as Write>::Error>)
        -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    ListFromSerializerWrite for SerializerWriter<E, F>
{
    type ElementWriter = Self;

    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::ElementWriter) -> Result<Self, E>)
        -> Result<Self::Return, Self::Error> { self.write_list(length, f) }
}