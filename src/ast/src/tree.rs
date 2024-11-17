#[derive(Clone, Debug, PartialEq)]
pub struct Program(pub Vec<ProgramPart>);

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramPart {
    Statement(Box<Statement>),
    Function(Box<Function>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement { 
    // Statement includes Macro, however, macro 
    // requires special management.
    Include(Box<String>), 
    GlobalVariable(Variable),
    Struct(Box<Vec<Variable>>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
    // The last one is for the dimension list, if null than a value only.
    // (identifier, values, dimensions)
    VarReference(Box<String>),
    VarDeclaration(Box<String>, Box<Vec<Value>>, Box<Vec<usize>>),
    FormalParameter(Box<String>, Box<Vec<Value>>, Box<Vec<usize>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    // (identifier, input_params, output_params, body)
    FuncReference(Box<String>, Box<Vec<Box<CompExpr>>>),
    FuncDeclaration(Box<String>, Box<Vec<Variable>>, Box<Value>, Body),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompExpr {
    Value(Value),
    Variable(Variable),
    // Mind that UnaryOperator can only operate on 
    // Integer, Float and Bool. (Remember might need
    // to cope with array types)
    UnaryOperation(UnaryOperator, Box<CompExpr>),
    // Binary Operator can operate on all types of Values.
    BinaryOperation(Box<CompExpr>, BinaryOperator, Box<CompExpr>),
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignExpr {
    // When assigning must check whether the variable already
    // exists 
    AssignOperation(Box<Variable>, Box<CompExpr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CondExpr {
    Bool(bool),
    UnaryCondition(UnaryOperator, Box<CompExpr>),
    Condition(Box<CompExpr>, JudgeOperator, Box<CompExpr>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i32),
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
    Or   // ||
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Not, // !
    Inc, // ++
    Dec  // --
}

#[derive(Clone, Debug, PartialEq)]
pub enum JudgeOperator {
    GT, // >
    GE, // >=
    LT, // <
    LE, // <=
    EQ, // ==
    NE  // !=
}

#[derive(Clone, Debug, PartialEq)]
pub enum If {
    IfExpr(Box<CondExpr>, Body),
    IfElseExpr(Box<CondExpr>, Body, Body)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Loop {
    WhileExpr(Box<CondExpr>, Body),
    ForExpr(Box<Expr>, Box<CondExpr>, Box<Expr>, Body)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Body(Vec<Expr>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr{
    If(If),
    Loop(Loop),
    VarDec(Variable),
    Assign(AssignExpr),
    FuncCall(Function),
    Break,
    Continue,
    Return(CompExpr)
}
