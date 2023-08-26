use std::collections::{BTreeMap, HashMap};

use serde::ser;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Int8(u8),
    String(String),
    //UnInt8(u8),
    // Boolean(bool),
    // Char(char),
    Struct(Map),
}

pub struct ValueMap {
    value: Value,
    index: &'static str,
}

impl ValueMap {
    pub(crate) fn current(&self) -> Option<&Value> {
        self.value.get(self.index)
    }
    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        self.value.get(key)
    }
}

type Map = BTreeMap<String, Value>;

pub fn to_value<T>(value: T) -> Result<Value, MyErr>
where
    T: ser::Serialize,
{
    value.serialize(Serializer)
}

impl Value {
    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        if let Self::Struct(map) = self {
            map.get(key)
        } else {
            None
        }
    }
    pub(crate) fn get_clone(&self, key: &str) -> Option<Value> {
        self.get(key).map(Clone::clone)
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Int8(_) => true,
            Self::String(_) => true,
            _ => false,
        }
    }
}

struct Serializer;

struct Compound;

#[derive(Debug, PartialEq, Eq)]
struct SerializeStruct {
    fields: Map,
}

#[derive(Debug)]
pub struct MyErr;

impl serde::ser::Error for MyErr {
    fn custom<T>(msg: T) -> Self {
        todo!()
    }
}

impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "abc".fmt(f)
    }
}

impl serde::ser::Serializer for Serializer {
    type Ok = Value;

    type Error = MyErr;

    type SerializeSeq = Compound;

    type SerializeTuple = Compound;

    type SerializeTupleStruct = Compound;

    type SerializeTupleVariant = Compound;

    type SerializeMap = Compound;

    type SerializeStruct = SerializeStruct;

    type SerializeStructVariant = Compound;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_owned()))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct {
            fields: Map::default(),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl serde::ser::SerializeSeq for Compound {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeTuple for Compound {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl ser::SerializeTupleStruct for Compound {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeTupleVariant for Compound {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeMap for Compound {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl ser::SerializeStruct for SerializeStruct {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.fields
            .insert(key.to_owned(), value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct(self.fields))
    }
}
impl ser::SerializeStructVariant for Compound {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
