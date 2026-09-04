use crate::error::{CompilerError, UnexpectedCharacter};
use crate::lexer::Keyword::{Break, For, Let, While};
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
    fn read_identifier(&mut self) -> Result<Token, CompilerError> {
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

        Ok(result)
    }

    fn read_integer(&mut self) -> Result<Token, CompilerError> {
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
        Ok(Token {
            span: Span {
                start: self.position - len,
                end: self.position,
            },
            kind: TokenKind::Integer(int),
        })
    }

    fn next_token(&mut self) -> Result<Token, CompilerError> {
        let result = match self.current() {
            Some(c) if c.is_ascii_alphabetic() => self.read_identifier(),
            Some(c) if c.is_ascii_digit() => self.read_integer(),
            Some('+') => {
                self.advance();
                Ok(Token {
                    span: Span {
                        start: self.position - 1,
                        end: self.position,
                    },
                    kind: TokenKind::Plus,
                })
            }
            Some('-') => {
                self.advance();
                Ok(Token {
                    span: Span {
                        start: self.position - 1,
                        end: self.position,
                    },
                    kind: TokenKind::Minus,
                })
            }
            Some('*') => {
                self.advance();
                Ok(Token {
                    span: Span {
                        start: self.position - 1,
                        end: self.position,
                    },
                    kind: TokenKind::Mul,
                })
            }
            Some('/') => {
                self.advance();
                Ok(Token {
                    span: Span {
                        start: self.position - 1,
                        end: self.position,
                    },
                    kind: TokenKind::Div,
                })
            }
            Some('!') => {
                self.advance();
                Ok(Token {
                    span: Span {
                        start: self.position - 1,
                        end: self.position,
                    },
                    kind: TokenKind::NOT,
                })
            }
            Some(';') => {
                self.advance();
                Ok(Token {
                    span: Span {
                        start: self.position - 1,
                        end: self.position,
                    },
                    kind: TokenKind::Semicolon,
                })
            }
            Some('=') => match self.peek() {
                Some(c) if c == '=' => {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        span: Span {
                            start: self.position - 2,
                            end: self.position,
                        },
                        kind: TokenKind::Equal,
                    })
                }
                _ => {
                    self.advance();
                    Ok(Token {
                        span: Span {
                            start: self.position - 1,
                            end: self.position,
                        },
                        kind: TokenKind::Assign,
                    })
                }
            },
            Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                self.advance();
                self.next_token()
            }
            None => Ok(Token {
                span: Span {
                    start: self.position,
                    end: self.position,
                },
                kind: TokenKind::EOF,
            }),
            _ => {
                self.advance();
                Err(CompilerError::UnexpectedCharacter(UnexpectedCharacter {
                    span: Span {
                        start: self.position - 1,
                        end: self.position,
                    },
                    message: String::from("compile error, found unexpected chars"),
                }))
            }
        };
        result
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompilerError> {
        let mut result = vec![];
        while &self.position < &self.input.chars().count() {
            match self.next_token() {
                Ok(t) => {
                    result.push(t);
                }
                Err(e) => {
                    return Err(e);
                }
            };
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::lexer::TokenKind::*;
    #[test]
    fn keyword_let_token() {
        let a = "let a = 1*2+3";
        let mut lexer = Lexer {
            input: a,
            position: 0,
        };
        let result = lexer.tokenize();
        println!("{:?}", result);
        // assert_eq!(result, [Token { kind: Keyword(Let), span: Span { start: 0, end: 3 } }, Token { kind: Identifier("a".to_string()), span: Span { start: 4, end: 5 } }, Token { kind: Assign, span: Span { start: 6, end: 7 } }, Token { kind: Integer(1), span: Span { start: 8, end: 9 } }, Token { kind: Mul, span: Span { start: 9, end: 10 } }, Token { kind: Integer(2), span: Span { start: 10, end: 11 } }, Token { kind: Plus, span: Span { start: 11, end: 12 } }, Token { kind: Integer(3), span: Span { start: 12, end: 13 } }])
    }
}
