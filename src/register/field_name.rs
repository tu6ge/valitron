use std::{
    convert::Infallible,
    fmt::Display,
    hash::{Hash, Hasher},
};

use serde::Serialize;

use super::{
    lexer::{Cursor, Token, TokenKind},
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

fn names_to_string(vec: &[FieldName]) -> String {
    let mut string = String::new();
    for item in vec.iter() {
        match item {
            FieldName::Literal(s) => {
                if !string.is_empty() {
                    string.push('.');
                }
                string.push_str(s);
            }
            FieldName::Array(n) => {
                string.push('[');
                string.push_str(&n.to_string());
                string.push(']');
            }
            FieldName::Tuple(n) => {
                if !string.is_empty() {
                    string.push('.');
                }
                string.push_str(&n.to_string());
            }
            FieldName::StructVariant(s) => {
                string.push('[');
                string.push_str(s);
                string.push(']');
            }
        }
    }
    string
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct FieldNames {
    string: String,
}

impl Serialize for FieldNames {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.string)
    }
}

impl Hash for FieldNames {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state)
    }
}

impl FieldNames {
    pub(crate) fn new(string: String) -> Self {
        Self { string }
    }

    // pub fn iter(&self) -> Iter<'_, FieldName> {
    //     self.vec.iter()
    // }
    pub fn as_str(&self) -> &str {
        &self.string
    }
}

impl From<Vec<FieldName>> for FieldNames {
    fn from(value: Vec<FieldName>) -> Self {
        Self {
            string: names_to_string(&value),
        }
    }
}
impl From<FieldName> for FieldNames {
    fn from(value: FieldName) -> Self {
        let vec = vec![value];
        Self {
            string: names_to_string(&vec),
        }
    }
}
impl<const N: usize> From<[FieldName; N]> for FieldNames {
    fn from(value: [FieldName; N]) -> Self {
        let vec: Vec<_> = value.into_iter().collect();
        Self::from(vec)
    }
}

/// Convert to FieldName trait
pub trait IntoFieldName {
    type Error: std::fmt::Display;
    fn into_field(self) -> Result<FieldNames, Self::Error>;
}

impl IntoFieldName for &str {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames {
            string: self.to_string(),
        })
    }
}
impl IntoFieldName for u8 {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames {
            string: self.to_string(),
        })
    }
}
impl IntoFieldName for (u8, u8) {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames {
            string: format!("{}.{}", self.0, self.1),
        })
    }
}
impl IntoFieldName for (u8, u8, u8) {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames {
            string: format!("{}.{}.{}", self.0, self.1, self.2),
        })
    }
}
impl IntoFieldName for [usize; 1] {
    type Error = Infallible;
    fn into_field(self) -> Result<FieldNames, Self::Error> {
        Ok(FieldNames {
            string: format!("[{}]", self[0]),
        })
    }
}
// impl IntoFieldName for [&str; 1] {
//     type Error = String;
//     fn into_field(self) -> Result<Vec<FieldName>, Self::Error> {
//         self[0].chars()
//         Ok(vec![FieldName::StructVariant(self[0].to_string())])
//     }
// }
impl<'a, T> IntoFieldName for &'a T
where
    T: IntoFieldName + Copy,
{
    type Error = T::Error;

    fn into_field(self) -> Result<FieldNames, Self::Error> {
        T::into_field(*self)
    }
}

