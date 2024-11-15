use std::fmt;
use crate::tree::*;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Program::Statements(statements) => write!(f, "Statements: [{}]", 
                statements.iter().map(|stmt| format!("{}", stmt)).collect::<Vec<String>>().join(", ")),
            Program::Functions(functions) => write!(f, "Functions: [{}]", 
                functions.iter().map(|func| format!("{}", func)).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Include(s) => write!(f, "Include: {}", s),
            Statement::GlobalVariable(v) => write!(f, "Global Variable: {}", v),
            Statement::Struct(vars) => write!(f, "Struct: [{}]", 
                vars.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::VarDeclaration(ident, values, dims) => write!(f, "Variable Declaration: {} = [{}] with dimensions [{}]", 
                ident, 
                values.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(", "), 
                dims.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::FuncDeclaration(ident, _params, num) => write!(f, "Function: {} with {} params", ident, num),
        }
    }
}

impl fmt::Display for CompExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompExpr::Value(val) => write!(f, "{}", val),
            CompExpr::Variable(var) => write!(f, "{}", var),
            CompExpr::UnaryOperation(op, expr) => write!(f, "({} {})", op, expr),
            CompExpr::BinaryOperation(left, op, right) => write!(f, "({} {} {})", left, op, right),
            CompExpr::Error => write!(f, "Error: Missing Term"),
        }
    }
}

impl fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignExpr::AssignOperation(var, expr) => write!(f, "Assignment: {} = {}", var, expr),
        }
    }
}

impl fmt::Display for CondExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CondExpr::Bool(b) => write!(f, "Condition: {}", b),
            CondExpr::Condition(left, op, right) => write!(f, "Condition: {} {} {}", left, op, right),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}: i32", i),
            Value::Float(fl) => write!(f, "{}: f32", fl),
            Value::String(s) => write!(f, "{}: String", s),
            Value::Char(c) => write!(f, "{}: char", c),
            Value::Bool(b) => write!(f, "{}: bool", b),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Pow => write!(f, "^"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
        }
    }
}


impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::Inc => write!(f, "++"),
            UnaryOperator::Dec => write!(f, "--"),
        }
    }
}

impl fmt::Display for JudgeOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JudgeOperator::GT => write!(f, ">"),
            JudgeOperator::GE => write!(f, ">="),
            JudgeOperator::LT => write!(f, "<"),
            JudgeOperator::LE => write!(f, "<="),
            JudgeOperator::EQ => write!(f, "=="),
            JudgeOperator::NE => write!(f, "!="),
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            If::IfExpr(cond, body, opt_body) => 
                write!(f, "If: {} then {} else {}", cond, body, 
                    opt_body.clone().unwrap_or_else(|| Body::Body(Box::new(vec![])))),
        }
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Loop::LoopExpr(cond, body) => write!(f, "Loop: {} do {}", cond, body),
        }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Body(expressions) => {
                write!(f, "Body: [{}]", 
                    expressions.iter().map(|expr| format!("{}", expr)).collect::<Vec<String>>().join(", "))
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::If(if_expr) => write!(f, "If expression: {}", if_expr),
            Expr::Loop(loop_expr) => write!(f, "Loop expression: {}", loop_expr),
            Expr::Assign(assign_expr) => write!(f, "Assignment: {}", assign_expr),
            Expr::Break() => write!(f, "Break"),
            Expr::Continue => write!(f, "Continue"),
            Expr::Return(opt_val) => write!(f, "Return: {}", opt_val.as_ref().map_or("None".to_string(), |v| format!("{}", v))),
        }
    }
}
