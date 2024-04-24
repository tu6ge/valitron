//! register validator
//!
//! The [`Validator`] has a generics argument `M`, it is used validate message type,
//! it default is `String`, but when `full` feature is enabled, default is [`Message`].
//! Besides, it can be every types with your idea.
//!
//! if `M1` can be converted to `M2`, then `Validator<M1>` can be
//! converted to `Validator<M2>` with [`map`] method:
//!
//! ```ignore
//! let validator1 = Validator::<M1>::new();
//! let validator2 = validator1.map(M2::from);
//! ```
//! This can integrate built-in [rules] with your application very well.
//!
//! [`Message`]: crate::available::Message
//! [`map`]: Validator::map
//! [rules]: crate::available

use std::{
    collections::{
        hash_map::{IntoIter, Iter, IterMut, Keys},
        HashMap,
    },
    error::Error,
    fmt::Display,
    hash::{Hash, Hasher},
    ops::Index,
};

use crate::{
    rule::{IntoRuleList, RuleList},
    ser::Serializer,
    value::ValueMap,
    Value,
};

pub use field_name::{FieldName, FieldNames};
pub(crate) use field_name::{IntoFieldName, Parser};
pub use message::{IntoMessage, ValidPhrase};
use serde::{Deserialize, Serialize};

mod field_name;
mod lexer;
mod message;
#[cfg(test)]
mod tests;

/// register a validator
/// ## This is an example:
///
/// ```rust
/// # use serde::{Deserialize, Serialize};
/// # use valitron::{
/// # available::{Required, StartWith, Message},
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
///         (
///             "introduce.start_with",
///             "introduce should be starts with `I am`",
///         ),
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
/// fn age_range(age: &mut u8) -> Result<(), Message> {
///     if *age >= 25 && *age <= 45 {
///         Ok(())
///     } else {
///         Err("age should be between 25 and 45".into())
///     }
/// }
/// ```
pub type Validator<'v, M> = InnerValidator<M, HashMap<MessageKey<'v>, M>>;

/// # A validator for build messages
/// build message with rule name, field name and value
///
/// *message need to implement [`IntoMessage`]*
///
/// [`IntoMessage`]: message::IntoMessage
pub type ValidatorRefine<M> = InnerValidator<M, ()>;

#[doc(hidden)]
pub struct InnerValidator<M, List> {
    rules: HashMap<FieldNames, RuleList<M>>,
    message: List,
    is_bail: bool,
}

macro_rules! panic_on_err {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => panic!("{err}"),
        }
    };
}

impl<M> Validator<'_, M> {
    pub fn new() -> Self {
        Self::default()
    }

    /// run validate without modifiable
    pub fn validate<T>(self, data: T) -> Result<(), ValidatorError<M>>
    where
        T: Serialize,
    {
        let value = data.serialize(Serializer).unwrap();

        debug_assert!(self.exist_field(&value));

        let mut value_map = ValueMap::new(value);

        self.inner_validate(&mut value_map).ok()
    }

    /// run validate with modifiable
    pub fn validate_mut<'de, T>(self, data: T) -> Result<T, ValidatorError<M>>
    where
        T: Serialize + serde::de::Deserialize<'de>,
    {
        let value = data.serialize(Serializer).unwrap();

        debug_assert!(self.exist_field(&value));

        let mut value_map = ValueMap::new(value);

        self.inner_validate(&mut value_map)
            .ok()
            .map(|_| T::deserialize(value_map.value()).unwrap())
    }

    fn inner_validate(self, value_map: &mut ValueMap) -> ValidatorError<M> {
        fn handle_msg<M>(
            rules: RuleList<M>,
            value_map: &mut ValueMap,
            message: &mut HashMap<MessageKey<'_>, M>,
        ) -> Vec<M> {
            rules
                .call(value_map)
                .into_iter()
                .map(|(rule, msg)| {
                    message
                        .remove(&MessageKey::new(value_map.as_index().clone(), rule))
                        .unwrap_or(msg)
                })
                .collect()
        }
        self.iter_validate(value_map, handle_msg)
    }

    fn exit_message(&self, MessageKey { fields, rule }: &MessageKey) -> bool {
        debug_assert!(
            self.rule_get(fields).is_some(),
            "the field \"{}\" not found in validator",
            fields.as_str()
        );

        debug_assert!(
            self.rule_get(fields).unwrap().contains(rule),
            "rule \"{rule}\" is not found in rules"
        );

        true
    }
}

