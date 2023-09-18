//! register validator

use std::{
    collections::{
        hash_map::{IntoIter, Iter, IterMut, Keys},
        HashMap,
    },
    error::Error,
    fmt::Display,
    hash::{Hash, Hasher},
};

use crate::{
    rule::{IntoRuleList, RuleList},
    ser::Serializer,
    value::ValueMap,
};

#[cfg(feature = "full")]
use crate::available::Message;
pub use field_name::{FieldName, FieldNames};
pub(crate) use field_name::{IntoFieldName, Parser};
use serde::{Deserialize, Serialize};

mod field_name;
mod lexer;

/// register a validator
#[cfg_attr(docsrs, doc(cfg(feature = "full")))]
#[derive(Default)]
#[cfg(feature = "full")]
pub struct Validator<M1, M = Message> {
    rules: HashMap<FieldNames, RuleList<M>>,
    message: HashMap<MessageKey, M1>,
}

/// register a validator
/// ## This is an example:
///
/// ```rust
/// # use serde::{Deserialize, Serialize};
/// # use valitron::{
/// # available::{Required, StartWith},
/// # custom, RuleExt, Validator
/// # };
/// #[derive(Serialize, Debug)]
/// struct Person {
///     introduce: &'static str,
///     age: u8,
///     weight: f32,
/// }
///
/// # fn main() {
/// let validator = Validator::new()
///     .rule("introduce", Required.and(StartWith("I am")))
///     .rule("age", custom(age_range))
///     .message([
///         ("introduce.required", "introduce is required"),
///         ("introduce.start_with", "introduce should be starts with `I am`"),
///     ]);
///
/// let person = Person {
///     introduce: "hi",
///     age: 18,
///     weight: 20.0,
/// };
///
/// let res = validator.validate(person).unwrap_err();
/// assert!(res.len() == 2);
/// # }
///
/// fn age_range(age: &mut u8) -> Result<(), String> {
///     if *age >= 25 && *age <= 45 {
///         Ok(())
///     } else {
///         Err("age should be between 25 and 45".into())
///     }
/// }
/// ```
#[cfg_attr(docsrs, doc(cfg(not(feature = "full"))))]
#[derive(Default)]
#[cfg(not(feature = "full"))]
pub struct Validator<M = String> {
    rules: HashMap<FieldNames, RuleList<M>>,
    message: HashMap<MessageKey, M>,
}

macro_rules! panic_on_err {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => panic!("{err}"),
        }
    };
}

#[cfg(feature = "full")]
impl<M1, M> Validator<M1, M> {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            message: HashMap::new(),
        }
    }
}

#[cfg(not(feature = "full"))]
impl<M> Validator<M> {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            message: HashMap::new(),
        }
    }
}

