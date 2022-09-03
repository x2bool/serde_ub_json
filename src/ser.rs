use std::io::Write;

use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::Serialize;

use crate::{Error, Result};
use crate::value::Marker;

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
{
    let mut bytes = Vec::new();
    let policy = SimpleFormatter::new(&mut bytes);
    let mut serializer = Serializer::new(policy);
    value.serialize(&mut serializer)?;
    Ok(bytes)
}

pub struct Serializer<F> {
    formatter: F,
}

impl<F> Serializer<F>
    where
        F: Formatter,
{
    pub fn new(formatter: F) -> Self {
        Self { formatter }
    }
}

impl<'a, F> serde::ser::Serializer for &'a mut Serializer<F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ArraySerializer<'a, F>;
    type SerializeTuple = ArraySerializer<'a, F>;
    type SerializeTupleStruct = ArraySerializer<'a, F>;
    type SerializeTupleVariant = VariantSerializer<'a, F>;
    type SerializeMap = ObjectSerializer<'a, F>;
    type SerializeStruct = ObjectSerializer<'a, F>;
    type SerializeStructVariant = VariantSerializer<'a, F>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.bool(v)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.i8(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.i16(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.i32(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.i64(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.u8(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.u16(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.u32(v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::Number)?;

        let s = v.to_string();
        let bytes = s.as_bytes();
        let len = bytes.len();

        self.formatter.len(len)?;
        self.formatter.raw(&bytes)?;

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.f32(v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.f64(v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let s = v.to_string();
        self.serialize_str(s.as_str())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_value() {
            self.formatter.mark(Marker::String)?;
        }

        let bytes = v.as_bytes();
        let len = bytes.len();

        self.formatter.len(len)?;
        self.formatter.raw(&bytes)?;

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::ArrayStart)?;
        self.formatter.mark(Marker::Length)?;
        self.formatter.len(v.len())?;

        for b in v {
            self.formatter.mark(Marker::U8)?;
            self.formatter.raw(&b.to_be_bytes())?;
        }

        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::Null)?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::Null)?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::ObjectStart)?;
        self.formatter.mark(Marker::Length)?;
        self.formatter.len(1)?;

        self.formatter.set_mode(FormatterMode::Key);
        variant.serialize(&mut *self)?;

        self.formatter.set_mode(FormatterMode::Value);
        value.serialize(&mut *self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::ArrayStart)?;

        if let Some(len) = len {
            self.formatter.mark(Marker::Length)?;
            self.formatter.len(len)?;
        }

        Ok(Self::SerializeSeq { len, ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::ObjectStart)?;
        self.formatter.mark(Marker::Length)?;
        self.formatter.len(1)?;

        self.formatter.set_mode(FormatterMode::Key);
        variant.serialize(&mut *self)?;

        self.formatter.set_mode(FormatterMode::Value);
        self.formatter.mark(Marker::ArrayStart)?;
        self.formatter.mark(Marker::Length)?;
        self.formatter.len(len)?;

        Ok(Self::SerializeTupleVariant { ser: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::ObjectStart)?;

        if let Some(len) = len {
            self.formatter.mark(Marker::Length)?;
            self.formatter.len(len)?;
        }

        Ok(Self::SerializeMap { len, ser: self })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        if self.formatter.get_mode().is_key() {
            return Err(Error::InvalidKey);
        }

        self.formatter.mark(Marker::ObjectStart)?;
        self.formatter.mark(Marker::Length)?;
        self.formatter.len(1)?;

        self.formatter.set_mode(FormatterMode::Key);
        variant.serialize(&mut *self)?;

        self.formatter.set_mode(FormatterMode::Value);
        self.formatter.mark(Marker::ObjectStart)?;
        self.formatter.mark(Marker::Length)?;
        self.formatter.len(len)?;

        Ok(Self::SerializeStructVariant { ser: self })
    }
}

pub struct ArraySerializer<'a, F> {
    len: Option<usize>,
    ser: &'a mut Serializer<F>,
}

impl<'a, F> SerializeSeq for ArraySerializer<'a, F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        if self.len.is_none() {
            self.ser.formatter.mark(Marker::ArrayEnd)?;
        }
        Ok(())
    }
}

impl<'a, F> SerializeTuple for ArraySerializer<'a, F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, F> SerializeTupleStruct for ArraySerializer<'a, F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

pub struct ObjectSerializer<'a, F> {
    len: Option<usize>,
    ser: &'a mut Serializer<F>,
}

