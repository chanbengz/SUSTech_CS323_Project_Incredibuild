/* 
    Supported Tokens:

    - EndOfProgram
    - Operators: >, <, <=, >=, ==, !=, =, +, -, *, /, %, ^, &&, ||, !, ++, -- (Start with Op)
    - Punctuation: ., ,, :, ;, [, ], (, ), {, } 
    - Keywords: if, else, while, for, return, break, continue (Start with Keyword)
    - Declaration: enum, struct, fn (Start with Declaration)
    - Type: bool, char, string, int, float, null (Start with Type)
    - Literals: bool, float, int, char, string (Start with Literal)
    - Identifier
    - LineComment: //...
    - BlockComment: /*...*/

*/

use logos::{Logos, FilterResult};
use core::error;
use std::fmt; 
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(String),
    InvalidCharacter(String),
    InvalidString(String),
    UnexpectedEndOfProgram,
    NonAsciiCharacter,
    #[default]
    UnknownToken
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexicalError::InvalidInteger("[ParseIntError] Overflow error".to_owned()),
            Empty => LexicalError::InvalidInteger("[ParseIntError] Cannot parse integer from empty string".to_owned()),
            InvalidDigit => LexicalError::InvalidInteger("[ParseIntError] Invalid digit found in the string".to_owned()),
            Zero => LexicalError::InvalidInteger("[ParseIntError] Number would be zero for non-zero type".to_owned()),
            _ => LexicalError::InvalidInteger("[ParseIntError] Other error".to_owned()),
        }
    }
}

#[derive(Debug, Logos, PartialEq, Clone)]
#[logos(error = LexicalError)]
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
    #[token(".", priority = 5)]
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
    #[token("#include")]
    DeclarationInclude,

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
    #[token("void")]
    TypeVoid,

    // Literals
    #[regex("true|false", |lex| lex.slice() == "true")]
    LiteralBool(bool),
    #[regex(r"-?(?:0|[1-9]\d*)?\.\d+(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f32>().unwrap(), priority = 10)]
    LiteralFloat(f32),
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    #[regex(r"-?0[xX][0-9a-fA-F]+", |lex| {
        if lex.slice().chars().nth(0) == Some('-') {
            let hex = &lex.slice()[3..]; // Extract the hex part after -0x
            -i32::from_str_radix(hex, 16).unwrap()
        } else {
            let hex = &lex.slice()[2..]; // Extract the hex part after 0x
            i32::from_str_radix(hex, 16).unwrap()
        }
    })]
    LiteralInt(i32),
    #[regex(r"'.'", |lex| {
        let slice = lex.slice();
        let c = slice.chars().nth(1).unwrap();
        if c.is_ascii() {
            Ok(c)
        } else {
            Err(LexicalError::NonAsciiCharacter)
        }
    })]
    #[regex(r"'\\u[0-9a-fA-F]{1,6}'", |lex| {
        let slice = lex.slice();
        let hex_part = &slice[3..slice.len() - 1]; // Extract the hex part after \x
        match u32::from_str_radix(hex_part, 16) {
            Ok(u) => {
                match std::char::from_u32(u) {
                    Some(c) => Ok(c),
                    None => return Err(LexicalError::InvalidCharacter(format!("Invalid Unicode character: {}", u))),
                }
            }
            Err(e) => return Err(LexicalError::InvalidCharacter(format!("{:?}", e))),
        }
    })]
    LiteralChar(char),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|\\x[0-9a-fA-F]{2}|\\u[a-fA-F0-9]{1,6})*""#, process_string)]
    LiteralString(String),

    // Identifier
    #[regex(r"[a-zA-Z_$][a-zA-Z0-9_$]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex(r"//[^\n]*\n?", logos::skip)]
    LineComment,
    #[token("/*", process_block_comment)]
    BlockComment
}

fn process_string(lex: &mut logos::Lexer<Token>) -> Result<String, LexicalError> {
    let slice = lex.slice();
    let mut chars = slice.chars().skip(1).take(slice.len() - 2);
    let mut result = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                match chars.next() {
                    Some('x') => {
                        // Parse hexadecimal byte (\xNN)
                        let hex = chars.by_ref().take(2).collect::<String>();
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            result.push(byte as char);
                        }
                    }
                    Some('u') => {
                        // Parse Unicode escape (\u{NNNN})
                        if chars.next() == Some('{') {
                            let unicode = chars.by_ref().take_while(|&c| c != '}').collect::<String>();
                            if let Ok(value) = u32::from_str_radix(&unicode, 16) {
                                if let Some(ch) = char::from_u32(value) {
                                    result.push(ch);
                                }
                            }
                        }
                    }
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some('b') => result.push('\x08'), // Backspace
                    Some('f') => result.push('\x0C'), // Form feed
                    Some(other) => result.push(other), // Any other escaped character
                    None => break, // End of string
                }
            }
            c => result.push(c),
        }
    }
    Ok(result)
}

fn process_block_comment(lex: &mut logos::Lexer<Token>) -> FilterResult<(), LexicalError> {
    if let Some(len) = lex.remainder().find("*/")  {
        lex.bump(len + 2);
        FilterResult::Skip
    } else {
        FilterResult::Error(LexicalError::UnexpectedEndOfProgram)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::InvalidInteger(s) => write!(f, "{}", s),
            LexicalError::InvalidCharacter(s) => write!(f, "{}", s),
            LexicalError::InvalidString(s) => write!(f, "{}", s),
            LexicalError::UnexpectedEndOfProgram => write!(f, "Unexpected end of program"),
            LexicalError::UnknownToken => write!(f, "Unknown token"),
            LexicalError::NonAsciiCharacter => write!(f, "Non-ASCII character"),
        }
    }
}