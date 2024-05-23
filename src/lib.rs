//! Valitron is an ergonomics, functional and configurable validator.
//!
//! ## This is an example:
//!
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! # use valitron::{
//! # available::{Message, Required, StartWith},
//! # custom, RuleExt, Validator
//! # };
//! #[derive(Serialize, Debug)]
//! struct Person {
//!     introduce: &'static str,
//!     age: u8,
//!     weight: f32,
//! }
//!
//! # fn main() {
//! let validator = Validator::new()
//!     .rule("introduce", Required.and(StartWith("I am")))
//!     .rule("age", custom(age_range))
//!     .message([
//!         ("introduce.required", "introduce is required"),
//!         (
//!             "introduce.start_with",
//!             "introduce should be starts with `I am`",
//!         ),
//!     ]);
//!
//! let person = Person {
//!     introduce: "hi",
//!     age: 18,
//!     weight: 20.0,
//! };
//!
//! let res = validator.validate(person).unwrap_err();
//! assert!(res.len() == 2);
//! # }
//!
//! fn age_range(age: &mut u8) -> Result<(), Message> {
//!     if *age >= 25 && *age <= 45 {
//!         Ok(())
//!     } else {
//!         Err("age should be between 25 and 45".into())
//!     }
//! }
//! ```
//!
//! ## Prerequisite
//!
//! input data needs implementation `serde::Serialize`, and if you want to modify data,
//! it should be also implementation `serde::Deserialize`
//!
//! ## Available Rule
//!
//! - [`Required`]
//! - [`StartWith`]
//! - [`Confirm`]
//! - [`Trim`]
//! - [`Range`]
//! - customizable
//!
//! To get started using all of Valitron's optional rule, add this to your
//! `Cargo.toml`:
//!
//! ```toml
//! valitron = { version = "0.1", features = ["full"] }
//! ```
//!
//! ## Closure Rule
//!
//! This is support closure with a primitive type mutable reference arguments and returning `message type`.
//!
//! > ### Tip
//! > `message type` is [`Validator`]'s only one generics argument, default is `String`, but when `full` features
//! > enabled default is [`Message`], and it is customable. And it's not immutable, it can be changed by [`map`] method.
//!
//! ## Custom Rule
//!
//! anything types implemented [`Rule`] trait can be used as a rule
//!
//! [`map`]: crate::register::Validator::map
//! [`Rule`]: crate::Rule
//! [`Message`]: crate::available::Message
//! [`Required`]: crate::available::required
//! [`StartWith`]: crate::available::start_with
//! [`Confirm`]: crate::available::confirm
//! [`Trim`]: crate::available::trim
//! [`Range`]: crate::available::range

#![cfg_attr(test, allow(unused_imports, dead_code))]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
//#![warn(clippy::unwrap_used)]
//#![doc(html_playground_url = "https://play.rust-lang.org/")]

mod de;
pub mod register;
pub mod rule;
mod ser;
pub mod value;

#[macro_use]
pub(crate) mod macros;

pub use register::{ValidPhrase, Validatable, Validator};
pub use rule::{custom, Rule, RuleExt};
pub use value::{FromValue, Value, ValueMap};

#[cfg(feature = "full")]
pub use rule::available;
