use serde::Serialize;

use crate::register::FieldName;

use super::*;

#[test]
fn test_get_value() {
    #[derive(Serialize)]
    struct A {
        b: B,
        foo: u8,
    }
    #[derive(Serialize)]
    struct B {
        c: C,
        foo_str: &'static str,
    }
    #[derive(Serialize)]
    struct C(u8);

    let value = A {
        b: B {
            c: C(37),
            foo_str: "bar",
        },
        foo: 11,
    };
    let value = to_value(value).unwrap();

    let name1 = FieldName::Literal("foo".into());
    let val1 = value.get_with_name(&name1).unwrap();
    assert_eq!(val1, &Value::UInt8(11));

    let name2 = vec![
        FieldName::Literal("b".into()),
        FieldName::Literal("foo_str".into()),
    ];
    let val2 = value.get_with_names(&name2.into()).unwrap();
    assert_eq!(val2, &Value::String("bar".into()));

    let name = vec![
        FieldName::Literal("b".into()),
        FieldName::Literal("c".into()),
        FieldName::Tuple(0),
    ];
    let val = value.get_with_names(&name.into()).unwrap();
    assert_eq!(val, &Value::UInt8(37));
}

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct MyType {
        name: String,
        age: u8,
    }
    let my_struct = MyType {
        name: "wang".into(),
        age: 18,
    };

    let value = to_value(my_struct).unwrap();

    assert_eq!(
        value,
        Value::Struct({
            let mut map = BTreeMap::new();
            map.insert(
                Value::StructKey("name".to_string()),
                Value::String("wang".into()),
            );
            map.insert(Value::StructKey("age".to_string()), Value::UInt8(18));
            map
        })
    )
}

#[test]
fn test_struct_nest() {
    #[derive(Serialize)]
    struct A {
        b: B,
        foo: u8,
    }
    #[derive(Serialize)]
    struct B {
        c: C,
        foo_str: String,
    }
    #[derive(Serialize)]
    struct C(u8);

    //let value = to_value(C(12, 15)).unwrap();

    let value = A {
        b: B {
            c: C(37),
            foo_str: "bar".to_string(),
        },
        foo: 11,
    };
    let value = to_value(value).unwrap();

    assert_eq!(
        value,
        Value::Struct({
            let mut map = BTreeMap::new();
            map.insert(Value::StructKey("foo".to_string()), Value::UInt8(11));
            map.insert(
                Value::StructKey("b".to_string()),
                Value::Struct({
                    let mut map = BTreeMap::new();
                    map.insert(
                        Value::StructKey("foo_str".to_string()),
                        Value::String("bar".to_string()),
                    );
                    map.insert(
                        Value::StructKey("c".to_string()),
                        Value::NewtypeStruct(vec![Value::UInt8(37)]),
                    );
                    map
                }),
            );
            map
        })
    )
}

#[test]
fn test_newtype_variant() {
    #[derive(Serialize)]
    struct A {
        age: u8,
    }

    #[derive(Serialize)]
    enum B {
        A(A),
        Foo(u8, A),
    }

    let value = B::A(A { age: 10 });
    let value = to_value(value).unwrap();
    assert_eq!(
        value,
        Value::Enum(
            "A",
            vec![Value::Struct({
                let mut map = BTreeMap::new();
                map.insert(Value::StructKey("age".to_string()), Value::UInt8(10));
                map
            })]
        )
    );

    let value = B::Foo(11, A { age: 10 });
    let value = to_value(value).unwrap();
    assert_eq!(
        value,
        Value::TupleVariant(
            "Foo",
            vec![
                Value::UInt8(11),
                Value::Struct({
                    let mut map = BTreeMap::new();
                    map.insert(Value::StructKey("age".to_string()), Value::UInt8(10));
                    map
                })
            ]
        )
    );
}

#[test]
fn test_int() {
    #[derive(Serialize)]
    struct MyType {
        v1: u8,
        v2: u16,
        v3: u32,
        v4: u64,
        v5: i8,
        v6: i16,
        v7: i32,
        v8: i64,
    }
    let my_struct = MyType {
        v1: u8::MAX,
        v2: u16::MAX,
        v3: u32::MAX,
        v4: u64::MAX,
        v5: i8::MIN,
        v6: i16::MIN,
        v7: i32::MIN,
        v8: i64::MIN,
    };
    let value = to_value(my_struct).unwrap();

    assert_eq!(
        value,
        Value::Struct({
            let mut map = BTreeMap::new();
            map.insert(Value::StructKey("v1".to_string()), Value::UInt8(u8::MAX));
            map.insert(Value::StructKey("v2".to_string()), Value::UInt16(u16::MAX));
            map.insert(Value::StructKey("v3".to_string()), Value::UInt32(u32::MAX));
            map.insert(Value::StructKey("v4".to_string()), Value::UInt64(u64::MAX));
            map.insert(Value::StructKey("v5".to_string()), Value::Int8(i8::MIN));
            map.insert(Value::StructKey("v6".to_string()), Value::Int16(i16::MIN));
            map.insert(Value::StructKey("v7".to_string()), Value::Int32(i32::MIN));
            map.insert(Value::StructKey("v8".to_string()), Value::Int64(i64::MIN));
            map
        })
    )
}

#[test]
fn test_float() {
    #[derive(Serialize)]
    struct MyType {
        v1: u8,
        v2: f32,
        v3: f64,
    }
    let my_struct = MyType {
        v1: u8::MAX,
        v2: f32::MAX,
        v3: f64::MIN,
    };
    let value = to_value(my_struct).unwrap();

    assert_eq!(
        value,
        Value::Struct({
            let mut map = BTreeMap::new();
            map.insert(Value::StructKey("v1".to_string()), Value::UInt8(u8::MAX));
            map.insert(
                Value::StructKey("v2".to_string()),
                Value::Float32(f32::MAX.into()),
            );
            map.insert(
                Value::StructKey("v3".to_string()),
                Value::Float64(f64::MIN.into()),
            );
            map
        })
    )
}

#[derive(Serialize, Debug, PartialEq)]
enum EnumA {
    Foo,
    Bar,
}

#[derive(Serialize, Debug, PartialEq)]
enum EnumB {
    A(u8),
    B { r: u8, g: u8, b: u8 },
}

#[test]
fn test_enum() {
    let val = EnumA::Foo;

    let value = to_value(val).unwrap();
    assert_eq!(value, Value::EnumUnit("Foo"));

    let val = EnumB::A(34);
    let value = to_value(val).unwrap();
    assert_eq!(value, Value::Enum("A", vec![Value::UInt8(34)]));

    let val = EnumB::B {
        r: 22,
        g: 33,
        b: 44,
    };
    let value = to_value(val).unwrap();
    assert_eq!(
        value,
        Value::StructVariant("B", {
            let mut map = BTreeMap::new();
            map.insert(Value::StructVariantKey("r".to_string()), Value::UInt8(22));
            map.insert(Value::StructVariantKey("g".to_string()), Value::UInt8(33));
            map.insert(Value::StructVariantKey("b".to_string()), Value::UInt8(44));
            map
        })
    );
}
