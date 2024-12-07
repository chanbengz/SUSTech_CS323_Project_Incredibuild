use crate::{error::SemanticError, symbol::{VarType, BasicType}};


pub struct TypeChecker{
    pub current_scope: ScopeType,
    pub current_type: BasicType,
    pub func_ret_type: FuncRetType,
    pub line: usize
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker{
            current_scope: ScopeType::Global,
            current_type: BasicType::Null,
            func_ret_type: FuncRetType::Null,
            line: 0
        }
    }

    pub fn check_binary_operations(&self, ltype: BasicType, rtype: BasicType) -> Result<BasicType, SemanticError> {
        if ltype == rtype {
            return Ok(ltype);
        } else {
            return Err(SemanticError::ImproperUsageError {
                id: 7,
                message: "Unmatched operands, such as adding an integer to a structure variable".to_string(),
                line: self.line,
            });
        }
    }

    pub fn check_assign_operation(&self, ltype: BasicType, rtype: BasicType) -> Result<BasicType, SemanticError> {
        if ltype == rtype {
            return Ok(rtype);
        } else {
            return Err(SemanticError::ImproperUsageError {
                id: 5,
                message: "Unmatched types appear at both sides of the assignment operator (=)".to_string(),
                line: self.line,
            });
        }
    }

    pub fn check_var_type(&self, var: VarType) -> Result<BasicType, SemanticError> {
        match var {
            VarType::Primitive(t) => Ok(t),
            VarType::Array((t, _)) => Ok(t),
            VarType::Struct(_) => Err(SemanticError::ImproperUsageError{
                id: 22,
                message: "Invalid use of struct in computation expression".to_owned(),
                line: self.line
            }),
        }
    }

    pub fn check_condition(&self, ltype: BasicType, rtype: BasicType) -> Result<BasicType, SemanticError>{
        match (ltype, rtype) {
            (BasicType::Int, BasicType::Int) => {
                Ok(BasicType::Bool)
            }
            (BasicType::Float, BasicType::Float) => {
                Ok(BasicType::Bool)
            }
            _ => {
                Err(SemanticError::TypeError{ 
                    id: 7, 
                    message: "Unmatched operands, such as adding an integer to a structure variable".to_owned(), 
                    line: self.line
                })
            }
        }
    }

    pub fn check_binary_condition(&self, ltype: BasicType, rtype: BasicType) -> Result<BasicType, SemanticError>{
        match (ltype, rtype) {
            (BasicType::Bool, BasicType::Bool) => {
                Ok(BasicType::Bool)
            }
            _ => {
                Err(SemanticError::TypeError{ 
                    id: 7, 
                    message: "Unmatched operands, such as adding an integer to a structure variable".to_owned(), 
                    line: self.line
                })
            }
        }
    }

    pub fn check_ret_type(&self, type_t: BasicType) -> Result<(), SemanticError>{
        if FuncRetType::from(type_t) == self.func_ret_type {
            Ok(())
        } else {
            Err(SemanticError::TypeError{
                id: 8,
                message: "A function’s return value type mismatches the declared type".to_string(),
                line: self.line
            })
        }
    }

    pub fn check_func_params(&self, params: Vec<BasicType>, args: Vec<BasicType>) -> Result<(), SemanticError>{
        if params.len() != args.len() {
            return Err(SemanticError::TypeError{
                id: 9,
                message: "The number of arguments passed to a function does not match the number of parameters in the function definition".to_string(),
                line: self.line
            });
        }
        for i in 0..params.len() {
            if params[i] != args[i] {
                return Err(SemanticError::TypeError{
                    id: 10,
                    message: "The type of an argument passed to a function does not match the type of the corresponding parameter in the function definition".to_string(),
                    line: self.line
                });
            }
        }
        Ok(())
    }

    // This is used to check the struct field.
    // When accessing a struct field, it is ensured that the field is defined in the struct.
    pub fn check_struct_field(&self, field_name: String, fields: Vec<(String, VarType)>) -> Result<VarType, SemanticError>{
        for (name, var) in fields {
            if name == field_name {
                return Ok(var);
            }
        }
        return Err(SemanticError::ImproperUsageError{
            id: 14,
            message: "Accessing an undefined structure member".to_string(),
            line: self.line
        });
    }

    // This is used when doing member assignments.
    // It is ensure that the member is defined in the struct.
    pub fn check_member_reference(&self, var: Vec<(String, VarType)>, reference: Vec<(String, String, Vec<usize>)>) -> Result<(BasicType, usize), SemanticError>{
        let mut var_def: Vec<String, VarType> = var;
        reference.iter_mut().for_each(|(var_name, field_name, usize)| {
            let mut found = false;
            match self.check_struct_field(field_name, var_def) {
                Ok(t) => {
                    match t {
                        VarType::Struct(s) => {
                            var_def = s.1;
                        },
                        VarType::BasicType(b) => {
                            return Ok((b, 0));
                        },
                        VarType::Array(a) => {
                            match self.check_array_type(a, usize) {
                                Ok((b, post_size)) => {
                                    return Ok((b, post_size));
                                },
                                Err(e) => return Err(e)
                            }
                        }
                    };
                    found = true;
                }
                Err(e) => return Err(e)
            }
            if found == false {
                return Err(SemanticError::ImproperUsageError{
                    id: 14,
                    message: "Accessing an undefined structure member".to_string(),
                    line: self.line
                });
            }
        });
    }

    // This is used to check the array offset.
    pub fn check_array_type(&self, var: (BasicType, Vec<usize>), reference: Vec<usize>) -> Result<(BasicType, usize), SemanticError>{
        if var.1.len() < reference.len() {
            return Err(SemanticError::TypeError{
                id: 10,
                message: "Applying indexing operator ([…]) on non-array type variables".to_string(),
                line: 0
            });
        }
        for i in 0..reference.len() {
            if var.1[i] < reference.1[i] {
                return Err(SemanticError::ImproperUsageError{
                    id: 21,
                    message: "Index out of bounds.".to_string(),
                    line: 0
                });
            }
            
        }
        return Ok((var.0, var.1.len() - reference.len()));
    }
    

    pub fn set_scope(&mut self, scope: ScopeType) -> ScopeType {
        let prev_scope = self.current_scope.clone();
        self.current_scope = scope;
        prev_scope
    }

    pub fn get_scope(&self) -> ScopeType {
        self.current_scope.clone()
    }

    pub fn set_type(&mut self, t: BasicType) {
        self.current_type = t;
    }

    pub fn set_ret_type(&mut self, t: BasicType){
        match t {
            BasicType::Int => self.func_ret_type = FuncRetType::Int,
            BasicType::Float => self.func_ret_type = FuncRetType::Float,
            BasicType::Char => self.func_ret_type = FuncRetType::Char,
            BasicType::Bool => self.func_ret_type = FuncRetType::Bool,
            BasicType::Null => self.func_ret_type = FuncRetType::Void,
            BasicType::String => self.func_ret_type = FuncRetType::String,
        }
    }

    pub fn reset_ret_type(&mut self){
        self.func_ret_type = FuncRetType::Null
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScopeType {
    Global,
    Func,
    LoopExpr
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FuncRetType {
	Int,
	Char,
	Float,
	Bool,
	String,
	Void,
    Null
}