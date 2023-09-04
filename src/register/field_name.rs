use std::{convert::Infallible, fmt::Display, slice::Iter};

use super::{
    lexer::{lexer, Token, TokenKind},
    MessageKey,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FieldName {
    Literal(String),
    Array(usize),
    Tuple(u8),

    /// get `g` on enum A { Color{ r:u8, g:u8, b:u8}}
    StructVariant(String),
}

impl FieldName {
    pub fn as_str(&self) -> &str {
        match self {
            FieldName::Literal(s) => s.as_str(),
            FieldName::StructVariant(s) => s.as_str(),
            _ => "",
        }
    }
}

impl Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldName::Literal(s) => s.fmt(f),
            FieldName::Array(n) => n.fmt(f),
            FieldName::Tuple(n) => n.fmt(f),
            FieldName::StructVariant(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FieldNames(Vec<FieldName>);

impl FieldNames {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> Iter<'_, FieldName> {
        self.0.iter()
    }
}

impl From<Vec<FieldName>> for FieldNames {
    fn from(value: Vec<FieldName>) -> Self {
        Self(value)
    }
}
impl From<FieldName> for FieldNames {
    fn from(value: FieldName) -> Self {
        Self(vec![value])
    }
}
impl<const N: usize> From<[FieldName; N]> for FieldNames {
    fn from(value: [FieldName; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

/// Convert to FieldName trait
pub trait IntoFieldName {
    type Error: std::fmt::Display;
    fn into_field(self) -> Result<FieldNames, Self::Error>;
}

impl IntoFieldName for &str {
    type Error = String;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames(parse(self)?))
    }
}
impl IntoFieldName for u8 {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames(vec![FieldName::Tuple(self)]))
    }
}
impl IntoFieldName for (u8, u8) {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames(vec![
            FieldName::Tuple(self.0),
            FieldName::Tuple(self.1),
        ]))
    }
}
impl IntoFieldName for (u8, u8, u8) {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames(vec![
            FieldName::Tuple(self.0),
            FieldName::Tuple(self.1),
            FieldName::Tuple(self.2),
        ]))
    }
}
impl IntoFieldName for [usize; 1] {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames(vec![FieldName::Array(self[0])]))
    }
}
// impl IntoFieldName for [&str; 1] {
//     type Error = String;
//     fn into_field(self) -> Result<Vec<FieldName>, Self::Error> {
//         self[0].chars()
//         Ok(vec![FieldName::StructVariant(self[0].to_string())])
//     }
// }

