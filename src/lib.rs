#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(dead_code))]

mod de;
pub mod register;
pub mod rule;
mod ser;
pub mod value;

pub use register::Validator;
pub use rule::{custom, Rule, RuleExt, RuleShortcut};
pub use value::Value;
