//! available rules collection

pub mod confirm;
pub mod range;
pub mod required;
pub mod start_with;
pub mod trim;

pub use confirm::Confirm;
pub use range::Range;
pub use required::Required;
use serde::{ser::SerializeMap, Serialize};
pub use start_with::StartWith;
pub use trim::Trim;

/// Error message returned when validate fail
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Message {
    kind: MessageKind,
    content: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum MessageKind {
    Required,
    Confirm,
    StartWith,
    Trim,
    Range,

    #[default]
    Undefined,
}

impl Message {
    pub fn new(kind: MessageKind, content: String) -> Self {
        Message { kind, content }
    }

    pub fn as_str(&self) -> &str {
        &self.content
    }

    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }
}

impl From<Message> for String {
    fn from(msg: Message) -> Self {
        msg.content
    }
}
impl From<String> for Message {
    fn from(content: String) -> Self {
        Self {
            kind: MessageKind::Undefined,
            content,
        }
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // if self.kind != 0 && !self.content.is_empty() {
        //     let mut map = serializer.serialize_map(Some(2))?;
        //     map.serialize_entry("code", &self.code)?;
        //     map.serialize_entry("content", &self.content)?;
        //     map.end()
        // } else if self.code != 0 {
        //     serializer.serialize_u8(self.code)
        // } else {

        // }
        serializer.serialize_str(&self.content)
    }
}

impl PartialEq<Message> for String {
    fn eq(&self, other: &Message) -> bool {
        self == &other.content
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
