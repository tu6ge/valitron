use std::{convert::Infallible, fmt::Display, slice::Iter};

use super::{
    lexer::{Cursor, Token, TokenKind},
    MessageKey,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FieldName<'field> {
    Literal(&'field str),
    Array(usize),
    Tuple(u8),

    /// get `g` on enum A { Color{ r:u8, g:u8, b:u8}}
    StructVariant(&'field str),
}

impl FieldName<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            FieldName::Literal(s) => s,
            FieldName::StructVariant(s) => s,
            _ => "",
        }
    }
}

impl Display for FieldName<'_> {
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
pub struct FieldNames<'field>(Vec<FieldName<'field>>);

impl FieldNames<'_> {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> Iter<'_, FieldName> {
        self.0.iter()
    }
}

impl<'field> From<Vec<FieldName<'field>>> for FieldNames<'field> {
    fn from(value: Vec<FieldName<'field>>) -> Self {
        Self(value)
    }
}
impl<'field> From<FieldName<'field>> for FieldNames<'field> {
    fn from(value: FieldName<'field>) -> Self {
        Self(vec![value])
    }
}
impl<'field, const N: usize> From<[FieldName<'field>; N]> for FieldNames<'field> {
    fn from(value: [FieldName<'field>; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

/// Convert to FieldName trait
pub trait IntoFieldName<'field> {
    type Error: std::fmt::Display;
    fn into_field(self) -> Result<FieldNames<'field>, Self::Error>;
}

impl<'a> IntoFieldName<'a> for &'a str {
    type Error = String;
    fn into_field(self) -> Result<FieldNames<'a>, Self::Error> {
        Ok(FieldNames(parse(self)?))
    }
}
impl<'a> IntoFieldName<'a> for u8 {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames<'a>, Self::Error> {
        Ok(FieldNames(vec![FieldName::Tuple(self)]))
    }
}
impl<'a> IntoFieldName<'a> for (u8, u8) {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames<'a>, Self::Error> {
        Ok(FieldNames(vec![
            FieldName::Tuple(self.0),
            FieldName::Tuple(self.1),
        ]))
    }
}
impl<'a> IntoFieldName<'a> for (u8, u8, u8) {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames<'a>, Self::Error> {
        Ok(FieldNames(vec![
            FieldName::Tuple(self.0),
            FieldName::Tuple(self.1),
            FieldName::Tuple(self.2),
        ]))
    }
}
impl<'a> IntoFieldName<'a> for [usize; 1] {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames<'a>, Self::Error> {
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
    source: &'a str,
    token: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        let token = Cursor::new(source);
        Self { source, token }
    }

    pub fn next_name(&mut self) -> Result<Option<FieldName<'a>>, String> {
        let token = self.token.advance();
        match token.kind() {
            TokenKind::Ident => {
                //self.current_pos += 1;
                let res = FieldName::Literal(&self.source[..token.len]);
                self.source = &self.source[token.len..];
                self.eat_dot()?;
                Ok(Some(res))
            }
            TokenKind::Dot => Err("`.` should not be start".into()),
            TokenKind::LeftBracket => {
                self.source = &self.source[token.len..];
                self.parse_bracket().map(Some)
            }
            TokenKind::RightBracket => Err("`]` should to stay behind `[`".into()),
            TokenKind::Index => {
                let res = FieldName::Tuple(
                    (self.source[..token.len])
                        .parse()
                        .map_err(|_| "tuple index is not u8 type".to_string())?,
                );
                self.source = &self.source[token.len..];
                if !(self.expect(TokenKind::Dot)
                    || self.expect(TokenKind::LeftBracket)
                    || self.expect(TokenKind::Eof))
                {
                    return Err("after tuple index should be `.` or `[` or eof".into());
                }

                self.eat_dot()?;
                Ok(Some(res))
            }
            TokenKind::Undefined => Err("undefined char".into()),
            TokenKind::Eof => Ok(None),
        }
    }

    /// parse `[0]` or `[abc]`
    fn parse_bracket(&mut self) -> Result<FieldName<'a>, String> {
        let mut peek = self.token.clone();
        let t = peek.advance();
        match t.kind() {
            TokenKind::Index => {
                if let Token {
                    kind: TokenKind::RightBracket,
                    ..
                } = peek.advance()
                {
                    let name = FieldName::Array(
                        (self.source[..t.len])
                            .parse()
                            .map_err(|_| "tuple index is not u8 type".to_string())?,
                    );
                    // eat index
                    self.token.advance();
                    self.source = &self.source[t.len..];
                    // eat `]`
                    self.token.advance();
                    self.source = &self.source[1..];

                    if !(self.expect(TokenKind::Dot)
                        || self.expect(TokenKind::LeftBracket)
                        || self.expect(TokenKind::Eof))
                    {
                        return Err("after `]` should be `.` or `[` or eof".into());
                    }
                    self.eat_dot()?;
                    return Ok(name);
                }
            }
            TokenKind::Ident => {
                if let Token {
                    kind: TokenKind::RightBracket,
                    ..
                } = peek.advance()
                {
                    let name = FieldName::StructVariant(&self.source[..t.len]);
                    // eat ident
                    self.token.advance();
                    self.source = &self.source[t.len..];
                    // eat `]`
                    self.token.advance();
                    self.source = &self.source[1..];

                    if !(self.expect(TokenKind::Dot)
                        || self.expect(TokenKind::LeftBracket)
                        || self.expect(TokenKind::Eof))
                    {
                        return Err("after `]` should be `.` or `[` or eof".into());
                    }

                    self.eat_dot()?;
                    return Ok(name);
                }
            }
            _ => return Err("Syntax error".into()),
        }

        Err("bracket syntax error".into())
    }

    fn expect(&self, token: TokenKind) -> bool {
        let peek = self.token.clone().advance();
        token == peek.kind
    }

    fn eat_dot(&mut self) -> Result<(), String> {
        let mut peek = self.token.clone();
        if let Token {
            kind: TokenKind::Dot,
            ..
        } = peek.advance()
        {
            let Token { kind, .. } = peek.advance();
            match kind {
                TokenKind::Eof => return Err("`.` should not be end".into()),
                TokenKind::LeftBracket => return Err("after `.` should not be `[`".into()),
                _ => (),
            }
            self.token.advance();
            self.source = &self.source[1..];
        }

        Ok(())
    }
}

pub fn parse<'a>(source: &'a str) -> Result<Vec<FieldName<'a>>, String> {
    let mut parser = Parser::new(source);

    let mut vec = Vec::new();
    loop {
        match parser.next_name()? {
            Some(name) => vec.push(name),
            None => break Ok(vec),
        }
    }
}

pub fn parse_message<'a>(source: &'a str) -> Result<MessageKey, String> {
    let (names, message) = source
        .rsplit_once('.')
        .ok_or("not found message".to_string())?;
    let names = parse(names)?;

    Ok(MessageKey::new(FieldNames(names), message.to_owned()))
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
