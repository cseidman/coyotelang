use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum BaseType {
    NoType,
    Undefined,
    Integer,
    Float,
    Text,
    Boolean,
    Array,
    List,
    Struct,
}

impl Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::NoType => write!(f, "NoType"),
            BaseType::Undefined => write!(f, "Undefined"),
            BaseType::Integer => write!(f, "Integer"),
            BaseType::Float => write!(f, "Float"),
            BaseType::Text => write!(f, "Text"),
            BaseType::Boolean => write!(f, "Boolean"),
            BaseType::Array => write!(f, "Array"),
            BaseType::List => write!(f, "List"),
            BaseType::Struct => write!(f, "Struct"),
        }
    }
}

impl BaseType {
    pub fn get_prefix(&self) -> String {
        match self {
            BaseType::NoType => "".to_string(),
            BaseType::Integer => "i".to_string(),
            BaseType::Float => "f".to_string(),
            BaseType::Text => "t".to_string(),
            BaseType::Boolean => "b".to_string(),
            BaseType::Array => "a".to_string(),
            BaseType::List => "l".to_string(),
            BaseType::Struct => "s".to_string(),
            BaseType::Undefined => "u".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Integer(f64), // Integers are represented by floats
    Float(f64),
    Text(String),
    Boolean(bool),
    Struct(HashMap<String, BaseType>),
    LBracket,
    RBracket,
    LParen,
    RParen,
    Bang,
    LBrace,
    RBrace,
    Dot,
    Comma,
    SemiColon,
    Colon,
    EqualEqual,
    NotEqual,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LessThan,
    GreaterThan,
    EqualLessThan,
    EqualGreaterThan,
    Ampersand,
    Pipe,
    Caret,
    Hash,
    At,
    Question,
    Newline,
    Dollar,
    Quote,
    Underscore,
    DataType(BaseType),
    Identifier(String),
    Let,
    Func,
    Print,
    EOF,
    If,
    Else,
    ElseIf,
    EndIf,
    Not,
    And,
    Or,
    For,
    In,
    To,
    EndFor,
    While,
    EndWhile,
    Break,
    Continue,
}
#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self { line: 1, column: 0 }
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    pub fn increment(&mut self, by: usize) {
        self.column += by;
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
}

#[cfg(test)]
mod test {
    use crate::tokens::Location;

    #[test]
    fn test_location() {
        let mut loc = Location::new();
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 0);
        loc.increment(1);
    }
}
