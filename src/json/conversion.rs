use crate::json::Visitor;

use super::BooleanVisitor;
use super::NumberVisitor;
use super::StringVisitor;
use super::TypedValueVisitor;
use super::ValueTracer;
use super::ValueVisitor;

pub trait FromJSON: Sized
{
    fn from_json<V: ValueVisitor>(visitor: V) -> Result<(Self, V::Return), V::Error>;
}

macro_rules! _num_from_json_impl
{
    ($(impl FromJSON for $type:ty as $collect:ident;)+) =>
    {
        $(
            impl FromJSON for $type
            {
                fn from_json<V: ValueVisitor>(visitor: V)
                    -> Result<(Self, V::Return), V::Error>
                {
                    match visitor.value()?
                    {
                        TypedValueVisitor::Object(visitor) => return Err(
                            visitor.into_invalid_value_err("object", "number")),
                        TypedValueVisitor::Array(visitor) => return Err(
                            visitor.into_invalid_value_err("array", "number")),
                        TypedValueVisitor::String(visitor) => return Err(
                            visitor.into_invalid_value_err("string", "number")),
                        TypedValueVisitor::Number(visitor) => return Ok(
                            visitor.$collect()?),
                        TypedValueVisitor::Boolean(visitor) => return Err(
                            visitor.into_invalid_value_err("boolean", "number")),
                        TypedValueVisitor::Null(visitor) => return Err(
                            visitor.into_invalid_value_err("null", "number")),
                    }
                }
            }
        )+
    };
}

_num_from_json_impl!
{
    impl FromJSON for u8 as collect_u8;
    impl FromJSON for i8 as collect_i8;
    impl FromJSON for u16 as collect_u16;
    impl FromJSON for i16 as collect_i16;
    impl FromJSON for u32 as collect_u32;
    impl FromJSON for i32 as collect_i32;
    impl FromJSON for u64 as collect_u64;
    impl FromJSON for i64 as collect_i64;
    impl FromJSON for usize as collect_usize;
    impl FromJSON for isize as collect_isize;
}

impl FromJSON for bool
{
    fn from_json<V: ValueVisitor>(visitor: V)
        -> Result<(Self, V::Return), V::Error>
    {
        match visitor.value()?
        {
            TypedValueVisitor::Object(visitor) => return Err(
                visitor.into_invalid_value_err("object", "number")),
            TypedValueVisitor::Array(visitor) => return Err(
                visitor.into_invalid_value_err("array", "number")),
            TypedValueVisitor::String(visitor) => return Err(
                visitor.into_invalid_value_err("string", "number")),
            TypedValueVisitor::Number(visitor) => return Err(
                visitor.into_invalid_value_err("object", "number")),
            TypedValueVisitor::Boolean(visitor) => return Ok(
                visitor.collect_bool()?),
            TypedValueVisitor::Null(visitor) => return Err(
                visitor.into_invalid_value_err("null", "number")),
        }
    }
}

impl<const N: usize> FromJSON for heapless::String<N>
{
    fn from_json<V: ValueVisitor>(visitor: V)
        -> Result<(Self, V::Return), V::Error>
    {
        match visitor.value()?
        {
            TypedValueVisitor::Object(visitor) => return Err(
                visitor.into_invalid_value_err("object", "string")),
            TypedValueVisitor::Array(visitor) => return Err(
                visitor.into_invalid_value_err("array", "string")),
            TypedValueVisitor::String(visitor) => return Ok(
                visitor.collect_string()?),
            TypedValueVisitor::Number(visitor) => return Err(
                visitor.into_invalid_value_err("object", "string")),
            TypedValueVisitor::Boolean(visitor) => return Err(
                visitor.into_invalid_value_err("object", "string")),
            TypedValueVisitor::Null(visitor) => return Err(
                visitor.into_invalid_value_err("null", "string")),
        }
    }
}

pub trait IntoJSON: Sized
{
    fn into_json<T: ValueTracer>(self, tracer: T) -> Result<T::Return, T::Error>;
}


macro_rules! _num_into_json_impl
{
    ($(impl IntoJSON for $type:ty as $number:ident;)+) =>
    {
        $(
            impl IntoJSON for $type
            {
                fn into_json<T: ValueTracer>(self, tracer: T)
                    -> Result<T::Return, T::Error>
                {
                    tracer.$number(self)
                }
            }
        )+
    };
}

_num_into_json_impl!
{
    impl IntoJSON for u8 as number_u8;
    impl IntoJSON for i8 as number_i8;
    impl IntoJSON for u16 as number_u16;
    impl IntoJSON for i16 as number_i16;
    impl IntoJSON for u32 as number_u32;
    impl IntoJSON for i32 as number_i32;
    impl IntoJSON for u64 as number_u64;
    impl IntoJSON for i64 as number_i64;
    impl IntoJSON for usize as number_usize;
    impl IntoJSON for isize as number_isize;
}

impl IntoJSON for bool
{
    fn into_json<T: ValueTracer>(self, tracer: T) -> Result<T::Return, T::Error>
    {
        tracer.bool(self)
    }
}

impl IntoJSON for &str
{
    fn into_json<T: ValueTracer>(self, tracer: T) -> Result<T::Return, T::Error>
    {
        tracer.str(self)
    }
}