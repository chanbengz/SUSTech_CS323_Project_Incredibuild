extern crate logos;

mod lexer;
pub use lexer::Token;
pub use logos::{Logos, Lexer, Source};

// #[inline]
// pub fn read_pragma<'source>(lex: &mut Lexer<Token>) -> S::Slice {
//     use logos::internal::LexerInternal;

//     loop {
//         match lex.read() {
//             0x01...0x20 => lex.bump(),
//             _           => break,
//         }
//     }

//     let start = lex.range().start;

//     loop {
//         match lex.read() {
//             0 => {
//                 lex.token = Token::UnexpectedEndOfProgram;
//                 let end = lex.range().end;

//                 return lex.source.slice(start..end).expect("0 guarantees being at the end; qed");
//             },
//             b';' => {
//                 let end = lex.range().end;

//                 let version = lex.source.slice(start..end).expect("Still within bounds; qed");

//                 lex.token = Token::Semicolon;
//                 lex.bump();

//                 return version;
//             },
//             _ => lex.bump(),
//         }
//     }
// }


#[cfg(test)]
mod test {
    use lexer::Token;
    use lexer::Token::*;
    use logos::Logos;

    fn assert_lex<T>(source: &str, tokens: T)
    where
        T: AsRef<[(Token, &'static str)]>
    {
        let mut lexer = Token::lexer(source);  // Create a Logos lexer for the source

        // Iterate over the expected tokens
        for &(ref token, slice) in tokens.as_ref() {
            // Get the next token from the lexer
            if let Some(lexed_token) = lexer.next() {
                // Assert that the token type and slice match the expected values
                assert!(
                    lexed_token == Ok(token.clone()) && lexer.slice() == slice,
                    "\n\n\n\tExpected {:?}({:?}), found {:?}({:?}) instead!\n\n\n",
                    token,
                    slice,
                    lexed_token,
                    lexer.slice()
                );
            } else {
                // If no more tokens are available but we still have expected tokens, fail the test
                panic!("Unexpected end of tokens while lexing.");
            }
        }

        // Ensure that the lexer has reached the end of the source
        assert_eq!(lexer.next(), None);  // Lexer should be done
    }


    #[test]
    fn empty_lexer() {
        assert_lex("   ", []);
    }

    #[test]
    fn line_comment() {
        assert_lex(" // foo\nbar", [(Identifier, "bar")]);
    }

    // #[test]
    // fn block_comment() {
    //     assert_lex(" /* foo */ bar", [(Identifier, "bar")]);
    //     assert_lex(" /* foo **/ bar", [(Identifier, "bar")]);
    //     assert_lex(" /* foo ***/ bar", [(Identifier, "bar")]);
    //     assert_lex(" /* foo ****/ bar", [(Identifier, "bar")]);
    //     assert_lex(" /* foo *****/ bar", [(Identifier, "bar")]);
    //     assert_lex(" /* foo ", [(UnexpectedEndOfProgram, "/* foo ")]);
    // }

    #[test]
    fn identifiers() {
        assert_lex(
            "
                foo _foo $foo $_foo _ $ $$ fooBar BarFoo foo10 $1
            ",
             &[
                (Identifier, "foo"),
                (Identifier, "_foo"),
                (Identifier, "$foo"),
                (Identifier, "$_foo"),
                (Identifier, "_"),
                (Identifier, "$"),
                (Identifier, "$$"),
                (Identifier, "fooBar"),
                (Identifier, "BarFoo"),
                (Identifier, "foo10"),
                (Identifier, "$1"),
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
                true false 0 42 0xDEAD 0Xdead 3.14 .12345 500.1 10.000 'foo bar' "doge to the moon"
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
            (Identifier, "foo"),
            (LiteralString(String::from("\x19Ethereum Signed Message:\n47Please take my Ether and try to build Polkadot.")), r#""\x19Ethereum Signed Message:\n47Please take my Ether and try to build Polkadot.""#),
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

    // #[test]
    // fn second_price_auction() {
    //     let source = include_str!("../../lunarity/benches/second-price-auction.sol");

    //     let mut lex = Token::lexer(source);
    //     let mut tokens = 0;

    //     while lex.token != EndOfProgram {
    //         assert_ne!(lex.token, UnexpectedToken, "Unexpected: {} at {:?}", lex.slice(), lex.range());
    //         assert_ne!(lex.token, UnexpectedEndOfProgram);

    //         tokens += 1;

    //         lex.advance();
    //     }

    //     assert_eq!(tokens, 1299);
    // }
}