pub(crate) struct Parser<'a> {
    source: &'a str,
    token: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        let token = Cursor::new(source);
        Self { source, token }
    }

    pub fn next_name(&mut self) -> Result<Option<FieldName>, ParserError> {
        let token = self.token.advance();
        match token.kind() {
            TokenKind::Ident => {
                //self.current_pos += 1;
                let ident;
                (ident, self.source) = self.source.split_at(token.len);
                let res = FieldName::Literal(ident.to_owned());
                self.eat_dot()?;
                Ok(Some(res))
            }
            TokenKind::Dot => Err(ParserError::DotStart),
            TokenKind::LeftBracket => {
                self.source = &self.source[token.len..];
                self.parse_bracket().map(Some)
            }
            TokenKind::RightBracket => Err(ParserError::BracketRight),
            TokenKind::Index => {
                let index_str;
                (index_str, self.source) = self.source.split_at(token.len);
                let res = FieldName::Tuple(
                    index_str
                        .parse()
                        .map_err(|_| ParserError::ParseTupleIndex)?,
                );
                if !(self.expect(TokenKind::Dot)
                    || self.expect(TokenKind::LeftBracket)
                    || self.expect(TokenKind::Eof))
                {
                    return Err(ParserError::TupleClose);
                }

                self.eat_dot()?;
                Ok(Some(res))
            }
            TokenKind::Undefined => Err(ParserError::Undefined),
            TokenKind::Eof => Ok(None),
        }
    }

    /// parse `[0]` or `[abc]`
    fn parse_bracket(&mut self) -> Result<FieldName, ParserError> {
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
                            .map_err(|_| ParserError::ParseArrayIndex)?,
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
                        return Err(ParserError::ArrayClose);
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
                    let str;
                    (str, self.source) = self.source.split_at(t.len);
                    let name = FieldName::StructVariant(str.to_owned());

                    // eat ident
                    self.token.advance();
                    // eat `]`
                    self.token.advance();
                    self.source = &self.source[1..];

                    if !(self.expect(TokenKind::Dot)
                        || self.expect(TokenKind::LeftBracket)
                        || self.expect(TokenKind::Eof))
                    {
                        return Err(ParserError::ArrayClose);
                    }

                    self.eat_dot()?;
                    return Ok(name);
                }
            }
            _ => return Err(ParserError::BracketSyntaxError),
        }

        Err(ParserError::BracketSyntaxError)
    }

    fn expect(&self, token: TokenKind) -> bool {
        let peek = self.token.clone().advance();
        token == peek.kind
    }

    fn eat_dot(&mut self) -> Result<(), ParserError> {
        let mut peek = self.token.clone();
        if let Token {
            kind: TokenKind::Dot,
            ..
        } = peek.advance()
        {
            let Token { kind, .. } = peek.advance();
            match kind {
                TokenKind::Eof => return Err(ParserError::DotIsLast),
                TokenKind::LeftBracket => return Err(ParserError::DotTieLeftBracket),
                _ => (),
            }
            self.token.advance();
            self.source = &self.source[1..];
        }

        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn parse(source: &str) -> Result<Vec<FieldName>, ParserError> {
    let mut parser = Parser::new(source);

    let mut vec = Vec::new();
    loop {
        match parser.next_name()? {
            Some(name) => vec.push(name),
            None => break Ok(vec),
        }
    }
}

pub fn parse_message(source: &str) -> Result<MessageKey, String> {
    let (name_str, string) = source
        .rsplit_once('.')
        .ok_or("not found message".to_owned())?;

    Ok(MessageKey::new(
        FieldNames::new(name_str.to_string()),
        string.to_string(),
    ))
}

#[derive(Debug)]
pub(crate) enum ParserError {
    DotStart,
    BracketRight,
    ParseTupleIndex,
    TupleClose,
    Undefined,
    ParseArrayIndex,
    ArrayClose,
    BracketSyntaxError,
    DotIsLast,
    DotTieLeftBracket,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserError::*;
        match self {
            DotStart => "`.` should not be start".fmt(f),
            BracketRight => "`]` should to stay behind `[`".fmt(f),
            ParseTupleIndex => "tuple index is not u8 type".fmt(f),
            TupleClose => "after tuple index should be `.` or `[` or eof".fmt(f),
            Undefined => "undefined character".fmt(f),
            ParseArrayIndex => "array index is not usize type".fmt(f),
            ArrayClose => "after `]` should be `.` or `[` or eof".fmt(f),
            BracketSyntaxError => "bracket syntax error".fmt(f),
            DotIsLast => "`.` should not be end".fmt(f),
            DotTieLeftBracket => "after `.` should not be `[`".fmt(f),
        }
    }
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
