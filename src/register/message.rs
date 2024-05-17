use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{rule::IntoRuleList, ser::Serializer, Validatable, Value, ValueMap};

use super::{field_name, FieldNames, InnerValidator, IntoFieldName, MessageKey, ValidatorError};

pub trait IntoMessage {
    fn into_message(rule: &'static str, field: &FieldNames, value: &Value) -> Self;
}

type CoreValidator<'v> = InnerValidator<String, HashMap<FieldNames, HashMap<&'v str, &'v str>>>;

/// register a string message validator
/// ## This is an example:
///
/// ```rust
/// # use serde::Serialize;
/// # use valitron::{custom, RuleExt, RuleShortcut, ValidPhrase};
/// #[derive(Serialize, Debug)]
/// struct Person {
///     introduce: &'static str,
///     age: u8,
///     weight: f32,
/// }
/// fn run() {
///     let validator = ValidPhrase::new()
///         .rule("introduce", Required.and(StartWith("I am")))
///         .rule("age", custom(age_range))
///         .message([
///             ("introduce.required", "{field} is required"),
///             (
///                 "introduce.start_with",
///                 "{field} should be starts with `I am`",
///             ),
///             ("age.custom", "age {value} is not in the range"),
///         ]);
///     let person = Person {
///         introduce: "hi",
///         age: 18,
///         weight: 20.0,
///     };
///     let res = validator.validate(person).unwrap_err();
///     assert!(res.len() == 2);
///     assert_eq!(res.get("introduce").unwrap()[0], "introduce should be starts with `I am`");
///     assert_eq!(res.get("age").unwrap()[0], "age 18 is not in the range");
/// }
///
/// fn age_range(age: &mut u8) -> Result<(), String> {
///     if *age >= 25 && *age <= 45 {
///         Ok(())
///     } else {
///         Err("age should be between 25 and 45".into())
///     }
/// }
///
/// #[derive(Clone)]
/// struct Required;
///
/// impl RuleShortcut for Required {
///     type Message = String;
///
///     fn call(&mut self, data: &mut valitron::Value) -> bool {
///         match data {
///             valitron::Value::String(s) => s.len() > 0,
///             _ => false,
///         }
///     }
///     fn message(&self) -> Self::Message {
///         "required msg".to_string()
///     }
///
///     const NAME: &'static str = "required";
/// }
///
/// #[derive(Clone)]
/// struct StartWith(&'static str);
///
/// impl RuleShortcut for StartWith {
///     type Message = String;
///     
///     fn call(&mut self, data: &mut valitron::Value) -> bool {
///         match data {
///             valitron::Value::String(s) => s.starts_with(self.0),
///             _ => false,
///         }
///     }
///     fn message(&self) -> Self::Message {
///         format!("this field must start with {}", self.0)
///     }
///     
///     const NAME: &'static str = "start_with";
/// }
/// ```
#[derive(Default, Clone)]
pub struct ValidPhrase<'v>(CoreValidator<'v>);

impl<'v> ValidPhrase<'v> {
    /// init a new ValidPhrase
    pub fn new() -> Self {
        Self::default()
    }

    /// validate given data
    pub fn validate<T>(self, data: T) -> Result<(), ValidatorError<String>>
    where
        T: Serialize,
    {
        let value = data.serialize(Serializer).unwrap();

        debug_assert!(self.0.exist_field(&value));

        let mut value_map = ValueMap::new(value);

        self.inner_validate(&mut value_map).ok()
    }

    /// validate given data and can modify it
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

