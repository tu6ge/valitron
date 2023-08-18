first model

```rust
let result = data.validate([
  ("name", "required"),
  ("name2", "required|length:6,12"),
  ("name3", custom_check),
  ("name4", ("required", "length:6,12", custom_check))
]).message([
  ("name.required", "msg1"),
  ("name2.required", "{field} is required"),
  ("name2.length", "{field} must not be {value}"),
]);

fn custom_check(s: &str) -> bool {
   s == "abc"
}
```

second model

```rust
let bar = "abc";
let result = data.validate([
  ("name", Required),
  ("name2", Required | LengthRange(6,12)),
  ("name4", StartWith("foo")),
  ("name4", StartWith("{name}")),
  ("password_confirm", Confirm("{password}")),
  ("name3", custom_check),
]).message([

]);

fn custom_check(s: &str) -> bool {
   s == "abc"
}

let result = data.validate(("name", Required));
```