use std::any::Any;
use std::collections::HashMap;
use std::ops::{Add, Deref};
use std::path::MAIN_SEPARATOR;
use inkwell::execution_engine::JitFunction;
use inkwell::AddressSpace;
use inkwell::types::{AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue, ArrayValue};
use crate::azuki::Loop;
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
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
        match self {
            tree::Statement::GlobalVariable(vars, _) => {
                for var in vars {
                    match var {
                        tree::Variable::VarAssignment(var, expr) => {
                            match var.as_ref() {
                                tree::Variable::VarDeclaration(name, ty, dims) => {
                                    // Dimensions reference
                                    let dims = dims.deref().iter().rev().map(|dim|
                                        dim.emit(emitter).into_int_value()).collect::<Vec<IntValue>>();

                                    let typ = if dims.is_empty() {
                                        match (*ty).deref() {
                                            tree::Value::Integer(_) => emitter.context.i32_type().as_basic_type_enum(),
                                            tree::Value::Float(_) => emitter.context.f32_type().as_basic_type_enum(),
                                            tree::Value::Pointer(_) => emitter.context.ptr_type(AddressSpace::default()).as_basic_type_enum(),
                                            _ => panic!("Type not supported"),
                                        }
                                    } else {
                                        dims.iter().fold(
                                            emitter.context.i32_type().as_basic_type_enum(),
                                            |acc, len| acc.array_type(len.get_zero_extended_constant().unwrap() as u32)
                                                .as_basic_type_enum()
                                        )
                                    };
                                    match typ {
                                        BasicTypeEnum::ArrayType(_) => {
                                            // Get the array values
                                            let assign_vals = expr.deref().iter().map(|expr| expr.emit(emitter)).collect::<Vec<BasicValueEnum>>();
                                            
                                            let mut dims = dims.iter();
                                            let top_size = dims.next().unwrap().get_zero_extended_constant().unwrap() as u32;

                                            let mut arrays = assign_vals
                                                .chunks(top_size as usize)
                                                .map(|a| emitter.context.i32_type().const_array(a.into_iter().map(|v| v.into_int_value()).collect::<Vec<IntValue>>().as_slice()))
                                                .collect::<Vec<ArrayValue>>();

                                            let mut typ_t = emitter.context.i32_type().array_type(top_size);

                                            // If it is a multidimensional array
                                            for dim in dims {
                                                println!("{:?}", dim);
                                                let size = dim.get_zero_extended_constant().unwrap() as u32;
                                                arrays = arrays
                                                    .chunks(size as usize)
                                                    .map(|a| typ_t.const_array(a))
                                                    .collect::<Vec<ArrayValue>>();
                                                typ_t = typ_t.array_type(size);
                                            }
                                            // Get the global variable and its pointer and its type (ArrayType if it is an array)
                                            let global = emitter.module.add_global(typ_t, Some(AddressSpace::default()), name.deref());
                                            global.set_initializer(&arrays.as_slice()[0]);
                                        },
                                        BasicTypeEnum::PointerType(_) => {
                                            unimplemented!()
                                        },
                                        BasicTypeEnum::StructType(_) => {
                                            unimplemented!()
                                        },
                                        _ => {
                                            unimplemented!()
                                        }
                                    }
                                }
                                _ => unimplemented!()
                            }
                        },
                        tree::Variable::VarDeclaration(name, ty, dims) => {
                            let dims = dims.deref().iter().rev().map(|dim|
                                dim.emit(emitter).into_int_value()).collect::<Vec<IntValue>>();
                            let typ = if dims.is_empty() {
                                match (*ty).deref() {
                                    tree::Value::Integer(_) => emitter.context.i32_type().as_basic_type_enum(),
                                    tree::Value::Float(_) => emitter.context.f32_type().as_basic_type_enum(),
                                    tree::Value::Pointer(_) => emitter.context.ptr_type(AddressSpace::default()).as_basic_type_enum(),
                                    _ => panic!("Type not supported"),
                                }
                            } else {
                                dims.iter().fold(
                                    emitter.context.i32_type().as_basic_type_enum(),
                                    |acc, len| acc.array_type(len
                                        .get_zero_extended_constant().unwrap() as u32)
                                        .as_basic_type_enum()
                                )
                            };
                            let global = emitter.module.add_global(typ, None, name.deref());
                            global.set_initializer(&typ.const_zero());
                        },
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
                let val: BasicValueEnum = expr.deref().first()?.emit(emitter).into(); // TODO: Array assignment
                match var.deref() {
                    tree::Variable::VarReference(name, _dims) => {
                        let ptr = emitter.get_var(name.deref()).unwrap().0;
                        emitter.builder.build_store(ptr, val).expect("");
                    },
                    _ => panic!("Error in Variable"),
                };
                None
            }
            tree::Variable::VarReference(name, dims) => {
                let (ptr, ty) = emitter.get_var(name.deref()).unwrap();
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
                    ty.deref().emit(emitter)?.get_type().into()
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
            tree::Variable::FormalParameter(_, ty, _) => ty.deref().emit(emitter),
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

                let paras_ty = params.iter().map(|param|
                    param.emit(emitter).unwrap().get_type().into()
                ).collect::<Vec<BasicMetadataTypeEnum>>();

                let ret_ty = match *ret_ty.deref() {
                    tree::Value::Integer(_) => emitter.context.i32_type().fn_type(paras_ty.as_ref(), false),
                    tree::Value::Float(_) => emitter.context.f32_type().fn_type(paras_ty.as_ref(), false),
                    tree::Value::Char(_) => emitter.context.i8_type().fn_type(paras_ty.as_ref(), false),
                    tree::Value::Pointer(_) => emitter.context.ptr_type(AddressSpace::default()).fn_type(paras_ty.as_ref(), false),
                    tree::Value::Null => emitter.context.void_type().fn_type(paras_ty.as_ref(), false),
                    _ => panic!("Error in Function"),
                };
                let func = emitter.module.add_function(name, ret_ty, None);
                let entry = emitter.context.append_basic_block(func, "entry");
                emitter.builder.position_at_end(entry);

                for (i, param) in params.iter().enumerate() {
                     if let tree::Variable::FormalParameter(name, _, _) = param {
                         let value = func.get_nth_param(i as u32).unwrap();
                         value.set_name(name);

                         let ptr = emitter.builder.build_alloca(value.get_type(), name).unwrap();
                         emitter.builder.build_store(ptr, value).expect("");
                         emitter.scope.last_mut().unwrap().insert(name, (ptr, value.get_type().into()));
                    } else {
                        panic!("Error in Function");
                    };
                }

                body.emit(emitter);
                let last_bb = emitter.builder.get_insert_block().unwrap();
                if last_bb.get_last_instruction().is_none() { // empty block, removed
                    last_bb.remove_from_function().expect("Cannot remove last block");
                }
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
            tree::Expr::If(if_expr, _) => {
                match if_expr {
                    tree::If::IfExpr(cond, if_body) => {
                        let cond = cond.emit(emitter);
                        let func = emitter.builder.get_insert_block().unwrap().get_parent().unwrap();
                        let then_bb = emitter.context.append_basic_block(func, "then");
                        let merge_bb = emitter.context.append_basic_block(func, "merge");
                        emitter.builder.build_conditional_branch(cond.into_int_value(), then_bb, merge_bb).expect("Error in IfExpr CondExpr");
                        emitter.builder.position_at_end(then_bb);
                        if_body.emit(emitter);
                        if emitter.no_terminator() {
                            emitter.builder.build_unconditional_branch(merge_bb).expect("Error in IfExpr If Body");
                        }

                        emitter.builder.position_at_end(merge_bb);
                    }
                    tree::If::IfElseExpr(cond, if_body, else_body) => {
                        let cond = cond.emit(emitter);
                        let func = emitter.builder.get_insert_block().unwrap().get_parent().unwrap();
                        let then_bb = emitter.context.append_basic_block(func, "then");
                        let else_bb = emitter.context.append_basic_block(func, "else");
                        let merge_bb = emitter.context.append_basic_block(func, "merge");

                        emitter.builder.build_conditional_branch(cond.into_int_value(), then_bb, else_bb).expect("Error in IfElseExpr CondExpr");
                        emitter.builder.position_at_end(then_bb);
                        if_body.emit(emitter);
                        if emitter.no_terminator() {
                            emitter.builder.build_unconditional_branch(merge_bb).expect("Error in IfElseExpr If Body");
                        }

                        emitter.builder.position_at_end(else_bb);
                        else_body.emit(emitter);
                        if emitter.no_terminator() {
                            emitter.builder.build_unconditional_branch(merge_bb).expect("Error in IfElseExpr Else Body");
                        }

                        emitter.builder.position_at_end(merge_bb);
                    }
                    _ => panic!("Error in Expr"),
                }
            }
            tree::Expr::Loop(loop_expr, _) => {
                match loop_expr {
                    tree::Loop::WhileExpr(cond, body) => {
                        let func = emitter.builder.get_insert_block().unwrap().get_parent().unwrap();
                        let cond_bb = emitter.context.append_basic_block(func, "cond");
                        let body_bb = emitter.context.append_basic_block(func, "body");
                        let merge_bb = emitter.context.append_basic_block(func, "merge");

                        emitter.builder.build_unconditional_branch(cond_bb).expect("Error in WhileLoop");
                        emitter.builder.position_at_end(cond_bb);
                        let cond = cond.emit(emitter);
                        emitter.builder.build_conditional_branch(cond.into_int_value(), body_bb, merge_bb).expect("Error in WhileLoop");

                        emitter.loops.push(Loop {
                            loop_head: cond_bb,
                            after_loop: merge_bb,
                        }); // Used to document the loop information
                        emitter.builder.position_at_end(body_bb);
                        body.emit(emitter);

                        if emitter.no_terminator() {
                            emitter.builder.build_unconditional_branch(cond_bb).expect("Error in WhileLoop");
                        }

                        emitter.loops.pop();
                        
                        emitter.builder.position_at_end(merge_bb);
                    }
                    tree::Loop::ForExpr(init, cond, step, body) => {
                        let func = emitter.builder.get_insert_block().unwrap().get_parent().unwrap();
                        let init_bb = emitter.context.append_basic_block(func, "init");
                        let cond_bb = emitter.context.append_basic_block(func, "cond");
                        let body_bb = emitter.context.append_basic_block(func, "body");
                        let step_bb = emitter.context.append_basic_block(func, "step");
                        let merge_bb = emitter.context.append_basic_block(func, "merge");

                        emitter.builder.build_unconditional_branch(init_bb).expect("Error in ForLoop");
                        emitter.builder.position_at_end(init_bb);
                        init.emit(emitter);
                        emitter.builder.build_unconditional_branch(cond_bb).expect("Error in ForLoop");

                        emitter.builder.position_at_end(cond_bb);
                        let cond = cond.emit(emitter);
                        emitter.builder.build_conditional_branch(cond.into_int_value(), body_bb, merge_bb).expect("Error in ForLoop");

                        emitter.loops.push(Loop {
                            loop_head: step_bb,
                            after_loop: merge_bb,
                        }); // Used to document the loop information

                        emitter.builder.position_at_end(body_bb);
                        body.emit(emitter);

                        if emitter.no_terminator() {
                            emitter.builder.build_unconditional_branch(step_bb).expect("Error in ForLoop");
                        }

                        emitter.builder.position_at_end(step_bb);
                        step.emit(emitter);
                        emitter.builder.build_unconditional_branch(cond_bb).expect("Error in ForLoop");

                        emitter.loops.pop();
                        
                        emitter.builder.position_at_end(merge_bb);
                    }
                    _ => panic!("Error in Expr"),
                }
            }
            tree::Expr::Break(_) => {
                let loop_info = emitter.loops.last().expect("Error in Break");
                emitter.builder.build_unconditional_branch(loop_info.after_loop).expect("Error in Break");
            }
            tree::Expr::Continue(_) => {
                let loop_info = emitter.loops.last().expect("Error in Continue");
                emitter.builder.build_unconditional_branch(loop_info.loop_head).expect("Error in Continue");
            }
            tree::Expr::Body(body, _ ) => {
                body.emit(emitter);
            }
            tree::Expr::Error => panic!("Error in Expr"),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::CompExpr {
    type Output = BasicValueEnum<'ctx>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast:'ctx
    {
        match self {
            tree::CompExpr::Value(val) => val.emit(emitter).unwrap(),
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
                            // let (ptr, ty) = (*ptr, *ty);
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
            tree::CondExpr::Error => panic!("Error in CondExpr"),
        }
    }
}

impl<'ast, 'ctx> Emit<'ast, 'ctx> for tree::Value {
    type Output = Option<BasicValueEnum<'ctx>>;
    fn emit(&'ast self, emitter: &mut Azuki<'ast, 'ctx>) -> Self::Output
        where 'ast: 'ctx
    {
        match self {
            tree::Value::Integer(n) => Some(emitter.context.i32_type().const_int(*n as u64, false).as_basic_value_enum()),
            tree::Value::Char(c) => Some(emitter.context.i8_type().const_int(*c as u64, false).as_basic_value_enum()),
            tree::Value::Float(f) => Some(emitter.context.f32_type().const_float(*f as f64).as_basic_value_enum()),
            tree::Value::String(s) => Some(emitter.emit_global_string(&mut s.to_owned(), "").as_basic_value_enum()),
            tree::Value::Pointer(_) => Some(emitter.context.ptr_type(AddressSpace::default()).const_null().as_basic_value_enum()),
            tree::Value::Null => None,
            _ => panic!("Error in Value"),
        }
    }
}