struct Parser<'a> {
    names: Vec<FieldName>,
    tokens: Iter<'a, Token>,
    current_pos: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: Iter<'a, Token>) -> Self {
        Self {
            names: Vec::new(),
            tokens,
            current_pos: 0,
        }
    }

    pub fn parse(&mut self, source: &str) -> Result<(), String> {
        while let Some(token) = self.tokens.next() {
            match token.kind() {
                TokenKind::Ident => {
                    //self.current_pos += 1;
                    self.names.push(FieldName::Literal(
                        source[self.current_pos..self.current_pos + token.len].to_owned(),
                    ));
                    self.eat_dot()?;
                }
                TokenKind::Dot => return Err("`.` should not be start".into()),
                TokenKind::LeftBracket => {
                    self.parse_bracket(&source)?;
                    continue;
                }
                TokenKind::RightBracket => return Err("`]` should to stay behind `[`".into()),
                TokenKind::Index => {
                    self.names.push(FieldName::Tuple(
                        (&source[self.current_pos..self.current_pos + token.len])
                            .parse()
                            .map_err(|_| "tuple index is not u8 type".to_string())?,
                    ));
                    if !(self.expect(TokenKind::Dot)
                        || self.expect(TokenKind::LeftBracket)
                        || self.expect(TokenKind::Eof))
                    {
                        return Err("after tuple index should be `.` or `[` or eof".into());
                    }
                    self.eat_dot()?;
                }
                TokenKind::Undefined => return Err("undefined char".into()),
                TokenKind::Eof => (),
            }
            self.current_pos += token.len;
        }

        Ok(())
    }

    /// parse `[0]` or `[abc]`
    fn parse_bracket(&mut self, source: &str) -> Result<(), String> {
        let mut peek = self.tokens.clone().peekable();
        if let Some(t) = peek.next() {
            match t.kind() {
                TokenKind::Index => {
                    if let Some(Token {
                        kind: TokenKind::RightBracket,
                        ..
                    }) = peek.next()
                    {
                        self.current_pos += 1;
                        self.names.push(FieldName::Array(
                            (&source[self.current_pos..self.current_pos + t.len])
                                .parse()
                                .map_err(|_| "tuple index is not u8 type".to_string())?,
                        ));
                        // eat index
                        self.tokens.next();
                        // eat `]`
                        self.tokens.next();
                        self.current_pos += t.len + 1;
                        if !(self.expect(TokenKind::Dot)
                            || self.expect(TokenKind::LeftBracket)
                            || self.expect(TokenKind::Eof))
                        {
                            return Err("after `]` should be `.` or `[` or eof".into());
                        }
                        self.eat_dot()?;
                        return Ok(());
                    }
                }
                TokenKind::Ident => {
                    if let Some(Token {
                        kind: TokenKind::RightBracket,
                        ..
                    }) = peek.next()
                    {
                        self.current_pos += 1;
                        self.names.push(FieldName::StructVariant(
                            (&source[self.current_pos..self.current_pos + t.len]).to_owned(),
                        ));
                        // eat ident
                        self.tokens.next();
                        // eat `]`
                        self.tokens.next();
                        self.current_pos += t.len + 1;
                        if !(self.expect(TokenKind::Dot)
                            || self.expect(TokenKind::LeftBracket)
                            || self.expect(TokenKind::Eof))
                        {
                            return Err("after `]` should be `.` or `[` or eof".into());
                        }

                        self.eat_dot()?;
                        return Ok(());
                    }
                }
                _ => return Err("Syntax error".into()),
            }
        } else {
            return Err("bracket not cloesed".into());
        }

        Err("bracket syntax error".into())
    }

    fn expect(&self, token: TokenKind) -> bool {
        let mut peek = self.tokens.clone().peekable();

        if let Some(Token { kind, .. }) = peek.next() {
            if &token == kind {
                return true;
            }
        }

        false
    }

    fn eat_dot(&mut self) -> Result<(), String> {
        let mut peek = self.tokens.clone().peekable();
        if let Some(Token {
            kind: TokenKind::Dot,
            ..
        }) = peek.next()
        {
            if let Some(Token { kind, .. }) = peek.next() {
                match kind {
                    TokenKind::Eof => return Err("`.` should not be end".into()),
                    TokenKind::LeftBracket => return Err("after `.` should not be `[`".into()),
                    _ => (),
                }
            }
            self.tokens.next();
            self.current_pos += 1;
        }

        Ok(())
    }
}

pub fn parse(source: &str) -> Result<Vec<FieldName>, String> {
    let tokens = lexer(source).unwrap();
    let mut parser = Parser::new(tokens.iter());
    parser.parse(source)?;

    Ok(parser.names)
}

pub fn parse_message(source: &str) -> Result<MessageKey, String> {
    let mut names = parse(source)?;

    if let Some(name) = names.pop() {
        if let FieldName::Literal(s) = name {
            return Ok(MessageKey::new(FieldNames(names), s));
        }
    }
    Err("not found validate rule name".into())
}

#[test]
fn test_parse() {
    let names = parse("abc").unwrap();
    assert_eq!(names, vec![FieldName::Literal("abc".into())]);

    let names = parse("name.full_name").unwrap();
    assert_eq!(
        names,
        vec![
            FieldName::Literal("name".into()),
            FieldName::Literal("full_name".into())
        ]
    );

    let names = parse("name.1").unwrap();
    assert_eq!(
        names,
        vec![FieldName::Literal("name".into()), FieldName::Tuple(1)]
    );

    let names = parse("name[511]").unwrap();
    assert_eq!(
        names,
        vec![FieldName::Literal("name".into()), FieldName::Array(511)]
    );

    let names = parse("name[age]").unwrap();
    assert_eq!(
        names,
        vec![
            FieldName::Literal("name".into()),
            FieldName::StructVariant("age".into())
        ]
    );

    let names = parse("5").unwrap();
    assert_eq!(names, vec![FieldName::Tuple(5)]);

    parse("511").unwrap_err();
    parse("5age").unwrap_err();
    parse("[5]age").unwrap_err();
    parse(".age").unwrap_err();

    let names = parse("name.age[foo][0].color.0").unwrap();
    assert_eq!(
        names,
        vec![
            FieldName::Literal("name".into()),
            FieldName::Literal("age".into()),
            FieldName::StructVariant("foo".into()),
            FieldName::Array(0),
            FieldName::Literal("color".into()),
            FieldName::Tuple(0),
        ]
    );
}
