use lexer::Token;
use logos::{Logos, Lexer, Source};

fn main(){
    let source = include_str!("./test.spl");
    let mut lex = Token::lexer(source);
    let mut tokens = 0;
    while lex.token != Token::EndOfProgram {
        println!({:?}, lex.token);
        lex.next();
        tokens += 1;
    }
    println!({:?}, tokens)
}