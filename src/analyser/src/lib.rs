mod error;
pub mod walker;
pub mod stack;
pub mod symbol;
pub mod typer;
pub mod impls;
pub mod table;
pub mod fmt;
pub mod from;
pub mod manager;

#[cfg(test)]
mod tests {
    use crate::symbol::{Symbol, VarType, BasicType, Val, VarSymbol}; 
    use spl_parser::parse;
    use crate::table::ScopeTable;
    use crate::stack::ScopeStack;
    use crate::manager::SymbolManager;
    use crate::walker::Walker;

    fn parse_program_symbols(program: &str) -> Vec<VarSymbol> {
        let ast = parse(program).unwrap();
        let manager = SymbolManager::default();
        let mut walker = Walker::new(ast, manager);
        walker.traverse();
        walker.get_symbols()
    }

    #[test]
    fn test_symbol_table() {
        let program = r#"
        int a = 0;
        int b = 0;
        "#;
        let symbols = parse_program_symbols(program);
        let mut table = ScopeTable::<VarSymbol>::new();
        for symbol in symbols {
            table.insert(symbol.identifier.clone(), symbol);
        }
        assert_eq!(format!("{}", table), "a: Symbol { id: 1, is_global: false, identifier: \"a\", symbol_type: Primitive((Int, Int(0))) }\nb: Symbol { id: 2, is_global: false, identifier: \"b\", symbol_type: Primitive((Int, Int(0))) }\n");
        let a = table.lookup(&String::from("a")).unwrap();
        assert_eq!(a.get_primitive().unwrap().0, (BasicType::Int));
        assert_eq!(a.get_primitive().unwrap().1, (Val::Int(0)));

        table.remove(&String::from("a"));
        if let Some(a) = table.lookup(&String::from("a")) {
            assert!(false);
        }
    }

    // fn test_scope_stack(){
    //     let program = r#"
    //     int a = 0;
    //     int b = 0;
    //     "#;
    //     let symbols = parse_program_symbols(program);
    //     let mut stack = ScopeStack::new();

    // }
}
