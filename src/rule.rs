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
pub trait RuleExt<T> {
    fn and<R: Rule<T>>(self, other: R) -> RuleBox<T>;
}

impl<T, R: Rule<T>> RuleExt<T> for R {
    fn and<R2: Rule<T>>(self, other: R2) -> RuleBox<T> {
        RuleBox {
            list: vec![Box::new(self), Box::new(other)],
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

/// rules collection
pub struct RuleBox<T = ()> {
    list: Vec<Box<dyn Rule<T>>>,
    is_bail: bool,
}

impl<T> RuleBox<T> {
    pub fn and<R: Rule<T>>(self, other: R) -> Self {
        let RuleBox { mut list, is_bail } = self;
        list.push(Box::new(other));
        Self { list, is_bail }
    }

    pub fn bail(self) -> Self {
        let RuleBox { list, .. } = self;
        let is_bail = true;
        Self { list, is_bail }
    }
}

// impl<R: Rule<()>> From<R> for RuleBox<()> {
//     fn from(value: R) -> Self {
//         Self {
//             list: vec![Box::new(value)],
//             is_bail: false,
//         }
//     }
// }

trait IntoRuleBox<T> {
    fn into_rule_box(self) -> RuleBox<T>;
}

impl IntoRuleBox<()> for RuleBox<()> {
    fn into_rule_box(self) -> RuleBox<()> {
        self
    }
}
impl<R, T> IntoRuleBox<T> for R
where
    R: Rule<T>,
{
    fn into_rule_box(self) -> RuleBox<T> {
        RuleBox::<T> {
            list: vec![Box::new(self)],
            is_bail: false,
        }
    }
}

fn register<T, R: IntoRuleBox<T>>(rule: R) {}

fn hander(val: &ValueMap) -> Result<(), String> {
    Ok(())
}
// fn hander2(val: &Value, list: &Value) -> Result<(), String> {
//     Ok(())
// }

fn test() {
    register(Required);
    register(Required.and(StartWith("foo")));
    register(Required.and(StartWith("foo")).bail());
    register(Required.and(StartWith("foo")).and(hander).bail());
    register(hander);
    //register(hander2);
    register(|a: &ValueMap| Ok::<_, String>(()));
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

impl<F> Rule<()> for F
where
    F: for<'a> FnOnce(&'a ValueMap) -> Result<(), String> + 'static + Clone,
{
    fn call(self, data: &ValueMap) -> Result<(), String> {
        self(&data)
    }

    fn name(&self) -> RuleName<()> {
        "custom".into()
    }
    fn message(&self) -> String {
        String::default()
    }
}

// impl<F> Rule<(Value, Value)> for F
// where
//     F: for<'a> FnOnce(&'a Value, &'a Value) -> Result<(), String> + 'static + Clone,
// {
//     fn call(self, data: &ValueMap) -> Result<(), String> {
//         self(&data, &all_data)
//     }

//     fn name(&self) -> RuleName<(Value, Value)> {
//         "custom".into()
//     }
//     fn message(&self) -> String {
//         String::default()
//     }
// }
