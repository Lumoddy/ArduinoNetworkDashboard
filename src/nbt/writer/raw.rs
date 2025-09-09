use super::super::Type;

pub trait WriteRaw
{
    type Error;

    unsafe fn write_type(&mut self, value: Type)
        -> Result<(), Self::Error>;

    unsafe fn write_name(&mut self, name: &str)
        -> Result<(), Self::Error>;

    unsafe fn write_end(&mut self)
        -> Result<(), Self::Error>;

    unsafe fn write_bool(&mut self, value: bool)
        -> Result<(), Self::Error>;

    unsafe fn write_byte(&mut self, value: i8)
        -> Result<(), Self::Error>;

    unsafe fn write_byte_unsigned(&mut self, value: u8)
        -> Result<(), Self::Error>;

    unsafe fn write_short(&mut self, value: i16)
        -> Result<(), Self::Error>;

    unsafe fn write_short_unsigned(&mut self, value: u16)
        -> Result<(), Self::Error>;

    unsafe fn write_int(&mut self, value: i32)
        -> Result<(), Self::Error>;

    unsafe fn write_int_unsigned(&mut self, value: u32)
        -> Result<(), Self::Error>;

    unsafe fn write_long(&mut self, value: i64)
        -> Result<(), Self::Error>;

    unsafe fn write_long_unsigned(&mut self, value: u64)
        -> Result<(), Self::Error>;

    unsafe fn write_length(&mut self, value: u32)
        -> Result<(), Self::Error>;

    unsafe fn write_string(&mut self, value: &str)
        -> Result<(), Self::Error>;
}

// MARK: impl

macro_rules! _write_raw_name_impl
{
    (
        $(
            impl $write:path { }
        )+
    ) =>
    {
        $(
            impl<W: super::WriteRaw> $write for W
            {
                type Parent = W;
                type Error = W::Error;
                type ValueWriter = W;

                fn name(mut self, name: &str)
                    -> Result<W, W::Error>
                { unsafe { self.write_name(name)?; Ok(self) } }
            }
        )+
    };
}

macro_rules! _write_raw_impl
{
    (
        $(
            impl $write:path
            {
                fn write($write_base_type:ty) as $write_base:ident;
                $(fn write_unsigned($write_unsigned_base_type:ty) as $write_unsigned_base:ident;)?
            }
        )+
    ) =>
    {
        $(
            impl<W: super::WriteRaw> $write for W
            {
                type Parent = W;
                type Error = W::Error;

                fn write(mut self, byte: $write_base_type)
                    -> Result<W, W::Error>
                { unsafe { self.$write_base(byte)?; Ok(self) } }

                $(
                    fn write_unsigned(mut self, byte: $write_unsigned_base_type)
                        -> Result<W, W::Error>
                    { unsafe { self.$write_unsigned_base(byte)?; Ok(self) } }
                )?
            }
        )+
    };
}

macro_rules! _write_raw_list_impl
{
    (
        $(
            impl $write:path
            {
                fn write($write_base_type:ty) as $write_base:ident;
                $(fn write_unsigned($write_unsigned_base_type:ty) as $write_unsigned_base:ident;)?
            }
        )+
    ) =>
    {
        $(
            impl<W: super::WriteRaw> $write for W
            {
                type Parent = W;
                type Error = W::Error;

                fn write(mut self, array: $write_base_type) -> Result<W, W::Error>
                {
                    unsafe
                    {
                        self.write_length(array.len() as u32)?;
                        for element in array { self.$write_base(*element)? }
                        Ok(self)
                    }
                }

                $(
                    fn write_unsigned(mut self, array: $write_unsigned_base_type) -> Result<W, W::Error>
                    {
                        unsafe
                        {
                            self.write_length(array.len() as u32)?;
                            for element in array { self.$write_unsigned_base(*element)? }
                            Ok(self)
                        }
                    }
                )?

                type WriteContents = (usize, W);

                fn length(mut self, length: u32) -> Result<Self::WriteContents, Self::Error>
                { unsafe { self.write_length(length).map(|_| (length as usize, self)) } }
            }
        )+
    };
}

