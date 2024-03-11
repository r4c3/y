use crate::ast::Expr;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::value::Value;
use core::cell::RefCell;
use std::rc::Rc;

impl Expr {
    pub fn evaluate(&self, env: &Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(env)?;
                let right = right.evaluate(env)?;
                match (&left, &right) {
                    (Value::Number(left), Value::Number(right)) => match operator.lexeme().as_str()
                    {
                        "+" => Ok(Value::Number(left + right)),
                        "-" => Ok(Value::Number(left - right)),
                        "*" => Ok(Value::Number(left * right)),
                        "/" => Ok(Value::Number(left / right)),
                        _ => Err(RuntimeError::new(
                            format!("Invalid binary operator '{}'", operator.lexeme()),
                            operator.line,
                        )),
                    },
                    _ => Err(RuntimeError::new(
                        format!("Operands must be numbers, got '{}' and '{}'", left, right),
                        operator.line,
                    )),
                }
            }
            Expr::Unary { operator, right } => {
                let right = right.evaluate(env)?;
                match right {
                    Value::Number(right) => match operator.lexeme().as_str() {
                        "-" => Ok(Value::Number(-right)),
                        _ => Err(RuntimeError::new(
                            format!("Invalid unary operator '{}'", operator.lexeme()),
                            operator.line,
                        )),
                    },
                    _ => Err(RuntimeError::new(
                        format!("Operand must be a number, got '{}'", right),
                        operator.line,
                    )),
                }
            }
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping(expression) => expression.evaluate(env),
        }
    }
}
