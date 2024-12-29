use std::collections::HashMap;
use std::ops::Deref;
use inkwell::AddressSpace;
use inkwell::types::{BasicType};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue};
use spl_ast::tree;
use crate::azuki::Azuki;

pub trait Emit<'ast, 'ctx> {
    type Output;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output where 'ast:'ctx;
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Program {
    type Output = ();
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
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
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
        match self {
            tree::ProgramPart::Statement(stmt) => stmt.emit(emitter),
            tree::ProgramPart::Function(func) => { func.emit(emitter); },
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Statement {
    type Output = ();
    fn emit(&'ast self, _emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
        match self {
            tree::Statement::GlobalVariable(vars, ..) => {
                for var in vars {
                    match var {
                        // tree::Variable::VarAssignment(var, expr) => {
                        //
                        // },
                        // tree::Variable::VarReference(name, dims) => {
                        //
                        // },
                        // tree::Variable::VarDeclaration(name, ty, dims) => {
                        //
                        // },
                        // tree::Variable::StructDefinition(_, _) => {}
                        // tree::Variable::StructDeclaration(_, _, _) => {}
                        // tree::Variable::StructReference(_) => {}
                        _ => unimplemented!()
                    }
                }
            }
            // tree::Statement::Struct(def, _) => {
            //
            // },
            _ => panic!("Error in Statement"),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Variable {
    type Output = Option<BasicValueEnum<'ctx>>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
        match self {
            tree::Variable::VarAssignment(var, expr) => {
                let val = expr.deref().emit(emitter).into_int_value();
                match var.deref() {
                    tree::Variable::VarReference(name, _dims) => {
                        let ptr = *(emitter.get_var(name.deref()).unwrap().0);
                        emitter.builder.build_store(ptr, val).expect("");
                    },
                    _ => panic!("Error in Variable"),
                };
                None
            }
            tree::Variable::VarReference(name, dims) => {
                let (ptr, ty) = emitter.get_var(name.deref()).unwrap();
                let (ptr, ty) = (*ptr, *ty);
                if dims.is_empty() {
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
            tree::Variable::VarDeclaration(name, ty, dims) => {
                let ty = if dims.is_empty() {
                    match (*ty).deref() {
                        tree::Value::Integer(_) => emitter.context.i32_type().as_basic_type_enum(),
                        tree::Value::Float(_) => emitter.context.f32_type().as_basic_type_enum(),
                        tree::Value::Pointer(_) => emitter.context.ptr_type(AddressSpace::default()).as_basic_type_enum(),
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
                let new_var = emitter.builder.build_alloca(ty, name.deref()).unwrap();
                emitter.scope.last_mut().unwrap().insert(name.deref(), (new_var, ty));

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
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
        match self {
            tree::Function::FuncDeclaration(name, params, ret_ty, body) => {
                emitter.scope.push(HashMap::new());

                let mut paras_ty = Vec::new();
                for param in params {
                    if let tree::Variable::FormalParameter(_, ty, _) = param {
                        match *ty.deref() {
                            tree::Value::Integer(_) => paras_ty.push(emitter.context.i32_type().into()),
                            tree::Value::Float(_) => paras_ty.push(emitter.context.f32_type().into()),
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
                let args = params.iter().map(|param| param.emit(emitter).into()).collect::<Vec<BasicMetadataValueEnum>>();
                if (*name).as_str().eq("printf") {
                    emitter.emit_printf_call(args.as_slice());
                    return None;
                } else if (*name).as_str().eq("scanf") {
                    emitter.emit_scanf_call(args.as_slice());
                    return None;
                }

                let func = emitter.module.get_function((*name).as_str()).expect("Function undeclared");
                let args = params.iter().map(|param| param.emit(emitter).into()).collect::<Vec<BasicMetadataValueEnum>>();
                Some(emitter.builder.build_call(func, args.as_slice(), (*name).as_str()).unwrap()
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
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
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
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
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
            tree::Expr::VarManagement(vars, _) => {
                let _ = vars.iter().map(|var| var.emit(emitter)).collect::<Vec<Option<BasicValueEnum>>>();
            },
            // if, while, for, ...
            _ => unimplemented!(),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::CompExpr {
    type Output = BasicValueEnum<'ctx>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
        match self {
            tree::CompExpr::Value(n) => match n {
                tree::Value::Integer(n) => emitter.context.i32_type().const_int(*n as u64, false).as_basic_value_enum(),
                tree::Value::Char(c) => emitter.context.i8_type().const_int(*c as u64, false).as_basic_value_enum(),
                tree::Value::Float(f) => emitter.context.f32_type().const_float(*f as f64).as_basic_value_enum(),
                tree::Value::String(s) => emitter.emit_global_string(&mut s.to_owned(), "").as_basic_value_enum(),
                _ => panic!("Error in CompExpr"),
            }
            tree::CompExpr::Variable(var) => var.emit(emitter).unwrap(),
            tree::CompExpr::FuncCall(function) => {
                function.emit(emitter).unwrap_or(
                    emitter.context.i32_type().const_int(0, false).as_basic_value_enum()
                )
            },
            tree::CompExpr::UnaryOperation(op, expr) => {
                let var = if let tree::CompExpr::Variable(var) = expr.deref() {
                    var
                } else {
                    panic!("Must be a pointer");
                };
                match op {
                    tree::UnaryOperator::Ref => {
                        let ptr = if let tree::Variable::VarReference(name, dims) = var {
                            let (ptr, ty) = emitter.get_var(name.deref()).unwrap();
                            let (ptr, ty) = (*ptr, *ty);
                            if dims.is_empty() {
                                ptr
                            } else {
                                let mut idx_vals = vec![emitter.context.i32_type().const_zero()];
                                idx_vals.extend(dims.deref().iter().map(|dim| dim.emit(emitter).into_int_value()));
                                unsafe {
                                    emitter.builder.build_in_bounds_gep(ty, ptr, idx_vals.as_ref(), "index").unwrap()
                                }
                            }
                        } else {
                            panic!("Must be a pointer");
                        };
                        ptr.as_basic_value_enum()
                    }
                    tree::UnaryOperator::Deref => {
                        var.emit(emitter).unwrap()
                    }
                    _ => panic!("Operator not supported in CompExpr"),
                }
            }
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
                    tree::BinaryOperator::Mod => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_int_signed_rem(lhs, rhs.into_int_value(), "modtmp").unwrap().as_basic_value_enum(),
                            BasicValueEnum::FloatValue(lhs) =>
                                emitter.builder.build_float_rem(lhs, rhs.into_float_value(), "modtmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"),
                        }
                    }
                    tree::BinaryOperator::BitwiseAnd => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_and(lhs, rhs.into_int_value(), "andtmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"),
                        }
                    }
                    tree::BinaryOperator::BitwiseOr => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_or(lhs, rhs.into_int_value(), "ortmp").unwrap().as_basic_value_enum(),
                            _ => panic!("Error in CompExpr"),
                        }
                    }
                    tree::BinaryOperator::BitwiseXor => {
                        match lhs {
                            BasicValueEnum::IntValue(lhs) =>
                                emitter.builder.build_xor(lhs, rhs.into_int_value(), "xortmp").unwrap().as_basic_value_enum(),
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
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
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
