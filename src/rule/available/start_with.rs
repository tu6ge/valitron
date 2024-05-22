//! Require string to start with provided parameter, the parameter support `String`, `&str` or `char`,
//! and verified data only support `String` or `&'static str` , other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{StartWith, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     title: String,
//!     other: &'static str,
//! }
//!
//! let input = Input {
//!     title: String::from("hi"),
//!     other: "foo",
//! };
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("title", StartWith("hello"))
//!             .rule("other", StartWith("bar")),
//!     )
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("title").unwrap()[0].kind(),
//!     MessageKind::StartWith(_)
//! ));
//!
//! let input = Input {
//!     title: String::from("hello world"),
//!     other: "foo",
//! };
//! input
//!     .validate(Validator::new().rule("title", StartWith("hello")))
//!     .unwrap();
//! ```

use std::fmt::{Debug, Display};

use crate::{rule::string::StringRule, Rule, Value};

use super::Message;

#[derive(Clone)]
pub struct StartWith<T>(pub T);

impl<T: Debug> Debug for StartWith<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StartWith").field(&self.0).finish()
    }
}

crate::__impl_copy!(StartWith);

crate::__impl_deref!(StartWith);

const NAME: &str = "start_with";

impl<T> StartWith<T> {
    pub const fn as_ref(&self) -> StartWith<&T> {
        let StartWith(ref t) = self;
        StartWith(t)
    }

    pub fn as_mut(&mut self) -> StartWith<&mut T> {
        let StartWith(ref mut t) = self;
        StartWith(t)
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

impl Rule for StartWith<&str> {
    type Message = Message;

    const NAME: &'static str = NAME;

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

impl Rule for StartWith<String> {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.starts_with(&self.0),
            _ => false,
        }
    }
}

impl Rule for StartWith<char> {
    type Message = Message;

    const NAME: &'static str = NAME;

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

impl StringRule for StartWith<&'static str> {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn call(&mut self, data: &mut String) -> Result<(), Self::Message> {
        if data.starts_with(self.0) {
            Ok(())
        } else {
            Err(self.message_in())
        }
    }
}
impl StringRule for StartWith<String> {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn call(&mut self, data: &mut String) -> Result<(), Self::Message> {
        if data.starts_with(&self.0) {
            Ok(())
        } else {
            Err(self.message_in())
        }
    }
}
impl StringRule for StartWith<char> {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn call(&mut self, data: &mut String) -> Result<(), Self::Message> {
        if data.starts_with(self.0) {
            Ok(())
        } else {
            Err(self.message_in())
        }
    }
}

impl<T> StartWith<&T> {
    pub const fn copied(self) -> StartWith<T>
    where
        T: Copy,
    {
        StartWith(*self.0)
    }

    pub fn cloned(self) -> StartWith<T>
    where
        T: Clone,
    {
        StartWith(self.0.clone())
    }
}

impl<T> StartWith<&mut T> {
    pub fn copied(self) -> StartWith<T>
    where
        T: Copy,
    {
        StartWith(*self.0)
    }

    pub fn cloned(self) -> StartWith<T>
    where
        T: Clone,
    {
        StartWith(self.0.clone())
    }
}

impl<T: PartialEq> PartialEq for StartWith<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
