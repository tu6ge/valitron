//! Modifies a string by leading and trailing whitespace removed

use crate::{RuleShortcut, Value};

use super::Message;

#[derive(Clone)]
pub struct Trim;

impl RuleShortcut for Trim {
    type Message = Message;

    fn call(&mut self, data: &mut crate::Value) -> bool {
        match data {
            Value::String(s) => *s = s.trim().to_string(),
            _ => (),
        }

        true
    }

    fn message(&self) -> Self::Message {
        Message::default()
    }

    fn name(&self) -> &'static str {
        "trim"
    }
}

#[test]
fn test_trim() {
    let mut value = Value::String(" hello ".to_string());

    let _ = Trim.call(&mut value);

    assert!(matches!(value, Value::String(s) if s == "hello".to_string()));
}
