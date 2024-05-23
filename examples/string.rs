use valitron::{
    available::{Email, Message, Required, Trim},
    register::string::Validator,
    rule::string::{custom, StringRuleExt},
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
    fn new(mut input: Input) -> Result<Self, Validator<Message>> {
        let valid = Validator::new()
            .insert("name", &mut input.name, Trim.and(Required))
            .insert("email", &mut input.email, Trim.and(Required).and(Email))
            .insert("gender", &mut input.gender, custom(validate_gender))
            .insert(
                "password",
                &mut input.password,
                Trim.custom(validate_password),
            )
            .insert_fn("age", || {
                if input.age < 10 {
                    input.age = 10;
                }
                if input.age > 18 {
                    Err(Message::fallback("age is out of range"))
                } else {
                    Ok(())
                }
            });

        valid.validate(input)
    }
}

fn validate_password(pass: &mut String) -> Result<(), Message> {
    let upper = pass.find(char::is_uppercase);
    let lower = pass.find(char::is_lowercase);
    let num = pass.find(char::is_numeric);
    if upper.is_some() && lower.is_some() && num.is_some() {
        Ok(())
    } else {
        Err(Message::fallback(
            "password need to contain uppercase, lowercase and numeric",
        ))
    }
}

fn validate_gender(gender: &mut String) -> Result<(), Message> {
    Ok(())
}
