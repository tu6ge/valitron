//! Run with
//!
//! ```not_rust
//! cargo run --example rocket --features="full"
//!
//! curl "127.0.0.1:8000?name=&second="
//! -> name is required
//!
//! curl "127.0.0.1:8000?name=%20foo&second=bar"
//! -> <h1>Hello, foo!</h1>
//! ```

use valitron::{
    available::{Required, Trim},
    RuleExt, Validatable, Validator,
};

#[macro_use]
extern crate rocket;

#[get("/?<name>&<second>")]
fn index(name: String, second: String) -> String {
    match (name, second).validate_mut(Validator::new().rule("0", Trim.and(Required))) {
        Ok((name, _)) => format!("Hello, {name}!"),
        Err(_) => format!("name is required"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
