use spl_ast::tree::*;
use std::fs::File;
use std::io::Read;
use spl_parser::parse;
use crate::manager::SymbolManager;
use crate::error::{SemanticError, SemanticErrorManager};
use crate::symbol::*;
use crate::stack::ScopeStack;
use crate::typer::{TypeChecker, ScopeType};
use spl_lexer::tokens::Span;

pub struct Walker {
    pub program: Program,
    pub program_source: String,
    pub symbol_tables: ScopeStack,
    pub manager: SymbolManager,
    pub errors: SemanticErrorManager,
    pub typer: TypeChecker
}

impl Walker {
    pub fn new(program_source: &str) -> Walker {
        let mut src_content = String::new();
        let mut src_file = File::open(program_source).expect("Unable to open file");
        src_file.read_to_string(&mut src_content).expect("Unable to read file");

        let ast = parse(&src_content).unwrap();

        Walker {
            program: ast,
            program_source: src_content,
            manager: SymbolManager::default(),
            errors: SemanticErrorManager::new(),
            symbol_tables: ScopeStack::new(),
            typer: TypeChecker::new()
        }
    }

    pub fn get_tables(&self) -> ScopeStack {
        self.symbol_tables.clone()
    }

    pub fn get_errors(&self) -> &Vec<SemanticError> {
        self.errors.get_errors()
    }

    pub fn update_line(&mut self) {
        self.errors.update_line();
    }

    pub fn update_line_with_span(&mut self, span: &Span) {
        let lineno = self.program_source[..span.start].lines().count();
        self.errors.update_line_with_value(lineno);
        // let lineno = if lineno == 0 { 1.to_string() } else { lineno.to_string() };
        // let begin = self.program_source[..span.start].rfind('\n').unwrap_or(0);
        // let line_str = self.program_source.lines().nth(lineno.parse::<usize>().unwrap() - 1).unwrap();
        // let padding = " ".repeat(lineno.len() + 1);
        // let padding_msg = " ".repeat(span.start - begin);
        // let bar = "|".purple();
        // let mut indicator = "^".to_string();
        // indicator.push_str(&"~".repeat(span.end - span.start - 1));
        // println!("{} {}:{lineno}:{}: {error_msg}\n{padding}{}\n{} {} {line_str}\n{padding}{}{padding_msg}{}",
        //          "-->".purple(), span.source, span.start, &bar, lineno.purple(), &bar, bar, indicator.red());
    }

    pub fn traverse(&mut self) {
        let program_clone = self.program.clone();
        println!("===================================Traversing Programs===================================");
        self.traverse_program(&program_clone);
    }

    fn traverse_program(&mut self, program: &Program) {
        match program {
            Program::Program(parts) => {
                println!("Program");
                self.update_line();
                for part in parts {
                    self.traverse_program_part(part);
                }
            }
            Program::Error => {
                println!("Error in Program");
            }
        }
    }

    fn traverse_program_part(&mut self, part: &ProgramPart) {
        match part {
            ProgramPart::Statement(statement) => {
                println!("Statement");
                self.traverse_statement(statement);
            }
            ProgramPart::Function(function) => {
                println!("Function");
                self.traverse_function(function);
            }
        }
    }

