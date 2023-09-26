use super::{FieldNames, Validator, ValidatorError};

#[test]
fn test_validator_error_serialize() {
    let mut error = ValidatorError::<String>::new();
    error.push(
        FieldNames::new("field1".into()),
        vec!["message1".into(), "message2".into()],
    );

    let json = serde_json::to_string(&error).unwrap();
    assert_eq!(json, r#"{"field1":["message1","message2"]}"#);
}

#[cfg(feature = "full")]
#[test]
fn repect_insert_rules() {
    use crate::{
        available::{Range, Required, Trim},
        RuleExt,
    };

    let validate = Validator::new()
        .rule("foo", Required)
        .rule("foo", Range::new(1..2));

    let vec = validate.rules.get(&FieldNames::new("foo".into())).unwrap();
    assert!(vec.len() == 2);
    assert!(vec.is_bail() == false);

    let validate = Validator::new()
        .rule("foo", Required.and(Trim).bail())
        .rule("foo", Range::new(1..2));

    let vec = validate.rules.get(&FieldNames::new("foo".into())).unwrap();
    assert!(vec.len() == 3);
    assert!(vec.is_bail() == true);

    let validate = Validator::new()
        .rule("foo", Required)
        .rule("foo", Range::new(1..2).and(Trim).bail());

    let vec = validate.rules.get(&FieldNames::new("foo".into())).unwrap();
    assert!(vec.len() == 3);
    assert!(vec.is_bail() == true);

    let validate = Validator::new()
        .rule("foo", Required.and(Trim).bail())
        .rule("foo", Range::new(1..2).and(Trim).bail());

    // TODO need remove duplicates
    let vec = validate.rules.get(&FieldNames::new("foo".into())).unwrap();
    assert!(vec.len() == 4);
    assert!(vec.is_bail() == true);
}

#[cfg(feature = "full")]
#[test]
fn multi_messages() {
    use crate::{
        available::{Message, Required},
        RuleExt,
    };
    let mut vali = Validator::<Message>::new()
        .rule("field1", Required)
        .rule("field2", Required)
        .message([("field1.required", "bar")]);
    vali = vali.message([("field2.required", "bar2")]);
    assert_eq!(vali.message.len(), 2);
}

#[test]
fn test_total() {
    let mut msg = ValidatorError::<&str>::new();
    msg.push("foo".into(), vec!["foo", "bar"]);
    msg.push("foo2".into(), vec!["foo2"]);

    assert_eq!(msg.total(), 3);
}
