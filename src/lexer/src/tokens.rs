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
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub source: String,
    pub start: usize,
    pub end: usize
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(String),
    InvalidCharacter(String),
    InvalidString(String),
    UnexpectedEndOfProgram,
    NonAsciiCharacter,
    // error for parser
    MissingLexeme(Span, String),
    StatementError(Span, String),
    UnknownLexeme(Span),
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
    #[regex(r"(?:0|[1-9]\d*)?\.\d+(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f32>().unwrap(), priority = 10)]
    LiteralFloat(f32),
    #[regex(r"(0|[1-9][0-9]*)", |lex| lex.slice().parse::<u32>().unwrap())]
    #[regex(r"0[xX][0-9a-fA-F]+", process_hex)]
    LiteralInt(u32),

    #[token("'", process_char)]
    LiteralChar(char),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|\\x[0-9a-fA-F]{2}|\\u[a-fA-F0-9]{1,6})*""#, process_string)]
    LiteralString(String),

    // Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex(r"//[^\n]*\n?", logos::skip)]
    LineComment,
    #[token("/*", process_block_comment)]
    BlockComment,

    // error handling
    #[regex(r"((0|[1-9][0-9]*)[a-zA-Z_][a-zA-Z0-9_]*)|(0[0-9]+(\.[0-9]+)?)")]
    Invalid,
    Error
}

fn process_char(lex: &mut logos::Lexer<Token>) -> Result<char, LexicalError> {
    if let Some(len) = lex.remainder().find("'")  {
        lex.bump(len + 1);
        let slice = &lex.slice()[1..len + 1];
        if len == 1 {
            Ok(slice.chars().next().unwrap())
        } else if &slice[..2] == "\\x" {
            match u8::from_str_radix(&slice[2..], 16) {
                Ok(byte) => Ok(byte as char),
                Err(_) => Err(LexicalError::InvalidCharacter(format!("Invalid hexadecimal character: {}", slice))),
            }
        } else {
            Err(LexicalError::InvalidCharacter(format!("Invalid character: {}", slice)))
        }
    } else {
        Err(LexicalError::UnexpectedEndOfProgram)
    }
}

fn process_hex(lex: &mut logos::Lexer<Token>) -> Result<u32, LexicalError> {
    let slice = lex.slice();
    let hex = &slice[2..];
    u32::from_str_radix(hex, 16).map_err(|_| LexicalError::InvalidInteger(format!("Invalid hexadecimal number: {}", hex)))
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
        match self {
            Token::OpGreaterThan => write!(f, ">"),
            Token::OpLessThan => write!(f, "<"),
            Token::OpLessThanEqual => write!(f, "<="),
            Token::OpGreaterThanEqual => write!(f, ">="),
            Token::OpEqual => write!(f, "=="),
            Token::OpNotEqual => write!(f, "!="),
            Token::OpAssign => write!(f, "="),
            Token::OpPlus => write!(f, "+"),
            Token::OpMinus => write!(f, "-"),
            Token::OpMul => write!(f, "*"),
            Token::OpDiv => write!(f, "/"),
            Token::OpMod => write!(f, "%"),
            Token::OpPow => write!(f, "^"),
            Token::OpAnd => write!(f, "&&"),
            Token::OpOr => write!(f, "||"),
            Token::OpNot => write!(f, "!"),
            Token::OpIncrement => write!(f, "++"),
            Token::OpDecrement => write!(f, "--"),
            _ => write!(f, "{:?}", self)
        }
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
            LexicalError::MissingLexeme(span, token) => {
                let mut input = String::new();
                File::open(&span.source).expect("File not found").read_to_string(&mut input).expect("Error reading file");
                let lineno = input[..span.start].lines().count();
                let lineno = if lineno == 0 { 1 } else { lineno };
                write!(f, "Error type B at Line {}: Missing {}", lineno, token.as_str().to_owned())
            },
            LexicalError::UnknownLexeme(span) => {
                let mut input = String::new();
                File::open(&span.source).expect("File not found").read_to_string(&mut input).expect("Error reading file");
                let lineno = input[..span.start].lines().count();
                write!(f, "Error type A at Line {}: unknown lexeme {}",
                    lineno, input[span.start..span.end].to_string().to_owned())
            },
            LexicalError::StatementError(span, msg) => {
                let mut input = String::new();
                File::open(&span.source).expect("File not found").read_to_string(&mut input).expect("Error reading file");
                let lineno = input[..span.start].lines().count();
                write!(f, "Error type B at Line {}: {}", lineno, msg.as_str().to_owned())
            }
        }
    }
}