impl<M> ValidatorRefine<M> {
    pub fn new() -> Self {
        Self::default()
    }

    /// run validate without modifiable
    pub fn validate<T, M2>(self, data: T) -> Result<(), ValidatorError<M2>>
    where
        T: Serialize,
        M2: IntoMessage,
    {
        let value = data.serialize(Serializer).unwrap();

        debug_assert!(self.exist_field(&value));

        let mut value_map = ValueMap::new(value);

        self.inner_validate(&mut value_map).ok()
    }

    /// run validate with modifiable
    pub fn validate_mut<'de, T, M2>(self, data: T) -> Result<T, ValidatorError<M2>>
    where
        T: Serialize + serde::de::Deserialize<'de>,
        M2: IntoMessage,
    {
        let value = data.serialize(Serializer).unwrap();

        debug_assert!(self.exist_field(&value));

        let mut value_map = ValueMap::new(value);

        self.inner_validate(&mut value_map)
            .ok()
            .map(|_| T::deserialize(value_map.value()).unwrap())
    }

    /// inner creating message by field name and current value.
    fn inner_validate<M2>(self, value_map: &mut ValueMap) -> ValidatorError<M2>
    where
        M2: IntoMessage,
    {
        self.iter_validate(value_map, |rules, data, _| rules.call_gen_message(data))
    }
}

impl<'v, M> Validator<'v, M> {
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
    pub fn message<const N: usize, Msg>(mut self, list: [(&'v str, Msg); N]) -> Self
    where
        Msg: Into<M>,
    {
        self.message.extend(list.map(|(key_str, v)| {
            let msg_key = panic_on_err!(field_name::parse_message(key_str));

            debug_assert!(self.exit_message(&msg_key));

            (msg_key, v.into())
        }));
        self
    }

    /// # convert `Validator<M1>` to `Validator<M2>`
    ///
    /// Using build-in rules and returning custom validator message type is able:
    /// ```rust
    /// # use valitron::{Validator, available::{Message, MessageKind, Required}};
    /// let validator = Validator::new()
    ///     .rule("introduce", Required)
    ///     .map(MyError::from)
    ///     .message([("introduce.required", MyError::IntroduceRequired)]);
    ///
    /// enum MyError {
    ///     IntroduceRequired,
    ///     NotReset,
    /// }
    ///
    /// impl From<Message> for MyError {
    ///     fn from(val: Message) -> Self {
    ///         match val.kind() {
    ///             MessageKind::Required => MyError::NotReset,
    ///             // ...
    ///             # _ => unreachable!(),
    ///         }
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn map<M2>(self, f: fn(message: M) -> M2) -> Validator<'v, M2>
    where
        M: 'static,
        M2: 'static,
    {
        Validator {
            rules: self
                .rules
                .into_iter()
                .map(|(field, list)| (field, list.map(f)))
                .collect(),
            message: self
                .message
                .into_iter()
                .map(|(key, msg)| (key, f(msg)))
                .collect(),
            is_bail: self.is_bail,
        }
    }
}

impl<M, List> Default for InnerValidator<M, List>
where
    List: Default,
{
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
            message: List::default(),
            is_bail: false,
        }
    }
}

impl<M, List> Clone for InnerValidator<M, List>
where
    List: Clone,
{
    fn clone(&self) -> Self {
        Self {
            rules: self.rules.clone(),
            message: self.message.clone(),
            is_bail: self.is_bail,
        }
    }
}

impl<M, List> InnerValidator<M, List> {
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
    /// fields's BNF:
    /// ```bnf
    /// fields                 ::= <tuple_index>
    ///                          | <array_index>
    ///                          | <ident>
    ///                          | <struct_variant_index>
    ///                          | <fields> '.' <tuple_index>
    ///                          | <fields> '.' <ident>
    ///                          | <fields> <array_index>
    ///                          | <fields> <struct_variant_index>
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
        let mut rules = rule.into_list();

        debug_assert!(rules.valid_name(), "invalid rule name");

        self.rules
            .entry(names)
            .and_modify(|list| list.merge(&mut rules))
            .or_insert(rules);
        self
    }

    /// when first validate error is encountered, right away return Err(message).
    pub fn bail(mut self) -> Self {
        self.is_bail = true;
        self
    }

    fn exist_field(&self, value: &Value) -> bool {
        for (field, _) in self.rules.iter() {
            if value.get_with_names(field).is_none() {
                panic!("field `{}` is not found", field.as_str());
            }
        }

        true
    }

