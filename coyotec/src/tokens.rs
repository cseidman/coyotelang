pub enum TokenType {
    Integer(i64),
    Float(f64),
    Text(Box<String>),
    Boolean(bool),
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
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LessThan,
    GreaterThan,
    Ampersand,
    Pipe,
    Caret,
    Hash,
    At,
    Question,
    Newline,
    Dollar,
    Quote,
    DataType,
    EOF,
}
#[derive(Clone, Copy)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 0,
        }
    }

    pub fn newline(&mut self) {
        self.line+=1;
        self.column=0;
    }

    pub fn increment(&mut self, by: usize) {
        self.column+=1;
    }
}


pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) location: Location
}


#[cfg(test)]
mod test {
    use crate::tokens::Location;

    #[test]
    fn test_location() {
        let mut loc = Location::new();
        assert_eq!(loc.line,1);
        assert_eq!(loc.column,0);
        loc.increment(1) ;
    }
}