macro_rules! _write_raw_array_impl
{
    (
        $(
            impl $write:path
            {
                fn write($write_base_type:ty) as $write_base:ident;
                $(fn write_unsigned($write_unsigned_base_type:ty) as $write_unsigned_base:ident;)?
            }
        )+
    ) =>
    {
        $(
            impl<W: WriteRaw> $write for W
            {
                type Parent = W;
                type Error = W::Error;

                fn write(mut self, array: $write_base_type) -> Result<W, W::Error>
                {
                    unsafe
                    {
                        self.write_length(array.len() as u32)?;

                        for element in array
                        { self.$write_base(*element)? }

                        Ok(self)
                    }
                }

                $(
                    fn write_unsigned(mut self, array: $write_unsigned_base_type) -> Result<W, W::Error>
                    {
                        unsafe
                        {
                            self.write_length(array.len() as u32)?;

                            for element in array
                            { self.$write_unsigned_base(*element)? }

                            Ok(self)
                        }
                    }
                )?

                type WriteContents = (usize, W);

                fn length(mut self, length: u32) -> Result<(usize, W), W::Error>
                { unsafe { self.write_length(length).map(|_| (length as usize, self)) } }
            }
        )+
    };
}

macro_rules! _write_raw_array_list_impl
{
    (
        $(
            impl $write:path
            {
                fn write($write_base_type:ty) as $write_base:ident;
                $(fn write_unsigned($write_unsigned_base_type:ty) as $write_unsigned_base:ident;)?
            }
        )+
    ) =>
    {
        $(
            impl<W: WriteRaw> $write for W
            {
                type Parent = W;
                type Error = W::Error;

                fn write(mut self, array: $write_base_type) -> Result<Self::Parent, Self::Error>
                {
                    unsafe
                    {
                        self.write_length(array.len() as u32)?;

                        for array in array
                        {
                            self.write_length(array.len() as u32)?;

                            for element in *array
                            { self.$write_base(*element)? }
                        }

                        Ok(self)
                    }
                }

                $(
                    fn write_unsigned(mut self, array: $write_unsigned_base_type) -> Result<Self::Parent, Self::Error>
                    {
                        unsafe
                        {
                            self.write_length(array.len() as u32)?;

                            for array in array
                            {
                                self.write_length(array.len() as u32)?;

                                for element in *array
                                { self.$write_unsigned_base(*element)? }
                            }

                            Ok(self)
                        }
                    }
                )?

                type WriteContents = (usize, W);

                fn length(mut self, length: u32) -> Result<Self::WriteContents, Self::Error>
                { unsafe { self.write_length(length).map(|_| (length as usize, self)) } }
            }
        )+
    };
}

macro_rules! _write_raw_list_contents_impl
{
    (
        $(
            impl $write_contents:path { }
        )+
    ) =>
    {
        $(
            impl<W: WriteRaw> $write_contents for (usize, W)
            {
                type Parent = W;
                type Error = W::Error;

                type WriteElement = W;

                fn map(
                    mut self,
                    mut f: impl FnMut(W) -> Result<W, W::Error>)
                    -> Result<Self::Parent, Self::Error>
                {
                    while self.0 > 0
                    {
                        self.1 = f(self.1)?;
                        self.0 -= 1;
                    }

                    Ok(self.1)
                }

                fn end(mut self) -> Result<Result<W, Self>, W::Error>
                {
                    if self.0 == 0
                    { unsafe { self.1.write_end().map(|_| Ok(self.1)) } }
                    else
                    { Ok(Err(self)) }
                }

                fn enter(
                    mut self,
                    f: impl FnOnce(W) -> Result<W, W::Error>)
                    -> Result<Result<Self, Self>, W::Error>
                {
                    if self.0 > 0
                    {
                        self.1 = f(self.1)?;
                        self.0 -= 1;
                        Ok(Ok(self))
                    }
                    else
                    {
                        Ok(Err(self))
                    }
                }
            }
        )+
    };
}