impl<'a, F> SerializeMap for ObjectSerializer<'a, F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
        where
            T: Serialize,
    {
        self.ser.formatter.set_mode(FormatterMode::Key);
        key.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: Serialize,
    {
        self.ser.formatter.set_mode(FormatterMode::Value);
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        if self.len.is_none() {
            self.ser.formatter.mark(Marker::ObjectEnd)?;
        }
        Ok(())
    }
}

impl<'a, F> SerializeStruct for ObjectSerializer<'a, F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        self.serialize_key(key)?;
        self.serialize_value(value)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        if self.len.is_none() {
            self.ser.formatter.mark(Marker::ObjectEnd)?;
        }
        Ok(())
    }
}

pub struct VariantSerializer<'a, F> {
    ser: &'a mut Serializer<F>,
}

impl<'a, F> SerializeTupleVariant for VariantSerializer<'a, F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, F> SerializeStructVariant for VariantSerializer<'a, F>
    where
        F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        self.ser.formatter.set_mode(FormatterMode::Key);
        key.serialize(&mut *self.ser)?;

        self.ser.formatter.set_mode(FormatterMode::Value);
        value.serialize(&mut *self.ser)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

pub trait Formatter {
    fn set_mode(&mut self, mode: FormatterMode);
    fn get_mode(&mut self) -> FormatterMode;

    fn raw(&mut self, v: &[u8]) -> std::io::Result<()>;

    fn bool(&mut self, v: bool) -> std::io::Result<()>;

    fn u8(&mut self, v: u8) -> std::io::Result<()>;
    fn u16(&mut self, v: u16) -> std::io::Result<()>;
    fn u32(&mut self, v: u32) -> std::io::Result<()>;

    fn i8(&mut self, v: i8) -> std::io::Result<()>;
    fn i16(&mut self, v: i16) -> std::io::Result<()>;
    fn i32(&mut self, v: i32) -> std::io::Result<()>;
    fn i64(&mut self, v: i64) -> std::io::Result<()>;

    fn f32(&mut self, v: f32) -> std::io::Result<()>;
    fn f64(&mut self, v: f64) -> std::io::Result<()>;

    fn mark(&mut self, marker: Marker) -> std::io::Result<()>;

    fn len(&mut self, v: usize) -> std::io::Result<()>;
}

pub struct SimpleFormatter<'a, W> {
    writer: &'a mut W,
    mode: FormatterMode,
}

