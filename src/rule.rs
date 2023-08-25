//! define Rule trait, inner rule type

use std::marker::PhantomData;

use crate::ser::{Value, ValueMap};

/// A Rule trait
pub trait Rule<T>: 'static {
    /// named rule type
    fn name(&self) -> RuleName<T>;

    /// default rule error message, when validate fails, return the message to user
    fn message(&self) -> String;

    #[doc(hidden)]
    fn after_call(&self, res: bool) -> Result<(), String> {
        if res {
            Ok(())
        } else {
            Err(self.message())
        }
    }

    /// rule specific implementation, data is current field's value, and all_data is all.
    fn call(&self, data: &ValueMap) -> Result<(), String>;
}

trait CloneRule<T>: Rule<T> {
    fn clone_box(&self) -> Box<dyn CloneRule<T>>;
}

impl<T, R> CloneRule<T> for R
where
    R: Rule<T> + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneRule<T>> {
        Box::new(self.clone())
    }
}

pub struct BoxCloneRule<T>(Box<dyn CloneRule<T>>);

impl<T> BoxCloneRule<T> {
    fn new<R>(rule: R) -> Self
    where
        R: Rule<T> + Clone + 'static,
    {
        BoxCloneRule(Box::new(rule))
    }
}
impl<T: 'static> BoxCloneRule<T> {
    fn call(&self, map: &ValueMap) -> Result<(), String> {
        self.0.call(map)
    }
}

impl<T: 'static> Clone for BoxCloneRule<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

/// rule extension, it contains some rules, such as
/// ```no_run
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
            is_bail: false,
        }
    }
    fn custom<R2: Rule<Value> + Clone>(self, other: R2) -> RuleList {
        RuleList {
            list: vec![
                Endpoint::Rule(BoxCloneRule::new(self)),
                Endpoint::HanderRule(BoxCloneRule::new(other)),
            ],
            is_bail: false,
        }
    }
    fn relate<R2: Rule<ValueMap> + Clone>(self, other: R2) -> RuleList {
        RuleList {
            list: vec![
                Endpoint::Rule(BoxCloneRule::new(self)),
                Endpoint::RelateRule(BoxCloneRule::new(other)),
            ],
            is_bail: false,
        }
    }
}

/// store rule name
pub struct RuleName<T> {
    name: &'static str,
    _marker: PhantomData<fn() -> T>,
}

impl<T> From<&'static str> for RuleName<T> {
    fn from(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }
}

enum Endpoint {
    Rule(BoxCloneRule<()>),
    HanderRule(BoxCloneRule<Value>),
    RelateRule(BoxCloneRule<ValueMap>),
}

/// rules collection
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

    fn call(self, data: &ValueMap) {
        for endpoint in self.list.iter() {
            match endpoint {
                Endpoint::Rule(rule) => {
                    let res = rule.call(&data);
                }
                Endpoint::HanderRule(handle) => {
                    let res = handle.call(&data);
                }
                Endpoint::RelateRule(handle) => {
                    let res = handle.call(&data);
                }
            }
        }
        "aa";
    }
}

// impl<F> From<F> for RuleBox<ValueMap>
// where
//     F: for<'a> FnOnce(&'a ValueMap) -> Result<(), String> + 'static + Clone,
// {
//     fn from(value: F) -> Self {
//         Self {
//             list: vec![Endpoint::HanderRule(Box::new(value))],
//             is_bail: false,
//         }
//     }
// }
// impl<F> From<F> for RuleBox<Value>
// where
//     F: for<'a> FnOnce(&'a Value) -> Result<(), String> + 'static + Clone,
// {
//     fn from(value: F) -> Self {
//         Self {
//             list: vec![Endpoint::HanderRule(Box::new(value))],
//             is_bail: false,
//         }
//     }
// }

trait IntoRuleList {
    fn into_rule_box(self) -> RuleList;
}

