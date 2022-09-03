use crate::{Error, Result};

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    NoOp,
    Bool(bool),
    I8(i8),
    U8(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Number(String),
    Char(char),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Marker {
    Null = b'Z',
    NoOp = b'N',
    True = b'T',
    False = b'F',
    I8 = b'i',
    U8 = b'U',
    I16 = b'I',
    I32 = b'l',
    I64 = b'L',
    F32 = b'd',
    F64 = b'D',
    Number = b'H',
    Char = b'C',
    String = b'S',
    ArrayStart = b'[',
    ArrayEnd = b']',
    ObjectStart = b'{',
    ObjectEnd = b'}',
    Length = b'#',
    OfType = b'$',
}

impl Into<char> for Marker {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl Into<&[u8]> for Marker {
    fn into(self) -> &'static [u8] {
        match self {
            Marker::Null => b"Z",
            Marker::NoOp => b"N",
            Marker::True => b"T",
            Marker::False => b"F",
            Marker::I8 => b"i",
            Marker::U8 => b"U",
            Marker::I16 => b"I",
            Marker::I32 => b"l",
            Marker::I64 => b"L",
            Marker::F32 => b"d",
            Marker::F64 => b"D",
            Marker::Number => b"H",
            Marker::Char => b"C",
            Marker::String => b"S",
            Marker::ArrayStart => b"[",
            Marker::ArrayEnd => b"]",
            Marker::ObjectStart => b"{",
            Marker::ObjectEnd => b"}",
            Marker::Length => b"#",
            Marker::OfType => b"$",
        }
    }
}

impl TryFrom<u8> for Marker {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let marker = match value {
            b'Z' => Marker::Null,
            b'N' => Marker::NoOp,
            b'T' => Marker::True,
            b'F' => Marker::False,
            b'i' => Marker::I8,
            b'U' => Marker::U8,
            b'I' => Marker::I16,
            b'l' => Marker::I32,
            b'L' => Marker::I64,
            b'd' => Marker::F32,
            b'D' => Marker::F64,
            b'H' => Marker::Number,
            b'C' => Marker::Char,
            b'S' => Marker::String,
            b'[' => Marker::ArrayStart,
            b']' => Marker::ArrayEnd,
            b'{' => Marker::ObjectStart,
            b'}' => Marker::ObjectEnd,
            b'#' => Marker::Length,
            b'$' => Marker::OfType,
            _ => return Err(Error::InvalidMarker),
        };
        Ok(marker)
    }
}

impl TryFrom<char> for Marker {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Marker::try_from(value as u8)
    }
}
