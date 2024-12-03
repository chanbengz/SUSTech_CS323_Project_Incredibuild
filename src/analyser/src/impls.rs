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
    pub fn primitive(manager: &mut SymbolManager, identifier: String, type_t: BasicType, value: Val, is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Primitive((type_t, value)), is_global)
    }
    
    pub fn array(manager: &mut SymbolManager, identifier: String, type_t: BasicType, value: Vec<Val>, dimensions: Vec<usize>, is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Array((type_t, value, dimensions)), is_global)
    }

    pub fn struct_type(manager: &mut SymbolManager, identifier: String, fields: (Vec<String>, Vec<VarType>), is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Struct(fields), is_global)
    }

    pub fn get_primitive(&self) -> Option<(BasicType, Val)> {
        match &self.symbol_type {
            VarType::Primitive((t, v)) => Some((*t, v.clone())),
            _ => None,
        }
    }

    pub fn get_array_dimensions(&self) -> Option<Vec<usize>> {
        match &self.symbol_type {
            VarType::Array((_, _, d)) => Some(d.clone()),
            _ => None,
        }
    }

    pub fn get_array_value(&self, index: Vec<usize>) -> Option<Val> {
        let mut index_value: usize = 0;
        let dimensions = self.get_array_dimensions().unwrap();
        for i in 0..index.len() {
            index_value += index[i] * dimensions[i];
        }
        match &self.symbol_type {
            VarType::Array((_, v, _)) => Some(v[index_value].clone()),
            _ => None,
        }
    }

    pub fn get_struct_field(&self, field: String) -> Option<(BasicType, Val)> {
        match &self.symbol_type {
            VarType::Struct((fields, types)) => {
                for i in 0..fields.len() {
                    if fields[i] == field {
                        return match &types[i] {
                            VarType::Primitive((t, v)) => Some((*t, v.clone())),
                            _ => None,
                        }
                    }
                }
                None
            },
            _ => None,
        }
    }
}

// From for FuncSymbol
impl FuncSymbol {
    fn define(manager: &mut SymbolManager, identifier: String, return_type: FuncReturnType, parameters: Vec<VarType>) -> FuncSymbol {
        manager.new_func_symbol(identifier, (return_type, parameters), true)
    }

    fn get_return_type(&self) -> FuncReturnType {
        match &self.symbol_type {
            (t, _) => t.clone(),
        }
    }

    fn get_parameters(&self) -> Vec<VarType> {
        match &self.symbol_type {
            (_, p) => p.clone(),
        }
    }
}