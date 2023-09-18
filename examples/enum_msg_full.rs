use valitron::{
    available::{Message, Required, StartWith},
    RuleExt, Validator,
};

fn main() {
    let _validator = Validator::<MyMessage>::new()
        .rule("name", Required.and(StartWith("foo")))
        .message([
            ("name.required", MyMessage::NameRequierd),
            ("name.start_with", MyMessage::NameStartWith),
        ]);
}

#[derive(Clone)]
enum MyMessage {
    NameRequierd,
    NameStartWith,
    Undefined,
}

impl From<Message> for MyMessage {
    fn from(_value: Message) -> Self {
        Self::Undefined
    }
}
