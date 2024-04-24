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

//email validate          time:   [12.942 µs 12.957 µs 12.975 µs]
//                        change: [-1.7710% -0.9200% -0.0549%] (p = 0.04 < 0.05)
//                        Change within noise threshold.
//Found 12 outliers among 100 measurements (12.00%)
//  5 (5.00%) high mild
//  7 (7.00%) high severe
//
//email validate out      time:   [23.147 µs 23.178 µs 23.214 µs]
//                        change: [-1.1500% -0.3149% +0.5635%] (p = 0.48 > 0.05)
//                        No change in performance detected.
//Found 12 outliers among 100 measurements (12.00%)
//  3 (3.00%) high mild
//  9 (9.00%) high severe
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
