first model:

```rust

struct Data {
  name: String,
  email: String,
  password: String,
  age: u8,
  weight: f32,
}

impl Validator for Data {
  fn validate(&self) -> Result<(), ValidatorMessage> {
    let val = Validate::new([
      be_string("name", &self.name, Required),
      be_string("email", &self.email. Required.and(Email).bail()),
      be_u8("age", &self.age, custom(handle1)),
    ]).bail();

    val.validate()
  }
}

```

second model:
```rust
trait Validator {
  validate(self, rules: impl IntoRuleList<M>) -> Result<(), Vec<M>>;
}

impl Validator for String {
  validate(self, rules: impl IntoRuleList<M>) -> Result<(), Vec<Message>>
  {
    todo!()
  }
}

String::from("abc").validate(Required.and(Email).bail());
self.title.validate(Required.and(Email).bail());
```