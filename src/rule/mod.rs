//! define Rule trait, and build-in rule types
//! # A custom rule example
//! ```rust
//! # use valitron::{Value, Rule};
//! #[derive(Clone)]
//! struct Gt10;
//!
//! impl Rule for Gt10 {
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

use std::{collections::HashMap, fmt::Display, slice::Iter};

use crate::{
    register::IntoMessage,
    value::{FromValue, Value, ValueMap},
};

use self::boxed::{ErasedRule, RuleIntoBoxed};

#[cfg(feature = "full")]
pub mod available;
mod boxed;
pub mod string;

#[cfg(test)]
mod test;

/// Trait used by creating CoreRule
///
/// # Example
/// ```rust
/// # use valitron::{rule::CoreRule, ValueMap};
/// #[derive(Clone)]
/// struct Gt10;
///
/// impl CoreRule<ValueMap, ()> for Gt10 {
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
pub trait CoreRule<I, T>: 'static + Sized + Clone {
    /// custom define returning message type
    type Message;

    /// Named rule type, used to distinguish between different rules.
    ///
    /// allow `a-z` | `A-Z` | `0-9` | `_` composed string, and not start with `0-9`
    const THE_NAME: &'static str;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    ///
    /// success returning Ok(()), or else returning message.
    fn call(&mut self, data: &mut I) -> Result<(), Self::Message>;

    #[doc(hidden)]
    fn into_boxed(self) -> RuleIntoBoxed<Self, Self::Message, T> {
        RuleIntoBoxed::new(self)
    }
}

mod private {
    use super::CoreRule;

    pub trait Sealed<I> {}

    impl<R, I> Sealed<I> for R where R: CoreRule<I, ()> {}
}

/// Rule extension, it can coupling some rules, such as
/// ```rust,ignore
/// Rule1.and(Rule2).and(Rule3)
/// ```
pub trait RuleExt<Input, Msg>: private::Sealed<Input> {
    fn and<R>(self, other: R) -> RuleList<Input, Msg>
    where
        R: CoreRule<Input, (), Message = Msg>;

    fn custom<F, V>(self, other: F) -> RuleList<Input, Msg>
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), Msg>,
        F: CoreRule<Input, V, Message = Msg>,
        V: FromValue + 'static;
}

impl<R, Input, Msg> RuleExt<Input, Msg> for R
where
    R: CoreRule<Input, (), Message = Msg>,
    Msg: 'static,
{
    fn and<R2>(self, other: R2) -> RuleList<Input, Msg>
    where
        R2: CoreRule<Input, (), Message = Msg>,
    {
        let is_dup = {
            if R::THE_NAME != R2::THE_NAME {
                false
            } else {
                !matches!(R::THE_NAME, "custom")
            }
        };
        RuleList {
            list: if is_dup {
                vec![ErasedRule::new(self)]
            } else {
                vec![ErasedRule::<Input, Msg>::new(self), ErasedRule::new(other)]
            },
            ..Default::default()
        }
    }

    fn custom<F, V>(self, other: F) -> RuleList<Input, Msg>
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), Msg>,
        F: CoreRule<Input, V, Message = Msg>,
        V: FromValue + 'static,
    {
        RuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }
}

/// Rules collection
pub struct RuleList<I, M> {
    pub(crate) list: Vec<ErasedRule<I, M>>,
    is_bail: bool,
}

impl<I, M> Default for RuleList<I, M> {
    fn default() -> Self {
        Self {
            list: Vec::new(),
            is_bail: false,
        }
    }
}

impl<I, M> Clone for RuleList<I, M> {
    fn clone(&self) -> Self {
        Self {
            list: self.list.clone(),
            is_bail: self.is_bail,
        }
    }
}

impl<I, M> RuleList<I, M> {
    pub fn remove_duplicate(&mut self, other: &ErasedRule<I, M>) {
        let name = other.name();

        let duplicate_rules: Vec<usize> = self
            .list
            .iter()
            .enumerate()
            .filter(|(_index, exist_rule)| {
                if exist_rule.name() != name {
                    return false;
                }
                !matches!(name, "custom")
            })
            .map(|(index, _)| index)
            .rev()
            .collect();

        for index in duplicate_rules {
            // Use `swap_remove` to get better performence because we don't
            // mind the order of rule list. If the order should be kept in
            // the future, please use `remove` instead of `swap_remove`.
            self.list.swap_remove(index);
        }
    }

    pub fn and<R>(mut self, other: R) -> Self
    where
        R: CoreRule<I, (), Message = M>,
        M: 'static,
    {
        let other = ErasedRule::new(other);
        self.remove_duplicate(&other);

        self.list.push(other);
        self
    }

    pub fn custom<F, V>(mut self, other: F) -> Self
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), M>,
        F: CoreRule<I, V, Message = M>,
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

    pub(crate) fn merge(&mut self, other: &mut RuleList<I, M>) {
        for new_rule in &other.list {
            self.remove_duplicate(new_rule);
        }

        self.list.append(&mut other.list);
        self.is_bail = self.is_bail || other.is_bail;
    }

    fn iter(&self) -> Iter<'_, ErasedRule<I, M>> {
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
    pub(crate) fn map<M2>(self, f: fn(M) -> M2) -> RuleList<I, M2>
    where
        M: 'static,
        M2: 'static,
        I: 'static,
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

