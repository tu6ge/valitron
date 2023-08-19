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

fn custom_check(s: &str) -> Result<(), Message> {
   if s == "abc" {
     Ok(())
   }else{
     Err(Message::new("the field must be abc"))
   }
}

let result = data.validate(("name", Required)).message("error value: {value}");
let result = data.validate((0, Required)).message("error msg");
let result = string.validate(Required);

let result = data.validate(("address.city", Required));
let result = data.validate(("address.0", Required));
let result = data.validate(("0.city", Required));
```

third model

```rust
let result = data.validate(|this|{
  [
    (this.name, Required),
    (this.name2, Required | LengthRange(6,12)),
    (this.name2, StartWith("foo")),
    (this.password_confirm, Confirm(this.password)),
    (this.name4, custom_check),
    (this.address.city, Required),
  ]
}).message([
  ("name.required", "msg1"),
  ("name2.required", "{field} is required"),
  ("name2.length", "{field} must not be {value}"),
  ("address.city.required", "city is required"),
]);
let result = data.validate(|d|(d.name, Required));
let result = data.validate(|d|(d.0, Required));

let result = data.validate(|d|(d.name, Required | LengthRange(6,12)));
```