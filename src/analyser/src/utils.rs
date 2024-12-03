use spl_ast::tree::*;

fn traverse_program(program: &Program) {
    match program {
        Program::Program(parts) => {
            println!("Program");
            for part in parts {
                traverse_program_part(part);
            }
        }
        Program::Error => {
            println!("Error in Program");
        }
    }
}

fn traverse_program_part(part: &ProgramPart) {
    match part {
        ProgramPart::Statement(statement) => {
            println!("Statement");
            traverse_statement(statement);
        }
        ProgramPart::Function(function) => {
            println!("Function");
            traverse_function(function);
        }
    }
}

fn traverse_statement(statement: &Statement) {
    match statement {
        Statement::Include(include) => println!("Include: {:?}", include),
        Statement::GlobalVariable(vars) => {
            println!("Global Variables");
            for var in vars {
                traverse_variable(var);
            }
        }
        Statement::Struct(var) => {
            println!("Struct");
            traverse_variable(var);
        }
    }
}

fn traverse_variable(variable: &Variable) {
    match variable {
        Variable::VarReference(name, dimensions) => {
            println!("VarReference: {:?}, Dimensions: {:?}", name, dimensions);
        }
        Variable::VarDeclaration(name, values, dimensions) => {
            println!("VarDeclaration: {:?}, Values: {:?}, Dimensions: {:?}", name, values, dimensions);
        }
        Variable::VarAssignment(name, value, dimensions) => {
            println!("VarAssignment: {:?}, Value: {:?}, Dimensions: {:?}", name, value, dimensions);
        }
        Variable::StructReference(name) => println!("StructReference: {:?}", name),
        Variable::StructDefinition(name, variables) => {
            println!("StructDefinition: {:?}", name);
            for var in variables {
                traverse_variable(var);
            }
        }
        Variable::StructDeclaration(obj_type, name, variables) => {
            println!("StructDeclaration: {:?}, Name: {:?}", obj_type, name);
            for var in variables {
                traverse_variable(var);
            }
        }
        Variable::StructAssignment(name, field, value) => {
            println!("StructAssignment: {:?}, Field: {:?}, Value: {:?}", name, field, value);
        }
        Variable::MemberReference(name, field) => println!("MemberReference: {:?}, Field: {:?}", name, field),
        Variable::FormalParameter(name, values, dimensions) => {
            println!("FormalParameter: {:?}, Values: {:?}, Dimensions: {:?}", name, values, dimensions);
        }
        Variable::Error => println!("Error in Variable"),
    }
}

fn traverse_function(function: &Function) {
    match function {
        Function::FuncReference(name, params) => {
            println!("FuncReference: {:?}, Params: {:?}", name, params);
        }
        Function::FuncDeclaration(name, inputs, output, body) => {
            println!("FuncDeclaration: {:?}, Output: {:?}", name, output);
            for input in inputs {
                traverse_variable(input);
            }
            traverse_body(body);
        }
        Function::Error => println!("Error in Function"),
    }
}

fn traverse_body(body: &Body) {
    match body {
        Body::Body(exprs) => {
            println!("Body");
            for expr in exprs {
                traverse_expr(expr);
            }
        }
        Body::Error => println!("Error in Body"),
    }
}

fn traverse_expr(expr: &Expr) {
    match expr {
        Expr::If(if_expr) => {
            println!("If Expression");
            traverse_if(if_expr);
        }
        Expr::Loop(loop_expr) => {
            println!("Loop Expression");
            traverse_loop(loop_expr);
        }
        Expr::VarManagement(vars) => {
            println!("VarManagement");
            for var in vars {
                traverse_variable(var);
            }
        }
        Expr::FuncCall(function) => {
            println!("Function Call");
            traverse_function(function);
        }
        Expr::Break => println!("Break"),
        Expr::Continue => println!("Continue"),
        Expr::Return(comp_expr) => {
            println!("Return");
            traverse_comp_expr(comp_expr);
        }
        Expr::Error => println!("Error in Expression"),
    }
}

fn traverse_if(if_expr: &If) {
    match if_expr {
        If::IfExpr(cond, body) => {
            println!("IfExpr");
            traverse_cond_expr(cond);
            traverse_body(body);
        }
        If::IfElseExpr(cond, then_body, else_body) => {
            println!("IfElseExpr");
            traverse_cond_expr(cond);
            traverse_body(then_body);
            traverse_body(else_body);
        }
        If::Error => println!("Error in If"),
    }
}

fn traverse_loop(loop_expr: &Loop) {
    match loop_expr {
        Loop::WhileExpr(cond, body) => {
            println!("WhileExpr");
            traverse_cond_expr(cond);
            traverse_body(body);
        }
        Loop::ForExpr(init, cond, increment, body) => {
            println!("ForExpr");
            traverse_expr(init);
            traverse_cond_expr(cond);
            traverse_expr(increment);
            traverse_body(body);
        }
        Loop::Error => println!("Error in Loop"),
    }
}

fn traverse_cond_expr(cond: &CondExpr) {
    match cond {
        CondExpr::Bool(value) => println!("Bool Condition: {:?}", value),
        CondExpr::UnaryCondition(op, expr) => {
            println!("UnaryCondition: {:?}", op);
            traverse_cond_expr(expr);
        }
        CondExpr::BinaryCondition(lhs, op, rhs) => {
            println!("BinaryCondition: {:?} {:?} {:?}", lhs, op, rhs);
            traverse_cond_expr(lhs);
            traverse_cond_expr(rhs);
        }
        CondExpr::Condition(lhs, op, rhs) => {
            println!("Condition: {:?} {:?} {:?}", lhs, op, rhs);
            traverse_comp_expr(lhs);
            traverse_comp_expr(rhs);
        }
        CondExpr::Error => println!("Error in Condition Expression"),
    }
}

fn traverse_comp_expr(comp: &CompExpr) {
    match comp {
        CompExpr::Value(value) => println!("Value: {:?}", value),
        CompExpr::Variable(variable) => {
            println!("Variable");
            traverse_variable(variable);
        }
        CompExpr::FuncCall(function) => {
            println!("Function Call");
            traverse_function(function);
        }
        CompExpr::UnaryOperation(op, expr) => {
            println!("UnaryOperation: {:?}", op);
            traverse_comp_expr(expr);
        }
        CompExpr::BinaryOperation(lhs, op, rhs) => {
            println!("BinaryOperation: {:?} {:?} {:?}", lhs, op, rhs);
            traverse_comp_expr(lhs);
            traverse_comp_expr(rhs);
        }
        CompExpr::Error => println!("Error in Computation Expression"),
    }
}
