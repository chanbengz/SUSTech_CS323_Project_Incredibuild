extern crate logos;

pub mod tokens;
pub mod lexer;
pub use tokens::Token;
pub use logos::{Logos, Lexer, Source};
pub use std::fs::File;


#[cfg(test)]
mod test {
    use crate::tokens::Token;
    use crate::tokens::Token::*;
    use logos::Logos;
    use std::fs::File;
    use std::io::Read;

    fn assert_lex<T>(source: &str, tokens: T)
    where
        T: AsRef<[(Token, &'static str)]>
    {
        let mut lexer = Token::lexer(source); 
        for &(ref token, slice) in tokens.as_ref() {
            if let Some(lexed_token) = lexer.next() {
                assert!(
                    lexed_token == Ok(token.clone()) && lexer.slice() == slice,
                    "\n\n\n\tExpected {:?}({:?}), found {:?}({:?}) instead!\n\n\n",
                    token,
                    slice,
                    lexed_token,
                    lexer.slice()
                );
            } else {
               panic!("Unexpected end of tokens while lexing.");
            }
        }

        assert_eq!(lexer.next(), None);
    }

    fn assert_lex_from_file<T>(file_path: &str, tokens: T) 
    where
        T: AsRef<[(Token, &'static str)]>
    {
        let mut file_content = String::new();
        let mut file = File::open(file_path).expect("Unable to open file");
        file.read_to_string(&mut file_content)
            .expect("Unable to read file");
    
        let mut lexer = Token::lexer(&file_content);
    
        for &(ref token, slice) in tokens.as_ref() {
            if let Some(lexed_token) = lexer.next() {
                assert!(
                    lexed_token == Ok(token.clone()) && lexer.slice() == slice,
                    "\n\n\n\tExpected {:?}({:?}), found {:?}({:?}) instead!\n\n\n",
                    token,
                    slice,
                    lexed_token,
                    lexer.slice()
                );
            } else {
               panic!("Unexpected end of tokens while lexing.");
            }
        }
    
        assert_eq!(lexer.next(), None);
    }


    #[test]
    fn empty_lexer() {
        assert_lex("   ", []);
    }

    #[test]
    fn line_comment() {
        assert_lex(" // foo\nbar", [(Identifier(String::from("bar")), "bar")]);
    }

    #[test]
    fn block_comment() {
        assert_lex(" /* foo */ bar", [(Identifier(String::from("bar")), "bar")]);
        assert_lex(" /* foo **/ bar", [(Identifier(String::from("bar")), "bar")]);
        assert_lex(" /* foo ***/ bar", [(Identifier(String::from("bar")), "bar")]);
        assert_lex(" /* foo ****/ bar", [(Identifier(String::from("bar")), "bar")]);
        assert_lex(" /* foo *****/ bar", [(Identifier(String::from("bar")), "bar")]);
    }

    #[test]
    fn identifiers() {
        assert_lex(
            "
                foo _foo $foo $_foo _ $ $$ fooBar BarFoo foo10 $1
            ",
             &[
                (Identifier(String::from("foo")), "foo"),
                (Identifier(String::from("_foo")), "_foo"),
                (Identifier(String::from("$foo")), "$foo"),
                (Identifier(String::from("$_foo")), "$_foo"),
                (Identifier(String::from("_")), "_"),
                (Identifier(String::from("$")), "$"),
                (Identifier(String::from("$$")), "$$"),
                (Identifier(String::from("fooBar")), "fooBar"),
                (Identifier(String::from("BarFoo")), "BarFoo"),
                (Identifier(String::from("foo10")), "foo10"),
                (Identifier(String::from("$1")), "$1"),
            ][..]
        );
    }

    #[test]
    fn controls() {
        assert_lex(
            "
                ; : , . ( ) { } [ ]
            ",
             &[
                (Semicolon, ";"),
                (Colon, ":"),
                (Comma, ","),
                (Dot, "."),
                (LeftParen, "("),
                (RightParen, ")"),
                (LeftBrace, "{"),
                (RightBrace, "}"),
                (LeftBracket, "["),
                (RightBracket, "]"),
            ][..]
        );
    }

    #[test]
    fn literals() {
        assert_lex(
            r#"
                true false 0 42 0xDEAD 0Xdead 3.14 .12345 500.1 10.000 'f' '\u2764' "doge to the moon"
            "#,
             &[
                (LiteralBool(true), "true"),
                (LiteralBool(false), "false"),
                (LiteralInt(0), "0"),
                (LiteralInt(42), "42"),
                (LiteralInt(0xDEAD), "0xDEAD"),
                (LiteralInt(0xdead), "0Xdead"),
                (LiteralFloat(3.14), "3.14"),
                (LiteralFloat(0.12345), ".12345"),
                (LiteralFloat(500.1), "500.1"),
                (LiteralFloat(10.000), "10.000"),
                (LiteralChar(char::from('f')), "'f'"),
                (LiteralChar(char::from('❤')), r"'\u2764'"),
                (LiteralString(String::from("doge to the moon")), r#""doge to the moon""#),
            ][..]
        );
    }

    #[test]
    fn strings() {
        assert_lex(r#"
            foo
            "\x19Ethereum Signed Message:\n47Please take my Ether and try to build Polkadot."
        "#,
        &[
            (Identifier(String::from("foo")), "foo"),
            (LiteralString(String::from("\x19Ethereum Signed Message:\n47Please take my Ether and try to build Polkadot.")), 
            r#""\x19Ethereum Signed Message:\n47Please take my Ether and try to build Polkadot.""#),
        ])
    }

    #[test]
    fn keywords() {
        assert_lex(
            "
                else for if return while continue break
            ",
             &[
                (KeywordElse, "else"),
                (KeywordFor, "for"),
                (KeywordIf, "if"),
                (KeywordReturn, "return"),
                (KeywordWhile, "while"),
                (KeywordContinue, "continue"),
                (KeywordBreak, "break")
            ][..]
        );
    }

    #[test]
    fn declarations() {
        assert_lex(
            "
                fn struct enum
            ",
             &[
                (DeclarationFunction, "fn"),
                (DeclarationStruct, "struct"),
                (DeclarationEnum, "enum"),
            ][..]
        );
    }

    #[test]
    fn operators() {
        assert_lex(
            "
                ++ -- ! * / % ^ + - < <= > >= == != && || =
            ",
             &[
                (OpIncrement, "++"),
                (OpDecrement, "--"),
                (OpNot, "!"),
                (OpMul, "*"),
                (OpDiv, "/"),
                (OpMod, "%"),
                (OpPow, "^"),
                (OpPlus, "+"),
                (OpMinus, "-"),
                (OpLessThan, "<"),
                (OpLessThanEqual, "<="),
                (OpGreaterThan, ">"),
                (OpGreaterThanEqual, ">="),
                (OpEqual, "=="),
                (OpNotEqual, "!="),
                (OpAnd, "&&"),
                (OpOr, "||"),
                (OpAssign, "=")
            ][..]
        );
    }

    #[test]
    fn types_easy() {
        assert_lex(
            "
                bool int string char float null
            ",
             &[
                (TypeBool, "bool"),
                (TypeInt, "int"),
                (TypeString, "string"),
                (TypeChar, "char"),
                (TypeFloat, "float"),
                (TypeNull, "null"),
            ][..]
        );
    }

    #[test]
    fn synthesis_test() {
        let file_path = "../test/test.spl";
        /*
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
         */
        assert_lex_from_file(
            file_path,
            &[
                (TypeInt, "int"),
                (Identifier(String::from("main")), "main"),
                (LeftParen, "("),
                (RightParen, ")"),
                (LeftBrace, "{"),
                (TypeInt, "int"),
                (Identifier(String::from("a")), "a"),
                (OpAssign, "="),
                (LiteralInt(3), "3"),
                (Semicolon, ";"),
                (KeywordWhile, "while"),
                (LeftParen, "("),
                (LiteralBool(true), "true"),
                (RightParen, ")"),
                (LeftBrace, "{"),
                (Identifier(String::from("a")), "a"),
                (OpAssign, "="),
                (Identifier(String::from("a")), "a"),
                (OpPlus, "+"),
                (LiteralInt(1), "1"),
                (Semicolon, ";"),
                (KeywordIf, "if"),
                (LeftParen, "("),
                (Identifier(String::from("a")), "a"),
                (OpEqual, "=="),
                (LiteralInt(5), "5"),
                (RightParen, ")"),
                (LeftBrace, "{"),
                (KeywordBreak,"break"),
                (Semicolon, ";"),
                (RightBrace, "}"),
                (RightBrace, "}"),
                (KeywordReturn, "return"),
                (Semicolon, ";"),
                (RightBrace, "}"),
            ][..]
        );
    }
}