    fn traverse_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Include(include, span) => {
                println!("Include: {:?}", include);
                self.update_line_with_span(span);
            },
            Statement::GlobalVariable(vars, span) => {
                println!("Global Variables");
                self.update_line_with_span(span);
                for var in vars {
                    self.traverse_variable(var);
                }
            }
            Statement::Struct(var, span) => {
                println!("Struct");
                self.update_line_with_span(span);
                self.traverse_variable(var);
            }
            Statement::Error => println!("Error in Statements.")
        }
    }

    fn handle_dimensions(&mut self, dimensions: Vec<CompExpr>) -> Option<Vec<usize>> {
        if dimensions.len() == 0 {
            return Some(Vec::new());
        }

        let dim = dimensions.iter()
            .map(|comp_expr| {
                match comp_expr {
                    CompExpr::Value(Value::Integer(value)) => Ok(*value as usize),
                    _ => {
                        if let Some(VarType::Primitive(BasicType::Int)) = self.traverse_comp_expr(&comp_expr) {
                            Ok(0_usize)
                        } else {
                            Err(SemanticError::ImproperUsageError {
                                id: 12,
                                message: "Array indexing with a non-integer type expression".to_owned(),
                                line: 0,
                            })
                        }
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| {
                self.errors.add_error(err);
            })
            .ok()?;

        if dim.len() != dimensions.len() {
            self.errors.add_error(SemanticError::ImproperUsageError {
                id: 12,
                message: "Array indexing with a non-integer type expression".to_owned(),
                line: 0,
            });
            None
        }else{
            Some(dim)
        }
    }
    
    fn traverse_variable(&mut self, variable: &Variable) -> Option<VarType> {
        match variable {
            Variable::VarReference(name, dimensions) => {
                println!("VarReference: {:?}, Dimensions: {:?}", name, dimensions);
                let dim = self.handle_dimensions(*dimensions.clone())?;
                let symbol = self.symbol_tables.get_var_symbol(name).map_err(|err| {
                    self.errors.add_error(err);
                }).ok()?;

                let var_type = self.typer.check_type(symbol.symbol_type.clone(), dim).map_err(|err| {
                    self.errors.add_error(err);
                }).ok()?;
                Some(var_type)
            }
            
            Variable::VarDeclaration(name, values, dimensions) => {
                println!("VarDeclaration: {:?}, Values: {:?}, Dimensions: {:?}", name, values, dimensions);
                let dim = self.handle_dimensions(*dimensions.clone())?;

                let symbol_type = BasicType::from(*values.clone());
                let var_type = if dim.len() > 0 {
                        VarType::Array((symbol_type, dim))
                    } else {
                        VarType::Primitive(symbol_type)
                    };

                let new_symbol = self.manager.new_var_symbol(
                    *name.clone(), 
                    var_type.clone(), 
                    false,
                );

                match self.symbol_tables.define_var_symbol(new_symbol) {
                    Ok(()) => Some(var_type),
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            
            Variable::VarAssignment(name, value, dimensions) => {
                println!("VarAssignment: {:?}, Value: {:?}, Dimensions: {:?}", name, value, dimensions);
                // Calculate the type of right hand side
                let right_type = self.traverse_comp_expr(value)?;

                let dim = self.handle_dimensions(*dimensions.clone())?;

                let symbol = self.symbol_tables.get_var_symbol(name).map_err(|err| {
                    self.errors.add_error(err);
                }).ok()?;

                let left_type = self.typer.check_type(symbol.symbol_type.clone(), dim).map_err(|err| {
                    self.errors.add_error(err);
                }).ok()?;

                match self.typer.check_assign_operation(left_type, right_type) {
                    Ok(t) => Some(t),
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }

            // Define in the global scope
            Variable::StructDefinition(name, variables) => {
                println!("StructDefinition: {:?}", name);
                let mut vars: Vec<(String, VarType)> = Vec::new();
                for var in *variables.clone() {
                    if let Some(var_type) = self.traverse_struct_field(&var) {
                        vars.push(var_type);
                    }
                }
                match self.symbol_tables.define_struct((*name.clone(), vars)) {
                    Ok(()) => None,
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            // Define a variable
            // First check if the struct exists
            // Then check if the variable is valid
            Variable::StructDeclaration(obj_type, name, _) => {
                match self.symbol_tables.get_struct(obj_type) {
                    Ok(struct_type) => {
                        let new_symbol = self.manager.new_var_symbol(
                            *name.clone(), 
                            VarType::Struct(struct_type.clone()), 
                            false,
                        );
                        match self.symbol_tables.define_var_symbol(new_symbol) {
                            Ok(()) => Some(VarType::Struct(struct_type.clone())),
                            Err(err) => {
                                self.errors.add_error(err);
                                None
                            }
                        }}
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }

            // TODO: Define a variable
            // First check if the struct exists
            // Then check if the variable is valid
            Variable::StructAssignment(member, value) => {
                println!("StructAssignment: Member: {:?} Value: {:?}", member, value);
                // Check the type of the assigned value
                let left_type = self.traverse_variable(&**member)?;
                let right_type = self.traverse_comp_expr(&**value)?;
                match self.typer.check_assign_operation(left_type, right_type) {
                    Ok(t) => Some(t),
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            Variable::MemberReference(members) => {
                println!("MemberReference: {:?}", members);
                if members.is_empty() {
                    return None;
                }
                
                let mems: Vec<(String, String, Vec<usize>)> = members.clone().iter_mut().map(|(form_name, field_name, dimensions,)|{
                    (*form_name.clone(), *field_name.clone(), self.handle_dimensions(*dimensions.clone()).unwrap_or(Vec::new()))
                }).collect::<Vec<(String, String, Vec<usize>)>>();
                
                let symbol = self.symbol_tables.get_var_symbol(&mems[0].0).map_err(|err| {
                    self.errors.add_error(err);
                }).ok()?;

                match self.typer.check_member_reference(symbol.symbol_type.clone(), mems) {
                    Ok(t) => Some(t),
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            },
            
            Variable::FormalParameter(name, values, dimensions) => {
                println!("FormalParameter: {:?}, Values: {:?}, Dimensions: {:?}", name, values, dimensions);
                let symbol_type = BasicType::from(*values.clone());
                let var_type = |dimensions: &[usize]| -> VarType {
                    if !dimensions.is_empty() {
                        VarType::Array((symbol_type.clone(), dimensions.to_vec()))
                    } else {
                        VarType::Primitive(symbol_type.clone())
                    }
                };

                let new_symbol = self.manager.new_var_symbol(
                    *name.clone(), 
                    var_type(&dimensions), 
                    false,
                );

                match self.symbol_tables.define_var_symbol(new_symbol) {
                    Ok(()) => Some(var_type(&dimensions)),
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            Variable::Error => None
        }
    }

    fn traverse_struct_field(&mut self, field: &Variable) -> Option<(String, VarType)> {
        match field {
            Variable::VarDeclaration(varname, type_t, offsets) => {
                let dim = self.handle_dimensions(*offsets.clone()).unwrap_or(Vec::new());
                let symbol_type = BasicType::from(*type_t.clone());
                let var: Option<(String, VarType)> = if dim.len() > 0 {
                    Some((*varname.clone(), VarType::Array((symbol_type, dim))))
                } else {
                    Some((*varname.clone(), VarType::Primitive(symbol_type)))
                };
                var
            }
            Variable::StructDeclaration(type_t, identifier, _) => {
                match self.symbol_tables.get_struct(type_t) {
                    Ok(struct_type) => {
                        Some((*identifier.clone(), VarType::Struct(struct_type.clone())))
                    }
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            _ => None
        }
    }

    fn traverse_function(&mut self, function: &Function) -> Option<FuncType>{
        match function {
            Function::FuncReference(name, params) => {
                println!("FuncReference: {:?}, Params: {:?}", name, params);
                let mut args: Vec<VarType> = Vec::new();
                for param in params {
                    if let Some(arg) = self.traverse_comp_expr(param) {
                        args.push(arg);
                    }else{
                        args.push(VarType::Primitive(BasicType::Null));
                    }
                }
                // println!("-> Travesing functions arguments: {:?}", args);
                let func_symbol = self.symbol_tables.get_func_symbol(name).map_err(|err| {
                    self.errors.add_error(err);
                }).ok()?;
                match self.typer.check_func_params(func_symbol.symbol_type.clone().1, args) {
                    Ok(()) => {
                        return Some(func_symbol.symbol_type.clone());
                    }
                    Err(err) => {
                        self.errors.add_error(err);
                        return None;
                    }
                }
            }
            Function::FuncDeclaration(name, inputs, output, body) => {
                self.typer.set_ret_type(BasicType::from(*output.clone()));
                println!("FuncDeclaration: {:?}, Output: {:?}", name, output);
                self.symbol_tables.extend_scope();
                
                let ret_type: BasicType = BasicType::from(*output.clone());
                let mut params: Vec<VarType> = Vec::new();
                for param in inputs {
                    if let Some(var_type) = self.traverse_variable(param) {
                        params.push(var_type);
                    }
                }
                // println!("-> Declaring parameters: {:?}", self.symbol_tables.get_current_scope());
                // println!("-> Travesing functions parameters: {:?}", params);
                let func = self.manager.new_func_symbol(*name.clone(), (ret_type, params), true);
                match self.symbol_tables.define_func_symbol(func) {
                    Ok(()) => {}
                    Err(err) => {
                        self.errors.add_error(err);
                        return None;
                    }
                }

                self.traverse_body(body);

                match self.symbol_tables.exit_scope() {
                    Ok(()) => {}
                    Err(err) => {
                        self.errors.add_error(err)
                    }
                }
                self.typer.reset_ret_type();
                return None;
            }
            Function::Error => {
                return None;
            }
        }
    }

    fn traverse_body(&mut self, body: &Body) {
        match body {
            Body::Body(exprs) => {
                println!("Body");
                self.symbol_tables.extend_scope();
                for expr in exprs {
                    self.traverse_expr(expr);
                }
                match self.symbol_tables.exit_scope() {
                    Ok(()) => {}
                    Err(err) => {
                        self.errors.add_error(err)
                    }
                }
            }
            Body::Error => println!("Error in Body"),
        }
    }

    fn traverse_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::If(if_expr, span) => {
                println!("If Expression");
                self.update_line_with_span(span);
                self.traverse_if(if_expr);
            }
            Expr::Loop(loop_expr, span) => {
                println!("Loop Expression");
                self.update_line_with_span(span);
                self.traverse_loop(loop_expr);
            }
            Expr::VarManagement(vars, span) => {
                println!("VarManagement");
                self.update_line_with_span(span);
                for var in vars {
                    self.traverse_variable(var);
                }
            }
            Expr::FuncCall(function, span) => {
                println!("Function Call");
                self.update_line_with_span(span);
                self.traverse_function(function);
            }
            Expr::Break(span) => {
                println!("Break");
                self.update_line_with_span(span);
                if self.typer.get_scope() != ScopeType::LoopExpr{
                    self.errors.add_error(SemanticError::ImproperUsageError { 
                        id: 17, 
                        message: "Continue and break should only appear in while loop or for loop".to_owned(), 
                        line: 0
                    });
                }
            },
            Expr::Continue(span) => {
                println!("Continue");
                self.update_line_with_span(span);
                if self.typer.get_scope() != ScopeType::LoopExpr{
                    self.errors.add_error(SemanticError::ImproperUsageError { 
                        id: 17, 
                        message: "Continue and break should only appear in while loop or for loop".to_owned(), 
                        line: 0
                    });
                }
            },
            Expr::Return(comp_expr, span) => {
                println!("Return");
                self.update_line_with_span(span);
                match self.traverse_comp_expr(comp_expr) {
                    Some(t) => {
                        let b = match t{
                            VarType::Primitive(b) => b,
                            _ => BasicType::Null
                        };
                        if let Err(err) = self.typer.check_ret_type(b) {
                            self.errors.add_error(err);
                        }
                    }
                    None => {}
                }
            }
            Expr::Body(body, span) => {
                println!("Body");
                self.update_line_with_span(span);
                self.traverse_body(body);
            }
            Expr::Error => println!("Error in Expression"),
        }
    }

    fn traverse_if(&mut self, if_expr: &If) {
        match if_expr {
            If::IfExpr(cond, body) => {
                println!("IfExpr");
                self.traverse_cond_expr(cond);
                self.traverse_body(body);
            }
            If::IfElseExpr(cond, then_body, else_body) => {
                println!("IfElseExpr");
                self.traverse_cond_expr(cond);
                self.traverse_body(then_body);
                self.traverse_body(else_body);
            }
            If::Error => println!("Error in If"),
        }
    }

    fn traverse_loop(&mut self, loop_expr: &Loop) {
        match loop_expr {
            Loop::WhileExpr(cond, body) => {
                println!("WhileExpr");
                self.traverse_cond_expr(cond);
                
                let prev_scope = self.typer.set_scope(ScopeType::LoopExpr);
                self.traverse_body(body);
                self.typer.set_scope(prev_scope);
            }
            Loop::ForExpr(init, cond, increment, body) => {
                println!("ForExpr");
                self.traverse_expr(init);
                self.traverse_cond_expr(cond);
                self.traverse_expr(increment);

                let prev_scope = self.typer.set_scope(ScopeType::LoopExpr);
                self.traverse_body(body);
                self.typer.set_scope(prev_scope);
            }
            Loop::Error => println!("Error in Loop"),
        }
    }

    fn traverse_cond_expr(&mut self, cond: &CondExpr) -> Option<BasicType> {
        match cond {
            CondExpr::Bool(_) => {
                return Some(BasicType::Bool)
            },
            CondExpr::UnaryCondition(op, expr) => {
                println!("UnaryCondition: {:?}", op);
                self.traverse_cond_expr(expr)
            }
            CondExpr::BinaryCondition(lhs, op, rhs) => {
                println!("BinaryCondition: {:?} {:?} {:?}", lhs, op, rhs);
                let left_type = self.traverse_cond_expr(lhs)?;
                let right_type = self.traverse_cond_expr(rhs)?;
                match self.typer.check_binary_operations(VarType::Primitive(left_type), VarType::Primitive(right_type)) {
                    Ok(_) => Some(BasicType::Bool),
                    Err(err) => {
                        self.errors.add_error(err);
                        return None;
                    }
                }
            }
            CondExpr::Condition(lhs, op, rhs) => {
                println!("Condition: {:?} {:?} {:?}", lhs, op, rhs);
                let left_type = self.traverse_comp_expr(lhs)?;
                let right_type = self.traverse_comp_expr(rhs)?;
                match self.typer.check_condition(left_type, right_type) {
                    Ok(t) => Some(t),
                    Err(err) => {
                        self.errors.add_error(err);
                        return None;
                    }
                }
            }
            CondExpr::Error => None
        }
    }

    fn traverse_comp_expr(&mut self, comp: &CompExpr) -> Option<VarType> {
        match comp {
            CompExpr::Value(value) => {
                println!("Value: {:?}", value);
                return Some(VarType::Primitive(BasicType::from(value.clone())));
            },
            CompExpr::Variable(variable) => {
                println!("Variable: {:?}", variable);
                return self.traverse_variable(variable);
            }
            CompExpr::FuncCall(function) => {
                println!("Function Call");
                let func_type = self.traverse_function(function)?;
                return Some(VarType::Primitive(func_type.0));
            }
            CompExpr::UnaryOperation(op, expr) => {
                println!("UnaryOperation: {:?}", op);
                let expr_type = self.traverse_comp_expr(expr)?;
                if let VarType::Primitive(BasicType::Bool) = expr_type {
                    Some(VarType::Primitive(BasicType::Bool))
                } else {
                    self.errors.add_error(SemanticError::TypeError {
                        id: 20,
                        message: "Unary '!' operator requires boolean operand".to_owned(),
                        line: 0,
                    });
                    None
                }
            }
            CompExpr::BinaryOperation(lhs, op, rhs) => {
                println!("BinaryOperation: {:?} {:?} {:?}", lhs, op, rhs);
                let left_type = self.traverse_comp_expr(lhs)?;
                let right_type = self.traverse_comp_expr(rhs)?;
                match self.typer.check_binary_operations(left_type, right_type) {
                    Ok(t) => Some(t),
                    Err(err) => {
                        self.errors.add_error(err);
                        return None;
                    }
                }
            }
            CompExpr::Error | CompExpr::Invalid | CompExpr::MissingRP => {
                None
            }
        }
    }

}
