use diesel::{Connection, PgConnection, Queryable, Selectable};
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
        weight: 102.5,
    };

    let data = Input::new(data).unwrap();

    assert_eq!(data.name, "Jone");

    PgConnection::establish("aaa").unwrap();
}

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Input {
    name: String,
    email: String,
    gender: String,
    password: String,
    age: i32,
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

pub fn establish_connection() -> PgConnection {
    PgConnection::establish("DATABASE_URL").unwrap()
}

diesel::table! {
  inputs (email) {
      name -> Varchar,
      email -> Varchar,
      gender -> Varchar,
      password -> Varchar,
      age -> Integer,
      weight -> Float,
  }
}

#[derive(Clone)]
struct UniqueEmail;

impl StringRule for UniqueEmail {
    type Message = String;
    const NAME: &'static str = "unique_email";
    fn call(&mut self, data: &mut String) -> bool {
        use self::inputs::dsl::*;
        use diesel::prelude::*;
        //use self::models::*;

        let conn = &mut establish_connection();

        let results = inputs
            .filter(email.eq(data.to_owned()))
            .select(Input::as_select())
            .load(conn)
            .unwrap();

        results.len() == 0
    }

    fn message(&self) -> Self::Message {
        format!("email is existing")
    }
}
