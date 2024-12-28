use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use inkwell as llvm;
use inkwell::AddressSpace;
use inkwell::module::Linkage;
use inkwell::targets::*;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, IntType};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue, PointerValue};
use spl_ast::tree;


pub fn emit_llvmir(source: &str, ast: tree::Program) -> String {
    let context = llvm::context::Context::create();
    let mut emitter = Azuki::new(&context, source);
    emitter.emit(&ast);
    emitter.module.print_to_string().to_string()
}

pub fn emit_object(source: &str, ast: tree::Program, path: &str) {
    let context = llvm::context::Context::create();
    let mut emitter = Azuki::new(&context, source);
    emitter.emit(&ast);
    emitter.gen_code(Path::new(path));
}

struct Azuki<'ast, 'ctx> {
    pub context: &'ctx llvm::context::Context,
    pub builder: llvm::builder::Builder<'ctx>,
    pub module: llvm::module::Module<'ctx>,

    pub scope: Vec<HashMap<&'ast str, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>>,
    printf: llvm::values::FunctionValue<'ctx>,
}

impl<'ast, 'ctx> Azuki<'ast, 'ctx> {
    pub fn new(context: &'ctx llvm::context::Context, source: &str) -> Self {
        let module = context.create_module(source);
        let i32type = context.i32_type();
        let strtype = context.ptr_type(AddressSpace::default()).into();
        let printf = module.add_function("printf", i32type.fn_type(&[strtype], true), Some(Linkage::External));

        Self {
            context,
            builder: context.create_builder(),
            module,
            scope: vec![HashMap::new()],
            printf,
        }
    }

    fn emit(&mut self, ast: &'ast tree::Program) {
        ast.emit(self);
    }

    fn emit_printf_call(&mut self, args: &[BasicMetadataValueEnum]) -> IntType {
        let i32_type = self.context.i32_type();
        self.builder.build_call(self.printf, args, "").expect("Error in emit_printf_call");
        i32_type
    }

    fn emit_global_string(&mut self, string: &mut String, name: &str) -> PointerValue<'ctx> {
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

    fn gen_code(&mut self, path: &Path) {
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

        target_machine.write_to_file(&self.module, FileType::Object, path).unwrap();
    }
}


