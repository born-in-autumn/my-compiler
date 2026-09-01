
mod ast;
use ast::Lexer;
use ast::Parser;

fn main() {
    let input = "let a = 1;";
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
    println!("{:?}", ast);

}
