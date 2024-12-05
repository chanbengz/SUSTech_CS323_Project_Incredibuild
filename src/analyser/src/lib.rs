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
    use crate::symbol::{Symbol, VarType, BasicType, VarSymbol}; 
    use spl_parser::parse;
    use crate::table::ScopeTable;
    use crate::stack::ScopeStack;
    use crate::manager::SymbolManager;
    use crate::walker::Walker;

    fn parse_program(program: &str) -> ScopeStack {
        let ast = parse(program).unwrap();
        let manager = SymbolManager::default();
        let mut walker = Walker::new(ast, manager);
        walker.traverse();
        walker.get_tables()
    }

    #[test]
    fn test_scope_stack(){
        let program = r#"
        int main(){
            int a = 0;
            int b = 1;
            {
                int a = 1;
                int b = 2;
            }
        }
        "#;
        let table = parse_program(program);
        assert_eq!(format!("{}", table), "")
    }
}
