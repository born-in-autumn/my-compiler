
// 在我们的Compiler中，Program这里是一个声明list

// let a = 10;


pub struct Program {
    declarations: Vec<Declaration>
}

pub enum Declaration {
    VariableDeclaration,
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
    BinaryExpression,
    IntegerLiteral(i64)
}


pub struct BinaryExpression {
    left: Expression,
    operator:Operator,
    right: Expression
}

pub enum Operator {
    Plus(String),
}
#[derive(Debug)]

pub enum Identifier {
    StringLiteral(String)
}