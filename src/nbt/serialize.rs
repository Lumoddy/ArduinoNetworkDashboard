use core::convert::Infallible;
use core::error;
use core::marker::PhantomData;

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use ufmt::uWrite;

use crate::common::serialize::{Serialize, SerializeResult};

pub trait SerializeAsTag
{
    type Error;

    fn serialize<W: NamedTagListWriter>(self, writer: W)
        -> Result<Result<W::Return, Self::Error>, W::Error>;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Endian
{
    Little,
    Big,
}

pub struct Serializer<Value: SerializeAsTag>
{
    _value: Value,
    _endian: Endian,
}

impl<Value: SerializeAsTag> Serializer<Value>
{
    pub const fn new(value: Value, endian: Endian) -> Self
    {
        Self { _value: value, _endian: endian }
    }
}

impl<Value: SerializeAsTag> Serialize for Serializer<Value>
{
    type Word = u8;
    type Error = Value::Error;

    fn drain<
        F: FnMut(u8) -> Result<(), E>,
        E>(self, f: F) -> SerializeResult<E, Value::Error>
    {
        match self._value.serialize(
            _SerializerWriter { f, endian: self._endian })
        {
            Ok(Ok(_)) => SerializeResult::Ok,
            Ok(Err(error)) => SerializeResult::SerializeErr(error),
            Err(error) => SerializeResult::DrainErr(error),
        }
    }

    fn drain_infallible<F: FnMut(u8)>(self, f: F)
        -> SerializeResult<Infallible, Value::Error>
    {
        let mut f = f;
        let f = |byte| Ok::<(), Infallible>(f(byte));
        match self._value.serialize(
            _SerializerWriter { f, endian: self._endian })
        {
            Ok(Ok(_)) => SerializeResult::Ok,
            Ok(Err(error)) => SerializeResult::SerializeErr(error),
        }
    }
}

struct _SerializerWriter<E, F: FnMut(u8) -> Result<(), E>>
{
    pub f: F,
    pub endian: Endian,
}

impl<E, F: FnMut(u8) -> Result<(), E>> _SerializerWriter<E, F>
{
    fn _f_byte_payload(mut self, payload: i8) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }

    fn _f_short_payload(mut self, payload: i16) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }

    fn _f_int_payload(mut self, payload: i32) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }

    fn _f_long_payload(mut self, payload: i64) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }

    fn _f_float_payload(mut self, payload: f32) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }

    fn _f_double_payload(mut self, payload: f64) -> Result<Self, E>
    {
        for byte in match (payload, self.endian)
        {
            (value, Endian::Little) => value.to_le_bytes(),
            (value, Endian::Big) => value.to_be_bytes(),
        }
        { (self.f)(byte)? }

        Ok(self)
    }

    fn _f_string_payload(mut self, payload: &str) -> Result<Self, E>
    {
        self = self._f_short_payload(payload.len() as i16)?;

        for byte in payload.as_bytes()
        { (self.f)(*byte)? }

        Ok(self)
    }

    fn _f_list_from_closure(
        mut self,
        length: u32,
        mut f: impl FnMut(u32, Self) -> Result<Self, E>) -> Result<Self, E>
    {
        self = self._f_int_payload(length as i32)?;
        for i in 0..length
        { self = f(i, self)?; }

        Ok(self)
    }
}

// MARK: NamedTagListWriter
pub trait NamedTagListWriter
{
    type Error;
    type Return;

    fn byte(self) -> Result<
        impl SimpleValueNameWriter<i8, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn short(self) -> Result<
        impl SimpleValueNameWriter<i16, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn int(self) -> Result<
        impl SimpleValueNameWriter<i32, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn long(self) -> Result<
        impl SimpleValueNameWriter<i64, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn float(self) -> Result<
        impl SimpleValueNameWriter<f32, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn double(self) -> Result<
        impl SimpleValueNameWriter<f64, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn string(self) -> Result<
        impl StrValueNameWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn byte_array(self) -> Result<
        impl SimpleListNameWriter<i8, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn list(self) -> Result<
        impl ListElementTypeNameWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn begin_compound(self) -> Result<
        impl CompoundNameWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn int_array(self) -> Result<
        impl SimpleListNameWriter<i32, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn long_array(self) -> Result<
        impl SimpleListNameWriter<i64, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    NamedTagListWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn byte(mut self) -> Result<
        impl SimpleValueNameWriter<i8, Error = E, Return = Self>,
        E> { (self.f)(1)?; Ok(self) }

    #[inline]
    fn short(mut self) -> Result<
        impl SimpleValueNameWriter<i16, Error = E, Return = Self>,
        E> { (self.f)(2)?; Ok(self) }

    #[inline]
    fn int(mut self) -> Result<
        impl SimpleValueNameWriter<i32, Error = E, Return = Self>,
        E> { (self.f)(3)?; Ok(self) }

