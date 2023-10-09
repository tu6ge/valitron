//! Range validate rule, support `u8`, `u16`, `u32`, `u64`, `i8`,
//! `i16`, `i32`, `i64`, `f32`, `f64` and char. other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Range, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     num: u8,
//! }
//!
//! let input = Input { num: 9 };
//! let err = input
//!     .validate(Validator::new().rule("num", Range::new(10_u8..20)))
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("num").unwrap()[0].kind(),
//!     MessageKind::Range
//! ));
//!
//! let input = Input { num: 15 };
//! input
//!     .validate(Validator::new().rule("num", Range::new(10_u8..20)))
//!     .unwrap();
//! ```

use std::{fmt::Debug, marker::PhantomData, ops::RangeBounds};

use async_trait::async_trait;

use super::Message;
use crate::{RuleShortcut, Value};

#[derive(Clone)]
pub struct Range<T, Num> {
    value: T,
    _marker: PhantomData<Num>,
}

impl<T: Debug, Num> Debug for Range<T, Num> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Range").field("value", &self.value).finish()
    }
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

    fn message_in(&self) -> Message {
        Message::new(super::MessageKind::Range)
    }
}

macro_rules! impl_range {
    ($val:ident($ty:ty)) => {
        #[async_trait]
        impl<T> RuleShortcut for Range<T, $ty>
        where
            T: RangeBounds<$ty> + Send,
        {
            type Message = Message;
            fn name(&self) -> &'static str {
                self.name_in()
            }
            fn message(&self) -> Self::Message {
                self.message_in()
            }
            async fn call(&mut self, data: &mut Value) -> bool {
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
impl_range!(Char(char));

#[async_trait]
impl<T> RuleShortcut for Range<T, f32>
where
    T: RangeBounds<f32> + Clone + 'static + Send,
{
    type Message = Message;
    fn name(&self) -> &'static str {
        self.name_in()
    }
    fn message(&self) -> Self::Message {
        self.message_in()
    }
    async fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::Float32(f) => self.value.contains(f.as_ref()),
            _ => false,
        }
    }
}

#[async_trait]
impl<T> RuleShortcut for Range<T, f64>
where
    T: RangeBounds<f64> + Clone + 'static + Send,
{
    type Message = Message;
    fn name(&self) -> &'static str {
        self.name_in()
    }
    fn message(&self) -> Self::Message {
        self.message_in()
    }
    async fn call(&mut self, data: &mut Value) -> bool {
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

    fn register<R: IntoRuleList<M>, M>(_: R) {}

    #[test]
    fn test_register() {
        register(Required.and(Range::new(1..10)));
    }
}
