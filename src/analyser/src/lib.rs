mod error;
pub mod utils;
pub mod scope;
pub mod symbol;
pub mod typer;
pub mod impls;
pub mod table;

use spl_parser::parse;

#[cfg(test)]
mod tests {

    #[test]
    fn test_symbol_table() {
        let code = r#"
        int a = 10;
        int b = 20;
        int c = 30;
        int d = 40;
        int e = 50;
        int f = 60;
        int g = 70;
        int h = 80;
        int i = 90;
        int j = 100;
        "#;
        let ast = parse(code).unwrap();
        let mut table = table::HashLinkedMap::new();
        for decl in ast.decls {
            table.insert(decl);
        }
        assert_eq!(table.symbols.len(), 10);
    }
}