#[cfg(feature = "full")]
impl<M1, M> Validator<M1, M>
where
    M1: Clone + 'static + From<M>,
    M: Clone + 'static,
{
    /// # Register rules
    ///
    /// **Feild support multiple formats:**
    /// - `field1` used to matching struct field
    /// - `0`,`1`.. used to matching tuple item or tuple struct field
    /// - `[0]`,`[1]` used to matching array item
    /// - `[foo]` used to matching struct variant, e.g. `enum Foo{ Color { r: u8, g: u8, b: u8 } }`
    ///
    /// fields support nest:
    /// - `field1.0`
    /// - `0.color`
    /// - `[12].1`
    /// - `foo.1[color]`
    /// - more combine
    ///
    /// BNF indicate
    /// ```bnf
    /// exp                    ::= <tuple_index>
    ///                          | <array_index>
    ///                          | <ident>
    ///                          | <struct_variant_index>
    ///                          | <exp> '.' <tuple_index>
    ///                          | <exp> '.' <ident>
    ///                          | <exp> <array_index>
    ///                          | <exp> <struct_variant_index>
    /// tuple_index            ::= <u8>
    /// array_index            ::= '[' <usize> ']'
    /// struct_variant_index   ::= '[' <ident> ']'
    /// ```
    ///
    /// **Rule also support multiple formats:**
    /// - `RuleFoo`
    /// - `RuleFoo.and(RuleBar)` combineable
    /// - `custom(handler)` closure usage
    /// - `RuleFoo.custom(handler)` type and closure
    /// - `custom(handler).and(RuleFoo)` closure and type
    /// - `RuleFoo.and(RuleBar).bail()` when first validate error, immediately return error with one message.
    ///
    /// *Available Rules*
    /// - [`Required`]
    /// - [`StartWith`]
    /// - [`Confirm`]
    /// - [`Trim`]
    /// - [`Range`]
    /// - customizable
    ///
    /// # Panic
    ///
    /// - Field format error will be panic
    /// - Invalid rule name will be panic
    ///
    /// [`Required`]: crate::available::required
    /// [`StartWith`]: crate::available::start_with
    /// [`Confirm`]: crate::available::confirm
    /// [`Trim`]: crate::available::trim
    /// [`Range`]: crate::available::range
    pub fn rule<F, R>(mut self, field: F, rule: R) -> Self
    where
        F: IntoFieldName,
        R: IntoRuleList<M>,
    {
        let names = panic_on_err!(field.into_field());
        let rules = rule.into_list();

        if !rules.valid_name() {
            panic!("invalid rule name")
        }

        self.rules.insert(names, rules);
        self
    }

    /// Custom validate error message
    ///
    /// Every rule has a default message, the method should be replace it with your need.
    ///
    /// parameter list item format:
    /// `(field_name.rule_name, message)`
    ///
    /// e.g: `("name.required", "name is required")`
    ///
    /// # Panic
    ///
    /// When field or rule is not existing ,this will panic
    pub fn message<'key, const N: usize, MSG>(mut self, list: [(&'key str, MSG); N]) -> Self
    where
        MSG: Into<M1>,
    {
        self.message = HashMap::from_iter(
            list.map(|(key_str, v)| {
                let msg_key = panic_on_err!(field_name::parse_message(key_str));

                panic_on_err!(self.exit_message(&msg_key));

                (msg_key, v.into())
            })
            .into_iter(),
        );
        self
    }

    /// run validate without modifiable
    pub fn validate<T>(self, data: T) -> Result<(), ValidatorError<M1>>
    where
        T: Serialize,
    {
        let value = data.serialize(Serializer).unwrap();

        let mut value_map = ValueMap::new(value);

        let message = self.inner_validate(&mut value_map);

        if message.is_empty() {
            Ok(())
        } else {
            Err(message)
        }
    }

    /// run validate with modifiable
    pub fn validate_mut<'de, T>(self, data: T) -> Result<T, ValidatorError<M1>>
    where
        T: Serialize + serde::de::Deserialize<'de>,
    {
        let value = data.serialize(Serializer).unwrap();

        let mut value_map = ValueMap::new(value);

        let message = self.inner_validate(&mut value_map);

        if message.is_empty() {
            Ok(T::deserialize(value_map.value()).unwrap())
        } else {
            Err(message)
        }
    }

    fn inner_validate(self, value_map: &mut ValueMap) -> ValidatorError<M1> {
        let mut message = ValidatorError::with_capacity(self.rules.len());

        for (names, rules) in self.rules.iter() {
            value_map.index(names.clone());
            let rule_resp = rules.clone().call(value_map);

            let mut field_msg = Vec::with_capacity(rule_resp.len());
            for (rule, msg) in rule_resp.into_iter() {
                let final_msg =
                    match self.get_message(&MessageKey::new(names.clone(), rule.to_string())) {
                        Some(s) => s.clone(),
                        None => msg.into(),
                    };
                field_msg.push(final_msg);
            }

            field_msg.shrink_to_fit();

            message.push(names.clone(), field_msg);
        }

        message.shrink_to_fit();

        message
    }

    fn rule_get(&self, names: &FieldNames) -> Option<&RuleList<M>> {
        self.rules.get(names)
    }

    fn exit_message(&self, MessageKey { fields, rule }: &MessageKey) -> Result<(), String> {
        let names = self.rule_get(fields).ok_or(format!(
            "the field \"{}\" not found in validator",
            fields.as_str()
        ))?;

        if names.contains(rule) {
            Ok(())
        } else {
            Err(format!("rule \"{rule}\" is not found in rules"))
        }
    }

    fn get_message(&self, key: &MessageKey) -> Option<&M1> {
        self.message.get(key)
    }
}

