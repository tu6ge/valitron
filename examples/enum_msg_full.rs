use async_trait::async_trait;
use serde::Serialize;
use valitron::{
    available::{Message, Required, StartWith, Trim},
    RuleExt, RuleShortcut, Validator, Value,
};

#[derive(Debug, Serialize)]
struct Input {
    name: String,
    num: u8,
}

#[tokio::main]
async fn main() {
    let validator = Validator::new()
        .rule("name", Trim.and(Required).and(StartWith("foo")))
        .map(MyMessage::from)
        .rule("num", Gt10)
        .message([
            ("name.required", MyMessage::NameRequierd),
            ("name.start_with", MyMessage::NameStartWith),
            ("num.gt10", MyMessage::Gt10),
        ]);

    let input = Input {
        name: "bar".into(),
        num: 9,
    };
    let res = validator.validate(&input).await.unwrap_err();

    assert_eq!(res.get("name").unwrap()[0], MyMessage::NameStartWith);
    assert_eq!(res.get("num").unwrap()[0], MyMessage::Gt10);
}

#[derive(Debug, Eq, PartialEq)]
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

#[async_trait]
impl RuleShortcut for Gt10 {
    type Message = MyMessage;

    fn name(&self) -> &'static str {
        "gt10"
    }

    fn message(&self) -> Self::Message {
        MyMessage::Gt10
    }

    async fn call(&mut self, data: &mut Value) -> bool {
        data > 10_u8
    }
}
