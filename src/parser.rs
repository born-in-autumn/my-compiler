use crate::ast::{
    BinaryExpression, BinaryOperator, Declaration, Expression, Operator, PrimaryExpression,
    Program, UnaryExpression, VariableDeclaration,
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
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, CompilerError> {
        let mut name = String::from("");
        let mut initializer = None;

        // 根据文法递归下降解析
        // 直接看第一次匹配是不是let
        match self.current_token().kind {
            TokenKind::Keyword(Let) => {
                self.advance();
            }
            _ => {
                return Err(CompilerError::UnexpectedToken(UnexpectedToken {
                    message: format!("unexpected token1"),
                    span: Span {
                        start: self.position,
                        end: self.position + 3,
                    },
                }));
            }
        }
        // 直接看第二次匹配是不是变量名字
        match self.current_token().kind {
            TokenKind::Identifier(s) => {
                println!("{:?}", name);
                name = s;
                self.advance();
            }
            _ => {
                return Err(CompilerError::UnexpectedToken(UnexpectedToken {
                    message: format!("unexpected token2"),
                    span: Span {
                        start: self.position,
                        end: self.position + 1,
                    },
                }));
            }
        }
        // 第三次匹配看是不是Assign或者分号，如果是分号，直接结束，如果是等于号，继续看下面的
        match self.current_token().kind {
            TokenKind::Assign => {
                self.advance();
                initializer = Some(self.parse_expression()?);
            }
            TokenKind::Semicolon => {
                self.advance();
                return Ok(VariableDeclaration { name, initializer });
            }
            _ => {
                return Err(CompilerError::UnexpectedToken(UnexpectedToken {
                    message: format!("unexpected token3"),
                    span: Span {
                        start: self.position,
                        end: self.position + 1,
                    },
                }));
            }
        }
        // 最后看有没有分号
        match self.current_token().kind {
            TokenKind::Semicolon => {
                self.advance();
            }
            _ => {
                return Err(CompilerError::UnexpectedToken(UnexpectedToken {
                    message: format!("unexpected token4"),
                    span: Span {
                        start: self.position,
                        end: self.position + 1,
                    },
                }));
            }
        }
        Ok(VariableDeclaration { name, initializer })
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
    // fn next_token(&self) -> Token {
    //     if self.position >= self.tokens.len() - 1 {
    //         return Token {
    //             span: Span {
    //                 start: self.tokens.len(),
    //                 end: self.tokens.len(),
    //             },
    //             kind: TokenKind::EOF,
    //         };
    //     }
    //     self.tokens[self.position + 1].clone()
    // }
    fn advance(&mut self) {
        self.position += 1;
    }
    // -1 + 2 * 3 + 2
    // 1 - (2 - 3)
    // let a = 4 * （-1 + 2 * 3）* 3 * (5+ 2)
    fn parse_expression(&mut self) -> Result<Expression, CompilerError> {
        self.parse_add()
    }

    // + -
    fn parse_add(&mut self) -> Result<Expression, CompilerError> {
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
                // 这里没有消耗任何Token，所以不advance，并且应该直接返回PrimaryExpression
                match self.parse_primary() {
                    Ok(epr) => {
                        Ok(Expression::PrimaryExpression(epr))
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn parse_primary(&mut self) -> Result<PrimaryExpression, CompilerError> {
        match self.current_token().kind {
            TokenKind::Integer(i) => {
                self.advance();
                Ok(PrimaryExpression::IntegerLiteral(i))
            }
            TokenKind::Identifier(s) => {
                self.advance();
                Ok(PrimaryExpression::Identifier(s))
            }
            TokenKind::LeftParen => {
                let result = self.parse_paren();
                match result {
                    Ok(r) => {
                        Ok(PrimaryExpression::Expression(Box::new(r)))
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            _ => Err(CompilerError::UnexpectedToken(UnexpectedToken {
                message: String::from("unexpected token"),
                span: Span {
                    start: self.position,
                    end: self.position,
                },
            })),
        }
    }

        // let a = 4 * （-1 + 2 * 3） * 2
    fn parse_paren(&mut self) -> Result<Expression, CompilerError> {
        match self.current_token().kind {
            TokenKind::LeftParen => {
                self.advance();
            }
            _ => {
                return Err(CompilerError::UnexpectedToken(UnexpectedToken {
                    message: format!("unexpected token"),
                    span: Span {
                        start: self.position,
                        end: self.position + 1,
                    },
                }));
            }
        };
        let expression = self.parse_expression();
        match self.current_token().kind {
            TokenKind::RightParen => {
                self.advance();
            }
            _ => {
                return Err(CompilerError::UnexpectedToken(UnexpectedToken {
                    message: format!("unexpected token"),
                    span: Span {
                        start: self.position,
                        end: self.position + 1,
                    },
                }));
            }
        }

        expression
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::Lexer;
    #[test]
    fn parse_program_test() {
        let a = "let a = - 1 + 2 * 3";
        let mut lexer = Lexer {
            input: a,
            position: 0,
        };
        let result = lexer.tokenize();
        println!("{:?}", result);
        let mut p = Parser {
            tokens: result.unwrap(),
            position: 0,
        };
        let ast = p.parse_program();
        match ast {
            Ok(node) => {
                assert_eq!(node.declarations.len(), 1);
            }
            Err(_) => {}
        }
    }
}
