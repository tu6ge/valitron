use valitron::{
    available::{Message, Required, StartWith},
    RuleExt, RuleShortcut, Validator, Value,
};

fn main() {
    let _validator = Validator::<MyMessage>::new()
        .rule("name", Required.and(StartWith("foo")).and(Gt10))
        .message([
            ("name.required", MyMessage::NameRequierd),
            ("name.start_with", MyMessage::NameStartWith),
        ]);
}

const GT_10_MESSAGE: &str = "gt10";

#[derive(Clone)]
enum MyMessage {
    NameRequierd,
    NameStartWith,
    Gt10,
    Undefined,
}

impl From<Message> for MyMessage {
    fn from(value: Message) -> Self {
        if value.as_str() == GT_10_MESSAGE {
            Self::Gt10
        } else {
            Self::Undefined
        }
    }
}

#[derive(Clone)]
struct Gt10;

impl RuleShortcut for Gt10 {
    type Message = Message;

    fn name(&self) -> &'static str {
        "gt10"
    }

    fn message(&self) -> Self::Message {
        Message::new(
            valitron::available::MessageKind::Undefined,
            GT_10_MESSAGE.into(),
        )
    }

    fn call(&mut self, data: &mut Value) -> bool {
        data > 10_u8
    }
}
