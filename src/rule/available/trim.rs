//! Modifies a string by leading and trailing whitespace removed,
//! and this alway return true
//!
//! # Examples
//! ```
//! # use serde::{Deserialize, Serialize};
//! # use valitron::{available::{Trim, MessageKind}, Validatable, Validator};
//! #[derive(Deserialize, Serialize, Debug)]
//! struct Input {
//!     title: String,
//! }
//!
//! let input = Input {
//!     title: String::from(" hi "),
//! };
//! let new_input = input
//!     .validate_mut(Validator::new().rule("title", Trim))
//!     .unwrap();
//!
//! assert_eq!(new_input.title, "hi");
//! ```

use crate::{rule::string::StringRule, Rule, Value};

use super::Message;

#[derive(Clone, Copy)]
pub struct Trim;

const NAME: &str = "trim";

impl Rule for Trim {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn call(&mut self, data: &mut crate::Value) -> bool {
        if let Value::String(s) = data {
            *s = s.trim().to_string()
        }

        true
    }

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Trim)
    }
}

impl StringRule for Trim {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn call(&mut self, data: &mut String) -> bool {
        *data = data.trim().to_string();

        true
    }

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Trim)
    }
}

#[test]
fn test_trim() {
    let mut value = Value::String(" hello ".to_string());

    let mut trim = Trim {};
    let _ = Rule::call(&mut trim, &mut value);

    assert!(matches!(value, Value::String(s) if s == "hello".to_string()));
}
