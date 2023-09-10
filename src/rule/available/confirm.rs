use crate::{register::FieldNames, value::ValueMap, RuleShortcut, Value};

#[derive(Clone)]
pub struct Confirm<T>(pub T);

impl RuleShortcut for Confirm<&str> {
    type Message = String;

    fn name(&self) -> &'static str {
        "confirm"
    }
    fn message(&self) -> Self::Message {
        "this field value must be eq {} field value".into()
    }
    fn call_with_relate(&mut self, value: &mut ValueMap) -> bool {
        let target = value.get(&FieldNames::new(self.0.to_string()));
        let target = match target {
            Some(target) if target.is_leaf() => target,
            _ => return false,
        };
        match (value.current().unwrap(), target) {
            (Value::Int8(v), Value::Int8(ref t)) if v == t => true,
            (Value::String(v), Value::String(ref t)) if v == t => true,
            _ => false,
        }
    }
    fn call(&mut self, _value: &mut Value) -> bool {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::Confirm;

    use crate::{register::FieldNames, rule::Rule, ser::to_value, RuleShortcut, Value, ValueMap};

    #[test]
    fn test_confirm() {
        #[derive(Serialize)]
        struct MyType {
            name: String,
            other_name: String,
            age: u8,
        }
        let my_struct = MyType {
            name: "wang".into(),
            other_name: "wanh".into(),
            age: 18,
        };

        let all_value = to_value(my_struct).unwrap();

        let mut confirm = Confirm("name");
        let mut map = ValueMap::new(all_value);
        map.index(FieldNames::new("other_name".to_string()));
        let res = confirm.call_with_relate(&mut map);
        assert!(res == false);
    }
}
