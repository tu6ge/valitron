use std::fmt::Display;

use crate::Value;

use super::FieldNames;

pub trait IntoMessage {
    fn into_message(rule: &'static str, field: &FieldNames, value: &Value) -> Self;
}

#[doc(hidden)]
#[derive(Debug)]
pub struct Formatter<'a> {
    template: &'a str,
    field: FieldNames,
    value: Value,
}

impl Default for Formatter<'_> {
    fn default() -> Self {
        Self {
            template: "",
            field: FieldNames::default(),
            value: Value::Unit,
        }
    }
}

impl<'a> Formatter<'a> {
    pub fn template(mut self, template: &'a str) -> Self {
        self.template = template;
        self
    }
}

impl IntoMessage for Formatter<'_> {
    fn into_message(_rule: &'static str, field: &FieldNames, value: &Value) -> Self {
        Self {
            field: field.clone(),
            value: value.clone(),
            ..Default::default()
        }
    }
}

impl Display for Formatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.template;

        let s = s.replace("{name}", self.field.as_str());
        let s = (&s).replace("{value}", &self.value.to_string());

        s.fmt(f)
    }
}
