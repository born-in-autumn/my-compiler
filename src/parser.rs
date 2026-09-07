use crate::ast::{
    BinaryExpression,
    Declaration, Expression, Operator,  PrimaryExpression,
    Program, UnaryExpression, VariableDeclaration,BinaryOperator
};
use crate::error::{CompilerError, UnexpectedToken};
use crate::lexer::{Keyword::Let, Span, Token, TokenKind};
#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn parse_program(&mut self) -> Result<Program, CompilerError> {
        let mut declarations: Vec<Declaration> = vec![];
        while self.current_token().kind != TokenKind::EOF {
            match self.current_token().kind {
                TokenKind::Keyword(Let) => {
                    // expect :VariableDeclaration { name: StringLiteral("a"), initializer: Identifier("1") }
                    declarations.push(Declaration::VariableDeclaration(
                        self.parse_variable_declaration()?,
                    ));
                }
                _ => {
                    self.advance();
                }
            }
        }
        Ok(Program { declarations })
    }

    /**
     *   expect: [ Keyword(Let),  Identifier("a"),  Assign,
     *   Identifier("1"), Semicolon]
     */
    // let a = -1 + 2 * 3;
    // let expect = TokenKind::Keyword(Let);
    //     loop {
    //         match self.current_token().kind {
                
    //         }
    //     }
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, CompilerError> {
        let mut name = String::from("");
        let mut initializer = Expression::Identifier(String::from(""));
        loop {
            match self.current_token().kind {
                TokenKind::Keyword(Let) => match self.next_token().kind {
                    TokenKind::Identifier(t) => {
                        name = t;
                        self.advance();
                    }
                    _ => {
                        println!("expect identifier but found {:?}", self.next_token());
                        break;
                    }
                },
                TokenKind::Identifier(_c) => match self.next_token().kind {
                    TokenKind::Assign => {
                        self.advance();
                    }
                    _ => {
                        println!("expect Assign symbol but found {:?}", self.next_token());
                        break;
                    }
                },
                TokenKind::Assign => match self.next_token().kind {
                    TokenKind::Minus => {
                        self.advance();
                        initializer = self.parse_expression()?;
                    }
                    TokenKind::Integer(_t) => {
                        self.advance();
                        initializer = self.parse_expression()?;
                    }
                    _ => {
                        println!("expect Expression but found {:?}", self.next_token());
                        break;
                    }
                },
                TokenKind::Integer(_e) => match self.next_token().kind {
                    TokenKind::Semicolon => {
                        self.advance();
                    }
                    _ => {
                        println!("expect Semicolon symbol but found {:?}", self.next_token());
                        break;
                    }
                },
                TokenKind::Semicolon => {
                    break;
                }
                _ => {
                    self.advance();
                    break;
                }
            }
        }
        Ok(VariableDeclaration { name, initializer: Some(initializer) })
    }
    fn current_token(&self) -> Token {
        if self.position >= self.tokens.len() {
            return Token {
                span: Span {
                    start: self.tokens.len(),
                    end: self.tokens.len(),
                },
                kind: TokenKind::EOF,
            };
        }
        self.tokens[self.position].clone()
    }
    fn next_token(&self) -> Token {
        if self.position >= self.tokens.len() - 1 {
            return Token {
                span: Span {
                    start: self.tokens.len(),
                    end: self.tokens.len(),
                },
                kind: TokenKind::EOF,
            };
        }
        self.tokens[self.position + 1].clone()
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    // -1 + 2 * 3 + 2
    // 1 - 2 - 3
    fn parse_expression(&mut self) -> Result<Expression, CompilerError> {
        self.parse_add()
    }

    // + -
    fn parse_add(&mut self) -> Result<Expression, CompilerError>{
        let mut left = self.parse_mul();
        loop {
            match self.current_token().kind {
                TokenKind::Plus => {
                    self.advance();
                    let result = self.parse_mul();
                    left = Ok(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left?),
                        operator: BinaryOperator::Plus,
                        right: Box::new(result?),
                    }))
                }
                TokenKind::Minus => {
                    self.advance();
                    let result = self.parse_mul();
                    left = Ok(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left?),
                        operator: BinaryOperator::Minus,
                        right: Box::new(result?),
                    }))
                }
                _ => {
                    break;
                }
            }
        }
        left
    }
    // * /
    // -1 + 2 * 3 / 4 + -4
    fn parse_mul(&mut self) -> Result<Expression, CompilerError> {
        let mut left = self.parse_unary();
        loop {
            match self.current_token().kind {
                TokenKind::Mul => {
                    self.advance();
                    let result = self.parse_unary();
                    left = Ok(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left?),
                        operator: BinaryOperator::Mul,
                        right: Box::new(result?),
                    }))
                }
                TokenKind::Div => {
                    self.advance();
                    let result = self.parse_unary();
                    left = Ok(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left?),
                        operator: BinaryOperator::Div,
                        right: Box::new(result?),
                    }))
                }
                _ => {
                    break;
                }
            }
        }
        left
    }
    // - or !
    fn parse_unary(&mut self) -> Result<Expression, CompilerError> {
        match self.current_token().kind {
            TokenKind::Minus => {
                self.advance();
                Ok(Expression::UnaryExpression(UnaryExpression {
                    prefix: Some(Operator::UnaryOperator(super::ast::UnaryOperator::Minus)),
                    value: self.parse_primary()?,
                }))
            }
            _ => {
                // 这里没有消耗任何Token，所以不advance
                Ok(Expression::UnaryExpression(UnaryExpression {
                    prefix: None,
                    value: self.parse_primary()?,
                }))
            }
        }
    }

    fn parse_primary(&mut self) -> Result<PrimaryExpression, CompilerError> {
        match self.current_token().kind {
            TokenKind::Integer(i) => {
                self.advance();
                Ok(PrimaryExpression::IntegerLiteral(i))
            }
            _ => {
                Err(CompilerError::UnexpectedToken(UnexpectedToken {
                    message: String::from("unexpected token"),
                    span: Span {
                        start: self.position,
                        end: self.position
                    }
                }))
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::Lexer;
    #[test]
    fn parse_program_test()  {
        let a = "let a = - 1 + 2 * 3";
        let mut lexer = Lexer {
            input: a,
            position: 0
        };
        let result = lexer.tokenize();
        println!("{:?}", result);
        let mut p = Parser {
            tokens: result.unwrap(),
            position: 0
        };
        let ast = p.parse_program();
        match ast {
            Ok(node) => {
                assert_eq!(node.declarations.len(), 1);
            },
            Err(_) => {}
        }
    }
}