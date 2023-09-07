use std::collections::BTreeMap;

use crate::register::{FieldName, FieldNames, Parser};

mod float;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Value {
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    Float32(float::Float32),
    Float64(float::Float64),
    String(String),
    Unit,
    // Boolean(bool),
    // Char(char),
    Option(Box<Option<Value>>),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    TupleStruct(Vec<Value>),
    NewtypeStruct(Vec<Value>),

    Enum(&'static str, Vec<Value>),
    EnumUnit(&'static str),
    TupleVariant(&'static str, Vec<Value>),

    Map(BTreeMap<Value, Value>),

    StructKey(String),
    /// the BtreeMap key only be StructKey(_)
    Struct(BTreeMap<Value, Value>),

    StructVariantKey(String),
    /// the BtreeMap key only be StructVariantKey(_)
    StructVariant(&'static str, BTreeMap<Value, Value>),
}

pub struct ValueMap {
    value: Value,
    index: FieldNames,
}

pub trait FromValue {
    fn from_value(value: &mut ValueMap) -> Option<&mut Self>;
}

impl ValueMap {
    pub(crate) fn new(value: Value) -> Self {
        Self {
            value,
            index: FieldNames::default(),
        }
    }
    pub fn index(&mut self, index: FieldNames) {
        self.index = index;
    }
    pub fn current(&self) -> Option<&Value> {
        self.value.get_with_names(&self.index)
    }
    pub fn current_mut(&mut self) -> Option<&mut Value> {
        self.value.get_with_names_mut(&self.index)
    }
    pub fn get(&self, key: &FieldName) -> Option<&Value> {
        self.value.get_with_name(key)
    }
    pub(crate) fn value(self) -> Value {
        self.value
    }
}

impl Value {
    pub fn get_with_name(&self, name: &FieldName) -> Option<&Value> {
        match (name, self) {
            (FieldName::Array(i), Value::Array(vec)) => vec.get(*i),
            (FieldName::Tuple(i), Value::Tuple(vec))
            | (FieldName::Tuple(i), Value::TupleStruct(vec))
            | (FieldName::Tuple(i), Value::NewtypeStruct(vec))
            | (FieldName::Tuple(i), Value::Enum(_, vec))
            | (FieldName::Tuple(i), Value::TupleVariant(_, vec)) => vec.get(*i as usize),
            (FieldName::Literal(str), Value::Struct(btree)) => {
                btree.get(&Value::StructKey(str.to_string()))
            }
            (FieldName::StructVariant(str), Value::StructVariant(_, btree)) => {
                btree.get(&Value::StructVariantKey(str.to_string()))
            }
            _ => None,
        }
    }
    pub fn get_with_names(&self, names: &FieldNames) -> Option<&Value> {
        let mut value = Some(self);
        let mut parser = Parser::new(names.string());
        loop {
            match parser.next_name() {
                Ok(Some(name)) => {
                    value = match value {
                        Some(v) => v.get_with_name(&name),
                        None => return None,
                    }
                }
                Ok(None) => break value,
                Err(_) => return None,
            }
        }
    }
    pub fn get_with_name_mut(&mut self, name: &FieldName) -> Option<&mut Value> {
        match (name, self) {
            (FieldName::Array(i), Value::Array(vec)) => vec.get_mut(*i),
            (FieldName::Tuple(i), Value::Tuple(vec))
            | (FieldName::Tuple(i), Value::TupleStruct(vec))
            | (FieldName::Tuple(i), Value::NewtypeStruct(vec))
            | (FieldName::Tuple(i), Value::Enum(_, vec))
            | (FieldName::Tuple(i), Value::TupleVariant(_, vec)) => vec.get_mut(*i as usize),
            (FieldName::Literal(str), Value::Struct(btree)) => {
                btree.get_mut(&Value::StructKey(str.to_string()))
            }
            (FieldName::StructVariant(str), Value::StructVariant(_, btree)) => {
                btree.get_mut(&Value::StructVariantKey(str.to_string()))
            }
            _ => None,
        }
    }
    pub fn get_with_names_mut(&mut self, names: &FieldNames) -> Option<&mut Value> {
        let mut value = Some(self);
        let mut parser = Parser::new(names.string());
        loop {
            match parser.next_name() {
                Ok(Some(name)) => {
                    value = match value {
                        Some(v) => v.get_with_name_mut(&name),
                        None => break None,
                    }
                }
                Ok(None) => break value,
                Err(_) => break None,
            }
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        if let Self::Struct(map) = self {
            map.get(&Value::StructKey(key.to_string()))
        } else {
            None
        }
    }
    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        if let Self::Struct(map) = self {
            map.get_mut(&Value::StructKey(key.to_string()))
        } else {
            None
        }
    }
    pub(crate) fn get_clone(&self, key: &str) -> Option<Value> {
        self.get(key).cloned()
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::UInt8(_)
            | Self::UInt16(_)
            | Self::UInt32(_)
            | Self::UInt64(_)
            | Self::Int8(_)
            | Self::Int16(_)
            | Self::Int32(_)
            | Self::Int64(_) => true,
            Self::Float32(_) | Self::Float64(_) => true,
            Self::Unit => true,
            Self::String(_) => true,
            _ => false,
        }
    }
}

impl FromValue for ValueMap {
    fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
        Some(value)
    }
}

impl FromValue for i8 {
    fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
        if let Some(Value::Int8(n)) = value.current_mut() {
            Some(n)
        } else {
            None
        }
    }
}