impl<M> RuleList<ValueMap, M> {
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

    pub(crate) fn call_string_message<'m>(
        self,
        data: &mut ValueMap,
        message: &HashMap<&'m str, &'m str>,
    ) -> Vec<String>
    where
        M: Display,
    {
        fn replace(s: &str, field: &str, value: &str) -> String {
            let s = s.replace("{field}", field);
            s.replace("{value}", value)
        }

        let RuleList { mut list, .. } = self;
        let mut msg = Vec::with_capacity(list.len());

        for endpoint in list.iter_mut() {
            let _ = endpoint.call(data).map_err(|def_msg| {
                let string = def_msg.to_string();
                let mes = *(message.get(endpoint.name())).unwrap_or(&string.as_str());
                let value = data.current().unwrap();
                //let field = data.index;
                msg.push(replace(mes, data.index.as_str(), &value.to_string()))
            });

            if self.is_bail && !msg.is_empty() {
                msg.shrink_to(1);
                return msg;
            }
        }

        msg.shrink_to_fit();
        msg
    }
}

impl<M> RuleList<String, M> {
    pub(crate) fn from_fn<F>(f: F) -> RuleList<String, M>
    where
        F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
        M: 'static,
    {
        RuleList {
            list: vec![ErasedRule::new(f)],
            ..Default::default()
        }
    }

    pub(crate) fn append_fn<S, F>(one: S, fun: F) -> RuleList<String, M>
    where
        S: CoreRule<String, (), Message = M>,
        F: FnOnce(&mut String) -> Result<(), M> + Clone + 'static,
        M: 'static,
    {
        RuleList {
            list: vec![ErasedRule::new(one), ErasedRule::new(fun)],
            ..Default::default()
        }
    }

    pub(crate) fn from_ext_and<S, S2>(one: S, two: S2) -> RuleList<String, M>
    where
        S: CoreRule<String, (), Message = M>,
        S2: CoreRule<String, (), Message = M>,
        M: 'static,
    {
        let is_dup = {
            if S::THE_NAME != S2::THE_NAME {
                false
            } else {
                !matches!(S::THE_NAME, "custom")
            }
        };
        RuleList {
            list: if is_dup {
                vec![ErasedRule::new(one)]
            } else {
                vec![ErasedRule::<String, M>::new(one), ErasedRule::new(two)]
            },
            ..Default::default()
        }
    }

    #[must_use]
    pub(crate) fn call(self, data: &mut String) -> Vec<M> {
        let RuleList { mut list, is_bail } = self;
        let mut msg = Vec::with_capacity(list.len());

        for endpoint in list.iter_mut() {
            let _ = endpoint.call(data).map_err(|m| msg.push(m));

            if is_bail && !msg.is_empty() {
                msg.shrink_to(1);
                return msg;
            }
        }

        msg.shrink_to_fit();
        msg
    }
}

pub trait IntoRuleList<I, M> {
    fn into_list(self) -> RuleList<I, M>;
}

/// load closure rule
pub fn custom<F, V, Input, Msg>(f: F) -> RuleList<Input, Msg>
where
    F: FnOnce(&mut V) -> Result<(), Msg>,
    F: CoreRule<Input, V, Message = Msg>,
    V: FromValue + 'static,
    Msg: 'static,
{
    RuleList {
        list: vec![ErasedRule::new(f)],
        ..Default::default()
    }
}

impl<I, M> IntoRuleList<I, M> for RuleList<I, M> {
    fn into_list(self) -> Self {
        self
    }
}
impl<R, M> IntoRuleList<ValueMap, M> for R
where
    R: CoreRule<ValueMap, (), Message = M>,
    M: 'static,
{
    fn into_list(self) -> RuleList<ValueMap, M> {
        RuleList {
            list: vec![ErasedRule::new(self)],
            ..Default::default()
        }
    }
}

impl<R, M> IntoRuleList<String, M> for R
where
    R: CoreRule<String, (), Message = M>,
    M: 'static,
{
    fn into_list(self) -> RuleList<String, M> {
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
    fn register<R: IntoRuleList<ValueMap, M>, M>(_: R) {}
    fn register2<R: IntoRuleList<ValueMap, Message>>(_: R) {}

    fn hander(_val: &mut ValueMap) -> Result<(), Message> {
        Ok(())
    }
    fn hander2(_val: &mut Value) -> Result<(), Message> {
        Ok(())
    }

    #[derive(Clone)]
    struct Gt10;

    impl Rule for Gt10 {
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
pub trait Rule: Clone {
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

impl<T> CoreRule<ValueMap, ()> for T
where
    T: Rule + 'static + Clone,
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

impl<F, V, M> CoreRule<ValueMap, V> for F
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
