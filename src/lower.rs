use crate::ast::{
    BinaryExpression, BinaryOperator, Expression, IfStatement, PrimaryExpression, Statement, UnaryExpression, UnaryOperator::Minus, VariableDeclaration,
};
use BinaryOperator::*;

/**
 *
 * 需要定义一种中间表示，把AST拍平，未来方便编译器后端直接转成ARM64汇编，IR不涉及任何寄存器相关的操作
 * 我们会采用TAC（三地址码）的核心思想来完成
 * 目前暂定如下：
 * Const dst， src  把整数常量src的值加载到dst
 * Neg dst，src 对src取负，结果存dst
 * Binary dst，op，lhs，rhs 执行二元运算，op是add/sub/mul/div
 * Load dst，var  把变量var读取值到dst
 * Store var，src 把src的值存入变量var
 * Print src 输出src的值
 * Label dst 给接下来的代码命名
 * Temp：一个临时的值，遵循静态单赋值（SSA）原则，只需要存储编号，实际的值在执行的时候算出来
 * 
 * 
 */

// IR Module
#[derive(Debug)]
pub enum IrInst {
    // Const {
    //     dst: Temp,
    //     src: i64,
    // },
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
#[derive(Debug)]

pub struct Temp {
    pub idx: i64,
}
#[derive(Debug)]

pub enum IrValue {
    Const(i64),
    Temp(Temp),
    True,
    False
}
#[derive(Debug)]

pub enum IrOperator {
    ArithOp(ArithOp),
    CmpOp(CmpOp)
}
#[derive(Debug)]

pub enum ArithOp {
    Plus,
    Sub,
    Mul,
    Div,
}
#[derive(Debug)]

pub enum CmpOp {
    Lt, // <
    Le, // <=
    Gt, //  >
    Ge, // >=
    Eq, // ==
    Ne // !=
}
#[derive(Debug)]

pub struct IrGen {
    pub instructments: Vec<IrInst>,
    pub idx: i64,
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

impl IrInst {}

impl IrGen {
    fn advance_idx(&mut self) {
        self.idx += 1;
    }
    fn get_idx(&self) -> i64 {
        return self.idx;
    }

    fn new_temp(&mut self) -> IrValue {
        self.advance_idx();
        return IrValue::Temp(Temp { idx: self.idx - 1 });
    }

    pub fn lower_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::PrintStatement(e) => {
                let res: IrValue = self.lower_expr(&e, String::from(""));
                self.instructments.push(IrInst::Print { src: res });
            }
            Statement::VariableDeclaration(d) => {
                self.lower_var_declaration(d);
            }
            Statement::IfStatement(s) => {
                self.lower_if_statement(s);
            }
        }
    }
    /**
     * 
     * 分析下现状
     * if三步走：设置label，跳转，把这些翻译成ir，然后让后续流程去处理
     * 
     * 
     */
    fn lower_if_statement(&mut self, stmt: &IfStatement) {
        self.lower_expr(&stmt.cond, String::from("label"));
    }

    // let a = 1; let b;
    fn lower_var_declaration(&mut self, d: &VariableDeclaration) {
        match &d.initializer {
            Some(e) => {
                let r = self.lower_expr(&e, String::from(""));
                self.instructments.push(IrInst::Store {
                    var: d.name.clone(),
                    src: r,
                });
            }
            None => {
                // 没值的话，先存个默认值兜底
                self.instructments.push(IrInst::Store {
                    var: d.name.clone(),
                    src: IrValue::Const(0),
                });
            }
        }
    }

    // let input = "let a = 4 * (-1 + 2 * 3);print a;";
    fn lower_expr(&mut self, expr: &Expression, label: String) -> IrValue {
        match expr {
            Expression::BinaryExpression(e) => if label == "" {
                return self.lower_binary_expr(e);
            }else {
                return self.lower_binary_expr_in_if_stmt(e);
            }
            Expression::PrimaryExpression(e) => self.lower_primary_expr(e),
            Expression::UnaryExpression(e) => self.lower_unary_expr(e),
        }
    }
    
