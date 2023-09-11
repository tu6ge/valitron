//! define Rule trait, inner rule type

use std::slice::Iter;

use crate::value::{FromValue, Value, ValueMap};

use self::boxed::{ErasedRule, RuleIntoBoxed};

#[cfg(feature = "full")]
pub mod available;
mod boxed;

/// Trait used by creating Rule
///
/// # Example
/// ```rust
/// # use valitron::{Rule, ValueMap};
/// #[derive(Clone)]
/// struct HelloWorld;
///
/// impl Rule<()> for HelloWorld {
///     type Message = String;
///
///     fn name(&self) -> &'static str {
///         "hello"
///     }
///
///     fn call(&mut self, data: &mut ValueMap) -> Result<(), Self::Message> {
///         Ok(())
///     }
/// }
/// ```
///
/// TODO! introduce ValueMap
pub trait Rule<M>: 'static + Sized + Clone {
    /// custom define returning message type
    type Message: IntoRuleMessage;

    /// Named rule type, allow `a-z` | `A-Z` | `0-9` | `_`, and not start with `0-9`
    fn name(&self) -> &'static str;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Self::Message>;

    #[doc(hidden)]
    fn into_boxed(self) -> RuleIntoBoxed<Self, M> {
        RuleIntoBoxed::new(self)
    }
}

/// Error message returned when validate fail
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Message {
    code: u8,
    content: String,
}

impl Message {
    pub fn new(code: u8, content: String) -> Self {
        Message { code, content }
    }

    pub fn from_content(content: String) -> Self {
        Self { code: 0, content }
    }

    pub fn from_code(code: u8) -> Self {
        Self {
            code,
            content: String::default(),
        }
    }
}

impl From<String> for Message {
    fn from(content: String) -> Self {
        Self { code: 0, content }
    }
}
impl From<Message> for String {
    fn from(msg: Message) -> Self {
        msg.content
    }
}
impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Self {
            code: 0,
            content: value.to_owned(),
        }
    }
}

impl PartialEq<Message> for String {
    fn eq(&self, other: &Message) -> bool {
        self == &other.content
    }
}
impl PartialEq<Message> for u8 {
    fn eq(&self, other: &Message) -> bool {
        self == &other.code
    }
}

pub trait IntoRuleMessage {
    fn into_message(self) -> Message;
}

impl IntoRuleMessage for Message {
    fn into_message(self) -> Message {
        self
    }
}

impl IntoRuleMessage for (u8, String) {
    fn into_message(self) -> Message {
        Message {
            code: self.0,
            content: self.1,
        }
    }
}

impl IntoRuleMessage for (u8, &str) {
    fn into_message(self) -> Message {
        Message {
            code: self.0,
            content: self.1.to_owned(),
        }
    }
}

impl IntoRuleMessage for u8 {
    fn into_message(self) -> Message {
        Message {
            code: self,
            content: String::default(),
        }
    }
}

impl IntoRuleMessage for String {
    fn into_message(self) -> Message {
        Message {
            code: 0,
            content: self,
        }
    }
}

impl IntoRuleMessage for &str {
    fn into_message(self) -> Message {
        Message {
            code: 0,
            content: self.to_owned(),
        }
    }
}

/// Rule extension, it contains some rules, such as
/// ```rust,ignore
/// Rule1.and(Rule2).and(Rule3)
/// ```
pub trait RuleExt {
    fn and<R: Rule<()> + Clone>(self, other: R) -> RuleList;
    fn custom<F, V, M>(self, other: F) -> RuleList
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), M> + 'static + Clone,
        F: Rule<V>,
        V: FromValue + 'static,
        M: IntoRuleMessage;
}

impl<R: Rule<()> + Clone> RuleExt for R {
    fn and<R2: Rule<()> + Clone>(self, other: R2) -> RuleList {
        RuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }
    fn custom<F, V, M>(self, other: F) -> RuleList
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), M> + 'static + Clone,
        F: Rule<V>,
        V: FromValue + 'static,
        M: IntoRuleMessage,
    {
        RuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }
}

/// Rules collection
#[derive(Default, Clone)]
pub struct RuleList {
    list: Vec<ErasedRule>,
    is_bail: bool,
}

