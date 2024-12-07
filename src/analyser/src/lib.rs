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
    use std::fs::File;
    use std::io::Read;
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

    fn assert_analyze_from_file(file_path: &str, out_path: &str){
        let mut src_content = String::new();
        let mut src_file = File::open(file_path).expect("Unable to open file");
        src_file.read_to_string(&mut src_content)
            .expect("Unable to read file");

        let mut out_content = String::new();
        let mut out_file = File::open(out_path).expect("Unable to open file");
        out_file.read_to_string(&mut out_content)
            .expect("Unable to read file");
        let expected = out_content.trim();

        let ast = parse(&src_content).unwrap();
        let manager = SymbolManager::default();
        let mut walker = Walker::new(ast, manager);

        walker.traverse();
        let table = walker.get_tables();
        let errors = walker.get_errors();
        assert_eq!(
            format!("{}", errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")),
            expected
        );
    }

    fn test_scope_stack(){
        let program = r#"
        int main(){
            int a = 0;
            int b = 1;
            if(a == 2){
                int a = 1;
                int b = 2;
            }
        }
        "#;
        let table = parse_program(program);
        assert_eq!(format!("{}", table), "")
    }

    #[test]
    fn test_phase2(){
        for i in 1..15 {
            if i == 6 || i == 10 || i == 13 {
                continue;
            }
            let in_path = format!("../test/phase2/test_2_r{:0>2}.spl", i);
            let out_path = format!("../test/phase2/test_2_r{:0>2}.out", i);
            assert_analyze_from_file(&in_path, &out_path);
        }
    }

    #[test]
    fn test_specific(){
        let in_path = "../test/phase2/test_2_r14.spl";
        let out_path = "../test/phase2/test_2_r14.out";
        assert_analyze_from_file(in_path, out_path);
    }
}


