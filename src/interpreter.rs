use crate::ast::{
    BinaryExpression, BinaryOperator, Expression, PrimaryExpression, Statement, UnaryExpression,
    UnaryOperator::Minus, VariableDeclaration,
};
use crate::error::{RunTimeError, UndefinedBehavior};
use std::collections::HashMap;
pub struct Interpreter {
    pub env: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    None,
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
impl Interpreter {
    pub fn execute_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::PrintStatement(e) => {
                // TODO：需要取hashmap，晚点实现
                let res = self.eval_expr(&e);
                
                println!("{:?}", res.unwrap());
            }
            Statement::VariableDeclaration(d) => {
                self.execute_var_declaration(d);
            }
        }
    }

    fn execute_var_declaration(&mut self, d: &VariableDeclaration) {
        // 标识符是hashmap的Key，先判断initializer是否有值，如果没值，直接把None存到hashmap里
        match &d.initializer {
            Some(e) => {
                // 如果有值，则正常把标识符对应的值存进去
                self.env.insert(d.name.clone(), self.eval_expr(&e).unwrap());
            }
            None => {
                self.env.insert(d.name.clone(), Value::None);
            }
        }
    }

    // let input = "let a = 4 * (-1 + 2 * 3);print a;";
    fn eval_expr(&self, expr: &Expression) -> Result<Value, RunTimeError> {
        match expr {
            Expression::BinaryExpression(e) => Ok(self.eval_binary_expr(e))?,
            Expression::PrimaryExpression(e) => Ok(self.eval_primary_expr(e))?,
            Expression::UnaryExpression(e) => Ok(self.eval_unary_expr(e))?,
        }
    }

    fn eval_binary_expr(&self, expr: &BinaryExpression) -> Result<Value, RunTimeError> {
        let left_value = self.eval_expr(&expr.left);
        let right_value = self.eval_expr(&expr.right);
        let mut left = None;
        let mut right = None;
        println!("{:?} {:?}", left,right);
        match left_value {
            Ok(Value::Integer(i)) => {
                left = Some(i);
            }
            Ok(Value::None) => {

                return Err(RunTimeError::UndefinedBehavior(UndefinedBehavior {
                    message: format!("Undefined behavior1"),
                }));
            }
            Err(e) => {

                return Err(e);
            }
        }
        match right_value {
            Ok(Value::Integer(i)) => {

                right = Some(i);
            }
            Ok(Value::None) => {

                return Err(RunTimeError::UndefinedBehavior(UndefinedBehavior {
                    message: format!("Undefined behavior2"),
                }));
            }
            Err(e) => {

                return Err(e);
            }
        }
        match expr.operator {
            BinaryOperator::Mul => {
                return Ok(Value::Integer(left.unwrap() * right.unwrap()));
            }
            BinaryOperator::Div => {
                return Ok(Value::Integer(left.unwrap() / right.unwrap()));
            }
            BinaryOperator::Plus => {
                return Ok(Value::Integer(left.unwrap() + right.unwrap()));
            }
            BinaryOperator::Minus => {
                return Ok(Value::Integer(left.unwrap() - right.unwrap()));
            }
        }
    }

    fn eval_unary_expr(&self, expr: &UnaryExpression) -> Result<Value, RunTimeError> {

        match &expr.prefix {
            Some(op) => match op {
                Minus => match self.eval_primary_expr(&expr.value) {
                    Ok(Value::Integer(i)) => Ok(Value::Integer(-i)),
                    Ok(Value::None) => {
                        return Err(RunTimeError::UndefinedBehavior(UndefinedBehavior {
                            message: format!("Undefined behavior3"),
                        }));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                },
            },
            _ => Ok(self.eval_primary_expr(&expr.value))?,
        }
    }

    //直接返回一个expr结果
    fn eval_primary_expr(&self, expr: &PrimaryExpression) -> Result<Value, RunTimeError> {
        match expr {
            PrimaryExpression::IntegerLiteral(number) => {
                return Ok(Value::Integer(*number));
            }
            PrimaryExpression::Expression(e) => {
                return Ok(self.eval_expr(&Box::new(e))?);
            }
            PrimaryExpression::Identifier(i) => {
                // 去找hashmap
                match self.env.get(i) {
                    Some(v) => {
                        return Ok(v.clone());
                    }
                    None => {
                        return Err(RunTimeError::UndefinedBehavior(UndefinedBehavior {
                            message: format!("Undefined Identifier i"),
                        }));
                    }
                }
            }
        }
    }
}
