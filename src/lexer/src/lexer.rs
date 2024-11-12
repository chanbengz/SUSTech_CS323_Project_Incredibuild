use logos::Logos;

use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    InvalidCharacter(String),
    InvalidString(String),
    #[default]
    NonAsciiCharacter,
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
            Empty => LexingError::InvalidInteger("empty string".to_owned()),
            InvalidDigit => LexingError::InvalidInteger("invalid digit".to_owned()),
            _ => LexingError::InvalidInteger("other error".to_owned()),
        }
    }
}

#[derive(Debug, Logos, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[end]
    EndOfProgram,

    // Operators
    #[token(">")]
    OpGreaterThan,
    #[token("<")]
    OpLessThan,
    #[token("<=")]
    OpLessThanEqual,
    #[token(">=")]
    OpGreaterThanEqual,
    #[token("==")]
    OpEqual,
    #[token("!=")]
    OpNotEqual,
    #[token("=")]
    OpAssign,
    #[token("+")]
    OpPlus,
    #[token("-")]
    OpMinus,
    #[token("*")]
    OpMul,
    #[token("/")]
    OpDiv,
    #[token("%")]
    OpMod,
    #[token("^")]
    OpPow,
    #[token("&&")]
    OpAnd,
    #[token("||")]
    OpOr,
    #[token("!")]
    OpNot,
    #[token("++")]
    OpIncrement,
    #[token("--")]
    OpDecrement,

    // Punctuation
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    // Keyword
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("while")]
    KeywordWhile,
    #[token("for")]
    KeywordFor,
    #[token("return")]
    KeywordReturn,
    #[token("break")]
    KeywordBreak,
    #[token("continue")]
    KeywordContinue,

    // Declaration
    #[token("enum")]
    DeclarationEnum,
    #[token("struct")]
    DeclarationStruct,
    #[token("fn")]
    DeclarationFunction,

    // Type
    #[token("bool")]
    TypeBool,
    #[token("char")]
    TypeChar,
    #[token("string")]
    TypeString,
    #[token("int")]
    TypeInt,
    #[token("float")]
    TypeFloat,
    #[token("null")]
    TypeNull,

    // Literals
    #[token("true|false", |lex| lex.slice() == "true", priority = 6)]
    LiteralBool(bool),
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f32>().unwrap(), priority = 6)]
    LiteralFloat(f32),
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    #[regex(r"-?0[xX][0-9a-fA-F]+", |lex| i32::from_str_radix(lex.slice(), 16).unwrap())]
    LiteralInt(i32),
    #[regex(r"'.'", |lex| lex.slice().chars().nth(1).unwrap_or_default())]
    #[regex(r"'\\[xX][0-9a-fA-F]{4}'", |lex| {
        let escape_sequence = &lex.slice()[3..]; // Extract the hex part after \x
        char::from_u32(u32::from_str_radix(escape_sequence, 16).unwrap_or_default()).unwrap_or_default()
    })]
    LiteralChar(char),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    LiteralString(String),

    // Identifier
    #[regex("[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    #[regex("//[^\n]*")]
    LineComment
}