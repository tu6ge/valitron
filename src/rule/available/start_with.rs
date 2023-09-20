//! Require string to start with provided parameters
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{StartWith, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     title: String,
//! }
//!
//! let input = Input {
//!     title: String::from("hi"),
//! };
//! let err = input
//!     .validate(Validator::new().rule("title", StartWith("hello")))
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("title").unwrap()[0].kind(),
//!     MessageKind::StartWith(_)
//! ));
//!
//! let input = Input {
//!     title: String::from("hello world"),
//! };
//! input
//!     .validate(Validator::new().rule("title", StartWith("hello")))
//!     .unwrap();
//! ```

use std::fmt::Display;

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone, Debug)]
pub struct StartWith<T>(pub T);

impl<T> StartWith<T> {
    fn name_in(&self) -> &'static str {
        "start_with"
    }
}

impl<T> StartWith<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::new(super::MessageKind::StartWith(self.0.to_string()))
    }
}

impl RuleShortcut for StartWith<&str> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.starts_with(self.0),
            _ => false,
        }
    }
}

impl RuleShortcut for StartWith<char> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.starts_with(self.0),
            _ => false,
        }
    }
}