// impl IntoRuleBox<ValueMap> for RuleBox<ValueMap> {
//     fn into_rule_box(self) -> RuleBox<ValueMap> {
//         self
//     }
// }
// impl IntoRuleBox<Value> for RuleBox<Value> {
//     fn into_rule_box(self) -> RuleBox<Value> {
//         self
//     }
// }
pub fn custom<F>(f: F) -> RuleList
where
    F: for<'a> FnOnce(&'a Value) -> Result<(), String> + 'static + Clone,
{
    RuleList {
        list: vec![Endpoint::HanderRule(BoxCloneRule::new(f))],
        is_bail: false,
    }
}
pub fn relate<F>(f: F) -> RuleList
where
    F: for<'a> FnOnce(&'a ValueMap) -> Result<(), String> + 'static + Clone,
{
    RuleList {
        list: vec![Endpoint::RelateRule(BoxCloneRule::new(f))],
        is_bail: false,
    }
}
impl IntoRuleList for RuleList {
    fn into_rule_box(self) -> Self {
        self
    }
}
impl<R> IntoRuleList for R
where
    R: Rule<()> + Clone,
{
    fn into_rule_box(self) -> RuleList {
        RuleList {
            list: vec![Endpoint::Rule(BoxCloneRule::new(self))],
            is_bail: false,
        }
    }
}

#[cfg(test)]
mod test_regster {
    use super::*;
    fn register<R: IntoRuleList>(rule: R) {}

    fn hander(_val: &ValueMap) -> Result<(), String> {
        Ok(())
    }
    fn hander2(_val: &Value) -> Result<(), String> {
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
struct Required;

impl Rule<()> for Required {
    fn name(&self) -> RuleName<()> {
        "required".into()
    }
    fn message(&self) -> String {
        "this field is required".into()
    }

    fn call(&self, map: &ValueMap) -> Result<(), String> {
        let value = map.current().unwrap();
        let bool = match value {
            Value::Int8(_) => true,
            Value::String(s) => !s.is_empty(),
            Value::Struct(_) => true,
        };

        Rule::<()>::after_call(self, bool)
    }
}

#[derive(Clone, Debug)]
struct StartWith<T>(T);

impl Rule<()> for StartWith<&'static str> {
    fn name(&self) -> RuleName<()> {
        "start_with".into()
    }
    fn message(&self) -> String {
        "this field must be start with {}".into()
    }
    fn call(&self, map: &ValueMap) -> Result<(), String> {
        let value = map.current().unwrap();
        let bool = match value {
            Value::Int8(_) => false,
            Value::String(s) => s.starts_with(&self.0),
            Value::Struct(_) => false,
        };
        Rule::<()>::after_call(self, bool)
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

// impl Rule for Confirm {
//     fn name(&self) -> &'static str {
//         "confirm"
//     }
//     fn message(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "this field value must be eq {} field value", self.0)
//     }
//     fn call(self, value: &Value, all: &Value) -> bool {
//         let target = all.get(self.0);
//         let target = match target {
//             Some(target) if target.is_leaf() => target,
//             _ => return false,
//         };
//         match (value, target) {
//             (Value::Int8(v), Value::Int8(ref t)) if v == t => true,
//             (Value::String(v), Value::String(ref t)) if v == t => true,
//             _ => false,
//         }
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

impl<F> Rule<ValueMap> for F
where
    F: for<'a> FnOnce(&'a ValueMap) -> Result<(), String> + 'static + Clone,
{
    fn call(&self, data: &ValueMap) -> Result<(), String> {
        self.clone()(&data)
    }

    fn name(&self) -> RuleName<ValueMap> {
        "custom".into()
    }
    fn message(&self) -> String {
        String::default()
    }
}

impl<F> Rule<Value> for F
where
    F: for<'a> FnOnce(&'a Value) -> Result<(), String> + 'static + Clone,
{
    fn call(&self, data: &ValueMap) -> Result<(), String> {
        let value = data.current().unwrap();
        self.clone()(value)
    }

    fn name(&self) -> RuleName<Value> {
        "custom".into()
    }
    fn message(&self) -> String {
        String::default()
    }
}
