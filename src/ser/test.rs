use serde::{Deserialize, Serialize};

use super::*;

#[test]
fn test_to_value() {
    #[derive(Serialize)]
    struct MyType {
        name: String,
        age: u8,
    }
    let my_struct = MyType {
        name: "wang".into(),
        age: 18,
    };

    let value = to_value(my_struct).unwrap();

    println!("{:?}", value);
}
