mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub use grammar::ExprParser;

#[test]
fn test_spl() {
}
