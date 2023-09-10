use std::cmp::Ordering;

use super::{
    float::{Float32, Float64},
    Value,
};

macro_rules! primitive_eq {
    ($val:ident($ty:ty)) => {
        impl PartialEq<Value> for $ty {
            fn eq(&self, other: &Value) -> bool {
                if let Value::$val(n) = other {
                    self == n
                } else {
                    false
                }
            }
        }
        impl PartialEq<$ty> for Value {
            fn eq(&self, other: &$ty) -> bool {
                if let Value::$val(n) = self {
                    n == other
                } else {
                    false
                }
            }
        }
    };
}

macro_rules! primitive_ord {
    ($val:ident($ty:ty)) => {
        impl PartialOrd<Value> for $ty {
            fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
                if let Value::$val(n) = other {
                    self.partial_cmp(n)
                } else {
                    None
                }
            }
        }
        impl PartialOrd<$ty> for Value {
            fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                if let Value::$val(n) = self {
                    n.partial_cmp(other)
                } else {
                    None
                }
            }
        }
    };
}

primitive_eq!(UInt8(u8));
primitive_eq!(Int8(i8));
primitive_eq!(UInt16(u16));
primitive_eq!(Int16(i16));
primitive_eq!(UInt32(u32));
primitive_eq!(Int32(i32));
primitive_eq!(UInt64(u64));
primitive_eq!(Int64(i64));
primitive_eq!(Boolean(bool));
primitive_eq!(Char(char));

primitive_ord!(UInt8(u8));
primitive_ord!(Int8(i8));
primitive_ord!(UInt16(u16));
primitive_ord!(Int16(i16));
primitive_ord!(UInt32(u32));
primitive_ord!(Int32(i32));
primitive_ord!(UInt64(u64));
primitive_ord!(Int64(i64));
primitive_ord!(Boolean(bool));
primitive_ord!(Char(char));

impl PartialEq<Value> for f32 {
    fn eq(&self, other: &Value) -> bool {
        if let Value::Float32(Float32(f)) = other {
            self == f
        } else {
            false
        }
    }
}

impl PartialEq<Value> for f64 {
    fn eq(&self, other: &Value) -> bool {
        if let Value::Float64(Float64(f)) = other {
            self == f
        } else {
            false
        }
    }
}

impl PartialEq<f32> for Value {
    fn eq(&self, other: &f32) -> bool {
        if let Value::Float32(Float32(f)) = self {
            f == other
        } else {
            false
        }
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        if let Value::Float64(Float64(f)) = self {
            f == other
        } else {
            false
        }
    }
}