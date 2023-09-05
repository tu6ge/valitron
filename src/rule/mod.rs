//! define Rule trait, inner rule type

use std::marker::PhantomData;

use crate::ser::{Value, ValueMap};

use boxed::BoxCloneRule;

mod boxed;

/// A Rule trait
pub trait Rule<M>: 'static {
    /// Named rule type, allow `a-z` | `A-Z` | `0-9` | `_`, and not start with `0-9`
    fn name(&self) -> &'static str;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message<M>>;
}

/// Error message returned when validation fails
pub struct Message<T> {
    inner: String,
    _marker: PhantomData<fn() -> T>,
}

impl<T> From<String> for Message<T> {
    fn from(inner: String) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
impl<T> From<Message<T>> for String {
    fn from(msg: Message<T>) -> Self {
        msg.inner
    }
}
impl<T> From<&str> for Message<T> {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_owned(),
            _marker: PhantomData,
        }
    }
}

/// Rule extension, it contains some rules, such as
/// ```rust,ignore
/// Rule1.and(Rule2).and(Rule3)
/// ```
pub trait RuleExt {
    fn and<R: Rule<()> + Clone>(self, other: R) -> RuleList;
    fn custom<R2: Rule<Value> + Clone>(self, other: R2) -> RuleList;
    fn relate<R2: Rule<ValueMap> + Clone>(self, other: R2) -> RuleList;
}

impl<R: Rule<()> + Clone> RuleExt for R {
    fn and<R2: Rule<()> + Clone>(self, other: R2) -> RuleList {
        RuleList {
            list: vec![
                Endpoint::Rule(BoxCloneRule::new(self)),
                Endpoint::Rule(BoxCloneRule::new(other)),
            ],
            ..Default::default()
        }
    }
    fn custom<R2: Rule<Value> + Clone>(self, other: R2) -> RuleList {
        RuleList {
            list: vec![
                Endpoint::Rule(BoxCloneRule::new(self)),
                Endpoint::HanderRule(BoxCloneRule::new(other)),
            ],
            ..Default::default()
        }
    }
    fn relate<R2: Rule<ValueMap> + Clone>(self, other: R2) -> RuleList {
        RuleList {
            list: vec![
                Endpoint::Rule(BoxCloneRule::new(self)),
                Endpoint::RelateRule(BoxCloneRule::new(other)),
            ],
            ..Default::default()
        }
    }
}

#[derive(Clone)]
enum Endpoint {
    Rule(BoxCloneRule<()>),
    HanderRule(BoxCloneRule<Value>),
    RelateRule(BoxCloneRule<ValueMap>),
}

impl Endpoint {
    fn name(&self) -> &'static str {
        match self {
            Endpoint::Rule(b) => b.name(),
            Endpoint::HanderRule(b) => b.name(),
            Endpoint::RelateRule(b) => b.name(),
        }
    }
}

/// Rules collection
#[derive(Default, Clone)]
pub struct RuleList {
    list: Vec<Endpoint>,
    is_bail: bool,
}

impl RuleList {
    pub fn and<R: Rule<()> + Clone>(mut self, other: R) -> Self {
        self.list.push(Endpoint::Rule(BoxCloneRule::new(other)));
        self
    }
    pub fn custom<R: Rule<Value> + Clone>(mut self, other: R) -> Self {
        self.list
            .push(Endpoint::HanderRule(BoxCloneRule::new(other)));
        self
    }

    pub fn relate<R: Rule<ValueMap> + Clone>(mut self, other: R) -> Self {
        self.list
            .push(Endpoint::RelateRule(BoxCloneRule::new(other)));
        self
    }

    pub fn bail(mut self) -> Self {
        self.is_bail = true;
        self
    }

    #[must_use]
    pub(crate) fn call(mut self, data: &mut ValueMap) -> Vec<(&'static str, String)> {
        let mut msg = Vec::new();
        for endpoint in self.list.iter_mut() {
            match endpoint {
                Endpoint::Rule(rule) => {
                    let _ = rule
                        .call(data)
                        .map_err(|m| msg.push((rule.name(), m.into())));
                }
                Endpoint::HanderRule(handle) => {
                    let _ = handle
                        .call(data)
                        .map_err(|m| msg.push((handle.name(), m.into())));
                }
                Endpoint::RelateRule(handle) => {
                    let _ = handle
                        .call(data)
                        .map_err(|m| msg.push((handle.name(), m.into())));
                }
            }
            if self.is_bail && !msg.is_empty() {
                return msg;
            }
        }
        msg
    }

    pub(crate) fn get_rules_name(&self) -> Vec<&'static str> {
        self.list.iter().map(|endpoint| endpoint.name()).collect()
    }
}

pub trait IntoRuleList {
    fn into_list(self) -> RuleList;
}

