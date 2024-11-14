use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());
}
