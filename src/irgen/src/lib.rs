mod checker;
mod emit;

#[cfg(test)]
mod tests {
    use crate::emit::{emit_llvmir, emit_object};

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
    fn test_printf() {
        let source = "int main() { printf(\"Number is %d\", 114000 + 514); return 0; } ";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_printf", ast.clone());
        // emit_object("test_printf", ast, "test_printf");
        assert_eq!(ir, "; ModuleID = 'test_printf'\nsource_filename = \"test_printf\"\n\n@tmp = internal global [12 x i8] c\"Number is %d\"\n\ndefine i32 @main() {\nentry:\n  %printf = call i32 (ptr, ...) @printf(ptr @tmp, i32 114514)\n  ret i32 0\n}\n\ndeclare i32 @printf(ptr, ...)\n");
    }
}