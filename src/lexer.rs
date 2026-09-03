
use crate::lexer::Keyword::{For, While, Break, Let };
#[derive(Debug)]
pub struct Lexer<'a> {
    pub position: usize,
    pub input: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
#[derive(Debug, Clone, PartialEq)]

pub struct Span {
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone, PartialEq)]

pub enum TokenKind {
    Keyword(Keyword),   // let keyword
    Identifier(String), // keyword
    Integer(i64),       // integer
    True,
    False,
    Equal,     // ==
    Assign,    // =
    Semicolon, // ;
    NOT,       // !
    Plus,      // +
    Minus,     // -
    Mul,       // *
    Div,       // /
    EOF,       // end
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    For,
    While,
    Break,
}
impl<'a> Lexer<'a> {
    // now
    fn current(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    // the next
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    // go to the next pointer
    fn advance(&mut self) {
        self.position += 1;
    }
    fn read_identifier(&mut self) -> Token {
        let mut vec = vec![];
        loop {
            match self.current() {
                Some(c) if c.is_ascii_digit() || c == '_' || c.is_ascii_alphabetic() => {
                    vec.push(c);
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
        let len = vec.len();
        let s: String = vec.into_iter().collect();
        // lookup
        let result = match s.as_str() {
            // "let" => Token::Keyword(Keyword::Let),
            "let" => Token {
                span: Span {
                    start: self.position - len,
                    end: self.position,
                },
                kind: TokenKind::Keyword(Let),
            },
            "for" => Token {
                span: Span {
                    start: self.position - len,
                    end: self.position,
                },
                kind: TokenKind::Keyword(For),
            },
            "break" => Token {
                span: Span {
                    start: self.position - len,
                    end: self.position,
                },
                kind: TokenKind::Keyword(Break),
            },
            "while" => Token {
                span: Span {
                    start: self.position - len,
                    end: self.position,
                },
                kind: TokenKind::Keyword(While),
            },
            _ => Token {
                span: Span {
                    start: self.position - len,
                    end: self.position,
                },
                kind: TokenKind::Identifier(s),
            },
        };

        result
    }

    fn read_integer(&mut self) -> Token {
        let mut vec = vec![];
        loop {
            match self.current() {
                Some(i) if i.is_ascii_digit() => {
                    vec.push(i);
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
        let len = vec.len();
        let s: String = vec.into_iter().collect();
        let int: i64 = s.parse().unwrap();
        Token {
            span: Span {
                start: self.position - len,
                end: self.position,
            },
            kind: TokenKind::Integer(int),
        }
    }

    //
    pub fn get_token(&mut self) -> Vec<Token> {
        let mut result = vec![];
        while &self.position < &self.input.chars().count() {
            if let Some(c) = self.current() {
                if c.is_ascii_alphabetic() {
                    result.push(self.read_identifier());
                    continue;
                }

                if c.is_ascii_digit() {
                    result.push(self.read_integer());
                    continue;
                }

                if c == '+' {
                    result.push(Token {
                        span: Span {
                            start: self.position,
                            end: self.position + 1,
                        },
                        kind: TokenKind::Plus,
                    });
                    self.advance();
                }

                if c == '-' {
                    result.push(Token {
                        span: Span {
                            start: self.position,
                            end: self.position + 1,
                        },
                        kind: TokenKind::Minus,
                    });
                    self.advance();
                }
                if c == '*' {
                    result.push(Token {
                        span: Span {
                            start: self.position,
                            end: self.position + 1,
                        },
                        kind: TokenKind::Mul,
                    });
                    self.advance();
                }
                if c == '/' {
                    result.push(Token {
                        span: Span {
                            start: self.position,
                            end: self.position + 1,
                        },
                        kind: TokenKind::Div,
                    });
                    self.advance();
                }

                if c == '!' {
                    result.push(Token {
                        span: Span {
                            start: self.position,
                            end: self.position + 1,
                        },
                        kind: TokenKind::NOT,
                    });
                    self.advance();
                }

                if c == '=' {
                    match self.peek() {
                        Some(c) if c == '=' => {
                            result.push(Token {
                                span: Span {
                                    start: self.position,
                                    end: self.position + 2,
                                },
                                kind: TokenKind::Equal,
                            });
                            self.advance();
                            self.advance();
                        }
                        _ => {
                            // only one equal symbol
                            result.push(Token {
                                span: Span {
                                    start: self.position,
                                    end: self.position + 1,
                                },
                                kind: TokenKind::Assign,
                            });
                            self.advance();
                        }
                    }
                }
                if c == ' ' {
                    self.advance();
                }
                if c == ';' {
                    result.push(Token {
                        span: Span {
                            start: self.position,
                            end: self.position + 1,
                        },
                        kind: TokenKind::Semicolon,
                    });
                    self.advance();
                }
            }
        }

        result
    }
}
