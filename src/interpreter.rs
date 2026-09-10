use std::collections::HashMap;

use crate::ast::{
    BinaryExpression, BinaryOperator, Expression, PrimaryExpression, Statement, UnaryExpression, UnaryOperator::Minus, VariableDeclaration,
};
pub struct Interpreter {
    env: HashMap<String, Value>,
}

#[derive(Debug)]
enum Value {
    Integer(i64),
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
    fn execute_stmt(&self, stmt: &Statement) {
        match stmt {
            Statement::PrintStatement(e) => {
                // TODO：需要取hashmap，晚点实现
                println!("{:?}", self.eval_expr(&e));
            }
            Statement::VariableDeclaration(d) => {
                self.execute_var_declaration(d);
            }
        }
    }

    fn execute_var_declaration(&self, d: &VariableDeclaration) {
        // 标识符是hashmap的Key，先判断initializer是否有值，如果没值，直接把None存到hashmap里
        // 如果有值，则正常把标识符对应的值存进去
    }

    fn eval_expr(&self, expr: &Expression) -> Value {
        match expr {
            Expression::BinaryExpression(e) => self.eval_binary_expr(e),
            Expression::PrimaryExpression(e) => self.eval_primary_expr(e),
            Expression::UnaryExpression(e) => self.eval_unary_expr(e),
        }
    }

    fn eval_binary_expr(&self, expr: &BinaryExpression) -> Value {
        let left_value = self.eval_expr(&expr.left);
        let right_value = self.eval_expr(&expr.right);
        let mut left = None;
        let mut right = None;

        match left_value {
            Value::Integer(i) => {
                left = Some(i);
            }
        }
        match right_value {
            Value::Integer(i) => {
                right = Some(i);
            }
        }
        match expr.operator {
            BinaryOperator::Mul => {
                return Value::Integer(left.unwrap() * right.unwrap());
            }
            BinaryOperator::Div => {
                return Value::Integer(left.unwrap() / right.unwrap());
            }
            BinaryOperator::Plus => {
                return Value::Integer(left.unwrap() + right.unwrap());
            }
            BinaryOperator::Minus => {
                return Value::Integer(left.unwrap() - right.unwrap());
            }
        }
    }

    fn eval_unary_expr(&self, expr: &UnaryExpression) -> Value {
        match &expr.prefix {
            Some(op) => match op {
                Minus => match self.eval_primary_expr(&expr.value) {
                    Value::Integer(i) => Value::Integer(-i),
                },
            },
            _ => self.eval_primary_expr(&expr.value),
        }
    }

    //直接返回一个expr结果
    fn eval_primary_expr(&self, expr: &PrimaryExpression) -> Value {
        match expr {
            PrimaryExpression::IntegerLiteral(number) => {
                return Value::Integer(*number);
            }
            _ => {
                return Value::Integer(9999);
            } // PrimaryExpression::Identifier(i) => {

              // }
              // PrimaryExpression::Expression(e) => {

              // }
        }
    }
}
