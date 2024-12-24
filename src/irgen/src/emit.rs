use std::ops::Deref;
use inkwell as llvm;
use inkwell::AddressSpace;
use inkwell::module::Linkage;
use inkwell::types::{BasicMetadataTypeEnum, IntType};
use inkwell::values::{BasicValue, BasicValueEnum, PointerValue};
use spl_ast::tree;

pub fn emit_llvmir(source: &str, ast: tree::Program) -> String {
    let context = llvm::context::Context::create();
    let emitter = LLZN::new(&context, source);
    emitter.emit(ast);
    emitter.module.print_to_string().to_string()
}

struct LLZN<'ctx> {
    pub context: &'ctx llvm::context::Context,
    pub builder: llvm::builder::Builder<'ctx>,
    pub module: llvm::module::Module<'ctx>,
}


impl<'ctx> LLZN<'ctx> {
    pub fn new(context: &'ctx llvm::context::Context, source: &str) -> Self {
        Self {
            context,
            builder: context.create_builder(),
            module: context.create_module(source),
        }
    }

    pub fn emit(&self, ast: tree::Program) {
        ast.emit(self);
    }

    pub fn emit_printf_call(&self, fmt_str: &str, name: &str) -> IntType {
        let i32_type = self.context.i32_type();
        let str_type = self.context.ptr_type(AddressSpace::default());
        let printf_type = i32_type.fn_type(&[str_type.into()], true);

        let printf = self
            .module
            .add_function("printf", printf_type, Some(Linkage::External));

        let pointer_value = self.emit_global_string(fmt_str, name);
        self.builder.build_call(printf, &[pointer_value.into()], "").expect("Error in emit_printf_call");

        i32_type
    }

