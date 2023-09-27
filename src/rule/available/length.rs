//! Length validate rule, support `String`, `Array`, `Vec`, `HashMap`, `BTreeMap`. other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Length, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     title: String,
//!     fruit: Vec<String>,
//! }
//!
//! let input = Input {
//!     title: "hello".into(),
//!     fruit: vec!["foo".into(), "bar".into()],
//! };
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("title", Length(30..40))
//!             .rule("fruit", Length(4..=4)),
//!     )
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("title").unwrap()[0].kind(),
//!     MessageKind::Length
//! ));
//! assert!(matches!(
//!     err.get("fruit").unwrap()[0].kind(),
//!     MessageKind::Length
//! ));
//!
//! input
//!     .validate(
//!         Validator::new()
//!             .rule("title", Length(..10))
//!             .rule("fruit", Length(2..=2)),
//!     )
//!     .unwrap();
//! ```

use std::{fmt::Debug, ops::RangeBounds};

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone)]
pub struct Length<T>(pub T);

impl<T: Debug> Debug for Length<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Length").field("0", &self.0).finish()
    }
}

impl<T> Length<T> {
    fn name_in(&self) -> &'static str {
        "length"
    }

    fn message_in(&self) -> Message {
        Message::new(super::MessageKind::Length)
    }

    pub const fn as_ref(&self) -> Length<&T> {
        let Length(ref t) = self;
        Length(t)
    }

    pub fn as_mut(&mut self) -> Length<&mut T> {
        let Length(ref mut t) = self;
        Length(t)
    }
}

impl<T> RuleShortcut for Length<T>
where
    T: RangeBounds<usize>,
{
    type Message = Message;
    fn name(&self) -> &'static str {
        self.name_in()
    }
    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::String(str) => self.0.contains(&str.len()),
            Value::Array(arr) => self.0.contains(&arr.len()),
            Value::Map(map) => self.0.contains(&map.len()),
            _ => false,
        }
    }
}

// impl RuleShortcut for Length<usize> {
//     type Message = Message;
//     fn name(&self) -> &'static str {
//         self.name_in()
//     }
//     fn message(&self) -> Self::Message {
//         self.message_in()
//     }
//     fn call(&mut self, data: &mut Value) -> bool {
//         match data {
//             Value::String(str) => self.0 == str.len(),
//             Value::Array(arr) => self.0 == arr.len(),
//             Value::Map(map) => self.0 == map.len(),
//             _ => false,
//         }
//     }
// }

impl<T> Length<&T> {
    pub const fn copied(self) -> Length<T>
    where
        T: Copy,
    {
        Length(*self.0)
    }

    pub fn cloned(self) -> Length<T>
    where
        T: Clone,
    {
        Length(self.0.clone())
    }
}

impl<T> Length<&mut T> {
    pub fn copied(self) -> Length<T>
    where
        T: Copy,
    {
        Length(*self.0)
    }

    pub fn cloned(self) -> Length<T>
    where
        T: Clone,
    {
        Length(self.0.clone())
    }
}

impl<T: PartialEq> PartialEq for Length<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
