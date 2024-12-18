mod checker;
mod emit;

#[cfg(test)]
mod tests {
    use crate::emit::emit_llvmir;

    #[test]
    fn gen_test_r00() {
    }

    #[test]
    fn test_compexpr() {
        let source = "int main() { return 1 + 2 * 3;} ";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_compexpr", ast);
        assert_eq!(ir, "; ModuleID = 'test_compexpr'\nsource_filename = \"test_compexpr\"\n\ndefine i32 @main() {\nentry:\n  ret i32 7\n}\n");
    }
}