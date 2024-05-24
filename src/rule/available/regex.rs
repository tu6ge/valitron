//! validater value by regex, supported `String`, other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Regex, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     title: String,
//! }
//!
//! let input = Input {
//!     title: String::from("ac"),
//! };
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("title", Regex::new(r"...")),
//!     )
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("title").unwrap()[0].kind(),
//!     MessageKind::Regex
//! ));
//!
//! let input = Input {
//!     title: String::from("abc"),
//! };
//! input
//!     .validate(
//!         Validator::new()
//!             .rule("title", Regex::new(r"...")),
//!     )
//!     .unwrap();
//! ```

use crate::{rule::string::StringRule, Rule};

use super::Message;

#[derive(Debug, Clone)]
pub struct Regex<'a>(&'a str);

impl<'a> Regex<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Self(pattern)
    }
}

impl<'a> Rule for Regex<'a> {
    type Message = Message;

    const NAME: &'static str = "regex";

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Regex)
    }

    fn call(&mut self, data: &mut crate::Value) -> bool {
        match data {
            crate::Value::String(s) => {
                let reg = regex::Regex::new(self.0)
                    .unwrap_or_else(|_| panic!("regex \"{}\" have syntax error", self.0));
                reg.is_match(s)
            }
            _ => false,
        }
    }
}

impl<'a> StringRule for Regex<'a> {
    type Message = Message;

    const NAME: &'static str = "regex";

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Regex)
    }

    fn call(&mut self, data: &mut String) -> bool {
        let reg = regex::Regex::new(self.0)
            .unwrap_or_else(|_| panic!("regex \"{}\" have syntax error", self.0));
        reg.is_match(data)
    }
}
