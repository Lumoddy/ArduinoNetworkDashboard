use crate::nbt::{Endian, Type};

use super::*;

pub const MAX_LIST_LENGTH: usize = 256;

pub enum ClosureWriterError<Closure>
{
    LengthOverflow,
    FromClosure(Closure),
}

impl<Closure> From<Closure> for ClosureWriterError<Closure>
{
    fn from(value: Closure) -> Self { Self::FromClosure(value) }
}

pub struct ClosureRawWriter<E, F: FnMut(u8) -> Result<(), E>>
{
    _closure: F,
    _endian: Endian,
}

pub fn new<E, F: FnMut(u8) -> Result<(), E>>(endian: Endian, f: F)
    -> ClosureRawWriter<E, F>
{
    ClosureRawWriter { _closure: f, _endian: endian }
}

impl<E, F: FnMut(u8) -> Result<(), E>> ClosureRawWriter<E, F>
{
    pub fn safe<'a>(&'a mut self)
        -> impl use<'a, E, F> + WriteCompound<Error = ClosureWriterError<E>>
    {
        _InternalClosureWriter { _raw: self }
    }
}

impl<E, F: FnMut(u8) -> Result<(), E>> ClosureRawWriter<E, F>
{
    pub unsafe fn write_type(&mut self, value: Type)
        -> Result<(), ClosureWriterError<E>>
    {
        Ok((self._closure)(match value
        {
            Type::Byte => 1,
            Type::Short => 2,
            Type::Int => 3,
            Type::Long => 4,
            Type::Float => 5,
            Type::Double => 6,
            Type::ByteArray => 7,
            Type::String => 8,
            Type::List => 9,
            Type::Compound => 10,
            Type::IntArray => 11,
            Type::LongArray => 12,
        })?)
    }

    pub unsafe fn write_name(&mut self, name: &str)
        -> Result<(), ClosureWriterError<E>>
    { self.write_string(name) }

    pub unsafe fn write_end(&mut self)
        -> Result<(), ClosureWriterError<E>>
    { Ok((self._closure)(0)?) }

    pub unsafe fn write_bool(&mut self, value: bool)
        -> Result<(), ClosureWriterError<E>>
    { Ok((self._closure)(if value { 1 } else { 0 })?) }

    pub unsafe fn write_byte(&mut self, value: i8)
        -> Result<(), ClosureWriterError<E>>
    { self.write_byte_unsigned(value as u8) }

    pub unsafe fn write_byte_unsigned(&mut self, value: u8)
        -> Result<(), ClosureWriterError<E>>
    { Ok((self._closure)(value)?) }

    pub unsafe fn write_short(&mut self, value: i16)
        -> Result<(), ClosureWriterError<E>>
    { self.write_short_unsigned(value as u16) }

