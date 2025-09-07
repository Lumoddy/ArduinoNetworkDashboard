use super::payload::PayloadWrite;
use super::primitive::PrimitiveSerializerWriteName;
use super::writer::{SerializerWriter, WriteReturn};
use super::PrimitiveListSerializerWriteName;

pub trait TypePickerSerializerWrite: WriteReturn
{
    fn byte(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i8,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn short(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i16,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn int(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i32,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn long(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i64,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn float(self) -> Result<
        impl PrimitiveSerializerWriteName<
            f32,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn double(self) -> Result<
        impl PrimitiveSerializerWriteName<
            f64,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn byte_array(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i8,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn string(self) -> Result<
        impl for<'a> PrimitiveSerializerWriteName<
            &'a str,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn compound(self) -> Result<
        impl CompoundSerializerWriteName<
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn int_array(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i32,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;

    fn long_array(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i64,
            Error = Self::Error,
            Return = Self::Return>,
        Self::Error>;
}

pub trait CompoundSerializerWrite: TypePickerSerializerWrite<Return = Self>
{
    type Parent;

    fn end(self) -> Result<Self::Parent, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    TypePickerSerializerWrite for SerializerWriter<E, F>
{
    #[inline]
    fn byte(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i8,
            Error = E,
            Return = Self>,
        E> { self.write(1i8) }

    #[inline]
    fn short(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i16,
            Error = E,
            Return = Self>,
        E> { self.write(2i8) }

    #[inline]
    fn int(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i32,
            Error = E,
            Return = Self>,
        E> { self.write(3i8) }

    #[inline]
    fn long(self) -> Result<
        impl PrimitiveSerializerWriteName<
            i64,
            Error = E,
            Return = Self>,
        E> { self.write(4i8) }

    #[inline]
    fn float(self) -> Result<
        impl PrimitiveSerializerWriteName<
            f32,
            Error = E,
            Return = Self>,
        E> { self.write(5i8) }

    #[inline]
    fn double(self) -> Result<
        impl PrimitiveSerializerWriteName<
            f64,
            Error = E,
            Return = Self>,
        E> { self.write(6i8) }

    #[inline]
    fn byte_array(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i8,
            Error = E,
            Return = Self>,
        E> { self.write(7i8) }

    #[inline]
    fn string(self) -> Result<
        impl for<'a> PrimitiveSerializerWriteName<
            &'a str,
            Error = E,
            Return = Self>,
        E> { self.write(8i8) }

    #[inline]
    fn compound(self) -> Result<
        impl CompoundSerializerWriteName<
            Error = E,
            Return = Self>,
        E> { self.write(10i8) }

    #[inline]
    fn int_array(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i32,
            Error = E,
            Return = Self>,
        E> { self.write(12i8) }

    #[inline]
    fn long_array(self) -> Result<
        impl PrimitiveListSerializerWriteName<
            i64,
            Error = E,
            Return = Self>,
        E> { self.write(12i8) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundSerializerWrite for SerializerWriter<E, F>
{
    type Parent = Self;

    #[inline]
    fn end(self) -> Result<Self, Self::Error> { self.write(0i8) }
}

pub trait CompoundSerializerWriteName: WriteReturn
{
    fn name(self, name: &str) -> Result<
        impl CompoundSerializerWrite<
            Error = Self::Error,
            Parent = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundSerializerWriteName for SerializerWriter<E, F>
{
    #[inline]
    fn name(self, name: &str) -> Result<
        impl CompoundSerializerWrite<
            Error = E,
            Parent = Self>,
        E> { self.write(name) }
}