//! Usual used by password confirm input
//!
//! # Example:
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Confirm, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     password: String,
//!     confirm_password: String,
//! }
//!
//! let input = Input {
//!     password: "foo".into(),
//!     confirm_password: "bar".into(),
//! };
//!
//! let err = input
//!     .validate(Validator::new().rule("confirm_password", Confirm("password")))
//!     .unwrap_err();
//! assert!(matches!(
//!     err.get("confirm_password").unwrap()[0].kind(),
//!     MessageKind::Confirm(_)
//! ));
//!
//! let input = Input {
//!     password: "foo".into(),
//!     confirm_password: "foo".into(),
//! };
//!
//! input
//!     .validate(Validator::new().rule("confirm_password", Confirm("password")))
//!     .unwrap();
//! ```

use std::fmt::{Debug, Display};

use crate::{register::FieldNames, value::ValueMap, RuleShortcut, Value};

use super::{Message, MessageKind};

#[derive(Clone)]
pub struct Confirm<T>(pub T);

impl<T: Debug> Debug for Confirm<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Confirm").field(&self.0).finish()
    }
}

impl<T: Copy> Copy for Confirm<T> {}

crate::__impl_deref!(Confirm);

const NAME: &'static str = "confirm";

impl<T> Confirm<T> {
    pub const fn as_ref(&self) -> Confirm<&T> {
        let Confirm(ref t) = self;
        Confirm(t)
    }

    pub fn as_mut(&mut self) -> Confirm<&mut T> {
        let Confirm(ref mut t) = self;
        Confirm(t)
    }
}

impl<T> Confirm<T>
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
        Message::new(MessageKind::Confirm(self.0.to_string()))
    }
}

impl RuleShortcut for Confirm<String> {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        let target = self.get_target_value(value);

        value.current().unwrap() == target.unwrap()
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

impl RuleShortcut for Confirm<&str> {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn message(&self) -> Self::Message {
        self.message_in()
    }

    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        let target = self.get_target_value(value);

        value.current().unwrap() == target.unwrap()
    }

    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

impl<T> Confirm<&T> {
    pub const fn copied(self) -> Confirm<T>
    where
        T: Copy,
    {
        Confirm(*self.0)
    }

    pub fn cloned(self) -> Confirm<T>
    where
        T: Clone,
    {
        Confirm(self.0.clone())
    }
}

impl<T> Confirm<&mut T> {
    pub fn copied(self) -> Confirm<T>
    where
        T: Copy,
    {
        Confirm(*self.0)
    }

    pub fn cloned(self) -> Confirm<T>
    where
        T: Clone,
    {
        Confirm(self.0.clone())
    }
}

impl<T: PartialEq> PartialEq for Confirm<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::Confirm;

    use crate::{register::FieldNames, rule::Rule, ser::to_value, RuleShortcut, Value, ValueMap};

    #[test]
    fn test_confirm() {
        #[derive(Serialize)]
        struct MyType {
            name: String,
            other_name: String,
            age: u8,
        }
        let my_struct = MyType {
            name: "wang".into(),
            other_name: "wanh".into(),
            age: 18,
        };

        let all_value = to_value(my_struct).unwrap();

        let mut confirm = Confirm("name");
        let mut map = ValueMap::new(all_value);
        map.index(FieldNames::new("other_name".to_string()));
        let res = confirm.call_with_relate(&mut map);
        assert!(res == false);
    }
}
