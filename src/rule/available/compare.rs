//! compare number fields
//!
//! # Example:
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Gt, Lt, MessageKind}, Validatable, Validator};
//! #[derive(Serialize)]
//! struct Input {
//!     min: u8,
//!     max: u8,
//! }
//!
//! let input = Input {
//!     min: 10,
//!     max: 20,
//! };
//!
//! Validator::new().rule("max", Gt("min"))
//!     .validate(&input)
//!     .unwrap();
//! 
//! Validator::new().rule("max", Lt(30_u8))
//!     .validate(&input)
//!     .unwrap();
//! ```

use std::fmt::Display;

use crate::{register::FieldNames, RuleShortcut, Value, ValueMap};

use super::{Message, MessageKind};

#[derive(Clone)]
pub struct Lt<T>(pub T);
#[derive(Clone)]
pub struct Elt<T>(pub T);

#[derive(Clone)]
pub struct Gt<T>(pub T);
#[derive(Clone)]
pub struct Egt<T>(pub T);

macro_rules! impl_compare {
    ($type:ty, $label:literal) => {
        impl<T> $type
        where
            T: Display,
        {
            fn get_target_value<'v>(&self, value: &'v ValueMap) -> Option<&'v Value> {
                let target = value.get(&FieldNames::new(self.0.to_string()));
                match target {
                    Some(target) if target.is_leaf() => Some(target),
                    _ => None,
                }
            }
            fn message_in(&self) -> Message {
                // greater
                Message::new(MessageKind::Compare($label.into(), self.0.to_string()))
            }
        }
    };
}

impl_compare!(Lt<T>, "less");
impl_compare!(Elt<T>, "less and equal");
impl_compare!(Gt<T>, "greater");
impl_compare!(Egt<T>, "greater and equal");

impl RuleShortcut for Lt<&str> {
    type Message = Message;

    const NAME: &'static str = "lt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        let target = self.get_target_value(value);

        value.current().unwrap() < target.unwrap()
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

impl RuleShortcut for Elt<&str> {
    type Message = Message;

    const NAME: &'static str = "elt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        let target = self.get_target_value(value);

        value.current().unwrap() <= target.unwrap()
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}
impl RuleShortcut for Gt<&str> {
    type Message = Message;

    const NAME: &'static str = "gt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        let target = self.get_target_value(value);

        value.current().unwrap() > target.unwrap()
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}
impl RuleShortcut for Egt<&str> {
    type Message = Message;

    const NAME: &'static str = "egt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        let target = self.get_target_value(value);

        value.current().unwrap() >= target.unwrap()
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

impl RuleShortcut for Lt<u8> {
    type Message = Message;

    const NAME: &'static str = "lt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        value.current().unwrap() < self.0
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

impl RuleShortcut for Elt<u8> {
    type Message = Message;

    const NAME: &'static str = "elt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        value.current().unwrap() <= self.0
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

impl RuleShortcut for Gt<u8> {
    type Message = Message;

    const NAME: &'static str = "gt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        value.current().unwrap() > self.0
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

impl RuleShortcut for Egt<u8> {
    type Message = Message;

    const NAME: &'static str = "egt";

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        value.current().unwrap() >= self.0
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}
