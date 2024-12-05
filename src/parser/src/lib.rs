use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
use spl_ast::tree;
pub use crate::error::display_error;
use crate::grammar::ProgramParser;

pub mod error;

pub fn parse(source: &str) -> Result<tree::Program, String> {
    let mut errors = Vec::new();
    let lexer = spl_lexer::lexer::Lexer::new(&source);
    let result = ProgramParser::new().parse(&mut errors, lexer).unwrap();
    if errors.len() > 0 {
        display_error(&errors, &source);
        Err(format!("Syntax Error: {}", errors.len()))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::error::format_errors;
    use crate::grammar::CompExprParser;
    use crate::grammar::CondExprParser;
    use crate::grammar::ParaDecsParser;
    use crate::grammar::FuncDecParser;
    use crate::grammar::BodyParser;
    use crate::grammar::StmtParser;
    use crate::grammar::ProgramParser;

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
            Parser::BodyParser => assert_eq!(format!("{}", BodyParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::ProgramParser => {
                let result = ProgramParser::new().parse(&mut errors, lexer).unwrap();
                if errors.len() > 0 {
                    let error_str = format_errors(&errors, &src_content);
                    // assert_eq!(error_str, expected)
                    match (&error_str, &expected) {
                        (error_str, expected) => {
                            let error_str = error_str.split("\n").collect::<Vec<&str>>();
                            let expected = expected.split("\n").collect::<Vec<&str>>();
                            for i in 0..error_str.len() {
                                if *error_str[i] != *expected[i] {
                                    println!("Error:    {}", error_str[i]);
                                    println!("Expected: {}", expected[i]);
                                    println!("Error Recovery: {:?})", errors[i]);
                                    assert_eq!(error_str[i], expected[i]);
                                }
                            }
                        }
                    }
                } else {
                    assert_eq!(format!("{}", result), expected)
                }
            }
        }
    }

    #[test]
    fn test_expr() {
        // Test if evaluation order is correct
        assert_parse(Parser::CompExprParser, "2 + 4 * 5", "(2: u32 + (4: u32 * 5: u32))");
        // Test expression with bracket
        assert_parse(Parser::CompExprParser, "(2 + 4) * 5", "((2: u32 + 4: u32) * 5: u32)");
        // Test conditional expression
        assert_parse(Parser::CondExprParser, "2 > 4", "Condition: 2: u32 > 4: u32"); 
        // Test conbination of condexpr
        assert_parse(Parser::CondExprParser, "true && (5 < 6 || 2 > 5)",
            "Condition: Condition: true && Condition: Condition: 5: u32 < 6: u32 || Condition: 2: u32 > 5: u32");
    }

    #[test]
    fn test_error_recovery() {
        // Test Comptuation Expression Error Recovery
        assert_parse(Parser::CompExprParser, "2 + * 5", "(2: u32 + ([CompExprError] * 5: u32))");
        assert_parse(Parser::CompExprParser, "2 + * 5 *", "(2: u32 + (([CompExprError] * 5: u32) * [CompExprError]))");
        assert_parse(Parser::CompExprParser, "2 + @", "(2: u32 + [CompExprError])");
        // Test Statement Error Recovery
        assert_parse(Parser::BodyParser, "break; return 0", "Body: [Break, [ExprError]]");
    }

    #[test]
    fn test_paradeclaration() {
        // Test parameter declaration
        assert_parse(Parser::ParaDecsParser, "int a, int b",
        "Formal Parameter: a = [0: u32] with dimensions [], Formal Parameter: b = [0: u32] with dimensions []");
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
        "Function: func:[Body: [While Loop (Condition: a > b):\ndo Body: [Variable Assignment: a = (a - 1: u32)]]]");
        // Test for loop
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { for(int i = 0; i < 10; i = i + 1) { a = a + 1; }}", 
        "Function: func:[Body: [For Loop ([Initial] Variable Declaration: i = [0: u32] with dimensions []; Variable Assignment: i = 0: u32; [Condition] Condition: i < 10: u32; [Increment] Variable Assignment: i = (i + 1: u32)): \n do Body: [Variable Assignment: a = (a + 1: u32)]]]");    
        // Test break and continue
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { while(a > b) { if (a == 5) { break; } continue; }}",
        "Function: func:[Body: [While Loop (Condition: a > b):\ndo Body: [If: Condition: a == 5: u32 then Body: [Break], Continue]]]");
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { int c = 0; {int d = c;} }",
            "Function: func:[Body: [Variable Declaration: c = [0: u32] with dimensions []; Variable Assignment: c = 0: u32, \nNested Body: [Variable Declaration: d = [0: u32] with dimensions []; Variable Assignment: d = c]]]")
    }

    #[test]
    fn test_assignexpr() {
        // Test assignment expression
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { int a = 2; int b = a; b = 3; b++; a--; return b + a; }", 
        "Function: func:[Body: [Variable Declaration: a = [0: u32] with dimensions []; Variable Assignment: a = 2: u32, Variable Declaration: b = [0: u32] with dimensions []; Variable Assignment: b = a, Variable Assignment: b = 3: u32, Variable Assignment: b = (b + 1: u32), Variable Assignment: a = (a - 1: u32), Return: (b + a)]]");
    }

    #[test]
    fn test_func_call() {
        // Test function call
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { add(a, b); return 0; }", 
        "Function: func:[Body: [FuncCall: add[a, b], Return: 0: u32]]");
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { return add(a, b); }",
        "Function: func:[Body: [Return: FuncCall: add[a, b]]]");
        // Test nested function call
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { int k = add(add(a, b), b); return k; }",
        "Function: func:[Body: [Variable Declaration: k = [0: u32] with dimensions []; Variable Assignment: k = FuncCall: add[FuncCall: add[a, b], b], Return: k]]");    
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { add(add(a + 1, b) * 2, b); return k; }",
        "Function: func:[Body: [FuncCall: add[(FuncCall: add[(a + 1: u32), b] * 2: u32), b], Return: k]]");
    }

    #[test]
    fn test_stmt(){
        assert_parse(Parser::StmtParser, "int a;", 
        "GlobalVariable: [Variable Declaration: a = [0: u32] with dimensions []]");
        assert_parse(Parser::StmtParser, "int a = 1, b = 2;",
        "GlobalVariable: [Variable Declaration: a = [0: u32] with dimensions [], Variable Assignment: a = 1: u32, Variable Declaration: b = [0: u32] with dimensions [], Variable Assignment: b = 2: u32]");
        assert_parse(Parser::StmtParser, "struct obj { int a; char b; };", 
        "Struct: Struct Definition: obj with [Variable Declaration: a = [0: u32] with dimensions [], Variable Declaration: b = [ : char] with dimensions []]");
        assert_parse(Parser::StmtParser, "#include \"../hi.h\"", "Include: ../hi.h");
        assert_parse(Parser::StmtParser, "int a[1];", "GlobalVariable: [Variable Declaration: a = [0: u32] with dimensions [1: u32]]");
    }

    #[test]
    fn test_0_r00() {
        assert_parse_from_file(Parser::FuncDecParser, "../test/test_0_r00.spl","../test/test_0_r00.out");
    }

    #[test]
    fn test_phase1() {
        for i in 1..13 {
            assert_parse_from_file(Parser::ProgramParser, 
                &format!("../test/phase1/test_1_r{:0>2}.spl", i), 
                &format!("../test/phase1/test_1_r{:0>2}.out", i)
            );
        }
        for i in 1..7 {
           assert_parse_from_file(Parser::ProgramParser,
               &format!("../test/phase1/extra/test_1_s{:0>2}.spl", i),
               &format!("../test/phase1/extra/test_1_s{:0>2}.out", i)
           );
        }
    }
}
