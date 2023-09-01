# Valitron is a validator on ergonomics

In the future, modularization will be supported

Inspired by axum

> Warning: This is currently in a very early stage of development

## Examples

```rust
fn main() {
    let validator = Validator::new()
        .rule("name", Required.and(StartWith("hello")))
        .rule("age", custom(age_limit))
        .message([
            ("name.required", "name is required"),
            ("name.start_with", "name should be starts with `hello`"),
        ]);

    let person = Person {
        name: "li",
        age: 18,
    };

    let res = validator.validate(person);
}

fn age_limit(v: &mut Value) -> Result<(), String> {
    if let Value::Int8(n) = v {
        if *n >= 25 && *n <= 45 {
            return Ok(());
        }
    }
    Err("age should be between 25 and 45".to_owned())
}
```

## Limit

Now, primitive type is only support `u8` and `String` as experiment.

