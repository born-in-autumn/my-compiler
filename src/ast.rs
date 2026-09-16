
// 在我们的Compiler中，Program这里是一个声明list

// let a = 1;

#[derive(Debug, PartialEq, Clone)]

pub struct Program {
    pub statements: Vec<Statement>,
}
#[derive(Debug, PartialEq, Clone)]


pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    PrintStatement(Expression)
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
    Minus 
}
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Minus, // -
    // Not, // !
}