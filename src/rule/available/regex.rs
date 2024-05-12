use crate::RuleShortcut;

use super::Message;

#[derive(Debug, Clone)]
pub struct Regex<'a>(&'a str);

impl<'a> Regex<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Self(pattern)
    }
}

impl<'a> RuleShortcut for Regex<'a> {
    type Message = Message;

    const NAME: &'static str = "regex";

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Regex)
    }

    fn call(&mut self, data: &mut crate::Value) -> bool {
        match data {
            crate::Value::String(s) => {
                let reg = regex::Regex::new(self.0)
                    .expect(&format!("regex \"{}\" have syntax error", self.0));
                reg.is_match(s)
            }
            _ => false,
        }
    }
}