// Emit LLVM IR from AST
trait Emit<'ast, 'ctx> {
    type Output;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output;
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Program {
    type Output = ();
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) {
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

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::ProgramPart {
    type Output = ();
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) {
        match self {
            tree::ProgramPart::Statement(stmt) => stmt.emit(emitter),
            tree::ProgramPart::Function(func) => { func.emit(emitter); },
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Statement {
    type Output = ();
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) {
        match self {
            tree::Statement::GlobalVariable(vars, ..) => {
                for var in vars {
                    match var {
                        tree::Variable::VarAssignment(var, expr) => {

                        },
                        tree::Variable::VarReference(name, dims) => {

                        },
                        tree::Variable::VarDeclaration(name, ty, dims) => {

                        },
                        // tree::Variable::StructDefinition(_, _) => {}
                        // tree::Variable::StructDeclaration(_, _, _) => {}
                        // tree::Variable::StructReference(_) => {}
                        _ => unimplemented!()
                    }
                }
            }
            tree::Statement::Struct(def, _) => {

            },
            _ => panic!("Error in Statement"),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Variable {
    type Output = Option<BasicMetadataValueEnum<'ctx>>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Option<BasicMetadataValueEnum<'ctx>> {
        match self {
            tree::Variable::VarAssignment(var, expr) => {
                let var = var.emit(emitter).unwrap();
                let expr = expr.emit(emitter);
                emitter.builder.build_store(var.into_pointer_value(), expr.into_int_value())
                    .expect("Error in variable assignment");
                None
            }
            tree::Variable::VarReference(name, dims) => {
                for scope in emitter.scope.iter().rev() {
                    if let Some((ptr, ty)) = scope.get(name.deref().as_str()) {
                        let (ptr, ty) = (*ptr, *ty);
                        return if dims.is_empty() {
                            Some(emitter.builder.build_load(ty, ptr, name.deref()).unwrap().as_basic_value_enum().into())
                        } else {
                            let mut idx_vals = vec![emitter.context.i32_type().const_zero()];
                            idx_vals.extend(dims.deref().iter().map(|dim| dim.emit(emitter).into_int_value()));

                            Some(emitter.builder.build_load(
                                ty,
                                unsafe {
                                    emitter.builder.build_in_bounds_gep(ty, ptr, idx_vals.as_ref(), "index").unwrap()
                                },
                                name.deref()
                            ).unwrap().into())
                        }
                    }
                }
                None
            }
            tree::Variable::VarDeclaration(name, ty, dims) => {
                let ty = if dims.is_empty() {
                    match (*ty).deref() {
                        tree::Value::Integer(_) => emitter.context.i32_type().as_basic_type_enum(),
                        tree::Value::Float(_) => emitter.context.f32_type().as_basic_type_enum(),
                        _ => panic!("Type not supported"),
                    }
                } else {
                    let dims = dims.deref().iter().rev().map(|dim|
                        dim.emit(emitter).into_int_value()).collect::<Vec<IntValue>>();
                    dims.iter().fold(
                        emitter.context.i32_type().as_basic_type_enum(),
                        |acc, len| acc.array_type(len.get_zero_extended_constant().unwrap() as u32)
                            .as_basic_type_enum()
                    )
                };
                let alloca = emitter.builder.build_alloca(ty, name.deref()).unwrap();
                emitter.scope.last_mut().unwrap().insert(name.deref(), (alloca, ty));

                None
            }
            // tree::Variable::StructDefinition(name, vars) => {}
            // tree::Variable::StructDeclaration(structname, inst, members) => {}
            // tree::Variable::StructReference(vars) => {}
            _ => panic!("Error in Variable"),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Function {
    type Output = Option<BasicValueEnum<'ctx>>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Option<BasicValueEnum<'ctx>> {
        match self {
            tree::Function::FuncDeclaration(name, params, ret_ty, body) => {
                emitter.scope.push(HashMap::new());

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

                let ret_ty = match *ret_ty.deref() {
                    tree::Value::Integer(_) => emitter.context.i32_type().fn_type(paras_ty.as_ref(), false),
                    tree::Value::Null => emitter.context.void_type().fn_type(paras_ty.as_ref(), false),
                    _ => panic!("Error in Function"),
                };
                let func = emitter.module.add_function(name, ret_ty, None);
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
                emitter.scope.pop();
                None
            },
            tree::Function::FuncReference(name, params) => {
                if (*name).as_str().eq("printf") {
                    let args = params.iter().map(|param| param.emit(emitter).into()).collect::<Vec<BasicMetadataValueEnum>>();
                    emitter.emit_printf_call(args.as_slice());
                    return None;
                }

                let func = emitter.module.get_function((*name).as_str()).expect("Function undeclared");
                let llvm_args = params.iter().map(|param| param.emit(emitter).into()).collect::<Vec<BasicMetadataValueEnum>>();
                Some(emitter.builder.build_call(func, llvm_args.as_slice(), (*name).as_str()).unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap_or(emitter.context.i32_type().const_int(0, false).as_basic_value_enum()))
            },
            tree::Function::Error => panic!("Error in Function")
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Body {
    type Output = ();
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) {
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

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Expr {
    type Output = ();
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) {
        match self {
            tree::Expr::Return(expr, ..) => {
                if expr.eq(&tree::CompExpr::Value(tree::Value::Null)) {
                    emitter.builder.build_return(None).expect("Error in Expr");
                } else {
                    let ret = expr.emit(emitter);
                    emitter.builder.build_return(Some(&ret)).expect("Error in Expr");
                }
            },
            tree::Expr::FuncCall(function, _) => {
                function.emit(emitter);
            },
            // if, while, for, ...
            _ => unimplemented!(),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::CompExpr {
    type Output = BasicValueEnum<'ctx>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> BasicValueEnum<'ctx> {
        match self {
            tree::CompExpr::Value(n) => match n {
                tree::Value::Integer(n) => emitter.context.i32_type().const_int(*n as u64, false).as_basic_value_enum(),
                tree::Value::Float(n) => emitter.context.f32_type().const_float(*n as f64).as_basic_value_enum(),
                tree::Value::String(s) => emitter.emit_global_string(&mut s.to_owned(), "").as_basic_value_enum(),
                _ => panic!("Error in CompExpr"),
            }
            // CompExpr::Variable(_) => {}
            tree::CompExpr::FuncCall(function) => {
                function.emit(emitter).unwrap_or(
                    emitter.context.i32_type().const_int(0, false).as_basic_value_enum()
                )
            },
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
                            _ => panic!("Error in CompExpr"),
                        }
                    }
                    tree::BinaryOperator::Sub => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_sub(lhs, rhs.into_int_value(), "subtmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_sub(lhs, rhs.into_float_value(), "subtmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"),
                        }
                    }
                    tree::BinaryOperator::Mul => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_mul(lhs, rhs.into_int_value(), "multmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_mul(lhs, rhs.into_float_value(), "multmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"),
                        }
                    }
                    tree::BinaryOperator::Div => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_unsigned_div(lhs, rhs.into_int_value(), "divtmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_div(lhs, rhs.into_float_value(), "divtmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"),
                        }
                    }
                _ => panic!("Error in CompExpr"),
            }}
            _ => panic!("Error in CompExpr"),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::CondExpr {
    type Output = BasicValueEnum<'ctx>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> BasicValueEnum<'ctx> {
        match self {
            tree::CondExpr::Bool(b) => emitter.context.bool_type().const_int(*b as u64, false).as_basic_value_enum(),
            tree::CondExpr::UnaryCondition(op, expr) => {
                let expr = (*expr.deref()).emit(emitter);
                match op {
                    tree::UnaryOperator::Not => {
                        match expr {
                            _ => panic!("Error in CondExpr"),
                        }
                    }
                    _ => panic!("Error in CondExpr"),
                }
            }
            tree::CondExpr::BinaryCondition(lhs, op, rhs) => {
                let lhs = (*lhs.deref()).emit(emitter).into_int_value();
                let rhs = (*rhs.deref()).emit(emitter).into_int_value();
                match op {
                    tree::BinaryOperator::And =>
                        emitter.builder.build_and(lhs, rhs, "andtmp").unwrap().as_basic_value_enum(),
                    tree::BinaryOperator::Or =>
                        emitter.builder.build_or(lhs, rhs, "ortmp").unwrap().as_basic_value_enum(),
                    _ => panic!("Operator not supported in CondExpr"),
                }
            }
            tree::CondExpr::Condition(lhs, op, rhs) => {
                let lhs = (*lhs.deref()).emit(emitter);
                let rhs = (*rhs.deref()).emit(emitter);
                match op {
                    tree::JudgeOperator::GT => match lhs {
                        BasicValueEnum::IntValue(lhs) => emitter.builder.build_int_compare(inkwell::IntPredicate::SGT, lhs, rhs.into_int_value(), "gttmp").unwrap().as_basic_value_enum(),
                        BasicValueEnum::FloatValue(lhs) => emitter.builder.build_float_compare(inkwell::FloatPredicate::OGT, lhs, rhs.into_float_value(), "gttmp").unwrap().as_basic_value_enum(),
                        _ => panic!("Error in CondExpr"),
                    }
                    tree::JudgeOperator::GE => match lhs {
                        BasicValueEnum::IntValue(lhs) => emitter.builder.build_int_compare(inkwell::IntPredicate::SGE, lhs, rhs.into_int_value(), "getmp").unwrap().as_basic_value_enum(),
                        BasicValueEnum::FloatValue(lhs) => emitter.builder.build_float_compare(inkwell::FloatPredicate::OGE, lhs, rhs.into_float_value(), "getmp").unwrap().as_basic_value_enum(),
                        _ => panic!("Error in CondExpr"),
                    }
                    tree::JudgeOperator::LT => match lhs {
                        BasicValueEnum::IntValue(lhs) => emitter.builder.build_int_compare(inkwell::IntPredicate::SLT, lhs, rhs.into_int_value(), "lttmp").unwrap().as_basic_value_enum(),
                        BasicValueEnum::FloatValue(lhs) => emitter.builder.build_float_compare(inkwell::FloatPredicate::OLT, lhs, rhs.into_float_value(), "lttmp").unwrap().as_basic_value_enum(),
                        _ => panic!("Error in CondExpr"),
                    }
                    tree::JudgeOperator::LE => match lhs {
                        BasicValueEnum::IntValue(lhs) => emitter.builder.build_int_compare(inkwell::IntPredicate::SLE, lhs, rhs.into_int_value(), "letmp").unwrap().as_basic_value_enum(),
                        BasicValueEnum::FloatValue(lhs) => emitter.builder.build_float_compare(inkwell::FloatPredicate::OLE, lhs, rhs.into_float_value(), "letmp").unwrap().as_basic_value_enum(),
                        _ => panic!("Error in CondExpr"),
                    }
                    tree::JudgeOperator::EQ => match lhs {
                        BasicValueEnum::IntValue(lhs) => emitter.builder.build_int_compare(inkwell::IntPredicate::EQ, lhs, rhs.into_int_value(), "eqtmp").unwrap().as_basic_value_enum(),
                        BasicValueEnum::FloatValue(lhs) => emitter.builder.build_float_compare(inkwell::FloatPredicate::OEQ, lhs, rhs.into_float_value(), "eqtmp").unwrap().as_basic_value_enum(),
                        _ => panic!("Error in CondExpr"),
                    }
                    tree::JudgeOperator::NE => match lhs {
                        BasicValueEnum::IntValue(lhs) => emitter.builder.build_int_compare(inkwell::IntPredicate::NE, lhs, rhs.into_int_value(), "netmp").unwrap().as_basic_value_enum(),
                        BasicValueEnum::FloatValue(lhs) => emitter.builder.build_float_compare(inkwell::FloatPredicate::ONE, lhs, rhs.into_float_value(), "netmp").unwrap().as_basic_value_enum(),
                        _ => panic!("Error in CondExpr"),
                    }
                    _ => panic!("Error in CondExpr"),
                }
            }
            _ => panic!("Error in CondExpr"),
        }
    }
}
