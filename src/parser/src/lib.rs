use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub use grammar::CompExprParser;
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
        ParaDecsParser,
        FuncDecParser,
        StmtParser,
        ProgramParser,
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
            Parser::StmtParser => assert_eq!(format!("{}", StmtParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::ProgramParser => assert_eq!(format!("{}", ProgramParser::new().parse(&mut errors, lexer).unwrap()), expected),
            Parser::BodyParser => assert_eq!(format!("{}", BodyParser::new().parse(&mut errors, lexer).unwrap()), expected),
        }
    }

    fn assert_parse_from_file(parser: Parser, file_path: &str, expected: &str){
        let mut file_content = String::new();
        let mut file = File::open(file_path).expect("Unable to open file");
        file.read_to_string(&mut file_content)
            .expect("Unable to read file");
    
        let mut errors = Vec::new();
        let lexer = spl_lexer::lexer::Lexer::new(&file_content);

        match parser {
            Parser::CompExprParser => assert_eq!(format!("{}", CompExprParser::new().parse(&mut errors, lexer).unwrap()), expected),
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
    }

    #[test]
    fn test_error_recovery() {
        assert_parse(Parser::CompExprParser, "2 + * 5", "(2: i32 + ([CompExprError] * 5: i32))");
        assert_parse(Parser::CompExprParser, "2 + * 5 *", "(2: i32 + (([CompExprError] * 5: i32) * [CompExprError]))");
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
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { if(a > b) { return a; }}", "Function: func:[Body: [If: Condition: a > b then Body: [Return: a]]]");
        // Test if else statement
        assert_parse(Parser::FuncDecParser, "int func(int a, int b) { if(a > b) { return a; } else { return b; }}", "Function: func:[Body: [If: Condition: a > b then Body: [Return: a] else Body: [Return: b]]]");
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
        assert_parse(Parser::StmtParser, "struct obj { int a; char b; };", 
        "Struct: Struct Definition: obj with [Variable Declaration: a = [0: i32] with dimensions [], Variable Declaration: b = [ : char] with dimensions []]");
        assert_parse(Parser::StmtParser, "#include \"../hi.h\";", "Include: ../hi.h");
    }

    #[test]
    fn test_0_r00() {
        assert_parse_from_file(Parser::FuncDecParser, "../test/test_0_r00.spl", 
        "Function: main:[Body: [Variable Declaration: a = [0: i32] with dimensions []; Variable Assignment: a = 3: i32, While Loop (Condition: true): \n do Body: [Variable Assignment: a = (a + 1: i32), If: Condition: a == 5: i32 then Body: [Break]], Return: null]]");
    }

    #[test]
    fn test_1_r01() {
        assert_parse_from_file(Parser::FuncDecParser, "../test/test_1_r01.spl", 
        "Function: test_1_r01:[Body: [Variable Assignment: c = c: char, If: Condition: a > b then Body: [Return: a] else Body: [Return: b]]]");
    }

    #[test]
    fn test_1_r02() {
        assert_parse_from_file(Parser::ProgramParser, "../test/test_1_r02.spl", 
        "Statement: GlobalVariable: [Variable Declaration: global = [0: i32] with dimensions []], Statement: Struct: Struct Definition: my_struct with [Variable Declaration: code = [0: i32] with dimensions [], Variable Declaration: data = [ : char] with dimensions []], Functions: Function: test_1_r02:[Body: [Struct Declaration: my_struct extends obj with [], Struct Assignment: obj.code = global, Variable Assignment: global = (global + 1: i32)]]"
    }

    // #[test]
    // fn test_1_s01() {
    //     assert_parse_from_file(Parser::FuncDecParser, "../test/test_1_s01.spl", 
    //     "Function: test_1_r03:[Body: [Assignment: c = c: char, If: Condition: a > b then Body: [Return: a] else Body: [Return: b]]]");
    // }

    // #[test]
    // fn test_1_s03() {
    //     assert_parse_from_file(Parser::FuncDecParser, "../test/test_1_s03.spl", 
    //     "Function: test_1_r03:[Body: [Assignment: c = c: char, If: Condition: a > b then Body: [Return: a] else Body: [Return: b]]]");
    // }

    // #[test]
    // fn test_1_s07() {
    //     assert_parse_from_file(Parser::FuncDecParser, "../test/test_1_s07.spl", 
    //     "Function: test_1_r04:[Body: [Assignment: c = c: char, If: Condition: a > b then Body: [Return: a] else Body: [Return: b]]]");
    // }

    // #[test]
    // fn test_1_s09() {
    //     assert_parse_from_file(Parser::FuncDecParser, "../test/test_1_s09.spl", 
    //     "Function: test_1_r08:[Body: [Assignment: c = c: char, If: Condition: a > b then Body: [Return: a] else Body: [Return: b]]]");
    // }
}

