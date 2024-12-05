extern crate llvm_ir as llvm;

use std::fs::File;
use std::io::Read;
use spl_parser::parse;
use clap::{arg, Arg, Command};
// use std::ffi::CString;
// use std::ptr;

fn main() {
    let args = Command::new("Incredibuild")
        .about("Compile SPL code to executable")
        .arg(
            Arg::new("input").index(1).required(true)
        )
        .arg(
            Arg::new("output").short('o').long("output").required(false)
        )
        .get_matches();
    let mut input = String::new();
    File::open(args.get_one::<String>("input").unwrap())
        .unwrap().read_to_string(&mut input)
        .expect(format!("error: could not read file {}",
            args.get_one::<String>("input").unwrap()).as_str()
        );


    let parsed_input = parse(&input).unwrap();
    println!("{:?}", parsed_input);

    // unsafe {
    //     codegen(parsed_input);
    // }
}

// unsafe fn codegen(input: String) {
//     let context = llvm::core::LLVMContextCreate();
//     let module = llvm::core::LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
//     let builder = llvm::core::LLVMCreateBuilderInContext(context);

//     // In LLVM, you get your types from functions.
//     let int_type = llvm::core::LLVMInt64TypeInContext(context);
//     let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
//     let function = llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);

//     let entry_name = CString::new("entry").unwrap();
//     let bb = llvm::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
//     llvm::core::LLVMPositionBuilderAtEnd(builder, bb);

//     // The juicy part: construct a `LLVMValue` from a Rust value:
//     let int_value: u64 = input.parse().unwrap();
//     let int_value = llvm::core::LLVMConstInt(int_type, int_value, 0);

//     llvm::core::LLVMBuildRet(builder, int_value);

//     // Instead of dumping to stdout, let's write out the IR to `out.ll`
//     let out_file = CString::new("out.ll").unwrap();
//     llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

//     llvm::core::LLVMDisposeBuilder(builder);
//     llvm::core::LLVMDisposeModule(module);
//     llvm::core::LLVMContextDispose(context);
// }
