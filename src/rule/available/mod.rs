//! available rules collection

use std::fmt::Display;

use serde::Serialize;

#[cfg(all(feature = "message", feature = "compare"))]
pub mod compare;

#[cfg(all(feature = "message", feature = "confirm"))]
pub mod confirm;

#[cfg(all(feature = "message", feature = "contains"))]
pub mod contains;

#[cfg(all(feature = "message", feature = "email"))]
pub mod email;

#[cfg(all(feature = "message", feature = "end_with"))]
pub mod end_with;

#[cfg(all(feature = "message", feature = "length"))]
pub mod length;

#[cfg(all(feature = "message", feature = "not"))]
pub mod not;

#[cfg(all(feature = "message", feature = "range"))]
pub mod range;

#[cfg(all(feature = "message", feature = "regex"))]
pub mod regex;

#[cfg(all(feature = "message", feature = "required"))]
pub mod required;

#[cfg(all(feature = "message", feature = "start_with"))]
pub mod start_with;

#[cfg(all(feature = "message", feature = "trim"))]
pub mod trim;

#[cfg(feature = "compare")]
pub use compare::{Egt, Elt, Gt, Lt};

#[cfg(feature = "confirm")]
pub use confirm::Confirm;

#[cfg(feature = "contains")]
pub use contains::Contains;

#[cfg(feature = "email")]
pub use email::Email;

#[cfg(feature = "end_with")]
pub use end_with::EndsWith;

#[cfg(feature = "length")]
pub use length::Length;

#[cfg(feature = "not")]
pub use not::Not;

#[cfg(feature = "range")]
pub use range::Range;

#[cfg(feature = "regex")]
pub use regex::Regex;

#[cfg(feature = "required")]
pub use required::Required;

#[cfg(feature = "start_with")]
pub use start_with::StartWith;

#[cfg(feature = "trim")]
pub use trim::Trim;

/// Error message, it is returned when build-in rules validate fail
#[cfg(any(feature = "full", feature = "message"))]
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Message {
    kind: MessageKind,
}

#[cfg(any(feature = "full", feature = "message"))]
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageKind {
    /// as required rule
    Required,

    /// as confrim rule, only one argument is other field name.
    Confirm(String),

    Compare(String, String),

    /// as contains rule
    Contains(String),

    /// as end_with rule
    EndsWith(String),

    /// as start_with rule, only one argument is text for comparison
    StartWith(String),

    /// as length rule,
    Length,

    /// as trim rule, this is unreachable, only mark
    Trim,

    /// as range rule
    Range,

    /// as email
    Email,

    /// as regex rule
    Regex,

    /// other way, it used by other type converting Message stopover
    Fallback(String),
}

#[cfg(any(feature = "full", feature = "message"))]
impl Serialize for MessageKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageKind::Required => serializer.serialize_str("required"),
            MessageKind::Range => serializer.serialize_str("range"),
            MessageKind::Length => serializer.serialize_str("length"),
            MessageKind::Confirm(_) => serializer.serialize_str("confirm"),
            MessageKind::Compare(_, _) => serializer.serialize_str("compare"),
            MessageKind::StartWith(_) => serializer.serialize_str("start_with"),
            MessageKind::EndsWith(_) => serializer.serialize_str("end_with"),
            MessageKind::Contains(_) => serializer.serialize_str("contains"),
            MessageKind::Trim => serializer.serialize_str("trim"),
            MessageKind::Email => serializer.serialize_str("email"),
            MessageKind::Fallback(s) => serializer.serialize_str(s),
            MessageKind::Regex => serializer.serialize_str("regex"),
        }
    }
}

#[cfg(any(feature = "full", feature = "message"))]
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

#[cfg(any(feature = "full", feature = "message"))]
impl From<Message> for String {
    fn from(msg: Message) -> Self {
        msg.to_string()
    }
}
#[cfg(any(feature = "full", feature = "message"))]
impl From<String> for Message {
    fn from(content: String) -> Self {
        Self {
            kind: MessageKind::Fallback(content),
        }
    }
}

#[cfg(any(feature = "full", feature = "message"))]
impl From<&str> for Message {
    fn from(content: &str) -> Self {
        content.to_string().into()
    }
}

#[cfg(any(feature = "full", feature = "message"))]
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

#[cfg(any(feature = "full", feature = "message"))]
impl Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageKind::Confirm(str) => {
                write!(f, "this field value must be equal to `{}` field", str)
            }
            MessageKind::Compare(ty, str) => {
                write!(f, "this field value must be {} to `{}` field", ty, str)
            }
            MessageKind::Required => "this field is required".fmt(f),
            MessageKind::StartWith(str) => write!(f, "this field must be start with `{}`", str),
            MessageKind::EndsWith(str) => write!(f, "this field must be end with `{}`", str),
            MessageKind::Contains(str) => write!(f, "this field must be contain `{}`", str),
            MessageKind::Trim => unreachable!(),
            MessageKind::Range => "the value not in the range".fmt(f),
            MessageKind::Length => "the value's length not in the range".fmt(f),
            MessageKind::Email => "the value is not a email address".fmt(f),
            MessageKind::Fallback(s) => s.fmt(f),
            MessageKind::Regex => "regular matching failed".fmt(f),
        }
    }
}

#[cfg(any(feature = "full", feature = "message"))]
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
