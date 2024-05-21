use std::str::CharIndices;

/// # valid email address
///
/// This is twice as efficient as [validator]
///
/// [validator]: https://github.com/Keats/validator
#[inline]
pub fn validate_email(email: &str) -> bool {
    let mut parse = Cursor::new(email);
    parse.parse()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EmailToken {
    Name(String),
    At,
    DomainPart(String),
    Dot,
    IdnaDomain,
    Ip,
    IllegalChar,
}

// Lexer from the specs
// https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
#[derive(Clone)]
pub struct Cursor<'a> {
    email_str: &'a str,
    char: CharIndices<'a>,
    token: Vec<EmailToken>,
    is_idna_domain: bool,
    is_ip: bool,
    at_index: usize,
}

macro_rules! name_chars {
    () => {
        'a'..='z' | 'A'..='Z' | '!' | '#' | '$' | '%'
        | '&' | '\'' | '*' | '+' | '-' | '/' | '='
        | '?' | '^' | '_' | '`' | '{' | '}' | '|' | '~'
    };
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            email_str: source,
            char: source.char_indices(),
            token: Vec::new(),
            is_idna_domain: false,
            is_ip: false,
            at_index: 0,
        }
    }

    fn advance(&mut self) -> Option<EmailToken> {
        if self.is_idna_domain || self.is_ip {
            return None;
        }
        let (start_usize, char) = self.char.next()?;

        if self.token.is_empty() {
            match char {
                name_chars!() => {
                    let mut iter = self.char.clone();
                    let mut current_usize = start_usize;
                    loop {
                        match iter.next() {
                            Some((last_usize, con)) => match con {
                                name_chars!() => {
                                    current_usize = last_usize;
                                    self.char.next();
                                }
                                _ => {
                                    let name = &self.email_str[..current_usize + 1];
                                    let token = EmailToken::Name(name.to_string());
                                    self.token.push(token.clone());
                                    return Some(token);
                                }
                            },
                            None => {
                                // not found other char, this is not a email
                                return None;
                            }
                        }
                    }
                }
                _ => {
                    self.token.push(EmailToken::IllegalChar);
                    Some(EmailToken::IllegalChar)
                }
            }
        } else if self.token.len() == 1 {
            return match char {
                '@' => {
                    self.token.push(EmailToken::At);
                    self.at_index = start_usize;
                    Some(EmailToken::At)
                }
                _ => None,
            };
        } else {
            match char {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    let mut iter = self.char.clone();
                    let mut current_usize = start_usize;
                    loop {
                        match iter.next() {
                            Some((last_usize, con)) => match con {
                                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' => {
                                    current_usize = last_usize;
                                    self.char.next();
                                }
                                _ => {
                                    let name = &self.email_str[start_usize..current_usize + 1];
                                    let token = EmailToken::DomainPart(name.to_string());
                                    self.token.push(token.clone());
                                    return Some(token);
                                }
                            },
                            None => {
                                let name = &self.email_str[start_usize..current_usize + 1];
                                let token = EmailToken::DomainPart(name.to_string());
                                self.token.push(token.clone());
                                return Some(token);
                            }
                        }
                    }
                }
                '.' => {
                    self.token.push(EmailToken::Dot);
                    return Some(EmailToken::Dot);
                }
                '@' => {
                    self.token.push(EmailToken::At);
                    return Some(EmailToken::At);
                }
                '[' => {
                    if start_usize != self.at_index + 1 {
                        return None;
                    }

                    let last_char = self.email_str.chars().last().unwrap();
                    if last_char != ']' {
                        return None;
                    }
                    let ip = &self.email_str[self.at_index + 2..self.email_str.len() - 1];
                    for ch in ip.chars() {
                        match ch {
                            'a'..='f' | 'A'..='F' | '0'..='9' | '.' | ':' => {
                                self.char.next();
                            }
                            _ => return None,
                        }
                    }
                    self.is_ip = true;
                    self.token.push(EmailToken::Ip);

                    return Some(EmailToken::Ip);
                }
                c => {
                    return if !c.is_ascii() {
                        let domain = &self.email_str[self.at_index + 1..];
                        idna::domain_to_ascii(domain).ok().map(|d| {
                            // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
                            if d.chars().count() > 255 {
                                return EmailToken::IllegalChar;
                            }
                            self.is_idna_domain = true;
                            EmailToken::IdnaDomain
                        })
                    } else {
                        // other ascii characters
                        self.token.push(EmailToken::IllegalChar);
                        return Some(EmailToken::IllegalChar);
                    };
                }
            }
        }
    }

    pub fn parse(&mut self) -> bool {
        loop {
            let token = self.advance();
            if token.is_none() {
                break;
            }
            if let Some(EmailToken::IllegalChar) = token {
                return false;
            }
        }

        if self.token.len() < 3 {
            return false;
        }

        // validate the length of each part of the email, BEFORE doing the regex
        // according to RFC5321 the max length of the local part is 64 characters
        // and the max length of the domain part is 255 characters
        // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
        if let EmailToken::Name(ref name) = self.token[0] {
            if name.chars().count() > 64 {
                return false;
            }
        } else {
            return false;
        }

        if !matches!(self.token[1], EmailToken::At) {
            return false;
        }

        match self.token[2] {
            EmailToken::DomainPart(_) | EmailToken::IdnaDomain | EmailToken::Ip => {}
            _ => return false,
        }

        if !self.is_idna_domain && !self.is_ip {
            // validate the length of each part of the email, BEFORE doing the regex
            // according to RFC5321 the max length of the local part is 64 characters
            // and the max length of the domain part is 255 characters
            // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
            let mut domain_chars_count = 0;

            for i in 2..self.token.len() {
                match self.token[i] {
                    EmailToken::DomainPart(ref part) => {
                        domain_chars_count += part.chars().count();

                        if !Self::valid_part(part) {
                            return false;
                        }
                    }
                    EmailToken::Dot => {
                        domain_chars_count += 1;
                    }
                    _ => return false,
                }
            }
            if domain_chars_count > 255 {
                return false;
            }

            if let Some(EmailToken::DomainPart(_)) = self.token.last() {
            } else {
                return false;
            }
        }

        true
    }

    fn valid_part(part: &str) -> bool {
        if part.len() > 63 {
            return false;
        }
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use super::EmailToken;

    use super::Cursor;

    #[test]
    fn name() {
        let str = "abc@def.com";

        let mut cursor = Cursor::new(str);

        let tokens = cursor.advance().unwrap();

        assert_eq!(format!("{tokens:?}"), "Name(\"abc\")");

        let at = cursor.advance().unwrap();

        assert_eq!(at, EmailToken::At);
    }

    #[test]
    fn domain_part() {
        let str = "abc@efg";
        let mut cursor = Cursor::new(str);

        cursor.advance();
        cursor.advance();
        let part = cursor.advance().unwrap();
        assert_eq!(part, EmailToken::DomainPart("efg".to_string()));

        let str = "abc@efg.";
        let mut cursor = Cursor::new(str);

        cursor.advance();
        cursor.advance();
        let part = cursor.advance().unwrap();
        assert_eq!(part, EmailToken::DomainPart("efg".to_string()));

        let str = "abc@e";
        let mut cursor = Cursor::new(str);

        cursor.advance();
        cursor.advance();
        let part = cursor.advance().unwrap();
        assert_eq!(part, EmailToken::DomainPart("e".to_string()));
    }

    #[test]
    fn domain_multi_part() {
        let str = "abc@efg.com";
        let mut cursor = Cursor::new(str);

        cursor.advance();
        cursor.advance();
        cursor.advance();
        let dot = cursor.advance().unwrap();
        let second = cursor.advance().unwrap();

        assert_eq!(dot, EmailToken::Dot);
        assert_eq!(second, EmailToken::DomainPart("com".to_string()));

        let str = "abc@efg.com.cn";
        let mut cursor = Cursor::new(str);

        cursor.advance();
        cursor.advance();
        cursor.advance();
        let dot = cursor.advance().unwrap();
        let second = cursor.advance().unwrap();
        let dot2 = cursor.advance().unwrap();
        let third = cursor.advance().unwrap();

        assert_eq!(dot, EmailToken::Dot);
        assert_eq!(second, EmailToken::DomainPart("com".to_string()));
        assert_eq!(dot2, EmailToken::Dot);
        assert_eq!(third, EmailToken::DomainPart("cn".to_string()));
    }

    #[test]
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
            let output = Cursor::new(input).parse();
            // println!("{} - {}", input, expected);
            assert_eq!(
                output, expected,
                "Email `{}` was not classified correctly",
                input
            );
        }
    }
}
