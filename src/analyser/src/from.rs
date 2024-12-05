use spl_ast::tree::{Value, Variable, CompExpr};
use crate::manager::SymbolManager;
use crate::symbol::*;
use crate::error::SemanticError;

impl From<Value> for Val {
    fn from(value: Value) -> Val {
        match value {
            Value::Integer(i) => Val::Int(i as i32),
            Value::Float(f) => Val::Float(f),
            Value::Char(c) => Val::Char(c),
            Value::Bool(b) => Val::Bool(b),
            Value::String(s) => Val::String(s),
            Value::Null => Val::Int(-1)
        }
    }
}

impl From<Val> for BasicType {
    fn from(val: Val) -> BasicType {
        match val {
            Val::Int(_) => BasicType::Int,
            Val::Float(_) => BasicType::Float,
            Val::Char(_) => BasicType::Char,
            Val::Bool(_) => BasicType::Bool,
            Val::String(_) => BasicType::String,
            Val::Array(_) => BasicType::Null,
        }
    }
}

// When conducting From methods, check whether the symbol is already in the scope table.
impl From<(&mut SymbolManager, Variable)> for VarSymbol {
    fn from(input: (&mut SymbolManager, Variable)) -> VarSymbol {
        let mut manager = input.0;
        let variable = input.1;
        match variable {
            Variable::VarDeclaration(identifier, value, dimensions) => {
                let val = Val::from(*value);
                let dim = dimensions.iter().map(|d| {
                    match d {
                        CompExpr::Value(Value::Integer(i)) => *i as usize,
                        _ => 0
                    }
                }).collect();
                manager.new_var_symbol(identifier, val, is_global)
            },
            _ => panic!("Invalid Variable Type")
        }
    }
}

