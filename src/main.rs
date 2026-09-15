mod ast;
mod error;
mod interpreter;
mod lexer;
mod lower;
mod parser;
use std::collections::HashMap;

use crate::interpreter::Interpreter;
use lexer::Lexer;
use lower::*;
use parser::Parser;
fn main() {
    let input = "let a = 4 * (-1 + 2 * 3);print a;";
    let mut lexer = Lexer { input, position: 0 };
    // println!("lexer: {:?}", lexer);

    let result = lexer.tokenize();

    // expect: [ Keyword(Let), Space, Identifier("a"), Space, Assign, Space, Identifier("1"), Space, Plus, Space, Identifier("3"), Space, Plus, Space, Identifier("4"), Space, Assign, Space, Identifier("8"), Semicolon]
    // println!("{:?}", result);

    match result {
        Ok(res) => {
            let mut p = Parser {
                tokens: res,
                position: 0,
            };
            let program = p.parse_program();
            println!("{:?}", program);
            let mut interpreter = Interpreter {
                env: HashMap::new(),
            };

            let mut ir_gen = IrGen {
                instructments: vec![],
                idx: 0,
            };
            match program {
                Ok(ast) => {
                    for i in ast.statements {
                        interpreter.execute_stmt(&i);
                        ir_gen.lower_stmt(&i);
                    }
                }
                Err(_e) => {}
            }
            println!("The IR is: {:?}", ir_gen.instructments);
            print_ir(ir_gen.instructments);
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    };
}


fn print_ir(instuctments: Vec<IrInst>) {
    for item in instuctments {
        match item {
            IrInst::Const { dst, src } => {
                println!("Const {:?} {:?}", dst.idx, src);
            }
            IrInst::Neg { dst, src } => {
                println!("Neg {:?} {:?}", dst.idx, src);
            }

            IrInst::Binary { dst, op, lsh, rhs } => {
                println!("Binary {:?} {:?} {:?} {:?}", dst.idx, op, lsh, rhs);
            }
            IrInst::Load { dst, var } => {
                println!("Load t{:?} {:?}", dst.idx, var);
            }

            IrInst::Store { var, src } => {
                println!("Store {:?} {:?}", var, src);
            }
            IrInst::Print { src } => match src {
                IrValue::Const(i) => {
                    println!("Print {:?}", i);
                }
                IrValue::Temp(t) => {
                    println!("Print t{:?}", t.idx);
                }
            },
        }
    }
}
