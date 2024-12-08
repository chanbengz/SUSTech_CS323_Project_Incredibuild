use std::fmt;
use std::fmt::format;

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

    pub fn check_binary_operations(&self, ltype: VarType, rtype: VarType) -> Result<VarType, SemanticError> {
        if ltype == rtype {
            return Ok(ltype);
        } else {
            return Err(SemanticError::ImproperUsageError {
                id: 7,
                message: format!("Unmatched operands, conducting operations between {} and {}", ltype, rtype),
                line: 0,
            });
        }
    }

    pub fn check_assign_operation(&self, ltype: VarType, rtype: VarType) -> Result<VarType, SemanticError> {
        if ltype == rtype {
            return Ok(rtype);
        } else {
            return Err(SemanticError::ImproperUsageError {
                id: 5,
                message: format!("Assigning a value of type {} to a variable of type {}", rtype, ltype),
                line: 0,
            });
        }
    }

    pub fn check_condition(&self, ltype: VarType, rtype: VarType) -> Result<BasicType, SemanticError>{
        match (ltype, rtype) {
            (VarType::Primitive(BasicType::Int), VarType::Primitive(BasicType::Int)) => {
                Ok(BasicType::Bool)
            }
            (VarType::Primitive(BasicType::Float), VarType::Primitive(BasicType::Float)) => {
                Ok(BasicType::Bool)
            }
            _ => {
                Err(SemanticError::TypeError{ 
                    id: 7, 
                    message: "Only type Int and type Float are supported in condition.".to_owned(), 
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
                    message: "Only type Bool is supported in binary condition.".to_owned(),
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
                message: format!("The return type of the function should be {}, but get {}", self.func_ret_type, type_t),
                line: 0
            })
        }
    }

    pub fn check_func_params(&self, params: Vec<VarType>, args: Vec<VarType>) -> Result<(), SemanticError>{
        if params.len() != args.len() {
            return Err(SemanticError::TypeError{
                id: 9,
                message: format!("The number of arguments passed to a function does not match the number of parameters in the function definition. Expected {} arguments, but got {}", params.len(), args.len()),
                line: 0
            });
        }
        for i in 0..params.len() {
            if params[i] != args[i] {
                return Err(SemanticError::TypeError{
                    id: 10,
                    message: format!("The type of the {}th argument does not match the type of the parameter. Expected {}, but got {}", i+1, params[i], args[i]),
                    line: 0
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
            message: format!("Field {} is not defined in the struct.", field_name),
            line: 0
        });
    }

    // This is used when doing member assignments.
    // var_type represents the type of the struct variable.
    pub fn check_member_reference(&mut self, var_type: VarType, members: Vec<(String, String, Vec<usize>)>) -> Result<VarType, SemanticError> {
        let mut current_type = var_type;
        let mut struct_fields = match current_type {
            VarType::Struct((_, ref fields)) => fields,
            _ => return Err(SemanticError::ImproperUsageError{
                id: 13,
                message: "Accessing a member of a non-structure variable".to_string(),
                line: 0
            })
        };
        let mut members_count = members.len();
        for (_, member_name, dim_indices) in members {

            let field = self.check_struct_field(member_name, struct_fields.clone())?;
            current_type = self.check_type(field, dim_indices)?;

            if members_count == 0 {
                return Ok(current_type);
            }
            
            members_count -= 1;
            match current_type {
                VarType::Struct((_, ref fields)) => {
                    struct_fields = fields;
                }
                _ => {
                    if members_count != 0 {
                        return Err(SemanticError::ImproperUsageError{
                            id: 13,
                            message: "Accessing a member of a non-structure variable".to_string(),
                            line: 0
                        });
                    } else {
                        return Ok(current_type);
                    }
                }
            }
        }
        Ok(current_type)
    }

    // This is used to check the type (if it is an array) and the indices.
    pub fn check_type(&self, var_type: VarType, reference: Vec<usize>) -> Result<VarType, SemanticError> {
        match var_type {
            VarType::Array((basic_type, dims)) => {
                let num_indices = reference.len();
                let num_dims = dims.len();

                if num_indices > num_dims {
                    return Err(SemanticError::ImproperUsageError{
                        id: 20,
                        message: format!("Expected to have {} indices, but got {}", num_dims, num_indices),
                        line: 0
                    });
                } else if num_indices == num_dims {
                    for i in 0..num_indices {
                        if reference[i] >= dims[i] {
                            return Err(SemanticError::ImproperUsageError{
                                id: 21,
                                message: format!("Index {} is out of bounds: {} > {}", i, reference[i], dims[i]),
                                line: 0
                            });
                        }
                    }
                    return Ok(VarType::Primitive(basic_type));
                } else {
                    let remaining_dims = dims[num_indices..].to_vec();
                    return Ok(VarType::Array((basic_type, remaining_dims)));
                }
            }
            _ => {
                return Ok(var_type);
            }
        }
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

impl fmt::Display for FuncRetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FuncRetType::Int => write!(f, "Int"),
            FuncRetType::Char => write!(f, "Char"),
            FuncRetType::Float => write!(f, "Float"),
            FuncRetType::Bool => write!(f, "Bool"),
            FuncRetType::String => write!(f, "String"),
            FuncRetType::Void => write!(f, "Void"),
            FuncRetType::Null => write!(f, "Null"),
        }
    }
}