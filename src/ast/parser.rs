
use crate::ast::lexer::Token;
use crate::ast::ast::{ VariableDeclaration, Expression };
use crate::ast::ast::Identifier::StringLiteral;
#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position:usize
}

impl Parser {
    pub fn parse_program(&mut self) {
        while self.position < self.tokens.len() {
            match self.current_token() {
                Token::Keyword(Let) => {

                    // expect :VariableDeclaration { name: StringLiteral("a"), initializer: Identifier("1") }
                    println!("{:?}", self.parse_variable_declaration());
                },
                _ => {
                    println!("to do");
                    self.advance();
                }
            }
        }
    }

    /**
     *   expect: [ Keyword(Let),  Identifier("a"),  Assign, 
     *   Identifier("1"), Semicolon]
     */

    fn parse_variable_declaration(&mut self) -> VariableDeclaration {
        let mut name = StringLiteral("b".to_string());
        let mut initializer = Expression::Identifier("1".to_string());
        loop {
            match self.current_token() {
                Token::Identifier(c)=> {
                    name = StringLiteral(c.to_string());
                    self.advance();
                }
                Token::Integer(e) => {
                    initializer = Expression::Identifier(e.to_string());
                    self.advance();
                }
                Token::Semicolon => {
                    break;
                }
                _ => {
                    // TODO:
                    self.advance();
                }
            }
        }
        VariableDeclaration {
            name,
            initializer
        }
    }
    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }
    fn advance(&mut self) {
        self.position +=1;
    }
}