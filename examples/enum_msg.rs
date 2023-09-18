//! usage without "full" feature

use valitron::{RuleExt, RuleShortcut, Validator, Value};

fn main() {
    let _validator = Validator::<MyMessage>::new()
        .rule("num", Gt10.and(Lt20))
        .message([("num.gt10", MyMessage::Gt10), ("num.lt20", MyMessage::Lt20)]);
}

#[derive(Clone)]
enum MyMessage {
    Gt10,
    Lt20,
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
    type Message = MyMessage;
    fn name(&self) -> &'static str {
        "gt10"
    }
    fn message(&self) -> Self::Message {
        MyMessage::Lt20
    }
    fn call(&mut self, data: &mut Value) -> bool {
        data < 20_u8
    }
}
