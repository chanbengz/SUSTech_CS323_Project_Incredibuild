use std::fs::File;
use std::io::Read;
use colored::Colorize;
use spl_lexer::tokens::{Token, LexicalError};
use lalrpop_util::ErrorRecovery;

pub trait EmitError {
    fn error(&self);
}

impl EmitError for LexicalError {
    fn error(&self) {
        let mut input = String::new();
        let (span, error_msg) = match self {
            LexicalError::UnknownLexeme(span) => {
                (span, format!("{} unknown lexeme [{}]", "error:".red(), "A".red()))
            },
            LexicalError::MissingLexeme(span, token) => {
                (span, format!("{} missing {} [{}]", "error:".red(), token.as_str(), "B".red()))
            },
            _ => { return; }
        };

        File::open(&span.source).unwrap().read_to_string(&mut input).unwrap();
        let lineno = input[..span.start].lines().count();
        let lineno = if lineno == 0 { 1.to_string() } else { lineno.to_string() };
        let begin = input[..span.start].rfind('\n').unwrap_or(0);
        let line_str = input.lines().nth(lineno.parse::<usize>().unwrap() - 1).unwrap();
        let padding = " ".repeat(lineno.len() + 1);
        let padding_msg = " ".repeat(span.start - begin);
        let bar = "|".purple();

        let mut indicator = "^".to_string();
        indicator.push_str(&"~".repeat(span.end - span.start - 1));
        println!("{} {}:{lineno}:{}: {error_msg}\n{padding}{}\n{} {} {line_str}\n{padding}{}{padding_msg}{}",
                 "-->".purple(), span.source, span.start, &bar, lineno.purple(), &bar,
                 bar, indicator.red());
    }
}

pub fn emit_error(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>) {
    for error in errors {
        let error = &error.error;
        if let lalrpop_util::ParseError::User { error } = error {
            error.error();
        }
    }
}

pub fn format_errors(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>)
    -> Vec<String> {
    let mut error_str = Vec::new();
    for error in errors {
        let error = &error.error;
        if let lalrpop_util::ParseError::User { error } = error {
            error_str.push(format!("{}", error));
        }
    }

    error_str.sort_by(|a, b| {
        let re = regex::Regex::new(r"\d+").unwrap();
        let a_lineno = re.find(&a).unwrap().as_str().parse::<usize>().unwrap();
        let b_lineno = re.find(&b).unwrap().as_str().parse::<usize>().unwrap();
        a_lineno.cmp(&b_lineno)
    });
    error_str
}