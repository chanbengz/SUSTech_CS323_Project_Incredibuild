use std::fmt::Display;

use crate::symbol::{FuncSymbol, Symbol, VarSymbol, FuncType, VarType};
use spl_ast::tree::Variable;
use spl_ast::tree::Function;


#[derive(Default)]
pub struct SymbolManager {
	cnt: i32,
}

impl SymbolManager {
	pub fn new_var_symbol(
		&mut self,
        identifier: String,
        symbol_type: VarType,
		is_global: bool,
	) -> VarSymbol {
		self.cnt += 1;
		Symbol {
			id: self.cnt,
			is_global,
			identifier,
			symbol_type
		}
	}

	pub fn new_func_symbol(
		&mut self,
        identifier: String,
        symbol_type: FuncType,
        is_global: bool,
	) -> FuncSymbol {
		self.cnt += 1;
		Symbol {
			id: self.cnt,
			is_global,
			identifier,
			symbol_type
		}
	}
}