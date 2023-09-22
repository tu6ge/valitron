use std::{collections::BTreeMap, fmt::Display, marker::PhantomData};

use serde::ser;

use crate::value::mock::Value;

pub(crate) struct Serializer<'ser>(PhantomData<&'ser u8>);

impl Serializer<'_> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

#[derive(Debug)]
pub struct Error;

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        todo!("{msg}")
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "seralize error".fmt(f)
    }
}

impl<'ser> serde::ser::Serializer for Serializer<'ser> {
    type Ok = Value<'ser>;

    type Error = Error;

    type SerializeSeq = SerializeSeq<'ser>;

    type SerializeTuple = SerializeTuple<'ser>;

    type SerializeTupleStruct = SerializeTupleStruct<'ser>;

    type SerializeTupleVariant = SerializeTupleVariant<'ser>;

    type SerializeMap = SerializeMap<'ser>;

    type SerializeStruct = SerializeStruct<'ser>;

    type SerializeStructVariant = SerializeStructVariant<'ser>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Boolean(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_owned()))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(Value::Option(Box::new(Some(
            value.serialize(Serializer(PhantomData))?,
        ))))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Unit)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::EnumUnit(variant))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(Value::NewtypeStruct(vec![value.serialize(self)?]))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(Value::Enum(variant, vec![value.serialize(self)?]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => Ok(SerializeSeq::with_capacity(len)),
            None => Ok(SerializeSeq::new()),
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple::with_capacity(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct::with_capacity(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant::with_capacity(variant, len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap::new())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct(BTreeMap::default()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant::new(variant))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Uint8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Uint16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Uint32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Uint64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        //Ok(Value::Float32(v.into()))
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        //Ok(Value::Float64(v.into()))
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Char(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bytes(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Option(Box::new(None)))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub(crate) struct SerializeSeq<'ser>(Vec<Value<'ser>>);

impl SerializeSeq<'_> {
    fn new() -> Self {
        Self::default()
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<'ser> serde::ser::SerializeSeq for SerializeSeq<'ser> {
    type Error = Error;
    type Ok = Value<'ser>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.push(value.serialize(Serializer(PhantomData))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.0))
    }
}

#[derive(Default)]
pub struct SerializeTuple<'ser>(Vec<Value<'ser>>);

impl SerializeTuple<'_> {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<'ser> ser::SerializeTuple for SerializeTuple<'ser> {
    type Error = Error;
    type Ok = Value<'ser>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.push(value.serialize(Serializer(PhantomData))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Tuple(self.0))
    }
}

#[derive(Default)]
pub struct SerializeTupleStruct<'ser>(Vec<Value<'ser>>);

impl SerializeTupleStruct<'_> {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<'ser> ser::SerializeTupleStruct for SerializeTupleStruct<'ser> {
    type Error = Error;
    type Ok = Value<'ser>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.push(value.serialize(Serializer(PhantomData))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleStruct(self.0))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SerializeTupleVariant<'ser> {
    variant: &'static str,
    map: Vec<Value<'ser>>,
}

impl SerializeTupleVariant<'_> {
    fn with_capacity(variant: &'static str, len: usize) -> Self {
        Self {
            variant,
            map: Vec::with_capacity(len),
        }
    }
}
impl<'ser> ser::SerializeTupleVariant for SerializeTupleVariant<'ser> {
    type Error = Error;
    type Ok = Value<'ser>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.map.push(value.serialize(Serializer(PhantomData))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleVariant(self.variant, self.map))
    }
}

pub(crate) struct SerializeMap<'ser> {
    map: BTreeMap<Value<'ser>, Value<'ser>>,
    next_key: Option<Value<'ser>>,
}

impl SerializeMap<'_> {
    fn new() -> Self {
        SerializeMap {
            map: BTreeMap::new(),
            next_key: None,
        }
    }
}

impl<'ser> ser::SerializeMap for SerializeMap<'ser> {
    type Error = Error;
    type Ok = Value<'ser>;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.next_key = Some(key.serialize(Serializer(PhantomData))?);

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let key = self.next_key.take().ok_or(Error)?;
        self.map
            .insert(key, value.serialize(Serializer(PhantomData))?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SerializeStruct<'ser>(BTreeMap<Value<'ser>, Value<'ser>>);
impl<'ser> ser::SerializeStruct for SerializeStruct<'ser> {
    type Error = Error;
    type Ok = Value<'ser>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.insert(
            Value::StructKey(key.to_string()),
            value.serialize(Serializer(PhantomData))?,
        );

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct(self.0))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SerializeStructVariant<'ser> {
    variant: &'static str,
    map: BTreeMap<Value<'ser>, Value<'ser>>,
}

impl SerializeStructVariant<'_> {
    fn new(variant: &'static str) -> Self {
        Self {
            variant,
            map: BTreeMap::new(),
        }
    }
}
impl<'ser> ser::SerializeStructVariant for SerializeStructVariant<'ser> {
    type Error = Error;
    type Ok = Value<'ser>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.map.insert(
            Value::StructVariantKey(key.to_string()),
            value.serialize(Serializer(PhantomData))?,
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::StructVariant(self.variant, self.map))
    }
}
