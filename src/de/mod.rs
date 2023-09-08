use std::{collections::BTreeMap, fmt::Display, vec::IntoIter};

use serde::de::{
    DeserializeSeed, Deserializer, EnumAccess, Expected, IntoDeserializer, MapAccess, SeqAccess,
    Unexpected, VariantAccess, Visitor,
};

use crate::value::Value;

#[cfg(test)]
mod test;

impl Value {
    #[cold]
    fn invalid_type<E>(&self, exp: &dyn Expected) -> E
    where
        E: serde::de::Error,
    {
        serde::de::Error::invalid_type(self.unexpected(), exp)
    }

    #[cold]
    fn unexpected(&self) -> Unexpected {
        match self {
            Value::UInt8(n) => Unexpected::Unsigned(*n as u64),
            Value::UInt16(n) => Unexpected::Unsigned(*n as u64),
            Value::UInt32(n) => Unexpected::Unsigned(*n as u64),
            Value::UInt64(n) => Unexpected::Unsigned(*n),
            Value::Int8(n) => Unexpected::Signed(*n as i64),
            Value::Int16(n) => Unexpected::Signed(*n as i64),
            Value::Int32(n) => Unexpected::Signed(*n as i64),
            Value::Int64(n) => Unexpected::Signed(*n),
            Value::Float32(n) => Unexpected::Float(n.get() as f64),
            Value::Float64(n) => Unexpected::Float(n.get()),
            Value::String(s) => Unexpected::Str(s),
            Value::StructKey(_s) => Unexpected::Other("struct field name"),
            Value::StructVariantKey(_s) => Unexpected::Other("struct variant"),
            Value::Unit => Unexpected::Unit,
            Value::Option(_) => Unexpected::Option,
            Value::Array(_) => Unexpected::Seq,
            Value::Tuple(_) => Unexpected::TupleVariant, // TODO
            Value::TupleStruct(_) => Unexpected::StructVariant,
            Value::NewtypeStruct(_) => Unexpected::NewtypeStruct,
            Value::Enum(..) => Unexpected::Enum,
            Value::EnumUnit(_) => Unexpected::Enum,
            Value::TupleVariant(..) => Unexpected::TupleVariant,
            Value::Map(_) => Unexpected::Map,
            Value::Struct(_) => Unexpected::NewtypeStruct, // TODO
            Value::StructVariant(..) => Unexpected::NewtypeVariant,
        }
    }
}

#[derive(Debug)]
pub struct MyErr;

impl serde::de::Error for MyErr {
    fn custom<T: Display>(msg: T) -> Self {
        todo!("{msg}")
    }
}

impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "abc".fmt(f)
    }
}

macro_rules! deserialize_primitive {
    ($method:ident, $type:ident, $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            if let Value::$type(n) = self {
                visitor.$visit(n)
            } else {
                Err(self.invalid_type(&visitor))
            }
        }
    };
}

impl<'de> Deserializer<'de> for Value {
    type Error = MyErr;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::UInt8(n) => visitor.visit_u8(n),
            Value::UInt16(n) => visitor.visit_u16(n),
            Value::UInt32(n) => visitor.visit_u32(n),
            Value::UInt64(n) => visitor.visit_u64(n),
            Value::Int8(n) => visitor.visit_i8(n),
            Value::Int16(n) => visitor.visit_i16(n),
            Value::Int32(n) => visitor.visit_i32(n),
            Value::Int64(n) => visitor.visit_i64(n),
            Value::String(s) => visitor.visit_string(s),
            Value::Option(val) => match *val {
                Some(s) => visitor.visit_some(s),
                None => visitor.visit_none(),
            },
            Value::Unit => visitor.visit_unit(),
            Value::Tuple(vec) => visit_array(vec, visitor),
            Value::Array(vec) => visit_array(vec, visitor),
            Value::TupleStruct(vec) => visit_array(vec, visitor),
            // Value::EnumUnit(variant) => visitor.visit_enum(EnumDeserializer {
            //     variant: variant.to_string(),
            //     value: vec![].into_iter(),
            // }),
            // Value::Enum(name, val) | Value::TupleVariant(name, val) => {
            //     visitor.visit_enum(EnumDeserializer {
            //         variant: name.to_string(),
            //         value: val.into_iter(),
            //     })
            // }
            //Value::Map(tree) => visitor.visit_map(MapValue::from_map(tree)),
            _ => todo!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    deserialize_primitive!(deserialize_i8, Int8, visit_i8);
    deserialize_primitive!(deserialize_i16, Int16, visit_i16);
    deserialize_primitive!(deserialize_i32, Int32, visit_i32);
    deserialize_primitive!(deserialize_i64, Int64, visit_i64);

