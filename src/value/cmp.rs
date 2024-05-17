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
                    unreachable!("type mismatch")
                }
            }
        }
        impl PartialEq<$ty> for Value {
            fn eq(&self, other: &$ty) -> bool {
                if let Value::$val(n) = self {
                    n == other
                } else {
                    unreachable!("type mismatch")
                }
            }
        }

        impl PartialEq<&Value> for $ty {
            fn eq(&self, other: &&Value) -> bool {
                if let Value::$val(n) = other {
                    self == n
                } else {
                    unreachable!("type mismatch")
                }
            }
        }
        impl PartialEq<$ty> for &Value {
            fn eq(&self, other: &$ty) -> bool {
                if let Value::$val(n) = self {
                    n == other
                } else {
                    unreachable!("type mismatch")
                }
            }
        }

        impl PartialEq<&mut Value> for $ty {
            fn eq(&self, other: &&mut Value) -> bool {
                if let Value::$val(n) = other {
                    self == n
                } else {
                    unreachable!("type mismatch")
                }
            }
        }
        impl PartialEq<$ty> for &mut Value {
            fn eq(&self, other: &$ty) -> bool {
                if let Value::$val(n) = self {
                    n == other
                } else {
                    unreachable!("type mismatch")
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

        impl PartialOrd<&Value> for $ty {
            fn partial_cmp(&self, other: &&Value) -> Option<Ordering> {
                if let Value::$val(n) = other {
                    self.partial_cmp(n)
                } else {
                    None
                }
            }
        }
        impl PartialOrd<$ty> for &Value {
            fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                if let Value::$val(n) = self {
                    n.partial_cmp(other)
                } else {
                    None
                }
            }
        }

        impl PartialOrd<&mut Value> for $ty {
            fn partial_cmp(&self, other: &&mut Value) -> Option<Ordering> {
                if let Value::$val(n) = other {
                    self.partial_cmp(n)
                } else {
                    None
                }
            }
        }
        impl PartialOrd<$ty> for &mut Value {
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

primitive_eq!(Uint8(u8));
primitive_eq!(Int8(i8));
primitive_eq!(Uint16(u16));
primitive_eq!(Int16(i16));
primitive_eq!(Uint32(u32));
primitive_eq!(Int32(i32));
primitive_eq!(Uint64(u64));
primitive_eq!(Int64(i64));
primitive_eq!(String(String));
primitive_eq!(Boolean(bool));
primitive_eq!(Char(char));

primitive_ord!(Uint8(u8));
primitive_ord!(Int8(i8));
primitive_ord!(Uint16(u16));
primitive_ord!(Int16(i16));
primitive_ord!(Uint32(u32));
primitive_ord!(Int32(i32));
primitive_ord!(Uint64(u64));
primitive_ord!(Int64(i64));
primitive_ord!(String(String));
primitive_ord!(Boolean(bool));
primitive_ord!(Char(char));

impl PartialEq<Value> for f32 {
    fn eq(&self, other: &Value) -> bool {
        if let Value::Float32(Float32(f)) = other {
            if self.is_finite() && f.is_finite() {
                self == f
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<f32> for Value {
    fn eq(&self, other: &f32) -> bool {
        if let Value::Float32(Float32(f)) = self {
            if f.is_finite() && other.is_finite() {
                f == other
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<&Value> for f32 {
    fn eq(&self, other: &&Value) -> bool {
        if let Value::Float32(Float32(f)) = other {
            if self.is_finite() && f.is_finite() {
                self == f
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<f32> for &Value {
    fn eq(&self, other: &f32) -> bool {
        if let Value::Float32(Float32(f)) = self {
            if f.is_finite() && other.is_finite() {
                f == other
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}
impl PartialEq<&mut Value> for f32 {
    fn eq(&self, other: &&mut Value) -> bool {
        if let Value::Float32(Float32(f)) = other {
            if self.is_finite() && f.is_finite() {
                self == f
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<f32> for &mut Value {
    fn eq(&self, other: &f32) -> bool {
        if let Value::Float32(Float32(f)) = self {
            if f.is_finite() && other.is_finite() {
                f == other
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<Value> for f64 {
    fn eq(&self, other: &Value) -> bool {
        if let Value::Float64(Float64(f)) = other {
            if self.is_finite() && f.is_finite() {
                self == f
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        if let Value::Float64(Float64(f)) = self {
            if f.is_finite() && other.is_finite() {
                f == other
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<&Value> for f64 {
    fn eq(&self, other: &&Value) -> bool {
        if let Value::Float64(Float64(f)) = other {
            if self.is_finite() && f.is_finite() {
                self == f
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<f64> for &Value {
    fn eq(&self, other: &f64) -> bool {
        if let Value::Float64(Float64(f)) = self {
            if f.is_finite() && other.is_finite() {
                f == other
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<&mut Value> for f64 {
    fn eq(&self, other: &&mut Value) -> bool {
        if let Value::Float64(Float64(f)) = other {
            if self.is_finite() && f.is_finite() {
                self == f
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

impl PartialEq<f64> for &mut Value {
    fn eq(&self, other: &f64) -> bool {
        if let Value::Float64(Float64(f)) = self {
            if f.is_finite() && other.is_finite() {
                f == other
            } else {
                false
            }
        } else {
            unreachable!("type mismatch")
        }
    }
}

#[test]
fn all() {
    let mut value = Value::Uint8(10);

    assert!(value == 10_u8);
    assert!(value > 9_u8);
    assert!(&value == 10_u8);
    assert!(&value > 9_u8);
    assert!(&mut value == 10_u8);
    assert!(&mut value > 9_u8);

    let value = Value::Float32(Float32(1.1));
    let f = 1.1_f32;
    assert!(value == f);

    let value_nan = Value::Float32(Float32(f32::NAN));
    let f_nan = f32::NAN;
    assert!(value_nan != f_nan);
}

#[test]
#[should_panic]
fn type_mismatch() {
    let value = Value::Uint8(10);
    assert!(value == 10_i8);
}
