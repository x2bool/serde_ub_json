use std::mem::size_of;
use std::str;

use serde::de::{DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;

use crate::{Error, Result};
use crate::value::Marker;

pub fn from_bytes<'de, T>(bytes: &'de [u8]) -> Result<T>
    where
        T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(bytes);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

pub struct Deserializer<'de> {
    bytes: &'de [u8],
    of_type: Option<Marker>,
}

impl<'de> Deserializer<'de> {
    pub fn new(bytes: &'de [u8]) -> Deserializer<'de> {
        Deserializer {
            bytes,
            of_type: None,
        }
    }

    fn peek_byte(&self) -> Result<u8> {
        if self.bytes.len() > 0 {
            let byte = self.bytes[0];
            Ok(byte)
        } else {
            Err(Error::Eof)
        }
    }

    fn read_byte(&mut self) -> Result<u8> {
        if self.bytes.len() > 0 {
            let byte = self.bytes[0];
            self.bytes = &self.bytes[1..];
            Ok(byte)
        } else {
            Err(Error::Eof)
        }
    }

    fn read_bytes_mut(&mut self, data: &mut [u8]) -> Result<()> {
        let len = data.len();
        if self.bytes.len() < len {
            return Err(Error::Eof);
        }
        data.copy_from_slice(&self.bytes[..len]);
        self.bytes = &self.bytes[len..];
        Ok(())
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'de [u8]> {
        if self.bytes.len() < len {
            return Err(Error::Eof);
        }
        let data = &self.bytes[..len];
        self.bytes = &self.bytes[len..];
        Ok(data)
    }

    fn peek_marker(&self) -> Result<Marker> {
        let byte = self.peek_byte()?;
        let marker = Marker::try_from(byte)?;
        Ok(marker)
    }

    fn read_marker(&mut self) -> Result<Marker> {
        let byte = self.read_byte()?;
        let marker = Marker::try_from(byte)?;
        Ok(marker)
    }

    fn take_or_read_marker(&mut self) -> Result<Marker> {
        if let Some(marker) = self.of_type.take() {
            return Ok(marker);
        }
        self.read_marker()
    }

    fn read_len(&mut self) -> Result<usize> {
        let size = match self.read_marker()? {
            Marker::I8 => self.read_i8()? as usize,
            Marker::I16 => self.read_i16()? as usize,
            Marker::I32 => self.read_i32()? as usize,
            Marker::I64 => self.read_i64()? as usize,
            _ => return Err(Error::ExpectedLength),
        };
        Ok(size)
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut data = [0u8; size_of::<u8>()];
        self.read_bytes_mut(&mut data)?;
        Ok(u8::from_be_bytes(data))
    }

    fn read_i8(&mut self) -> Result<i8> {
        let mut data = [0u8; size_of::<i8>()];
        self.read_bytes_mut(&mut data)?;
        Ok(i8::from_be_bytes(data))
    }

    fn read_i16(&mut self) -> Result<i16> {
        let mut data = [0u8; size_of::<i16>()];
        self.read_bytes_mut(&mut data)?;
        Ok(i16::from_be_bytes(data))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let mut data = [0u8; size_of::<i32>()];
        self.read_bytes_mut(&mut data)?;
        Ok(i32::from_be_bytes(data))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut data = [0u8; size_of::<i64>()];
        self.read_bytes_mut(&mut data)?;
        Ok(i64::from_be_bytes(data))
    }

    fn read_f32(&mut self) -> Result<f32> {
        let mut data = [0u8; size_of::<f32>()];
        self.read_bytes_mut(&mut data)?;
        Ok(f32::from_be_bytes(data))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let mut data = [0u8; size_of::<f64>()];
        self.read_bytes_mut(&mut data)?;
        Ok(f64::from_be_bytes(data))
    }

    fn read_string(&mut self) -> Result<String> {
        let size = self.read_len()?;
        let mut data = vec![0; size];
        self.read_bytes_mut(&mut data)?;

        match String::from_utf8(data) {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::InvalidString),
        }
    }

    fn read_str(&mut self) -> Result<&'de str> {
        let size = self.read_len()?;
        let data = self.read_bytes(size)?;

        match str::from_utf8(data) {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::InvalidString),
        }
    }
}