impl<'a, W> SimpleFormatter<'a, W>
    where
        W: Write,
{
    pub fn new(writer: &'a mut W) -> SimpleFormatter<'a, W> {
        SimpleFormatter {
            writer,
            mode: FormatterMode::Value,
        }
    }
}

impl<'a, W> Formatter for SimpleFormatter<'a, W>
    where
        W: Write,
{
    fn set_mode(&mut self, mode: FormatterMode) {
        self.mode = mode;
    }

    fn get_mode(&mut self) -> FormatterMode {
        self.mode
    }

    fn raw(&mut self, v: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(v)
    }

    fn bool(&mut self, v: bool) -> std::io::Result<()> {
        self.mark(if v { Marker::True } else { Marker::False })
    }

    fn u8(&mut self, v: u8) -> std::io::Result<()> {
        self.mark(Marker::U8)?;
        self.writer.write_all(&v.to_be_bytes())
    }

    fn u16(&mut self, v: u16) -> std::io::Result<()> {
        self.i32(v as i32)
    }

    fn u32(&mut self, v: u32) -> std::io::Result<()> {
        self.i64(v as i64)
    }

    fn i8(&mut self, v: i8) -> std::io::Result<()> {
        self.mark(Marker::I8)?;
        self.writer.write_all(&v.to_be_bytes())
    }

    fn i16(&mut self, v: i16) -> std::io::Result<()> {
        self.mark(Marker::I16)?;
        self.writer.write_all(&v.to_be_bytes())
    }

    fn i32(&mut self, v: i32) -> std::io::Result<()> {
        self.mark(Marker::I32)?;
        self.writer.write_all(&v.to_be_bytes())
    }

    fn i64(&mut self, v: i64) -> std::io::Result<()> {
        self.mark(Marker::I64)?;
        self.writer.write_all(&v.to_be_bytes())
    }

    fn f32(&mut self, v: f32) -> std::io::Result<()> {
        self.mark(Marker::F32)?;
        self.writer.write_all(&v.to_be_bytes())
    }

    fn f64(&mut self, v: f64) -> std::io::Result<()> {
        self.mark(Marker::F64)?;
        self.writer.write_all(&v.to_be_bytes())
    }

    fn mark(&mut self, marker: Marker) -> std::io::Result<()> {
        self.writer.write_all(marker.into())
    }

    fn len(&mut self, v: usize) -> std::io::Result<()> {
        self.i64(v as i64)
    }
}

#[derive(Copy, Clone)]
pub enum FormatterMode {
    Key,
    Value,
}

impl FormatterMode {
    pub fn is_key(&self) -> bool {
        match self {
            FormatterMode::Key => true,
            FormatterMode::Value => false,
        }
    }
    pub fn is_value(&self) -> bool {
        !self.is_key()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::mem::*;

    use super::*;

    #[derive(Serialize)]
    struct SimpleStruct {
        field1: i32,
        field2: String,
    }

    #[test]
    fn serializing_true_produces_1_byte_big_t_value() {
        let value = true;
        let out = to_bytes(&value).unwrap();
        assert_eq!(out, vec![b'T']);
    }

    #[test]
    fn serializing_false_produces_1_byte_big_f_value() {
        let value = false;
        let out = to_bytes(&value).unwrap();
        assert_eq!(out, vec![b'F']);
    }

    #[test]
    fn serializing_u8_produces_2_byte_big_u_value() {
        let value = 255u8;
        let out = to_bytes(&value).unwrap();
        assert_eq!(out, vec![b'U', value.to_be()]);
    }

    #[test]
    fn serializing_i8_produces_2_byte_small_i_value() {
        let value = 127i8;
        let out = to_bytes(&value).unwrap();
        assert_eq!(
            out,
            vec![b'i', unsafe { transmute::<i8, u8>(value.to_be()) }]
        );
    }

    #[test]
    fn serializing_i16_produces_3_byte_big_i_value() {
        let value = 32767i16;
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 3);
        assert_eq!(out[0], b'I');
        assert_eq!(out[1..], value.to_be_bytes());
    }

    #[test]
    fn serializing_i32_produces_5_byte_small_l_value() {
        let value = 2147483647i32;
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 5);
        assert_eq!(out[0], b'l');
        assert_eq!(out[1..], value.to_be_bytes());
    }

    #[test]
    fn serializing_i64_produces_9_byte_big_l_value() {
        let value = 9_223_372_036_854_775_807i64;
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 9);
        assert_eq!(out[0], b'L');
        assert_eq!(out[1..], value.to_be_bytes());
    }

    #[test]
    fn serializing_f32_produces_5_byte_small_d_value() {
        let value = 3.14f32;
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 5);
        assert_eq!(out[0], b'd');
        assert_eq!(out[1..], value.to_be_bytes());
    }

    #[test]
    fn serializing_f64_produces_9_byte_big_d_value() {
        let value = 3.14f64;
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 9);
        assert_eq!(out[0], b'D');
        assert_eq!(out[1..], value.to_be_bytes());
    }

    #[test]
    fn serializing_str_produces_big_l_string_value() {
        let str = (0..127).map(|_| 'X').collect::<String>();
        let value = str.as_str();
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 1 + 8 + 1 + 127); // S + L + (size) + 127
        assert_eq!(out[0], b'S');
        assert_eq!(out[1], b'L');
        assert_eq!(out[2..10], 127i64.to_be_bytes());
        assert_eq!(&out[10..], value.as_bytes());
    }

    // #[test]
    // fn serializing_str_of_length_127_produces_small_i_string_value() {
    //     let str = (0..127).map(|n| 'X').collect::<String>();
    //     let value = str.as_str();
    //     let out = to_bytes(&value).unwrap();
    //
    //     assert_eq!(out.len(), 1 + 1 + 1 + 127); // S + i + (size) + 127
    //     assert_eq!(out[0], b'S');
    //     assert_eq!(out[1], b'i');
    //     assert_eq!(out[2..3], 127i8.to_be_bytes());
    //     assert_eq!(&out[3..], value.as_bytes());
    // }
    //
    // #[test]
    // fn serializing_str_of_length_32767_produces_big_i_string_value() {
    //     let str = (0..32767).map(|n| 'X').collect::<String>();
    //     let value = str.as_str();
    //     let out = to_bytes(&value).unwrap();
    //
    //     assert_eq!(out.len(), 1 + 1 + 2 + 32767); // S + i + (size) + 32767
    //     assert_eq!(out[0], b'S');
    //     assert_eq!(out[1], b'I');
    //     assert_eq!(out[2..4], 32767i16.to_be_bytes());
    //     assert_eq!(&out[4..], value.as_bytes());
    // }

    #[test]
    fn serializing_none_produces_big_z_string_value() {
        let value: Option<i32> = None;
        let out = to_bytes(&value).unwrap();

        assert_eq!(out, vec![b'Z']);
    }

    #[test]
    fn serializing_unit_produces_big_n_string_value() {
        let value = ();
        let out = to_bytes(&value).unwrap();

        assert_eq!(out, vec![b'Z']);
    }

    #[test]
    fn serializing_vec_of_bytes_produces_array_value() {
        let value = b"test".to_vec();
        let out = to_bytes(&value).unwrap();

        let len = (value.len() as i64).to_be_bytes();

        assert_eq!(b"[#L", &out[..3]);
        assert_eq!(&len, &out[3..11]);
        assert_eq!(&out[11..], b"UtUeUsUt");
    }

    #[test]
    fn serializing_vec_of_strings_produces_array_value() {
        let value = vec!["one", "two"];
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 37);

        let len = (value.len() as i64).to_be_bytes();
        let mut span = vec![b'[', b'#', b'L'];
        span.extend_from_slice(&len);
        assert_eq!(out[..11], span);

        let len = (value[0].len() as i64).to_be_bytes();
        let mut span = vec![b'S', b'L'];
        span.extend_from_slice(&len);
        span.extend_from_slice(value[0].as_bytes());
        assert_eq!(out[11..24], span);

        let len = (value[1].len() as i64).to_be_bytes();
        let mut span = vec![b'S', b'L'];
        span.extend_from_slice(&len);
        span.extend_from_slice(value[1].as_bytes());
        assert_eq!(out[24..37], span);
    }

    #[test]
    fn serializing_map_of_strings_to_i32_produces_object_value() {
        let value = HashMap::from([("key1", 1i32), ("key2", 2i32)]);
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 47);

        let len = (value.len() as u64).to_be_bytes();
        let mut span = vec![b'{', b'#', b'L'];
        span.extend_from_slice(&len);
        assert_eq!(out[..11], span);

        let entries = value.iter().collect::<Vec<_>>();

        // 1st entry
        let len = (entries[0].0.len() as i64).to_be_bytes();
        let mut span = vec![b'L'];
        span.extend_from_slice(&len);
        span.extend_from_slice(entries[0].0.as_bytes());
        assert_eq!(out[11..24], span);

        assert_eq!(out[24], b'l');
        assert_eq!(out[25..29], entries[0].1.to_be_bytes());

        // 2nd entry
        let len = (entries[1].0.len() as i64).to_be_bytes();
        let mut span = vec![b'L'];
        span.extend_from_slice(&len);
        span.extend_from_slice(entries[1].0.as_bytes());
        assert_eq!(out[29..42], span);

        assert_eq!(out[42], b'l');
        assert_eq!(out[43..47], entries[1].1.to_be_bytes());
    }

    #[test]
    fn serializing_map_of_not_string_to_any_produces_error() {
        let value = HashMap::from([(true, "val1"), (false, "val2")]);
        let result = to_bytes(&value);
        match result {
            Err(e) => assert!(matches!(e, Error::InvalidKey)),
            _ => panic!("Expected error"),
        }

        let value = HashMap::from([(0, "val1"), (1, "val2")]);
        let result = to_bytes(&value);
        match result {
            Err(e) => assert!(matches!(e, Error::InvalidKey)),
            _ => panic!("Expected error"),
        }
    }

    #[test]
    fn serializing_simple_struct_produces_object_value() {
        let value = SimpleStruct {
            field1: 1,
            field2: "val".to_string(),
        };
        let out = to_bytes(&value).unwrap();

        assert_eq!(out.len(), 59);

        let len = 2i64.to_be_bytes();
        let mut span = vec![b'{', b'#', b'L'];
        span.extend_from_slice(&len);
        assert_eq!(out[..11], span);

        // 1st field
        let len = ("field1".len() as i64).to_be_bytes();
        let mut span = vec![b'L'];
        span.extend_from_slice(&len);
        span.extend_from_slice("field1".as_bytes());
        assert_eq!(out[11..26], span);

        assert_eq!(out[26], b'l');
        assert_eq!(out[27..31], 1i32.to_be_bytes());

        // 2nd field
        let len = ("field2".len() as i64).to_be_bytes();
        let mut span = vec![b'L'];
        span.extend_from_slice(&len);
        span.extend_from_slice("field2".as_bytes());
        assert_eq!(out[31..46], span);

        assert_eq!(out[46], b'S');
        assert_eq!(out[47], b'L');
        assert_eq!(out[48..56], 3i64.to_be_bytes());
        assert_eq!(&out[56..], b"val");
    }
}
