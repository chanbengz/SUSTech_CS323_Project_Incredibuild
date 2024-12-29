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
        let ir = emit_llvmir("test_compexpr.spl", ast);
        assert_eq!(ir, "; ModuleID = 'test_compexpr.spl'\nsource_filename = \"test_compexpr.spl\"\n\ndefine i32 @main() {\nentry:\n  ret i32 7\n}\n");
    }

    #[test]
    fn test_var() {
        let source = "int main() { int a = 1; return a; } ";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_var.spl", ast);
        assert_eq!(ir, "; ModuleID = 'test_var.spl'\nsource_filename = \"test_var.spl\"\n\ndefine i32 @main() {\nentry:\n  %a = alloca i32, align 4\n  store i32 1, ptr %a, align 4\n  %a1 = load i32, ptr %a, align 4\n  ret i32 %a1\n}\n");
    }

    #[test]
    fn test_funccall() {
        let source = "int foo() { return 114+514; } int main() { printf(\"%d\\n\", foo()); return 0; } ";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_funccall.spl", ast);
        assert_eq!(ir, "; ModuleID = 'test_funccall.spl'\nsource_filename = \"test_funccall.spl\"\n\n@0 = internal global [4 x i8] c\"%d\\0A\\00\"\n\ndefine i32 @foo() {\nentry:\n  ret i32 628\n}\n\ndefine i32 @main() {\nentry:\n  %foo = call i32 @foo()\n  %printf_tmp = call i32 (ptr, ...) @printf(ptr @0, i32 %foo)\n  ret i32 0\n}\n\ndeclare i32 @printf(ptr, ...)\n");
    }
}