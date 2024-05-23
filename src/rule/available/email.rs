//! Value must be a valid email address, supported `String`, and other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Email, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     email: String,
//!     password: String,
//! }
//!
//! let input = Input {
//!     email: String::from("user"),
//!     password: String::default(),
//! };
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("email", Email)
//!     )
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("email").unwrap()[0].kind(),
//!     MessageKind::Email
//! ));
//!
//! let input = Input {
//!     email: String::from("user@example.com"),
//!     password: String::from("bar"),
//! };
//! input
//!     .validate(
//!         Validator::new()
//!             .rule("email", Email)
//!     )
//!     .unwrap();
//! ```

use super::Message;
use crate::{rule::CoreRule, Rule, Value};

mod parse;

pub use parse::validate_email;

#[derive(Clone, Copy, Debug)]
pub struct Email;

const NAME: &str = "email";

impl Rule for Email {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Email)
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => validate_email(s),
            _ => false,
        }
    }
}

impl CoreRule<String, ()> for Email {
    type Message = Message;

    const THE_NAME: &'static str = NAME;

    fn call(&mut self, data: &mut String) -> Result<(), Self::Message> {
        if validate_email(data) {
            Ok(())
        } else {
            Err(Message::new(super::MessageKind::Email))
        }
    }
}