    #[inline]
    fn long(mut self) -> Result<
        impl SimpleValueNameWriter<i64, Error = E, Return = Self>,
        E> { (self.f)(4)?; Ok(self) }

    #[inline]
    fn float(mut self) -> Result<
        impl SimpleValueNameWriter<f32, Error = E, Return = Self>,
        E> { (self.f)(5)?; Ok(self) }

    #[inline]
    fn double(mut self) -> Result<
        impl SimpleValueNameWriter<f64, Error = E, Return = Self>,
        E> { (self.f)(6)?; Ok(self) }

    #[inline]
    fn string(mut self) -> Result<
        impl StrValueNameWriter<Error = E, Return = Self>,
        E> { (self.f)(7)?; Ok(self) }

    #[inline]
    fn byte_array(mut self) -> Result<
        impl SimpleListNameWriter<i8, Error = E, Return = Self>,
        E> { (self.f)(8)?; Ok(self) }

    #[inline]
    fn list(mut self) -> Result<
        impl ListElementTypeNameWriter<Error = E, Return = Self>,
        E> { (self.f)(9)?; Ok(self) }

    #[inline]
    fn begin_compound(mut self) -> Result<
        impl CompoundNameWriter<Error = E, Return = Self>,
        E> { (self.f)(10)?; Ok(self) }

    #[inline]
    fn int_array(mut self) -> Result<
        impl SimpleListNameWriter<i32, Error = E, Return = Self>,
        E> { (self.f)(11)?; Ok(self) }

    #[inline]
    fn long_array(mut self) -> Result<
        impl SimpleListNameWriter<i64, Error = E, Return = Self>,
        E> { (self.f)(12)?; Ok(self) }
}

// MARK: ListElementTypeNameWriter
pub trait ListElementTypeNameWriter
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl ListElementTypeWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    ListElementTypeNameWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl ListElementTypeWriter<Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: ListElementTypeWriter
pub trait ListElementTypeWriter
{
    type Error;
    type Return;

    fn empty(self) -> Result<
        impl EmptyListNameWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_byte(self) -> Result<
        impl SimpleListNameWriter<i8, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_short(self) -> Result<
        impl SimpleListNameWriter<i16, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_int(self) -> Result<
        impl SimpleListNameWriter<i32, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_long(self) -> Result<
        impl SimpleListNameWriter<i64, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_float(self) -> Result<
        impl SimpleListNameWriter<f32, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_double(self) -> Result<
        impl SimpleListNameWriter<f64, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_string(self) -> Result<
        impl StrListNameWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_byte_array(self) -> Result<
        impl SimpleListListNameWriter<i8, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_list(self) -> Result<
        impl TypedListListNameWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_compound(self) -> Result<
        impl CompoundListNameWriter<Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_int_array(self) -> Result<
        impl SimpleListListNameWriter<i32, Error = Self::Error, Return = Self::Return>,
        Self::Error>;

    fn of_long_array(self) -> Result<
        impl SimpleListListNameWriter<i64, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    ListElementTypeWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn empty(mut self)
        -> Result<impl EmptyListNameWriter<Error = E, Return = Self>, E>
    { (self.f)(1)?; Ok(self) }

    #[inline]
    fn of_byte(mut self)
        -> Result<impl SimpleListNameWriter<i8, Error = E, Return = Self>, E>
    { (self.f)(1)?; Ok(self) }

    #[inline]
    fn of_short(mut self)
        -> Result<impl SimpleListNameWriter<i16, Error = E, Return = Self>, E>
    { (self.f)(2)?; Ok(self) }

    #[inline]
    fn of_int(mut self)
        -> Result<impl SimpleListNameWriter<i32, Error = E, Return = Self>, E>
    { (self.f)(3)?; Ok(self) }

    #[inline]
    fn of_long(mut self)
        -> Result<impl SimpleListNameWriter<i64, Error = E, Return = Self>, E>
    { (self.f)(4)?; Ok(self) }

    #[inline]
    fn of_float(mut self)
        -> Result<impl SimpleListNameWriter<f32, Error = E, Return = Self>, E>
    { (self.f)(5)?; Ok(self) }

    #[inline]
    fn of_double(mut self)
        -> Result<impl SimpleListNameWriter<f64, Error = E, Return = Self>, E>
    { (self.f)(6)?; Ok(self) }

    #[inline]
    fn of_string(mut self)
        -> Result<impl StrListNameWriter<Error = E, Return = Self>, E>
    { (self.f)(7)?; Ok(self) }

    #[inline]
    fn of_byte_array(mut self)
        -> Result<impl SimpleListListNameWriter<i8, Error = E, Return = Self>, E>
    { (self.f)(8)?; Ok(self) }

