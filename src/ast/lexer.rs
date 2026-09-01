
#[derive(Debug)]
pub struct Lexer <'a> {
    pub position: usize,
    pub input: &'a str,
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword), // let keyword
    Identifier(String), // keyword
    Integer(i64), // integer
    Equal, // ==
    Assign, // =
    Semicolon,  // ;
    Plus, // +
}
#[derive(Debug)]
pub enum Keyword {
    Let,
    For,
    While,
    Break,
}
impl<'a> Lexer <'a> {

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
        self.position +=1;
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
        let s: String = vec.into_iter().collect();
        // lookup
        let result = match s.as_str() {
            "let" => Token::Keyword(Keyword::Let),
            "for" => Token::Keyword(Keyword::For),
            "break" => Token::Keyword(Keyword::Break),
            "while" => Token::Keyword(Keyword::While),
            _ => Token::Identifier(s)
        };

        result
        
    }
    
    fn read_integer(&mut self)  -> Token {
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
        let s: String = vec.into_iter().collect();
        let int: i64 = s.parse().unwrap();
        Token::Integer(int)
    }

    
    //
    pub fn get_token(&mut self) -> Vec<Token> {
        let mut result = vec![];
        while &self.position < &self.input.chars().count()  {
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
                    result.push(Token::Plus);
                    self.advance();
                }

                if c == '=' {
                    match self.peek() {
                        Some(c) if c == '=' => {
                            result.push(Token::Equal);
                            self.advance();
                            self.advance();
                        }
                        _ => {
                            // only one equal symbol
                            result.push(Token::Assign);
                            self.advance();
                        }
                    }
                }
                if c == ' ' {
                    self.advance();
                }
                if c == ';' {
                    result.push(Token::Semicolon);
                    self.advance();
                }
            }


        }

        result
    }

}
