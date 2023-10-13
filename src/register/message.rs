use crate::Value;

use super::FieldNames;

pub trait IntoMessage {
    fn into_message(rule: &'static str, field: &FieldNames, value: &Value) -> Self;
}
