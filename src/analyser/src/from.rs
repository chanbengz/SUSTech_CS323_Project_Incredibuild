use spl_ast::tree::Value;
use crate::symbol::*;
use crate::typer::FuncRetType;

impl From<Value> for BasicType {
    fn from(value: Value) -> BasicType {
        match value {
            Value::Integer(_) => BasicType::Int,
            Value::Float(_) => BasicType::Float,
            Value::Char(_) => BasicType::Char,
            Value::Bool(_) => BasicType::Bool,
            Value::String(_) => BasicType::String,
            Value::Struct(_) => BasicType::Struct,
            Value::Null => BasicType::Null
        }
    }
}

impl From<BasicType> for FuncRetType {
    fn from(basic_type: BasicType) -> FuncRetType {
        match basic_type {
            BasicType::Int => FuncRetType::Int,
            BasicType::Float => FuncRetType::Float,
            BasicType::Char => FuncRetType::Char,
            BasicType::Bool => FuncRetType::Bool,
            BasicType::String => FuncRetType::String,
            BasicType::Struct => FuncRetType::Void,
            BasicType::Null => FuncRetType::Void
        }
    }
}