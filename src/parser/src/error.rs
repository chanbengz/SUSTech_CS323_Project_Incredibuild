use spl_lexer::tokens::{Token, LexicalError};
use lalrpop_util::ErrorRecovery;


pub fn display_error(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>, input: &str) -> String {
    let mut error_str = String::new();
    for error in errors {
        match &error.error {
            lalrpop_util::ParseError::UnrecognizedToken { expected, .. } => {
                let expected_str = &expected[0][1..expected[0].len() - 1];
                error_str.push_str(&format!("Missing token {:?}\n",
                    expected_str));
            },
            lalrpop_util::ParseError::User { error } => {
                error_str.push_str(&format!("{}\n", error));
            },
            _ => {}
        }
    }

    error_str.trim().to_string()
}