    pub unsafe fn write_short_unsigned(&mut self, value: u16)
        -> Result<(), ClosureWriterError<E>>
    {
        for byte in match self._endian
        {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
        { (self._closure)(byte)? }

        Ok(())
    }

    pub unsafe fn write_int(&mut self, value: i32)
        -> Result<(), ClosureWriterError<E>>
    { self.write_int_unsigned(value as u32) }

    pub unsafe fn write_int_unsigned(&mut self, value: u32)
        -> Result<(), ClosureWriterError<E>>
    {
        for byte in match self._endian
        {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
        { (self._closure)(byte)? }

        Ok(())
    }

    pub unsafe fn write_long(&mut self, value: i64)
        -> Result<(), ClosureWriterError<E>>
    { self.write_long_unsigned(value as u64) }

    pub unsafe fn write_long_unsigned(&mut self, value: u64)
        -> Result<(), ClosureWriterError<E>>
    {
        for byte in match self._endian
        {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
        { (self._closure)(byte)? }

        Ok(())
    }

    pub unsafe fn write_length(&mut self, value: u32)
        -> Result<(), ClosureWriterError<E>>
    {
        if value > MAX_LIST_LENGTH as u32
        { return Err(ClosureWriterError::LengthOverflow) }

        self.write_int_unsigned(value)?;

        Ok(())
    }

    pub unsafe fn write_string(&mut self, value: &str)
        -> Result<(), ClosureWriterError<E>>
    {
        self.write_short_unsigned(value.len() as u16);

        for byte in value.as_bytes()
        { (self._closure)(*byte)? }

        Ok(())
    }
}

struct _InternalClosureWriter<'a, E, F: FnMut(u8) -> Result<(), E>>
{
    _raw: &'a mut ClosureRawWriter<E, F>,
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    ErrorWrite for _InternalClosureWriter<'a, E, F>
{
    type Error = ClosureWriterError<E>;
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    ChildWrite for _InternalClosureWriter<'a, E, F>
{
    type Parent = Self;
}

macro_rules! _internal_closure_writer_int_impl
{
    (
        impl $type:ty as $write:ident;
    ) =>
    {
        impl<'a, E, F: FnMut(u8) -> Result<(), E>>
            Write<$type> for _InternalClosureWriter<'a, E, F>
        {
            type Next = Self;

            fn write(self, value: $type) -> Result<Self, Self::Error>
            {
                unsafe { self._raw.$write(value)? }
                Ok(self)
            }
        }

        impl<'a, E, F: FnMut(u8) -> Result<(), E>>
            Write<$type> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = Result<Self, Self>;

            fn write(mut self, value: $type) -> Result<Result<Self, Self>, Self::Error>
            {
                if self.0 == self.1
                {
                    Ok(Err(self))
                }
                else
                {
                    self.2 = self.2.write(value)?;
                    self.0 += self.1;
                    Ok(Ok(self))
                }
            }
        }

        impl<'a, E, F: FnMut(u8) -> Result<(), E>>
            WriteAppend<$type, u32> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = _InternalClosureWriter<'a, E, F>;

            fn map(
                mut self,
                mut f: impl FnMut(u32) -> Result<$type, Self::Error>)
                -> Result<<Self as WriteAppend<$type, u32>>::Next, Self::Error>
            {
                for i in self.0..self.1
                {
                    self.2 = self.2.write(f(i as u32)?)?;
                }

                Ok(self.2)
            }
        }
    };

    (
        impl $type:ty as $write:ident : unsigned;
    ) =>
    {
        impl<'a, E, F: FnMut(u8) -> Result<(), E>>
            WriteUnsigned<$type> for _InternalClosureWriter<'a, E, F>
        {
            type Next = Self;

            fn write_unsigned(self, value: $type) -> Result<Self, Self::Error>
            {
                unsafe { self._raw.$write(value)? }
                Ok(self)
            }
        }

        impl<'a, E, F: FnMut(u8) -> Result<(), E>>
            WriteUnsigned<$type> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = Result<Self, Self>;

            fn write_unsigned(mut self, value: $type) -> Result<Result<Self, Self>, Self::Error>
            {
                if self.0 == self.1
                {
                    Ok(Err(self))
                }
                else
                {
                    self.2 = self.2.write_unsigned(value)?;
                    self.0 += self.1;
                    Ok(Ok(self))
                }
            }
        }

        impl<'a, E, F: FnMut(u8) -> Result<(), E>>
            WriteAppendUnsigned<$type, u32> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = _InternalClosureWriter<'a, E, F>;

            fn map_unsigned(
                mut self,
                mut f: impl FnMut(u32) -> Result<$type, Self::Error>)
                -> Result<<Self as WriteAppendUnsigned<$type, u32>>::Next, Self::Error>
            {
                for i in self.0..self.1
                {
                    self.2 = self.2.write_unsigned(f(i as u32)?)?;
                }

                Ok(self.2)
            }
        }
    };

    (
        impl $first_type:ty as $first_write:ident $(: $first_signed:ident)? ;
        $(impl $rest_type:ty as $rest_write:ident $(: $rest_signed:ident)? ;)+
    ) =>
    {
        _internal_closure_writer_int_impl!
        {
            impl $first_type as $first_write $(: $first_signed)? ;
        }
        $(
            _internal_closure_writer_int_impl!
            {
                impl $rest_type as $rest_write $(: $rest_signed)? ;
            }
        )+
    };
}

macro_rules! _internal_closure_writer_int_array_impl
{
    (
        impl $type:ty;
    ) =>
    {
        impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
            Write<&'b [$type]> for _InternalClosureWriter<'a, E, F>
        {
            type Next = Self;

            fn write(self, value: &'b [$type]) -> Result<Self, Self::Error>
            {
                self.each(value.len() as u32, |i, w| w.write(value[i as usize]))
            }
        }

        impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
            Write<&'b [$type]> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = Result<Self, Self>;

            fn write(mut self, value: &'b [$type]) -> Result<Result<Self, Self>, Self::Error>
            {
                if self.0 == self.1
                {
                    Ok(Err(self))
                }
                else
                {
                    self.2 = self.2.write(value)?;
                    self.0 += self.1;
                    Ok(Ok(self))
                }
            }
        }

        impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
            WriteAppend<&'b [$type], u32> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = _InternalClosureWriter<'a, E, F>;

            fn map(
                mut self,
                mut f: impl FnMut(u32) -> Result<&'b [$type], Self::Error>)
                -> Result<<Self as WriteAppend<&'b [$type], u32>>::Next, Self::Error>
            {
                for i in self.0..self.1
                {
                    self.2 = self.2.write(f(i as u32)?)?;
                }

                Ok(self.2)
            }
        }
    };

    (
        impl $type:ty : unsigned;
    ) =>
    {
        impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
            WriteUnsigned<&'b [$type]> for _InternalClosureWriter<'a, E, F>
        {
            type Next = Self;

            fn write_unsigned(self, value: &'b [$type]) -> Result<Self, Self::Error>
            {
                self.each(value.len() as u32, |i, w| w.write_unsigned(value[i as usize]))
            }
        }

        impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
            WriteUnsigned<&'b [$type]> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = Result<Self, Self>;

            fn write_unsigned(mut self, value: &'b [$type]) -> Result<Result<Self, Self>, Self::Error>
            {
                if self.0 == self.1
                {
                    Ok(Err(self))
                }
                else
                {
                    self.2 = self.2.write_unsigned(value)?;
                    self.0 += self.1;
                    Ok(Ok(self))
                }
            }
        }

        impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
            WriteAppendUnsigned<&'b [$type], u32> for (usize, usize, _InternalClosureWriter<'a, E, F>)
        {
            type Next = _InternalClosureWriter<'a, E, F>;

            fn map_unsigned(
                mut self,
                mut f: impl FnMut(u32) -> Result<&'b [$type], Self::Error>)
                -> Result<<Self as WriteAppendUnsigned<&'b [$type], u32>>::Next, Self::Error>
            {
                for i in self.0..self.1
                {
                    self.2 = self.2.write_unsigned(f(i as u32)?)?;
                }

                Ok(self.2)
            }
        }
    };

    (
        impl $first_type:ty $(: $first_signed:ident)? ;
        $(impl $rest_type:ty $(: $rest_signed:ident)? ;)+
    ) =>
    {
        _internal_closure_writer_int_array_impl!
        {
            impl $first_type $(: $first_signed)? ;
        }
        $(
            _internal_closure_writer_int_array_impl!
            {
                impl $rest_type $(: $rest_signed)? ;
            }
        )+
    };
}

_internal_closure_writer_int_impl!
{
    impl i8 as write_byte;
    impl u8 as write_byte_unsigned: unsigned;
    impl i16 as write_short;
    impl u16 as write_short_unsigned: unsigned;
    impl i32 as write_int;
    impl u32 as write_int_unsigned: unsigned;
    impl i64 as write_long;
    impl u64 as write_long_unsigned: unsigned;
}

_internal_closure_writer_int_array_impl!
{
    impl i8;
    impl u8: unsigned;
    impl i16;
    impl u16: unsigned;
    impl i32;
    impl u32: unsigned;
    impl i64;
    impl u64: unsigned;

    impl &'b [i8];
    impl &'b [u8]: unsigned;
    impl &'b [i32];
    impl &'b [u32]: unsigned;
    impl &'b [i64];
    impl &'b [u64]: unsigned;
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    Write<bool> for _InternalClosureWriter<'a, E, F>
{
    type Next = Self;

    fn write(self, value: bool) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_bool(value)? }
        Ok(self)
    }
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    Write<&str> for _InternalClosureWriter<'a, E, F>
{
    type Next = Self;

    fn write(self, value: &str) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_string(value)? }
        Ok(self)
    }
}

impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
    Write<&'b [&'b str]> for _InternalClosureWriter<'a, E, F>
{
    type Next = Self;

    fn write(self, value: &'b [&'b str]) -> Result<Self, Self::Error>
    {
        self.each(value.len() as u32, |i, w| w.write(value[i as usize]))
    }
}

impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
    Write<&'b str> for (usize, usize, _InternalClosureWriter<'a, E, F>)
{
    type Next = Result<Self, Self>;

    fn write(mut self, value: &'b str) -> Result<Result<Self, Self>, Self::Error>
    {
        if self.0 == self.1
        {
            Ok(Err(self))
        }
        else
        {
            self.2 = self.2.write(value)?;
            self.0 += self.1;
            Ok(Ok(self))
        }
    }
}

impl<'a, 'b, E, F: FnMut(u8) -> Result<(), E>>
    WriteAppend<&'b str, u32> for (usize, usize, _InternalClosureWriter<'a, E, F>)
{
    type Next = _InternalClosureWriter<'a, E, F>;

    fn map(
        mut self,
        mut f: impl FnMut(u32) -> Result<&'b str, Self::Error>)
        -> Result<<Self as WriteAppend<&'b str, u32>>::Next, Self::Error>
    {
        for i in self.0..self.1
        {
            self.2 = self.2.write(f(i as u32)?)?;
        }

        Ok(self.2)
    }
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteName for _InternalClosureWriter<'a, E, F>
{
    type Next = Self;

    fn name(self, name: &str) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_string(name)? }
        Ok(self)
    }
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLen<u16> for _InternalClosureWriter<'a, E, F>
{
    type ChildNext = (usize, usize, Self);

    fn len(self, len: u16) -> Result<(usize, usize, Self), Self::Error>
    {
        if len > MAX_LIST_LENGTH as u16
        { return Err(ClosureWriterError::LengthOverflow) }

        self.write_unsigned(len).map(|x| (0, len as usize, x))
    }
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLen<u32> for _InternalClosureWriter<'a, E, F>
{
    type ChildNext = (usize, usize, Self);

    fn len(self, len: u32) -> Result<(usize, usize, Self), Self::Error>
    {
        if len > MAX_LIST_LENGTH as u32
        { return Err(ClosureWriterError::LengthOverflow) }

        self.write_unsigned(len).map(|x| (0, len as usize, x))
    }
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteArray<u32> for _InternalClosureWriter<'a, E, F>
{
    type ChildWrite = _InternalClosureWriter<'a, E, F>;
    type Next = _InternalClosureWriter<'a, E, F>;

    fn each(
        self,
        len: u32,
        mut f: impl FnMut(u32, Self)
            -> Result<Self, Self::Error>)
        -> Result<Self, Self::Error>
    {
        let mut child = self.len(len)?;

        for i in 0..len
        {
            child.2 = f(i, child.2)?;
        }

        Ok(child.2)
    }
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    ErrorWrite for (usize, usize, _InternalClosureWriter<'a, E, F>)
{
    type Error = ClosureWriterError<E>;
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    ChildWrite for (usize, usize, _InternalClosureWriter<'a, E, F>)
{
    type Parent = _InternalClosureWriter<'a, E, F>;
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteExtend<u32> for (usize, usize, _InternalClosureWriter<'a, E, F>)
{
    type ChildWrite = _InternalClosureWriter<'a, E, F>;
    type Next = _InternalClosureWriter<'a, E, F>;

    fn each(
        mut self,
        mut f: impl FnMut(u32, _InternalClosureWriter<'a, E, F>)
            -> Result<_InternalClosureWriter<'a, E, F>, Self::Error>)
        -> Result<Self::Next, Self::Error>
    {
        for i in self.0..self.1
        {
            self.2 = f(i as u32, self.2)?;
        }

        Ok(self.2)
    }

    fn end(self) -> Result<Result<Self::Next, Self>, Self::Error>
    {
        if self.0 == self.1
        {
            unsafe { self.2._raw.write_end()? }
            Ok(Ok(self.2))
        }
        else
        {
            Ok(Err(self))
        }
    }

    fn enter(
        mut self,
        f: impl FnOnce(u32, _InternalClosureWriter<'a, E, F>)
            -> Result<_InternalClosureWriter<'a, E, F>, Self::Error>)
        -> Result<Result<Self, Self>, Self::Error>
    {
        if self.0 == self.1
        {
            Ok(Err(self))
        }
        else
        {
            self.2 = f(self.0 as u32, self.2)?;
            self.0 += self.1;
            Ok(Ok(self))
        }
    }
}

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByte for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteBoolName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteBool for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteShortName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteShort for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteShortListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteShortList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteShortListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteInt for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLong for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteArrayName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteArray for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteArrayContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteArrayListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteArrayList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteByteArrayListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteStringName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteString for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteStringListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteStringList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteStringListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteList for _InternalClosureWriter<'a, E, F>
{
    fn empty(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_end().map(|_| self) }
    }

    type WriteByte = Self;

    fn of_byte(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Byte).map(|_| self) }
    }

    type WriteShort = Self;

    fn of_short(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Short).map(|_| self) }
    }

    type WriteInt = Self;

    fn of_int(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Int).map(|_| self) }
    }

    type WriteLong = Self;

    fn of_long(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Long).map(|_| self) }
    }

    type WriteByteArray = Self;

    fn of_byte_array(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::ByteArray).map(|_| self) }
    }

    type WriteString = Self;

    fn of_string(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::String).map(|_| self) }
    }

    type WriteList = Self;

    fn of_list(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::List).map(|_| self) }
    }

    type WriteCompound = Self;

    fn of_compound(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Compound).map(|_| self) }
    }

    type WriteIntArray = Self;

    fn of_int_array(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::IntArray).map(|_| self) }
    }

    type WriteLongArray = Self;

    fn of_long_array(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::LongArray).map(|_| self) }
    }
}
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteListListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteListList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteListListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteCompoundName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteCompound for _InternalClosureWriter<'a, E, F>
{
    fn end(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_end().map(|_| self) }
    }

    type WriteBool = Self;

    fn bool(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Byte).map(|_| self) }
    }

    type WriteByte = Self;

    fn byte(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Byte).map(|_| self) }
    }

    type WriteShort = Self;

    fn short(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Short).map(|_| self) }
    }

    type WriteInt = Self;

    fn int(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Int).map(|_| self) }
    }

    type WriteLong = Self;

    fn long(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Long).map(|_| self) }
    }

    type WriteByteArray = Self;

    fn byte_array(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::ByteArray).map(|_| self) }
    }

    type WriteString = Self;

    fn string(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::String).map(|_| self) }
    }

    type WriteList = Self;

    fn list(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::List).map(|_| self) }
    }

    type WriteCompound = Self;

    fn compound(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::Compound).map(|_| self) }
    }

    type WriteIntArray = Self;

    fn int_array(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::IntArray).map(|_| self) }
    }

    type WriteLongArray = Self;

    fn long_array(self) -> Result<Self, Self::Error>
    {
        unsafe { self._raw.write_type(Type::LongArray).map(|_| self) }
    }
}
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteCompoundListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteCompoundList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteCompoundListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntArrayName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntArray for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntArrayContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntArrayListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntArrayList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteIntArrayListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }

impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongArrayName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongArray for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongArrayContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongArrayListName for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongArrayList for _InternalClosureWriter<'a, E, F> { }
impl<'a, E, F: FnMut(u8) -> Result<(), E>>
    WriteLongArrayListContents for (usize, usize, _InternalClosureWriter<'a, E, F>) { }