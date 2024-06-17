use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    // single character tokens
    LeftPar,
    RightPar,
    LeftCurl,
    RightCurl,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColumn,
    Slash,
    Star,

    // one for two
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literal
    STRING,
    NUMBER,
    Identifier,

    // keywords
    And,
    Or,
    If,
    Else,
    ElseIf,
    False,
    True,
    Class,
    For,
    Null,
    Function,
    Print,
    Return,
    Super,
    This,
    Declare,
    While,

    // end
    Eof,
}

// #[derive(Debug)]
// pub enum TokenLiterals {
//     String,
//     Number,
//     Identifier,
// }

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.r#type, self.lexeme, self.literal)
    }
}
