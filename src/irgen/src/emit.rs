use inkwell as llvm;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use spl_ast::tree;
use spl_ast::tree::{CompExpr, Value};

pub fn emit_llvmir(source: &str, ) -> String {
    let context = llvm::context::Context::create();
    let module = context.create_module(source.unwrap());
    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let fn_value = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(basic_block);

    let i32_type = context.i32_type();
    let i32_value = i32_type.const_int(42, false);
    builder.build_return(Some(&i32_value));

    module.print_to_string().to_string()
}

fn codegen_compexpr(context: &llvm::context::Context, builder: &llvm::builder::Builder, compexpr: &tree::CompExpr) -> LLVMValueRef {
    match compexpr {
        CompExpr::Value(n) => {
            match n {
                Value::Integer(_) => {}
                Value::Float(_) => {}
                Value::String(_) => {}
                Value::Char(_) => {}
                Value::Bool(_) => {}
                Value::Struct(_) => {}
                Value::Null => {}
            }
        },
        // CompExpr::Variable(var) => {},
        CompExpr::UnaryOperation(op, rhs) => {

        },
        CompExpr::BinaryOperation(lhs, op, rhs) => {

        },
        // CompExpr::FuncCall(_) => {},
        _ => {
            panic!("Invalid CompExpr");
        }
    }
}