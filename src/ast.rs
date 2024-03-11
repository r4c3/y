use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::token::Token;
use crate::value::Value;
use core::cell::RefCell;
use std::rc::Rc;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => parenthesize(&operator.lexeme, &[left, right]),
            Expr::Unary { operator, right } => parenthesize(&operator.lexeme, &[right]),
            Expr::Literal { value } => match value {
                Value::Number(n) => n.to_string(),
                Value::String(s) => s.clone(),
                Value::Boolean(b) => b.to_string(),
                Value::Nil => "nil".to_string(),
            },
            Expr::Grouping(expression) => {
                format!("(group {})", expression.print())
            }
        }
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut s = format!("({}", name);
    for expr in exprs {
        s += &format!(" {}", expr.print());
    }
    s + ")"
}

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(String, Option<Expr>),
    Block(Vec<Stmt>),
}

impl Stmt {
    pub fn execute(&self, env: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        match self {
            Stmt::Expression(expr) => {
                expr.evaluate(&env)?;
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate(&env)?;
                println!("{}", value);
            }
            Stmt::Var(name, maybe_expr) => {
                let value = if let Some(expr) = maybe_expr {
                    expr.evaluate(&env)?
                } else {
                    Value::Nil
                };
                env.borrow_mut().define(name.to_string(), value);
            }
            Stmt::Block(statements) => {
                let new_env = Environment::with_enclosing(Rc::clone(&env));
                for statement in statements {
                    statement.execute(new_env.clone())?;
                }
            }
        }
        Ok(())
    }
}
