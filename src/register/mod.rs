//! register rules

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    ops::Deref,
};

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
            list.map(|(key_str, v)| {
                let msg_key = panic_on_err!(field_name::parse_message(key_str));
                panic_on_err!(self.exit_message(&msg_key));
                (msg_key, v)
            })
            .into_iter(),
        );
        self
    }

    pub fn validate<'de, T>(self, data: T) -> Result<T, Response>
    where
        T: serde::ser::Serialize + serde::de::Deserialize<'de>,
    {
        let value = data.serialize(Serializer).unwrap();
        let mut value_map: ValueMap = ValueMap::new(value);
        let mut message = Response::with_capacity(self.rules.len());

        for (names, rules) in self.rules.iter() {
            value_map.index(names.clone());
            let rule_resp = rules.clone().call(&mut value_map);

            let mut field_msg = Vec::with_capacity(rule_resp.len());
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
            Ok(T::deserialize(value_map.value()).unwrap())
        } else {
            Err(message)
        }
    }

    fn rule_get(&self, names: &FieldNames) -> Option<&RuleList> {
        self.rules.get(names)
    }

    fn exit_message(&self, MessageKey { fields, rule }: &MessageKey) -> Result<(), String> {
        let names = self.rule_get(fields).ok_or({
            let field_name = fields.string();
            format!("the field \"{}\" not found in validator", field_name)
        })?;

        if names.contains(rule) {
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
    fn with_capacity(capacity: usize) -> Self {
        Self {
            message: Vec::with_capacity(capacity),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MessageKey {
    fields: FieldNames,
    rule: String,
}

impl Hash for MessageKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fields.hash(state);
        self.rule.hash(state);
    }
}

impl MessageKey {
    pub(crate) fn new(fields: FieldNames, rule: String) -> Self {
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