impl<'a, 'de: 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.peek_marker()? {
            Marker::Null => self.deserialize_option(visitor),
            Marker::NoOp => self.deserialize_unit(visitor),
            Marker::True => self.deserialize_bool(visitor),
            Marker::False => self.deserialize_bool(visitor),
            _ => Err(Error::InvalidMarker),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::True => visitor.visit_bool(true),
            Marker::False => visitor.visit_bool(false),
            _ => Err(Error::Expected(vec![Marker::True, Marker::False])),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::I8 => visitor.visit_i8(self.read_i8()?),
            _ => Err(Error::Expected(vec![Marker::I8])),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::I16 => visitor.visit_i16(self.read_i16()?),
            Marker::I8 => visitor.visit_i16((self.read_i8()?) as i16),
            _ => Err(Error::Expected(vec![Marker::I16, Marker::I8])),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::I32 => visitor.visit_i32(self.read_i32()?),
            Marker::I16 => visitor.visit_i32((self.read_i16()?) as i32),
            Marker::I8 => visitor.visit_i32((self.read_i8()?) as i32),
            _ => Err(Error::Expected(vec![Marker::I32, Marker::I16, Marker::I8])),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::I64 => visitor.visit_i64(self.read_i64()?),
            Marker::I32 => visitor.visit_i64((self.read_i32()?) as i64),
            Marker::I16 => visitor.visit_i64((self.read_i16()?) as i64),
            Marker::I8 => visitor.visit_i64((self.read_i8()?) as i64),
            _ => Err(Error::Expected(vec![Marker::I64, Marker::I32, Marker::I16, Marker::I8])),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::U8 => visitor.visit_u8(self.read_u8()?),
            _ => Err(Error::Expected(vec![Marker::U8])),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::U8 => visitor.visit_u16((self.read_u8()?) as u16),
            _ => Err(Error::Expected(vec![Marker::U8])),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::U8 => visitor.visit_u32((self.read_u8()?) as u32),
            _ => Err(Error::Expected(vec![Marker::U8])),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::U8 => visitor.visit_u64((self.read_u8()?) as u64),
            _ => Err(Error::Expected(vec![Marker::U8])),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::F32 => visitor.visit_f32(self.read_f32()?),
            _ => Err(Error::Expected(vec![Marker::F32])),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::F64 => visitor.visit_f64(self.read_f64()?),
            Marker::F32 => visitor.visit_f64((self.read_f32()?) as f64),
            _ => Err(Error::Expected(vec![Marker::F64, Marker::F32])),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::Char => {
                let c = self.read_byte()?;
                visitor.visit_char(c as char)
            }
            Marker::String => {
                let s = self.read_str()?;
                if s.len() == 1 && s.is_ascii() {
                    let c = s.as_bytes()[0];
                    visitor.visit_char(c as char)
                } else {
                    Err(Error::InvalidString)
                }
            }
            _ => Err(Error::Expected(vec![Marker::Char, Marker::String])),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::String => visitor.visit_borrowed_str(self.read_str()?),
            Marker::Char => {
                let bytes = self.read_bytes(1)?;
                match str::from_utf8(bytes) {
                    Ok(s) => visitor.visit_borrowed_str(s),
                    Err(_) => Err(Error::InvalidString),
                }
            }
            _ => Err(Error::Expected(vec![Marker::String, Marker::Char])),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.take_or_read_marker()? {
            Marker::String => visitor.visit_string(self.read_string()?),
            Marker::Char => {
                let c = self.read_byte()?;
                visitor.visit_string((c as char).to_string())
            }
            _ => Err(Error::Expected(vec![Marker::String, Marker::Char])),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.read_marker()? {
            Marker::ArrayStart => {
                let (len, _of_type) = match self.peek_marker()? {
                    Marker::OfType => {
                        // both type and length are specified
                        self.read_marker()?;
                        let marker = self.read_marker()?;
                        match self.read_marker()? {
                            Marker::Length => {
                                let len = self.read_len()?;
                                (Some(len), Some(marker))
                            }
                            _ => return Err(Error::Expected(vec![Marker::Length])),
                        }
                    }
                    Marker::Length => {
                        // only length is specified
                        self.read_marker()?;
                        let len = self.read_len()?;
                        (Some(len), None)
                    }
                    _ => (None, None), // neither type nor length are specified
                };

                let value = match len {
                    Some(len) => { // read borrowed bytes
                        let bytes = self.read_bytes(len)?;
                        visitor.visit_borrowed_bytes::<Error>(bytes)?
                    }
                    None => { // this will fail because it is impossible to read as borrowed bytes
                        let bytes = vec![0u8];
                        visitor.visit_bytes::<Error>(&bytes)?
                    }
                };

                Ok(value)
            }
            _ => Err(Error::Expected(vec![Marker::ArrayStart])),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.peek_marker()? {
            Marker::Null => {
                self.read_marker()?;
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.read_marker()? {
            Marker::Null => visitor.visit_unit(),
            _ => Err(Error::Expected(vec![Marker::Null])),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.read_marker()? {
            Marker::ArrayStart => {
                let (len, of_type) = match self.peek_marker()? {
                    Marker::OfType => {
                        // both type and length are specified
                        self.read_marker()?;
                        let marker = self.read_marker()?;
                        match self.read_marker()? {
                            Marker::Length => {
                                let len = self.read_len()?;
                                (Some(len), Some(marker))
                            }
                            _ => return Err(Error::Expected(vec![Marker::Length])),
                        }
                    }
                    Marker::Length => {
                        // only length is specified
                        self.read_marker()?;
                        let len = self.read_len()?;
                        (Some(len), None)
                    }
                    _ => (None, None), // neither type nor length are specified
                };

                let value = visitor.visit_seq(ArrayAccess {
                    de: &mut self,
                    len,
                    of_type,
                    trailer: if len.is_some() { None } else { Some(Marker::ArrayEnd) },
                })?;
                Ok(value)
            }
            _ => Err(Error::Expected(vec![Marker::ArrayStart])),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.read_marker()? {
            Marker::ObjectStart => {
                let (len, of_type) = match self.peek_marker()? {
                    Marker::OfType => {
                        // both type and length are specified
                        self.read_marker()?;
                        let marker = self.read_marker()?;
                        match self.read_marker()? {
                            Marker::Length => {
                                let len = self.read_len()?;
                                (Some(len), Some(marker))
                            }
                            _ => return Err(Error::Expected(vec![Marker::Length])),
                        }
                    }
                    Marker::Length => {
                        // only length is specified
                        self.read_marker()?;
                        let len = self.read_len()?;
                        (Some(len), None)
                    }
                    _ => (None, None), // neither type nor length are specified
                };

                let value = visitor.visit_map(ObjectAccess {
                    de: &mut self,
                    len,
                    of_type,
                    trailer: if len.is_some() { None } else { Some(Marker::ObjectEnd) }
                })?;
                Ok(value)
            }
            _ => Err(Error::Expected(vec![Marker::ObjectStart])),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.read_marker()? {
            Marker::String => {
                let s = self.read_str()?;
                visitor.visit_enum(s.into_deserializer())
            }
            Marker::ObjectStart => {
                let len = match self.peek_marker()? {
                    Marker::OfType => {
                        // both type and length are specified
                        self.read_marker()?; // of type
                        self.read_marker()?; // type
                        match self.read_marker()? { // length
                            Marker::Length => {
                                let len = self.read_len()?;
                                Some(len)
                            }
                            _ => return Err(Error::Expected(vec![Marker::Length])),
                        }
                    }
                    Marker::Length => {
                        // only length is specified
                        self.read_marker()?;
                        let len = self.read_len()?;
                        Some(len)
                    }
                    _ => None
                };

                let value = visitor.visit_enum(ItemAccess {
                    de: self
                })?;

                match len {
                    Some(_) => Ok(value),
                    None => {
                        match self.read_marker()? {
                            Marker::ObjectEnd => Ok(value),
                            _ => Err(Error::Expected(vec![Marker::ObjectEnd])),
                        }
                    },
                }
            }
            _ => Err(Error::Expected(vec![Marker::String, Marker::ObjectStart]))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct ArrayAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: Option<usize>,
    of_type: Option<Marker>,
    trailer: Option<Marker>,
}

impl<'de, 'a> SeqAccess<'de> for ArrayAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where
            T: DeserializeSeed<'de>,
    {
        match self.len {
            Some(len) => {
                if len == 0 {
                    Ok(None)
                } else {
                    // hint type to the deserializer if set
                    self.de.of_type = self.of_type;
                    let value = seed.deserialize(&mut *self.de)?;
                    self.len = Some(len - 1);

                    // consume trailing marker
                    if len == 1 {
                        if let Some(m) = self.trailer {
                            let marker = self.de.peek_marker()?;
                            if marker == m {
                                self.de.read_marker()?;
                                self.len = Some(0);
                            }
                        }
                    }

                    Ok(Some(value))
                }
            }
            None => {
                // consume trailing marker
                if let Some(m) = self.trailer {
                    let marker = self.de.peek_marker()?;
                    if marker == m {
                        self.de.read_marker()?;
                        self.len = Some(0);
                        return Ok(None)
                    }
                }

                let value = seed.deserialize(&mut *self.de)?;

                // try consume trailing marker
                if let Some(m) = self.trailer {
                    let byte = self.de.peek_byte()?;
                    if let Ok(marker) = Marker::try_from(byte) {
                        if marker == m {
                            self.de.read_marker()?;
                            self.len = Some(0);
                        }
                    }
                }

                Ok(Some(value))
            }
        }
    }
}

struct ObjectAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: Option<usize>,
    of_type: Option<Marker>,
    trailer: Option<Marker>,
}

impl<'de, 'a> MapAccess<'de> for ObjectAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where
            K: DeserializeSeed<'de>,
    {
        match self.len {
            Some(len) => {
                if len == 0 {
                    Ok(None)
                } else {
                    // objects always have string keys
                    self.de.of_type = Some(Marker::String);
                    let value = seed.deserialize(&mut *self.de)?;
                    self.len = Some(len - 1);

                    // consume trailing marker
                    if len == 1 {
                        if let Some(m) = self.trailer {
                            let marker = self.de.peek_marker()?;
                            if marker == m {
                                self.de.read_marker()?;
                                self.len = Some(0);
                            }
                        }
                    }

                    Ok(Some(value))
                }
            }
            None => {
                // consume trailing marker
                if let Some(m) = self.trailer {
                    let marker = self.de.peek_marker()?;
                    if marker == m {
                        self.de.read_marker()?;
                        self.len = Some(0);
                        return Ok(None)
                    }
                }

                // objects always have string keys
                self.de.of_type = Some(Marker::String);
                let value = seed.deserialize(&mut *self.de)?;

                // try consume trailing marker
                if let Some(m) = self.trailer {
                    let byte = self.de.peek_byte()?;
                    if let Ok(marker) = Marker::try_from(byte) {
                        if marker == m {
                            self.de.read_marker()?;
                            self.len = Some(0);
                        }
                    }
                }

                Ok(Some(value))
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where
            V: DeserializeSeed<'de>,
    {
        // hint type to the deserializer if set
        self.de.of_type = self.of_type;
        let value = seed.deserialize(&mut *self.de)?;
        Ok(value)
    }
}

struct ItemAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> EnumAccess<'de> for ItemAccess<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
        where
            V: DeserializeSeed<'de>,
    {
        // objects always have string keys
        self.de.of_type = Some(Marker::String);
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for ItemAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Err(Error::InvalidMarker)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
        where
            T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_map(self.de, visitor)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[derive(Deserialize)]
    struct SimpleStruct {
        field1: i32,
        field2: String,
    }

    #[derive(Deserialize)]
    struct UniformStruct {
        field1: i32,
        field2: i32,
    }

    #[derive(Deserialize)]
    enum SimpleEnum {
        Unit,
        NewType(i32),
        Tuple(i32, i32),
        Struct { field1: i32, field2: i32 },
    }

    #[test]
    fn deserializing_big_t_value_can_produce_true() {
        let data = b"T";
        let value = from_bytes::<'_, bool>(data).unwrap();
        assert_eq!(value, true);
    }

    #[test]
    fn deserializing_big_f_value_can_produce_false() {
        let data = b"F";
        let value = from_bytes::<'_, bool>(data).unwrap();
        assert_eq!(value, false);
    }

    #[test]
    fn deserializing_small_i_value_can_produce_i8() {
        let mut data = vec![b'i'];
        data.extend_from_slice(&i8::MAX.to_be_bytes());

        let value = from_bytes::<'_, i8>(&data).unwrap();
        assert_eq!(value, i8::MAX);
    }

    #[test]
    fn deserializing_big_i_value_can_produce_i16() {
        let mut data = vec![b'I'];
        data.extend_from_slice(&i16::MAX.to_be_bytes());

        let value = from_bytes::<'_, i16>(&data).unwrap();
        assert_eq!(value, i16::MAX);
    }

    #[test]
    fn deserializing_small_i_value_can_produce_i16() {
        let mut data = vec![b'i'];
        data.extend_from_slice(&i8::MAX.to_be_bytes());

        let value = from_bytes::<'_, i16>(&data).unwrap();
        assert_eq!(value, i8::MAX as i16);
    }

    #[test]
    fn deserializing_small_l_value_can_produce_i32() {
        let mut data = vec![b'l'];
        data.extend_from_slice(&i32::MAX.to_be_bytes());

        let value = from_bytes::<'_, i32>(&data).unwrap();
        assert_eq!(value, i32::MAX);
    }

    #[test]
    fn deserializing_big_i_value_can_produce_i32() {
        let mut data = vec![b'I'];
        data.extend_from_slice(&i16::MAX.to_be_bytes());

        let value = from_bytes::<'_, i32>(&data).unwrap();
        assert_eq!(value, i16::MAX as i32);
    }

    #[test]
    fn deserializing_small_i_value_can_produce_i32() {
        let mut data = vec![b'i'];
        data.extend_from_slice(&i8::MAX.to_be_bytes());

        let value = from_bytes::<'_, i32>(&data).unwrap();
        assert_eq!(value, i8::MAX as i32);
    }

    #[test]
    fn deserializing_big_l_value_can_produce_i64() {
        let mut data = vec![b'L'];
        data.extend_from_slice(&i64::MAX.to_be_bytes());

        let value = from_bytes::<'_, i64>(&data).unwrap();
        assert_eq!(value, i64::MAX);
    }

    #[test]
    fn deserializing_small_l_value_can_produce_i64() {
        let mut data = vec![b'l'];
        data.extend_from_slice(&i32::MAX.to_be_bytes());

        let value = from_bytes::<'_, i64>(&data).unwrap();
        assert_eq!(value, i32::MAX as i64);
    }

    #[test]
    fn deserializing_big_i_value_can_produce_i64() {
        let mut data = vec![b'I'];
        data.extend_from_slice(&i16::MAX.to_be_bytes());

        let value = from_bytes::<'_, i64>(&data).unwrap();
        assert_eq!(value, i16::MAX as i64);
    }

    #[test]
    fn deserializing_small_i_value_can_produce_i64() {
        let mut data = vec![b'i'];
        data.extend_from_slice(&i8::MAX.to_be_bytes());

        let value = from_bytes::<'_, i64>(&data).unwrap();
        assert_eq!(value, i8::MAX as i64);
    }

    #[test]
    fn deserializing_small_d_value_can_produce_f32() {
        let mut data = vec![b'd'];
        data.extend_from_slice(&f32::MAX.to_be_bytes());

        let value = from_bytes::<'_, f32>(&data).unwrap();
        assert_eq!(value, f32::MAX);
    }

    #[test]
    fn deserializing_big_d_value_can_produce_f64() {
        let mut data = vec![b'D'];
        data.extend_from_slice(&f64::MAX.to_be_bytes());

        let value = from_bytes::<'_, f64>(&data).unwrap();
        assert_eq!(value, f64::MAX);
    }

    #[test]
    fn deserializing_small_d_value_can_produce_f64() {
        let mut data = vec![b'd'];
        data.extend_from_slice(&f32::MAX.to_be_bytes());

        let value = from_bytes::<'_, f64>(&data).unwrap();
        assert_eq!(value, f32::MAX as f64);
    }

    #[test]
    fn deserializing_big_c_value_can_produce_char() {
        let data = &[b'C', b'A'];

        let value = from_bytes::<'_, char>(data).unwrap();
        assert_eq!(value, 'A');
    }

    #[test]
    fn deserializing_big_s_value_of_small_i_len_can_produce_char() {
        let mut data = vec![b'S', b'i'];
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"A");

        let value = from_bytes::<'_, char>(&data).unwrap();
        assert_eq!(value, 'A');
    }

    #[test]
    fn deserializing_big_s_value_of_small_i_len_can_produce_string() {
        let mut data = vec![b'S', b'i'];
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, String>(&data).unwrap();
        assert_eq!(value, "test".to_string());
    }

    #[test]
    fn deserializing_big_s_value_of_big_i_len_can_produce_string() {
        let mut data = vec![b'S', b'I'];
        data.extend_from_slice(&4i16.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, String>(&data).unwrap();
        assert_eq!(value, "test".to_string());
    }

    #[test]
    fn deserializing_big_s_value_of_small_l_len_can_produce_string() {
        let mut data = vec![b'S', b'l'];
        data.extend_from_slice(&4i32.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, String>(&data).unwrap();
        assert_eq!(value, "test".to_string());
    }

    #[test]
    fn deserializing_big_s_value_of_big_l_len_can_produce_string() {
        let mut data = vec![b'S', b'L'];
        data.extend_from_slice(&4i64.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, String>(&data).unwrap();
        assert_eq!(value, "test".to_string());
    }

    #[test]
    fn deserializing_big_c_value_can_produce_string() {
        let data = b"CA";

        let value = from_bytes::<'_, String>(data).unwrap();
        assert_eq!(value, "A".to_string());
    }

    #[test]
    fn deserializing_big_s_value_of_small_i_len_can_produce_str() {
        let mut data = vec![b'S', b'i'];
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, &str>(&data).unwrap();
        assert_eq!(value, "test");
    }

    #[test]
    fn deserializing_big_s_value_of_big_i_len_can_produce_str() {
        let mut data = vec![b'S', b'I'];
        data.extend_from_slice(&4i16.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, &str>(&data).unwrap();
        assert_eq!(value, "test");
    }

    #[test]
    fn deserializing_big_s_value_of_small_l_len_can_produce_str() {
        let mut data = vec![b'S', b'l'];
        data.extend_from_slice(&4i32.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, &str>(&data).unwrap();
        assert_eq!(value, "test");
    }

    #[test]
    fn deserializing_big_s_value_of_big_l_len_can_produce_str() {
        let mut data = vec![b'S', b'L'];
        data.extend_from_slice(&4i64.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, &str>(&data).unwrap();
        assert_eq!(value, "test");
    }

    #[test]
    fn deserializing_big_c_value_can_produce_str() {
        let data = b"CA";

        let value = from_bytes::<'_, &str>(data).unwrap();
        assert_eq!(value, "A");
    }

    #[test]
    fn deserializing_big_z_value_can_produce_none() {
        let data = &[b'Z'];

        let value = from_bytes::<'_, Option<String>>(data).unwrap();
        assert!(matches!(value, None));
    }

    #[test]
    fn deserializing_big_s_value_of_small_i_len_can_produce_some_string() {
        let data = &[b'S', b'i', 4u8, b't', b'e', b's', b't'];

        let value = from_bytes::<'_, Option<String>>(data).unwrap();
        match value {
            Some(s) => assert_eq!(s, "test"),
            None => panic!("Expected string value"),
        }
    }

    #[test]
    fn deserializing_big_z_value_can_produce_unit() {
        let data = &[b'Z'];

        let value = from_bytes::<'_, ()>(data).unwrap();
        assert_eq!(value, ());
    }

    #[test]
    fn deserializing_open_and_close_bracket_can_produce_empty_vec() {
        let data = b"[]";

        let value = from_bytes::<'_, Vec<i8>>(data).unwrap();
        assert_eq!(value, vec![]);
    }

    #[test]
    fn deserializing_open_bracket_with_length_of_0_can_produce_empty_vec() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&0i8.to_be_bytes());

        let value = from_bytes::<'_, Vec<i8>>(&data).unwrap();
        assert_eq!(value, vec![]);
    }

    #[test]
    fn deserializing_open_and_close_bracket_with_small_i_values_can_produce_vec() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());
        data.extend_from_slice(b"i");
        data.extend_from_slice(&3i8.to_be_bytes());
        data.extend_from_slice(b"]");

        let value = from_bytes::<'_, Vec<i8>>(&data).unwrap();
        assert_eq!(value, vec![1i8, 2i8, 3i8]);
    }

    #[test]
    fn deserializing_open_and_close_bracket_with_strings_of_i_len_can_produce_vec() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());
        data.extend_from_slice(b"te");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&3i8.to_be_bytes());
        data.extend_from_slice(b"tes");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(b"]");

        let value = from_bytes::<'_, Vec<String>>(&data).unwrap();
        assert_eq!(
            value,
            vec![
                "t".to_string(),
                "te".to_string(),
                "tes".to_string(),
                "test".to_string(),
            ]
        );
    }

    #[test]
    fn deserializing_open_bracket_with_small_i_values_and_length_of_i_can_produce_vec() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&3i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());
        data.extend_from_slice(b"i");
        data.extend_from_slice(&3i8.to_be_bytes());

        let value = from_bytes::<'_, Vec<i8>>(&data).unwrap();
        assert_eq!(value, vec![1i8, 2i8, 3i8]);
    }

    #[test]
    fn deserializing_open_bracket_with_string_values_and_length_of_i_can_produce_vec() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());

        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());
        data.extend_from_slice(b"te");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&3i8.to_be_bytes());
        data.extend_from_slice(b"tes");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, Vec<String>>(&data).unwrap();
        assert_eq!(
            value,
            vec![
                "t".to_string(),
                "te".to_string(),
                "tes".to_string(),
                "test".to_string(),
            ]
        );
    }

