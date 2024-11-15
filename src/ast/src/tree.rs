#[derive(Clone, Debug, PartialEq)]
pub enum Program {
    Statements(Box<Vec<Statement>>),
    Functions(Box<Vec<Function>>)
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
    VarDeclaration(Box<String>, Box<Vec<Value>>, Box<Vec<usize>>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    // (identifier, input_params, params_num)
    FuncDeclaration(Box<String>, Box<Vec<Variable>>, Box<usize>)
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignExpr {
    // When assigning must check whether the variable already
    // exists 
    AssignOperation(Variable, CompExpr)
}

#[derive(Clone, Debug, PartialEq)]
pub enum CondExpr {
    Bool(bool),
    Condition(Box<Expr>, JudgeOperator, Box<Expr>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
    Char(char),
    Bool(bool),
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
    IfExpr(CondExpr, Body, Option<Body>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Loop {
    LoopExpr(CondExpr, Body)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    Body(Box<Vec<Expr>>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr{
    If(If),
    Loop(Loop),
    Assign(AssignExpr),
    Break(),
    Continue,
    Return(Option<Value>)
}
