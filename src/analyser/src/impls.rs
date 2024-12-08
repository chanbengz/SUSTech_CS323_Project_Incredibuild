use crate::symbol::*;
use crate::manager::SymbolManager;

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

// From for VarSymbol
impl VarSymbol {
    pub fn primitive(manager: &mut SymbolManager, identifier: String, type_t: BasicType, is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Primitive(type_t), is_global)
    }
    
    pub fn array(manager: &mut SymbolManager, identifier: String, type_t: BasicType, dimensions: Vec<usize>, is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Array((type_t, dimensions)), is_global)
    }

    pub fn struct_type(manager: &mut SymbolManager, struct_type: String, identifier: String, fields: Vec<(String, VarType)>, is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Struct((struct_type, fields)), is_global)
    }

    pub fn get_primitive(&self) -> Option<BasicType> {
        match &self.symbol_type {
            VarType::Primitive(t) => Some(*t),
            _ => None,
        }
    }

    pub fn get_array_dimensions(&self) -> Option<Vec<usize>> {
        match &self.symbol_type {
            VarType::Array((_, d)) => Some(d.clone()),
            _ => None,
        }
    }

    pub fn get_struct_field(&self, field: String) -> Option<VarType> {
        match &self.symbol_type {
            VarType::Struct((_ , fields)) => {
                for i in fields.iter() {
                    if i.0 == field {
                        return Some(i.1.clone());
                    }
                }
                None
            },
            _ => None,
        }
    }

    pub fn get_struct_type(&self) -> Option<String> {
        match &self.symbol_type {
            VarType::Struct((t, _)) => Some(t.clone()),
            _ => None,
        }
    }
}

impl PartialEq for VarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VarType::Primitive(a), VarType::Primitive(b)) => a == b,
            (VarType::Array((type_a, dims_a)), VarType::Array((type_b, dims_b))) => {
                type_a == type_b && dims_a.len() == dims_b.len()
            },
            (VarType::Struct(a), VarType::Struct(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for VarType {}