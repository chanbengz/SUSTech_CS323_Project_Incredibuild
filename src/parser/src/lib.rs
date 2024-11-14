pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub use grammar::ExprParser;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spl() {
        let source_code = "4 * 5 + 2";
        let lexer = spl_lexer::lexer::Lexer::new(&source_code);
        let parser = ExprParser::new();
        let ast = parser.parse(lexer).unwrap();
        assert_eq!(format!("{}", ast), "((4 * 5) + 2)");
    }
}

