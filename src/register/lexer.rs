use std::slice::Iter;

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    /// match field name
    Ident,

    /// number index
    Index,

    /// match `.`
    Dot,

    /// match `[`
    LeftBracket,

    /// match `]`
    RightBracket,

    /// undefined
    Undefined,

    /// Eof
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub(super) kind: TokenKind,
    pub(super) len: usize,
}

impl Token {
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
    fn eof() -> Self {
        Self {
            kind: TokenKind::Eof,
            len: 0,
        }
    }
    fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

impl From<(TokenKind, usize)> for Token {
    fn from((kind, len): (TokenKind, usize)) -> Self {
        Self { kind, len }
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
        let token = match char {
            '.' => (TokenKind::Dot, 1),
            '[' => (TokenKind::LeftBracket, 1),
            ']' => (TokenKind::RightBracket, 1),
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut iter = indices.clone().peekable();
                let mut current_usize = start_usize;
                loop {
                    match iter.next() {
                        Some((last_usize, con)) => {
                            current_usize = last_usize;
                            if !(con.is_ascii_alphanumeric() || con == '_') {
                                break (TokenKind::Ident, current_usize - start_usize);
                            } else {
                                indices.next();
                            }
                        }
                        None => {
                            break (TokenKind::Ident, current_usize - start_usize + 1);
                        }
                    }
                }
            }
            '0'..='9' => {
                let mut iter = indices.clone().peekable();
                let mut current_usize = start_usize;
                loop {
                    match iter.next() {
                        Some((last_usize, con)) => {
                            current_usize = last_usize;
                            if !matches!(con, '0'..='9') {
                                break (TokenKind::Index, current_usize - start_usize);
                            } else {
                                indices.next();
                            }
                        }
                        None => {
                            break (TokenKind::Index, current_usize - start_usize + 1);
                        }
                    }
                }
            }
            other => (TokenKind::Undefined, other.len_utf8()),
        };
        tokens.push(token.into());
    }

    tokens.push((TokenKind::Eof, 0).into());

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::{lexer, Token, TokenKind};

    fn vec_kind(token: Vec<Token>) -> Vec<TokenKind> {
        token.into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_lexer() {
        let vec = lexer(".").unwrap();
        assert_eq!(vec_kind(vec), vec![TokenKind::Dot, TokenKind::Eof]);

        let vec = lexer("[").unwrap();
        assert_eq!(vec_kind(vec), vec![TokenKind::LeftBracket, TokenKind::Eof]);

        let vec = lexer("]").unwrap();
        assert_eq!(vec_kind(vec), vec![TokenKind::RightBracket, TokenKind::Eof]);

        let vec = lexer("abc").unwrap();
        assert_eq!(
            vec_kind(vec.clone()),
            vec![TokenKind::Ident, TokenKind::Eof]
        );
        assert!(vec[0].len == 3);

        let vec = lexer("abc_23").unwrap();
        assert_eq!(vec_kind(vec), vec![TokenKind::Ident, TokenKind::Eof]);

        let vec = lexer("_23").unwrap();
        assert_eq!(vec_kind(vec), vec![TokenKind::Ident, TokenKind::Eof]);

        let vec = lexer("234").unwrap();
        assert_eq!(
            vec_kind(vec.clone()),
            vec![TokenKind::Index, TokenKind::Eof]
        );
        assert!(vec[0].len == 3);

        let vec = lexer("234abc".into()).unwrap();
        assert_eq!(
            vec_kind(vec),
            vec![TokenKind::Index, TokenKind::Ident, TokenKind::Eof]
        );

        let vec = lexer("234abc我".into()).unwrap();
        assert_eq!(
            vec_kind(vec),
            vec![
                TokenKind::Index,
                TokenKind::Ident,
                TokenKind::Undefined,
                TokenKind::Eof
            ]
        );

        let vec = lexer("abc.d23[cde].ff99.pp[8]").unwrap();
        assert_eq!(
            vec_kind(vec),
            vec![
                TokenKind::Ident,
                TokenKind::Dot,
                TokenKind::Ident,
                TokenKind::LeftBracket,
                TokenKind::Ident,
                TokenKind::RightBracket,
                TokenKind::Dot,
                TokenKind::Ident,
                TokenKind::Dot,
                TokenKind::Ident,
                TokenKind::LeftBracket,
                TokenKind::Index,
                TokenKind::RightBracket,
                TokenKind::Eof
            ]
        );
    }
}
