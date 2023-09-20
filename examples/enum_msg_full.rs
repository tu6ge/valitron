use valitron::{
    available::{Message, Required, StartWith},
    RuleExt, RuleShortcut, Validator, Value,
};

fn main() {
    let _validator = Validator::new()
        .rule("name", Required.and(StartWith("foo")))
        .map(MyMessage::from)
        .rule("num", Gt10)
        .message([
            ("name.required", MyMessage::NameRequierd),
            ("name.start_with", MyMessage::NameStartWith),
            ("num.gt10", MyMessage::Gt10),
        ]);
}

enum MyMessage {
    NameRequierd,
    NameStartWith,
    Gt10,
    NotReset,
}

impl From<Message> for MyMessage {
    fn from(_value: Message) -> Self {
        Self::NotReset
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
