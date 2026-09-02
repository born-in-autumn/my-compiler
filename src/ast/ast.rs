
// 在我们的Compiler中，Program这里是一个声明list

// let a = 1;

#[derive(Debug)]

pub struct Program {
    pub declarations: Vec<Declaration>
}
#[derive(Debug)]

pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration
}
#[derive(Debug)]

pub struct VariableDeclaration {
    pub name: Identifier,
    pub initializer: Expression
}
#[derive(Debug)]

pub enum Expression {
    Identifier(String),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    IntegerLiteral(i64)
}
#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator:Operator,
    pub right: Box<Expression>
}

#[derive(Debug)]

pub struct UnaryExpression {
    pub prefix: Option<Operator>,
    pub value: PrimaryExpression
}

pub struct MulExpression {
    pub left: UnaryExpression,
    pub right: Option<UnaryExpression>,
    pub operator: Option<BinaryOperator>
}

#[derive(Debug)]
pub enum PrimaryExpression {
    IntegerLiteral(i64),
}
#[derive(Debug)]
pub enum Operator {
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator)
}
#[derive(Debug)]
pub enum BinaryOperator {
    MulOperator(MulOperator),
    AddOperator(AddOperator),
}
#[derive(Debug)]
pub enum UnaryOperator {
    Minus, // -
    Not, // !
}

#[derive(Debug)]
pub enum AddOperator {
    Plus,
    Minus
}
#[derive(Debug)]
pub enum MulOperator {
    Mul,
    Div
}


#[derive(Debug)]
pub enum Identifier {
    StringLiteral(String)
}