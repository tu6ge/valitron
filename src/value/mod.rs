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
//! let mut value = Value::Uint8(10);
//! assert!(value == 10_u8);
//! assert!(value > 9_u8);
//! assert!(&value == 10_u8);
//! assert!(&value > 9_u8);
//! assert!(&mut value == 10_u8);
//! assert!(&mut value > 9_u8);
//! # }
//! ```

use std::{collections::BTreeMap, mem};

use crate::register::{FieldName, FieldNames, Parser};

mod cmp;
mod float;

/// # serialized resultant
///
/// All rust types will be serialized into this, contains nested structures.
///
/// This is [`Rule`], [`RuleShortcut`] implementation's basis.
///
/// [`Rule`]: crate::rule::Rule
/// [`RuleShortcut`]: crate::rule::RuleShortcut
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
    #[doc(hidden)]
    Option(Box<Option<Value>>),

    #[doc(hidden)]
    Array(Vec<Value>),

    #[doc(hidden)]
    Tuple(Vec<Value>),

    #[doc(hidden)]
    TupleStruct(Vec<Value>),

    #[doc(hidden)]
    NewtypeStruct(Vec<Value>),

    #[doc(hidden)]
    Enum(&'static str, Vec<Value>),
    #[doc(hidden)]
    EnumUnit(&'static str),
    #[doc(hidden)]
    TupleVariant(&'static str, Vec<Value>),

    #[doc(hidden)]
    Map(BTreeMap<Value, Value>),

    #[doc(hidden)]
    StructKey(String),
    /// the BtreeMap key only be StructKey(_)
    #[doc(hidden)]
    Struct(BTreeMap<Value, Value>),

    #[doc(hidden)]
    StructVariantKey(String),
    /// the BtreeMap key only be StructVariantKey(_)
    #[doc(hidden)]
    StructVariant(&'static str, BTreeMap<Value, Value>),
}

/// contain full [`Value`] and cursor
///
/// [`Value`]: self::Value
pub struct ValueMap {
    pub(crate) value: Value,
    pub(crate) index: FieldNames,
}

pub trait FromValue<'a>: Sized {
    fn from_value(value: &'a mut ValueMap) -> Option<Self>;
}

impl ValueMap {
    pub(crate) fn new(value: Value) -> Self {
        Self {
            value,
            index: FieldNames::default(),
        }
    }

    /// change index
    pub fn index(&mut self, index: FieldNames) {
        debug_assert!(
            self.value.get_with_names(&index).is_some(),
            "field `{}` is not exist",
            index.as_str()
        );

        self.index = index;
    }

    /// Takes the FieldNames out of the ValueMap
    pub fn take_index(&mut self) -> FieldNames {
        let mut x = FieldNames::default();
        mem::swap(&mut self.index, &mut x);
        x
    }

    /// get current field value
    pub fn current(&self) -> Option<&Value> {
        self.value.get_with_names(&self.index)
    }

    /// get current field mutable value
    pub fn current_mut(&mut self) -> Option<&mut Value> {
        self.value.get_with_names_mut(&self.index)
    }

    /// get field value by field names
    pub fn get(&self, key: &FieldNames) -> Option<&Value> {
        self.value.get_with_names(key)
    }

    /// get field mutable value by field names
    pub fn get_mut(&mut self, key: &FieldNames) -> Option<&mut Value> {
        self.value.get_with_names_mut(key)
    }

    pub(crate) fn value(self) -> Value {
        self.value
    }
}

impl Value {
    /// get field value by field name
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

    /// get field value by field names
    pub fn get_with_names(&self, names: &FieldNames) -> Option<&Value> {
        let mut value = Some(self);
        let mut parser = Parser::new(names.as_str());
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

    /// get field mutable value by field name
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

    /// get field mutable value by field names
    pub fn get_with_names_mut(&mut self, names: &FieldNames) -> Option<&mut Value> {
        let mut value = Some(self);
        let mut parser = Parser::new(names.as_str());
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

    pub fn f32(&self) -> Option<&f32> {
        match self {
            Value::Float32(float::Float32(f)) => Some(f),
            _ => None,
        }
    }

    pub fn f32_mut(&mut self) -> Option<&mut f32> {
        match self {
            Value::Float32(float::Float32(f)) => Some(f),
            _ => None,
        }
    }

    pub fn f64(&self) -> Option<&f64> {
        match self {
            Value::Float64(float::Float64(f)) => Some(f),
            _ => None,
        }
    }

    pub fn f64_mut(&mut self) -> Option<&mut f64> {
        match self {
            Value::Float64(float::Float64(f)) => Some(f),
            _ => None,
        }
    }
}

impl<'a> FromValue<'a> for &'a mut ValueMap {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        Some(value)
    }
}
impl<'a> FromValue<'a> for &'a ValueMap {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        Some(value)
    }
}

impl<'a> FromValue<'a> for &'a mut Value {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        value.current_mut()
    }
}
impl<'a> FromValue<'a> for &'a Value {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        value.current()
    }
}

macro_rules! primitive_impl {
    ($val:ident($ty:ty)) => {
        impl<'a> FromValue<'a> for &'a mut $ty {
            fn from_value(value: &'a mut ValueMap) -> Option<Self> {
                if let Some(Value::$val(n)) = value.current_mut() {
                    Some(n)
                } else {
                    None
                }
            }
        }

        impl<'a> FromValue<'a> for &'a $ty {
            fn from_value(value: &'a mut ValueMap) -> Option<Self> {
                if let Some(Value::$val(n)) = value.current() {
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
primitive_impl!(String(String));
primitive_impl!(Boolean(bool));
primitive_impl!(Char(char));

impl<'a> FromValue<'a> for &'a mut f32 {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        if let Some(Value::Float32(float::Float32(n))) = value.current_mut() {
            Some(n)
        } else {
            None
        }
    }
}

impl<'a> FromValue<'a> for &'a f32 {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        if let Some(Value::Float32(float::Float32(n))) = value.current() {
            Some(n)
        } else {
            None
        }
    }
}

impl<'a> FromValue<'a> for &'a mut f64 {
    fn from_value(value: &'a mut ValueMap) -> Option<&'a mut f64> {
        if let Some(Value::Float64(float::Float64(n))) = value.current_mut() {
            Some(n)
        } else {
            None
        }
    }
}
impl<'a> FromValue<'a> for &'a f64 {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        if let Some(Value::Float64(float::Float64(n))) = value.current() {
            Some(n)
        } else {
            None
        }
    }
}

pub type Bytes = Vec<u8>;

impl<'a> FromValue<'a> for &'a mut Bytes {
    fn from_value(value: &'a mut ValueMap) -> Option<Self> {
        if let Some(Value::Bytes(bytes)) = value.current_mut() {
            Some(bytes)
        } else {
            None
        }
    }
}
