//! Require string to contain provided parameter, the parameter support `String`, `&str` or `char`,
//! and verified data only support `String` or `&'static str` , other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Contains, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     email: String,
//! }
//!
//! let input = Input {
//!     email: String::from("hi"),
//! };
//! let err = input
//!     .validate(Validator::new().rule("email", Contains('@')))
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("email").unwrap()[0].kind(),
//!     MessageKind::Contains(_)
//! ));
//!
//! let input = Input {
//!     email: String::from("user@foo.com"),
//! };
//! input
//!     .validate(Validator::new().rule("email", Contains('@')))
//!     .unwrap();
//! ```

use std::fmt::{Debug, Display};

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone)]
pub struct Contains<T>(pub T);

impl<T: Debug> Debug for Contains<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Contains").field("0", &self.0).finish()
    }
}

impl<T> Contains<T> {
    fn name_in(&self) -> &'static str {
        "contains"
    }
}

impl<T> Contains<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::new(super::MessageKind::Contains(self.0.to_string()))
    }
}

impl RuleShortcut for Contains<&str> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.contains(self.0),
            _ => false,
        }
    }
}

impl RuleShortcut for Contains<String> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.contains(&self.0),
            _ => false,
        }
    }
}

impl RuleShortcut for Contains<char> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.contains(self.0),
            _ => false,
        }
    }
}
