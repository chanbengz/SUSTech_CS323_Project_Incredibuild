use spl_lexer::tokens::Span;

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
    Include(Box<String>, Span), 
    GlobalVariable(Vec<Variable>, Span),
    Struct(Variable, Span),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
    // VarAssignment allows only for VarReference and StructReference.
    VarAssignment(Box<Variable>, Box<CompExpr>),

    // Variable can be a single value or an array.
    // The last one is for the dimension list, if null than a value only.
    // (identifier, dimensions)
    VarReference(Box<String>, Box<Vec<CompExpr>>), // varname, offsets
    VarDeclaration(Box<String>, Box<Value>, Box<Vec<CompExpr>>), // varname, type, offsets
    
    // Struct definition and declaration
    StructDefinition(Box<String>, Box<Vec<Variable>>),
    StructDeclaration(Box<String>, Box<String>, Box<Vec<CompExpr>>),
    // Struct Reference accepts a recursive call of VarReference
    // The first represents the struct variable name and the others are member fields.
    StructReference(Box<Vec<Variable>>),
    
    // Function Parameter
    FormalParameter(Box<String>, Box<Value>, Box<Vec<usize>>),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    // (identifier, input_params, output_params, body)
    FuncReference(Box<String>, Vec<Box<CompExpr>>),
    FuncDeclaration(Box<String>, Vec<Variable>, Box<Value>, Body),
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
    MissingRP,
    Invalid,
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
    Struct(String),
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
    If(If, Span),
    Loop(Loop, Span),
    VarManagement(Vec<Variable>, Span),
    FuncCall(Function, Span),
    Body(Body, Span),
    Break(Span),
    Continue(Span),
    Return(CompExpr, Span),
    Error
}
