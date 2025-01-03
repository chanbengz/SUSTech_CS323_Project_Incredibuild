use std::fmt;
use crate::tree::*;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Program::Program(parts) => write!(f, "{}", parts.iter().map(|part| format!("{}", part)).collect::<Vec<String>>().join(", ")),
            Program::Error => write!(f, "[ProgramError]"),
        }
    }
}

impl fmt::Display for ProgramPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramPart::Statement(statement) => write!(f, "Statement: {}", statement),
            ProgramPart::Function(function) => write!(f, "Functions: {}", function),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Include(s, _) => write!(f, "Include: {}", s),
            Statement::GlobalVariable(vars, _) => write!(f, "GlobalVariable: [{}]", 
                vars.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", ")),
            Statement::Struct(structure, _) => write!(f, "Struct: {}", structure),
            Statement::Error => write!(f, "[StatementError]"),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::VarDeclaration(ident, value, dims) => write!(f, "Variable Declaration: {} = [{}] with dimensions [{}]",
                ident,
                value,
                dims.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")),
            Variable::FormalParameter(ident, value, dims) => write!(f, "Formal Parameter: {} = [{}] with dimensions [{}]",
                ident,
                value,
                dims.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")),
            Variable::VarReference(ident, dims) => write!(f, "{}{}", 
                ident, dims.iter().map(|d| format!("[{}]", d)).collect::<Vec<String>>().join("][")),
            Variable::VarAssignment(ident, expr) => {
                write!(f, "Variable Assignment: {} = {}", ident, expr.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", "))
            },
            Variable::StructDefinition(ident, vars) => write!(f, "Struct Definition: {} with [{}]",
                ident, 
                vars.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(", ")),
            Variable::StructDeclaration(ident, parent, vars) => write!(f, "Struct Declaration: {} extends {} with [{}]",
                ident, 
                parent, 
                vars.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(", ")),
            Variable::StructReference(vars) => write!(f, "Struct Reference: [{}]",
                vars.iter().map(|v| {
                    match v {
                        Variable::VarReference(ident, dims) => format!("{}{}", ident, dims.iter().map(|d| format!("[{}]", d)).collect::<Vec<String>>().join("][")),
                        _ => format!("{}", v),
                    }
                }).collect::<Vec<String>>().join(", ")),
            Variable::Error => write!(f, "[VariableError]"),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::FuncReference(ident, input_params) => write!(f, "FuncCall: {}[{}]", 
                ident, 
                input_params.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(", ")
            ),
            Function::FuncDeclaration(ident, _input_params, _output_param, body) => write!(f, "Function: {}:[{}]", ident,  body),
            Function::Error => write!(f, "[FunctionError]"),
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
            CompExpr::FuncCall(func) => write!(f, "{}", func),
            CompExpr::MissingRP => write!(f, "[CompExprError]"),
            CompExpr::Invalid => write!(f, "[Invalid]"),
            CompExpr::Error => write!(f, "[CompExprError]"),
        }
    }
}

impl fmt::Display for CondExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CondExpr::Bool(b) => write!(f, "Condition: {}", b),
            CondExpr::UnaryCondition(op, expr) => write!(f, "Condition: {} {}", op, expr),
            CondExpr::Condition(left, op, right) => write!(f, "Condition: {} {} {}", left, op, right),
            CondExpr::BinaryCondition(left, op, right) => write!(f, "Condition: {} {} {}", left, op, right),
            CondExpr::Error => write!(f, "[CondExprError]"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}: u32", i),
            Value::Float(fl) => write!(f, "{}: f32", fl),
            Value::String(s) => write!(f, "{}: String", s),
            Value::Char(c) => write!(f, "{}: char", c),
            Value::Bool(b) => write!(f, "{}: bool", b),
            Value::Struct(s) => write!(f, "Struct({})", s),
            Value::Pointer(p) => write!(f, "Pointer({})", p),
            Value::Null => write!(f, "null")
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
            BinaryOperator::BitwiseXor => write!(f, "^"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::Error => write!(f, "[BinaryOperatorError]"),
            BinaryOperator::BitwiseAnd => write!(f, "&"),
            BinaryOperator::BitwiseOr => write!(f, "|"),
        }
    }
}


impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::Inc => write!(f, "++"),
            UnaryOperator::Dec => write!(f, "--"),
            UnaryOperator::Error => write!(f, "[UnaryOperatorError]"),
            UnaryOperator::Ref => write!(f, "&"),
            UnaryOperator::Deref => write!(f, "*"),
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
            JudgeOperator::Error => write!(f, "[JudgeOperatorError]"),
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            If::IfExpr(cond, body) => write!(f, "If: {} then {}", cond, body),
            If::IfElseExpr(cond, body, opt_body) => 
                write!(f, "If: {} then {} else {}", cond, body, opt_body),
            If::Error => write!(f, "[IfError]"),
        }
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Loop::WhileExpr(cond, body) => write!(f, "While Loop ({}):\ndo {}", cond, body),
            Loop::ForExpr(init, cond, update, body) => write!(f, "For Loop ([Initial] {}; [Condition] {}; [Increment] {}): \n do {}", init, cond, update, body),
            Loop::Error => write!(f, "[LoopError]"),
        }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Body(expressions) => {
                write!(f, "Body: [{}]", 
                    expressions.iter().map(|expr| format!("{}", expr)).collect::<Vec<String>>().join(", "))
            },
            Body::Error => write!(f, "[BodyError]"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::If(if_expr, _) => write!(f, "{}", if_expr),
            Expr::Loop(loop_expr, _) => write!(f, "{}", loop_expr),
            Expr::Break(_) => write!(f, "Break"),
            Expr::Continue(_) => write!(f, "Continue"),
            Expr::Body(body, _) => write!(f, "\nNested {}", body),
            Expr::Return(val, _) => write!(f, "Return: {}", val),
            Expr::FuncCall(func, _) => write!(f, "{}", func),
            Expr::VarManagement(vars, _) => write!(f, "{}", 
                vars.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join("; ")),
            Expr::Error => write!(f, "[ExprError]"),
        }
    }
}
