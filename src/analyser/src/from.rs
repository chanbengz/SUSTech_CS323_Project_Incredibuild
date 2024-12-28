use spl_ast::tree::Value;
use crate::symbol::*;

impl From<Value> for BasicType {
    fn from(value: Value) -> BasicType {
        match value {
            Value::Integer(_) => BasicType::Int,
            Value::Float(_) => BasicType::Float,
            Value::Char(_) => BasicType::Char,
            Value::Bool(_) => BasicType::Bool,
            Value::String(_) => BasicType::String,
            Value::Struct(obj) => BasicType::Struct(obj),
            Value::Pointer(e) => BasicType::Pointer(Box::new(BasicType::from(*e))),
            Value::Null => BasicType::Null
        }
    }
}