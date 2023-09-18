//! Require string to start with provided parameters

use std::fmt::Display;

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone, Debug)]
pub struct StartWith<T>(pub T);

impl<T> StartWith<T> {
    fn name_in(&self) -> &'static str {
        "start_with"
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

impl RuleShortcut for StartWith<&str> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

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

impl RuleShortcut for StartWith<char> {
    type Message = Message;

    fn name(&self) -> &'static str {
        self.name_in()
    }

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
