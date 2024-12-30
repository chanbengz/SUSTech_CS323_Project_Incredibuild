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
    use std::fs::File;
    use std::io::Read;
    #[allow(unused_imports)]
    use crate::{emit_llvmir, emit_llvmir_to_file, emit_object};

    fn test_from_file(source_path: &str, expected_path: &str, ir: bool) {
        let mut source = String::new();
        let mut expected = String::new();
        File::open(source_path).unwrap().read_to_string(&mut source).unwrap();
        File::open(expected_path).unwrap().read_to_string(&mut expected).unwrap();
        let ast = spl_parser::parse(&source).unwrap();
        let res = if ir {
            emit_llvmir(source_path.split("/").last().unwrap(), ast)
        } else {
            emit_object(source_path.split("/").last().unwrap(), ast)
        };
        assert_eq!(res, expected);
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
        let source = "int foo(int a) { return a+114000; } int main() { printf(\"%d\\n\", foo(514)); return 0; } ";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_funccall.spl", ast);
        assert_eq!(ir, "; ModuleID = 'test_funccall.spl'\nsource_filename = \"test_funccall.spl\"\n\n@0 = internal global [4 x i8] c\"%d\\0A\\00\"\n\ndefine i32 @foo(i32 %a) {\nentry:\n  %a1 = alloca i32, align 4\n  store i32 %a, ptr %a1, align 4\n  %a2 = load i32, ptr %a1, align 4\n  %addtmp = add i32 %a2, 114000\n  ret i32 %addtmp\n}\n\ndefine i32 @main() {\nentry:\n  %foo = call i32 @foo(i32 514)\n  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %foo)\n  ret i32 0\n}\n\ndeclare i32 @printf(ptr, ...)\n");
    }

    #[test]
    fn test_ifexpr(){
        let source = "int main() { int a = 1; if (a == 1) { printf(\"%d\\n\", a); return 1; } else { printf(\"%d\\n\", a); return 0; } return 0;}";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_ifexpr.spl", ast.clone());
        // emit_llvmir_to_file("test_ifexpr.spl", ast, "test_ifexpr.ll");
        assert_eq!(ir, "; ModuleID = 'test_ifexpr.spl'\nsource_filename = \"test_ifexpr.spl\"\n\n@0 = internal global [4 x i8] c\"%d\\0A\\00\"\n@1 = internal global [4 x i8] c\"%d\\0A\\00\"\n\ndefine i32 @main() {\nentry:\n  %a = alloca i32, align 4\n  store i32 1, ptr %a, align 4\n  %a1 = load i32, ptr %a, align 4\n  %eqtmp = icmp eq i32 %a1, 1\n  br i1 %eqtmp, label %then, label %else\n\nthen:                                             ; preds = %entry\n  %a2 = load i32, ptr %a, align 4\n  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %a2)\n  ret i32 1\n\nelse:                                             ; preds = %entry\n  %a3 = load i32, ptr %a, align 4\n  %1 = call i32 (ptr, ...) @printf(ptr @1, i32 %a3)\n  ret i32 0\n\nmerge:                                            ; No predecessors!\n  ret i32 0\n}\n\ndeclare i32 @printf(ptr, ...)\n");
    }

    #[test]
    fn test_whileexpr(){
        let source = "int main() { int a = 1; while (a < 10) { a = a + 1; printf(\"%d\\n\", a); if(a == 4) {break;}} return a; }";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_whileexpr.spl", ast.clone());
        // emit_llvmir_to_file("test_whileexpr.spl", ast, "test_whileexpr.ll");
        assert_eq!(ir, "; ModuleID = 'test_whileexpr.spl'\nsource_filename = \"test_whileexpr.spl\"\n\n@0 = internal global [4 x i8] c\"%d\\0A\\00\"\n\ndefine i32 @main() {\nentry:\n  %a = alloca i32, align 4\n  store i32 1, ptr %a, align 4\n  br label %cond\n\ncond:                                             ; preds = %merge5, %entry\n  %a1 = load i32, ptr %a, align 4\n  %lttmp = icmp slt i32 %a1, 10\n  br i1 %lttmp, label %body, label %merge\n\nbody:                                             ; preds = %cond\n  %a2 = load i32, ptr %a, align 4\n  %addtmp = add i32 %a2, 1\n  store i32 %addtmp, ptr %a, align 4\n  %a3 = load i32, ptr %a, align 4\n  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %a3)\n  %a4 = load i32, ptr %a, align 4\n  %eqtmp = icmp eq i32 %a4, 4\n  br i1 %eqtmp, label %then, label %merge5\n\nmerge:                                            ; preds = %then, %cond\n  %a6 = load i32, ptr %a, align 4\n  ret i32 %a6\n\nthen:                                             ; preds = %body\n  br label %merge\n\nmerge5:                                           ; preds = %body\n  br label %cond\n}\n\ndeclare i32 @printf(ptr, ...)\n");
    }

    #[test]
    fn test_forexpr(){
        let source = "int main() { int a = 1; for(int i = 0; i < 10; i = i + 1) { a = a + 1; printf(\"%d\\n\", a); if(a == 4) {break;}} return a; }";
        let ast = spl_parser::parse(source).unwrap();
        let ir = emit_llvmir("test_forexpr.spl", ast.clone());
        // emit_llvmir_to_file("test_forexpr.spl", ast, "test_forexpr.ll");
        assert_eq!(ir, "; ModuleID = 'test_forexpr.spl'\nsource_filename = \"test_forexpr.spl\"\n\n@0 = internal global [4 x i8] c\"%d\\0A\\00\"\n\ndefine i32 @main() {\nentry:\n  %a = alloca i32, align 4\n  store i32 1, ptr %a, align 4\n  br label %init\n\ninit:                                             ; preds = %entry\n  %i = alloca i32, align 4\n  store i32 0, ptr %i, align 4\n  br label %cond\n\ncond:                                             ; preds = %step, %init\n  %i1 = load i32, ptr %i, align 4\n  %lttmp = icmp slt i32 %i1, 10\n  br i1 %lttmp, label %body, label %merge\n\nbody:                                             ; preds = %cond\n  %a2 = load i32, ptr %a, align 4\n  %addtmp = add i32 %a2, 1\n  store i32 %addtmp, ptr %a, align 4\n  %a3 = load i32, ptr %a, align 4\n  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %a3)\n  %a4 = load i32, ptr %a, align 4\n  %eqtmp = icmp eq i32 %a4, 4\n  br i1 %eqtmp, label %then, label %merge5\n\nstep:                                             ; preds = %merge5\n  %i6 = load i32, ptr %i, align 4\n  %addtmp7 = add i32 %i6, 1\n  store i32 %addtmp7, ptr %i, align 4\n  br label %cond\n\nmerge:                                            ; preds = %then, %cond\n  %a8 = load i32, ptr %a, align 4\n  ret i32 %a8\n\nthen:                                             ; preds = %body\n  br label %merge\n\nmerge5:                                           ; preds = %body\n  br label %step\n}\n\ndeclare i32 @printf(ptr, ...)\n");
    }

    #[test]
    fn gen_test_r00() {
        test_from_file("../../test/test_0_r00.spl", "../../test/test_0_r00.ll", true);
    }

    #[test]
    fn test_phase3() {
        for i in 1..=6 {
            let source_path = format!("../../test/phase3/test_3_r{:02}.spl", i);
            let expected_path = format!("../../test/phase3/test_3_r{:02}.ll", i);
            test_from_file(&source_path, &expected_path, true);
        }
    }
}