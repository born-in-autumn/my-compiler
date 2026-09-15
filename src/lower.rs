use crate::ast::{
    BinaryExpression, BinaryOperator, Expression, PrimaryExpression, Statement, UnaryExpression,
    UnaryOperator::Minus, VariableDeclaration,
};

/**
 *
 * 需要定义一种中间表示，把AST拍平，未来方便编译器后端直接转成x86-64汇编，IR不涉及任何寄存器相关的操作
 * 我们会采用TAC（三地址码）的核心思想来完成
 * 目前暂定如下：
 * Const dst， src  把整数常量src的值加载到dst
 * Neg dst，src 对src取负，结果存dst
 * Binary dst，op，lhs，rhs 执行二元运算，op是add/sub/mul/div
 * Load dst，var  把变量var读取值到dst
 * Store var，src 把src的值存入变量var
 * Print src 输出src的值
 * Temp：一个临时的值，遵循静态单赋值（SSA）原则，只需要存储编号，实际的值在执行的时候算出来
 */

// IR Module
pub enum IrInst {
    Const {
        dst: Temp,
        src: i64,
    },
    Neg {
        dst: Temp,
        src: IrValue,
    },
    Binary {
        dst: Temp,
        op: IrOperator,
        lsh: IrValue,
        rhs: IrValue,
    },
    Load {
        dst: Temp,
        var: String,
    },
    Store {
        var: String,
        src: IrValue,
    },
    Print {
        src: IrValue,
    },
}
pub struct Temp {
    idx: i64,
}
pub enum IrValue {
    Const(i64),
    Temp(Temp),
}
pub enum IrOperator {
    Plus,
    Sub,
    Mul,
    Div,
}

pub struct IrGen {
    instructments: Vec<IrInst>,
    idx: i64,
}

/**
 * let input = "let a = 4 * (-1 + 2 * 3);print a;"
 * AST example：
 * Ok(Program { statements: [VariableDeclaration(VariableDeclaration
 * { name: "a", initializer:
 *  Some(BinaryExpression(BinaryExpression
 * { left: PrimaryExpression(IntegerLiteral(4)), operator: Mul, right:
 * PrimaryExpression(Expression(BinaryExpression(BinaryExpression
 *  { left: UnaryExpression(UnaryExpression { prefix: Some(UnaryOperator(Minus)), value: IntegerLiteral(1) }), operator: Plus,
 * right: BinaryExpression(BinaryExpression {
 * left: PrimaryExpression(IntegerLiteral(2)), operator: Mul,
 * right: PrimaryExpression(IntegerLiteral(3)) }) }))) })) }),
 * PrintStatement(PrimaryExpression(Identifier("a")))] })
 *
 */

impl IrInst {
    pub fn lower_stmt(&mut self, stmt: &Statement, ir_box: &mut IrGen) {
        match stmt {
            Statement::PrintStatement(e) => {
                let res = self.lower_expr(&e, ir_box);
                ir_box.instructments.push(IrInst::Print { src: res });
            }
            Statement::VariableDeclaration(d) => {
                self.lower_var_declaration(d, ir_box);
            }
        }
    }
    // let a = 1; let b;
    fn lower_var_declaration(&mut self, d: &VariableDeclaration, ir_box: &mut IrGen) -> IrValue {
        match &d.initializer {
            Some(e) => {
                let idx = ir_box.get_idx();
                let r = self.lower_expr(&e, ir_box);
                ir_box.instructments.push(IrInst::Store {
                    var: d.name.clone(),
                    src: r,
                });
                ir_box.advance_idx();
                IrValue::Temp(Temp { idx: idx - 1 })
            }
            None => {
                let idx = ir_box.get_idx();
                // 没值的话，先存个默认值兜底
                ir_box.instructments.push(IrInst::Store {
                    var: d.name.clone(),
                    src: IrValue::Const(0),
                });
                ir_box.advance_idx();
                IrValue::Temp(Temp { idx: idx - 1 })
            }
        }
    }

