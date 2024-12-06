use colored::Colorize;
use spl_lexer::tokens::{Token, LexicalError};
use lalrpop_util::ErrorRecovery;


pub fn display_error(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>, input: &str, source_path: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    for error in errors {
        let error = &error.error;
        if let lalrpop_util::ParseError::User { error } = error { match error {
            LexicalError::MissingLexeme(span, token) => {
                let lineno = input[..span.start].lines().count();
                let lineno = if lineno == 0 { 1 } else { lineno };
                let begin = input[..span.start].rfind('\n').unwrap_or(0);
                line_error(
                    (span.start - begin, span.end - begin), lineno,
                    lines[lineno - 1],
                    &format!("{} missing {} [{}]",
                        "error:".red(), token.as_str(), "B".red()
                    ),
                    source_path
                );
            },
            LexicalError::UnknownLexeme(span) => {
                let lineno = input[..span.start].lines().count();
                let lineno = if lineno == 0 { 1 } else { lineno };
                let begin = input[..span.start].rfind('\n').unwrap_or(0);
                line_error(
                    (span.start - begin, span.end - begin), lineno,
                    lines[lineno - 1],
                    &format!("{} unknown lexeme [{}]",
                         "error:".red(), "A".red()
                    ),
                    source_path,
                );
            }, _ => {} }
        }
    }
}

fn line_error( span: (usize, usize), lineno: usize, line_str: &str, error_msg: &str, source_path: &str) {
    let lineno = lineno.to_string();
    let padding = " ".repeat(lineno.len() + 1);
    let padding_msg = " ".repeat(span.0);
    let mut indicator = "^".to_string();
    indicator.push_str(&"~".repeat(span.1 - span.0 - 1));
    println!("{} {}:{lineno}:{}: {error_msg}\n{padding}{}\n{} {} {line_str}\n\
        {padding}{}{padding_msg}{}",
        "-->".purple(), source_path, span.0, "|".purple(), lineno.purple(), "|".purple(),
        "|".purple(), indicator.red()
    )
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