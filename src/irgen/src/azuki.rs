use inkwell as llvm;
use std::collections::HashMap;
use inkwell::AddressSpace;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Linkage;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::{BasicTypeEnum, IntType};
use inkwell::values::{BasicMetadataValueEnum, PointerValue};

pub(crate) struct Azuki<'ast, 'ctx> {
    pub context: &'ctx llvm::context::Context,
    pub builder: llvm::builder::Builder<'ctx>,
    pub module: llvm::module::Module<'ctx>,

    pub scope: Vec<HashMap<&'ast str, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>>,
    printf: Option<llvm::values::FunctionValue<'ctx>>,
    scanf: Option<llvm::values::FunctionValue<'ctx>>,
}

impl<'ast, 'ctx> Azuki<'ast, 'ctx> {
    pub fn new(context: &'ctx llvm::context::Context, source: &str) -> Self {
        let module = context.create_module(source);

        Self {
            context,
            builder: context.create_builder(),
            module,
            scope: vec![HashMap::new()],
            printf: None,
            scanf: None,
        }
    }

    pub(crate) fn emit_printf_call(&mut self, args: &[BasicMetadataValueEnum]) -> IntType {
        if self.printf.is_none() {
            let i32type = self.context.i32_type();
            let strtype = self.context.ptr_type(AddressSpace::default()).into();
            self.printf = Some(self.module.add_function("printf", i32type.fn_type(&[strtype], true), Some(Linkage::External)));
        }

        self.builder.build_call(self.printf.unwrap(), args, "printf_tmp").expect("Error in emit_printf_call");
        self.context.i32_type()
    }

    pub(crate) fn emit_scanf_call(&mut self, args: &[BasicMetadataValueEnum]) -> IntType {
        if self.scanf.is_none() {
            let i32type = self.context.i32_type();
            let strtype = self.context.ptr_type(AddressSpace::default()).into();
            self.scanf = Some(self.module.add_function("scanf", i32type.fn_type(&[strtype], true), Some(Linkage::External)));
        }

        self.builder.build_call(self.scanf.unwrap(), args, "scanf_tmp").expect("Error in emit_scanf_call");
        self.context.i32_type()
    }

    pub(crate) fn emit_global_string(&mut self, string: &mut String, name: &str) -> PointerValue<'ctx> {
        string.push('\0');
        let ty = self.context.i8_type().array_type(string.len() as u32);
        let gv = self
            .module
            .add_global(ty, Some(AddressSpace::default()), name);
        gv.set_linkage(Linkage::Internal);
        gv.set_initializer(&self.context.const_string(string.as_ref(), false));

        let pointer_value = self.builder.build_pointer_cast(
            gv.as_pointer_value(),
            self.context.ptr_type(AddressSpace::default()),
            name,
        );

        pointer_value.unwrap()
    }

    pub(crate) fn gen_code(&mut self) -> MemoryBuffer {
        Target::initialize_all(&InitializationConfig::default());
        let triple = TargetMachine::get_default_triple();
        let target  = Target::from_triple(&triple).unwrap();
        let target_machine = target
            .create_target_machine(
                &triple,
                TargetMachine::get_host_cpu_name().to_str().unwrap_or_default(),
                TargetMachine::get_host_cpu_features().to_str().unwrap_or_default(),
                inkwell::OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        target_machine.write_to_memory_buffer(&self.module, FileType::Assembly).unwrap()
    }

    pub(crate) fn get_var(&self, name: &str) -> Option<(&PointerValue<'ctx>, &BasicTypeEnum<'ctx>)> {
        for scope in self.scope.iter().rev() {
            if let Some((ptr, ty)) = scope.get(name) {
                return Some((ptr, ty));
            }
        }
        None
    }
}
