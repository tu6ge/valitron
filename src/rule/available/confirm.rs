// use crate::{register::FieldName, rule::Message, value::ValueMap, RuleShortcut, Value};

// #[derive(Clone)]
// struct Confirm(&'static str);

// impl RuleShortcut for Confirm {
//     fn name(&self) -> &'static str {
//         "confirm"
//     }
//     fn message(&self) -> Message {
//         "this field value must be eq {} field value".into()
//     }
//     fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
//         let target = value.get(&FieldName::Literal(self.0.to_string()));
//         let target = match target {
//             Some(target) if target.is_leaf() => target,
//             _ => return false,
//         };
//         match (value.current().unwrap(), target) {
//             (Value::Int8(v), Value::Int8(ref t)) if v == t => true,
//             (Value::String(v), Value::String(ref t)) if v == t => true,
//             _ => false,
//         }
//     }
//     fn call(&mut self, _value: &mut Value) -> bool {
//         unreachable!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use serde::Serialize;

//     use super::Confirm;

//     use crate::{rule::Rule, ser::to_value, Value};

//     #[test]
//     fn test_confirm() {
//         #[derive(Serialize)]
//         struct MyType {
//             name: String,
//             age: u8,
//         }
//         let my_struct = MyType {
//             name: "wang".into(),
//             age: 18,
//         };

//         let all_value = to_value(my_struct).unwrap();

//         let confirm = Confirm("name");
//         let name = Value::String("wang".into());
//         let res = confirm.call(&name, &all_value);
//         assert!(res);
//     }
// }