    // let input = "let a = 4 * (-1 + 2 * 3);print a;";
    fn lower_expr(&self, expr: &Expression, ir_box: &mut IrGen) -> IrValue {
        match expr {
            Expression::BinaryExpression(e) => self.lower_binary_expr(e, ir_box),
            Expression::PrimaryExpression(e) => self.lower_primary_expr(e, ir_box),
            Expression::UnaryExpression(e) => self.lower_unary_expr(e, ir_box),
        }
    }

    fn lower_binary_expr(&self, expr: &BinaryExpression, ir_box: &mut IrGen) -> IrValue {
        let left_value = self.lower_expr(&expr.left, ir_box);
        let right_value = self.lower_expr(&expr.right, ir_box);
        match (&left_value, &right_value) {
            (IrValue::Const(l), IrValue::Const(r)) => match expr.operator {
                BinaryOperator::Mul => {
                    return IrValue::Const(l * r);
                }
                BinaryOperator::Div => {
                    return IrValue::Const(l / r);
                }
                BinaryOperator::Plus => {
                    return IrValue::Const(l + r);
                }
                BinaryOperator::Minus => {
                    return IrValue::Const(l - r);
                }
            },
            _ => {}
        }
        match expr.operator {
            BinaryOperator::Mul => {
                let idx = ir_box.get_idx();
                ir_box.instructments.push(IrInst::Binary {
                    dst: Temp { idx },
                    op: IrOperator::Mul,
                    lsh: left_value,
                    rhs: right_value,
                });
                ir_box.advance_idx();
                return IrValue::Temp(Temp { idx: idx - 1 });
            }
            BinaryOperator::Div => {
                let idx = ir_box.get_idx();
                ir_box.instructments.push(IrInst::Binary {
                    dst: Temp { idx },
                    op: IrOperator::Mul,
                    lsh: left_value,
                    rhs: right_value,
                });
                ir_box.advance_idx();
                return IrValue::Temp(Temp { idx: idx - 1 });
            }
            BinaryOperator::Plus => {
                let idx = ir_box.get_idx();
                ir_box.instructments.push(IrInst::Binary {
                    dst: Temp { idx },
                    op: IrOperator::Mul,
                    lsh: left_value,
                    rhs: right_value,
                });
                ir_box.advance_idx();
                return IrValue::Temp(Temp { idx: idx - 1 });
            }
            BinaryOperator::Minus => {
                let idx = ir_box.get_idx();
                ir_box.instructments.push(IrInst::Binary {
                    dst: Temp { idx },
                    op: IrOperator::Mul,
                    lsh: left_value,
                    rhs: right_value,
                });
                ir_box.advance_idx();
                return IrValue::Temp(Temp { idx: idx - 1 });
            }
        }
    }

    fn lower_unary_expr(&self, expr: &UnaryExpression, ir_box: &mut IrGen) -> IrValue {
        match &expr.prefix {
            Some(op) => match op {
                Minus => match self.lower_primary_expr(&expr.value, ir_box) {
                    IrValue::Const(i) => IrValue::Const(-i),
                    IrValue::Temp(t) => {
                        let idx = ir_box.get_idx();
                        ir_box.instructments.push(IrInst::Neg {
                            dst: Temp { idx },
                            src: IrValue::Temp(t),
                        });
                        ir_box.advance_idx();
                        IrValue::Temp(Temp { idx: idx - 1 })
                    }
                },
            },
            _ => self.lower_primary_expr(&expr.value, ir_box),
        }
    }

    //直接返回一个expr结果
    fn lower_primary_expr(&self, expr: &PrimaryExpression, ir_box: &mut IrGen) -> IrValue {
        match expr {
            PrimaryExpression::IntegerLiteral(number) => {
                return IrValue::Const(*number);
            }
            PrimaryExpression::Expression(e) => {
                return self.lower_expr(&Box::new(e), ir_box);
            }
            PrimaryExpression::Identifier(i) => {
                let idx = ir_box.get_idx();
                ir_box.instructments.push(IrInst::Load {
                    dst: Temp { idx },
                    var: i.to_string(),
                });
                ir_box.advance_idx();
                IrValue::Temp(Temp { idx: idx - 1 })
            }
        }
    }
}

impl IrGen {
    fn advance_idx(&mut self) {
        self.idx += 1;
    }
    fn get_idx(&self) -> i64 {
        return self.idx;
    }
}