_write_raw_name_impl!
{
    impl super::WriteBoolName { }
    impl super::WriteByteName { }
    impl super::WriteShortName { }
    impl super::WriteIntName { }
    impl super::WriteLongName { }
    impl super::WriteByteArrayName { }
    impl super::WriteStringName { }
    impl super::WriteListName { }
    impl super::WriteCompoundName { }
    impl super::WriteIntArrayName { }
    impl super::WriteLongArrayName { }

    impl super::WriteByteListName { }
    impl super::WriteShortListName { }
    impl super::WriteIntListName { }
    impl super::WriteLongListName { }
    impl super::WriteByteArrayListName { }
    impl super::WriteStringListName { }
    impl super::WriteListListName { }
    impl super::WriteCompoundListName { }
    impl super::WriteIntArrayListName { }
    impl super::WriteLongArrayListName { }
}

_write_raw_impl!
{
    impl super::WriteBool
    {
        fn write(bool) as write_bool;
    }

    impl super::WriteByte
    {
        fn write(i8) as write_byte;
        fn write_unsigned(u8) as write_byte_unsigned;
    }

    impl super::WriteShort
    {
        fn write(i16) as write_short;
        fn write_unsigned(u16) as write_short_unsigned;
    }

    impl super::WriteInt
    {
        fn write(i32) as write_int;
        fn write_unsigned(u32) as write_int_unsigned;
    }

    impl super::WriteLong
    {
        fn write(i64) as write_long;
        fn write_unsigned(u64) as write_long_unsigned;
    }

    impl super::WriteString
    {
        fn write(&str) as write_string;
    }
}

_write_raw_list_impl!
{
    impl super::WriteByteList
    {
        fn write(&[i8]) as write_byte;
        fn write_unsigned(&[u8]) as write_byte_unsigned;
    }

    impl super::WriteShortList
    {
        fn write(&[i16]) as write_short;
        fn write_unsigned(&[u16]) as write_short_unsigned;
    }

    impl super::WriteIntList
    {
        fn write(&[i32]) as write_int;
        fn write_unsigned(&[u32]) as write_int_unsigned;
    }

    impl super::WriteLongList
    {
        fn write(&[i64]) as write_long;
        fn write_unsigned(&[u64]) as write_long_unsigned;
    }

    impl super::WriteStringList
    {
        fn write(&[&str]) as write_string;
    }
}

_write_raw_array_impl!
{
    impl super::WriteByteArray
    {
        fn write(&[i8]) as write_byte;
        fn write_unsigned(&[u8]) as write_byte_unsigned;
    }

    impl super::WriteIntArray
    {
        fn write(&[i32]) as write_int;
        fn write_unsigned(&[u32]) as write_int_unsigned;
    }

    impl super::WriteLongArray
    {
        fn write(&[i64]) as write_long;
        fn write_unsigned(&[u64]) as write_long_unsigned;
    }
}

_write_raw_array_list_impl!
{
    impl super::WriteByteArrayList
    {
        fn write(&[&[i8]]) as write_byte;
        fn write_unsigned(&[&[u8]]) as write_byte_unsigned;
    }

    impl super::WriteIntArrayList
    {
        fn write(&[&[i32]]) as write_int;
        fn write_unsigned(&[&[u32]]) as write_int_unsigned;
    }

    impl super::WriteLongArrayList
    {
        fn write(&[&[i64]]) as write_long;
        fn write_unsigned(&[&[u64]]) as write_long_unsigned;
    }
}

_write_raw_list_contents_impl!
{
    impl super::WriteByteArrayContents { }
    impl super::WriteByteListContents { }
    impl super::WriteShortListContents { }
    impl super::WriteIntListContents { }
    impl super::WriteLongListContents { }
    impl super::WriteStringListContents { }
    impl super::WriteByteArrayListContents { }
    impl super::WriteListListContents { }
    impl super::WriteCompoundListContents { }
    impl super::WriteIntArrayListContents { }
    impl super::WriteLongArrayListContents { }
    impl super::WriteIntArrayContents { }
    impl super::WriteLongArrayContents { }
}