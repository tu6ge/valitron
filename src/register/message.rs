use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{rule::IntoRuleList, ser::Serializer, Validatable, Value, ValueMap};

use super::{field_name, FieldNames, InnerValidator, IntoFieldName, MessageKey, ValidatorError};

pub trait IntoMessage {
    fn into_message(rule: &'static str, field: &FieldNames, value: &Value) -> Self;
}

macro_rules! panic_on_err {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => panic!("{err}"),
        }
    };
}

type CoreValidator<'v> = InnerValidator<&'v str, HashMap<FieldNames, HashMap<&'v str, &'v str>>>;

pub struct StringValidator<'v>(CoreValidator<'v>);

impl<'v> StringValidator<'v> {
    pub fn new() -> Self {
        Self(CoreValidator::default())
    }

    pub fn validate<T>(self, data: T) -> Result<(), ValidatorError<String>>
    where
        T: Serialize,
    {
        let value = data.serialize(Serializer).unwrap();

        debug_assert!(self.0.exist_field(&value));

        let mut value_map = ValueMap::new(value);

        self.inner_validate(&mut value_map).ok()
    }

    pub fn validate_mut<'de, T>(self, data: T) -> Result<T, ValidatorError<String>>
    where
        T: Serialize + serde::de::Deserialize<'de>,
    {
        let value = data.serialize(Serializer).unwrap();

        debug_assert!(self.0.exist_field(&value));

        let mut value_map = ValueMap::new(value);

        self.inner_validate(&mut value_map)
            .ok()
            .map(|_| T::deserialize(value_map.value()).unwrap())
    }

    pub fn message<const N: usize>(mut self, list: [(&'v str, &'v str); N]) -> Self {
        list.map(|(key_str, v)| {
            let MessageKey { fields, rule } = panic_on_err!(field_name::parse_message(key_str));

            debug_assert!(
                self.0.rule_get(&fields).is_some(),
                "the field \"{}\" not found in validator",
                fields.as_str()
            );
            debug_assert!(
                self.0.rule_get(&fields).unwrap().contains(rule),
                "rule \"{rule}\" is not found in rules"
            );

            self.0
                .message
                .entry(fields)
                .and_modify(|field| {
                    field
                        .entry(rule)
                        .and_modify(|msg| {
                            *msg = v;
                        })
                        .or_insert(v);
                })
                .or_insert({
                    let mut map = HashMap::new();
                    map.insert(rule, v);
                    map
                });
        });

        Self(self.0)
    }

    // pub fn map<M2>(self, f: fn(message: &'v str) -> M2) -> CoreValidator<'v, M2>
    // where
    //     M2: 'static,
    // {
    //     todo!()
    // }

    pub fn rule<F, R>(self, field: F, rule: R) -> Self
    where
        F: IntoFieldName,
        R: IntoRuleList<&'v str>,
    {
        Self(self.0.rule(field, rule))
    }

    pub fn bail(self) -> Self {
        Self(self.0.bail())
    }

    fn inner_validate(self, value_map: &mut ValueMap) -> ValidatorError<String> {
        let mut resp_message = ValidatorError::with_capacity(self.0.rules.len());

        let StringValidator(InnerValidator {
            rules,
            message,
            is_bail,
        }) = self;

        let default_map = HashMap::new();

        for (mut names, mut rules) in rules.into_iter() {
            if is_bail {
                rules.set_bail();
            }

            let msgs = message.get(&names).unwrap_or(&default_map);

            value_map.index(names);

            let field_msg = rules.call_string_message(value_map, msgs);

            names = value_map.take_index();

            resp_message.push(names, field_msg);

            if is_bail && !resp_message.is_empty() {
                resp_message.shrink_to(1);
                return resp_message;
            }
        }

        resp_message.shrink_to_fit();

        resp_message
    }
}

impl<'v, T> Validatable<StringValidator<'v>, ValidatorError<String>> for T
where
    T: Serialize,
{
    fn validate(&self, validator: StringValidator<'v>) -> Result<(), ValidatorError<String>> {
        validator.validate(self)
    }

    fn validate_mut<'de>(
        self,
        validator: StringValidator<'v>,
    ) -> Result<Self, ValidatorError<String>>
    where
        Self: Deserialize<'de>,
    {
        validator.validate_mut(self)
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::RuleShortcut;

    use super::*;

    #[derive(Clone, Copy)]
    struct Required;

    impl RuleShortcut for Required {
        type Message = &'static str;

        const NAME: &'static str = "required";

        fn message(&self) -> Self::Message {
            "{field} is default msg"
        }

        fn call(&mut self, data: &mut Value) -> bool {
            if *data == 8_i8 {
                true
            } else {
                false
            }
        }
    }

    #[test]
    fn original() {
        let num = (10_i8, 11_i8);

        let validator = StringValidator::new()
            .rule("0", Required)
            .message([("0.required", "foo_message")]);

        let res = validator.validate(num).unwrap_err();

        let (filed, msg) = res.into_iter().next().unwrap();

        assert_eq!(filed.as_str(), "0");

        assert_eq!(msg[0], "foo_message");
    }

    #[test]
    fn test_trait() {
        let num = (10_i8, 11_i8);

        let validator = StringValidator::new()
            .rule("0", Required)
            .message([("0.required", "foo_message")]);
        num.validate(validator).unwrap_err();
    }

    #[test]
    fn field() {
        let num = (10_i8, 11_i8);

        let validator = StringValidator::new()
            .rule("0", Required)
            .message([("0.required", "{field} is required")]);

        let res = validator.validate(num).unwrap_err();

        let (filed, msg) = res.into_iter().next().unwrap();

        assert_eq!(filed.as_str(), "0");

        assert_eq!(msg[0], "0 is required");
    }

    #[test]
    fn default_field() {
        let num = (10_i8, 11_i8);

        let validator = StringValidator::new().rule("0", Required);

        let res = validator.validate(num).unwrap_err();

        let (filed, msg) = res.into_iter().next().unwrap();

        assert_eq!(filed.as_str(), "0");

        assert_eq!(msg[0], "0 is default msg");
    }

    #[test]
    fn value() {
        let num = (10_i8, 11_i8);

        let validator = StringValidator::new()
            .rule("0", Required)
            .message([("0.required", "{value} is error value, 8 is true value")]);

        let res = validator.validate(num).unwrap_err();

        let (filed, msg) = res.into_iter().next().unwrap();

        assert_eq!(filed.as_str(), "0");

        assert_eq!(msg[0], "10 is error value, 8 is true value");
    }
}
