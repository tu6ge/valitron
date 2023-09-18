//! available rules collection

pub mod confirm;
pub mod range;
pub mod required;
pub mod start_with;
pub mod trim;

use std::fmt::Display;

pub use confirm::Confirm;
pub use range::Range;
pub use required::Required;
use serde::Serialize;
pub use start_with::StartWith;
pub use trim::Trim;

/// Error message returned when validate fail
#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize)]
pub struct Message {
    kind: MessageKind,
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

impl Default for MessageKind {
    fn default() -> Self {
        Self::Undefined(String::new())
    }
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

// #[test]
// fn test_message_serialize() {
//     let msg = Message::new(10, "hello world".into());
//     let json = serde_json::to_string(&msg).unwrap();
//     assert_eq!(json, r#"{"code":10,"content":"hello world"}"#);

//     let msg = Message::from_code(10);
//     let json = serde_json::to_string(&msg).unwrap();
//     assert_eq!(json, "10");

//     let msg = Message::from_content("hello".into());
//     let json = serde_json::to_string(&msg).unwrap();
//     assert_eq!(json, r#""hello""#);
// }
