use crate::token::Token;

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

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
            } => parenthesize(operator.lexeme(), &[left, right]),
            Expr::Unary { operator, right } => parenthesize(operator.lexeme(), &[right]),
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
