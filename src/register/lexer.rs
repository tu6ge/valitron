use std::slice::Iter;

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// match field name
    Ident(String),

    /// number index
    Index(usize),

    /// match `.`
    Point,

    /// match `[`
    LeftBracket,

    /// match `]`
    RightBracket,

    /// undefined
    Undefined,

    /// Eof
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub(super) kind: TokenKind,
}

impl Token {
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
    fn eof() -> Self {
        Self {
            kind: TokenKind::Eof,
        }
    }
}

impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl<'a> From<&'a str> for Error {
    fn from(value: &'a str) -> Self {
        Self {
            message: value.to_owned(),
        }
    }
}
// Mimics the behaviour of `Symbol::can_be_raw` from `rustc_span`
fn can_be_raw(string: &str) -> bool {
    match string {
        "_" | "super" | "self" | "Self" | "crate" => false,
        _ => true,
    }
}

pub fn lexer<'a>(source: &'a str) -> Result<Vec<Token>, Error> {
    let mut indices = source.char_indices();

    let mut tokens = Vec::new();
    loop {
        let (start_usize, char) = match indices.next() {
            None => break,
            Some(res) => res,
        };
        let token: Token = match char {
            '.' => TokenKind::Point.into(),
            '[' => TokenKind::LeftBracket.into(),
            ']' => TokenKind::RightBracket.into(),
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut iter = indices.clone().peekable();
                let t;
                let mut current_usize = start_usize;
                loop {
                    match iter.next() {
                        Some((last_usize, con)) => {
                            current_usize = last_usize;
                            if !(con.is_ascii_alphanumeric() || con == '_') {
                                t = TokenKind::Ident(source[start_usize..last_usize].into()).into();
                                break;
                            } else {
                                indices.next();
                            }
                        }
                        None => {
                            t = TokenKind::Ident(source[start_usize..current_usize + 1].into())
                                .into();
                            break;
                        }
                    }
                }
                t
            }
            '0'..='9' => {
                let mut iter = indices.clone().peekable();
                let t;
                let mut current_usize = start_usize;
                loop {
                    match iter.next() {
                        Some((last_usize, con)) => {
                            current_usize = last_usize;
                            if !matches!(con, '0'..='9') {
                                t = TokenKind::Index(
                                    source[start_usize..last_usize].parse().unwrap(),
                                )
                                .into();
                                break;
                            } else {
                                indices.next();
                            }
                        }
                        None => {
                            t = TokenKind::Index(
                                source[start_usize..current_usize + 1].parse().unwrap(),
                            )
                            .into();
                            break;
                        }
                    }
                }
                t
            }
            _ => TokenKind::Undefined.into(),
        };
        tokens.push(token);
    }

    tokens.push(TokenKind::Eof.into());

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::{lexer, Token, TokenKind};

    #[test]
    fn test_lexer() {
        let vec = lexer(".").unwrap();
        assert_eq!(vec, vec![TokenKind::Point.into(), Token::eof()]);

        let vec = lexer("[").unwrap();
        assert_eq!(vec, vec![TokenKind::LeftBracket.into(), Token::eof()]);

        let vec = lexer("]").unwrap();
        assert_eq!(vec, vec![TokenKind::RightBracket.into(), Token::eof()]);

        let vec = lexer("abc").unwrap();
        assert_eq!(
            vec,
            vec![TokenKind::Ident("abc".into()).into(), Token::eof()]
        );

        let vec = lexer("abc_23").unwrap();
        assert_eq!(
            vec,
            vec![TokenKind::Ident("abc_23".into()).into(), Token::eof()]
        );

        let vec = lexer("_23").unwrap();
        assert_eq!(
            vec,
            vec![TokenKind::Ident("_23".into()).into(), Token::eof()]
        );

        let vec = lexer("234").unwrap();
        assert_eq!(vec, vec![TokenKind::Index(234).into(), Token::eof()]);

        let vec = lexer("234abc".into()).unwrap();
        assert_eq!(
            vec,
            vec![
                TokenKind::Index(234).into(),
                TokenKind::Ident("abc".into()).into(),
                Token::eof()
            ]
        );

        let vec = lexer("abc.d23[cde].ff99.pp[8]").unwrap();
        assert_eq!(
            vec,
            vec![
                TokenKind::Ident("abc".into()).into(),
                TokenKind::Point.into(),
                TokenKind::Ident("d23".into()).into(),
                TokenKind::LeftBracket.into(),
                TokenKind::Ident("cde".into()).into(),
                TokenKind::RightBracket.into(),
                TokenKind::Point.into(),
                TokenKind::Ident("ff99".into()).into(),
                TokenKind::Point.into(),
                TokenKind::Ident("pp".into()).into(),
                TokenKind::LeftBracket.into(),
                TokenKind::Index(8).into(),
                TokenKind::RightBracket.into(),
                Token::eof()
            ]
        );
    }
}
