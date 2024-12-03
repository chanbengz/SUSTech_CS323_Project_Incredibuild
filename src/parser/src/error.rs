use spl_lexer::tokens::{Token, LexicalError};
use lalrpop_util::ErrorRecovery;


pub fn display_error(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>, input: &str) -> String {
    let mut error_str = Vec::new();
    for error in errors {
        match &error.error {
            lalrpop_util::ParseError::UnrecognizedToken { token, expected, .. } => {
                let expected_str = match &expected[0][1..expected[0].len() - 1] {
                    ";" => "semicolon ';'",
                    ")" => "closing parenthesis ')'",
                    _ => expected[0].as_str(),
                };
                let lineno = input[..token.0].lines().count() - ((input.as_bytes()[token.0 - 1] == 32) as usize);
                error_str.push((lineno, format!("Error type B at Line {}: Missing {}\n", lineno,
                    expected_str).to_owned()));
            },
            lalrpop_util::ParseError::User { error } => {
                match error {
                    LexicalError::MissingLexeme(l, token, _) => {
                        let lineno = input[..*l].lines().count();
                        error_str.push((lineno, format!("Error type B at Line {}: Missing {}\n",
                                                         lineno, token).to_owned()));
                    },
                    LexicalError::UnknownLexeme(l, _) => {
                        let lineno = input[..*l].lines().count();
                        error_str.push((lineno, format!("Error type A at Line {}: unknown lexeme {}\n",
                            lineno, input.chars().nth(*l).unwrap())));
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