impl RuleList {
    pub fn and<R: Rule<()> + Clone>(mut self, other: R) -> Self {
        self.list.push(ErasedRule::new(other));
        self
    }
    pub fn custom<F, V>(mut self, other: F) -> Self
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), String> + 'static + Clone,
        F: Rule<V>,
        V: FromValue + 'static,
    {
        self.list.push(ErasedRule::new(other));
        self
    }

    pub fn bail(mut self) -> Self {
        self.is_bail = true;
        self
    }

    #[must_use]
    pub(crate) fn call(mut self, data: &mut ValueMap) -> Vec<(&'static str, Message)> {
        let mut msg = Vec::new();
        for endpoint in self.list.iter_mut() {
            let _ = endpoint
                .call(data)
                .map_err(|e| msg.push((endpoint.name(), e)));
            if self.is_bail && !msg.is_empty() {
                return msg;
            }
        }
        msg
    }

    fn iter(&self) -> Iter<'_, ErasedRule> {
        self.list.iter()
    }

    /// check the rule name is existing
    pub(crate) fn contains(&self, rule: &str) -> bool {
        self.iter()
            .map(|endpoint| endpoint.name())
            .any(|name| name == rule)
    }

    /// check all rule names is valid or not
    pub(crate) fn valid_name(&self) -> bool {
        self.iter().map(|endpoint| endpoint.name()).all(|name| {
            let mut chares = name.chars();
            let first = match chares.next() {
                Some(ch) => ch,
                None => return false,
            };

            if !(first.is_ascii_alphabetic() || first == '_') {
                return false;
            }

            loop {
                match chares.next() {
                    Some(ch) if ch.is_ascii_alphanumeric() || ch == '_' => (),
                    None => break true,
                    _ => break false,
                }
            }
        })
    }
}

pub trait IntoRuleList {
    fn into_list(self) -> RuleList;
}

pub fn custom<F, V, M>(f: F) -> RuleList
where
    F: for<'a> FnOnce(&'a mut V) -> Result<(), M> + 'static + Clone,
    F: Rule<V>,
    V: FromValue + 'static,
    M: IntoRuleMessage,
{
    RuleList {
        list: vec![ErasedRule::new(f)],
        ..Default::default()
    }
}

impl IntoRuleList for RuleList {
    fn into_list(self) -> Self {
        self
    }
}
impl<R> IntoRuleList for R
where
    R: Rule<()> + Clone,
{
    fn into_list(self) -> RuleList {
        RuleList {
            list: vec![ErasedRule::new(self)],
            ..Default::default()
        }
    }
}

#[cfg(all(test, feature = "full"))]
mod test_regster {
    use super::available::*;
    use super::*;
    fn register<R: IntoRuleList>(_: R) {}

    fn hander(_val: &mut ValueMap) -> Result<(), String> {
        Ok(())
    }
    fn hander2(_val: &mut Value) -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn test() {
        register(Required);
        register(Required.custom(hander2));
        register(Required.custom(hander));
        register(Required.and(StartWith("foo")));
        register(Required.and(StartWith("foo")).bail());
        register(Required.and(StartWith("foo")).custom(hander2).bail());
        register(
            Required
                .and(StartWith("foo"))
                .custom(hander2)
                .custom(hander)
                .bail(),
        );
        register(custom(hander2));
        register(custom(hander));
        register(custom(hander).and(StartWith("foo")));
        register(custom(hander).and(StartWith("foo")).bail());
        register(custom(|_a: &mut u8| Ok::<_, u8>(())));
        register(custom(|_a: &mut u8| Ok::<_, u8>(())));
    }
}

pub trait RuleShortcut {
    /// custom define returning message type
    type Message: IntoRuleMessage;

    /// Named rule type
    fn name(&self) -> &'static str;

    /// Default rule error message, when validate fails, return the message to user
    fn message(&self) -> Self::Message;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    /// when the method return true, call_message will return Ok(()), or else return Err(String)
    ///
    /// *Panic*
    /// when not found value
    fn call_with_relate(&mut self, data: &mut ValueMap) -> bool {
        self.call(data.current_mut().expect("not found value with fields"))
    }

    /// Rule specific implementation, data is current field's value
    fn call(&mut self, data: &mut Value) -> bool;
}

impl<T> Rule<()> for T
where
    T: RuleShortcut + 'static + Clone,
{
    type Message = T::Message;

    fn name(&self) -> &'static str {
        self.name()
    }
    /// Rule specific implementation, data is gived type all field's value, and current field index.
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Self::Message> {
        if self.call_with_relate(data) {
            Ok(())
        } else {
            Err(self.message())
        }
    }
}

impl<F, V, M> Rule<V> for F
where
    F: for<'a> FnOnce(&'a mut V) -> Result<(), M> + 'static + Clone,
    V: FromValue,
    M: IntoRuleMessage,
{
    type Message = M;
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Self::Message> {
        let val = V::from_value(data).expect("argument type can not be matched");
        self.clone()(val)
    }
    fn name(&self) -> &'static str {
        "custom"
    }
}
