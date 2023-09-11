//! # define `Value`, `ValueMap` types
//! input data will be converted Value with Serialization,
//! and Value will be converted new output data with Deserialization
//!
//! In any rule, you should be comparing it with primitive type
//!
//! ## cmp
//! `Value` comparing and ordering with primitive type(`u8`,`u16`,`u32`,`u64`,`i8`,`i16`,`i32`,`i64`,`f32`,`f64`,`str`,`bool`,`String`)
//!
//! Example:
//! ```
//! # use valitron::Value;
//! # fn main() {
//! assert!(Value::Uint8(9) == 9_u8);
//! assert!(10_u8 == Value::Uint8(10));
//! assert_eq!(Value::Uint8(10) > 9_u8, true);
//! assert_eq!(9_u8 < Value::Uint8(10), true);
//! # }
//! ```

use std::collections::BTreeMap;

use crate::register::{FieldName, FieldNames, Parser};

mod cmp;
mod float;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Value {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Uint64(u64),
    Int64(i64),
    Float32(float::Float32),
    Float64(float::Float64),
    String(String),
    Unit,
    Boolean(bool),
    Char(char),
    Bytes(Vec<u8>),

    // fn unimplemented
    // i128 u128 unimplemented
    // ISize(isize), unimplemented
    // USize(usize), unimplemented
    // pointer, Raw pointer unimplemented
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

    pub fn get(&self, key: &FieldNames) -> Option<&Value> {
        self.value.get_with_names(key)
    }
    pub fn get_mut(&mut self, key: &FieldNames) -> Option<&mut Value> {
        self.value.get_with_names_mut(key)
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
                Err(e) => panic!("{e}"),
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
                Err(e) => panic!("{e}"),
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        matches!(
            self,
            Self::Uint8(_)
                | Self::Uint16(_)
                | Self::Uint32(_)
                | Self::Uint64(_)
                | Self::Int8(_)
                | Self::Int16(_)
                | Self::Int32(_)
                | Self::Int64(_)
                | Self::Boolean(_)
                | Self::Char(_)
                | Self::Float32(_)
                | Self::Float64(_)
                | Self::Unit
                | Self::String(_)
        )
    }
}

impl FromValue for ValueMap {
    fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
        Some(value)
    }
}

impl FromValue for Value {
    fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
        value.current_mut()
    }
}

macro_rules! primitive_impl {
    ($val:ident($ty:ty)) => {
        impl FromValue for $ty {
            fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
                if let Some(Value::$val(n)) = value.current_mut() {
                    Some(n)
                } else {
                    None
                }
            }
        }
    };
}

primitive_impl!(Uint8(u8));
primitive_impl!(Int8(i8));
primitive_impl!(Uint16(u16));
primitive_impl!(Int16(i16));
primitive_impl!(Uint32(u32));
primitive_impl!(Int32(i32));
primitive_impl!(Uint64(u64));
primitive_impl!(Int64(i64));
primitive_impl!(Boolean(bool));
primitive_impl!(Char(char));

impl FromValue for f32 {
    fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
        if let Some(Value::Float32(float::Float32(n))) = value.current_mut() {
            Some(n)
        } else {
            None
        }
    }
}

impl FromValue for f64 {
    fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
        if let Some(Value::Float64(float::Float64(n))) = value.current_mut() {
            Some(n)
        } else {
            None
        }
    }
}

pub type Bytes = Vec<u8>;

impl FromValue for Bytes {
    fn from_value(value: &mut ValueMap) -> Option<&mut Bytes> {
        if let Some(Value::Bytes(bytes)) = value.current_mut() {
            Some(bytes)
        } else {
            None
        }
    }
}
