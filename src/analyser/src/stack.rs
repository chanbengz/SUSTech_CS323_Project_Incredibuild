use crate::table::ScopeTable;
use crate::symbol::{VarSymbol, FuncSymbol};
use std::rc::Rc;
use std::cell::RefCell;
use crate::error::SemanticError;

#[derive(Clone, Debug)]
pub struct ScopeStack {
    pub func_scope: Rc<RefCell<ScopeTable<FuncSymbol>>>,
    pub stack: Vec<Rc<RefCell<ScopeTable<VarSymbol>>>>,
    depth: usize,
}

impl ScopeStack {
    pub fn new() -> Self {
        let func_scope = Rc::new(RefCell::new(ScopeTable::new()));
        let stack = vec![Rc::new(RefCell::new(ScopeTable::new()))];
        ScopeStack { func_scope, stack , depth: 0}
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

    pub fn update_var_symbol(&self, symbol: VarSymbol) -> Result<(), SemanticError> {
        // Update the symbol in the top scope
        for scope in self.stack.iter().rev() {
            if let Some(current_symbol) = scope.borrow().lookup(&symbol.identifier) {
                let mut current_symbol = current_symbol.clone();
                current_symbol.symbol_type = symbol.symbol_type.clone();
                return Ok(());
            }
        }
        Err(SemanticError::ReferenceError {
            id: 1,
            variable: symbol.identifier.clone(),
            line: 0,
        })
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
