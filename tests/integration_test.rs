#![cfg(feature = "full")]

use serde::{Deserialize, Serialize};
use valitron::{
    available::{Message, Required, StartWith},
    custom, RuleExt, Validator,
};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: &'static str,
    age: u8,
    weight: f32,
}

#[test]
fn test_validator() {
    let validator = Validator::new()
        .rule("name", Required.and(StartWith("hello")))
        .rule("age", custom(age_limit))
        .rule("weight", custom(weight_limit))
        .message([
            ("name.required", "name is required"),
            ("name.start_with", "name should be starts with `hello`"),
        ]);

    let person = Person {
        name: "li",
        age: 18,
        weight: 20.0,
    };

    let res = validator.validate(person).unwrap_err();

    assert!(res.len() == 3);
    assert_eq!(
        res.get("age").unwrap()[0].to_string(),
        "age should be between 25 and 45"
    );
    assert_eq!(
        res.get("weight").unwrap()[0].to_string(),
        "weight should be between 40 and 80",
    );
    assert_eq!(
        res.get("name").unwrap()[0].to_string(),
        "name should be starts with `hello`",
    );
}

fn age_limit(n: &mut u8) -> Result<(), Message> {
    if *n >= 25 && *n <= 45 {
        return Ok(());
    }
    Err("age should be between 25 and 45".into())
}

fn weight_limit(v: &f32) -> Result<(), Message> {
    if *v >= 40.0 && *v <= 80.0 {
        return Ok(());
    }
    Err("weight should be between 40 and 80".into())
}

#[test]
fn test_has_tuple() {
    let validator = Validator::new()
        .rule(0, StartWith("hello"))
        .message([("0.start_with", "first item should be start with `hello`")]);

    #[derive(Serialize, Deserialize, Debug)]
    struct Foo(&'static str, &'static str);

    let res = validator.validate(Foo("heoo", "bar")).unwrap_err();
    assert!(res.len() == 1);

    assert_eq!(
        res.get(0).unwrap()[0].to_string(),
        "first item should be start with `hello`"
    );
}

#[test]
fn test_has_array() {
    let validator = Validator::new().rule([1], StartWith("hello"));

    let res = validator.validate(vec!["foo", "bar"]).unwrap_err();

    assert!(res.len() == 1);
    assert_eq!(
        res.get([1]).unwrap()[0].to_string(),
        "this field must be start with `hello`"
    );
}
