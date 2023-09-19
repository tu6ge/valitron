//! available rules collection

use std::fmt::Display;

use serde::Serialize;

pub mod confirm;
pub mod range;
pub mod required;
pub mod start_with;
pub mod trim;

pub use confirm::Confirm;
pub use range::Range;
pub use required::Required;
pub use start_with::StartWith;
pub use trim::Trim;

/// Error message returned when validate fail
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Message {
    kind: MessageKind,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.kind.serialize(serializer)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum MessageKind {
    Required,
    Confirm(String),
    StartWith(String),
    Trim,
    Range,
    Undefined(String),
}

impl Message {
    pub fn new(kind: MessageKind) -> Self {
        Message { kind }
    }

    pub fn undefined(content: String) -> Self {
        Message {
            kind: MessageKind::Undefined(content),
        }
    }

    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }
}

impl From<Message> for String {
    fn from(msg: Message) -> Self {
        msg.to_string()
    }
}
impl From<String> for Message {
    fn from(content: String) -> Self {
        Self {
            kind: MessageKind::Undefined(content),
        }
    }
}

impl From<&str> for Message {
    fn from(content: &str) -> Self {
        content.to_string().into()
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageKind::Confirm(str) => {
                write!(f, "this field value must be equal to `{}` field", str)
            }
            MessageKind::Required => "this field is required".fmt(f),
            MessageKind::StartWith(str) => write!(f, "this field must be start with `{}`", str),
            MessageKind::Trim => unreachable!(),
            MessageKind::Range => "the value not in the range".fmt(f),
            MessageKind::Undefined(s) => s.fmt(f),
        }
    }
}

impl PartialEq<Message> for String {
    fn eq(&self, other: &Message) -> bool {
        self == &other.to_string()
    }
}

pub trait FromRuleMessage {
    fn from_message(msg: Message) -> Self;
}

impl FromRuleMessage for Message {
    fn from_message(msg: Message) -> Self {
        msg
    }
}

#[test]
fn test_message_serialize() {
    let msg = Message::new(MessageKind::Required);
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, r#""Required""#);

    let msg = Message::new(MessageKind::Confirm("foo".into()));
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, r#"{"Confirm":"foo"}"#);
}
