use crate::ast::{
    AddOperator::Minus, AddOperator::Plus, BinaryExpression, BinaryOperator::AddOperator,
    BinaryOperator::MulOperator, Declaration, Expression, Identifier::StringLiteral,
    MulOperator::Div, MulOperator::Mul, Operator, Operator::BinaryOperator, PrimaryExpression,
    Program, UnaryExpression, VariableDeclaration,
};
use crate::lexer::{Keyword::Let, Span, Token, TokenKind};
#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn parse_program(&mut self) -> Program {
        let mut declarations: Vec<Declaration> = vec![];
        while self.position < self.tokens.len() {
            match self.current_token().kind {
                TokenKind::Keyword(Let) => {
                    // expect :VariableDeclaration { name: StringLiteral("a"), initializer: Identifier("1") }
                    declarations.push(Declaration::VariableDeclaration(
                        self.parse_variable_declaration(),
                    ));
                }
                _ => {
                    self.advance();
                }
            }
        }
        Program { declarations }
    }

    /**
     *   expect: [ Keyword(Let),  Identifier("a"),  Assign,
     *   Identifier("1"), Semicolon]
     */
    // let a = -1 + 2 * 3;
    fn parse_variable_declaration(&mut self) -> VariableDeclaration {
        let mut name = StringLiteral("Error".to_string());
        let mut initializer = Expression::Identifier("Error".to_string());
        loop {
            match self.current_token().kind {
                TokenKind::Keyword(Let) => match self.next_token().kind {
                    TokenKind::Identifier(t) => {
                        name = StringLiteral(t.to_string());
                        self.advance();
                    }
                    _ => {
                        println!("expect identifier but found {:?}", self.next_token());
                        break;
                    }
                },
                TokenKind::Keyword(_for) => {
                    break;
                }
                TokenKind::Identifier(_c) => match self.next_token().kind {
                    TokenKind::Assign => {
                        self.advance();
                    }
                    _ => {
                        println!("expect Assign symbol but found {:?}", self.next_token());
                        break;
                    }
                },
                TokenKind::Equal => {
                    break;
                }
                TokenKind::Plus => {
                    break;
                }
                TokenKind::Assign => match self.next_token().kind {
                    TokenKind::Minus => {
                        self.advance();
                        initializer = self.parse_expression();
                    }
                    TokenKind::Integer(_t) => {
                        self.advance();
                        initializer = self.parse_expression();
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
        VariableDeclaration { name, initializer }
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
    fn parse_expression(&mut self) -> Expression {
        println!("parse expression");
        self.parse_add()
    }

    // + -
    fn parse_add(&mut self) -> Expression {
        let mut left = self.parse_mul();
        loop {
            match self.current_token().kind {
                TokenKind::Plus => {
                    self.advance();
                    let result = self.parse_mul();
                    left = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left),
                        operator: BinaryOperator(AddOperator(Plus)),
                        right: Box::new(result),
                    })
                }
                TokenKind::Minus => {
                    self.advance();
                    let result = self.parse_mul();
                    left = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left),
                        operator: BinaryOperator(AddOperator(Minus)),
                        right: Box::new(result),
                    })
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
    fn parse_mul(&mut self) -> Expression {
        let mut left = self.parse_unary();
        // let operator = BinaryOperator(MulOperator(Mul));
        // let right = self.parse_unary();
        loop {
            match self.current_token().kind {
                TokenKind::Mul => {
                    self.advance();
                    let result = self.parse_unary();
                    left = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left),
                        operator: BinaryOperator(MulOperator(Mul)),
                        right: Box::new(result),
                    })
                }
                TokenKind::Div => {
                    self.advance();
                    let result = self.parse_unary();
                    left = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(left),
                        operator: BinaryOperator(MulOperator(Div)),
                        right: Box::new(result),
                    })
                }
                _ => {
                    break;
                }
            }
        }
        left
    }
    // - or !
    fn parse_unary(&mut self) -> Expression {
        match self.current_token().kind {
            TokenKind::Minus => {
                self.advance();
                Expression::UnaryExpression(UnaryExpression {
                    prefix: Some(Operator::UnaryOperator(super::ast::UnaryOperator::Minus)),
                    value: self.parse_primary(),
                })
            }
            _ => {
                // 这里没有消耗任何Token，所以不advance
                Expression::UnaryExpression(UnaryExpression {
                    prefix: None,
                    value: self.parse_primary(),
                })
            }
        }
    }

    fn parse_primary(&mut self) -> PrimaryExpression {
        match self.current_token().kind {
            TokenKind::Integer(i) => {
                self.advance();
                PrimaryExpression::IntegerLiteral(i)
            }
            _ => PrimaryExpression::IntegerLiteral(999999), // Error占位，未来再补
        }
    }

    // fn unwrap_token(t: &Token) -> {}

    // }
    // fn expect(&mut self, token: &Token) -> bool {
    //     if self.next_token() == token {
    //         self.advance();
    //         true
    //     } else {
    //         println!("expect {:?} but found {:?}", token, self.next_token());
    //         false
    //     }
    // }
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
            position: 0
        };
        let result = lexer.get_token();
        println!("{:?}", result);
        let mut p = Parser {
            tokens: result,
            position: 0
        };
        let ast = p.parse_program();
        assert_eq!(ast.declarations.len(), 1);
    }
}