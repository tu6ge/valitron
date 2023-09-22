use std::{collections::BTreeMap, mem};

use crate::register::{FieldName, FieldNames, Parser};

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Value<'v> {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Uint64(u64),
    Int64(i64),
    // Float32(float::Float32),
    // Float64(float::Float64),
    String(String),
    Str(&'v str),
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
    Option(Box<Option<Value<'v>>>),

    #[doc(hidden)]
    Array(Vec<Value<'v>>),

    #[doc(hidden)]
    Tuple(Vec<Value<'v>>),

    #[doc(hidden)]
    TupleStruct(Vec<Value<'v>>),

    #[doc(hidden)]
    NewtypeStruct(Vec<Value<'v>>),

    #[doc(hidden)]
    Enum(&'static str, Vec<Value<'v>>),
    #[doc(hidden)]
    EnumUnit(&'static str),
    #[doc(hidden)]
    TupleVariant(&'static str, Vec<Value<'v>>),

    #[doc(hidden)]
    Map(BTreeMap<Value<'v>, Value<'v>>),

    #[doc(hidden)]
    StructKey(String),
    /// the BtreeMap key only be StructKey(_)
    #[doc(hidden)]
    Struct(BTreeMap<Value<'v>, Value<'v>>),

    #[doc(hidden)]
    StructVariantKey(String),
    /// the BtreeMap key only be StructVariantKey(_)
    #[doc(hidden)]
    StructVariant(&'static str, BTreeMap<Value<'v>, Value<'v>>),
}

/// contain full [`Value`] and cursor
///
/// [`Value`]: self::Value
pub struct ValueMap<'map> {
    pub(crate) value: Value<'map>,
    pub(crate) index: FieldNames,
}

pub trait FromValue<'map> {
    fn from_value<'a>(value: &'a mut ValueMap<'map>) -> Option<&'a mut Self>;
}

impl<'map> ValueMap<'map> {
    pub(crate) fn new(value: Value<'map>) -> Self {
        Self {
            value,
            index: FieldNames::default(),
        }
    }

    /// change index
    pub fn index(&mut self, index: FieldNames) {
        self.index = index;
    }

    /// Takes the FieldNames out of the ValueMap
    pub fn take_index(&mut self) -> FieldNames {
        let mut x = FieldNames::default();
        mem::swap(&mut self.index, &mut x);
        x
    }

    /// get current field value
    pub fn current(&self) -> Option<&Value<'map>> {
        self.value.get_with_names(&self.index)
    }

    /// get current field mutable value
    pub fn current_mut(&mut self) -> Option<&mut Value<'map>> {
        self.value.get_with_names_mut(&self.index)
    }

    /// get field value by field names
    pub fn get(&self, key: &FieldNames) -> Option<&Value<'map>> {
        self.value.get_with_names(key)
    }

    /// get field mutable value by field names
    pub fn get_mut(&mut self, key: &FieldNames) -> Option<&mut Value<'map>> {
        self.value.get_with_names_mut(key)
    }

    pub(crate) fn value(self) -> Value<'map> {
        self.value
    }
}

impl<'map> Value<'map> {
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
    pub fn get_with_names(&self, names: &FieldNames) -> Option<&Value<'map>> {
        // let mut value = Some(self);
        // let mut parser = Parser::new(names.as_str());
        // loop {
        //     match parser.next_name() {
        //         Ok(Some(name)) => {
        //             value = match value {
        //                 Some(v) => v.get_with_name(&name),
        //                 None => return None,
        //             }
        //         }
        //         Ok(None) => break value,
        //         Err(e) => panic!("{e}"),
        //     }
        // }
        todo!()
    }

    /// get field mutable value by field name
    pub fn get_with_name_mut(&mut self, name: &FieldName) -> Option<&mut Value<'map>> {
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
    pub fn get_with_names_mut(&mut self, names: &FieldNames) -> Option<&mut Value<'map>> {
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
                // | Self::Float32(_)
                // | Self::Float64(_)
                | Self::Unit
                | Self::String(_)
        )
    }

    pub fn f32(&self) -> Option<&f32> {
        // match self {
        //     Value::Float32(float::Float32(f)) => Some(f),
        //     _ => None,
        // }
        todo!()
    }

    pub fn f32_mut(&mut self) -> Option<&mut f32> {
        // match self {
        //     Value::Float32(float::Float32(f)) => Some(f),
        //     _ => None,
        // }
        todo!()
    }

    pub fn f64(&self) -> Option<&f64> {
        // match self {
        //     Value::Float64(float::Float64(f)) => Some(f),
        //     _ => None,
        // }
        todo!()
    }

    pub fn f64_mut(&mut self) -> Option<&mut f64> {
        // match self {
        //     Value::Float64(float::Float64(f)) => Some(f),
        //     _ => None,
        // }
        todo!()
    }
}

impl<'map> FromValue<'map> for ValueMap<'map> {
    fn from_value<'a>(value: &'a mut ValueMap<'map>) -> Option<&'a mut Self> {
        Some(value)
    }
}

impl<'map> FromValue<'map> for Value<'map> {
    fn from_value<'a>(value: &'a mut ValueMap<'map>) -> Option<&'a mut Self> {
        value.current_mut()
    }
}

macro_rules! primitive_impl {
    ($val:ident($ty:ty)) => {
        impl<'map> FromValue<'map> for $ty {
            fn from_value<'a>(value: &'a mut ValueMap<'map>) -> Option<&'a mut Self> {
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

// impl FromValue for f32 {
//     fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
//         if let Some(Value::Float32(float::Float32(n))) = value.current_mut() {
//             Some(n)
//         } else {
//             None
//         }
//     }
// }

// impl FromValue for f64 {
//     fn from_value(value: &mut ValueMap) -> Option<&mut Self> {
//         if let Some(Value::Float64(float::Float64(n))) = value.current_mut() {
//             Some(n)
//         } else {
//             None
//         }
//     }
// }

pub type Bytes = Vec<u8>;

// impl FromValue for Bytes {
//     fn from_value(value: &mut ValueMap) -> Option<&mut Bytes> {
//         if let Some(Value::Bytes(bytes)) = value.current_mut() {
//             Some(bytes)
//         } else {
//             None
//         }
//     }
// }
