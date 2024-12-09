#[derive(Clone, Debug, PartialEq)]
pub struct Symbol<T> {
	pub id: i32, // Unique identifier
	pub is_global: bool,
	pub identifier: String, // Symbol Table Key
	pub symbol_type: T, // VarType + FuncType
}

pub type VarSymbol = Symbol<VarType>;
pub type FuncSymbol = Symbol<FuncType>;

/*
	There are in total two kinds of Symbol Type:
	- Variable Type `VarType`
		- Primitive Type `(BasicType, Value)`
			- Int
			- Char
			- Float
			- Bool
			- String
		- Array Type `(Vec<usize>, Vec<Value>)`
		- Struct Type `(Vec<VarType>)`
	- Function Type `FuncType`
		- `(BasicType, Vec<VarType>)`
	Different　Symbols are stored in different Symbol Tables.
*/

#[derive(Clone, Debug)]
pub enum VarType {
	Primitive(PrimType),
	Array(ArrayType),
	Struct(StructType)
}

pub type PrimType = BasicType;
pub type ArrayType = (BasicType, Vec<usize>);
pub type StructType = (String, Vec<(String, VarType)>);
pub type FuncType = (BasicType, Vec<VarType>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BasicType {
	Int,
	Char,
	Float,
	Bool,
	String,
	Struct,
	Null
}