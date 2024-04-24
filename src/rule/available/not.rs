//! Reverse existing rules
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Contains, MessageKind, Not}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     email: String,
//! }
//!
//! let input = Input {
//!     email: String::from("hi"),
//! };
//! input
//!     .validate(Validator::new().rule("email", Not(Contains('@'))))
//!     .unwrap();
//!
//!
//!
//! let input = Input {
//!     email: String::from("user@foo.com"),
//! };
//! let err = input
//!     .validate(Validator::new().rule("email", Not(Contains('@'))))
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("email").unwrap()[0].kind(),
//!     MessageKind::Contains(_)
//! ));
//! ```

use crate::{RuleShortcut, Value};

#[derive(Clone)]
pub struct Not<T>(pub T);

impl<T: RuleShortcut> RuleShortcut for Not<T> {
    type Message = T::Message;

    const NAME: &'static str = T::NAME;

    fn message(&self) -> Self::Message {
        self.0.message()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        !self.0.call(value)
    }
}