    #[inline]
    fn of_list(mut self)
        -> Result<impl TypedListListNameWriter<Error = E, Return = Self>, E>
    { (self.f)(9)?; Ok(self) }

    #[inline]
    fn of_compound(mut self)
        -> Result<impl CompoundListNameWriter<Error = E, Return = Self>, E>
    { (self.f)(10)?; Ok(self) }

    #[inline]
    fn of_int_array(mut self)
        -> Result<impl SimpleListListNameWriter<i32, Error = E, Return = Self>, E>
    { (self.f)(11)?; Ok(self) }

    #[inline]
    fn of_long_array(mut self)
        -> Result<impl SimpleListListNameWriter<i64, Error = E, Return = Self>, E>
    { (self.f)(12)?; Ok(self) }
}

// MARK: CompoundNameWriter
pub trait CompoundNameWriter
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl CompoundContentWriter<Error = Self::Error, End = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundNameWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl CompoundContentWriter<Error = Self::Error, End = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: CompoundContentWriter
pub trait CompoundContentWriter: NamedTagListWriter<Return = Self>
{
    type End;

    fn end_compound(self) -> Result<Self::End, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundContentWriter for _SerializerWriter<E, F>
{
    type End = Self;

    #[inline]
    fn end_compound(mut self) -> Result<Self, E>
    { (self.f)(0)?; Ok(self) }
}

// MARK: SimpleValueNameWriter
pub trait SimpleValueNameWriter<Value>
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl SimpleValueWriter<Value, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueNameWriter<i8> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleValueWriter<i8, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueNameWriter<i16> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleValueWriter<i16, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueNameWriter<i32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleValueWriter<i32, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueNameWriter<i64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleValueWriter<i64, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueNameWriter<f32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleValueWriter<f32, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueNameWriter<f64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleValueWriter<f64, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: SimpleValueWriter
pub trait SimpleValueWriter<Value>
{
    type Error;
    type Return;

