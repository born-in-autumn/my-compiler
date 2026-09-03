mod lexer;
mod parser;
mod ast;
use lexer::Lexer;
use parser::Parser;
fn main() {
    let input = "let a = 1*2+3";
    let mut lexer = Lexer {
        input,
        position: 0
    };
    let result = lexer.get_token();
    println!("{:?}", lexer);

    // expect: [ Keyword(Let), Space, Identifier("a"), Space, Assign, Space, Identifier("1"), Space, Plus, Space, Identifier("3"), Space, Plus, Space, Identifier("4"), Space, Assign, Space, Identifier("8"), Semicolon]
    println!("{:?}", result);

    let mut p = Parser {
        tokens: result,
        position: 0
    };
    let ast = p.parse_program();
    //Program { declarations: [VariableDeclaration(VariableDeclaration { name: StringLiteral("a"), initializer: BinaryExpression(BinaryExpression { left: UnaryExpression(UnaryExpression { prefix: Some(UnaryOperator(Minus)), value: IntegerLiteral(1) }), operator: BinaryOperator(AddOperator(Plus)), right: BinaryExpression(BinaryExpression { left: UnaryExpression(UnaryExpression { prefix: None, value: IntegerLiteral(2) }), operator: BinaryOperator(MulOperator(Mul)), right: UnaryExpression(UnaryExpression { prefix: None, value: IntegerLiteral(3) }) }) }) })] }
    println!("{:?}", ast);

}
