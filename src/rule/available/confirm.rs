//! Usual used by password confirm input

use std::fmt::Display;

use crate::{register::FieldNames, value::ValueMap, RuleShortcut, Value};

use super::{Message, MessageKind};

#[derive(Clone)]
pub struct Confirm<T>(pub T);

impl<T> Confirm<T> {
    fn name_in(&self) -> &'static str {
        "confirm"
    }
}

impl<T> Confirm<T>
where
    T: ToString,
{
    fn get_target_value<'v>(&self, value: &'v ValueMap) -> Option<&'v Value> {
        let target = value.get(&FieldNames::new(self.0.to_string()));
        match target {
            Some(target) if target.is_leaf() => Some(target),
            _ => None,
        }
    }
}

impl<T> Confirm<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::new(
            MessageKind::Confirm,
            format!("this field value must be equal to `{}` field", self.0),
        )
    }
}

impl RuleShortcut for Confirm<String> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

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

    fn name(&self) -> &'static str {
        self.name_in()
    }

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
