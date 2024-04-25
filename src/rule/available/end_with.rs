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

use std::fmt::{Debug, Display};

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone)]
pub struct EndsWith<T>(pub T);

impl<T: Debug> Debug for EndsWith<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EndsWith").field(&self.0).finish()
    }
}

crate::__impl_copy!(EndsWith);

crate::__impl_deref!(EndsWith);

const NAME: &'static str = "end_with";

impl<T> EndsWith<T> {
    pub const fn as_ref(&self) -> EndsWith<&T> {
        let EndsWith(ref t) = self;
        EndsWith(t)
    }

    pub fn as_mut(&mut self) -> EndsWith<&mut T> {
        let EndsWith(ref mut t) = self;
        EndsWith(t)
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

    const NAME: &'static str = NAME;

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

    const NAME: &'static str = NAME;

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

    const NAME: &'static str = NAME;

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

impl<T> EndsWith<&T> {
    pub const fn copied(self) -> EndsWith<T>
    where
        T: Copy,
    {
        EndsWith(*self.0)
    }

    pub fn cloned(self) -> EndsWith<T>
    where
        T: Clone,
    {
        EndsWith(self.0.clone())
    }
}

impl<T> EndsWith<&mut T> {
    pub fn copied(self) -> EndsWith<T>
    where
        T: Copy,
    {
        EndsWith(*self.0)
    }

    pub fn cloned(self) -> EndsWith<T>
    where
        T: Clone,
    {
        EndsWith(self.0.clone())
    }
}

impl<T: PartialEq> PartialEq for EndsWith<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
