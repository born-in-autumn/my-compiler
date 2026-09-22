
// 在我们的Compiler中，Program这里是一个声明list

// let a = 1;

use crate::lexer::TokenKind;

#[derive(Debug, PartialEq, Clone)]

pub struct Program {
    pub statements: Vec<Statement>,
}
#[derive(Debug, PartialEq, Clone)]


pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    PrintStatement(Expression),
    IfStatement(IfStatement)
}


#[derive(Debug,PartialEq, Clone )]

pub struct IfStatement {
    pub cond: Expression,
    pub if_body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>
}

// pub enum Declaration {
//     VariableDeclaration(VariableDeclaration) // 目前只做变量声明
// }
#[derive(Debug, PartialEq, Clone)]

pub struct VariableDeclaration {
    pub name: String,
    pub initializer: Option<Expression>
}
#[derive(Debug, PartialEq, Clone)]

pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    PrimaryExpression(PrimaryExpression),
}
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator:BinaryOperator,
    pub right: Box<Expression>
}

#[derive(Debug, PartialEq, Clone)]

pub struct UnaryExpression {
    pub prefix: Option<UnaryOperator>,
    pub value: PrimaryExpression
}


#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpression {
    IntegerLiteral(i64),
    Identifier(String),
    Expression(Box<Expression>)
}
// #[derive(Debug, PartialEq, Clone)]
// pub enum Operator {
//     BinaryOperator(BinaryOperator),
//     UnaryOperator(UnaryOperator)
// }
#[derive(Debug,PartialEq, Clone )]
pub enum BinaryOperator {
    Mul,
    Div,
    Plus,
    Minus,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    NotEqual
}

impl From<TokenKind> for BinaryOperator {
    fn from(tk: TokenKind) -> Self {
        match tk {
            TokenKind::Equal => BinaryOperator::Equal,
            TokenKind::Greater => BinaryOperator::Greater,
            TokenKind::Less => BinaryOperator::Less,
            TokenKind::LessEqual => BinaryOperator::LessEqual,
            TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
            TokenKind::NotEqual => BinaryOperator::NotEqual,
            TokenKind::Plus => BinaryOperator::Plus,
            TokenKind::Minus => BinaryOperator::Minus,
            TokenKind::Mul => BinaryOperator::Mul,
            TokenKind::Div => BinaryOperator::Div,
            _ => unreachable!("token {:?} 不是二元运算符", tk),
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Minus, // -
    // Not, // !
}

pub enum Type {

}