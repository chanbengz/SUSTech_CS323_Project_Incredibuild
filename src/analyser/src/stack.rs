use crate::table::ScopeTable;
use crate::symbol::{VarSymbol, FuncSymbol, VarType, StructType};
use std::process::id;
use std::rc::Rc;
use std::cell::RefCell;
use crate::error::SemanticError;

#[derive(Clone, Debug)]
pub struct ScopeStack {
    pub func_scope: Rc<RefCell<ScopeTable<FuncSymbol>>>,
    pub struct_scope: Rc<RefCell<ScopeTable<StructType>>>,
    pub stack: Vec<Rc<RefCell<ScopeTable<VarSymbol>>>>,
    depth: usize,
}

impl ScopeStack {
    pub fn new() -> Self {
        let func_scope = Rc::new(RefCell::new(ScopeTable::new()));
        let struct_scope = Rc::new(RefCell::new(ScopeTable::new()));
        let stack = vec![Rc::new(RefCell::new(ScopeTable::new()))];
        ScopeStack { func_scope, struct_scope, stack , depth: 0}
    }

    // Scope Relevant
    pub fn extend_scope(&mut self) {
        self.stack.push(Rc::new(RefCell::new(ScopeTable::new())));
        self.depth += 1;
    }

    pub fn exit_scope(&mut self) -> Result<(), SemanticError> {
        if self.depth > 0 {
            self.stack.pop();
            self.depth -= 1;
            Ok(())
        }else{
            Err(SemanticError::ScopeError {
                id: 17,
                message: "Scope exit errors".to_string(),
                line: 0,
            })
        }
    }

    pub fn get_current_scope(&self) -> ScopeTable<VarSymbol> {
        self.stack.last().unwrap().borrow().clone()
    }

    pub fn get_current_func_scope(&self) -> ScopeTable<FuncSymbol>{ 
        self.func_scope.borrow().clone()
    }

    // Variable Relevant
    pub fn define_var_symbol(&self, symbol: VarSymbol) -> Result<(), SemanticError> {
        if let Some(current_scope) = self.stack.last() {
            let mut current_scope = current_scope.borrow_mut();
            if current_scope.lookup(&symbol.identifier).is_some() {
                return Err(SemanticError::RedefinitionError {
                    id: 3,
                    variable: symbol.identifier.clone(),
                    line: 0,
                });
            }
            current_scope.insert(symbol.identifier.clone(), symbol);
            Ok(())
        } else {
            Err(SemanticError::ScopeError {
                id: 17,
                message: "No scope found".to_string(),
                line: 0,
            })
        }
    }

    pub fn get_var_symbol(&self, identifier: &String) -> Result<VarSymbol, SemanticError> {
        // Search for the symbol in the stack from top to bottom
        for scope in self.stack.iter().rev() {
            if let Some(symbol) = scope.borrow().lookup(identifier) {
                return Ok(symbol.clone());
            }
        }
        Err(SemanticError::ReferenceError {
            id: 1,
            variable: identifier.clone(),
            line: 0,
        })
    }

    pub fn validate_var_symbol(&self, identifier: &String, dim: Vec<usize>) -> Result<VarType, SemanticError> {
        // Search for the symbol in the stack from top to bottom
        for scope in self.stack.iter().rev() {
            if let Some(symbol) = scope.borrow().lookup(identifier) {
                match &symbol.symbol_type {
                    VarType::Array((_, dimensions)) => {
                        if dimensions.len() != dim.len() {
                            return Err(SemanticError::TypeError {
                                id: 23,
                                message: "Dimension Mismatched".to_string(),
                                line: 0,
                            });
                        }
                        return Ok(symbol.symbol_type.clone());
                    }
                    _ => {
                        if dim.len() > 0 {
                            return Err(SemanticError::TypeError {
                                id: 10,
                                message: "Applying indexing operator ([…]) on non-array type variables".to_string(),
                                line: 0,
                            });
                        }
                        return Ok(symbol.symbol_type.clone());
                    }
                }
            }
        }
        Err(SemanticError::ReferenceError {
            id: 1,
            variable: identifier.clone(),
            line: 0,
        })
    }

    // Struct Relevant
    pub fn define_struct(&self, struct_type: StructType) -> Result<(), SemanticError> {
        let (identifier, fields) = struct_type.clone();
        // Check if there are repetitive field
        let mut field_set = std::collections::HashSet::new();
        for field in fields.iter() {
            if field_set.contains(&field.0) {
                return Err(SemanticError::RedefinitionError {
                    id: 15,
                    variable: field.0.clone(),
                    line: 0,
                });
            }
            field_set.insert(field.0.clone());
        }
        if self.struct_scope.borrow().lookup(&identifier).is_some() {
            return Err(SemanticError::RedefinitionError {
                id: 15,
                variable: identifier.clone(),
                line: 0,
            });
        }else {
            self.struct_scope.borrow_mut().insert(identifier.clone(), struct_type);
            Ok(())
        }
    }

    pub fn get_struct(&self, type_t: &String) -> Result<StructType, SemanticError> {
        if let Some(struct_type) = self.struct_scope.borrow().lookup(type_t) {
            return Ok(struct_type.clone());
        }
        Err(SemanticError::ReferenceError {
            id: 14,
            variable: type_t.clone(),
            line: 0,
        })
    }

    // Function Relevant
    pub fn define_func_symbol(&self, symbol: FuncSymbol) -> Result<(), SemanticError> {
        if self.func_scope.borrow().lookup(&symbol.identifier).is_some() {
            return Err(SemanticError::RedefinitionError {
                id: 4,
                variable: symbol.identifier.clone(),
                line: 0,
            });
        }else {
            self.func_scope.borrow_mut().insert(symbol.identifier.clone(), symbol);
            Ok(())
        }
    }

    pub fn get_func_symbol(&self, identifier: &String) -> Result<FuncSymbol, SemanticError> {
        if let Some(symbol) = self.func_scope.borrow().lookup(identifier) {
            return Ok(symbol.clone());
        }
        Err(SemanticError::ReferenceError {
            id: 2,
            variable: identifier.clone(),
            line: 0,
        })
    }
}
