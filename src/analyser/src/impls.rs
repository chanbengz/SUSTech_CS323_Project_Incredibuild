use crate::symbol::*;
use spl_ast::tree::Value;

impl From<Value> for Type {
    fn from(value: Value) -> Type {
        match value {
            Value::Integer(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Char(_) => Type::Char,
            Value::Bool(_) => Type::Bool,
            Value::Null => Type::Null,
        }
    }
}

impl Symbol<VarType> {
	pub fn new(id: i32, is_global: bool, identifier: String, symbol_type: VarType) -> Symbol<VarType> {
		Symbol {
			id,
			is_global,
			identifier,
			symbol_type,
			symbol_table_next: None,
			scope_stack_next: None
		}
	}
}

impl Symbol<FuncType> {
	pub fn new(id: i32, is_global: bool, identifier: String, symbol_type: FuncType) -> Symbol<FuncType> {
		Symbol {
			id,
			is_global,
			identifier,
			symbol_type,
			symbol_table_next: None,
			scope_stack_next: None
		}
	}
}