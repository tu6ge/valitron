//! register rules

use std::{collections::HashMap, ops::Deref};

use crate::{
    rule::{IntoRuleList, RuleList},
    ser::{Serializer, ValueMap},
};

mod field_name;
mod lexer;

pub use field_name::{FieldName, FieldNames};

use self::field_name::IntoFieldName;

#[derive(Default)]
pub struct Validator<'a> {
    rules: HashMap<FieldNames, RuleList>,
    message: HashMap<MessageKey, &'a str>,
}

macro_rules! panic_on_err {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => panic!("{err}"),
        }
    };
}

impl<'a> Validator<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn rule<F: IntoFieldName, R: IntoRuleList>(mut self, field: F, rule: R) -> Self {
        let names = panic_on_err!(field.into_field());
        self.rules.insert(names, rule.into_list());
        self
    }

    pub fn message<const N: usize>(mut self, list: [(&'a str, &'a str); N]) -> Self {
        self.message = HashMap::from_iter(
            list.map(|(k_str, v)| {
                let key = panic_on_err!(field_name::parse_message(k_str));
                panic_on_err!(self.exit_message(&k_str, &key));
                (key, v)
            })
            .into_iter(),
        );
        self
    }

    pub fn validate<T>(self, data: T) -> Result<(), Response>
    where
        T: serde::ser::Serialize,
    {
        let value = data.serialize(Serializer).unwrap();
        let mut value_map: ValueMap = ValueMap::new(value);
        let mut message = Response::new();

        for (names, rules) in self.rules.iter() {
            value_map.index(names.clone());
            let rule_resp = rules.clone().call(&mut value_map);

            let mut field_msg = Vec::new();
            for (rule, msg) in rule_resp.into_iter() {
                let final_msg =
                    match self.get_message(&MessageKey::new(names.clone(), rule.to_string())) {
                        Some(s) => s.to_string(),
                        None => msg,
                    };
                field_msg.push(final_msg);
            }

            message.push(names.clone(), field_msg);
        }

        if message.is_empty() {
            Ok(())
        } else {
            Err(message)
        }
    }

    fn rule_get(&self, names: &FieldNames) -> Option<&RuleList> {
        self.rules.get(names)
    }

    fn rules_name(&self, names: &FieldNames) -> Option<Vec<&'static str>> {
        self.rule_get(names).map(|rule| rule.get_rules_name())
    }

    fn exit_message(
        &self,
        k_str: &str,
        MessageKey { fields, rule }: &MessageKey,
    ) -> Result<(), String> {
        let point_index = k_str
            .rfind('.')
            .ok_or(format!("no found `.` in the message index"))?;
        let names = self.rules_name(fields).ok_or(format!(
            "the field \"{}\" not found in validator",
            &k_str[..point_index]
        ))?;

        if names.contains(&rule.as_str()) {
            Ok(())
        } else {
            Err(format!("rule \"{rule}\" is not found in rules"))
        }
    }

    fn get_message(&self, key: &MessageKey) -> Option<&&str> {
        self.message.get(key)
    }
}

#[derive(Debug)]
pub struct Response {
    message: Vec<(FieldNames, Vec<String>)>,
}

impl Deref for Response {
    type Target = Vec<(FieldNames, Vec<String>)>;
    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl Response {
    fn new() -> Self {
        Self {
            message: Vec::new(),
        }
    }

    fn push(&mut self, field_name: FieldNames, message: Vec<String>) {
        if !message.is_empty() {
            self.message.push((field_name, message));
        }
    }
}

impl From<Response> for Result<(), Response> {
    fn from(value: Response) -> Self {
        if value.is_empty() {
            Ok(())
        } else {
            Err(value)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MessageKey {
    fields: FieldNames,
    rule: String,
}

impl MessageKey {
    pub fn new(fields: FieldNames, rule: String) -> Self {
        Self { fields, rule }
    }
}

// #[test]
// fn test_message() {
//     let ruler = Ruler::new().message([
//         ("name.required", "name mut not be null"),
//         ("password.password", "password mut not very simple"),
//     ]);
// }
