use crate::scan::token::{
    Token,
    TokenType::{self, *},
};

pub struct Scanner {
    pub source: String,
    pub had_error: bool,
    pub list_tokens: Vec<Token>,
    pub current: usize,
    pub start: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            had_error: false,
            list_tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn is_end(&mut self) -> bool {
        if self.current > self.source.len() {
            return true;
        }
        false
    }

    pub fn scan_tokens(&mut self) {
        loop {
            if !self.is_end() {
                self.scan_token();
            } else {
                break;
            }
        }

        self.list_tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        println!("{:?}", self.list_tokens)
    }

    pub fn scan_token(&mut self) {
        let symbol = self.move_cursor();
        match symbol {
            '(' => self.add_token(LeftPar),
            ')' => self.add_token(RightPar),
            '{' => self.add_token(LeftCurl),
            '}' => self.add_token(RightCurl),
            '=' => {
                if self.r#match('=') {
                    self.add_token(EqualEqual);
                } else {
                    self.add_token(Equal);
                }
            }

            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            '*' => self.add_token(Star),
            ';' => self.add_token(SemiColumn),
            '/' => self.add_token(Slash),
            '!' => {
                if self.r#match('=') {
                    self.add_token(NotEqual);
                } else {
                    self.add_token(Not);
                }
            }

            '<' => {
                if self.r#match('=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less)
                }
            }

            '>' => {
                if self.r#match('=') {
                    self.add_token(GreaterEqual)
                } else {
                    self.add_token(Greater)
                }
            }
            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        self.move_cursor();
                    }
                } else {
                    self.add_token(Slash)
                }
            }

            '"' => 'string_literal: {
                self.string();
                break 'string_literal;
            }

            ' ' | '\t' | '\r' => 'ignore: {
                break 'ignore;
            }

            '\n' => 'line_break: {
                self.line += 1;
                break 'line_break;
            }

            _ => {
                if self.is_digit(symbol) {
                    self.number();
                } else {
                    panic!(
                        "{}",
                        self.error(self.line, "unexpected character found".to_string())
                    );
                }
            }
        };
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_fn(token_type, None);
    }

    pub fn add_token_fn(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current - 1];
        self.list_tokens.push(Token::new(
            token_type,
            text.to_string(),
            Some(literal.expect("unbale to wrap literal value -> line:68")),
            self.line,
        ));
        self.line += 1;
    }

    pub fn move_cursor(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    pub fn r#match(&mut self, expected: char) -> bool {
        if self.is_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    pub fn peek(&mut self) -> char {
        if self.is_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    pub fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            if self.is_end() {
                panic!(
                    "{}",
                    self.error(self.line, "unterminated string literal".to_string())
                );
            }

            self.move_cursor();
            let string_value = &self.source[self.start + 1..self.current + 1];
            self.add_token_fn(STRING, Some(string_value.to_string()));
        }
    }

    pub fn is_digit(&mut self, c: char) -> bool {
        if c >= '0' || c <= '9' {
            return true;
        }
        return false;
    }

    pub fn number(&mut self) {
        let next = { self.peek() };

        while self.is_digit(next) {
            self.move_cursor();
        }

        let next_char = { self.peek_next() };
        if self.peek() == '.' && self.is_digit(next_char) {
            self.move_cursor();
            let next_fract = { self.peek() };
            while self.is_digit(next_fract) {
                self.move_cursor();
            }
        }

        let num_lit = &self.source[self.start..self.current];
        self.add_token_fn(NUMBER, Some(num_lit.to_string()))
    }

    pub fn peek_next(&mut self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(self.current + 1).unwrap();
    }

    pub fn error(&mut self, line: usize, message: String) -> String {
        self.report(line, "".to_string(), message)
    }

    pub fn report(&mut self, line: usize, r#where: String, message: String) -> String {
        self.had_error = true;
        format!(
            "[line:{}],where:{},error: message:{}",
            line, r#where, message
        )
    }
}