    fn emit_global_string(&self, string: &str, name: &str) -> PointerValue {
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
}


trait Emit<'ctx> {
    type Output;
    fn emit(&self, emitter: &'ctx LLZN) -> Self::Output;
}

impl<'ctx> Emit<'ctx> for tree::Program {
    type Output = ();
    fn emit(&self, emitter: &'ctx LLZN) {
        match self {
            tree::Program::Program(parts) => {
                for part in parts {
                    part.emit(emitter);
                }
            }
            tree::Program::Error => panic!("Error in Program"),
        }
    }
}

impl<'ctx> Emit<'ctx> for tree::ProgramPart {
    type Output = ();
    fn emit(&self, emitter: &'ctx LLZN) {
        match self {
            tree::ProgramPart::Statement(stmt) => stmt.emit(emitter),
            tree::ProgramPart::Function(func) => func.emit(emitter),
        }
    }
}

impl<'ctx> Emit<'ctx> for tree::Statement {
    type Output = ();
    fn emit(&self, emitter: &'ctx LLZN) {
        match self {
            tree::Statement::GlobalVariable(vars, ..) => {
                for var in vars {
                    match var {
                        tree::Variable::VarAssignment(_var, _) => {},
                        tree::Variable::VarReference(_, _) => {},
                        tree::Variable::VarDeclaration(_, _, _) => {},
                        tree::Variable::StructDefinition(_, _) => {}
                        tree::Variable::StructDeclaration(_, _, _) => {}
                        tree::Variable::StructReference(_) => {}
                        _ => unimplemented!()
                    }
                }
            }
            tree::Statement::Struct(def, _) => def.emit(emitter),
            _ => panic!("Error in Statement"),
        }
    }
}

impl<'ctx> Emit<'ctx> for tree::Variable {
    type Output = ();
    fn emit(&self, _emitter: &'ctx LLZN) {
        match self {
            tree::Variable::VarAssignment(var, expr) => {
                // TODO: Implement VarAssignment
            }
            tree::Variable::VarReference(name, dims) => {
                // TODO: Implement VarReference
            }
            tree::Variable::VarDeclaration(name, ty, dims) => {
                // TODO: Implement VarDeclaration
            }
            tree::Variable::StructDefinition(name, vars) => {
                // TODO: Implement StructDefinition
            }
            tree::Variable::StructDeclaration(structname, inst, members) => {
                // TODO: Implement StructDeclaration
            }
            tree::Variable::StructReference(vars) => {
                // TODO: Implement StructReference
            }
            _ => panic!("Error in Variable"),
        }
    }
}

impl<'ctx> Emit<'ctx> for tree::Function {
    type Output = ();
    fn emit(&self, emitter: &'ctx LLZN) {
        match self {
            tree::Function::FuncDeclaration(name, params, ret_ty, body) => {
                let mut paras_ty = Vec::new();
                for param in params {
                    if let tree::Variable::FormalParameter(_, ty, _) = param {
                        match *ty.deref() {
                            tree::Value::Integer(_) => paras_ty.push(BasicMetadataTypeEnum::from(emitter.context.i32_type())),
                            tree::Value::Float(_) => paras_ty.push(BasicMetadataTypeEnum::from(emitter.context.f32_type())),
                            _ => panic!("Error in Function"),
                        }
                    } else {
                        panic!("Error in Function");
                    }
                }
                let retty = match *ret_ty.deref() {
                    tree::Value::Integer(_) => emitter.context.i32_type().fn_type(paras_ty.as_ref(), false),
                    _ => panic!("Error in Function"),
                };
                let func = emitter.module.add_function(name, retty, None);
                let entry = emitter.context.append_basic_block(func, "entry");
                emitter.builder.position_at_end(entry);

                for (i, param) in params.iter().enumerate() {
                    let (ptr, value) = if let tree::Variable::FormalParameter(name, ty, _) = param {
                        let value = func.get_nth_param(i as u32).unwrap();
                        value.set_name(name);
                        match *ty.deref() {
                            tree::Value::Integer(_) => (emitter.builder.build_alloca(emitter.context.i32_type(), name), value),
                            tree::Value::Float(_) => (emitter.builder.build_alloca(emitter.context.f32_type(), name), value),
                            _ => panic!("Error in Function"),
                        }
                    } else {
                        panic!("Error in Function");
                    };

                    emitter.builder.build_store(ptr.unwrap(), value).expect("Error in Function");
                }

                body.emit(emitter);

            },
            _ => panic!("Error in Function"), // FuncCall
        }
    }
}

impl<'ctx> Emit<'ctx> for tree::Body {
    type Output = ();
    fn emit(&self, emitter: &'ctx LLZN) {
        match self {
            tree::Body::Body(stmts) => {
                for stmt in stmts {
                    stmt.emit(emitter);
                }
            }
            _ => panic!("Error in Body"),
        }
    }
}

impl<'ctx> Emit<'ctx> for tree::Expr {
    type Output = ();
    fn emit(&self, emitter: &'ctx LLZN) {
        match self {
            tree::Expr::Return(expr, ..) => {
                if expr.eq(&tree::CompExpr::Value(tree::Value::Null)) {
                    emitter.builder.build_return(None).expect("Error in Expr");
                } else {
                    let ret = expr.emit(emitter);
                    emitter.builder.build_return(Some(&ret)).expect("Error in Expr");
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl<'ctx> Emit<'ctx> for tree::CompExpr {
    type Output = BasicValueEnum<'ctx>;

    fn emit(&self, emitter: &'ctx LLZN) -> BasicValueEnum<'ctx> {
        match self {
            tree::CompExpr::Value(n) => match n {
                tree::Value::Integer(n) => emitter.context.i32_type().const_int(*n as u64, false).as_basic_value_enum(),
                tree::Value::Float(n) => emitter.context.f32_type().const_float(*n as f64).as_basic_value_enum(),
                _ => panic!("Error in CompExpr"), // TODO: implement other types
            }
            // CompExpr::Variable(_) => {}
            // CompExpr::FuncCall(_) => {}
            tree::CompExpr::BinaryOperation(lhs, op, rhs) => {
                let lhs = (*lhs.deref()).emit(emitter);
                let rhs = (*rhs.deref()).emit(emitter);
                match op {
                    tree::BinaryOperator::Add => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_add(lhs, rhs.into_int_value(), "addtmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_add(lhs, rhs.into_float_value(), "addtmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"), // TODO: implement other types
                        }
                    }
                    tree::BinaryOperator::Sub => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_sub(lhs, rhs.into_int_value(), "subtmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_sub(lhs, rhs.into_float_value(), "subtmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"), // TODO: implement other types
                        }
                    }
                    tree::BinaryOperator::Mul => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_mul(lhs, rhs.into_int_value(), "multmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_mul(lhs, rhs.into_float_value(), "multmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"), // TODO: implement other types
                        }
                    }
                    tree::BinaryOperator::Div => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_unsigned_div(lhs, rhs.into_int_value(), "divtmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_div(lhs, rhs.into_float_value(), "divtmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"), // TODO: implement other types
                        }
                    }
                _ => panic!("Error in CompExpr"),
            }}
            _ => panic!("Error in CompExpr"),
        }
    }
}