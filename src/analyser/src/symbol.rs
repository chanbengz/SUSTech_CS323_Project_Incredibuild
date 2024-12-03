use spl_ast::tree::Value;

#[derive(Clone, Debug)]
pub struct Symbol<T> {
	pub id: i32, // Unique identifier
	pub is_global: bool,
	pub identifier: String, // Symbol Table Key
	pub symbol_type: T, // VarType + FuncType
	pub symbol_table_next: Option<Box<Symbol<T>>>, // Next symbol with the same key
	pub scope_stack_next: Option<Box<Symbol<T>>>, // Next symbol in the same scope stack
}

/*
	There are in total two kinds of Symbol Type:
	- Variable Type
		- Primitive Type (type_t, Value)
			- Int
			- Char
			- Float
			- Bool
			- String
			- Null
		- Array Type (type_t, Vec<usize>, Vec<PrimitiveType>)
		- Struct Type (Vec<PrimitiveType  + Array Type>)
	- Function Type
		- (Return Type, Vec<VarType>)
*/
pub type VarType = PrimType + ArrayType + StructType;
pub type FuncType = (PrimType, Vec<VarType>);
pub type PrimType = (Type, Value);
pub type ArrayType = (Vec<usize>, Vec<PrimType>);
pub type StructType = Vec<VarType>;

pub enum Type {
	Int,
	Char,
	Float,
	Bool,
	String,
	Null
}
