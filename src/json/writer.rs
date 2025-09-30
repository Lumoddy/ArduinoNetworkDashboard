use core::cmp;
use core::hint::unreachable_unchecked;
use core::ptr::addr_eq;

use crate::unreachable_payload;

use super::{ArrayTracer, BooleanTracer, KeyTracer, NumberTracer, ObjectTracer, StringTracer, Tracer, ValueTracer};

pub enum WriterError<E>
{
    Source
    {
        char_index: usize,
        error: E,
    },
}

pub struct Writer<F: FnMut(u8) -> Result<(), E>, E>
{
    _f: F,
    _char_counter: usize,
    _is_first: bool,
}

impl<F: FnMut(u8) -> Result<(), E>, E> Writer<F, E>
{
    pub fn new(f: F) -> Self
    {
        Self
        {
            _f: f,
            _char_counter: 0,
            _is_first: true,
        }
    }

    pub fn into_inner(self) -> F { self._f }

    fn _write(&mut self, char: char) -> Result<(), WriterError<E>>
    {
        for &byte in char.encode_utf8(&mut [0; 4]).as_bytes()
        {
            match (self._f)(byte)
            {
                Ok(()) => continue,
                Err(error) => return Err(WriterError::Source
                {
                    char_index: self._char_counter,
                    error,
                }),
            }
        }

        self._char_counter += 1;

        Ok(())
    }
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    Tracer for Writer<F, E>
{
    type Error = WriterError<E>;
    type Return = Self;
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    ValueTracer for Writer<F, E>
{
    type ObjectTracer = Self;
    type ArrayTracer = Self;
    type StringTracer = Self;
    type NumberTracer = Self;
    type BooleanTracer = Self;

    fn object(mut self) -> Result<Self, WriterError<E>>
    {
        self._write('{')?;
        self._is_first = true;
        Ok(self)
    }

    fn array(mut self) -> Result<Self, WriterError<E>>
    {
        self._write('[')?;
        self._is_first = true;
        Ok(self)
    }

    fn str(self, value: &str) -> Result<Self, WriterError<E>>
    {
        StringTracer::end(self.string()?.over(value.chars())?)
    }

    fn string(mut self) -> Result<Self, WriterError<E>>
    {
        self._write('"')?;
        self._is_first = true;
        Ok(self)
    }

    fn number(self) -> Result<Self, WriterError<E>>
    {
        Ok(self)
    }

    fn number_u8(self, value: u8) -> Result<Self, WriterError<E>>
    {
        self.number()?.u8(value)
    }

    fn number_i8(self, value: i8) -> Result<Self, WriterError<E>>
    {
        self.number()?.i8(value)
    }

    fn number_u16(self, value: u16) -> Result<Self, WriterError<E>>
    {
        self.number()?.u16(value)
    }

    fn number_i16(self, value: i16) -> Result<Self, WriterError<E>>
    {
        self.number()?.i16(value)
    }

    fn number_u32(self, value: u32) -> Result<Self, WriterError<E>>
    {
        self.number()?.u32(value)
    }

    fn number_i32(self, value: i32) -> Result<Self, WriterError<E>>
    {
        self.number()?.i32(value)
    }

    fn number_u64(self, value: u64) -> Result<Self, WriterError<E>>
    {
        self.number()?.u64(value)
    }

    fn number_i64(self, value: i64) -> Result<Self, WriterError<E>>
    {
        self.number()?.i64(value)
    }

    fn number_usize(self, value: usize) -> Result<Self, WriterError<E>>
    {
        self.number()?.usize(value)
    }

    fn number_isize(self, value: isize) -> Result<Self, WriterError<E>>
    {
        self.number()?.isize(value)
    }

    fn boolean(self) -> Result<Self, WriterError<E>>
    {
        Ok(self)
    }

    fn bool(self, value: bool) -> Result<Self, WriterError<E>>
    {
        BooleanTracer::bool(self.boolean()?, value)
    }

    fn null(mut self) -> Result<Self, WriterError<E>>
    {
        self._write('n')?;
        self._write('u')?;
        self._write('l')?;
        self._write('l')?;
        Ok(self)
    }
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    ObjectTracer for Writer<F, E>
{
    type KeyTracer = Self;

    fn entry_key(self, value: &str) -> Result<Self, WriterError<E>>
    {
        self.entry()?.key_over(value.chars())
    }

    fn entry(mut self) -> Result<Self, WriterError<E>>
    {
        self._write('"')?;
        Ok(self)
    }

    fn end(mut self) -> Result<Self, WriterError<E>>
    {
        self._write('}')?;
        Ok(self)
    }
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    ArrayTracer for Writer<F, E>
{
    type ElementTracer = Self;

    fn element(mut self) -> Result<Self, WriterError<E>>
    {
        if self._is_first { self._is_first = false }
        else { self._write(',')? }

        Ok(self)
    }

    fn end(mut self) -> Result<Self, WriterError<E>>
    {
        self._write(']')?;
        Ok(self)
    }
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    StringTracer for Writer<F, E>
{
    fn append(mut self, char: char)
        -> Result<Self, WriterError<E>>
    {
        match char
        {
            '\"' =>
            {
                self._write('/')?;
                self._write('"')?;
            },
            '\\' =>
            {
                self._write('/')?;
                self._write('\\')?;
            },
            '/' =>
            {
                self._write('/')?;
                self._write('/')?;
            },
            '\u{0008}' =>
            {
                self._write('/')?;
                self._write('b')?;
            },
            '\u{000C}' =>
            {
                self._write('/')?;
                self._write('f')?;
            },
            '\u{000A}' =>
            {
                self._write('/')?;
                self._write('n')?;
            },
            '\u{000D}' =>
            {
                self._write('/')?;
                self._write('r')?;
            },
            '\u{0009}' =>
            {
                self._write('/')?;
                self._write('t')?;
            },
            char => self._write(char)?,
        }
        Ok(self)
    }

    fn over(mut self, chars: impl Iterator<Item = char>)
        -> Result<Self, WriterError<E>>
    {
        for char in chars
        {
            self._write(char)?;
        }

        Ok(self)
    }

    fn end(mut self) -> Result<Self, WriterError<E>>
    {
        self._write('"')?;
        Ok(self)
    }
}

macro_rules! _unsigned_tracer_impl
{
    (<$digits:literal> as $value:ident in $writer:ident) =>
    {{
        let mut chars = heapless::String::<$digits>::new();

        loop
        {
            let Ok(()) = chars.push(match $value % 10
            {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => unreachable_payload!(),
            })
            else { unreachable_payload!() };

            $value /= 10;

            if $value == 0 { break };
        }

        for char in chars.chars().rev()
        {
            $writer._write(char)?
        }

        Ok($writer)
    }};
}

macro_rules! _signed_tracer_impl
{
    (<$digits:literal> as $value:ident in $writer:ident) =>
    {{
        if $value < 0 { $writer._write('-')? }

        let mut $value = $value.unsigned_abs();
        let mut chars = heapless::String::<$digits>::new();

        loop
        {
            let Ok(()) = chars.push(match $value % 10
            {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => unreachable_payload!(),
            })
            else { unreachable_payload!() };

            $value /= 10;

            if $value == 0 { break };
        }

        for char in chars.chars().rev()
        {
            $writer._write(char)?
        }

        Ok($writer)
    }};
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    NumberTracer for Writer<F, E>
{
    fn u8(mut self, mut value: u8) -> Result<Self, WriterError<E>>
    {
        _unsigned_tracer_impl!(<3> as value in self)
    }

    fn i8(mut self, value: i8) -> Result<Self, WriterError<E>>
    {
        _signed_tracer_impl!(<3> as value in self)
    }

    fn u16(mut self, mut value: u16) -> Result<Self, WriterError<E>>
    {
        _unsigned_tracer_impl!(<5> as value in self)
    }

    fn i16(mut self, value: i16) -> Result<Self, WriterError<E>>
    {
        _signed_tracer_impl!(<5> as value in self)
    }

    fn u32(mut self, mut value: u32) -> Result<Self, WriterError<E>>
    {
        _unsigned_tracer_impl!(<10> as value in self)
    }

    fn i32(mut self, value: i32) -> Result<Self, WriterError<E>>
    {
        _signed_tracer_impl!(<10> as value in self)
    }

    fn u64(mut self, mut value: u64) -> Result<Self, WriterError<E>>
    {
        _unsigned_tracer_impl!(<20> as value in self)
    }

    fn i64(mut self, value: i64) -> Result<Self, WriterError<E>>
    {
        _signed_tracer_impl!(<20> as value in self)
    }

    fn usize(self, value: usize) -> Result<Self, WriterError<E>>
    {
        self.u16(value as u16)
    }

    fn isize(self, value: isize) -> Result<Self, WriterError<E>>
    {
        self.i16(value as i16)
    }
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    BooleanTracer for Writer<F, E>
{
    fn bool(mut self, value: bool) -> Result<Self, WriterError<E>>
    {
        if value
        {
            self._write('t')?;
            self._write('r')?;
            self._write('u')?;
            self._write('e')?;
        }
        else
        {
            self._write('f')?;
            self._write('a')?;
            self._write('l')?;
            self._write('s')?;
            self._write('e')?;
        }

        Ok(self)
    }
}

impl<F: FnMut(u8) -> Result<(), E>, E>
    KeyTracer for Writer<F, E>
{
    type ValueTracer = Self;

    fn key_append(self, char: char)
        -> Result<Self, WriterError<E>>
    {
        self.append(char)
    }

    fn key_over(self, chars: impl Iterator<Item = char>)
        -> Result<Self, WriterError<E>>
    {
        self.over(chars)
    }
}