#[cfg(not(feature = "full"))]
impl<M> Validator<M>
where
    M: Clone + 'static,
{
    /// # Register rules
    ///
    /// **Feild support multiple formats:**
    /// - `field1` used to matching struct field
    /// - `0`,`1`.. used to matching tuple item or tuple struct field
    /// - `[0]`,`[1]` used to matching array item
    /// - `[foo]` used to matching struct variant, e.g. `enum Foo{ Color { r: u8, g: u8, b: u8 } }`
    ///
    /// fields support nest:
    /// - `field1.0`
    /// - `0.color`
    /// - `[12].1`
    /// - `foo.1[color]`
    /// - more combine
    ///
    /// BNF indicate
    /// ```bnf
    /// exp                    ::= <tuple_index>
    ///                          | <array_index>
    ///                          | <ident>
    ///                          | <struct_variant_index>
    ///                          | <exp> '.' <tuple_index>
    ///                          | <exp> '.' <ident>
    ///                          | <exp> <array_index>
    ///                          | <exp> <struct_variant_index>
    /// tuple_index            ::= <u8>
    /// array_index            ::= '[' <usize> ']'
    /// struct_variant_index   ::= '[' <ident> ']'
    /// ```
    ///
    /// **Rule also support multiple formats:**
    /// - `RuleFoo`
    /// - `RuleFoo.and(RuleBar)` combineable
    /// - `custom(handler)` closure usage
    /// - `RuleFoo.custom(handler)` type and closure
    /// - `custom(handler).and(RuleFoo)` closure and type
    /// - `RuleFoo.and(RuleBar).bail()` when first validate error, immediately return error with one message.
    ///
    /// *Available Rules*
    /// - [`Required`]
    /// - [`StartWith`]
    /// - [`Confirm`]
    /// - [`Trim`]
    /// - [`Range`]
    /// - customizable
    ///
    /// # Panic
    ///
    /// - Field format error will be panic
    /// - Invalid rule name will be panic
    ///
    /// [`Required`]: crate::available::required
    /// [`StartWith`]: crate::available::start_with
    /// [`Confirm`]: crate::available::confirm
    /// [`Trim`]: crate::available::trim
    /// [`Range`]: crate::available::range
    pub fn rule<F, R>(mut self, field: F, rule: R) -> Self
    where
        F: IntoFieldName,
        R: IntoRuleList<M>,
    {
        let names = panic_on_err!(field.into_field());
        let rules = rule.into_list();

        if !rules.valid_name() {
            panic!("invalid rule name")
        }

        self.rules.insert(names, rules);
        self
    }

    /// Custom validate error message
    ///
    /// Every rule has a default message, the method should be replace it with your need.
    ///
    /// parameter list item format:
    /// `(field_name.rule_name, message)`
    ///
    /// e.g: `("name.required", "name is required")`
    ///
    /// # Panic
    ///
    /// When field or rule is not existing ,this will panic
    pub fn message<'key, const N: usize, MSG>(mut self, list: [(&'key str, MSG); N]) -> Self
    where
        MSG: Into<M>,
    {
        self.message = HashMap::from_iter(
            list.map(|(key_str, v)| {
                let msg_key = panic_on_err!(field_name::parse_message(key_str));

                panic_on_err!(self.exit_message(&msg_key));

                (msg_key, v.into())
            })
            .into_iter(),
        );
        self
    }

    /// run validate without modifiable
    pub fn validate<T>(self, data: T) -> Result<(), ValidatorError<M>>
    where
        T: Serialize,
    {
        let value = data.serialize(Serializer).unwrap();

        let mut value_map = ValueMap::new(value);

        let message = self.inner_validate(&mut value_map);

        if message.is_empty() {
            Ok(())
        } else {
            Err(message)
        }
    }

    /// run validate with modifiable
    pub fn validate_mut<'de, T>(self, data: T) -> Result<T, ValidatorError<M>>
    where
        T: Serialize + serde::de::Deserialize<'de>,
    {
        let value = data.serialize(Serializer).unwrap();

        let mut value_map = ValueMap::new(value);

        let message = self.inner_validate(&mut value_map);

        if message.is_empty() {
            Ok(T::deserialize(value_map.value()).unwrap())
        } else {
            Err(message)
        }
    }

    fn inner_validate(self, value_map: &mut ValueMap) -> ValidatorError<M> {
        let mut message = ValidatorError::with_capacity(self.rules.len());

        for (names, rules) in self.rules.iter() {
            value_map.index(names.clone());
            let rule_resp = rules.clone().call(value_map);

            let mut field_msg = Vec::with_capacity(rule_resp.len());
            for (rule, msg) in rule_resp.into_iter() {
                let final_msg =
                    match self.get_message(&MessageKey::new(names.clone(), rule.to_string())) {
                        Some(s) => s.clone(),
                        None => msg,
                    };
                field_msg.push(final_msg);
            }

            field_msg.shrink_to_fit();

            message.push(names.clone(), field_msg);
        }

        message.shrink_to_fit();

        message
    }

    fn rule_get(&self, names: &FieldNames) -> Option<&RuleList<M>> {
        self.rules.get(names)
    }

    fn exit_message(&self, MessageKey { fields, rule }: &MessageKey) -> Result<(), String> {
        let names = self.rule_get(fields).ok_or(format!(
            "the field \"{}\" not found in validator",
            fields.as_str()
        ))?;

        if names.contains(rule) {
            Ok(())
        } else {
            Err(format!("rule \"{rule}\" is not found in rules"))
        }
    }

    fn get_message(&self, key: &MessageKey) -> Option<&M> {
        self.message.get(key)
    }
}

/// validateable for any types
#[cfg(not(feature = "full"))]
pub trait Validatable<M> {
    /// if not change value
    fn validate(&self, validator: Validator<M>) -> Result<(), ValidatorError<M>>;

