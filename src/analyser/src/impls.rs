use crate::symbol::*;

impl<T> Symbol<T> {
    pub fn new(id: i32, is_global: bool, identifier: String, symbol_type: T) -> Symbol<T>{
        Symbol {
            id,
            is_global,
            identifier,
            symbol_type,
        }
    }
}

impl PartialEq for VarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VarType::Primitive(a), VarType::Primitive(b)) => a == b,
            (VarType::Array((type_a, dims_a)), VarType::Array((type_b, dims_b))) => {
                type_a == type_b && dims_a.len() == dims_b.len()
            }
            _ => false,
        }
    }
}

impl Eq for VarType {}