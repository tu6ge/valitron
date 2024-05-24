use serde::{Deserialize, Serialize};

use url::Url;
// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};

use criterion::{criterion_group, criterion_main, Criterion};
use valitron::{
    available::{Email, Length, Message},
    register::string::Validator,
    rule::string::{custom, StringRuleExt},
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("validtron score", |b| {
        let mut valitron_data = ValitronData {
            mail: "aaa@example.com".into(),
            site: "example.com".into(),
            first_name: "John".into(),
            age: 18,
            height: 120.0,
        };
        let validator = Validator::new()
            .insert("mail", &mut valitron_data.mail, Email)
            .insert("site", &mut valitron_data.site, custom(valid_url))
            .insert(
                "firstName",
                &mut valitron_data.first_name,
                Length(1..).custom(validate_unique_username2),
            )
            .insert_fn("age", || {
                if (18..=20).contains(&valitron_data.age) {
                    Ok(())
                } else {
                    Err(Message::fallback("age is out of range"))
                }
            })
            .insert_fn("height", || {
                if (0.0_f32..100.0).contains(&valitron_data.height) {
                    Ok(())
                } else {
                    Err(Message::fallback("height is out of range"))
                }
            });
        b.iter(|| {
            let _ = validator.clone().validate(valitron_data.clone());
        })
    });

    c.bench_function("validator score", |b| {
        let signup_data = SignupData {
            mail: "aaa@example.com".into(),
            site: "example.com".into(),
            first_name: "John".into(),
            age: 18,
            height: 120.0,
        };
        b.iter(|| {
            let _ = signup_data.clone().validate();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[derive(Debug, Validate, Deserialize, Clone)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom(function = "validate_unique_username"))]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
    #[validate(range(exclusive_min = 0.0, max = 100.0))]
    height: f32,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ValitronData {
    mail: String,
    site: String,
    #[serde(rename = "firstName")]
    first_name: String,
    age: u32,
    height: f32,
}

fn valid_url(url: &mut String) -> Result<(), Message> {
    match Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid url".into()),
    }
}

fn validate_unique_username2(username: &mut String) -> Result<(), Message> {
    if username == "xXxShad0wxXx" {
        return Err("Invalid username".into());
    }

    Ok(())
}
