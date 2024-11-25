use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub use grammar::CompExprParser;
pub use grammar::CondExprParser;
pub use grammar::ParaDecsParser;
pub use grammar::FuncDecParser;
pub use grammar::BodyParser;
pub use grammar::StmtParser;
pub use grammar::ProgramParser;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    enum Parser {
        CompExprParser,
        CondExprParser,
        ParaDecsParser,
        FuncDecParser,
        StmtParser,
        ProgramParser,
        BodyParser,
    }

    fn assert_parse(parser: Parser, source: &str, expected: &str) {
        let mut errors = Vec::new();
        let lexer = spl_lexer::lexer::Lexer::new(&source);

        match parser {
            Parser::CompExprParser => assert_eq!(format!("{}", CompExprParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::CondExprParser => assert_eq!(format!("{}", CondExprParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::ParaDecsParser => assert_eq!(format!("{}", ParaDecsParser::new().parse(&mut errors, lexer)
                    .unwrap().iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", ")), expected), 
            Parser::FuncDecParser => assert_eq!(format!("{}", FuncDecParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::StmtParser => assert_eq!(format!("{}", StmtParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::ProgramParser => assert_eq!(format!("{}", ProgramParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::BodyParser => assert_eq!(format!("{}", BodyParser::new().parse(&mut errors, lexer).unwrap()), expected),
        }
    }

    fn assert_parse_from_file(parser: Parser, file_path: &str, out_path: &str){
        let mut src_content = String::new();
        let mut src_file = File::open(file_path).expect("Unable to open file");
        src_file.read_to_string(&mut src_content)
            .expect("Unable to read file");

        let mut out_content = String::new();
        let mut out_file = File::open(out_path).expect("Unable to open file");
        out_file.read_to_string(&mut out_content)
            .expect("Unable to read file");
        let expected = out_content.trim();
    
        let mut errors = Vec::new();
        let lexer = spl_lexer::lexer::Lexer::new(&src_content);

        match parser {
            Parser::CompExprParser => assert_eq!(format!("{}", CompExprParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::CondExprParser => assert_eq!(format!("{}", CondExprParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::ParaDecsParser => assert_eq!(format!("{}", ParaDecsParser::new().parse(&mut errors, lexer)
                    .unwrap().iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", ")), expected), 
            Parser::FuncDecParser => assert_eq!(format!("{}", FuncDecParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::StmtParser => assert_eq!(format!("{}", StmtParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::ProgramParser => assert_eq!(format!("{}", ProgramParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::BodyParser => assert_eq!(format!("{}", BodyParser::new().parse(&mut errors, lexer).unwrap()), expected),
        }
    }

    #[test]
    fn test_expr() {
        // Test if evaluation order is correct
        assert_parse(Parser::CompExprParser, "2 + 4 * 5", "(2: i32 + (4: i32 * 5: i32))");
        // Test expression with bracket
        assert_parse(Parser::CompExprParser, "(2 + 4) * 5", "((2: i32 + 4: i32) * 5: i32)");
        // Test conditional expression
        assert_parse(Parser::CondExprParser, "2 > 4", "Condition: 2: i32 > 4: i32"); 
        // Test conbination of condexpr
        assert_parse(Parser::CondExprParser, "true && (5 < 6 || 2 > 5)", "Condition: Condition: true && Condition: Condition: 5: i32 < 6: i32 || Condition: 2: i32 > 5: i32");
    }

    #[test]
    fn test_error_recovery() {
        // Test Comptuation Expression Error Recovery
        assert_parse(Parser::CompExprParser, "2 + * 5", "(2: i32 + ([CompExprError] * 5: i32))");
        assert_parse(Parser::CompExprParser, "2 + * 5 *", "(2: i32 + (([CompExprError] * 5: i32) * [CompExprError]))");
        assert_parse(Parser::CompExprParser, "2 + @", "(2: i32 + [CompExprError])");
        // Test Statement Error Recovery
        assert_parse(Parser::BodyParser, "break; return 0", "Body: [Break, [ExprError]]");
    }

    #[test]
    fn test_paradeclaration() {
        // Test parameter declaration
        assert_parse(Parser::ParaDecsParser, "int a, int b",
        "Formal Parameter: a = [0: i32] with dimensions [], Formal Parameter: b = [0: i32] with dimensions []");
    }

    #[test]
    fn test_func() {
        // Test function declaration
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { return a + b; }", "Function: func:[Body: [Return: (a + b)]]");
    }

    #[test]
    fn test_if() {
        // Test if statement
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { if(a > b) { return a; }}",
            "Function: func:[Body: [If: Condition: a > b then Body: [Return: a]]]");
        // Test if else statement
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { if(a > b) { return a; } else { return b; }}",
            "Function: func:[Body: [If: Condition: a > b then Body: [Return: a] else Body: [Return: b]]]");
    }

    #[test]
    fn test_loop() {
        // Test while loop
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { while(a > b) { a = a - 1; }}", 
        "Function: func:[Body: [While Loop (Condition: a > b): \n do Body: [Variable Assignment: a = (a - 1: i32)]]]");
        // Test for loop
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { for(int i = 0; i < 10; i = i + 1) { a = a + 1; }}", 
        "Function: func:[Body: [For Loop ([Initial] Variable Declaration: i = [0: i32] with dimensions []; Variable Assignment: i = 0: i32; [Condition] Condition: i < 10: i32; [Increment] Variable Assignment: i = (i + 1: i32)): \n do Body: [Variable Assignment: a = (a + 1: i32)]]]");    
        // Test break and continue
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { while(a > b) { if (a == 5) { break; } continue; }}", 
        "Function: func:[Body: [While Loop (Condition: a > b): \n do Body: [If: Condition: a == 5: i32 then Body: [Break], Continue]]]");
    }

    #[test]
    fn test_assignexpr() {
        // Test assignment expression
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { int a = 2; int b = a; b = 3; b++; a--; return b + a; }", 
        "Function: func:[Body: [Variable Declaration: a = [0: i32] with dimensions []; Variable Assignment: a = 2: i32, Variable Declaration: b = [0: i32] with dimensions []; Variable Assignment: b = a, Variable Assignment: b = 3: i32, Variable Assignment: b = (b + 1: i32), Variable Assignment: a = (a - 1: i32), Return: (b + a)]]");
    }

    #[test]
    fn test_func_call() {
        // Test function call
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { add(a, b); return 0; }", 
        "Function: func:[Body: [FuncCall: add[a, b], Return: 0: i32]]");
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { return add(a, b); }",
        "Function: func:[Body: [Return: FuncCall: add[a, b]]]");
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { int k = add(add(a, b), b); return k; }",
        "Function: func:[Body: [Variable Declaration: k = [0: i32] with dimensions []; Variable Assignment: k = FuncCall: add[FuncCall: add[a, b], b], Return: k]]");    
    }

    #[test]
    fn test_stmt(){
        assert_parse(Parser::StmtParser, "int a;", 
        "GlobalVariable: [Variable Declaration: a = [0: i32] with dimensions []]");
        assert_parse(Parser::StmtParser, "int a = 1, b = 2;",
        "GlobalVariable: [Variable Declaration: a = [0: i32] with dimensions [], Variable Assignment: a = 1: i32, Variable Declaration: b = [0: i32] with dimensions [], Variable Assignment: b = 2: i32]");
        assert_parse(Parser::StmtParser, "struct obj { int a; char b; };", 
        "Struct: Struct Definition: obj with [Variable Declaration: a = [0: i32] with dimensions [], Variable Declaration: b = [ : char] with dimensions []]");
        assert_parse(Parser::StmtParser, "#include \"../hi.h\";", "Include: ../hi.h");
    }

    #[test]
    fn test_0_r00() {
        assert_parse_from_file(Parser::FuncDecParser, "../test/test_0_r00.spl","../test/test_0_r00.out");
    }

    #[test]
    fn test_phase1() {
        assert_parse_from_file(Parser::ProgramParser, "../test/phase1/test_1_r01.spl", "../test/phase1/test_1_r01.out");
        assert_parse_from_file(Parser::ProgramParser, "../test/phase1/test_1_r02.spl", "../test/phase1/test_1_r02.out");
        assert_parse_from_file(Parser::ProgramParser, "../test/phase1/test_1_r03.spl", "../test/phase1/test_1_r03.out");
        assert_parse_from_file(Parser::ProgramParser, "../test/phase1/test_1_r04.spl", "../test/phase1/test_1_r04.out");
    }

    // #[test]
    // fn test_phase2() {
    // }

    // #[test]
    // fn test_phase3() {
    // }
}
