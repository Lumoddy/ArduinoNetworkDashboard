use super::{ArrayVisitor, BooleanVisitor, KeyVisitor, NullVisitor, NumberVisitor, ObjectVisitor, StringVisitor, TypedValueVisitor, ValueVisitor, Visitor};

pub enum ReaderError<E>
{
    Source
    {
        char_index: usize,
        error: E,
    },
    InvalidUTF8
    {
        char_index: usize,
    },
    InvalidSyntax
    {
        char_index: usize,
        found: char,
        expected: &'static str,
    },
    InvalidValue
    {
        char_index: usize,
        found: &'static str,
        expected: &'static str,
    },
    FieldNotFound
    {
        char_index: usize,
        expected: &'static str,
    },
    DuplicateField
    {
        char_index: usize,
        found: &'static str,
    },
    InvalidField
    {
        char_index: usize,
    },
    NumberOverflow
    {
        char_index: usize,
        size: usize,
    },
    StringOverflow
    {
        char_index: usize,
        capacity: usize,
    },
}

pub struct Reader<F: FnMut() -> Result<u8, E>, E>
{
    _f: F,
    _char_counter: usize,
    _buffered_char: Option<char>,
    _is_first: bool,
}

impl<F: FnMut() -> Result<u8, E>, E> Reader<F, E>
{
    pub fn new(f: F) -> Self
    {
        Self
        {
            _f: f,
            _char_counter: 0,
            _buffered_char: None,
            _is_first: true,
        }
    }

    pub fn into_inner(self) -> F { self._f }

    fn _read(&mut self) -> Result<char, ReaderError<E>>
    {
        if let Some(char) = self._buffered_char.take()
        { return Ok(char) }

        let mut bytes =
        [
            match (self._f)()
            {
                Err(error) => return Err(ReaderError::Source
                {
                    char_index: self._char_counter,
                    error,
                }),
                Ok(byte) => byte,
            },
            0,
            0,
            0,
        ];

        for i in 0..4
        {
            if let Ok(char) = _char_from_slice(&bytes[0..i])
            {
                self._char_counter += 1;
                return Ok(char);
            }

            bytes[i] = match (self._f)()
            {
                Err(error) => return Err(ReaderError::Source
                {
                    char_index: self._char_counter,
                    error,
                }),
                Ok(byte) => byte,
            };
        }

        return Err(ReaderError::InvalidUTF8 { char_index: self._char_counter });

        fn _char_from_slice(slice: &[u8]) -> Result<char, ()>
        {
            match str::from_utf8(slice)
            {
                Ok(str) => match str.chars().next()
                {
                    Some(char) => Ok(char),
                    None => return Err(()),
                },
                Err(_) => return Err(()),
            }
        }
    }

    fn _buffer_char(&mut self, char: char)
    {
        if let None = self._buffered_char { self._char_counter -= 1 };
        self._buffered_char = Some(char);
    }
}

macro_rules! _whitespace
{
    () => { '\u{0020}' | '\u{000A}' | '\u{000D}' | '\u{0009}' };
}

