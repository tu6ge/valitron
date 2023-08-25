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
    fn call(self, data: &ValueMap) -> Result<(), String>;
}

pub enum Judge {
    Pass,
    Failed(String),
}

/// rule extension, it contains some rules, such as
/// ```no_run
/// Rule1.and(Rule2).and(Rule3)
/// ```
pub trait RuleExt {
    fn and<R: Rule<()>>(self, other: R) -> RuleBox;
    fn custom<R2: Rule<Value>>(self, other: R2) -> RuleBox;
    fn fusion<R2: Rule<ValueMap>>(self, other: R2) -> RuleBox;
}

impl<R: Rule<()>> RuleExt for R {
    fn and<R2: Rule<()>>(self, other: R2) -> RuleBox {
        RuleBox {
            list: vec![
                Endpoint::Rule(Box::new(self)),
                Endpoint::Rule(Box::new(other)),
            ],
            is_bail: false,
        }
    }
    fn custom<R2: Rule<Value>>(self, other: R2) -> RuleBox {
        RuleBox {
            list: vec![
                Endpoint::Rule(Box::new(self)),
                Endpoint::HanderRule(Box::new(other)),
            ],
            is_bail: false,
        }
    }
    fn fusion<R2: Rule<ValueMap>>(self, other: R2) -> RuleBox {
        RuleBox {
            list: vec![
                Endpoint::Rule(Box::new(self)),
                Endpoint::FusionRule(Box::new(other)),
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
    Rule(Box<dyn Rule<()>>),
    HanderRule(Box<dyn Rule<Value>>),
    FusionRule(Box<dyn Rule<ValueMap>>),
}

/// rules collection
pub struct RuleBox {
    list: Vec<Endpoint>,
    is_bail: bool,
}

impl RuleBox {
    pub fn and<R: Rule<()>>(self, other: R) -> Self {
        let RuleBox { mut list, is_bail } = self;
        list.push(Endpoint::Rule(Box::new(other)));
        Self { list, is_bail }
    }
    pub fn custom<R: Rule<Value>>(self, other: R) -> Self {
        let RuleBox { mut list, is_bail } = self;
        list.push(Endpoint::HanderRule(Box::new(other)));
        Self { list, is_bail }
    }

    pub fn fusion<R: Rule<ValueMap>>(self, other: R) -> Self {
        let RuleBox { mut list, is_bail } = self;
        list.push(Endpoint::FusionRule(Box::new(other)));
        Self { list, is_bail }
    }

    pub fn bail(self) -> Self {
        let RuleBox { list, .. } = self;
        let is_bail = true;
        Self { list, is_bail }
    }

    // fn call(self) {
    //     let data = ValueMap{
    //       value: Value::Int8(18),
    //       index: "abc",
    //     };
    //     for endpoint in self.list.iter() {
    //         match endpoint {
    //             Endpoint::Rule(rule) => {
    //               let res = (**rule).call(&data);
    //               ()
    //             },
    //             _ => (),
    //         }
    //     }
    //     "aa";
    // }
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

trait IntoRuleBox {
    fn into_rule_box(self) -> RuleBox;
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
pub fn custom<F>(f: F) -> RuleBox
where
    F: for<'a> FnOnce(&'a Value) -> Result<(), String> + 'static + Clone,
{
    RuleBox {
        list: vec![Endpoint::HanderRule(Box::new(f))],
        is_bail: false,
    }
}
pub fn fusion<F>(f: F) -> RuleBox
where
    F: for<'a> FnOnce(&'a ValueMap) -> Result<(), String> + 'static + Clone,
{
    RuleBox {
        list: vec![Endpoint::FusionRule(Box::new(f))],
        is_bail: false,
    }
}
impl IntoRuleBox for RuleBox {
    fn into_rule_box(self) -> Self {
        self
    }
}
impl<R> IntoRuleBox for R
where
    R: Rule<()>,
{
    fn into_rule_box(self) -> RuleBox {
        RuleBox {
            list: vec![Endpoint::Rule(Box::new(self))],
            is_bail: false,
        }
    }
}

#[cfg(test)]
mod test_regster {
    use super::*;
    fn register<R: IntoRuleBox>(_rule: R) {}

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
        register(Required.fusion(hander));
        register(Required.and(StartWith("foo")));
        register(Required.and(StartWith("foo")).bail());
        register(Required.and(StartWith("foo")).custom(hander2).bail());
        register(Required.and(StartWith("foo")).fusion(hander).bail());
        register(
            Required
                .and(StartWith("foo"))
                .custom(hander2)
                .fusion(hander)
                .bail(),
        );
        register(custom(hander2));
        register(fusion(hander));
        register(fusion(hander).and(StartWith("foo")));
        register(fusion(hander).and(StartWith("foo")).bail());
        register(custom(|_a| Ok(())));
        register(fusion(|_a| Ok(())));
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

    fn call(self, map: &ValueMap) -> Result<(), String> {
        let value = map.current().unwrap();
        let bool = match value {
            Value::Int8(_) => true,
            Value::String(s) => !s.is_empty(),
            Value::Struct(_) => true,
        };

        Rule::<()>::after_call(&self, bool)
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
    fn call(self, map: &ValueMap) -> Result<(), String> {
        let value = map.current().unwrap();
        let bool = match value {
            Value::Int8(_) => false,
            Value::String(s) => s.starts_with(&self.0),
            Value::Struct(_) => false,
        };
        Rule::<()>::after_call(&self, bool)
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
    fn call(self, data: &ValueMap) -> Result<(), String> {
        self(&data)
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
    fn call(self, data: &ValueMap) -> Result<(), String> {
        let value = data.current().unwrap();
        self(value)
    }

    fn name(&self) -> RuleName<Value> {
        "custom".into()
    }
    fn message(&self) -> String {
        String::default()
    }
}
