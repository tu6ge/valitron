use std::{collections::BTreeMap, fmt::Display, vec::IntoIter};

use serde::de::{
    DeserializeSeed, Deserializer, EnumAccess, Expected, IntoDeserializer, MapAccess, SeqAccess,
    Unexpected, VariantAccess, Visitor,
};

use crate::value::mock::Value;

impl Value<'_> {
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
            Value::Uint8(n) => Unexpected::Unsigned(*n as u64),
            Value::Uint16(n) => Unexpected::Unsigned(*n as u64),
            Value::Uint32(n) => Unexpected::Unsigned(*n as u64),
            Value::Uint64(n) => Unexpected::Unsigned(*n),
            //Value::USize(n) => Unexpected::Unsigned(*n as u64),
            Value::Int8(n) => Unexpected::Signed(*n as i64),
            Value::Int16(n) => Unexpected::Signed(*n as i64),
            Value::Int32(n) => Unexpected::Signed(*n as i64),
            Value::Int64(n) => Unexpected::Signed(*n),
            //Value::ISize(n) => Unexpected::Signed(*n as i64),
            // Value::Float32(n) => Unexpected::Float(n.get() as f64),
            // Value::Float64(n) => Unexpected::Float(n.get()),
            Value::Boolean(b) => Unexpected::Bool(*b),
            Value::Char(ch) => Unexpected::Char(*ch),
            Value::String(s) => Unexpected::Str(s),
            Value::Str(s) => Unexpected::Str(s),
            Value::Bytes(n) => Unexpected::Bytes(n),
            Value::StructKey(_s) => Unexpected::Other("struct field name"),
            Value::StructVariantKey(_s) => Unexpected::Other("struct variant"),
            Value::Unit => Unexpected::Unit,
            Value::Option(_) => Unexpected::Option,
            Value::Array(_) => Unexpected::Seq,
            Value::Tuple(_) => Unexpected::TupleVariant,
            Value::TupleStruct(_) => Unexpected::StructVariant,
            Value::NewtypeStruct(_) => Unexpected::NewtypeStruct,
            Value::Enum(..) => Unexpected::Enum,
            Value::EnumUnit(_) => Unexpected::Enum,
            Value::TupleVariant(..) => Unexpected::TupleVariant,
            Value::Map(_) => Unexpected::Map,
            Value::Struct(_) => Unexpected::NewtypeStruct,
            Value::StructVariant(..) => Unexpected::NewtypeVariant,
        }
    }
}

#[derive(Debug)]
pub struct Error;

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        todo!("{msg}")
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "deseralize error".fmt(f)
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

impl<'de: 'v, 'v> Deserializer<'de> for Value<'v> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }

    deserialize_primitive!(deserialize_bool, Boolean, visit_bool);

    deserialize_primitive!(deserialize_i8, Int8, visit_i8);
    deserialize_primitive!(deserialize_i16, Int16, visit_i16);
    deserialize_primitive!(deserialize_i32, Int32, visit_i32);
    deserialize_primitive!(deserialize_i64, Int64, visit_i64);
    //deserialize_primitive!(deserialize_isize, ISize, visit_isize);

    deserialize_primitive!(deserialize_u8, Uint8, visit_u8);
    deserialize_primitive!(deserialize_u16, Uint16, visit_u16);
    deserialize_primitive!(deserialize_u32, Uint32, visit_u32);
    deserialize_primitive!(deserialize_u64, Uint64, visit_u64);
    //deserialize_primitive!(deserialize_i64, Int64, visit_);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // if let Value::Float32(n) = self {
        //     visitor.visit_f32(n.into())
        // } else {
        //     Err(self.invalid_type(&visitor))
        // }
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // if let Value::Float64(n) = self {
        //     visitor.visit_f64(n.into())
        // } else {
        //     Err(self.invalid_type(&visitor))
        // }
        todo!()
    }

    deserialize_primitive!(deserialize_char, Char, visit_char);

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Str(n) = self {
            visitor.visit_str(n)
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
        if let Value::Bytes(n) = self {
            visitor.visit_bytes(n.as_slice())
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
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

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
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
            let mut deserializer = MapDeserializer::<'v>::new(map);
            visitor.visit_map(&mut deserializer)
        } else {
            Err(self.invalid_type(&visitor))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
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
        _name: &'static str,
        _variants: &'static [&'static str],
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

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct SeqDeserializer<'de> {
    iter: IntoIter<Value<'de>>,
}

impl<'de> SeqDeserializer<'de> {
    fn new(vec: Vec<Value<'de>>) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
        }
    }
}

impl<'de: 'v, 'v> SeqAccess<'de> for SeqDeserializer<'v> {
    type Error = Error;

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

fn visit_array<'de: 'v, 'v, V>(array: Vec<Value<'v>>, visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let mut deserializer = SeqDeserializer::new(array);
    visitor.visit_seq(&mut deserializer)
}

struct EnumDeserializer<'de> {
    variant: String,
    value: Vec<Value<'de>>,
    tree: BTreeMap<Value<'de>, Value<'de>>,
}

impl<'de> EnumDeserializer<'de> {
    fn from_value(variant: String, value: Vec<Value<'de>>) -> Self {
        Self {
            variant,
            value,
            tree: BTreeMap::new(),
        }
    }
    fn from_map(variant: String, tree: BTreeMap<Value<'de>, Value<'de>>) -> Self {
        Self {
            variant,
            value: vec![],
            tree,
        }
    }
}

impl<'de: 'v, 'v> EnumAccess<'de> for EnumDeserializer<'v> {
    type Error = Error;
    type Variant = VariantDeserializer<'v>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantDeserializer<'v>), Self::Error>
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

struct VariantDeserializer<'de> {
    value: Vec<Value<'de>>,
    tree: BTreeMap<Value<'de>, Value<'de>>,
}

impl<'de: 'v, 'v> VariantAccess<'de> for VariantDeserializer<'v> {
    type Error = Error;

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

struct MapDeserializer<'de> {
    iter: <BTreeMap<Value<'de>, Value<'de>> as IntoIterator>::IntoIter,
    value: Option<Value<'de>>,
}

impl<'de> MapDeserializer<'de> {
    fn new(map: BTreeMap<Value<'de>, Value<'de>>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}

impl<'de: 'v, 'v> MapAccess<'de> for MapDeserializer<'v> {
    type Error = Error;

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

#[cfg(test)]
#[test]
fn test_str() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct A<'a> {
        foo: &'a str,
        num: u8,
    }

    let val = A {
        foo: "bar",
        num: 11,
    };

    let foo = "bar";

    let value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(Value::StructKey("foo".into()), Value::Str(foo));
        map.insert(Value::StructKey("num".into()), Value::Uint8(11));
        map
    });

    let new_value = A::deserialize(value).unwrap();

    assert_eq!(new_value.foo, "bar");
    assert_eq!(new_value.num, 11);
}
