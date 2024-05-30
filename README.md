# Valitron is an ergonomic, functional and configurable validator

In the future, modularization will be supported

Inspired by axum

## Features

- Ergonomics validation
- Build-in rule, e.g. Required, StartWith ...
- Closure validate
- Related validate, e.g. password confirm
- Custom rule with other parameter
- Check / modify input data
- Custom error message type
- Support different error types convert, it can use both build-in rules and custom error type simultaneously
- Collect validate error messages
- Support all types data on `#[derive(Serialize, Deserialize)]` ( visit [`serde`](https://serde.rs/) for more info)

## Example 1

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
    // or using trait
    let res = person.validate(validator);
}

fn age_limit(n: &mut u8) -> Result<(), Message> {
    if *n >= 25 && *n <= 45 {
        return Ok(());
    }
    Err("age should be between 25 and 45".into())
}
```
## Example 2
```rust
use valitron::register::string::Validator;
impl Input {
    fn new(mut input: Input) -> Result<Self, Validator<Message>> {
        let valid = Validator::new()
            .insert("name", &mut input.name, Trim.and(Required))
            .insert("email", &mut input.email, Trim.and(Required).and(Email))
            .insert("gender", &mut input.gender, custom(validate_gender))
            .insert(
                "password",
                &mut input.password,
                Trim.custom(validate_password),
            )
            .insert_fn("age", || {
                if input.age < 10 {
                    input.age = 10;
                }
                if input.age > 18 {
                    Err(Message::fallback("age is out of range"))
                } else {
                    Ok(())
                }
            });

        valid.validate(input)
    }
}
fn validate_password(pass: &mut String) -> Result<(), Message> {
    todo!()
}

fn validate_gender(gender: &mut String) -> Result<(), Message> {
    Ok(())
}
```

## Rules Usage

|Usage| Description|
|---|--- |
| `Required` | one rule |
| `Required.and(StartsWith("foo"))` | multi rules |
| `Required.and(StartsWith('a')).bail()`| multi rules and bail|
| `custom(my_handler)` | custom handler rule |
| `Required.custom(my_handler)` | rule and handler rule |
| `Not(StartsWith("foo"))` | negative rule |
| `Required.and(Not(StartsWith("foo")))` | negative rule |

## Bench
| [second scheme](https://github.com/tu6ge/valitron/blob/string/examples/string.rs)| validator(lib) |
| --- | --- |
| 192.55 ns | 680.43 ns |

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
