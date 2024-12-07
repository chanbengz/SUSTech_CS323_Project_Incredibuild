use crate::{error::SemanticError, symbol::{VarType, BasicType}};


pub struct TypeChecker{
    pub current_scope: ScopeType,
    pub current_type: BasicType,
    pub func_ret_type: FuncRetType
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker{
            current_scope: ScopeType::Global,
            current_type: BasicType::Null,
            func_ret_type: FuncRetType::Null
        }
    }
    pub fn check_binary_operations(&self, ltype: BasicType, rtype: BasicType) -> Result<BasicType, SemanticError> {
        if ltype == rtype {
            return Ok(ltype);
        } else {
            return Err(SemanticError::ImproperUsageError {
                id: 7,
                message: "Unmatched operands, such as adding an integer to a structure variable".to_string(),
                line: 0,
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
                line: 0
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
                    line: 0 
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
                    line: 0 
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
                line: 0
            })
        }
    }

    pub fn check_func_params(&self, params: Vec<BasicType>, args: Vec<BasicType>) -> Result<(), SemanticError>{
        if params.len() != args.len() {
            return Err(SemanticError::TypeError{
                id: 9,
                message: "The number of arguments passed to a function does not match the number of parameters in the function definition".to_string(),
                line: 0
            });
        }
        for i in 0..params.len() {
            if params[i] != args[i] {
                return Err(SemanticError::TypeError{
                    id: 10,
                    message: "The type of an argument passed to a function does not match the type of the corresponding parameter in the function definition".to_string(),
                    line: 0
                });
            }
        }
        Ok(())
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