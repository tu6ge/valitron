//! Require string to ends with provided parameter, the parameter support `String`, `&str` or `char`,
//! and verified data only support `String` or `&'static str` , other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{EndsWith, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     email: String,
//! }
//!
//! let input = Input {
//!     email: String::from("hi"),
//! };
//! let err = input
//!     .validate(Validator::new().rule("email", EndsWith("gmail.com")))
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("email").unwrap()[0].kind(),
//!     MessageKind::EndsWith(_)
//! ));
//!
//! let input = Input {
//!     email: String::from("guest@gmail.com"),
//! };
//! input
//!     .validate(Validator::new().rule("email", EndsWith("gmail.com")))
//!     .unwrap();
//! ```

use std::fmt::Display;

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone, Debug)]
pub struct EndsWith<T>(pub T);

impl<T> EndsWith<T> {
    fn name_in(&self) -> &'static str {
        "end_with"
    }
}

impl<T> EndsWith<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::new(super::MessageKind::EndsWith(self.0.to_string()))
    }
}

impl RuleShortcut for EndsWith<&str> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.ends_with(self.0),
            _ => false,
        }
    }
}

impl RuleShortcut for EndsWith<String> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.ends_with(&self.0),
            _ => false,
        }
    }
}

impl RuleShortcut for EndsWith<char> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.ends_with(self.0),
            _ => false,
        }
    }
}
