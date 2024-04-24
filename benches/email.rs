use criterion::{criterion_group, criterion_main, Criterion};
use valitron::available::email::validate_email;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("email validate", |b| {
        b.iter(|| {
            parse();
        })
    });

    c.bench_function("email validate other", |b| {
        b.iter(|| {
            parse_other();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn parse() {
    let list = vec![
        ("email@here.com", true),
        ("weirder-email@here.and.there.com", true),
        (r#"!def!xyz%abc@example.com"#, true),
        ("email@[127.0.0.1]", true),
        ("email@[2001:dB8::1]", true),
        ("email@[2001:dB8:0:0:0:0:0:1]", true),
        ("email@[::fffF:127.0.0.1]", true),
        ("example@valid-----hyphens.com", true),
        ("example@valid-with-hyphens.com", true),
        ("test@domain.with.idn.tld.उदाहरण.परीक्षा", true),
        (r#""test@test"@example.com"#, false),
        // max length for domain name labels is 63 characters per RFC 1034
        (
            "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            true,
        ),
        (
            "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm",
            true,
        ),
        (
            "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
            true,
        ),
        // 64 * a
        (
            "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            false,
        ),
        ("", false),
        ("abc", false),
        ("abc@", false),
        ("abc@bar", true),
        ("a @x.cz", false),
        ("abc@.com", false),
        ("something@@somewhere.com", false),
        ("email@127.0.0.1", true),
        //("email@[127.0.0.256]", false),
        //("email@[2001:db8::12345]", false),
        //("email@[2001:db8:0:0:0:0:1]", false),
        //("email@[::ffff:127.0.0.256]", false),
        ("example@invalid-.com", false),
        ("example@-invalid.com", false),
        ("example@invalid.com-", false),
        ("example@inv-.alid-.com", false),
        ("example@inv-.-alid.com", false),
        (r#"test@example.com\n\n<script src="x.js">"#, false),
        (r#""\\\011"@here.com"#, false),
        (r#""\\\012"@here.com"#, false),
        ("trailingdot@shouldfail.com.", false),
        // Trailing newlines in username or domain not allowed
        ("a@b.com\n", false),
        ("a\n@b.com", false),
        (r#""test@test"\n@example.com"#, false),
        ("a@[127.0.0.1]\n", false),
        // underscores are not allowed
        ("John.Doe@exam_ple.com", false),
    ];

    for (input, expected) in list {
        let output = validate_email(input);
        // println!("{} - {}", input, expected);
        assert_eq!(
            output, expected,
            "Email `{}` was not classified correctly",
            input
        );
    }
}

//#[test]
fn parse_other() {
    let list = vec![
        ("email@here.com", true),
        ("weirder-email@here.and.there.com", true),
        (r#"!def!xyz%abc@example.com"#, true),
        ("email@[127.0.0.1]", true),
        ("email@[2001:dB8::1]", true),
        ("email@[2001:dB8:0:0:0:0:0:1]", true),
        ("email@[::fffF:127.0.0.1]", true),
        ("example@valid-----hyphens.com", true),
        ("example@valid-with-hyphens.com", true),
        ("test@domain.with.idn.tld.उदाहरण.परीक्षा", true),
        (r#""test@test"@example.com"#, false),
        // max length for domain name labels is 63 characters per RFC 1034
        (
            "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            true,
        ),
        (
            "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm",
            true,
        ),
        (
            "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
            true,
        ),
        // 64 * a
        (
            "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            false,
        ),
        ("", false),
        ("abc", false),
        ("abc@", false),
        ("abc@bar", true),
        ("a @x.cz", false),
        ("abc@.com", false),
        ("something@@somewhere.com", false),
        ("email@127.0.0.1", true),
        //("email@[127.0.0.256]", false),
        //("email@[2001:db8::12345]", false),
        //("email@[2001:db8:0:0:0:0:1]", false),
        //("email@[::ffff:127.0.0.256]", false),
        ("example@invalid-.com", false),
        ("example@-invalid.com", false),
        ("example@invalid.com-", false),
        ("example@inv-.alid-.com", false),
        ("example@inv-.-alid.com", false),
        (r#"test@example.com\n\n<script src="x.js">"#, false),
        (r#""\\\011"@here.com"#, false),
        (r#""\\\012"@here.com"#, false),
        ("trailingdot@shouldfail.com.", false),
        // Trailing newlines in username or domain not allowed
        ("a@b.com\n", false),
        ("a\n@b.com", false),
        (r#""test@test"\n@example.com"#, false),
        ("a@[127.0.0.1]\n", false),
        // underscores are not allowed
        ("John.Doe@exam_ple.com", false),
    ];
    for (input, expected) in list {
        assert_eq!(
            validator::ValidateEmail::validate_email(&input),
            expected,
            "Email `{}` was not classified correctly",
            input
        );
    }
}
