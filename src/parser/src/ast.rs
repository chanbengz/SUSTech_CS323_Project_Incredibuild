use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Integer(i32),
    Float(f32),
    UnaryOperation(Operator, Box<Expr>),
    BinaryOperation(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Integer(i) => write!(f, "{}", i),
            Expr::BinaryOperation(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::Float(fl) => write!(f, "{}", fl),
            Expr::UnaryOperation(op, expr) => write!(f, "({}{})", op, expr), // Minus and Not
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

