//! register rules

use std::collections::HashMap;

use crate::rule::{IntoRuleList, RuleList};

use self::field_name::Name;

mod field_name;
mod lexer;

#[derive(Default)]
pub struct Ruler<'a> {
    rule: HashMap<Vec<Name>, RuleList>,
    message: HashMap<Vec<Name>, &'a str>,
}

macro_rules! panic_on_err {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(err) => panic!("{err}"),
        }
    };
}

impl<'a> Ruler<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn rule<R: IntoRuleList>(mut self, field: &'a str, rule: R) -> Self {
        let names = panic_on_err!(field_name::parse(field));
        self.rule.insert(names, rule.into_list());
        self
    }

    pub fn message<const N: usize>(mut self, list: [(&'a str, &'a str); N]) -> Self {
        self.message = HashMap::from_iter(
            list.map(|(k, v)| {
                let k = panic_on_err!(field_name::parse_message(k));
                (k, v)
            })
            .into_iter(),
        );
        self
    }
}

#[test]
fn test_message() {
    let ruler = Ruler::new().message([
        ("name.required", "name mut not be null"),
        ("password.password", "password mut not very simple"),
    ]);
}
