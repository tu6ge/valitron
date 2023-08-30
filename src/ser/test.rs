use serde::Serialize;

use super::*;

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
            map.insert("name", Value::String("wang".into()));
            map.insert("age", Value::Int8(18));
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
            map.insert("foo", Value::Int8(11));
            map.insert(
                "b",
                Value::Struct({
                    let mut map = BTreeMap::new();
                    map.insert("foo_str", Value::String("bar".to_string()));
                    map.insert("c", Value::TupleStruct(vec![Value::Int8(37)]));
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
                map.insert("age", Value::Int8(10));
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
                Value::Int8(11),
                Value::Struct({
                    let mut map = BTreeMap::new();
                    map.insert("age", Value::Int8(10));
                    map
                })
            ]
        )
    );
}
