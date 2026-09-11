mod ast;
mod error;
mod lexer;
mod parser;
mod interpreter;
use std::collections::HashMap;

use lexer::Lexer;
use parser::Parser;
use crate::interpreter::Interpreter;
fn main() {
    let input = "let a = 4 * (-1 + 2 * 3);print a;";
    let mut lexer = Lexer { input, position: 0 };
    println!("lexer: {:?}", lexer);

    let result = lexer.tokenize();

    // expect: [ Keyword(Let), Space, Identifier("a"), Space, Assign, Space, Identifier("1"), Space, Plus, Space, Identifier("3"), Space, Plus, Space, Identifier("4"), Space, Assign, Space, Identifier("8"), Semicolon]
    println!("{:?}", result);

    match result {
        Ok(res) => {
            let mut p = Parser {
                tokens: res,
                position: 0,
            };
            let program = p.parse_program();
            println!("{:?}", program);
            let mut interpreter = Interpreter {
                env: HashMap::new()
            };
            match program {
                Ok(ast) => {
                    for i in ast.statements {
                        interpreter.execute_stmt(&i);
                    }
                }
                Err(_e) => {}
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    };

}