    deserialize_primitive!(deserialize_u8, UInt8, visit_u8);
    deserialize_primitive!(deserialize_u16, UInt16, visit_u16);
    deserialize_primitive!(deserialize_u32, UInt32, visit_u32);
    deserialize_primitive!(deserialize_u64, UInt64, visit_u64);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Float32(n) = self {
            visitor.visit_f32(n.into())
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Float64(n) = self {
            visitor.visit_f64(n.into())
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::String(n) = self {
            visitor.visit_str(&n)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::String(n) = self {
            visitor.visit_string(n)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Option(val) = self {
            match *val {
                Some(value) => visitor.visit_some(value),
                None => visitor.visit_none(),
            }
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Unit = self {
            visitor.visit_unit()
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::NewtypeStruct(vec) = self {
            visit_array(vec, visitor)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Array(vec) = self {
            visit_array(vec, visitor)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Tuple(vec) = self {
            visit_array(vec, visitor)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::TupleStruct(vec) = self {
            visit_array(vec, visitor)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Map(map) = self {
            let mut deserializer = MapDeserializer::new(map);
            visitor.visit_map(&mut deserializer)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Struct(map) = self {
            let mut deserializer = MapDeserializer::new(map);
            visitor.visit_map(&mut deserializer)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let der = match self {
            Value::Enum(variant, value) | Value::TupleVariant(variant, value) => {
                EnumDeserializer::from_value(variant.to_string(), value)
            }
            Value::EnumUnit(variant) => EnumDeserializer::from_value(variant.to_string(), vec![]),
            Value::StructVariant(variant, map) => {
                EnumDeserializer::from_map(variant.to_string(), map)
            }
            other => {
                return Err(serde::de::Error::invalid_type(
                    other.unexpected(),
                    &"string or map",
                ));
            }
        };
        visitor.visit_enum(der)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::StructKey(n) = self {
            visitor.visit_string(n)
        } else if let Value::StructVariantKey(n) = self {
            visitor.visit_string(n)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

struct SeqDeserializer {
    iter: IntoIter<Value>,
}

impl SeqDeserializer {
    fn new(vec: Vec<Value>) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = MyErr;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

fn visit_array<'de, V>(array: Vec<Value>, visitor: V) -> Result<V::Value, MyErr>
where
    V: Visitor<'de>,
{
    let len = array.len();
    let mut deserializer = SeqDeserializer::new(array);
    visitor.visit_seq(&mut deserializer)
}

struct EnumDeserializer {
    variant: String,
    value: Vec<Value>,
    tree: BTreeMap<Value, Value>,
}

impl EnumDeserializer {
    fn from_value(variant: String, value: Vec<Value>) -> Self {
        Self {
            variant,
            value,
            tree: BTreeMap::new(),
        }
    }
    fn from_map(variant: String, tree: BTreeMap<Value, Value>) -> Self {
        Self {
            variant,
            value: vec![],
            tree,
        }
    }
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = MyErr;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer {
            value: self.value,
            tree: self.tree,
        };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

struct VariantDeserializer {
    value: Vec<Value>,
    tree: BTreeMap<Value, Value>,
}

impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = MyErr;

    fn unit_variant(self) -> Result<(), Self::Error> {
        // debug_assert!(self.value.len()==0);
        // debug_assert!(self.tree.len()==0);
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // debug_assert!(self.tree.len()==0);
        let mut value = self.value;
        match value.pop() {
            Some(v) => seed.deserialize(v),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_empty() {
            // TODO
            visitor.visit_unit()
        } else {
            visit_array(self.value, visitor)
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut deserializer = MapDeserializer::new(self.tree);
        visitor.visit_map(&mut deserializer)
    }
}

struct MapDeserializer {
    iter: <BTreeMap<Value, Value> as IntoIterator>::IntoIter,
    value: Option<Value>,
}

impl MapDeserializer {
    fn new(map: BTreeMap<Value, Value>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = MyErr;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct MapRefDeserializer<'de> {
    iter: <&'de BTreeMap<Value, Value> as IntoIterator>::IntoIter,
    value: Option<&'de Value>,
}

impl<'de> MapRefDeserializer<'de> {
    fn new(map: &'de BTreeMap<Value, Value>) -> Self {
        MapRefDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}

// TODO  &Value need to implement Deserializer
impl<'de> MapAccess<'de> for MapRefDeserializer<'de> {
    type Error = MyErr;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.clone()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value.clone()),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