    /// custom validation message
    pub fn message<const N: usize>(mut self, list: [(&'v str, &'v str); N]) -> Self {
        list.map(|(key_str, v)| {
            let MessageKey { fields, rule } =
                crate::panic_on_err!(field_name::parse_message(key_str));

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

    /// register rules
    pub fn rule<F, R>(self, field: F, rule: R) -> Self
    where
        F: IntoFieldName,
        R: IntoRuleList<String>,
    {
        Self(self.0.rule(field, rule))
    }

    /// when first validate error is encountered, right away return Err(message).
    pub fn bail(self) -> Self {
        Self(self.0.bail())
    }

    fn inner_validate(self, value_map: &mut ValueMap) -> ValidatorError<String> {
        let mut resp_message = ValidatorError::with_capacity(self.0.rules.len());

        let ValidPhrase(InnerValidator {
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

impl<'v, T> Validatable<ValidPhrase<'v>, ValidatorError<String>> for T
where
    T: Serialize,
{
    fn validate(&self, validator: ValidPhrase<'v>) -> Result<(), ValidatorError<String>> {
        validator.validate(self)
    }

    fn validate_mut<'de>(self, validator: ValidPhrase<'v>) -> Result<Self, ValidatorError<String>>
    where
        Self: Deserialize<'de>,
    {
        validator.validate_mut(self)
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{Rule, RuleExt};

    use super::*;

    #[derive(Clone, Copy)]
    struct Required;

    impl Rule for Required {
        type Message = String;

        const NAME: &'static str = "required";

        fn message(&self) -> Self::Message {
            "{field} is default msg".to_string()
        }

        fn call(&mut self, data: &mut Value) -> bool {
            if *data == 8_i8 {
                true
            } else {
                false
            }
        }
    }

    #[derive(Clone)]
    struct StartWith(&'static str);

    impl Rule for StartWith {
        type Message = String;

        fn call(&mut self, data: &mut Value) -> bool {
            match data {
                Value::String(s) => s.starts_with(self.0),
                _ => false,
            }
        }
        fn message(&self) -> Self::Message {
            format!("this field must start with {}", self.0)
        }

        const NAME: &'static str = "starts_with";
    }

    #[test]
    fn original() {
        let num = (10_i8, 11_i8);

        let validator = ValidPhrase::new()
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

        let validator = ValidPhrase::new()
            .rule("0", Required)
            .message([("0.required", "foo_message")]);
        num.validate(validator).unwrap_err();
    }

    #[test]
    fn field() {
        let num = (10_i8, 11_i8);

        let validator = ValidPhrase::new()
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

        let validator = ValidPhrase::new().rule("0", Required);

        let res = validator.validate(num).unwrap_err();

        let (filed, msg) = res.into_iter().next().unwrap();

        assert_eq!(filed.as_str(), "0");

        assert_eq!(msg[0], "0 is default msg");
    }

    #[test]
    fn value() {
        let num = (10_i8, 11_i8);

        let validator = ValidPhrase::new()
            .rule("0", Required)
            .message([("0.required", "{value} is error value, 8 is true value")]);

        let res = validator.validate(num).unwrap_err();

        let (filed, msg) = res.into_iter().next().unwrap();

        assert_eq!(filed.as_str(), "0");

        assert_eq!(msg[0], "10 is error value, 8 is true value");
    }

    #[test]
    fn message() {
        let validator = ValidPhrase::new()
            .rule("field1", Required.and(StartWith("foo")))
            .rule("field2", Required.and(StartWith("foo2")))
            .message([("field1.required", "msg1")]);
        let message_list = validator.0.get_message();
        assert_eq!(
            message_list
                .get(&("field1".into()))
                .unwrap()
                .get(&"required")
                .unwrap(),
            &"msg1"
        );
        assert_eq!(message_list.len(), 1);
        assert_eq!(message_list.get(&("field1".into())).unwrap().len(), 1);

        let validator2 = validator.clone().message([("field1.starts_with", "msg2")]);
        let message_list2 = validator2.0.get_message();
        assert_eq!(
            message_list2
                .get(&("field1".into()))
                .unwrap()
                .get(&"required")
                .unwrap(),
            &"msg1"
        );
        assert_eq!(
            message_list2
                .get(&("field1".into()))
                .unwrap()
                .get(&"starts_with")
                .unwrap(),
            &"msg2"
        );
        assert_eq!(message_list2.len(), 1);
        assert_eq!(message_list2.get(&("field1".into())).unwrap().len(), 2);

        let validator3 = validator2.clone().message([("field1.required", "msg3")]);
        let message_list3 = validator3.0.get_message();
        assert_eq!(
            message_list3
                .get(&("field1".into()))
                .unwrap()
                .get(&"required")
                .unwrap(),
            &"msg3"
        );
        assert_eq!(
            message_list3
                .get(&("field1".into()))
                .unwrap()
                .get(&"starts_with")
                .unwrap(),
            &"msg2"
        );
        assert_eq!(message_list3.len(), 1);
        assert_eq!(message_list3.get(&("field1".into())).unwrap().len(), 2);
    }
}
