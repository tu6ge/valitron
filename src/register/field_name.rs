use std::{fmt::Display, slice::Iter};

use super::lexer::{lexer, Token, TokenKind};

#[derive(Debug, PartialEq, Eq, Hash)]
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

struct Parser<'a> {
    names: Vec<FieldName>,
    tokens: Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Iter<'a, Token>) -> Self {
        Self {
            names: Vec::new(),
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        while let Some(token) = self.tokens.next() {
            match token.kind() {
                TokenKind::Ident(s) => {
                    self.names.push(FieldName::Literal(s.to_owned()));
                    self.eat_point()?;
                }
                TokenKind::Point => return Err("`.` should not be start".into()),
                TokenKind::LeftBracket => {
                    self.parse_bracket()?;
                }
                TokenKind::RightBracket => return Err("`]` should to stay behind `[`".into()),
                TokenKind::Index(n) => {
                    self.names.push(FieldName::Tuple(
                        (*n).try_into()
                            .map_err(|_| "tuple index is not u8 type".to_string())?,
                    ));
                    if !(self.expect(TokenKind::Point)
                        || self.expect(TokenKind::LeftBracket)
                        || self.expect(TokenKind::Eof))
                    {
                        return Err("after tuple index should be `.` or `[` or eof".into());
                    }
                    self.eat_point()?;
                }
                TokenKind::Undefined => return Err("undefined char".into()),
                TokenKind::Eof => (),
            }
        }

        Ok(())
    }

    /// parse `[0]` or `[abc]`
    fn parse_bracket(&mut self) -> Result<(), String> {
        let mut peek = self.tokens.clone().peekable();
        if let Some(t) = peek.next() {
            match t.kind() {
                TokenKind::Index(n) => {
                    if let Some(Token {
                        kind: TokenKind::RightBracket,
                        ..
                    }) = peek.next()
                    {
                        self.names.push(FieldName::Array(*n));
                        // eat index
                        self.tokens.next();
                        // eat `]`
                        self.tokens.next();
                        if !(self.expect(TokenKind::Point)
                            || self.expect(TokenKind::LeftBracket)
                            || self.expect(TokenKind::Eof))
                        {
                            return Err("after `]` should be `.` or `[` or eof".into());
                        }
                        self.eat_point()?;
                        return Ok(());
                    }
                }
                TokenKind::Ident(s) => {
                    if let Some(Token {
                        kind: TokenKind::RightBracket,
                        ..
                    }) = peek.next()
                    {
                        self.names.push(FieldName::StructVariant(s.to_owned()));
                        // eat ident
                        self.tokens.next();
                        // eat `]`
                        self.tokens.next();
                        if !(self.expect(TokenKind::Point)
                            || self.expect(TokenKind::LeftBracket)
                            || self.expect(TokenKind::Eof))
                        {
                            return Err("after `]` should be `.` or `[` or eof".into());
                        }
                        self.eat_point()?;
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

        if let Some(Token { kind }) = peek.next() {
            if &token == kind {
                return true;
            }
        }

        false
    }

    fn eat_point(&mut self) -> Result<(), String> {
        let mut peek = self.tokens.clone().peekable();
        if let Some(Token {
            kind: TokenKind::Point,
            ..
        }) = peek.next()
        {
            if let Some(Token { kind }) = peek.next() {
                match kind {
                    TokenKind::Eof => return Err("`.` should not be end".into()),
                    TokenKind::LeftBracket => return Err("after `.` should not be `[`".into()),
                    _ => (),
                }
            }
            self.tokens.next();
        }

        Ok(())
    }
}

pub fn parse(source: &str) -> Result<Vec<FieldName>, String> {
    let tokens = lexer(source).unwrap();
    let mut parser = Parser::new(tokens.iter());
    parser.parse()?;

    Ok(parser.names)
}

pub fn parse_message(source: &str) -> Result<(Vec<FieldName>, FieldName), String> {
    let mut names = parse(source)?;

    if let Some(name) = names.pop() {
        if let FieldName::Literal(_) = name {
            return Ok((names, name));
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
