pub mod emit;
mod checker;

#[cfg(test)]
mod tests {
    use crate::emit::{emit_llvmir};

    #[test]
    fn gen_test_r00() {
    }

    #[test]
    fn test_compexpr() {
        let source = "int main() { return 1 + 2 * 3; } ";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_compexpr", ast);
        assert_eq!(ir, "; ModuleID = 'test_compexpr'\nsource_filename = \"test_compexpr\"\n\ndefine i32 @main() {\nentry:\n  ret i32 7\n}\n");
    }

    #[test]
    fn test_var() {
        let source = "int main() { int a = 1; return a; } ";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_var", ast);
        assert_eq!(ir, "; ModuleID = 'test_var'\nsource_filename = \"test_var\"\n\ndefine i32 @main() {\nentry:\n  %a = alloca i32, align 4\n  store i32 1, ptr %a, align 4\n  %a1 = load i32, ptr %a, align 4\n  ret i32 %a1\n}\n");
    }
}