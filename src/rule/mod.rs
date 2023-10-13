//! define Rule trait, and build-in rule types
//! # A custom rule example
//! ```rust
//! # use valitron::{Value, RuleShortcut};
//! #[derive(Clone)]
//! struct Gt10;
//!
//! impl RuleShortcut for Gt10 {
//!     type Message = &'static str;
//!
//!     const NAME: &'static str = "gt10";
//!
//!     fn message(&self) -> Self::Message {
//!         "the number should be greater than 10"
//!     }
//!
//!     fn call(&mut self, data: &mut Value) -> bool {
//!         data > 10_u8
//!     }
//! }
//! ```

use std::slice::Iter;

use crate::{
    register::IntoMessage,
    value::{FromValue, Value, ValueMap},
};

use self::boxed::{ErasedRule, RuleIntoBoxed};

#[cfg(feature = "full")]
pub mod available;
mod boxed;

#[cfg(test)]
mod test;

/// Trait used by creating Rule
///
/// # Example
/// ```rust
/// # use valitron::{Rule, ValueMap};
/// #[derive(Clone)]
/// struct Gt10;
///
/// impl Rule<()> for Gt10 {
///     type Message = &'static str;
///
///     const THE_NAME: &'static str = "gt10";
///
///     fn call(&mut self, data: &mut ValueMap) -> Result<(), Self::Message> {
///         if data.current().unwrap() > &10 {
///             Ok(())
///         } else {
///             Err("the number should be greater than 10")
///         }
///     }
/// }
/// ```
///
/// TODO! introduce ValueMap
pub trait Rule<T>: 'static + Sized + Clone {
    /// custom define returning message type
    ///
    /// u8 or String or both
    type Message;

    /// Named rule type, used to distinguish between different rules.
    ///
    /// allow `a-z` | `A-Z` | `0-9` | `_` composed string, and not start with `0-9`
    const THE_NAME: &'static str;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    ///
    /// success returning Ok(()), or else returning message.
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Self::Message>;

    #[doc(hidden)]
    fn into_boxed(self) -> RuleIntoBoxed<Self, Self::Message, T> {
        RuleIntoBoxed::new(self)
    }
}

mod private {
    use super::Rule;

    pub trait Sealed {}

    impl<R> Sealed for R where R: Rule<()> {}
}

/// Rule extension, it can coupling some rules, such as
/// ```rust,ignore
/// Rule1.and(Rule2).and(Rule3)
/// ```
pub trait RuleExt<M>: private::Sealed {
    fn and<R>(self, other: R) -> RuleList<M>
    where
        R: Rule<(), Message = M>;

    fn custom<F, V>(self, other: F) -> RuleList<M>
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), M>,
        F: Rule<V, Message = M>,
        V: FromValue + 'static;
}

impl<R, M> RuleExt<M> for R
where
    R: Rule<(), Message = M>,
    M: 'static,
{
    fn and<R2>(self, other: R2) -> RuleList<M>
    where
        R2: Rule<(), Message = M>,
    {
        RuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }

    fn custom<F, V>(self, other: F) -> RuleList<M>
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), M>,
        F: Rule<V, Message = M>,
        V: FromValue + 'static,
    {
        RuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }
}

/// Rules collection
pub struct RuleList<M> {
    list: Vec<ErasedRule<M>>,
    is_bail: bool,
}

impl<M> Default for RuleList<M> {
    fn default() -> Self {
        Self {
            list: Vec::new(),
            is_bail: false,
        }
    }
}

impl<M> Clone for RuleList<M> {
    fn clone(&self) -> Self {
        Self {
            list: self.list.clone(),
            is_bail: self.is_bail,
        }
    }
}

impl<M> RuleList<M> {
    pub fn and<R>(mut self, other: R) -> Self
    where
        R: Rule<(), Message = M>,
        M: 'static,
    {
        self.list.push(ErasedRule::new(other));
        self
    }

