use std::{collections::HashMap, error::Error, fmt::Display};

pub mod rule;
pub mod ser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// data
// rule
// message
pub trait Validator<Rule> {
    type Error;
    fn validator(&self, rule: Rule, list: &MessageList) -> Result<(), Self::Error>;
}

/// 需要一个返回bool 值的运算
/// 这个运算参数
///
/// 需要一个类型保存错误信息
fn check_is_zero(data: u8) -> Result<(), String> {
    if data == 0 {
        Ok(())
    } else {
        Err("message".into())
    }
}

trait Rule<Data> {
    fn check(&self, other: &Data) -> bool;
}

struct IsZero;

impl<Data: PartialEq<u8>> Rule<Data> for IsZero {
    fn check(&self, other: &Data) -> bool {
        *other == 0
    }
}

struct MyData(u8);

impl Display for MyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Validator<IsZero> for MyData {
    type Error = String;
    fn validator(&self, rule: IsZero, list: &MessageList) -> Result<(), Self::Error> {
        rule.check(&self.0);

        todo!()
    }
}

// impl<Msg, T: Fn() -> Result<(), Msg>> Rule<Msg> for T {
//     fn check(&mut self) -> Result<(), Msg> {
//         self()
//     }
// }

// impl<Rhs, T: PartialEq<Rhs>> Rule<Rhs> for T {
//     fn check(&self, other: &Rhs) -> bool {
//         self.eq(other)
//     }
// }

//
// e.g. firstname.required = "firstname is required"

type MessageList = HashMap<String, String>;

type ResultMessage = Vec<String>;

const A: u8 = 1 | 2;

struct A {
    a: u8,
    b: u8,
}

fn error(str: &str) {
    let a = stringify!(str);
    println!("{a}");
}

#[test]
fn test_error() {
    //error("abc");
    let aaa = "bbb";
}
