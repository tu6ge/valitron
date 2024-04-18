use std::collections::BTreeMap;

use serde::Deserialize;

use crate::value::Value;

#[derive(Deserialize, Debug)]
struct A {
    b: B,
    foo: u8,
}
#[derive(Deserialize, Debug)]
struct B {
    c: C,
    foo_str: String,
}
#[derive(Deserialize, Debug)]
struct C(i8, u64, f32, f64);

#[test]
fn test_dep() {
    let c_value = Value::TupleStruct(vec![
        Value::Int8(22),
        Value::Uint64(33),
        Value::Float32(5.0_f32.into()),
        Value::Float64(100.0_f64.into()),
    ]);
    let b_value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(Value::StructKey("c".to_string()), c_value);
        map.insert(
            Value::StructKey("foo_str".to_string()),
            Value::String("hello".to_string()),
        );
        map
    });
    let a_value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(Value::StructKey("b".to_string()), b_value);
        map.insert(Value::StructKey("foo".to_string()), Value::Uint8(11));
        map
    });

    let res = A::deserialize(a_value).unwrap();

    assert_eq!(
        format!("{res:?}"),
        r#"A { b: B { c: C(22, 33, 5.0, 100.0), foo_str: "hello" }, foo: 11 }"#
    );
}

#[derive(Deserialize, Debug, PartialEq)]
enum EnumA {
    Foo,
    Bar,
}

#[derive(Deserialize, Debug, PartialEq)]
enum EnumB {
    A(u8),
    B { r: u8, g: u8, b: u8 },
}

#[test]
fn test_enum() {
    let value = Value::EnumUnit("Foo");
    let a = EnumA::deserialize(value).unwrap();
    assert_eq!(a, EnumA::Foo);

    let value = Value::Enum("A", vec![Value::Uint8(11)]);
    let b = EnumB::deserialize(value).unwrap();
    assert!(matches!(b, EnumB::A(11)));

    let value = Value::StructVariant("B", {
        let mut map = BTreeMap::new();
        map.insert(Value::StructVariantKey("r".to_string()), Value::Uint8(22));
        map.insert(Value::StructVariantKey("g".to_string()), Value::Uint8(33));
        map.insert(Value::StructVariantKey("b".to_string()), Value::Uint8(44));
        map
    });
    let b = EnumB::deserialize(value).unwrap();
    assert!(matches!(
        b,
        EnumB::B {
            r: 22,
            g: 33,
            b: 44
        }
    ));
}

#[derive(Deserialize, Debug, PartialEq)]
struct Opt {
    val_u8: Option<u8>,
    val_f32: Option<f32>,
    val_string: Option<String>,
}

#[test]
fn test_option() {
    let value = Value::Option(Box::new(Some(Value::Uint32(10))));
    let a = Option::deserialize(value).unwrap();
    assert_eq!(a, Some(10u32));

    let value = Value::Option(Box::new(None));
    let b: Option<u32> = Option::deserialize(value).unwrap();
    assert_eq!(b, None);

    let value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(
            Value::StructKey("val_u8".to_string()),
            Value::Option(Box::new(Some(Value::Uint8(22)))),
        );
        map.insert(
            Value::StructKey("val_f32".to_string()),
            Value::Option(Box::new(Some(Value::Float32(33.0_f32.into())))),
        );
        map.insert(
            Value::StructKey("val_string".to_string()),
            Value::Option(Box::new(None)),
        );
        map
    });
    let c = Opt::deserialize(value).unwrap();
    assert_eq!(
        c,
        Opt {
            val_u8: Some(22),
            val_f32: Some(33_f32),
            val_string: None,
        }
    );
}

#[test]
fn unsupport_str() {
    #[derive(Deserialize, Debug)]
    struct A<'a> {
        str: &'a str,
        num: u8,
    }

    let value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(Value::StructKey("str".into()), Value::String("foo".into()));
        map.insert(Value::StructKey("num".into()), Value::Uint8(11));
        map
    });

    let err = A::deserialize(value).unwrap_err();
    println!("{err}")
}

#[test]
fn skip_str() {
    #[derive(Deserialize, Debug)]
    struct A<'a> {
        #[serde(skip_deserializing)]
        str: &'a str,
        num: u8,
    }

    let value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(Value::StructKey("str".into()), Value::String("foo".into()));
        map.insert(Value::StructKey("num".into()), Value::Uint8(11));
        map
    });

    let a = A::deserialize(value).unwrap();
    assert!(a.str.is_empty());
}
