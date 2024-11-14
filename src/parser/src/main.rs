use spl_lexer::lexer::Lexer;
use spl_parser::grammar::ExprParser;

fn main() {
    let source_code = "
4 * 5 + 2
";

    let lexer = Lexer::new(&source_code);
    let parser = ExprParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{}", ast);
}
