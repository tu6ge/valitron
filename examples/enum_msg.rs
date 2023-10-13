//! usage without "full" feature

use valitron::{RuleShortcut, Validator, Value};

fn main() {
    let validator = Validator::new()
        .rule("num", Gt10)
        .message([("num.gt10", MyMessage::Gt10)]);

    let _validate = validator
        .map(MyMessage2::from)
        .rule("num", Lt20)
        .message([("num.gt20", MyMessage2::Lt20)]);
}

enum MyMessage {
    Gt10,
}

enum MyMessage2 {
    Gt10,
    Lt20,
}

impl From<MyMessage> for MyMessage2 {
    fn from(value: MyMessage) -> Self {
        match value {
            MyMessage::Gt10 => MyMessage2::Gt10,
        }
    }
}

#[derive(Clone)]
struct Gt10;

impl RuleShortcut for Gt10 {
    type Message = MyMessage;
    fn name(&self) -> &'static str {
        "gt10"
    }
    fn message(&self) -> Self::Message {
        MyMessage::Gt10
    }
    fn call(&mut self, data: &mut Value) -> bool {
        data > 10_u8
    }
}

#[derive(Clone)]
struct Lt20;

impl RuleShortcut for Lt20 {
    type Message = MyMessage2;
    fn name(&self) -> &'static str {
        "gt10"
    }
    fn message(&self) -> Self::Message {
        MyMessage2::Lt20
    }
    fn call(&mut self, data: &mut Value) -> bool {
        data < 20_u8
    }
}
