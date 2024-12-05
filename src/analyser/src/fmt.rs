use std::fmt::{Display, Result};
use crate::symbol::{Symbol, VarType, BasicType, FuncReturnType};
use crate::table::ScopeTable;
use crate::stack::ScopeStack;

impl<T> Display for Symbol<T> 
where 
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        write!(f, "Symbol: id: {}, is_global: {}, identifier: {}, symbol_type: {:}", self.id, self.is_global, self.identifier, self.symbol_type)
    }
}

impl Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        match self {
            VarType::Primitive(basic_type) => {
                write!(f, "Primitive: type: {:?}", basic_type)
            },
            VarType::Array(array_type) => {
                write!(f, "Array: {:?}", array_type)
            },
            VarType::Struct(struct_type) => {
                write!(f, "Struct: {:?}", struct_type)
            }
        }
    }
}

impl Display for BasicType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for FuncReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Clone + std::fmt::Debug> Display for ScopeTable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        for (key, value) in &self.symbols {
            result.push_str(&format!("{}: ", key));
            result.push_str(&format!("{:?}\n", value));
        }
        write!(f, "{}", result)
    }
}

impl Display for ScopeStack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str("Function Table");
        result.push_str(&format!("  {:?}\n", self.func_scope.borrow()));
        result.push_str("Variable Table");
        for scope in &self.stack {
            result.push_str(&format!("  {:?}\n", scope.borrow()));
        }
        write!(f, "{}", result)
    }
}