//! # Second scheme of validator
//! The focus of this scheme is on string, as most inputs is string or number, and the verification of numbers is relatively simple.
//! The advantage of this scheme over the first one is that it does not require serialization of all data. 
//! and you can still use build-in rules.
//!
//! this is an example:
//! ```rust
//! # use valitron::{
//! #    available::{Email, Message, Required, Trim},
//! #    register::string::Validator,
//! #    rule::string::{custom, StringRuleExt},
//! # };
//!
//! pub fn main() {
//!    let data = Input {
//!        name:" Jone ".into(),
//!        email:"jone@gmail.com".into(),
//!        gender:"male".into(),
//!        password: "Abc123".into(),
//!        age: 12,
//!        weight: 101.2,
//!     };
//!
//!     let data = Input::new(data).unwrap();
//!
//!     assert_eq!(data.name, "Jone");
//! }
//!
//! struct Input {
//!     name: String,
//!     email: String,
//!     gender: String,
//!     password: String,
//!     age: u8,
//!     weight: f32,
//! }
//!
//! impl Input {
//!     fn new(mut input: Input) -> Result<Self, Validator<Message>> {
//!         let valid = Validator::new()
//!             .insert("name", &mut input.name, Trim.and(Required))
//!             .insert("email", &mut input.email, Trim.and(Required).and(Email))
//!             .insert("gender", &mut input.gender, custom(validate_gender))
//!             .insert(
//!                 "password",
//!                 &mut input.password,
//!                 Trim.custom(validate_password),
//!             )
//!             .insert_fn("age", || {
//!                 if input.age < 10 {
//!                     input.age = 10;
//!                 }
//!                 if input.age > 18 {
//!                     Err(Message::fallback("age is out of range"))
//!                 } else {
//!                     Ok(())
//!                 }
//!             });
//!
//!         valid.validate(input)
//!     }
//! }
//!
//! fn validate_password(pass: &mut String) -> Result<(), Message> {
//!     let upper = pass.find(char::is_uppercase);
//!     let lower = pass.find(char::is_lowercase);
//!     let num = pass.find(char::is_numeric);
//!     if upper.is_some() && lower.is_some() && num.is_some() {
//!         Ok(())
//!     } else {
//!         Err(Message::fallback(
//!             "password need to contain uppercase, lowercase and numeric",
//!         ))
//!     }
//! }
//!
//! fn validate_gender(gender: &mut String) -> Result<(), Message> {
//!     Ok(())
//! }
//!
//! ```

use std::collections::HashMap;

use crate::rule::IntoRuleList;

use super::InnerValidatorError;

pub fn validate<R: IntoRuleList<String, M>, M>(value: String, rules: R) -> Vec<M> {
    let list = rules.into_list();
    let mut string = value;
    list.call(&mut string)
}

pub fn validate_ref<R: IntoRuleList<String, M>, M>(value: &mut String, rules: R) -> Vec<M> {
    rules.into_list().call(value)
}

type Validator<M> = InnerValidatorError<String, M>;

impl<M> Default for Validator<M> {
    fn default() -> Self {
        Self {
            message: HashMap::new(),
        }
    }
}

impl<M> Validator<M> {
    pub fn insert<R, F: Into<String>>(mut self, field: F, value: &mut String, rules: R) -> Self
    where
        R: IntoRuleList<String, M>,
    {
        let res = validate_ref(value, rules);
        if !res.is_empty() {
            self.message.insert(field.into(), res);
        }
        self
    }

    pub fn insert_fn<Field, F>(mut self, field: Field, f: F) -> Self
    where
        F: FnOnce() -> Result<(), M>,
        Field: Into<String>,
    {
        let res = f();
        if res.is_err() {
            self.message.insert(field.into(), vec![res.unwrap_err()]);
        }
        self
    }

    pub fn validate<T>(self, data: T) -> Result<T, Validator<M>> {
        if self.message.is_empty() {
            Ok(data)
        } else {
            Err(self)
        }
    }
}
