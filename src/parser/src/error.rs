use std::fmt::format;
use spl_lexer::tokens::{Token, LexicalError};
use lalrpop_util::ErrorRecovery;


pub fn display_error(errors: &Vec<ErrorRecovery<usize, Token, LexicalError>>, input: &str) -> String {
    let mut error_str = Vec::new();
    for error in errors {
        match &error.error {
            lalrpop_util::ParseError::UnrecognizedToken { token, expected} => {
                let expected_str = match &expected[0][1..expected[0].len() - 1] {
                    ";" => "semicolon ';'",
                    ")" => "closing parenthesis ')'",
                    _ => expected[0].as_str(),
                };
                let mut last = token.0;
                while last > 0 && (input.as_bytes()[last - 1] == 32 || input.as_bytes()[last - 1] == 9) {
                    last -= 1;
                }
                let lineno = input[..token.0].lines().count() - ((input.as_bytes()[last - 1] == 10) as usize);
                error_str.push((lineno, format!("Error type B at Line {}: Missing {}\n", lineno,
                    expected_str).to_owned()));
            },
            lalrpop_util::ParseError::User { error } => {
                match error {
                    LexicalError::MissingLexeme(l, token, r) => {
                        let mut last = *l;
                        while last > 0 && (input.as_bytes()[last - 1] == 32 || input.as_bytes()[last - 1] == 9) {
                            last -= 1;
                        }
                        let expected_str = match token.as_str() {
                            "';'" => format!("semicolon {}", token),
                            "')'" => format!("closing parenthesis {}", token),
                            "Exp" => format!("{} after {}", token, input[last-1..last].to_owned()),
                            _ => token.to_owned(),
                        };
                        let lineno = input[..*r].lines().count() - ((input.as_bytes()[last - 1] == 10) as usize);
                        error_str.push((lineno, format!("Error type B at Line {}: Missing {}\n",
                                                         lineno, expected_str).to_owned()));
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