    fn value(self, value: Value) -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueWriter<i8> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn value(mut self, value: i8) -> Result<Self, E>
    { self = self._f_byte_payload(value)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueWriter<i16> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn value(mut self, value: i16) -> Result<Self, E>
    { self = self._f_short_payload(value)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueWriter<i32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn value(mut self, value: i32) -> Result<Self, E>
    { self = self._f_int_payload(value)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueWriter<i64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn value(mut self, value: i64) -> Result<Self, E>
    { self = self._f_long_payload(value)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueWriter<f32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn value(mut self, value: f32) -> Result<Self, E>
    { self = self._f_float_payload(value)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleValueWriter<f64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn value(mut self, value: f64) -> Result<Self, E>
    { self = self._f_double_payload(value)?; Ok(self) }
}

// MARK: StrValueNameWriter
pub trait StrValueNameWriter
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl StrValueWriter<Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    StrValueNameWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl StrValueWriter<Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: StrValueWriter
pub trait StrValueWriter
{
    type Error;
    type Return;

    fn value(self, value: &str) -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    StrValueWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn value(mut self, value: &str) -> Result<Self, E>
    { self = self._f_string_payload(value)?; Ok(self) }
}

// MARK: EmptyListNameWriter
pub trait EmptyListNameWriter
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    EmptyListNameWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<Self, E>
    { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: SimpleListNameWriter
pub trait SimpleListNameWriter<Value>
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl SimpleListWriter<Value, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListNameWriter<i8> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleListWriter<i8, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListNameWriter<i16> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleListWriter<i16, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListNameWriter<i32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleListWriter<i32, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListNameWriter<i64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleListWriter<i64, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListNameWriter<f32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleListWriter<f32, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListNameWriter<f64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl SimpleListWriter<f64, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: SimpleListWriter
pub trait SimpleListWriter<Value>
{
    type Error;
    type Return;
    type Writer: SimpleValueWriter<Value, Error = Self::Error>;

    fn of(self, elements: &[Value]) -> Result<Self::Return, Self::Error>;

    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<
            <Self::Writer as SimpleValueWriter<Value>>::Return,
            Self::Error>)
        -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListWriter<i8> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[i8]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for element in elements
        { self = self._f_byte_payload(*element)? }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListWriter<i16> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[i16]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for element in elements
        { self = self._f_short_payload(*element)? }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListWriter<i32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[i32]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for element in elements
        { self = self._f_int_payload(*element)? }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListWriter<i64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[i64]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for element in elements
        { self = self._f_long_payload(*element)? }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListWriter<f32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[f32]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for element in elements
        { self = self._f_float_payload(*element)? }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListWriter<f64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[f64]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for element in elements
        { self = self._f_double_payload(*element)? }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

// MARK: StrListNameWriter
pub trait StrListNameWriter
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl StrListWriter<Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    StrListNameWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl StrListWriter<Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: StrListWriter
pub trait StrListWriter
{
    type Error;
    type Return;
    type Writer: StrValueWriter<Error = Self::Error>;

    fn of(self, elements: &[&str]) -> Result<Self::Return, Self::Error>;

    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<
            <Self::Writer as StrValueWriter>::Return,
            Self::Error>)
        -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    StrListWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[&str]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for element in elements
        { self = self._f_string_payload(*element)? }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

// MARK: SimpleListListNameWriter
pub trait SimpleListListNameWriter<Value>
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl SimpleListListWriter<Value, Error = Self::Error, Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListNameWriter<i8> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    fn name(mut self, name: &str) -> Result<
        impl SimpleListListWriter<i8, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListNameWriter<i16> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    fn name(mut self, name: &str) -> Result<
        impl SimpleListListWriter<i16, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListNameWriter<i32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    fn name(mut self, name: &str) -> Result<
        impl SimpleListListWriter<i32, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListNameWriter<i64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    fn name(mut self, name: &str) -> Result<
        impl SimpleListListWriter<i64, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListNameWriter<f32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    fn name(mut self, name: &str) -> Result<
        impl SimpleListListWriter<f32, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListNameWriter<f64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    fn name(mut self, name: &str) -> Result<
        impl SimpleListListWriter<f64, Error = Self::Error, Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: SimpleListListWriter
pub trait SimpleListListWriter<Value>
{
    type Error;
    type Return;
    type Writer: SimpleValueWriter<Value, Error = Self::Error>;

    fn of(self, elements: &[&[Value]]) -> Result<Self::Return, Self::Error>;

    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<
            <Self::Writer as SimpleValueWriter<Value>>::Return,
            Self::Error>)
        -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListWriter<i8> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[&[i8]]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for list in elements
        {
            self = self._f_int_payload(list.len() as i32)?;
            for element in *list
            { self = self._f_byte_payload(*element)? }
        }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListWriter<i16> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[&[i16]]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for list in elements
        {
            self = self._f_int_payload(list.len() as i32)?;
            for element in *list
            { self = self._f_short_payload(*element)? }
        }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListWriter<i32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[&[i32]]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for list in elements
        {
            self = self._f_int_payload(list.len() as i32)?;
            for element in *list
            { self = self._f_int_payload(*element)? }
        }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListWriter<i64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[&[i64]]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for list in elements
        {
            self = self._f_int_payload(list.len() as i32)?;
            for element in *list
            { self = self._f_long_payload(*element)? }
        }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListWriter<f32> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[&[f32]]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for list in elements
        {
            self = self._f_int_payload(list.len() as i32)?;
            for element in *list
            { self = self._f_float_payload(*element)? }
        }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    SimpleListListWriter<f64> for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn of(mut self, elements: &[&[f64]]) -> Result<Self, E>
    {
        self = self._f_int_payload(elements.len() as i32)?;
        for list in elements
        {
            self = self._f_int_payload(list.len() as i32)?;
            for element in *list
            { self = self._f_double_payload(*element)? }
        }

        Ok(self)
    }

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

// MARK: TypedListListNameWriter
pub trait TypedListListNameWriter
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl TypedListListWriter<Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    TypedListListNameWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    fn name(mut self, name: &str) -> Result<
        impl TypedListListWriter<Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: TypedListListWriter
pub trait TypedListListWriter
{
    type Error;
    type Return;
    type Writer: ListElementTypeWriter<Error = Self::Error>;

    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<
            <Self::Writer as ListElementTypeWriter>::Return,
            Self::Error>)
        -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    TypedListListWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}

// MARK: CompoundListNameWriter
pub trait CompoundListNameWriter
{
    type Error;
    type Return;

    fn name(self, name: &str) -> Result<
        impl CompoundListWriter<Return = Self::Return>,
        Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundListNameWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;

    #[inline]
    fn name(mut self, name: &str) -> Result<
        impl CompoundListWriter<Return = Self>,
        E> { self = self._f_string_payload(name)?; Ok(self) }
}

// MARK: CompoundListWriter
pub trait CompoundListWriter
{
    type Error;
    type Return;
    type Writer: CompoundContentWriter<Error = Self::Error>;

    fn from(
        self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self::Writer, Self::Error>)
        -> Result<Self::Return, Self::Error>;
}

impl<E, F: FnMut(u8) -> Result<(), E>>
    CompoundListWriter for _SerializerWriter<E, F>
{
    type Error = E;
    type Return = Self;
    type Writer = Self;

    #[inline]
    fn from(
        mut self,
        length: u32,
        f: impl FnMut(u32, Self::Writer) -> Result<Self, E>)
        -> Result<Self, E>
    {
        self = self._f_list_from_closure(length, f)?;
        Ok(self)
    }
}