#[derive(Clone, Debug, PartialEq)]
pub enum Program{
    Program(Vec<ProgramPart>),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramPart {
    Statement(Box<Statement>),
    Function(Box<Function>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement { 
    // Statement includes Macro, however, macro 
    // requires special management.
    Include(Box<String>), 
    GlobalVariable(Vec<Variable>),
    Struct(Variable),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
    // The last one is for the dimension list, if null than a value only.
    // (identifier, values, dimensions)
    // Variable can be used to declare a variable or reference a variable.
    // Variable can be a single value or an array.
    VarReference(Box<String>, Box<Vec<CompExpr>>),
    VarDeclaration(Box<String>, Box<Vec<Value>>, Box<Vec<CompExpr>>),
    VarAssignment(Box<String>, Box<CompExpr>, Box<Vec<CompExpr>>),

    // Struct is defined to realize object.
    StructReference(Box<String>),
    StructDefinition(Box<String>, Box<Vec<Variable>>),
    // Object type, Identifier, Variables
    StructDeclaration(Box<String>, Box<String>, Box<Vec<Variable>>),
    // Identifier, Field, Variable
    StructAssignment(Box<String>, Box<String>, Box<CompExpr>),

    MemberReference(Box<String>, Box<String>),
    FormalParameter(Box<String>, Box<Vec<Value>>, Box<Vec<usize>>),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    // (identifier, input_params, output_params, body)
    FuncReference(Box<String>, Box<Vec<Box<CompExpr>>>),
    FuncDeclaration(Box<String>, Box<Vec<Variable>>, Box<Value>, Body),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompExpr {
    Value(Value),
    Variable(Variable),
    FuncCall(Function),
    // Mind that UnaryOperator can only operate on 
    // Integer, Float and Bool. (Remember might need
    // to cope with array types)
    UnaryOperation(UnaryOperator, Box<CompExpr>),
    // Binary Operator can operate on all types of Values.
    BinaryOperation(Box<CompExpr>, BinaryOperator, Box<CompExpr>),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum CondExpr {
    Bool(bool),
    UnaryCondition(UnaryOperator, Box<CondExpr>),
    BinaryCondition(Box<CondExpr>, BinaryOperator, Box<CondExpr>),
    Condition(Box<CompExpr>, JudgeOperator, Box<CompExpr>),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(u32),
    Float(f32),
    String(String),
    Char(char),
    Bool(bool),
    Null
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^
    Mod, // %
    And, // && 
    Or,  // ||
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Not, // !
    Inc, // ++
    Dec, // --
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum JudgeOperator {
    GT, // >
    GE, // >=
    LT, // <
    LE, // <=
    EQ, // ==
    NE, // !=
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum If {
    IfExpr(Box<CondExpr>, Body),
    IfElseExpr(Box<CondExpr>, Body, Body),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum Loop {
    WhileExpr(Box<CondExpr>, Body),
    ForExpr(Box<Expr>, Box<CondExpr>, Box<Expr>, Body),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Body(Vec<Expr>),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr{
    If(If),
    Loop(Loop),
    VarManagement(Vec<Variable>),
    FuncCall(Function),
    Break,
    Continue,
    Return(CompExpr),
    Error
}
