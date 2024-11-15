use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub use grammar::ExprParser;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        // Test if evaluation order is correct
        let source_code = "2 + 4 * 5";
        let lexer = spl_lexer::lexer::Lexer::new(&source_code);
        let parser = ExprParser::new();
        let ast = parser.parse(lexer).unwrap();
        assert_eq!(format!("{}", ast), "(2: i32 + (4: i32 * 5: i32))");

        // Test expression with bracket
        let source_code = "(2 + 4) * 5";
        let lexer = spl_lexer::lexer::Lexer::new(&source_code);
        let parser = ExprParser::new();
        let ast = parser.parse(lexer).unwrap();
        assert_eq!(format!("{}", ast), "((2: i32 + 4: i32) * 5: i32)");
    }


}