    pub fn custom<F, V>(mut self, other: F) -> Self
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), M>,
        F: Rule<V, Message = M>,
        V: FromValue + 'static,
        M: 'static,
    {
        self.list.push(ErasedRule::new(other));
        self
    }

    /// when first validate error is encountered, right away return Err(message) in one field.
    ///
    /// when [`Validator`] set bail, it will cover, and comply with [`Validator`]
    ///
    /// [`Validator`]: crate::Validator
    pub fn bail(mut self) -> Self {
        self.is_bail = true;
        self
    }

    pub(crate) fn set_bail(&mut self) {
        self.is_bail = true;
    }

    pub fn is_bail(&self) -> bool {
        self.is_bail
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub(crate) fn merge(&mut self, other: &mut RuleList<M>) {
        self.list.append(&mut other.list);
        self.is_bail = self.is_bail || other.is_bail;
    }

    #[must_use]
    pub(crate) fn call(self, data: &mut ValueMap) -> Vec<(&'static str, M)> {
        let RuleList { mut list, .. } = self;
        let mut msg = Vec::with_capacity(list.len());

        for endpoint in list.iter_mut() {
            let _ = endpoint
                .call(data)
                .map_err(|e| msg.push((endpoint.name(), e)));

            if self.is_bail && !msg.is_empty() {
                msg.shrink_to(1);
                return msg;
            }
        }

        msg.shrink_to_fit();
        msg
    }

    #[must_use]
    pub(crate) fn call_gen_message<M2>(self, data: &mut ValueMap) -> Vec<M2>
    where
        M2: IntoMessage,
    {
        let RuleList { mut list, .. } = self;
        let mut msg = Vec::with_capacity(list.len());

        for endpoint in list.iter_mut() {
            let _ = endpoint.call(data).map_err(|_| {
                let value = data.current().unwrap();
                msg.push(M2::into_message(endpoint.name(), data.as_index(), value))
            });

            if self.is_bail && !msg.is_empty() {
                msg.shrink_to(1);
                return msg;
            }
        }

        msg.shrink_to_fit();
        msg
    }

    fn iter(&self) -> Iter<'_, ErasedRule<M>> {
        self.list.iter()
    }

    /// check the rule name is existing
    pub(crate) fn contains(&self, rule: &str) -> bool {
        self.iter().map(ErasedRule::name).any(|name| name == rule)
    }

    /// check all rule names is valid or not
    pub(crate) fn valid_name(&self) -> bool {
        self.iter().map(ErasedRule::name).all(|name| {
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

    #[must_use]
    pub(crate) fn map<M2>(self, f: fn(M) -> M2) -> RuleList<M2>
    where
        M: 'static,
        M2: 'static,
    {
        let list = self
            .list
            .into_iter()
            .map(|endpoint| endpoint.map(f))
            .collect();

        RuleList {
            list,
            is_bail: self.is_bail,
        }
    }
}

pub trait IntoRuleList<M> {
    fn into_list(self) -> RuleList<M>;
}

/// load closure rule
pub fn custom<F, V, M>(f: F) -> RuleList<M>
where
    F: for<'a> FnOnce(&'a mut V) -> Result<(), M>,
    F: Rule<V, Message = M>,
    V: FromValue + 'static,
    M: 'static,
{
    RuleList {
        list: vec![ErasedRule::new(f)],
        ..Default::default()
    }
}

impl<M> IntoRuleList<M> for RuleList<M> {
    fn into_list(self) -> Self {
        self
    }
}
impl<R, M> IntoRuleList<M> for R
where
    R: Rule<(), Message = M>,
    M: 'static,
{
    fn into_list(self) -> RuleList<M> {
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
    fn register<R: IntoRuleList<M>, M>(_: R) {}
    fn register2<R: IntoRuleList<Message>>(_: R) {}

    fn hander(_val: &mut ValueMap) -> Result<(), Message> {
        Ok(())
    }
    fn hander2(_val: &mut Value) -> Result<(), Message> {
        Ok(())
    }

    #[derive(Clone)]
    struct Gt10;

    impl RuleShortcut for Gt10 {
        type Message = u8;

        const NAME: &'static str = "gt10";

        fn message(&self) -> Self::Message {
            1
        }
        fn call(&mut self, data: &mut Value) -> bool {
            data > 10_u8
        }
    }

    #[test]
    fn test() {
        assert_eq!(Gt10::NAME, "gt10");
        assert_eq!(Confirm::<&str>::NAME, "confirm");

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

        register(custom(hander));
        register(custom(hander2));
        register2(custom(hander));
        register2(custom(hander2));

        register(custom(hander).and(StartWith("foo")));
        register(custom(hander).and(StartWith("foo")).bail());
        register(custom(|_a: &mut u8| Ok(())).and(Gt10));
        register(Gt10.custom(|_a: &mut u8| Ok(())));
    }
}

/// used by convenient implementation custom rules.
pub trait RuleShortcut {
    /// custom define returning message type
    type Message;

    /// Named rule type, used to distinguish different rules
    ///
    /// allow `a-z` | `A-Z` | `0-9` | `_` composed string, and not start with `0-9`
    const NAME: &'static str;

    /// Default rule error message, when validate fails, return the message to user
    fn message(&self) -> Self::Message;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    /// when the method return true, call_message will return Ok(()), or else return Err(String)
    ///
    /// *Panic*
    /// when not found value
    #[must_use]
    fn call_with_relate(&mut self, data: &mut ValueMap) -> bool {
        self.call(data.current_mut().expect("not found value with fields"))
    }

    /// Rule specific implementation, data is current field's value
    #[must_use]
    fn call(&mut self, data: &mut Value) -> bool;
}

impl<T> Rule<()> for T
where
    T: RuleShortcut + 'static + Clone,
{
    type Message = T::Message;

    const THE_NAME: &'static str = T::NAME;

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
{
    type Message = M;

    const THE_NAME: &'static str = "custom";

    fn call(&mut self, data: &mut ValueMap) -> Result<(), Self::Message> {
        let val = V::from_value(data).expect("argument type can not be matched");
        self.clone()(val)
    }
}
