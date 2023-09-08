//! register rules

use std::{
    collections::{
        hash_map::{Iter, IterMut},
        HashMap,
    },
    hash::{Hash, Hasher},
};

use crate::{
    rule::{IntoRuleList, RuleList},
    ser::Serializer,
    value::ValueMap,
};

mod field_name;
mod lexer;

pub(crate) use field_name::Parser;
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
        let rules = rule.into_list();

        // if !rules.valid_name() {
        //     panic!("")
        // }

        self.rules.insert(names, rules);
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

    pub fn validate<'de, T>(self, data: T) -> Result<T, ValidatorError>
    where
        T: serde::ser::Serialize + serde::de::Deserialize<'de>,
    {
        let value = data.serialize(Serializer).unwrap();
        let mut value_map: ValueMap = ValueMap::new(value);
        let mut message = ValidatorError::with_capacity(self.rules.len());

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

            field_msg.shrink_to_fit();

            message.push(names.clone(), field_msg);
        }

        message.shrink_to_fit();

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorError {
    message: HashMap<FieldNames, Vec<String>>,
}

// impl Deref for ValidatorError {
//     type Target = HashMap<FieldNames, Vec<String>>;
//     fn deref(&self) -> &Self::Target {
//         &self.message
//     }
// }

impl ValidatorError {
    fn new() -> Self {
        Self {
            message: HashMap::new(),
        }
    }
    fn with_capacity(capacity: usize) -> Self {
        Self {
            message: HashMap::with_capacity(capacity),
        }
    }

    fn push(&mut self, field_name: FieldNames, message: Vec<String>) {
        if !message.is_empty() {
            self.message.insert(field_name, message);
        }
    }

    fn shrink_to_fit(&mut self) {
        self.message.shrink_to_fit()
    }

    pub fn get<K: IntoFieldName>(&self, key: K) -> Option<&Vec<String>> {
        let k = key.into_field().ok()?;
        self.message.get(&k)
    }

    pub fn get_key_value<K: IntoFieldName>(&self, key: K) -> Option<(&FieldNames, &Vec<String>)> {
        let k = key.into_field().ok()?;
        self.message.get_key_value(&k)
    }

    pub fn contains_key<K: IntoFieldName>(&self, key: K) -> bool {
        match key.into_field() {
            Ok(k) => self.message.contains_key(&k),
            Err(_) => false,
        }
    }

    pub fn iter(&self) -> Iter<'_, FieldNames, Vec<String>> {
        self.message.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, FieldNames, Vec<String>> {
        self.message.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.message.is_empty()
    }

    pub fn len(&self) -> usize {
        self.message.len()
    }
}

impl From<ValidatorError> for Result<(), ValidatorError> {
    fn from(value: ValidatorError) -> Self {
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
