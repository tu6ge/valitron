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
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Message {
    kind: MessageKind,
}

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageKind {
    Required,
    Confirm(String),
    StartWith(String),
    Trim,
    Range,
    Fallback(String),
}

impl Serialize for MessageKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageKind::Required => serializer.serialize_str("required"),
            MessageKind::Range => serializer.serialize_str("range"),
            MessageKind::Confirm(_) => serializer.serialize_str("confirm"),
            MessageKind::StartWith(_) => serializer.serialize_str("start_with"),
            MessageKind::Trim => serializer.serialize_str("trim"),
            MessageKind::Fallback(s) => serializer.serialize_str(s),
        }
    }
}

impl Message {
    pub fn new(kind: MessageKind) -> Self {
        Message { kind }
    }

    pub fn fallback<C>(content: C) -> Self
    where
        C: Into<String>,
    {
        Message {
            kind: MessageKind::Fallback(content.into()),
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
            kind: MessageKind::Fallback(content),
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
            MessageKind::Fallback(s) => s.fmt(f),
        }
    }
}

impl PartialEq<Message> for String {
    fn eq(&self, other: &Message) -> bool {
        self == &other.to_string()
    }
}

#[test]
fn test_message_serialize() {
    let msg = Message::new(MessageKind::Required);
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, r#"{"kind":"required"}"#);

    let msg = Message::new(MessageKind::Confirm("foo".into()));
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, r#"{"kind":"confirm"}"#);

    let msg = Message::new(MessageKind::Fallback("foo".into()));
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, r#"{"kind":"foo"}"#);
}
