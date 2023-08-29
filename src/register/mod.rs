//! register rules

use std::collections::HashMap;

use crate::{
    rule::{IntoRuleList, RuleList},
    ser::Serializer,
};

mod field_name;
mod lexer;

pub use field_name::FieldName;

#[derive(Default)]
pub struct Ruler<'a> {
    rule: HashMap<Vec<FieldName>, RuleList>,
    message: HashMap<(Vec<FieldName>, FieldName), &'a str>,
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
            list.map(|(k_str, v)| {
                let key = panic_on_err!(field_name::parse_message(k_str));
                panic_on_err!(self.exit_message(&k_str, &key));
                (key, v)
            })
            .into_iter(),
        );
        self
    }

    pub fn validate<T>(self, data: T) -> Result<T, String>
    where
        T: serde::ser::Serialize,
    {
        let value = data.serialize(Serializer);
        todo!()
    }

    fn rule_get(&self, names: &Vec<FieldName>) -> Option<&RuleList> {
        self.rule.get(names)
    }

    fn rules_name(&self, names: &Vec<FieldName>) -> Option<Vec<&'static str>> {
        self.rule_get(names).map(|rule| rule.get_rules_name())
    }

    fn exit_message(
        &self,
        k_str: &str,
        (names, rule_name): &(Vec<FieldName>, FieldName),
    ) -> Result<(), String> {
        let point_index = k_str
            .rfind('.')
            .ok_or(format!("no found `.` in the message index"))?;
        let names = self.rules_name(names).ok_or(format!(
            "the field \"{}\" not found in ruler",
            &k_str[..point_index]
        ))?;

        let rule_name_str = rule_name.as_str();

        if names.contains(&rule_name_str) {
            Ok(())
        } else {
            Err(format!("rule \"{rule_name_str}\" is not found in rules"))
        }
    }
}

// #[test]
// fn test_message() {
//     let ruler = Ruler::new().message([
//         ("name.required", "name mut not be null"),
//         ("password.password", "password mut not very simple"),
//     ]);
// }