pub fn custom<F>(f: F) -> RuleList
where
    F: for<'a> FnOnce(&'a mut Value) -> Result<(), String> + 'static + Clone,
    F: Rule<Value>,
{
    RuleList {
        list: vec![Endpoint::HanderRule(BoxCloneRule::new(f))],
        ..Default::default()
    }
}
pub fn relate<F>(f: F) -> RuleList
where
    F: for<'a> FnOnce(&'a mut ValueMap) -> Result<(), String> + 'static + Clone,
    F: Rule<ValueMap>,
{
    RuleList {
        list: vec![Endpoint::RelateRule(BoxCloneRule::new(f))],
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
            list: vec![Endpoint::Rule(BoxCloneRule::new(self))],
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test_regster {
    use super::*;
    fn register<R: IntoRuleList>(rule: R) {}

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
        register(Required.relate(hander));
        register(Required.and(StartWith("foo")));
        register(Required.and(StartWith("foo")).bail());
        register(Required.and(StartWith("foo")).custom(hander2).bail());
        register(Required.and(StartWith("foo")).relate(hander).bail());
        register(
            Required
                .and(StartWith("foo"))
                .custom(hander2)
                .relate(hander)
                .bail(),
        );
        register(custom(hander2));
        register(relate(hander));
        register(relate(hander).and(StartWith("foo")));
        register(relate(hander).and(StartWith("foo")).bail());
        register(custom(|_a| Ok(())));
        register(relate(|_a| Ok(())));
    }
}

#[derive(Clone, Debug)]
pub struct Required;

impl<M> RuleShortcut<M> for Required {
    fn name(&self) -> &'static str {
        "required"
    }
    fn message(&self) -> Message<M> {
        "this field is required".into()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::UInt8(_)
            | Value::UInt16(_)
            | Value::UInt32(_)
            | Value::UInt64(_)
            | Value::Int8(_)
            | Value::Int16(_)
            | Value::Int32(_)
            | Value::Int64(_) => true,
            Value::String(s) => !s.is_empty(),
            Value::Struct(_) => true,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StartWith<T>(pub T);

impl<M> RuleShortcut<M> for StartWith<&'static str> {
    fn name(&self) -> &'static str {
        "start_with"
    }
    fn message(&self) -> Message<M> {
        "this field must be start with {}".into()
    }
    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::Int8(_) => false,
            Value::String(s) => s.starts_with(&self.0),
            Value::Struct(_) => false,
            _ => todo!(),
        }
    }
}

// impl Rule for StartWith<char> {
//     fn name(&self) -> &'static str {
//         "start_with"
//     }
//     fn message(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "this field must be start with {}", self.0)
//     }
//     fn call(&mut self, value: &Value, all_data: &Value) -> bool {
//         match value {
//             Value::Int8(_) => false,
//             Value::String(s) => s.starts_with(self.0),
//             Value::Struct(_) => false,
//         }
//     }
// }

// #[derive(Clone)]
// struct Confirm(&'static str);

// impl<M> RuleShortcut<M> for Confirm {
//     fn name(&self) -> &'static str {
//         "confirm"
//     }
//     fn message(&self) -> Message<M> {
//         "this field value must be eq {} field value".into()
//     }
//     fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
//         let target = value.get(self.0);
//         let target = match target {
//             Some(target) if target.is_leaf() => target,
//             _ => return false,
//         };
//         match (value.current().unwrap(), target) {
//             (Value::Int8(v), Value::Int8(ref t)) if v == t => true,
//             (Value::String(v), Value::String(ref t)) if v == t => true,
//             _ => false,
//         }
//     }
//     fn call(&mut self, _value: &mut Value) -> bool {
//         unreachable!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use serde::Serialize;

//     use crate::{
//         rule::{Confirm, Rule},
//         ser::{to_value, Value},
//     };

//     #[test]
//     fn test_confirm() {
//         #[derive(Serialize)]
//         struct MyType {
//             name: String,
//             age: u8,
//         }
//         let my_struct = MyType {
//             name: "wang".into(),
//             age: 18,
//         };

//         let all_value = to_value(my_struct).unwrap();

//         let confirm = Confirm("name");
//         let name = Value::String("wang".into());
//         let res = confirm.call(&name, &all_value);
//         assert!(res);
//     }
// }

pub trait RuleShortcut<T> {
    /// Named rule type
    fn name(&self) -> &'static str;

    /// Default rule error message, when validate fails, return the message to user
    fn message(&self) -> Message<T>;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    /// when the method return true, call_message will return Ok(()), or else return Err(String)
    fn call_with_relate(&mut self, data: &mut ValueMap) -> bool {
        // TODO unwrap
        let value = data.current_mut().unwrap();
        self.call(value)
    }

    /// Rule specific implementation, data is current field's value
    fn call(&mut self, data: &mut Value) -> bool;
}

impl<T> Rule<()> for T
where
    T: RuleShortcut<()> + 'static,
{
    fn name(&self) -> &'static str {
        self.name()
    }
    /// Rule specific implementation, data is gived type all field's value, and current field index.
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message<()>> {
        if self.call_with_relate(data) {
            Ok(())
        } else {
            Err(self.message())
        }
    }
}

impl<F> Rule<ValueMap> for F
where
    F: for<'a> FnOnce(&'a mut ValueMap) -> Result<(), String> + 'static + Clone,
{
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message<ValueMap>> {
        self.clone()(data).map_err(|s| s.into())
    }

    fn name(&self) -> &'static str {
        "relate"
    }
}

impl<F> Rule<Value> for F
where
    F: for<'a> FnOnce(&'a mut Value) -> Result<(), String> + 'static + Clone,
{
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message<Value>> {
        // TODO unwrap
        let value = data.current_mut().unwrap();
        self.clone()(value).map_err(|e| e.into())
    }

    fn name(&self) -> &'static str {
        "custom"
    }
}
