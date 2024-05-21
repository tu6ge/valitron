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
//! let mut email_rule = Contains('@');
//!
//! let input = Input {
//!     email: String::from("user@foo.com"),
//! };
//! input
//!     .validate(Validator::new().rule("email", email_rule))
//!     .unwrap();
//!
//! *email_rule = 'A';
//!
//! let input = Input {
//!     email: String::from("user@foo.com"),
//! };
//! input
//!     .validate(Validator::new().rule("email", email_rule))
//!     .unwrap_err();
//! ```

use std::fmt::{Debug, Display};

use crate::{Rule, Value};

use super::Message;

#[derive(Clone)]
pub struct Contains<T>(pub T);

impl<T: Debug> Debug for Contains<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Contains").field(&self.0).finish()
    }
}

crate::__impl_copy!(Contains);

crate::__impl_deref!(Contains);

const NAME: &str = "contains";

impl<T> Contains<T> {
    pub const fn as_ref(&self) -> Contains<&T> {
        let Contains(ref t) = self;
        Contains(t)
    }

    pub fn as_mut(&mut self) -> Contains<&mut T> {
        let Contains(ref mut t) = self;
        Contains(t)
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

impl Rule for Contains<&str> {
    type Message = Message;

    const NAME: &'static str = NAME;

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

impl Rule for Contains<String> {
    type Message = Message;

    const NAME: &'static str = NAME;

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

impl Rule for Contains<char> {
    type Message = Message;

    const NAME: &'static str = NAME;

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

impl<T> Contains<&T> {
    pub const fn copied(self) -> Contains<T>
    where
        T: Copy,
    {
        Contains(*self.0)
    }

    pub fn cloned(self) -> Contains<T>
    where
        T: Clone,
    {
        Contains(self.0.clone())
    }
}

impl<T> Contains<&mut T> {
    pub fn copied(self) -> Contains<T>
    where
        T: Copy,
    {
        Contains(*self.0)
    }

    pub fn cloned(self) -> Contains<T>
    where
        T: Clone,
    {
        Contains(self.0.clone())
    }
}

impl<T: PartialEq> PartialEq for Contains<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
