use logos::{Logos, FilterResult};

use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    InvalidCharacter(String),
    InvalidString(String),
    UnexpectedEndOfProgram,
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
    #[regex(r"'.'", |lex| lex.slice().chars().nth(1).unwrap_or_default())]
    #[regex(r"'\\u[0-9a-fA-F]{1,6}'", |lex| {
        let slice = lex.slice();
        let hex_part = &slice[3..slice.len() - 1]; // Extract the hex part after \x
        match u32::from_str_radix(hex_part, 16) {
            Ok(u) => {
                match std::char::from_u32(u) {
                    Some(c) => Ok(c),
                    None => return Err(LexingError::NonAsciiCharacter),
                }
            }
            Err(e) => return Err(e.into()),
        }
    })]
    LiteralChar(char),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|\\x[0-9a-fA-F]{2}|\\u[a-fA-F0-9]{1,6})*""#, process_string)]
    LiteralString(String),

    // Identifier
    #[regex(r"[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    #[regex(r"//[^\n]*\n?", logos::skip)]
    LineComment,
    #[token("/*", process_block_comment)]
    BlockComment
}

fn process_string(lex: &mut logos::Lexer<Token>) -> Result<String, LexingError> {
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

fn process_block_comment(lex: &mut logos::Lexer<Token>) -> FilterResult<(), LexingError> {
    if let Some(len) = lex.remainder().find("*/")  {
        lex.bump(len + 2);
        FilterResult::Skip
    } else {
        FilterResult::Error(LexingError::UnexpectedEndOfProgram)
    }
}