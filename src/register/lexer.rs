use std::str::CharIndices;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    /// match field name
    Ident,

    /// number index
    Index,

    /// match `.`
    Dot,

    /// match `?`
    Option,

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

    fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

impl From<(TokenKind, usize)> for Token {
    fn from((kind, len): (TokenKind, usize)) -> Self {
        Self { kind, len }
    }
}

// Mimics the behaviour of `Symbol::can_be_raw` from `rustc_span`
// fn can_be_raw(string: &str) -> bool {
//     match string {
//         "_" | "super" | "self" | "Self" | "crate" => false,
//         _ => true,
//     }
// }

#[derive(Clone)]
pub struct Cursor<'a> {
    char: CharIndices<'a>,
}
impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            char: source.char_indices(),
        }
    }

    pub fn advance(&mut self) -> Token {
        let (start_usize, char) = match self.char.next() {
            Some(res) => res,
            None => return Token::new(TokenKind::Eof, 0),
        };
        let token = match char {
            '.' => (TokenKind::Dot, 1),
            '[' => (TokenKind::LeftBracket, 1),
            ']' => (TokenKind::RightBracket, 1),
            '?' => (TokenKind::Option, 1),
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut iter = self.char.clone().peekable();
                let mut current_usize = start_usize;
                loop {
                    match iter.next() {
                        Some((last_usize, con)) => {
                            current_usize = last_usize;
                            if !(con.is_ascii_alphanumeric() || con == '_') {
                                break (TokenKind::Ident, current_usize - start_usize);
                            } else {
                                self.char.next();
                            }
                        }
                        None => {
                            break (TokenKind::Ident, current_usize - start_usize + 1);
                        }
                    }
                }
            }
            '0'..='9' => {
                let mut iter = self.char.clone().peekable();
                let mut current_usize = start_usize;
                loop {
                    match iter.next() {
                        Some((last_usize, con)) => {
                            current_usize = last_usize;
                            if !con.is_ascii_digit() {
                                break (TokenKind::Index, current_usize - start_usize);
                            } else {
                                self.char.next();
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

        token.into()
    }
}

#[cfg(test)]
mod test {
    use super::{Cursor, TokenKind};

    #[test]
    fn test_lexer() {
        let mut vec = Cursor::new(".");
        assert_eq!(vec.advance().kind(), &TokenKind::Dot);
        assert_eq!(vec.advance().kind(), &TokenKind::Eof);

        let mut vec = Cursor::new("[");
        assert_eq!(vec.advance().kind(), &TokenKind::LeftBracket);
        assert_eq!(vec.advance().kind(), &TokenKind::Eof);

        let mut vec = Cursor::new("]");
        assert_eq!(vec.advance().kind(), &TokenKind::RightBracket);
        assert_eq!(vec.advance().kind(), &TokenKind::Eof);

        let mut vec = Cursor::new("abc");
        let first = vec.advance();
        assert_eq!(first.kind(), &TokenKind::Ident);
        assert!(first.len == 3);
        assert_eq!(vec.advance().kind(), &TokenKind::Eof);

        let mut vec = Cursor::new("abc.d23?[cde].ff99.pp[8]");
        assert_eq!(vec.advance().kind(), &TokenKind::Ident);
        assert_eq!(vec.advance().kind(), &TokenKind::Dot);
        assert_eq!(vec.advance().kind(), &TokenKind::Ident);
        assert_eq!(vec.advance().kind(), &TokenKind::Option);
        assert_eq!(vec.advance().kind(), &TokenKind::LeftBracket);
        assert_eq!(vec.advance().kind(), &TokenKind::Ident);
        assert_eq!(vec.advance().kind(), &TokenKind::RightBracket);
        assert_eq!(vec.advance().kind(), &TokenKind::Dot);
        assert_eq!(vec.advance().kind(), &TokenKind::Ident);
        assert_eq!(vec.advance().kind(), &TokenKind::Dot);
        assert_eq!(vec.advance().kind(), &TokenKind::Ident);
        assert_eq!(vec.advance().kind(), &TokenKind::LeftBracket);
        assert_eq!(vec.advance().kind(), &TokenKind::Index);
        assert_eq!(vec.advance().kind(), &TokenKind::RightBracket);
        assert_eq!(vec.advance().kind(), &TokenKind::Eof);
    }
}