    #[test]
    fn deserializing_open_bracket_with_small_i_values_of_type_i_and_of_len_i_can_produce_vec() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"$");
        data.extend_from_slice(b"i");
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&3i8.to_be_bytes());

        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(&2i8.to_be_bytes());
        data.extend_from_slice(&3i8.to_be_bytes());

        let value = from_bytes::<'_, Vec<i8>>(&data).unwrap();
        assert_eq!(value, vec![1i8, 2i8, 3i8]);
    }

    #[test]
    fn deserializing_open_bracket_with_string_values_of_type_big_s_and_of_len_i_can_produce_vec() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"$");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());
        data.extend_from_slice(b"te");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&3i8.to_be_bytes());
        data.extend_from_slice(b"tes");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");

        let value = from_bytes::<'_, Vec<String>>(&data).unwrap();
        assert_eq!(
            value,
            vec![
                "t".to_string(),
                "te".to_string(),
                "tes".to_string(),
                "test".to_string(),
            ]
        );
    }

    #[test]
    fn deserializing_open_bracket_with_big_u_values_of_len_i_can_produce_byte_slice() {
        let mut data = vec![b'['];
        data.extend_from_slice(b"$");
        data.extend_from_slice(b"U");
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());

        data.extend_from_slice(&1u8.to_be_bytes());
        data.extend_from_slice(&2u8.to_be_bytes());
        data.extend_from_slice(&3u8.to_be_bytes());
        data.extend_from_slice(&4u8.to_be_bytes());

        let value = from_bytes::<'_, &[u8]>(&data).unwrap();
        assert_eq!(value, &[1u8, 2u8, 3u8, 4u8]);
    }

    #[test]
    fn deserializing_open_and_close_brace_can_produce_empty_map() {
        let data = b"{}";

        let value = from_bytes::<'_, HashMap<String, i8>>(data).unwrap();
        assert_eq!(value.len(), 0);
    }

    #[test]
    fn deserializing_open_brace_with_length_of_0_can_produce_empty_map() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&0i8.to_be_bytes());

        let value = from_bytes::<'_, HashMap<String, i8>>(&data).unwrap();
        assert_eq!(value.len(), 0);
    }

    #[test]
    fn deserializing_open_and_close_brace_with_small_i_values_can_produce_map() {
        let mut data = vec![b'{'];

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());

        data.extend_from_slice(b"}");

        let value = from_bytes::<'_, HashMap<String, i8>>(&data).unwrap();
        assert_eq!(
            value,
            HashMap::from([("t".to_string(), 1), ("test".to_string(), 4)])
        );
    }

    #[test]
    fn deserializing_open_and_close_brace_with_strings_of_i_len_can_produce_map() {
        let mut data = vec![b'{'];

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"1");

        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"4");

        data.extend_from_slice(b"}");

        let value = from_bytes::<'_, HashMap<String, String>>(&data).unwrap();
        assert_eq!(
            value,
            HashMap::from([
                ("t".to_string(), "1".to_string()),
                ("test".to_string(), "4".to_string()),
            ])
        );
    }

    #[test]
    fn deserializing_open_brace_with_small_i_values_and_length_of_i_can_produce_map() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());

        let value = from_bytes::<'_, HashMap<String, i8>>(&data).unwrap();
        assert_eq!(
            value,
            HashMap::from([("t".to_string(), 1), ("test".to_string(), 4)])
        );
    }

    #[test]
    fn deserializing_open_brace_with_string_values_and_length_of_i_can_produce_map() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"1");

        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"4");

        let value = from_bytes::<'_, HashMap<String, String>>(&data).unwrap();
        assert_eq!(
            value,
            HashMap::from([
                ("t".to_string(), "1".to_string()),
                ("test".to_string(), "4".to_string()),
            ])
        );
    }

    #[test]
    fn deserializing_open_brace_with_small_i_values_of_type_i_and_of_len_i_can_produce_map() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"$");
        data.extend_from_slice(b"i");
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(&1i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(&4i8.to_be_bytes());

        let value = from_bytes::<'_, HashMap<String, i8>>(&data).unwrap();
        assert_eq!(
            value,
            HashMap::from([("t".to_string(), 1), ("test".to_string(), 4)])
        );
    }

    #[test]
    fn deserializing_open_brace_with_string_values_of_type_big_s_and_of_len_i_can_produce_map() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"$");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"t");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"1");

        data.extend_from_slice(b"i");
        data.extend_from_slice(&4i8.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"4");

        let value = from_bytes::<'_, HashMap<String, String>>(&data).unwrap();
        assert_eq!(
            value,
            HashMap::from([
                ("t".to_string(), "1".to_string()),
                ("test".to_string(), "4".to_string()),
            ])
        );
    }

    #[test]
    fn deserializing_open_and_close_brace_with_mixed_values_can_produce_struct() {
        let mut data = vec![b'{'];

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field1");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&1i32.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field2");
        data.extend_from_slice(b"S");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());
        data.extend_from_slice(b"6");

        data.extend_from_slice(b"}");

        let value = from_bytes::<'_, SimpleStruct>(&data).unwrap();
        assert_eq!(value.field1, 1i32);
        assert_eq!(value.field2, "6".to_string());
    }

    #[test]
    fn deserializing_open_brace_with_uniform_values_of_len_i_can_produce_struct() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"$");
        data.extend_from_slice(b"l");
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&2i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field1");
        data.extend_from_slice(&1i32.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field2");
        data.extend_from_slice(&2i32.to_be_bytes());

        let value = from_bytes::<'_, UniformStruct>(&data).unwrap();
        assert_eq!(value.field1, 1i32);
        assert_eq!(value.field2, 2i32);
    }

    #[test]
    fn deserializing_open_and_close_brace_with_1_value_can_produce_newtype_variant_of_enum() {
        let mut data = vec![b'{'];

        data.extend_from_slice(b"i");
        data.extend_from_slice(&7i8.to_be_bytes());
        data.extend_from_slice(b"NewType");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&7i32.to_be_bytes());

        data.extend_from_slice(b"}");

        let value = from_bytes::<'_, SimpleEnum>(&data).unwrap();
        match value {
            SimpleEnum::NewType(n) => assert_eq!(n, 7),
            _ => panic!("Expected newtype"),
        }
    }

    #[test]
    fn deserializing_open_brace_with_1_value_can_produce_newtype_variant_of_enum() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&7i8.to_be_bytes());
        data.extend_from_slice(b"NewType");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&7i32.to_be_bytes());

        let value = from_bytes::<'_, SimpleEnum>(&data).unwrap();
        match value {
            SimpleEnum::NewType(n) => assert_eq!(n, 7),
            _ => panic!("Expected newtype"),
        }
    }

    #[test]
    fn deserializing_open_and_close_brace_with_1_value_can_produce_tuple_variant_of_enum() {
        let mut data = vec![b'{'];

        data.extend_from_slice(b"i");
        data.extend_from_slice(&5i8.to_be_bytes());
        data.extend_from_slice(b"Tuple");

        data.extend_from_slice(b"[");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&1i32.to_be_bytes());
        data.extend_from_slice(b"l");
        data.extend_from_slice(&2i32.to_be_bytes());
        data.extend_from_slice(b"]");

        data.extend_from_slice(b"}");

        let value = from_bytes::<'_, SimpleEnum>(&data).unwrap();
        match value {
            SimpleEnum::Tuple(one, two) => {
                assert_eq!(one, 1);
                assert_eq!(two, 2);
            },
            _ => panic!("Expected tuple"),
        }
    }

    #[test]
    fn deserializing_open_brace_with_1_value_can_produce_tuple_variant_of_enum() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&5i8.to_be_bytes());
        data.extend_from_slice(b"Tuple");

        data.extend_from_slice(b"[");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&1i32.to_be_bytes());
        data.extend_from_slice(b"l");
        data.extend_from_slice(&2i32.to_be_bytes());
        data.extend_from_slice(b"]");

        let value = from_bytes::<'_, SimpleEnum>(&data).unwrap();
        match value {
            SimpleEnum::Tuple(one, two) => {
                assert_eq!(one, 1);
                assert_eq!(two, 2);
            },
            _ => panic!("Expected tuple"),
        }
    }

    #[test]
    fn deserializing_open_and_close_brace_with_1_value_can_produce_struct_variant_of_enum() {
        let mut data = vec![b'{'];

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"Struct");

        data.extend_from_slice(b"{");

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field1");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&1i32.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field2");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&2i32.to_be_bytes());

        data.extend_from_slice(b"}");

        data.extend_from_slice(b"}");

        let value = from_bytes::<'_, SimpleEnum>(&data).unwrap();
        match value {
            SimpleEnum::Struct { field1, field2 } => {
                assert_eq!(field1, 1);
                assert_eq!(field2, 2);
            },
            _ => panic!("Expected struct"),
        }
    }

    #[test]
    fn deserializing_open_brace_with_1_value_can_produce_struct_variant_of_enum() {
        let mut data = vec![b'{'];
        data.extend_from_slice(b"#");
        data.extend_from_slice(b"i");
        data.extend_from_slice(&1i8.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"Struct");

        data.extend_from_slice(b"{");

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field1");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&1i32.to_be_bytes());

        data.extend_from_slice(b"i");
        data.extend_from_slice(&6i8.to_be_bytes());
        data.extend_from_slice(b"field2");
        data.extend_from_slice(b"l");
        data.extend_from_slice(&2i32.to_be_bytes());

        data.extend_from_slice(b"}");

        let value = from_bytes::<'_, SimpleEnum>(&data).unwrap();
        match value {
            SimpleEnum::Struct { field1, field2 } => {
                assert_eq!(field1, 1);
                assert_eq!(field2, 2);
            },
            _ => panic!("Expected struct"),
        }
    }
}
