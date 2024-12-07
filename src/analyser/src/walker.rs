use spl_ast::tree::*;
use crate::manager::SymbolManager;
use crate::error::{SemanticError, SemanticErrorManager};
use crate::symbol::*;
use crate::stack::ScopeStack;
use crate::typer::{TypeChecker, ScopeType};

pub struct Walker {
    pub program: Program,
    pub symbol_tables: ScopeStack,
    pub manager: SymbolManager,
    pub errors: SemanticErrorManager,
    pub typer: TypeChecker
}

impl Walker {
    pub fn new(program: Program, manager: SymbolManager) -> Walker {
        Walker {
            program: program,
            manager: manager,
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

    pub fn traverse(&mut self) {
        let program_clone = self.program.clone();
        println!("===================================Traversing Programs===================================");
        self.traverse_program(&program_clone);
    }

    fn traverse_program(&mut self, program: &Program) {
        match program {
            Program::Program(parts) => {
                println!("Program");
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
            Statement::Include(include) => println!("Include: {:?}", include),
            Statement::GlobalVariable(vars) => {
                println!("Global Variables");
                for var in vars {
                    self.traverse_variable(var);
                }
            }
            Statement::Struct(var) => {
                println!("Struct");
                self.traverse_variable(var);
            }
            Statement::Error => println!("Error in Statements.")
        }
    }

    fn traverse_variable(&mut self, variable: &Variable) -> Option<VarType> {
        match variable {
            Variable::VarReference(name, dimensions) => {
                println!("VarReference: {:?}, Dimensions: {:?}", name, dimensions);
                let dim = dimensions.iter()
                    .map(|comp_expr| {
                        if let Some(BasicType::Int) = self.traverse_comp_expr(comp_expr) {
                            Ok(0_usize)
                        } else {
                            Err(SemanticError::ImproperUsageError {
                                id: 12,
                                message: "Array indexing with a non-integer type expression".to_owned(),
                                line: 0,
                            })
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| {
                        self.errors.add_error(err);
                    });
                
                //TODO: If error is reported, the dimensions calculation will be faulty. Test r12
                let mut dim_vec: Vec<usize> = Vec::new();
                match dim {
                    Ok(dim) => {
                        dim_vec = dim;
                    }
                    Err(_) => {}
                }
                print!("Dimentions: {:?}\n", dim_vec);
                match self.symbol_tables.validate_var_symbol(name, dim_vec) {
                    Ok(t) => {
                        println!("Variable Reference return type: {:?}", t);
                        Some(t)
                    },
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            Variable::VarDeclaration(name, values, dimensions) => {
                println!("VarDeclaration: {:?}, Values: {:?}, Dimensions: {:?}", name, values, dimensions);
                // Validate all dimensions and collect them
                let dim = dimensions.iter()
                    .map(|comp_expr| {
                        if let Some(BasicType::Int) = self.traverse_comp_expr(comp_expr) {
                            Ok(0_usize)
                        } else {
                            Err(SemanticError::ImproperUsageError {
                                id: 12,
                                message: "Array indexing with a non-integer type expression".to_owned(),
                                line: 0,
                            })
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| {
                        self.errors.add_error(err);
                    });

                let symbol_type = BasicType::from(*values.clone());
                let var_type: Option<VarType> = match dim {
                    Ok(dim) => {
                        if dim.len() > 0 {
                            Some(VarType::Array((symbol_type, dim)))
                        } else {
                            Some(VarType::Primitive(symbol_type))
                        }
                    }
                    Err(_) => None
                };

                let new_symbol = self.manager.new_var_symbol(
                    *name.clone(), 
                    var_type.clone().unwrap(), 
                    false,
                );

                match self.symbol_tables.define_var_symbol(new_symbol) {
                    Ok(()) => Some(var_type.unwrap()),
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            Variable::VarAssignment(name, value, dimensions) => {
                println!("VarAssignment: {:?}, Value: {:?}, Dimensions: {:?}", name, value, dimensions);
                let val_type = self.traverse_comp_expr(value);
                let dim = dimensions.iter()
                    .map(|comp_expr| {
                        if let Some(BasicType::Int) = self.traverse_comp_expr(comp_expr) {
                            Ok(0_usize)
                        } else {
                            Err(SemanticError::ImproperUsageError {
                                id: 12,
                                message: "Array indexing with a non-integer type expression".to_owned(),
                                line: 0,
                            })
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| {
                        self.errors.add_error(err);
                    });
                
                match dim {
                    Ok(dim) => match self.symbol_tables.validate_var_symbol(name, dim) {
                        Ok(t) => {
                            match val_type {
                                Some(type_t) => {
                                    match self.typer.check_var_type(t.clone()) {
                                        Ok(basic_type) if basic_type == type_t => Some(t),
                                        Ok(_) => None,
                                        Err(err) => {
                                            self.errors.add_error(err);
                                            None
                                        }
                                    }
                                },
                                None => None
                            }
                        },
                        Err(err) => {
                            self.errors.add_error(err);
                            None
                        }
                    },
                    Err(_) => None,
                }
                
            }
            
            Variable::StructReference(name) => {
                println!("StructReference: {:?}", name);
                return None;
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
                    Ok(()) => {
                        return None;
                    }
                    Err(err) => {
                        self.errors.add_error(err);
                        return None;
                    }
                }
            }
            // Define a variable
            // First check if the struct exists
            // Then check if the variable is valid
            Variable::StructDeclaration(obj_type, name, variables) => {
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
            // Define a variable
            // First check if the struct exists
            // Then check if the variable is valid
            Variable::StructAssignment(name, field, value) => {
                let var = self.symbol_tables.get_var_symbol(name).map_err(|err| {
                    self.errors.add_error(err);
                }).ok()?;

                let fields = if let VarType::Struct((_, fields)) = var.symbol_type {
                    fields
                } else {
                    self.errors.add_error(SemanticError::ImproperUsageError {
                        id: 13,
                        message: "Accessing members of a non-structure variable (i.e., misuse the dot operator)".to_owned(),
                        line: 0,
                    });
                    return None;
                };

                let basic_type = self.traverse_comp_expr(&**value)?;
                fields.iter()
                    .find(|(field_name, _)| field_name == &*field.clone())
                    .and_then(|(_, field_type)| {
                        match self.typer.check_var_type(field_type.clone()) {
                            Ok(vartype) if basic_type == vartype => Some(field_type.clone()),
                            Ok(_) => {
                                self.errors.add_error(SemanticError::TypeError { 
                                    id: 5, 
                                    message: "Unmatched types appear at both sides of the assignment operator (=)".to_owned(), 
                                    line: 0 
                                });
                                None
                            },
                            Err(err) => {
                                self.errors.add_error(SemanticError::NotImplementedFeatureError {
                                    message: "Not supported assignment to struct field".to_owned() 
                                });
                                None
                            }
                        }
                    })
            }
            Variable::MemberReference(name, field) => {
                match self.symbol_tables.get_var_symbol(name) {
                    Ok(var) => {
                        if let VarType::Struct((struct_name, fields)) = var.symbol_type {
                            fields
                                .iter()
                                .find(|(field_name, _)| field_name == &*field.clone())
                                .map(|(_, field_type)| field_type.clone())
                        } else {
                            self.errors.add_error(SemanticError::ImproperUsageError {
                                id: 13,
                                message: "Accessing members of a non-structure variable (i.e., misuse the dot operator)".to_owned(),
                                line: 0,
                            });
                            None
                        }
                    }
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
            Variable::Error => {
                return None;
            }
        }
    }

    fn traverse_struct_field(&mut self, field: &Variable) -> Option<(String, VarType)> {
        let mut vars: Vec<(String, VarType)> = Vec::new();
        match field {
            Variable::VarDeclaration(varname, type_t, offsets) => {
                let dim = offsets.iter()
                    .map(|comp_expr| {
                        if let Some(BasicType::Int) = self.traverse_comp_expr(comp_expr) {
                            Ok(0_usize)
                        } else {
                            Err(SemanticError::ImproperUsageError {
                                id: 12,
                                message: "Array indexing with a non-integer type expression".to_owned(),
                                line: 0,
                            })
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| {
                        self.errors.add_error(err);
                    });

                let symbol_type = BasicType::from(*type_t.clone());
                let var: Option<(String, VarType)> = match dim {
                    Ok(dim) => {
                        if dim.len() > 0 {
                            Some((*varname.clone(), VarType::Array((symbol_type, dim))))
                        } else {
                            Some((*varname.clone(), VarType::Primitive(symbol_type)))
                        }
                    }
                    Err(_) => None
                };
                var
            }
            Variable::StructDeclaration(type_t, identifier, variables) => {
                match self.symbol_tables.get_struct(type_t) {
                    Ok(struct_type) => {
                        let mut fields: Vec<(String, VarType)> = Vec::new();
                        Some((*identifier.clone(), VarType::Struct((*type_t.clone(), fields))))
                    }
                    Err(err) => {
                        self.errors.add_error(err);
                        None
                    }
                }
            }
            Variable::VarAssignment(_, _, _) => {
                self.errors.add_error(SemanticError::ImproperUsageError {
                    id: 24,
                    message: "Struct field assignment is not allowed".to_owned(),
                    line: 0,
                });
                None
            }
            _ => {
                None
            }
        }
    }

    fn traverse_function(&mut self, function: &Function) -> Option<FuncType>{
        match function {
            Function::FuncReference(name, params) => {
                println!("FuncReference: {:?}, Params: {:?}", name, params);
                let mut args: Vec<BasicType> = Vec::new();
                for param in params {
                    if let Some(arg) = self.traverse_comp_expr(param) {
                        args.push(arg);
                    }else{
                        args.push(BasicType::Null)
                    }
                }
                // println!("-> Travesing functions arguments: {:?}", args);
                let mut params: Vec<BasicType> = Vec::new();
                let mut return_type: FuncType = (BasicType::Null, Vec::new());
                match self.symbol_tables.get_func_symbol(name) {
                    Ok(func_symbol) =>{
                        func_symbol.symbol_type.1.iter().for_each(
                            |t| {
                                match t {
                                    VarType::Primitive(basic_t) => {
                                        params.push(*basic_t);
                                    }
                                    VarType::Array((basic_t, _)) => {
                                        params.push(*basic_t);
                                    }
                                    VarType::Struct((_, _)) => {
                                        self.errors.add_error(SemanticError::NotImplementedFeatureError {
                                            message: "No struct type as function parameter".to_owned(),
                                        });
                                    }
                                }
                            }
                        );
                        return_type = func_symbol.symbol_type;
                    }
                    Err(err) => {
                        self.errors.add_error(err);
                        return None;
                    }
                }
                match self.typer.check_func_params(params, args) {
                    Ok(()) => {
                        return Some(return_type);
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
            Expr::If(if_expr) => {
                println!("If Expression");
                self.traverse_if(if_expr);
            }
            Expr::Loop(loop_expr) => {
                println!("Loop Expression");
                self.traverse_loop(loop_expr);
            }
            Expr::VarManagement(vars) => {
                println!("VarManagement");
                for var in vars {
                    self.traverse_variable(var);
                }
            }
            Expr::FuncCall(function) => {
                println!("Function Call");
                self.traverse_function(function);
            }
            Expr::Break => {
                println!("Break");
                if self.typer.get_scope() != ScopeType::LoopExpr{
                    self.errors.add_error(SemanticError::ImproperUsageError { 
                        id: 17, 
                        message: "Continue and break should only appear in while loop or for loop".to_owned(), 
                        line: 0
                    });
                }
            },
            Expr::Continue => {
                println!("Continue");
                if self.typer.get_scope() != ScopeType::LoopExpr{
                    self.errors.add_error(SemanticError::ImproperUsageError { 
                        id: 17, 
                        message: "Continue and break should only appear in while loop or for loop".to_owned(), 
                        line: 0
                    });
                }
            },
            Expr::Return(comp_expr) => {
                println!("Return");
                match self.traverse_comp_expr(comp_expr) {
                    Some(t) => {
                        if let Err(err) = self.typer.check_ret_type(t) {
                            self.errors.add_error(err);
                        }
                    }
                    None => {}
                }
            }
            Expr::Body(body) => {
                println!("Body");
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
            CondExpr::Bool(value) => {
                return Some(BasicType::Bool)
            },
            CondExpr::UnaryCondition(op, expr) => {
                println!("UnaryCondition: {:?}", op);
                match self.traverse_cond_expr(expr) {
                    Some(t) => {
                        return Some(t);
                    }
                    None => {
                        return None;
                    }
                }
            }
            CondExpr::BinaryCondition(lhs, op, rhs) => {
                println!("BinaryCondition: {:?} {:?} {:?}", lhs, op, rhs);
                if let Some(lhs_type) = self.traverse_cond_expr(lhs) {
                    if let Some(rhs_type) = self.traverse_cond_expr(rhs) {
                        match self.typer.check_binary_condition(lhs_type, rhs_type) {
                            Ok(t) => {
                                return Some(t);
                            }
                            Err(err) => {
                                self.errors.add_error(err);
                                return None;
                            }
                        }
                    }
                }
                return None;
            }
            CondExpr::Condition(lhs, op, rhs) => {
                println!("Condition: {:?} {:?} {:?}", lhs, op, rhs);
                if let Some(lhs_type) = self.traverse_comp_expr(lhs) {
                    if let Some(rhs_type) = self.traverse_comp_expr(rhs) {
                        match self.typer.check_condition(lhs_type, rhs_type) {
                            Ok(t) => {
                                return Some(t);
                            }
                            Err(err) => {
                                self.errors.add_error(err);
                                return None;
                            }
                        }
                    }
                }
                return None;
            }
            CondExpr::Error => {
                return None;
            },
        }
    }

    fn traverse_comp_expr(&mut self, comp: &CompExpr) -> Option<BasicType> {
        match comp {
            CompExpr::Value(value) => {
                println!("Value: {:?}", value);
                return Some(BasicType::from(value.clone()));
            },
            CompExpr::Variable(variable) => {
                println!("Variable: {:?}", variable);
                if let Some(var_type) = self.traverse_variable(variable) {
                    match self.typer.check_var_type(var_type) {
                        Ok(t) => {
                            return Some(t);
                        }
                        Err(err) => {
                            self.errors.add_error(err);
                            return None;
                        }
                    }
                } else {
                    return None;
                }
            }
            CompExpr::FuncCall(function) => {
                println!("Function Call");
                match self.traverse_function(function) {
                    Some((return_type, _)) => {
                       return Some(return_type);
                    }
                    None => {
                        return None;
                    }
                }
            }
            CompExpr::UnaryOperation(op, expr) => {
                println!("UnaryOperation: {:?}", op);
                match self.traverse_comp_expr(expr) {
                    Some(t) => {
                        return Some(t);
                    }
                    None => {
                        return None;
                    }
                }
            }
            CompExpr::BinaryOperation(lhs, op, rhs) => {
                println!("BinaryOperation: {:?} {:?} {:?}", lhs, op, rhs);
                if let Some(lhs_type) = self.traverse_comp_expr(lhs) {
                    if let Some(rhs_type) = self.traverse_comp_expr(rhs) {
                        match self.typer.check_binary_operations(lhs_type, rhs_type) {
                            Ok(t) => {
                                return Some(t);
                            }
                            Err(err) => {
                                self.errors.add_error(err);
                                return None;
                            }
                        }
                    }
                }
                return None;
            }
            CompExpr::Error => {
                return None;
            }
            CompExpr::Invalid => {
                return None;
            }
        }
    }

}
