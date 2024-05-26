use valitron::{
    available::{Email, Trim},
    register::string::Validator,
    rule::string::{StringRule, StringRuleExt},
};

pub fn main() {
    let data = Input {
        name: " Jone ".into(),
        email: "jone@gmail.com".into(),
        gender: "male".into(),
        password: "Abc123".into(),
        age: 12,
        weight: 101.2,
    };

    let data = Input::new(data).unwrap();

    assert_eq!(data.name, "Jone");
}

struct Input {
    name: String,
    email: String,
    gender: String,
    password: String,
    age: u8,
    weight: f32,
}

impl Input {
    fn new(mut input: Input) -> Result<Self, Validator<String>> {
        let valid = Validator::new()
            .insert("name", &mut input.name, Trim)
            .insert("email", &mut input.email, Trim.and(Email))
            .map(Into::<String>::into)
            .insert("name", &mut input.name, MyRequired("name"))
            .insert("email", &mut input.email, MyRequired("email"));

        valid.validate(input)
    }
}

#[derive(Debug, Clone)]
struct MyRequired<'a>(&'a str);

impl StringRule for MyRequired<'_> {
    type Message = String;
    const NAME: &'static str = "my_required";
    fn call(&mut self, data: &mut String) -> bool {
        !data.is_empty()
    }

    fn message(&self) -> Self::Message {
        format!("{} is not be empty", self.0)
    }
}
