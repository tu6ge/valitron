//! Range validate rule
//!
//! # Examples
//! ```
//! # use valitron::{Validator, available::Range};
//! # fn main() {
//! let validator = Validator::new()
//!     .rule("num", Range::new(10..20));
//! # }
//! ```

use std::{marker::PhantomData, ops::RangeBounds};

use crate::{RuleShortcut, Value};

#[derive(Clone)]
pub struct Range<T, Num> {
    value: T,
    _marker: PhantomData<Num>,
}

impl<T, Num> Range<T, Num> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    fn name_in(&self) -> &'static str {
        "range"
    }
    fn message_in(&self) -> &'static str {
        "the value not in the range"
    }
}

macro_rules! impl_range {
    ($val:ident($ty:ty)) => {
        impl<T> RuleShortcut for Range<T, $ty>
        where
            T: RangeBounds<$ty> + Clone + 'static,
        {
            type Message = &'static str;
            fn name(&self) -> &'static str {
                self.name_in()
            }
            fn message(&self) -> Self::Message {
                self.message_in()
            }
            fn call(&mut self, data: &mut Value) -> bool {
                match data {
                    Value::$val(n) => self.value.contains(n),
                    _ => false,
                }
            }
        }
    };
}

impl_range!(Uint8(u8));
impl_range!(Int8(i8));
impl_range!(Uint16(u16));
impl_range!(Int16(i16));
impl_range!(Uint32(u32));
impl_range!(Int32(i32));
impl_range!(Uint64(u64));
impl_range!(Int64(i64));

impl<T> RuleShortcut for Range<T, f32>
where
    T: RangeBounds<f32> + Clone + 'static,
{
    type Message = &'static str;
    fn name(&self) -> &'static str {
        self.name_in()
    }
    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::Float32(f) => self.value.contains(f.as_ref()),
            _ => false,
        }
    }
}

impl<T> RuleShortcut for Range<T, f64>
where
    T: RangeBounds<f64> + Clone + 'static,
{
    type Message = &'static str;
    fn name(&self) -> &'static str {
        self.name_in()
    }
    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::Float64(f) => self.value.contains(f.as_ref()),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{rule::IntoRuleList, RuleExt};

    use super::{super::Required, Range};

    fn register<R: IntoRuleList>(_: R) {}

    #[test]
    fn test_register() {
        register(Required.and(Range::new(1..10)));
    }
}