impl<F: FnMut() -> Result<u8, E>, E>
    Visitor for Reader<F, E>
{
    type Error = ReaderError<E>;
    type Return = Self;

    fn into_invalid_value_err(self, found: &'static str, expected: &'static str) -> ReaderError<E>
    {
        ReaderError::InvalidValue
        {
            char_index: self._char_counter,
            found,
            expected,
        }
    }

    fn into_field_not_found_err(self, expected: &'static str) -> ReaderError<E>
    {
        ReaderError::FieldNotFound
        {
            char_index: self._char_counter,
            expected,
        }
    }

    fn into_duplicate_field_err(self, found: &'static str) -> ReaderError<E>
    {
        ReaderError::DuplicateField
        {
            char_index: self._char_counter,
            found,
        }
    }

    fn into_invalid_field_err(self) -> ReaderError<E>
    {
        ReaderError::InvalidField
        {
            char_index: self._char_counter,
        }
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    ValueVisitor for Reader<F, E>
{
    type ObjectVisitor = Self;
    type ArrayVisitor = Self;
    type StringVisitor = Self;
    type NumberVisitor = Self;
    type BooleanVisitor = (bool, Self);
    type NullVisitor = Self;

    fn value(mut self) -> Result<
        TypedValueVisitor<Self, Self, Self, Self, (bool, Self), Self>,
        Self::Error>
    {
        self._is_first = false;

        loop
        {
            match self._read()?
            {
                _whitespace!() => continue,
                '{' =>
                {
                    self._is_first = true;
                    return Ok(TypedValueVisitor::Object(self));
                },
                '[' =>
                {
                    self._is_first = true;
                    return Ok(TypedValueVisitor::Array(self));
                },
                '"' =>
                {
                    return Ok(TypedValueVisitor::String(self));
                },
                '0' =>
                {
                    self._buffer_char('0');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '1' =>
                {
                    self._buffer_char('1');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '2' =>
                {
                    self._buffer_char('2');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '3' =>
                {
                    self._buffer_char('3');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '4' =>
                {
                    self._buffer_char('4');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '5' =>
                {
                    self._buffer_char('5');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '6' =>
                {
                    self._buffer_char('6');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '7' =>
                {
                    self._buffer_char('7');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '8' =>
                {
                    self._buffer_char('8');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '9' =>
                {
                    self._buffer_char('9');
                    return Ok(TypedValueVisitor::Number(self));
                },
                '-' =>
                {
                    self._buffer_char('-');
                    return Ok(TypedValueVisitor::Number(self));
                },
                't' =>
                {
                    match self._read()?
                    {
                        'r' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'true'",
                        }),
                    }

                    match self._read()?
                    {
                        'u' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'true'",
                        }),
                    }

                    match self._read()?
                    {
                        'e' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'true'",
                        }),
                    }

                    return Ok(TypedValueVisitor::Boolean((true, self)));
                },
                'f' =>
                {
                    match self._read()?
                    {
                        'a' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'false'",
                        }),
                    }

                    match self._read()?
                    {
                        'l' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'false'",
                        }),
                    }

                    match self._read()?
                    {
                        's' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'false'",
                        }),
                    }

                    match self._read()?
                    {
                        'e' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'false'",
                        }),
                    }

                    return Ok(TypedValueVisitor::Boolean((false, self)));
                },
                'n' =>
                {
                    match self._read()?
                    {
                        'u' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'null'",
                        }),
                    }

                    match self._read()?
                    {
                        'l' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'null'",
                        }),
                    }

                    match self._read()?
                    {
                        'l' => (),
                        found => return Err(ReaderError::InvalidSyntax
                        {
                            char_index: self._char_counter,
                            found,
                            expected: "rest of 'null'",
                        }),
                    }

                    return Ok(TypedValueVisitor::Null(self));
                },
                found => return Err(ReaderError::InvalidSyntax
                {
                    char_index: self._char_counter,
                    found,
                    expected: "start of value",
                })
            }
        }
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    ObjectVisitor for Reader<F, E>
{
    type KeyVisitor = Self;

    fn next(mut self) -> Result<Result<Self, Self>, ReaderError<E>>
    {
        if !self._is_first
        {
            loop
            {
                match self._read()?
                {
                    _whitespace!() => continue,
                    ',' => break,
                    '}' => return Ok(Err(self)),
                    found => return Err(ReaderError::InvalidSyntax
                    {
                        char_index: self._char_counter,
                        found,
                        expected: "','",
                    }),
                }
            }
        }
        else
        {
            self._is_first = false;
        }

        loop
        {
            match self._read()?
            {
                _whitespace!() => continue,
                '"' => return Ok(Ok(self)),
                found => return Err(ReaderError::InvalidSyntax
                {
                    char_index: self._char_counter,
                    found,
                    expected: "'\"'",
                }),
            }
        }
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    ArrayVisitor for Reader<F, E>
{
    type ElementVisitor = Self;

    fn next(mut self) -> Result<Result<Self, Self>, ReaderError<E>>
    {
        if !self._is_first
        {
            loop
            {
                match self._read()?
                {
                    _whitespace!() => continue,
                    ',' => break,
                    ']' => return Ok(Err(self)),
                    found => return Err(ReaderError::InvalidSyntax
                    {
                        char_index: self._char_counter,
                        found,
                        expected: "','",
                    }),
                }
            }
        }
        else
        {
            self._is_first = false;
        }

        loop
        {
            match self._read()?
            {
                _whitespace!() => continue,
                char =>
                {
                    self._buffer_char(char);
                    return Ok(Ok(self));
                },
            }
        }
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    StringVisitor for Reader<F, E>
{
    fn next_char(mut self)
        -> Result<Result<(char, Self), Self>, ReaderError<E>>
    {
        match self._read()?
        {
            '"' => Ok(Err(self)),
            '\\' => match self._read()?
            {
                '"' => Ok(Ok(('\"', self))),
                '\\' => Ok(Ok(('\\', self))),
                '/' => Ok(Ok(('/', self))),
                'b' => Ok(Ok(('\u{0008}', self))),
                'f' => Ok(Ok(('\u{000C}', self))),
                'n' => Ok(Ok(('\u{000A}', self))),
                'r' => Ok(Ok(('\u{000D}', self))),
                't' => Ok(Ok(('\u{0009}', self))),
                'u' =>
                {
                    let mut hex = 0u32;
                    for _ in 0..4
                    {
                        hex <<= 4;
                        hex |= match self._read()?
                        {
                            '0' => 0,
                            '1' => 1,
                            '2' => 2,
                            '3' => 3,
                            '4' => 4,
                            '5' => 5,
                            '6' => 6,
                            '7' => 7,
                            '8' => 8,
                            '9' => 9,
                            'a' | 'A' => 10,
                            'b' | 'B' => 11,
                            'c' | 'C' => 12,
                            'd' | 'D' => 13,
                            'e' | 'E' => 14,
                            'f' | 'F' => 15,
                            found => return Err(ReaderError::InvalidSyntax
                            {
                                char_index: self._char_counter,
                                found,
                                expected: "hex digit",
                            }),
                        };
                    }

                    Ok(Ok((match char::from_u32(hex)
                    {
                        Some(char) => char,
                        None => todo!(),
                    },
                    self)))
                },
                found => return Err(ReaderError::InvalidSyntax
                {
                    char_index: self._char_counter,
                    found,
                    expected: "valid escape code",
                }),
            },
            char => Ok(Ok((char, self))),
        }
    }

    fn collect_string<const N: usize>(mut self)
        -> Result<(heapless::String<N>, Self), ReaderError<E>>
    {
        let mut result = heapless::String::new();

        loop
        {
            match self.next_char()
            {
                Ok(Ok((char, next))) =>
                {
                    self = next;
                    match result.push(char)
                    {
                        Ok(()) => continue,
                        Err(()) => return Err(ReaderError::StringOverflow
                        {
                            char_index: self._char_counter,
                            capacity: N,
                        }),
                    }
                },
                Ok(Err(next)) => return Ok((result, next)),
                Err(error) => return Err(error),
            }
        }
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    Visitor for (bool, Reader<F, E>)
{
    type Error = ReaderError<E>;
    type Return = Reader<F, E>;

    fn into_invalid_value_err(self, found: &'static str, expected: &'static str) -> ReaderError<E>
    {
        self.1.into_invalid_value_err(found, expected)
    }

    fn into_field_not_found_err(self, expected: &'static str) -> ReaderError<E>
    {
        self.1.into_field_not_found_err(expected)
    }

    fn into_invalid_field_err(self) -> ReaderError<E>
    {
        self.1.into_invalid_field_err()
    }

    fn into_duplicate_field_err(self, found: &'static str) -> ReaderError<E>
    {
        self.1.into_duplicate_field_err(found)
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    BooleanVisitor for (bool, Reader<F, E>)
{
    fn collect_bool(self) -> Result<(bool, Reader<F, E>), ReaderError<E>>
    {
        Ok(self)
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    NullVisitor for Reader<F, E>
{
    fn collect_null(self) -> Result<((), Reader<F, E>), ReaderError<E>>
    {
        Ok(((), self))
    }
}

macro_rules! _num_collect_impl
{
    ($visitor:ident as $type:ident) =>
    {
        let mut total = 0;

        let append_digit = match $visitor._read()?
        {
            '-' =>
            {
                |pre: $type, new: $type| pre.overflowing_sub(new)
            }
            char =>
            {
                $visitor._buffer_char(char);
                |pre: $type, new: $type| pre.overflowing_add(new)
            }
        };

        loop
        {
            let new_digit = match $visitor._read()?
            {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                '.' | 'e' | 'E' =>
                {
                    loop
                    {
                        match $visitor._read()?
                        {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                            | '.' | 'e' | 'E' | '+' | '-' => (),
                            char =>
                            {
                                $visitor._buffer_char(char);
                                return Ok((total, $visitor));
                            }
                        }
                    }
                },
                char =>
                {
                    $visitor._buffer_char(char);
                    return Ok((total, $visitor));
                },
            };

            match total.overflowing_mul(10)
            {
                (_, true) => return Err(ReaderError::NumberOverflow
                {
                    char_index: $visitor._char_counter,
                    size: $type::BITS as usize,
                }),
                (pre_add, false) =>
                {
                    match append_digit(pre_add, new_digit)
                    {
                        (_, true) => return Err(ReaderError::NumberOverflow
                        {
                            char_index: $visitor._char_counter,
                            size: $type::BITS as usize,
                        }),
                        (new_total, false) => total = new_total,
                    }
                }
            }
        }
    };
}

impl<F: FnMut() -> Result<u8, E>, E>
    NumberVisitor for Reader<F, E>
{
    fn collect_u8(mut self) -> Result<(u8, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as u8);
    }

    fn collect_i8(mut self) -> Result<(i8, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as i8);
    }

    fn collect_u16(mut self) -> Result<(u16, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as u16);
    }

    fn collect_i16(mut self) -> Result<(i16, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as i16);
    }

    fn collect_u32(mut self) -> Result<(u32, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as u32);
    }

    fn collect_i32(mut self) -> Result<(i32, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as i32);
    }

    fn collect_u64(mut self) -> Result<(u64, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as u64);
    }

    fn collect_i64(mut self) -> Result<(i64, Self), ReaderError<E>>
    {
        _num_collect_impl!(self as i64);
    }

    fn collect_usize(self) -> Result<(usize, Self), ReaderError<E>>
    {
        self.collect_u16().map(|x| (x.0 as usize, x.1))
    }

    fn collect_isize(self) -> Result<(isize, Self), ReaderError<E>>
    {
        self.collect_u16().map(|x| (x.0 as isize, x.1))
    }
}

impl<F: FnMut() -> Result<u8, E>, E>
    KeyVisitor for Reader<F, E>
{
    type ValueVisitor = Self;

    fn collect_key<const N: usize>(self)
        -> Result<(heapless::String<N>, Self), ReaderError<E>>
    {
        let mut result = StringVisitor::collect_string(self)?;

        loop
        {
            match result.1._read()?
            {
                _whitespace!() => continue,
                ':' => break,
                found => return Err(ReaderError::InvalidSyntax
                {
                    char_index: result.1._char_counter,
                    found,
                    expected: "':'",
                }),
            }
        }

        loop
        {
            match result.1._read()?
            {
                _whitespace!() => continue,
                char =>
                {
                    result.1._buffer_char(char);
                    return Ok(result)
                },
            }
        }
    }
}