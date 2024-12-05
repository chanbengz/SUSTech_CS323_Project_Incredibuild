use std::borrow::Borrow;

use spl_ast::tree::{Value, Variable, CompExpr};
use crate::manager::SymbolManager;
use crate::symbol::*;
use crate::error::SemanticError;

impl From<Value> for BasicType {
    fn from(value: Value) -> BasicType {
        match value {
            Value::Integer(_) => BasicType::Int,
            Value::Float(_) => BasicType::Float,
            Value::Char(_) => BasicType::Char,
            Value::Bool(_) => BasicType::Bool,
            Value::String(_) => BasicType::String,
            Value::Null => BasicType::Null
        }
    }
}
