a model:

```rust

struct Data {
  name: String,
  email: String,
  password: String,
  age: u8,
  weight: f32,
}

valitron!{
  Data{
    name: Required,
    email: Required.and(Trim),
    password: custom(handle)
  }
}
message!{
  Data{
    name.required => "name is required",
  }
}


// finaly code: 

impl Data {
   fn validate(&self) -> bool {
      self.name
   }
}
```