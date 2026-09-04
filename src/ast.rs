
// 在我们的Compiler中，Program这里是一个声明list

// let a = 1;

#[derive(Debug, PartialEq, Clone)]

pub struct Program {
    pub declarations: Vec<Declaration>
}
#[derive(Debug, PartialEq, Clone)]

pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration
}
#[derive(Debug, PartialEq, Clone)]

pub struct VariableDeclaration {
    pub name: Identifier,
    pub initializer: Expression
}
#[derive(Debug, PartialEq, Clone)]

pub enum Expression {
    Identifier(String),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    IntegerLiteral(i64)
}
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator:Operator,
    pub right: Box<Expression>
}

#[derive(Debug, PartialEq, Clone)]

pub struct UnaryExpression {
    pub prefix: Option<Operator>,
    pub value: PrimaryExpression
}

pub struct MulExpression {
    pub left: UnaryExpression,
    pub right: Option<UnaryExpression>,
    pub operator: Option<BinaryOperator>
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpression {
    IntegerLiteral(i64),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator)
}
#[derive(Debug,PartialEq, Clone )]
pub enum BinaryOperator {
    MulOperator(MulOperator),
    AddOperator(AddOperator),
}
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Minus, // -
    Not, // !
}

#[derive(Debug, PartialEq, Clone)]
pub enum AddOperator {
    Plus,
    Minus
}
#[derive(Debug, PartialEq, Clone)]
pub enum MulOperator {
    Mul,
    Div
}


#[derive(Debug, PartialEq, Clone)]
pub enum Identifier {
    StringLiteral(String)
}