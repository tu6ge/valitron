use crate::ValueMap;

pub trait IntoMessage {
    fn into_message(rule_name: &'static str, map: &ValueMap) -> Self;
}