    fn lower_binary_expr(&mut self, expr: &BinaryExpression) -> IrValue {
        let left_value = self.lower_expr(&expr.left, String::from(""));
        let right_value = self.lower_expr(&expr.right, String::from(""));
        if let (IrValue::Const(l), IrValue::Const(r)) = (&left_value, &right_value) {
            return match expr.operator {
                Mul => IrValue::Const(l * r),
                Div => IrValue::Const(l / r),
                Plus => IrValue::Const(l + r),
                Greater => if l > r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                GreaterEqual => if l >= r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                Less => if l < r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                LessEqual => if l <= r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                NotEqual => if l != r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                Equal => if l == r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                BinaryOperator::Minus => IrValue::Const(l - r),
            };
        }
        let op: IrOperator = match expr.operator {
            Mul => IrOperator::ArithOp(ArithOp::Mul),
            Div => IrOperator::ArithOp(ArithOp::Div),
            Plus => IrOperator::ArithOp(ArithOp::Plus),
            Greater => IrOperator::CmpOp(CmpOp::Gt),
            GreaterEqual => IrOperator::CmpOp(CmpOp::Ge),
            Less => IrOperator::CmpOp(CmpOp::Lt),
            LessEqual => IrOperator::CmpOp(CmpOp::Le),
            NotEqual => IrOperator::CmpOp(CmpOp::Ne),
            Equal => IrOperator::CmpOp(CmpOp::Eq),
            BinaryOperator::Minus => IrOperator::ArithOp(ArithOp::Sub),
        };
        let idx = self.get_idx();
        self.instructments.push(IrInst::Binary {
            dst: Temp { idx },
            op,
            lsh: left_value,
            rhs: right_value,
        });
        self.new_temp()
    }



    // 目前是专门给if条件用的函数，未来可能会改名
    fn lower_binary_expr_in_if_stmt(&mut self, expr: &BinaryExpression) -> IrValue {
        let left_value = self.lower_expr(&expr.left, String::from(""));
        let right_value = self.lower_expr(&expr.right, String::from(""));
        if let (IrValue::Const(l), IrValue::Const(r)) = (&left_value, &right_value) {
            return match expr.operator {
                Mul => IrValue::Const(l * r),
                Div => IrValue::Const(l / r),
                Plus => IrValue::Const(l + r),
                BinaryOperator::Minus => IrValue::Const(l - r),
                Greater => if l > r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                GreaterEqual => if l >= r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                Less => if l < r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                LessEqual => if l <= r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                NotEqual => if l != r {
                    IrValue::True
                } else {
                    IrValue::False
                }
                Equal => if l == r {
                    IrValue::True
                } else {
                    IrValue::False
                }
            };
        }
        let op: IrOperator = match expr.operator {
            Mul => IrOperator::ArithOp(ArithOp::Mul),
            Div => IrOperator::ArithOp(ArithOp::Div),
            Plus => IrOperator::ArithOp(ArithOp::Plus),
            Greater => IrOperator::CmpOp(CmpOp::Gt),
            GreaterEqual => IrOperator::CmpOp(CmpOp::Ge),
            Less => IrOperator::CmpOp(CmpOp::Lt),
            LessEqual => IrOperator::CmpOp(CmpOp::Le),
            NotEqual => IrOperator::CmpOp(CmpOp::Ne),
            Equal => IrOperator::CmpOp(CmpOp::Eq),
            BinaryOperator::Minus => IrOperator::ArithOp(ArithOp::Sub),
        };
        let idx = self.get_idx();
        self.instructments.push(IrInst::Binary {
            dst: Temp { idx },
            op,
            lsh: left_value,
            rhs: right_value,
        });
        self.new_temp()
    }


    fn lower_unary_expr(&mut self, expr: &UnaryExpression) -> IrValue {
        match &expr.prefix {
            Some(op) => match op {
                Minus => match self.lower_primary_expr(&expr.value) {
                    IrValue::Const(i) => IrValue::Const(-i),
                    IrValue::Temp(t) => {
                        let idx = self.get_idx();
                        self.instructments.push(IrInst::Neg {
                            dst: Temp { idx },
                            src: IrValue::Temp(t),
                        });
                        self.new_temp()
                    }
                    // 目前只有true和false，直接报错即可
                    _ => unreachable!("IR Error, can not neg to bool type"),
                },
            },
            _ => self.lower_primary_expr(&expr.value),
        }
    }

    //直接返回一个expr结果
    fn lower_primary_expr(&mut self, expr: &PrimaryExpression) -> IrValue {
        match expr {
            PrimaryExpression::IntegerLiteral(number) => {
                return IrValue::Const(*number);
            }
            PrimaryExpression::Expression(e) => {
                return self.lower_expr(&Box::new(e), String::from(""));
            }
            PrimaryExpression::Identifier(i) => {
                let idx = self.get_idx();
                self.instructments.push(IrInst::Load {
                    dst: Temp { idx },
                    var: i.to_string(),
                });
                self.new_temp()
            }
        }
    }
}
