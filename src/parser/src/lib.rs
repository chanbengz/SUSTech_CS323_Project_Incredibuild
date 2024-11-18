use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub use grammar::CompExprParser;
pub use grammar::ParaDecsParser;
pub use grammar::FuncDecParser;
pub use grammar::BodyParser;

#[cfg(test)]
mod tests {
    use super::*;

    enum Parser {
        CompExprParser,
        ParaDecsParser,
        FuncDecParser,
        #[allow(dead_code)]
        BodyParser,
    }

    fn assert_parse(parser: Parser, source: &str, expected: &str) {
        let mut errors = Vec::new();
        let lexer = spl_lexer::lexer::Lexer::new(&source);

        match parser {
            Parser::CompExprParser => assert_eq!(format!("{}", CompExprParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::ParaDecsParser => assert_eq!(format!("{}", ParaDecsParser::new().parse(&mut errors, lexer)
                    .unwrap().iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", ")), expected), 
            Parser::FuncDecParser => assert_eq!(format!("{}", FuncDecParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::BodyParser => assert_eq!(format!("{}", BodyParser::new().parse(&mut errors, lexer).unwrap()), expected),
        }
    }

    #[test]
    fn test_expr() {
        // Test if evaluation order is correct
        assert_parse(Parser::CompExprParser, "2 + 4 * 5", "(2: i32 + (4: i32 * 5: i32))");
        // Test expression with bracket
        assert_parse(Parser::CompExprParser, "(2 + 4) * 5", "((2: i32 + 4: i32) * 5: i32)");
    }

    #[test]
    fn test_error_recovery() {
        assert_parse(Parser::CompExprParser, "2 + * 5", "(2: i32 + (MissingTermError * 5: i32))");
        assert_parse(Parser::CompExprParser, "2 + * 5 *", "(2: i32 + ((MissingTermError * 5: i32) * MissingTermError))");
    }

    #[test]
    fn test_paradeclaration() {
        // Test parameter declaration
        assert_parse(Parser::ParaDecsParser, "int a, int b", "Formal Parameter: a = [0: i32] with dimensions [], Formal Parameter: b = [0: i32] with dimensions []");
        // Test empty parameter declaration
        assert_parse(Parser::ParaDecsParser, "", "");
    }

    #[test]
    fn test_func() {
        // Test function declaration
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { return a + b; }", "Function: func:[Body: [Return: (a + b)]]");
    }

    #[test]
    fn test_if() {
        // Test if statement
        assert_parse(Parser::FuncDecParser, "
            int main(){
                int a = 3;
                while (true){
                    a = a + 1;
                    if (a == 5){
                        break;
                    }
                }
                return;
            }
        ", "Function: main:[Body: [Assignment: Variable Declaration: a = [0: i32] with dimensions [] = 3: i32, While Loop (Condition: true): \n do Body: [Assignment: a = (a + 1: i32), If: Condition: a == 5: i32 then Body: [Break]], Return: null]]");
    }
}

