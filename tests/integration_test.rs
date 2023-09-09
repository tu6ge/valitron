use serde::{Deserialize, Serialize};
use valitron::{
    available::{Required, StartWith},
    custom, RuleExt, Validator, Value,
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
        res.get("age"),
        Some(&vec!["age should be between 25 and 45".to_string()])
    );
    assert_eq!(
        res.get("weight"),
        Some(&vec!["weight should be between 40 and 80".to_string()])
    );
    assert_eq!(
        res.get("name"),
        Some(&vec!["name should be starts with `hello`".to_string()])
    );

    //println!("{res:?}");
}

fn age_limit(n: &mut u8) -> Result<(), String> {
    if *n >= 25 && *n <= 45 {
        return Ok(());
    }
    Err("age should be between 25 and 45".to_owned())
}

fn weight_limit(v: &mut Value) -> Result<(), String> {
    if let Value::Float32(n) = v {
        let n = n.get();
        if n >= 40.0 && n <= 80.0 {
            return Ok(());
        }
    }
    Err("weight should be between 40 and 80".to_owned())
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
        res.get(0),
        Some(&vec!["first item should be start with `hello`".to_string()])
    );
}

#[test]
fn test_has_array() {
    let validator = Validator::new().rule([1], StartWith("hello"));

    let res = validator.validate(vec!["foo", "bar"]).unwrap_err();

    assert!(res.len() == 1);
    assert_eq!(
        res.get([1]),
        Some(&vec!["this field must be start with {}".to_string()])
    );
}
