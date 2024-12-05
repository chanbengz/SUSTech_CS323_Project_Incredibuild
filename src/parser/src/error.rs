use spl_lexer::tokens::{Token, LexicalError};
use lalrpop_util::ErrorRecovery;


pub fn display_error(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>, input: &str) {
    let error_str = format_errors(errors, input);
    eprintln!("{}", error_str);
}

pub fn format_errors(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>, input: &str) -> String {
    let mut error_str = Vec::new();
    for error in errors {
        match &error.error {
            lalrpop_util::ParseError::User { error } => {
                match error {
                    LexicalError::MissingLexeme(l, token, _) => {
                        let lineno = input[..*l].lines().count();
                        let lineno = if lineno == 0 { 1 } else { lineno };
                        error_str.push((lineno, format!("Error type B at Line {}: Missing {}\n",
                                                         lineno, token.as_str()).to_owned()));
                    },
                    LexicalError::UnknownLexeme(l, r) => {
                        let lineno = input[..*l].lines().count();
                        error_str.push((lineno, format!("Error type A at Line {}: unknown lexeme {}\n",
                            lineno, input[*l..*r].to_string()).to_owned()));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    error_str.sort_by(|(a, _), (b, _)| a.cmp(b));
    let error_str = error_str.into_iter().map(|(_, s)| s.to_owned()).collect::<String>();
    error_str.trim().to_string()
}