use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use spl_ast::tree;
use crate::azuki::Azuki;
use crate::emit::Emit;
use inkwell as llvm;

mod emit;
mod azuki;

pub fn emit_llvmir(source: &str, ast: tree::Program) -> String {
    let context = llvm::context::Context::create();
    let mut emitter = Azuki::new(&context, source);
    ast.emit(&mut emitter);
    emitter.module.print_to_string().to_string()
}

pub fn emit_object(source: &str, ast: tree::Program) -> String {
    let context = llvm::context::Context::create();
    let mut emitter = Azuki::new(&context, source);
    ast.emit(&mut emitter);
    let mut s = String::new();
    emitter.gen_code().as_slice().read_to_string(&mut s).expect("");
    s
}

pub fn emit_llvmir_to_file(source: &str, ast: tree::Program, path: &str) {
    let context = llvm::context::Context::create();
    let mut emitter = Azuki::new(&context, source);
    ast.emit(&mut emitter);
    emitter.module.print_to_file(Path::new(path)).expect("Error in emit_llvmir_to_file");
}

pub fn emit_object_to_file(source: &str, ast: tree::Program, path: &str) {
    let s = emit_object(source, ast);
    let mut file = File::create(path).expect("Error in emit_object_to_file");
    file.write_all(s.as_bytes()).expect("Error in emit_object_to_file");
}

#[cfg(test)]
mod tests {
    use crate::emit_llvmir;

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