    /// if need to change value, e.g. `trim`
    fn validate_mut<'de>(self, validator: Validator<M>) -> Result<Self, ValidatorError<M>>
    where
        Self: Sized + Deserialize<'de>;
}

#[cfg(not(feature = "full"))]
impl<T, M> Validatable<M> for T
where
    T: Serialize,
    M: Clone + 'static,
{
    fn validate(&self, validator: Validator<M>) -> Result<(), ValidatorError<M>> {
        validator.validate(self)
    }

    fn validate_mut<'de>(self, validator: Validator<M>) -> Result<Self, ValidatorError<M>>
    where
        Self: Sized + Deserialize<'de>,
    {
        validator.validate_mut(self)
    }
}

#[cfg(feature = "full")]
pub trait Validatable<M1, M> {
    /// if not change value
    fn validate(&self, validator: Validator<M1, M>) -> Result<(), ValidatorError<M1>>;

    /// if need to change value, e.g. `trim`
    fn validate_mut<'de>(self, validator: Validator<M1, M>) -> Result<Self, ValidatorError<M1>>
    where
        Self: Sized + Deserialize<'de>;
}

#[cfg(feature = "full")]
impl<T, M, M1> Validatable<M1, M> for T
where
    T: Serialize,
    M: Clone + 'static,
    M1: Clone + 'static + From<M>,
{
    fn validate(&self, validator: Validator<M1, M>) -> Result<(), ValidatorError<M1>> {
        validator.validate(self)
    }

    fn validate_mut<'de>(self, validator: Validator<M1, M>) -> Result<Self, ValidatorError<M1>>
    where
        Self: Sized + Deserialize<'de>,
    {
        validator.validate_mut(self)
    }
}

/// store validate error message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorError<M = String> {
    message: HashMap<FieldNames, Vec<M>>,
}

impl<M> Serialize for ValidatorError<M>
where
    M: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.message.serialize(serializer)
    }
}

impl<M> Display for ValidatorError<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "validate error".fmt(f)
    }
}

impl<M> Error for ValidatorError<M> where M: std::fmt::Debug {}

// impl Deref for ValidatorError {
//     type Target = HashMap<FieldNames, Vec<String>>;
//     fn deref(&self) -> &Self::Target {
//         &self.message
//     }
// }

impl<M> ValidatorError<M> {
    #[cfg(test)]
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

    fn push(&mut self, field_name: FieldNames, message: Vec<M>) {
        if !message.is_empty() {
            self.message.insert(field_name, message);
        }
    }

    fn shrink_to_fit(&mut self) {
        self.message.shrink_to_fit()
    }

    pub fn get<K: IntoFieldName>(&self, key: K) -> Option<&Vec<M>> {
        let k = key.into_field().ok()?;
        self.message.get(&k)
    }

    pub fn get_key_value<K: IntoFieldName>(&self, key: K) -> Option<(&FieldNames, &Vec<M>)> {
        let k = key.into_field().ok()?;
        self.message.get_key_value(&k)
    }

    pub fn contains_key<K: IntoFieldName>(&self, key: K) -> bool {
        match key.into_field() {
            Ok(k) => self.message.contains_key(&k),
            Err(_) => false,
        }
    }

    pub fn keys(&self) -> Keys<'_, FieldNames, Vec<M>> {
        self.message.keys()
    }

    pub fn iter(&self) -> Iter<'_, FieldNames, Vec<M>> {
        self.message.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, FieldNames, Vec<M>> {
        self.message.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.message.is_empty()
    }

    pub fn len(&self) -> usize {
        self.message.len()
    }
}

impl<'a, M> IntoIterator for &'a mut ValidatorError<M> {
    type Item = (&'a FieldNames, &'a mut Vec<M>);
    type IntoIter = IterMut<'a, FieldNames, Vec<M>>;
    fn into_iter(self) -> Self::IntoIter {
        self.message.iter_mut()
    }
}

impl<M> IntoIterator for ValidatorError<M> {
    type Item = (FieldNames, Vec<M>);
    type IntoIter = IntoIter<FieldNames, Vec<M>>;
    fn into_iter(self) -> Self::IntoIter {
        self.message.into_iter()
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

// impl From<ValidatorError> for Result<(), ValidatorError> {
//     fn from(value: ValidatorError) -> Self {
//         if value.is_empty() {
//             Ok(())
//         } else {
//             Err(value)
//         }
//     }
// }

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

#[test]
fn test_validator_error_serialize() {
    let mut error = ValidatorError::<String>::new();
    error.push(
        FieldNames::new("field1".into()),
        vec!["message1".into(), "message2".into()],
    );

    let json = serde_json::to_string(&error).unwrap();
    assert_eq!(json, r#"{"field1":["message1","message2"]}"#);
}
