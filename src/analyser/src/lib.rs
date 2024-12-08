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
    use crate::walker::Walker;

    fn assert_analyze_from_file(file_path: &str, out_path: &str){
        let mut out_content = String::new();
        let mut out_file = File::open(out_path).expect("Unable to open file");
        out_file.read_to_string(&mut out_content)
            .expect("Unable to read file");
        let expected = out_content.trim();

        let mut walker = Walker::new(file_path);

        walker.traverse();
        let _table = walker.get_tables();
        let errors = walker.get_errors();
        assert_eq!(
            format!("{}", errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")),
            expected
        );
    }

    #[test]
    fn test_phase2(){
        for i in 1..16 {
            if i == 6 {
                continue;
            }
            let in_path = format!("../test/phase2/test_2_r{:0>2}.spl", i);
            let out_path = format!("../test/phase2/test_2_r{:0>2}.out", i);
            assert_analyze_from_file(&in_path, &out_path);
        }
    }

    #[test]
    fn test_self_defined(){
        for i in 1..4 {
            let in_path = format!("../test/phase2/self_def_s{:0>2}.spl", i);
            let out_path = format!("../test/phase2/self_def_s{:0>2}.out", i);
            assert_analyze_from_file(&in_path, &out_path);
        }
    }
}


