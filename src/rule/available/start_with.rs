use crate::{rule::Message, RuleShortcut, Value};

#[derive(Clone, Debug)]
pub struct StartWith<T>(pub T);

impl RuleShortcut for StartWith<&str> {
    fn name(&self) -> &'static str {
        "start_with"
    }
    fn message(&self) -> Message {
        "this field must be start with {}".into()
    }
    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => s.starts_with(self.0),
            _ => false,
        }
    }
}

// impl Rule for StartWith<char> {
//     fn name(&self) -> &'static str {
//         "start_with"
//     }
//     fn message(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "this field must be start with {}", self.0)
//     }
//     fn call(&mut self, value: &Value, all_data: &Value) -> bool {
//         match value {
//             Value::Int8(_) => false,
//             Value::String(s) => s.starts_with(self.0),
//             Value::Struct(_) => false,
//         }
//     }
// }
