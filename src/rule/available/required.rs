//! Value can not be empty
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Required, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     username: String,
//!     password: String,
//! }
//!
//! let input = Input {
//!     username: String::default(),
//!     password: String::default(),
//! };
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("username", Required)
//!             .rule("password", Required),
//!     )
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("username").unwrap()[0].kind(),
//!     MessageKind::Required
//! ));
//! assert!(matches!(
//!     err.get("password").unwrap()[0].kind(),
//!     MessageKind::Required
//! ));
//!
//! let input = Input {
//!     username: String::from("foo"),
//!     password: String::from("bar"),
//! };
//! input
//!     .validate(
//!         Validator::new()
//!             .rule("username", Required)
//!             .rule("password", Required),
//!     )
//!     .unwrap();
//! ```

use super::Message;
use crate::{RuleShortcut, Value};

#[derive(Clone, Debug)]
pub struct Required;

impl RuleShortcut for Required {
    type Message = Message;

    fn name(&self) -> &'static str {
        "required"
    }

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Required)
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::Uint8(_)
            | Value::Uint16(_)
            | Value::Uint32(_)
            | Value::Uint64(_)
            | Value::Int8(_)
            | Value::Int16(_)
            | Value::Int32(_)
            | Value::Int64(_) => true,
            Value::String(s) => !s.is_empty(),
            _ => unreachable!("invalid Value variant"),
        }
    }
}
