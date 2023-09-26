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
//! let input = Input { title: "hello".into(), fruit: vec!["foo".into(), "bar".into()]};
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("title", Length(30..40))
//!             .rule("fruit", Length(4..=4))
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
//!             .rule("fruit", Length(2..=2))
//!     )
//!     .unwrap();
//! ```

use std::ops::RangeBounds;

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone)]
pub struct Length<T>(pub T);

impl<T> Length<T> {
    fn name_in(&self) -> &'static str {
        "length"
    }

    fn message_in(&self) -> Message {
        Message::new(super::MessageKind::Length)
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
