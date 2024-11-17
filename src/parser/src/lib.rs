use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub use grammar::CompExprParser;
pub use grammar::ParaDecsParser;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_compexpr_parse(source: &str, expected: &str) {
        let mut errors = Vec::new();
        let lexer = spl_lexer::lexer::Lexer::new(&source);
        let parser = CompExprParser::new();
        let ast = parser.parse(&mut errors, lexer).unwrap();
        assert_eq!(format!("{}", ast), expected);
    }

    fn assert_paradecs_parse(source: &str, expected: &str) {
        let mut errors = Vec::new();
        let lexer = spl_lexer::lexer::Lexer::new(&source);
        let parser = ParaDecsParser::new();
        let ast = parser.parse(&mut errors, lexer).unwrap();
        assert_eq!(format!("{}", ast.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", ")), expected);
    }

    #[test]
    fn test_expr() {
        // Test if evaluation order is correct
        assert_compexpr_parse("2 + 4 * 5", "(2: i32 + (4: i32 * 5: i32))");
        // Test expression with bracket
        assert_compexpr_parse("(2 + 4) * 5", "((2: i32 + 4: i32) * 5: i32)");
    }

    #[test]
    fn test_error_recovery() {
        assert_compexpr_parse("2 + * 5", "(2: i32 + (MissingTermError * 5: i32))");
        assert_compexpr_parse("2 + 5 *", "(2: i32 + (5: i32 * MissingTermError))");
    }

    #[test]
    fn test_paradeclaration() {
        // Test parameter declaration
        assert_paradecs_parse("int a, int b", "Formal Parameter: a = [0: i32] with dimensions [], Formal Parameter: b = [0: i32] with dimensions []");
        // Test empty parameter declaration
        assert_paradecs_parse("", "");
    }
}