    #[inline(always)]
    fn rule_get(&self, names: &FieldNames) -> Option<&RuleList<M>> {
        self.rules.get(names)
    }

    fn iter_validate<F, T>(self, value_map: &mut ValueMap, handle_msg: F) -> ValidatorError<T>
    where
        F: Fn(RuleList<M>, &mut ValueMap, &mut List) -> Vec<T>,
    {
        let mut resp_message = ValidatorError::with_capacity(self.rules.len());

        let Self {
            rules,
            mut message,
            is_bail,
        } = self;

        for (mut names, mut rules) in rules.into_iter() {
            if is_bail {
                rules.set_bail();
            }

            value_map.index(names);

            let field_msg = handle_msg(rules, value_map, &mut message);

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

    #[cfg(test)]
    pub(crate) fn get_message(&self) -> &List {
        &self.message
    }
}

impl<M> From<Validator<'_, M>> for ValidatorRefine<M> {
    fn from(value: Validator<'_, M>) -> Self {
        let Validator { rules, is_bail, .. } = value;
        Self {
            rules,
            message: (),
            is_bail,
        }
    }
}

/// validateable for more types
pub trait Validatable<V, E> {
    /// if not change value
    fn validate(&self, validator: V) -> Result<(), E>;

    /// if need to change value, e.g. `trim`
    fn validate_mut<'de>(self, validator: V) -> Result<Self, E>
    where
        Self: Deserialize<'de>;
}

impl<T, M> Validatable<Validator<'_, M>, ValidatorError<M>> for T
where
    T: Serialize,
    M: 'static,
{
    fn validate(&self, validator: Validator<M>) -> Result<(), ValidatorError<M>> {
        validator.validate(self)
    }

    fn validate_mut<'de>(self, validator: Validator<M>) -> Result<Self, ValidatorError<M>>
    where
        Self: Deserialize<'de>,
    {
        validator.validate_mut(self)
    }
}

impl<T, M, M2> Validatable<ValidatorRefine<M>, ValidatorError<M2>> for T
where
    T: Serialize,
    M: 'static,
    M2: IntoMessage,
{
    fn validate(&self, validator: ValidatorRefine<M>) -> Result<(), ValidatorError<M2>> {
        validator.validate(self)
    }

    fn validate_mut<'de>(self, validator: ValidatorRefine<M>) -> Result<Self, ValidatorError<M2>>
    where
        Self: Deserialize<'de>,
    {
        validator.validate_mut(self)
    }
}

/// store validate error message
pub struct ValidatorError<M> {
    message: HashMap<FieldNames, Vec<M>>,
}

impl<M: Clone> Clone for ValidatorError<M> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
        }
    }
}

impl<M: PartialEq<M>> PartialEq<Self> for ValidatorError<M> {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl<M> std::fmt::Debug for ValidatorError<M>
where
    M: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidatorError")
            .field("message", &self.message)
            .finish()
    }
}

impl<M> Index<&str> for ValidatorError<M> {
    type Output = Vec<M>;

    fn index(&self, index: &str) -> &Self::Output {
        self.message
            .get(&(index.into()))
            .expect("this field is not found")
    }
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

    fn shrink_to(&mut self, min_capacity: usize) {
        self.message.shrink_to(min_capacity)
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

    /// `ValidatorError<M1>` convert to `ValidatorError<M2>`
    pub fn map<M2>(self, f: fn(M) -> M2) -> ValidatorError<M2> {
        ValidatorError {
            message: self
                .message
                .into_iter()
                .map(|(name, msg)| (name, msg.into_iter().map(f).collect()))
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.message.is_empty()
    }

    pub fn len(&self) -> usize {
        self.message.len()
    }

    /// total length of the message
    pub fn total(&self) -> usize {
        self.message.values().map(|msg| msg.len()).sum()
    }

    fn ok(self) -> Result<(), Self> {
        if self.message.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MessageKey<'key> {
    fields: FieldNames,
    rule: &'key str,
}

impl Hash for MessageKey<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fields.hash(state);
        self.rule.hash(state);
    }
}

impl<'key> MessageKey<'key> {
    pub(crate) fn new(fields: FieldNames, rule: &'key str) -> Self {
        Self { fields, rule }
    }
}

#[test]
#[cfg(feature = "full")]
fn test_refine() {
    use crate::available::Required;
    let _ = ValidatorRefine::new().rule("foo", Required);
}
