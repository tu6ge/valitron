use serde::Serialize;
use valitron::{
    available::{Gt, Required},
    RuleExt, Validator,
};

#[derive(Serialize, Debug)]
struct Address {
    street: String,
    number: u8,
}

#[derive(Serialize, Debug)]
struct Person {
    name: String,
    home: Option<Address>,
}

#[derive(Serialize, Debug)]
struct Person2 {
    name: String,
    age: Option<u8>,
}

pub fn main() {
    let validator = Validator::new()
        .rule("name", Required)
        .rule("home?.number", Required.and(Gt(8_u8)));

    let person = Person {
        name: "Michael".to_string(),
        home: Some(Address {
            street: "Broadway".to_string(),
            number: 10,
        }),
    };

    assert!(validator.validate(person).is_ok());

    let validator2 = Validator::new()
        .rule("name", Required)
        .rule("age?", Required.and(Gt(8_u8)));

    let person2 = Person2 {
        name: "Michael".to_string(),
        age: Some(10_u8),
    };

    assert!(validator2.validate(person2).is_ok());
}
