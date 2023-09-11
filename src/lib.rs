//! Valitron is a validator on ergonomics
//!
//! ## This is an example:
//!
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! # use valitron::{
//! # available::{Required, StartWith},
//! # custom, Message, RuleExt, Validator
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
//!         ("introduce.start_with", "introduce should be starts with `I am`"),
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
//! fn age_range(age: &mut u8) -> Result<(), &'static str> {
//!     if *age >= 25 && *age <= 45 {
//!         Ok(())
//!     }else {
//!         Err("age should be between 25 and 45")
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
//! - Required
//! - StartWith
//! - Confirm
//! - Trim (soon)
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
//! This is support closure with a primitive type mutable reference arguments and returns something that can be converted into a [`Message`].
//!
//! About returning, it just need to implement [`IntoRuleMessage`] trait.
//!
//! ## Custom Rule
//!
//! anything types implemented [`Rule`] trait can be used as a rule
//!
//! [`Rule`]: crate::Rule
//! [`Message`]: crate::rule::Message
//! [`IntoRuleMessage`]: crate::rule::IntoRuleMessage

#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod de;
pub mod register;
pub mod rule;
mod ser;
pub mod value;

pub use register::Validator;
pub use rule::{custom, IntoRuleMessage, Message, Rule, RuleExt, RuleShortcut};
pub use value::{FromValue, Value, ValueMap};

#[cfg(feature = "full")]
pub use rule::available;
