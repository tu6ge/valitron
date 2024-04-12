first model

```rust
let result = data.register("rule_name", |s|{
  s == "abc"
}).validate([
  ("name", "required"),
  ("name2", "required|length:6,12"),
  ("name3", "rule_name"),
  ("name4", "required|length:6,12|rule_name"),
  ("password_confirm", "confirm:$password")
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
let result = data.validate(rule!(
  ("name", Required),
  ("name2", (Required , LengthRange(6,12))),
  ("name4", StartWith("foo")),
  ("name4", StartWith("{name}")),
  ("password_confirm", Confirm("{password}")),
  ("password_confirm", |this|Confirm(this.password)),
  ("password_confirm", (Required, |this|Confirm(this.password))),
  ("name3", custom_check),
  ("address.city", Required),
  ("address.0", Required),
  ("address[0]", Required),
  ("0.city", Required),
  ("[0].city", Required),
  ("map", MapExit("item 1")),
)).message([

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
let result = data.validate([(0, Required),(1, Required)]).message(["error msg","error msg"]);
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

result 

```rust
fn validate(..) -> Result<Self, ValidateError>{
  todo!()
}

struct ValidateError {
  list: HashMap<String, Vec<String>>,
}
```

fourth model

```rust
let bar = "abc";
let result = data
  .check("name", Required)
  .check("name2", Required.and(LengthRange(6,12)).bail())
  .check("name3", StartWith("foo"))
  .check("name4", StartWith("{name}"))
  .check("password_confirm", Confirm("{password}"))
  .check("name3", custom_check)
  .check("name4", Required.and(custom_check))
  .check("address.city", Required)
  .check("address.0", Required)
  .check("address[0]", Required)
  .check("0.city", Required)
  .check("[0].city", Required);
  .message([
    ("name.required", "msg1"),
    ("name2.required", "{field} is required"),
    ("name2.length", "{field} must not be {value}"),
    ("address.city.required", "city is required"),
  ]);

fn custom_check(value) -> Result<(), String> {
    if value == "abc" {
      Err("error msg".into())
    }
    Ok(())
}
```

fifth model
```rust
let bar = "abc";
let validator = Validator::new()
  .rule("name", Required)
  .rule("name2", Required.and(LengthRange(6,12)).bail())
  .rule("name3", StartWith("foo"))
  .rule("name4", StartWith("{name}"))
  .rule("password_confirm", Confirm("{password}"))
  .rule("name3", custom_check)
  .rule("name4", Required.custom(custom_check))
  .rule("address.city", Required)
  .rule("address.0", Required)
  .rule("address[0]", Required)
  .rule("0.city", Required)
  .rule("[0].city", Required)
  .rule("color[red]", Required) // match struct variant
  .rule(0, Required)
  .rule([10], Required)
  .message([
    ("name.required", "msg1"),
    ("name2.required", "{field} is required"),
    ("name2.length", "{field} must not be {value}"),
    ("address.city.required", "city is required"),
    ("name3.custom", "custom check's message"),
    ("name4.custom", "custom check's message"),
  ]);

let data = validator.validate(data);
```