// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Core IR → Cranelift IR code generation.
//!
//! Translates `CoreModuleChirho` bindings into Cranelift IR functions,
//! builds an object module, and emits a native object file.
//!
//! ## Pipeline
//!
//! ```text
//! CoreModuleChirho
//!   └─ for each CoreBindingChirho
//!        ├─ count_params_chirho   → arity
//!        ├─ declare_function      → FuncId
//!        ├─ lower_binding_chirho  → Cranelift IR
//!        └─ define_function       → object module
//! ObjectModule::finish() → .o bytes
//! ```

use std::collections::{HashMap, HashSet};

use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{
    AbiParam as AbiParamChirho, Function as ClFunctionChirho, InstBuilder as _,
};
use cranelift_codegen::isa as cl_isa_chirho;
use cranelift_codegen::settings as cl_settings_chirho;
use cranelift_frontend::{
    FunctionBuilder as FuncBuilderChirho, FunctionBuilderContext as FuncBuilderCtxChirho,
};
use cranelift_module::{Linkage as LinkageChirho, Module as ModuleTraitChirho};
use cranelift_object::{ObjectBuilder as ObjBuilderChirho, ObjectModule as ObjModuleChirho};

use haskelujah_core_chirho::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use crate::lower_chirho::{
    emit_partial_application_closure_with_alloc_ref_chirho, ensure_i64_chirho,
    lower_tail_expr_chirho, LowerCtxChirho, PapWrapperKeyChirho, TailLowerOutcomeChirho,
    VarEnvChirho,
};
use crate::{NativeObjectChirho, TargetConfigChirho};

/// Compile a `CoreModuleChirho` to a native object file via Cranelift,
/// with dictionary elision and reachability filtering from `main`.
pub fn compile_core_to_object_executable_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &TargetConfigChirho,
) -> Result<NativeObjectChirho, String> {
    let mut filtered_module_chirho =
        haskelujah_core_chirho::elide_dicts_and_filter_chirho(module_chirho);
    restore_selector_bindings_chirho(module_chirho, &mut filtered_module_chirho);
    compile_core_to_object_inner_chirho(&filtered_module_chirho, config_chirho, true)
}

/// Compile a `CoreModuleChirho` to a native object file via Cranelift.
pub fn compile_core_to_object_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &TargetConfigChirho,
) -> Result<NativeObjectChirho, String> {
    compile_core_to_object_inner_chirho(module_chirho, config_chirho, false)
}

fn compile_core_to_object_inner_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &TargetConfigChirho,
    executable_mode_chirho: bool,
) -> Result<NativeObjectChirho, String> {
    // ── Build Cranelift ISA from target triple ─────────────────────────────
    let mut flag_builder_chirho = cl_settings_chirho::builder();
    // Enable PIC for macOS ARM64 (required for linking with libc)
    cranelift_codegen::settings::Configurable::set(&mut flag_builder_chirho, "is_pic", "true")
        .map_err(|e_chirho| format!("failed to set is_pic: {e_chirho}"))?;
    let isa_builder_chirho =
        cl_isa_chirho::lookup_by_name(&config_chirho.triple_chirho).map_err(|e_chirho| {
            format!(
                "unsupported target triple '{}': {}",
                config_chirho.triple_chirho, e_chirho
            )
        })?;
    let flags_chirho = cl_settings_chirho::Flags::new(flag_builder_chirho);
    let isa_chirho = isa_builder_chirho
        .finish(flags_chirho)
        .map_err(|e_chirho| format!("failed to build ISA: {e_chirho}"))?;

    // ── Create object module ───────────────────────────────────────────────
    let obj_builder_chirho = ObjBuilderChirho::new(
        isa_chirho,
        "haskelujah_module_chirho",
        cranelift_module::default_libcall_names(),
    )
    .map_err(|e_chirho| format!("failed to create object builder: {e_chirho}"))?;
    let mut obj_module_chirho = ObjModuleChirho::new(obj_builder_chirho);

    let mut fb_ctx_chirho = FuncBuilderCtxChirho::new();

    // ── Pass 1: Declare all functions ──────────────────────────────────────
    // Build a map from CoreId → (FuncId, arity) for direct calls.
    let mut func_decl_map_chirho: HashMap<
        haskelujah_core_chirho::expr_chirho::CoreIdChirho,
        (cranelift_module::FuncId, usize),
    > = HashMap::new();

    for binding_chirho in &module_chirho.bindings_chirho {
        let name_chirho = &binding_chirho.binder_chirho.name_chirho;
        let (param_binders_chirho, _body_chirho) = peel_lambdas_chirho(&binding_chirho.rhs_chirho);
        let param_count_chirho = param_binders_chirho.len();

        let mut sig_chirho = obj_module_chirho.make_signature();
        sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        for _ in 0..param_count_chirho {
            sig_chirho
                .params
                .push(AbiParamChirho::new(cl_types_chirho::I64));
        }

        let (symbol_name_chirho, linkage_chirho) =
            if executable_mode_chirho && name_chirho == "main" {
                ("haskelujah_main", LinkageChirho::Local)
            } else if name_chirho == "main" {
                ("main", LinkageChirho::Export)
            } else {
                (name_chirho.as_str(), LinkageChirho::Local)
            };
        let func_id_chirho = obj_module_chirho
            .declare_function(symbol_name_chirho, linkage_chirho, &sig_chirho)
            .map_err(|e_chirho| {
                format!("failed to declare function '{symbol_name_chirho}': {e_chirho}")
            })?;
        func_decl_map_chirho.insert(
            binding_chirho.binder_chirho.id_chirho,
            (func_id_chirho, param_count_chirho),
        );
    }

    // ── Import RTS/native helpers for Prelude IO ────────────────────────
    let (
        put_str_ln_func_id_chirho,
        print_int_func_id_chirho,
        append_str_func_id_chirho,
        alloc_func_id_chirho,
        show_int_func_id_chirho,
        show_bool_func_id_chirho,
        show_char_func_id_chirho,
        show_float_func_id_chirho,
        put_str_func_id_chirho,
        get_line_func_id_chirho,
        write_file_func_id_chirho,
        read_file_func_id_chirho,
        unpack_string_func_id_chirho,
        main_with_large_stack_func_id_chirho,
    ) = {
        let mut put_str_ln_sig_chirho = obj_module_chirho.make_signature();
        put_str_ln_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        put_str_ln_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let put_str_ln_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_put_str_ln_chirho",
                LinkageChirho::Import,
                &put_str_ln_sig_chirho,
            )
            .ok();

        let mut print_int_sig_chirho = obj_module_chirho.make_signature();
        print_int_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        print_int_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let print_int_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_print_int_chirho",
                LinkageChirho::Import,
                &print_int_sig_chirho,
            )
            .ok();

        let mut append_str_sig_chirho = obj_module_chirho.make_signature();
        append_str_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        append_str_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        append_str_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let append_str_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_append_str_chirho",
                LinkageChirho::Import,
                &append_str_sig_chirho,
            )
            .ok();

        let mut alloc_sig_chirho = obj_module_chirho.make_signature();
        alloc_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        alloc_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let alloc_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_alloc_chirho",
                LinkageChirho::Import,
                &alloc_sig_chirho,
            )
            .ok();

        // show_int: i64 → i64 (pointer to NUL-terminated string)
        let show_int_sig_chirho = put_str_ln_sig_chirho.clone(); // same sig: i64 → i64
        let show_int_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_show_int_chirho",
                LinkageChirho::Import,
                &show_int_sig_chirho,
            )
            .ok();

        let show_bool_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_show_bool_chirho",
                LinkageChirho::Import,
                &show_int_sig_chirho,
            )
            .ok();

        let show_char_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_show_char_chirho",
                LinkageChirho::Import,
                &show_int_sig_chirho,
            )
            .ok();

        let show_float_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_show_float_chirho",
                LinkageChirho::Import,
                &show_int_sig_chirho,
            )
            .ok();

        // putStr: same sig as putStrLn (i64 → i64)
        let put_str_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_put_str_chirho",
                LinkageChirho::Import,
                &put_str_ln_sig_chirho,
            )
            .ok();

        // getLine: () → i64 (returns heap string pointer)
        let mut get_line_sig_chirho = obj_module_chirho.make_signature();
        get_line_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let get_line_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_get_line_chirho",
                LinkageChirho::Import,
                &get_line_sig_chirho,
            )
            .ok();

        let mut write_file_sig_chirho = obj_module_chirho.make_signature();
        write_file_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        write_file_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        write_file_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let write_file_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_write_file_chirho",
                LinkageChirho::Import,
                &write_file_sig_chirho,
            )
            .ok();

        let mut read_file_sig_chirho = obj_module_chirho.make_signature();
        read_file_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        read_file_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let read_file_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_read_file_chirho",
                LinkageChirho::Import,
                &read_file_sig_chirho,
            )
            .ok();

        // unpack_string: same sig as read_file (i64 → i64)
        let unpack_string_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_unpack_string_chirho",
                LinkageChirho::Import,
                &read_file_sig_chirho,
            )
            .ok();

        let mut main_with_large_stack_sig_chirho = obj_module_chirho.make_signature();
        main_with_large_stack_sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        main_with_large_stack_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I64));
        let main_with_large_stack_func_id_chirho = obj_module_chirho
            .declare_function(
                "haskelujah_main_with_large_stack_chirho",
                LinkageChirho::Import,
                &main_with_large_stack_sig_chirho,
            )
            .ok();

        (
            put_str_ln_func_id_chirho,
            print_int_func_id_chirho,
            append_str_func_id_chirho,
            alloc_func_id_chirho,
            show_int_func_id_chirho,
            show_bool_func_id_chirho,
            show_char_func_id_chirho,
            show_float_func_id_chirho,
            put_str_func_id_chirho,
            get_line_func_id_chirho,
            write_file_func_id_chirho,
            read_file_func_id_chirho,
            unpack_string_func_id_chirho,
            main_with_large_stack_func_id_chirho,
        )
    };

    // ── Pre-scan: embed string literals in data section ────────────────────
    let mut string_data_ids_chirho: HashMap<String, cranelift_module::DataId> = HashMap::new();
    // Always include printf format strings
    let builtin_strings_chirho = ["%ld\n", "%s\n", "True", "False"];
    {
        let mut string_counter_chirho = 0u32;
        let add_string_chirho = |s_chirho: &str,
                                 obj_mod_chirho: &mut ObjModuleChirho,
                                 map_chirho: &mut HashMap<String, cranelift_module::DataId>,
                                 counter_chirho: &mut u32| {
            if !map_chirho.contains_key(s_chirho) {
                let name_chirho = format!(".str.{}", counter_chirho);
                *counter_chirho += 1;
                let mut data_desc_chirho = cranelift_module::DataDescription::new();
                let mut bytes_chirho = s_chirho.as_bytes().to_vec();
                bytes_chirho.push(0);
                data_desc_chirho.define(bytes_chirho.into_boxed_slice());
                if let Ok(data_id_chirho) =
                    obj_mod_chirho.declare_data(&name_chirho, LinkageChirho::Local, false, false)
                {
                    let _ = obj_mod_chirho.define_data(data_id_chirho, &data_desc_chirho);
                    map_chirho.insert(s_chirho.to_string(), data_id_chirho);
                }
            }
        };
        for s_chirho in &builtin_strings_chirho {
            add_string_chirho(
                s_chirho,
                &mut obj_module_chirho,
                &mut string_data_ids_chirho,
                &mut string_counter_chirho,
            );
        }
        collect_string_literals_chirho(module_chirho, &mut |s_chirho: &str| {
            add_string_chirho(
                s_chirho,
                &mut obj_module_chirho,
                &mut string_data_ids_chirho,
                &mut string_counter_chirho,
            );
        });
    }

    // ── Pass 2: Define all function bodies ─────────────────────────────────
    for binding_chirho in &module_chirho.bindings_chirho {
        lower_binding_chirho(
            &mut obj_module_chirho,
            &mut fb_ctx_chirho,
            binding_chirho,
            &func_decl_map_chirho,
            module_chirho,
            put_str_ln_func_id_chirho,
            print_int_func_id_chirho,
            append_str_func_id_chirho,
            alloc_func_id_chirho,
            show_int_func_id_chirho,
            show_bool_func_id_chirho,
            show_char_func_id_chirho,
            show_float_func_id_chirho,
            put_str_func_id_chirho,
            get_line_func_id_chirho,
            write_file_func_id_chirho,
            read_file_func_id_chirho,
            unpack_string_func_id_chirho,
            &string_data_ids_chirho,
        )?;
    }

    if executable_mode_chirho {
        if let Some(main_binding_chirho) = module_chirho
            .bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "main")
        {
            if let (Some((main_func_id_chirho, _)), Some(main_with_large_stack_func_id_chirho)) = (
                func_decl_map_chirho.get(&main_binding_chirho.binder_chirho.id_chirho),
                main_with_large_stack_func_id_chirho,
            ) {
                define_native_main_wrapper_chirho(
                    &mut obj_module_chirho,
                    &mut fb_ctx_chirho,
                    *main_func_id_chirho,
                    main_with_large_stack_func_id_chirho,
                )?;
            }
        }
    }

    // ── Finalize and emit object bytes ─────────────────────────────────────
    let product_chirho = obj_module_chirho.finish();
    let object_bytes_chirho = product_chirho
        .emit()
        .map_err(|e_chirho| format!("failed to emit object: {e_chirho}"))?;

    Ok(NativeObjectChirho {
        object_bytes_chirho,
        target_triple_chirho: config_chirho.triple_chirho.clone(),
    })
}

/// Map of CoreId → (FuncId, arity) for declared top-level functions.
type FuncDeclMapChirho =
    HashMap<haskelujah_core_chirho::expr_chirho::CoreIdChirho, (cranelift_module::FuncId, usize)>;

#[derive(Debug, Clone)]
struct LocalLiftedBindingChirho {
    binder_chirho: BinderChirho,
    rhs_chirho: CoreExprChirho,
    symbol_name_chirho: String,
    arity_chirho: usize,
    captured_ids_chirho: Vec<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
}

#[derive(Debug, Clone)]
struct PapWrapperBindingChirho {
    target_id_chirho: haskelujah_core_chirho::expr_chirho::CoreIdChirho,
    target_func_id_chirho: cranelift_module::FuncId,
    applied_arg_count_chirho: usize,
    remaining_arity_chirho: usize,
    hidden_capture_count_chirho: usize,
    symbol_name_chirho: String,
}

fn is_runtime_lambda_chirho(expr_chirho: &CoreExprChirho) -> bool {
    !peel_lambdas_chirho(expr_chirho).0.is_empty()
}

#[derive(Clone, Copy)]
enum InlineLambdaRewriteModeChirho {
    NormalChirho,
    PreserveLambdaRootChirho,
}

fn fresh_inline_lambda_binder_chirho(next_inline_id_chirho: &mut u32) -> BinderChirho {
    let inline_id_chirho = CoreIdChirho(*next_inline_id_chirho);
    *next_inline_id_chirho += 1;
    BinderChirho {
        id_chirho: inline_id_chirho,
        name_chirho: format!("haskelujah_inline_lambda_{}_chirho", inline_id_chirho.0),
        // The native backends only need the binder id/name here; the precise
        // Core function type is irrelevant once runtime lowering starts.
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn rewrite_inline_lambda_values_chirho(
    expr_chirho: &CoreExprChirho,
    next_inline_id_chirho: &mut u32,
    mode_chirho: InlineLambdaRewriteModeChirho,
) -> CoreExprChirho {
    if matches!(mode_chirho, InlineLambdaRewriteModeChirho::NormalChirho)
        && is_runtime_lambda_chirho(expr_chirho)
    {
        let binder_chirho = fresh_inline_lambda_binder_chirho(next_inline_id_chirho);
        let lifted_rhs_chirho = rewrite_inline_lambda_values_chirho(
            expr_chirho,
            next_inline_id_chirho,
            InlineLambdaRewriteModeChirho::PreserveLambdaRootChirho,
        );
        return CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(binder_chirho.clone(), lifted_rhs_chirho)],
            body_chirho: Box::new(CoreExprChirho::VarChirho(binder_chirho.id_chirho)),
        };
    }

    match expr_chirho {
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => CoreExprChirho::AppChirho {
            fun_chirho: Box::new(rewrite_inline_lambda_values_chirho(
                fun_chirho,
                next_inline_id_chirho,
                InlineLambdaRewriteModeChirho::NormalChirho,
            )),
            arg_chirho: Box::new(rewrite_inline_lambda_values_chirho(
                arg_chirho,
                next_inline_id_chirho,
                InlineLambdaRewriteModeChirho::NormalChirho,
            )),
        },
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            let body_mode_chirho = if matches!(
                mode_chirho,
                InlineLambdaRewriteModeChirho::PreserveLambdaRootChirho
            ) {
                InlineLambdaRewriteModeChirho::PreserveLambdaRootChirho
            } else {
                InlineLambdaRewriteModeChirho::NormalChirho
            };
            CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho.clone(),
                body_chirho: Box::new(rewrite_inline_lambda_values_chirho(
                    body_chirho,
                    next_inline_id_chirho,
                    body_mode_chirho,
                )),
            }
        }
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => CoreExprChirho::LetChirho {
            rec_chirho: *rec_chirho,
            binds_chirho: binds_chirho
                .iter()
                .map(|(binder_chirho, rhs_chirho)| {
                    let rhs_mode_chirho = if is_runtime_lambda_chirho(rhs_chirho) {
                        InlineLambdaRewriteModeChirho::PreserveLambdaRootChirho
                    } else {
                        InlineLambdaRewriteModeChirho::NormalChirho
                    };
                    (
                        binder_chirho.clone(),
                        rewrite_inline_lambda_values_chirho(
                            rhs_chirho,
                            next_inline_id_chirho,
                            rhs_mode_chirho,
                        ),
                    )
                })
                .collect(),
            body_chirho: Box::new(rewrite_inline_lambda_values_chirho(
                body_chirho,
                next_inline_id_chirho,
                InlineLambdaRewriteModeChirho::NormalChirho,
            )),
        },
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(rewrite_inline_lambda_values_chirho(
                scrutinee_chirho,
                next_inline_id_chirho,
                InlineLambdaRewriteModeChirho::NormalChirho,
            )),
            bind_chirho: bind_chirho.clone(),
            result_ty_chirho: result_ty_chirho.clone(),
            alts_chirho: alts_chirho
                .iter()
                .map(
                    |alt_chirho| haskelujah_core_chirho::expr_chirho::CoreAltChirho {
                        con_chirho: alt_chirho.con_chirho.clone(),
                        binders_chirho: alt_chirho.binders_chirho.clone(),
                        rhs_chirho: rewrite_inline_lambda_values_chirho(
                            &alt_chirho.rhs_chirho,
                            next_inline_id_chirho,
                            InlineLambdaRewriteModeChirho::NormalChirho,
                        ),
                    },
                )
                .collect(),
        },
        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => {
            let body_mode_chirho = if matches!(
                mode_chirho,
                InlineLambdaRewriteModeChirho::PreserveLambdaRootChirho
            ) {
                InlineLambdaRewriteModeChirho::PreserveLambdaRootChirho
            } else {
                InlineLambdaRewriteModeChirho::NormalChirho
            };
            CoreExprChirho::TyLamChirho {
                ty_var_chirho: ty_var_chirho.clone(),
                body_chirho: Box::new(rewrite_inline_lambda_values_chirho(
                    body_chirho,
                    next_inline_id_chirho,
                    body_mode_chirho,
                )),
            }
        }
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(rewrite_inline_lambda_values_chirho(
                inner_chirho,
                next_inline_id_chirho,
                InlineLambdaRewriteModeChirho::NormalChirho,
            )),
            ty_chirho: ty_chirho.clone(),
        },
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|arg_chirho| {
                    rewrite_inline_lambda_values_chirho(
                        arg_chirho,
                        next_inline_id_chirho,
                        InlineLambdaRewriteModeChirho::NormalChirho,
                    )
                })
                .collect(),
        },
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|arg_chirho| {
                    rewrite_inline_lambda_values_chirho(
                        arg_chirho,
                        next_inline_id_chirho,
                        InlineLambdaRewriteModeChirho::NormalChirho,
                    )
                })
                .collect(),
        },
    }
}

fn max_core_id_in_expr_chirho(expr_chirho: &CoreExprChirho, max_id_chirho: &mut u32) {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            *max_id_chirho = (*max_id_chirho).max(id_chirho.0);
        }
        CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            max_core_id_in_expr_chirho(fun_chirho, max_id_chirho);
            max_core_id_in_expr_chirho(arg_chirho, max_id_chirho);
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
            max_core_id_in_expr_chirho(body_chirho, max_id_chirho);
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (binder_chirho, rhs_chirho) in binds_chirho {
                *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                max_core_id_in_expr_chirho(rhs_chirho, max_id_chirho);
            }
            max_core_id_in_expr_chirho(body_chirho, max_id_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            *max_id_chirho = (*max_id_chirho).max(bind_chirho.id_chirho.0);
            max_core_id_in_expr_chirho(scrutinee_chirho, max_id_chirho);
            for alt_chirho in alts_chirho {
                for binder_chirho in &alt_chirho.binders_chirho {
                    *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                }
                max_core_id_in_expr_chirho(&alt_chirho.rhs_chirho, max_id_chirho);
            }
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            max_core_id_in_expr_chirho(body_chirho, max_id_chirho);
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            max_core_id_in_expr_chirho(expr_chirho, max_id_chirho);
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                max_core_id_in_expr_chirho(arg_chirho, max_id_chirho);
            }
        }
    }
}

fn next_inline_core_id_chirho(module_chirho: &CoreModuleChirho) -> u32 {
    let mut max_id_chirho = module_chirho
        .names_chirho
        .keys()
        .map(|id_chirho| id_chirho.0)
        .max()
        .unwrap_or(0);
    for binding_chirho in &module_chirho.bindings_chirho {
        max_id_chirho = max_id_chirho.max(binding_chirho.binder_chirho.id_chirho.0);
        max_core_id_in_expr_chirho(&binding_chirho.rhs_chirho, &mut max_id_chirho);
    }
    max_id_chirho.saturating_add(1)
}

fn collect_local_lambda_bindings_chirho(
    owner_id_chirho: haskelujah_core_chirho::expr_chirho::CoreIdChirho,
    expr_chirho: &CoreExprChirho,
    next_local_idx_chirho: &mut u32,
    lifted_bindings_chirho: &mut Vec<LocalLiftedBindingChirho>,
) {
    match expr_chirho {
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (binder_chirho, rhs_chirho) in binds_chirho {
                if is_runtime_lambda_chirho(rhs_chirho) {
                    let symbol_name_chirho = format!(
                        "haskelujah_local_{}_{}_chirho",
                        owner_id_chirho.0, *next_local_idx_chirho
                    );
                    *next_local_idx_chirho += 1;
                    let arity_chirho = peel_lambdas_chirho(rhs_chirho).0.len();
                    lifted_bindings_chirho.push(LocalLiftedBindingChirho {
                        binder_chirho: binder_chirho.clone(),
                        rhs_chirho: rhs_chirho.clone(),
                        symbol_name_chirho,
                        arity_chirho,
                        captured_ids_chirho: Vec::new(),
                    });
                }
                collect_local_lambda_bindings_chirho(
                    owner_id_chirho,
                    rhs_chirho,
                    next_local_idx_chirho,
                    lifted_bindings_chirho,
                );
            }
            collect_local_lambda_bindings_chirho(
                owner_id_chirho,
                body_chirho,
                next_local_idx_chirho,
                lifted_bindings_chirho,
            );
        }
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            collect_local_lambda_bindings_chirho(
                owner_id_chirho,
                fun_chirho,
                next_local_idx_chirho,
                lifted_bindings_chirho,
            );
            collect_local_lambda_bindings_chirho(
                owner_id_chirho,
                arg_chirho,
                next_local_idx_chirho,
                lifted_bindings_chirho,
            );
        }
        CoreExprChirho::LamChirho { body_chirho, .. }
        | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            collect_local_lambda_bindings_chirho(
                owner_id_chirho,
                body_chirho,
                next_local_idx_chirho,
                lifted_bindings_chirho,
            );
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            collect_local_lambda_bindings_chirho(
                owner_id_chirho,
                scrutinee_chirho,
                next_local_idx_chirho,
                lifted_bindings_chirho,
            );
            for alt_chirho in alts_chirho {
                collect_local_lambda_bindings_chirho(
                    owner_id_chirho,
                    &alt_chirho.rhs_chirho,
                    next_local_idx_chirho,
                    lifted_bindings_chirho,
                );
            }
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            collect_local_lambda_bindings_chirho(
                owner_id_chirho,
                expr_chirho,
                next_local_idx_chirho,
                lifted_bindings_chirho,
            );
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                collect_local_lambda_bindings_chirho(
                    owner_id_chirho,
                    arg_chirho,
                    next_local_idx_chirho,
                    lifted_bindings_chirho,
                );
            }
        }
        CoreExprChirho::LitChirho(_) | CoreExprChirho::VarChirho(_) => {}
    }
}

fn declare_function_signature_chirho(
    module_chirho: &mut ObjModuleChirho,
    param_count_chirho: usize,
) -> cranelift_codegen::ir::Signature {
    let mut sig_chirho = module_chirho.make_signature();
    sig_chirho
        .returns
        .push(AbiParamChirho::new(cl_types_chirho::I64));
    for _ in 0..param_count_chirho {
        sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
    }
    sig_chirho
}

fn declare_local_lifted_bindings_chirho(
    module_chirho: &mut ObjModuleChirho,
    lifted_bindings_chirho: &[LocalLiftedBindingChirho],
) -> Result<FuncDeclMapChirho, String> {
    let mut local_decl_map_chirho = HashMap::new();
    for lifted_binding_chirho in lifted_bindings_chirho {
        let sig_chirho = declare_function_signature_chirho(
            module_chirho,
            lifted_binding_chirho.arity_chirho + lifted_binding_chirho.captured_ids_chirho.len(),
        );
        let func_id_chirho = module_chirho
            .declare_function(
                &lifted_binding_chirho.symbol_name_chirho,
                LinkageChirho::Local,
                &sig_chirho,
            )
            .map_err(|e_chirho| {
                format!(
                    "failed to declare lifted local function '{}': {e_chirho}",
                    lifted_binding_chirho.symbol_name_chirho
                )
            })?;
        local_decl_map_chirho.insert(
            lifted_binding_chirho.binder_chirho.id_chirho,
            (func_id_chirho, lifted_binding_chirho.arity_chirho),
        );
    }
    Ok(local_decl_map_chirho)
}

fn collect_partial_application_sites_chirho(
    owner_id_chirho: haskelujah_core_chirho::expr_chirho::CoreIdChirho,
    _exprs_chirho: &[&CoreExprChirho],
    imported_decl_map_chirho: &FuncDeclMapChirho,
    lifted_capture_ids_map_chirho: &HashMap<
        haskelujah_core_chirho::expr_chirho::CoreIdChirho,
        Vec<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
    >,
) -> Vec<PapWrapperBindingChirho> {
    let mut pap_wrappers_by_key_chirho = HashMap::new();
    for (target_id_chirho, (target_func_id_chirho, total_arity_chirho)) in imported_decl_map_chirho
    {
        let hidden_capture_count_chirho = lifted_capture_ids_map_chirho
            .get(target_id_chirho)
            .map_or(0, Vec::len);
        if *total_arity_chirho == 0
            || (*total_arity_chirho == 1 && hidden_capture_count_chirho == 0)
        {
            continue;
        }
        for applied_arg_count_chirho in 0..*total_arity_chirho {
            pap_wrappers_by_key_chirho.insert(
                (*target_id_chirho, applied_arg_count_chirho),
                PapWrapperBindingChirho {
                    target_id_chirho: *target_id_chirho,
                    target_func_id_chirho: *target_func_id_chirho,
                    applied_arg_count_chirho,
                    remaining_arity_chirho: total_arity_chirho - applied_arg_count_chirho,
                    hidden_capture_count_chirho,
                    symbol_name_chirho: format!(
                        "haskelujah_pap_{}_{}_{}_chirho",
                        owner_id_chirho.0, target_id_chirho.0, applied_arg_count_chirho
                    ),
                },
            );
        }
    }
    let mut pap_wrappers_chirho: Vec<_> = pap_wrappers_by_key_chirho.into_values().collect();
    pap_wrappers_chirho.sort_by_key(|wrapper_chirho| {
        (
            wrapper_chirho.target_id_chirho.0,
            wrapper_chirho.applied_arg_count_chirho,
        )
    });
    pap_wrappers_chirho
}

fn declare_pap_wrappers_chirho(
    module_chirho: &mut ObjModuleChirho,
    pap_wrappers_chirho: &[PapWrapperBindingChirho],
) -> Result<HashMap<PapWrapperKeyChirho, cranelift_module::FuncId>, String> {
    let mut pap_wrapper_decl_map_chirho = HashMap::new();
    for pap_wrapper_chirho in pap_wrappers_chirho {
        let sig_chirho = declare_function_signature_chirho(module_chirho, 2);
        let func_id_chirho = module_chirho
            .declare_function(
                &pap_wrapper_chirho.symbol_name_chirho,
                LinkageChirho::Local,
                &sig_chirho,
            )
            .map_err(|e_chirho| {
                format!(
                    "failed to declare partial application wrapper '{}': {e_chirho}",
                    pap_wrapper_chirho.symbol_name_chirho
                )
            })?;
        pap_wrapper_decl_map_chirho.insert(
            (
                pap_wrapper_chirho.target_id_chirho,
                pap_wrapper_chirho.applied_arg_count_chirho,
            ),
            func_id_chirho,
        );
    }
    Ok(pap_wrapper_decl_map_chirho)
}

fn is_dictionary_param_name_chirho(name_chirho: &str) -> bool {
    name_chirho.starts_with("$d") || name_chirho.starts_with("$dict")
}

fn collect_free_runtime_var_ids_inner_chirho(
    expr_chirho: &CoreExprChirho,
    known_func_ids_chirho: &HashSet<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
    bound_ids_chirho: &mut HashSet<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
    seen_ids_chirho: &mut HashSet<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
    free_ids_chirho: &mut Vec<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
) {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if !bound_ids_chirho.contains(id_chirho)
                && !known_func_ids_chirho.contains(id_chirho)
                && seen_ids_chirho.insert(*id_chirho)
            {
                free_ids_chirho.push(*id_chirho);
            }
        }
        CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            collect_free_runtime_var_ids_inner_chirho(
                fun_chirho,
                known_func_ids_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                free_ids_chirho,
            );
            collect_free_runtime_var_ids_inner_chirho(
                arg_chirho,
                known_func_ids_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                free_ids_chirho,
            );
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            let inserted_chirho = bound_ids_chirho.insert(binder_chirho.id_chirho);
            collect_free_runtime_var_ids_inner_chirho(
                body_chirho,
                known_func_ids_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                free_ids_chirho,
            );
            if inserted_chirho {
                bound_ids_chirho.remove(&binder_chirho.id_chirho);
            }
        }
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => {
            let mut inserted_ids_chirho = Vec::new();
            if *rec_chirho {
                for (binder_chirho, _) in binds_chirho {
                    if bound_ids_chirho.insert(binder_chirho.id_chirho) {
                        inserted_ids_chirho.push(binder_chirho.id_chirho);
                    }
                }
            }
            for (binder_chirho, rhs_chirho) in binds_chirho {
                collect_free_runtime_var_ids_inner_chirho(
                    rhs_chirho,
                    known_func_ids_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
                if !*rec_chirho && bound_ids_chirho.insert(binder_chirho.id_chirho) {
                    inserted_ids_chirho.push(binder_chirho.id_chirho);
                }
            }
            collect_free_runtime_var_ids_inner_chirho(
                body_chirho,
                known_func_ids_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                free_ids_chirho,
            );
            for inserted_id_chirho in inserted_ids_chirho {
                bound_ids_chirho.remove(&inserted_id_chirho);
            }
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            collect_free_runtime_var_ids_inner_chirho(
                scrutinee_chirho,
                known_func_ids_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                free_ids_chirho,
            );
            let case_inserted_chirho = bound_ids_chirho.insert(bind_chirho.id_chirho);
            for alt_chirho in alts_chirho {
                let mut alt_inserted_ids_chirho = Vec::new();
                for binder_chirho in &alt_chirho.binders_chirho {
                    if bound_ids_chirho.insert(binder_chirho.id_chirho) {
                        alt_inserted_ids_chirho.push(binder_chirho.id_chirho);
                    }
                }
                collect_free_runtime_var_ids_inner_chirho(
                    &alt_chirho.rhs_chirho,
                    known_func_ids_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
                for inserted_id_chirho in alt_inserted_ids_chirho {
                    bound_ids_chirho.remove(&inserted_id_chirho);
                }
            }
            if case_inserted_chirho {
                bound_ids_chirho.remove(&bind_chirho.id_chirho);
            }
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            collect_free_runtime_var_ids_inner_chirho(
                body_chirho,
                known_func_ids_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                free_ids_chirho,
            );
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            collect_free_runtime_var_ids_inner_chirho(
                expr_chirho,
                known_func_ids_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                free_ids_chirho,
            );
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                collect_free_runtime_var_ids_inner_chirho(
                    arg_chirho,
                    known_func_ids_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
            }
        }
    }
}

fn collect_free_runtime_var_ids_chirho(
    expr_chirho: &CoreExprChirho,
    imported_decl_map_chirho: &FuncDeclMapChirho,
) -> Vec<haskelujah_core_chirho::expr_chirho::CoreIdChirho> {
    let known_func_ids_chirho: HashSet<_> = imported_decl_map_chirho.keys().copied().collect();
    collect_free_runtime_var_ids_with_known_ids_chirho(expr_chirho, &known_func_ids_chirho)
}

fn collect_free_runtime_var_ids_with_known_ids_chirho(
    expr_chirho: &CoreExprChirho,
    known_func_ids_chirho: &HashSet<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
) -> Vec<haskelujah_core_chirho::expr_chirho::CoreIdChirho> {
    let mut bound_ids_chirho = HashSet::new();
    let mut seen_ids_chirho = HashSet::new();
    let mut free_ids_chirho = Vec::new();
    collect_free_runtime_var_ids_inner_chirho(
        expr_chirho,
        known_func_ids_chirho,
        &mut bound_ids_chirho,
        &mut seen_ids_chirho,
        &mut free_ids_chirho,
    );
    free_ids_chirho
}

fn bind_missing_param_aliases_chirho(
    env_chirho: &mut VarEnvChirho,
    param_binders_chirho: &[&BinderChirho],
    block_params_chirho: &[cranelift_codegen::ir::Value],
    body_chirho: &CoreExprChirho,
    imported_decl_map_chirho: &FuncDeclMapChirho,
) {
    let free_ids_chirho =
        collect_free_runtime_var_ids_chirho(body_chirho, imported_decl_map_chirho);
    let free_id_set_chirho: HashSet<_> = free_ids_chirho.iter().copied().collect();
    let param_ids_chirho: HashSet<_> = param_binders_chirho
        .iter()
        .map(|binder_chirho| binder_chirho.id_chirho)
        .collect();

    let unresolved_ids_chirho: Vec<_> = free_ids_chirho
        .iter()
        .copied()
        .filter(|id_chirho| !param_ids_chirho.contains(id_chirho))
        .collect();
    if unresolved_ids_chirho.is_empty() {
        return;
    }

    let candidate_param_ids_chirho: Vec<_> = param_binders_chirho
        .iter()
        .filter(|binder_chirho| !is_dictionary_param_name_chirho(&binder_chirho.name_chirho))
        .map(|binder_chirho| binder_chirho.id_chirho)
        .filter(|id_chirho| !free_id_set_chirho.contains(id_chirho))
        .collect();
    if candidate_param_ids_chirho.is_empty() {
        return;
    }

    let alias_pairs_chirho: Vec<_> = if candidate_param_ids_chirho.len() == 1 {
        unresolved_ids_chirho
            .iter()
            .copied()
            .map(|alias_id_chirho| (alias_id_chirho, candidate_param_ids_chirho[0]))
            .collect()
    } else if unresolved_ids_chirho.len() <= candidate_param_ids_chirho.len() {
        unresolved_ids_chirho
            .iter()
            .copied()
            .zip(candidate_param_ids_chirho.iter().copied())
            .collect()
    } else {
        Vec::new()
    };

    if alias_pairs_chirho.is_empty() {
        return;
    }

    let param_vals_by_id_chirho: HashMap<_, _> = param_binders_chirho
        .iter()
        .zip(block_params_chirho.iter())
        .map(|(binder_chirho, block_param_chirho)| (binder_chirho.id_chirho, *block_param_chirho))
        .collect();

    for (alias_id_chirho, source_id_chirho) in alias_pairs_chirho {
        if env_chirho.lookup_chirho(alias_id_chirho).is_some() {
            continue;
        }
        if let Some(source_val_chirho) = param_vals_by_id_chirho.get(&source_id_chirho) {
            env_chirho.bind_chirho(alias_id_chirho, *source_val_chirho);
        }
    }
}

fn define_pap_wrapper_body_chirho(
    module_chirho: &mut ObjModuleChirho,
    fb_ctx_chirho: &mut FuncBuilderCtxChirho,
    wrapper_func_id_chirho: cranelift_module::FuncId,
    pap_wrapper_chirho: &PapWrapperBindingChirho,
    pap_wrapper_decl_map_chirho: &HashMap<PapWrapperKeyChirho, cranelift_module::FuncId>,
    alloc_func_id_chirho: Option<cranelift_module::FuncId>,
) -> Result<(), String> {
    let sig_chirho = declare_function_signature_chirho(module_chirho, 2);
    let mut func_chirho = ClFunctionChirho::with_name_signature(
        cranelift_codegen::ir::UserFuncName::user(0, wrapper_func_id_chirho.as_u32()),
        sig_chirho,
    );

    {
        let mut builder_chirho = FuncBuilderChirho::new(&mut func_chirho, fb_ctx_chirho);
        let entry_block_chirho = builder_chirho.create_block();
        builder_chirho.append_block_params_for_function_params(entry_block_chirho);
        builder_chirho.switch_to_block(entry_block_chirho);
        builder_chirho.seal_block(entry_block_chirho);

        let target_func_ref_chirho = module_chirho.declare_func_in_func(
            pap_wrapper_chirho.target_func_id_chirho,
            builder_chirho.func,
        );
        let block_params_chirho = builder_chirho.block_params(entry_block_chirho).to_vec();
        let env_val_chirho = block_params_chirho[0];
        let ptr_mask_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, i64::MAX);
        let env_ptr_chirho = builder_chirho.ins().band(env_val_chirho, ptr_mask_chirho);
        let mem_flags_chirho = cranelift_codegen::ir::MemFlags::new();
        let arg_val_chirho = block_params_chirho[1];

        let mut stored_arg_vals_chirho = Vec::with_capacity(
            pap_wrapper_chirho.applied_arg_count_chirho
                + 1
                + pap_wrapper_chirho.hidden_capture_count_chirho,
        );
        for arg_idx_chirho in 0..pap_wrapper_chirho.applied_arg_count_chirho {
            let offset_chirho = ((arg_idx_chirho + 1) * 8) as i32;
            let applied_arg_chirho = builder_chirho.ins().load(
                cl_types_chirho::I64,
                mem_flags_chirho,
                env_ptr_chirho,
                offset_chirho,
            );
            stored_arg_vals_chirho.push(applied_arg_chirho);
        }
        stored_arg_vals_chirho.push(arg_val_chirho);
        for hidden_capture_idx_chirho in 0..pap_wrapper_chirho.hidden_capture_count_chirho {
            let offset_chirho =
                ((pap_wrapper_chirho.applied_arg_count_chirho + hidden_capture_idx_chirho + 1) * 8)
                    as i32;
            let hidden_capture_val_chirho = builder_chirho.ins().load(
                cl_types_chirho::I64,
                mem_flags_chirho,
                env_ptr_chirho,
                offset_chirho,
            );
            stored_arg_vals_chirho.push(hidden_capture_val_chirho);
        }

        let result_chirho = if pap_wrapper_chirho.remaining_arity_chirho == 1 {
            let call_inst_chirho = builder_chirho
                .ins()
                .call(target_func_ref_chirho, &stored_arg_vals_chirho);
            builder_chirho.inst_results(call_inst_chirho)[0]
        } else if let Some(alloc_func_id_chirho) = alloc_func_id_chirho {
            let next_wrapper_func_id_chirho = pap_wrapper_decl_map_chirho
                .get(&(
                    pap_wrapper_chirho.target_id_chirho,
                    pap_wrapper_chirho.applied_arg_count_chirho + 1,
                ))
                .ok_or_else(|| {
                    format!(
                        "missing PAP wrapper for target {} applied {}",
                        pap_wrapper_chirho.target_id_chirho.0,
                        pap_wrapper_chirho.applied_arg_count_chirho + 1
                    )
                })?;
            let alloc_ref_chirho =
                module_chirho.declare_func_in_func(alloc_func_id_chirho, builder_chirho.func);
            let next_wrapper_ref_chirho = module_chirho
                .declare_func_in_func(*next_wrapper_func_id_chirho, builder_chirho.func);
            emit_partial_application_closure_with_alloc_ref_chirho(
                &mut builder_chirho,
                alloc_ref_chirho,
                next_wrapper_ref_chirho,
                &stored_arg_vals_chirho,
            )
        } else {
            builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
        };
        builder_chirho.ins().return_(&[result_chirho]);
        builder_chirho.finalize();
    }

    let mut ctx_chirho = cranelift_codegen::Context::for_function(func_chirho);
    if let Err(verifier_error_chirho) = ctx_chirho.verify(module_chirho.isa()) {
        return Err(format!(
            "failed to verify partial application wrapper '{}': {verifier_error_chirho}\n{}",
            pap_wrapper_chirho.symbol_name_chirho,
            ctx_chirho.func.display()
        ));
    }
    module_chirho
        .define_function(wrapper_func_id_chirho, &mut ctx_chirho)
        .map_err(|e_chirho| {
            format!(
                "failed to define partial application wrapper '{}': {e_chirho}",
                pap_wrapper_chirho.symbol_name_chirho
            )
        })?;
    Ok(())
}

fn define_function_body_chirho(
    module_chirho: &mut ObjModuleChirho,
    fb_ctx_chirho: &mut FuncBuilderCtxChirho,
    func_id_chirho: cranelift_module::FuncId,
    current_core_id_chirho: haskelujah_core_chirho::expr_chirho::CoreIdChirho,
    debug_name_chirho: &str,
    rhs_chirho: &CoreExprChirho,
    current_capture_ids_chirho: &[haskelujah_core_chirho::expr_chirho::CoreIdChirho],
    imported_decl_map_chirho: &FuncDeclMapChirho,
    lifted_capture_ids_map_chirho: &HashMap<
        haskelujah_core_chirho::expr_chirho::CoreIdChirho,
        Vec<haskelujah_core_chirho::expr_chirho::CoreIdChirho>,
    >,
    pap_wrapper_decl_map_chirho: &HashMap<PapWrapperKeyChirho, cranelift_module::FuncId>,
    toplevel_names_chirho: &HashMap<haskelujah_core_chirho::expr_chirho::CoreIdChirho, String>,
    put_str_ln_func_id_chirho: Option<cranelift_module::FuncId>,
    print_int_func_id_chirho: Option<cranelift_module::FuncId>,
    append_str_func_id_chirho: Option<cranelift_module::FuncId>,
    alloc_func_id_chirho: Option<cranelift_module::FuncId>,
    show_int_func_id_chirho: Option<cranelift_module::FuncId>,
    show_bool_func_id_chirho: Option<cranelift_module::FuncId>,
    show_char_func_id_chirho: Option<cranelift_module::FuncId>,
    show_float_func_id_chirho: Option<cranelift_module::FuncId>,
    put_str_func_id_chirho: Option<cranelift_module::FuncId>,
    get_line_func_id_chirho: Option<cranelift_module::FuncId>,
    write_file_func_id_chirho: Option<cranelift_module::FuncId>,
    read_file_func_id_chirho: Option<cranelift_module::FuncId>,
    unpack_string_func_id_chirho: Option<cranelift_module::FuncId>,
    string_data_ids_chirho: &HashMap<String, cranelift_module::DataId>,
) -> Result<(), String> {
    let (param_binders_chirho, body_chirho) = peel_lambdas_chirho(rhs_chirho);
    let sig_chirho = declare_function_signature_chirho(
        module_chirho,
        param_binders_chirho.len() + current_capture_ids_chirho.len(),
    );

    let mut func_chirho = ClFunctionChirho::with_name_signature(
        cranelift_codegen::ir::UserFuncName::user(0, func_id_chirho.as_u32()),
        sig_chirho,
    );

    {
        let mut builder_chirho = FuncBuilderChirho::new(&mut func_chirho, fb_ctx_chirho);
        let entry_block_chirho = builder_chirho.create_block();
        builder_chirho.append_block_params_for_function_params(entry_block_chirho);
        builder_chirho.switch_to_block(entry_block_chirho);
        let loop_block_chirho = builder_chirho.create_block();
        let loop_param_count_chirho = param_binders_chirho.len() + current_capture_ids_chirho.len();
        for _ in 0..loop_param_count_chirho {
            builder_chirho.append_block_param(loop_block_chirho, cl_types_chirho::I64);
        }
        let entry_params_chirho = builder_chirho.block_params(entry_block_chirho).to_vec();
        builder_chirho
            .ins()
            .jump(loop_block_chirho, &entry_params_chirho);
        builder_chirho.seal_block(entry_block_chirho);
        builder_chirho.switch_to_block(loop_block_chirho);

        let mut func_ref_map_chirho: HashMap<
            haskelujah_core_chirho::expr_chirho::CoreIdChirho,
            (cranelift_codegen::ir::FuncRef, usize),
        > = HashMap::new();
        for (core_id_chirho, (decl_func_id_chirho, arity_chirho)) in imported_decl_map_chirho {
            let fref_chirho =
                module_chirho.declare_func_in_func(*decl_func_id_chirho, builder_chirho.func);
            func_ref_map_chirho.insert(*core_id_chirho, (fref_chirho, *arity_chirho));
        }
        let mut pap_wrapper_ref_map_chirho: HashMap<
            PapWrapperKeyChirho,
            cranelift_codegen::ir::FuncRef,
        > = HashMap::new();
        for (wrapper_key_chirho, wrapper_func_id_chirho) in pap_wrapper_decl_map_chirho {
            let wrapper_ref_chirho =
                module_chirho.declare_func_in_func(*wrapper_func_id_chirho, builder_chirho.func);
            pap_wrapper_ref_map_chirho.insert(*wrapper_key_chirho, wrapper_ref_chirho);
        }

        let mut env_chirho = VarEnvChirho::new_chirho();
        let block_params_chirho = builder_chirho.block_params(loop_block_chirho).to_vec();
        for (binder_chirho, param_val_chirho) in
            param_binders_chirho.iter().zip(block_params_chirho.iter())
        {
            env_chirho.bind_chirho(binder_chirho.id_chirho, *param_val_chirho);
        }
        for (capture_idx_chirho, capture_id_chirho) in current_capture_ids_chirho.iter().enumerate()
        {
            if let Some(capture_val_chirho) =
                block_params_chirho.get(param_binders_chirho.len() + capture_idx_chirho)
            {
                env_chirho.bind_chirho(*capture_id_chirho, *capture_val_chirho);
            }
        }
        bind_missing_param_aliases_chirho(
            &mut env_chirho,
            &param_binders_chirho,
            &block_params_chirho,
            body_chirho,
            imported_decl_map_chirho,
        );

        let mut next_var_idx_chirho: u32 = 0;
        let mut cl_vars_chirho: HashMap<
            haskelujah_core_chirho::expr_chirho::CoreIdChirho,
            cranelift_frontend::Variable,
        > = HashMap::new();
        let put_str_ln_fref_chirho = put_str_ln_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let print_int_fref_chirho = print_int_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let append_str_fref_chirho = append_str_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let alloc_fref_chirho = alloc_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let show_int_fref_chirho = show_int_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let show_bool_fref_chirho = show_bool_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let show_char_fref_chirho = show_char_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let show_float_fref_chirho = show_float_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let put_str_fref_chirho = put_str_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let get_line_fref_chirho = get_line_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let write_file_fref_chirho = write_file_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let read_file_fref_chirho = read_file_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let unpack_string_fref_chirho = unpack_string_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));

        let mut string_globals_chirho: HashMap<String, cranelift_codegen::ir::GlobalValue> =
            HashMap::new();
        for (s_chirho, data_id_chirho) in string_data_ids_chirho {
            let gv_chirho =
                module_chirho.declare_data_in_func(*data_id_chirho, builder_chirho.func);
            string_globals_chirho.insert(s_chirho.clone(), gv_chirho);
        }

        let mut ctx_chirho = LowerCtxChirho {
            env_chirho: &mut env_chirho,
            next_var_idx_chirho: &mut next_var_idx_chirho,
            cl_vars_chirho: &mut cl_vars_chirho,
            func_ref_map_chirho: &func_ref_map_chirho,
            lifted_capture_ids_chirho: lifted_capture_ids_map_chirho,
            pap_wrapper_ref_map_chirho: &pap_wrapper_ref_map_chirho,
            toplevel_names_chirho,
            put_str_ln_ref_chirho: put_str_ln_fref_chirho,
            print_int_ref_chirho: print_int_fref_chirho,
            append_str_ref_chirho: append_str_fref_chirho,
            alloc_ref_chirho: alloc_fref_chirho,
            show_int_ref_chirho: show_int_fref_chirho,
            show_bool_ref_chirho: show_bool_fref_chirho,
            show_char_ref_chirho: show_char_fref_chirho,
            show_float_ref_chirho: show_float_fref_chirho,
            put_str_ref_chirho: put_str_fref_chirho,
            get_line_ref_chirho: get_line_fref_chirho,
            write_file_ref_chirho: write_file_fref_chirho,
            read_file_ref_chirho: read_file_fref_chirho,
            unpack_string_ref_chirho: unpack_string_fref_chirho,
            string_globals_chirho,
            tco_self_id_chirho: Some(current_core_id_chirho),
            tco_loop_block_chirho: Some(loop_block_chirho),
        };

        match lower_tail_expr_chirho(&mut builder_chirho, &mut ctx_chirho, body_chirho) {
            TailLowerOutcomeChirho::ValueChirho(result_val_chirho) => {
                let result_i64_chirho =
                    ensure_i64_chirho(&mut builder_chirho, result_val_chirho, false);
                builder_chirho.ins().return_(&[result_i64_chirho]);
            }
            TailLowerOutcomeChirho::TerminatedChirho => {}
        }
        builder_chirho.seal_block(loop_block_chirho);

        builder_chirho.finalize();
    }

    let mut ctx_chirho = cranelift_codegen::Context::for_function(func_chirho);
    if let Err(verifier_error_chirho) = ctx_chirho.verify(module_chirho.isa()) {
        return Err(format!(
            "failed to verify function '{debug_name_chirho}': {verifier_error_chirho}\n{}",
            ctx_chirho.func.display()
        ));
    }
    module_chirho
        .define_function(func_id_chirho, &mut ctx_chirho)
        .map_err(|e_chirho| {
            format!("failed to define function '{debug_name_chirho}': {e_chirho}")
        })?;

    Ok(())
}

fn define_native_main_wrapper_chirho(
    module_chirho: &mut ObjModuleChirho,
    fb_ctx_chirho: &mut FuncBuilderCtxChirho,
    haskelujah_main_func_id_chirho: cranelift_module::FuncId,
    main_with_large_stack_func_id_chirho: cranelift_module::FuncId,
) -> Result<(), String> {
    let mut native_main_sig_chirho = module_chirho.make_signature();
    native_main_sig_chirho
        .returns
        .push(AbiParamChirho::new(cl_types_chirho::I32));
    let native_main_func_id_chirho = module_chirho
        .declare_function("main", LinkageChirho::Export, &native_main_sig_chirho)
        .map_err(|error_chirho| format!("failed to declare native main wrapper: {error_chirho}"))?;

    let mut func_chirho = ClFunctionChirho::with_name_signature(
        cranelift_codegen::ir::UserFuncName::user(0, native_main_func_id_chirho.as_u32()),
        native_main_sig_chirho,
    );

    {
        let mut builder_chirho = FuncBuilderChirho::new(&mut func_chirho, fb_ctx_chirho);
        let entry_block_chirho = builder_chirho.create_block();
        builder_chirho.switch_to_block(entry_block_chirho);
        builder_chirho.seal_block(entry_block_chirho);

        let haskelujah_main_ref_chirho =
            module_chirho.declare_func_in_func(haskelujah_main_func_id_chirho, builder_chirho.func);
        let main_with_large_stack_ref_chirho = module_chirho
            .declare_func_in_func(main_with_large_stack_func_id_chirho, builder_chirho.func);
        let haskelujah_main_addr_chirho = builder_chirho
            .ins()
            .func_addr(cl_types_chirho::I64, haskelujah_main_ref_chirho);
        let call_inst_chirho = builder_chirho.ins().call(
            main_with_large_stack_ref_chirho,
            &[haskelujah_main_addr_chirho],
        );
        let result_chirho = builder_chirho.inst_results(call_inst_chirho)[0];
        let exit_code_chirho = builder_chirho
            .ins()
            .ireduce(cl_types_chirho::I32, result_chirho);
        builder_chirho.ins().return_(&[exit_code_chirho]);
        builder_chirho.finalize();
    }

    let mut ctx_chirho = cranelift_codegen::Context::for_function(func_chirho);
    if let Err(verifier_error_chirho) = ctx_chirho.verify(module_chirho.isa()) {
        return Err(format!(
            "failed to verify native main wrapper: {verifier_error_chirho}\n{}",
            ctx_chirho.func.display()
        ));
    }
    module_chirho
        .define_function(native_main_func_id_chirho, &mut ctx_chirho)
        .map_err(|error_chirho| format!("failed to define native main wrapper: {error_chirho}"))?;

    Ok(())
}

fn restore_selector_bindings_chirho(
    original_module_chirho: &CoreModuleChirho,
    filtered_module_chirho: &mut CoreModuleChirho,
) {
    let original_bindings_by_id_chirho: HashMap<_, _> = original_module_chirho
        .bindings_chirho
        .iter()
        .map(|binding_chirho| (binding_chirho.binder_chirho.id_chirho, binding_chirho))
        .collect();

    for binding_chirho in &mut filtered_module_chirho.bindings_chirho {
        if !binding_chirho
            .binder_chirho
            .name_chirho
            .starts_with("$sel_")
        {
            continue;
        }
        if let Some(original_binding_chirho) =
            original_bindings_by_id_chirho.get(&binding_chirho.binder_chirho.id_chirho)
        {
            binding_chirho.rhs_chirho = original_binding_chirho.rhs_chirho.clone();
        }
    }
}

/// Lower a single Core binding to a Cranelift function definition.
///
/// Lambda arguments are peeled from the RHS expression and become function
/// parameters. The body is then lowered through the Cranelift expression
/// lowerer, with direct self-tail-calls rewritten into loop backedges.
/// Collect all string literals from a Core module.
fn collect_string_literals_chirho(
    module_chirho: &CoreModuleChirho,
    callback_chirho: &mut dyn FnMut(&str),
) {
    fn walk_expr_chirho(expr_chirho: &CoreExprChirho, cb_chirho: &mut dyn FnMut(&str)) {
        match expr_chirho {
            CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(s_chirho)) => {
                cb_chirho(s_chirho);
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                walk_expr_chirho(fun_chirho, cb_chirho);
                walk_expr_chirho(arg_chirho, cb_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. } => {
                walk_expr_chirho(body_chirho, cb_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_, rhs_chirho) in binds_chirho {
                    walk_expr_chirho(rhs_chirho, cb_chirho);
                }
                walk_expr_chirho(body_chirho, cb_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                walk_expr_chirho(scrutinee_chirho, cb_chirho);
                for alt_chirho in alts_chirho {
                    walk_expr_chirho(&alt_chirho.rhs_chirho, cb_chirho);
                }
            }
            CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
                walk_expr_chirho(expr_chirho, cb_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    walk_expr_chirho(arg_chirho, cb_chirho);
                }
            }
            _ => {}
        }
    }
    for binding_chirho in &module_chirho.bindings_chirho {
        walk_expr_chirho(&binding_chirho.rhs_chirho, callback_chirho);
    }
}

fn lower_binding_chirho(
    module_chirho: &mut ObjModuleChirho,
    fb_ctx_chirho: &mut FuncBuilderCtxChirho,
    binding_chirho: &CoreBindingChirho,
    func_decl_map_chirho: &FuncDeclMapChirho,
    core_module_chirho: &CoreModuleChirho,
    put_str_ln_func_id_chirho: Option<cranelift_module::FuncId>,
    print_int_func_id_chirho: Option<cranelift_module::FuncId>,
    append_str_func_id_chirho: Option<cranelift_module::FuncId>,
    alloc_func_id_chirho: Option<cranelift_module::FuncId>,
    show_int_func_id_chirho: Option<cranelift_module::FuncId>,
    show_bool_func_id_chirho: Option<cranelift_module::FuncId>,
    show_char_func_id_chirho: Option<cranelift_module::FuncId>,
    show_float_func_id_chirho: Option<cranelift_module::FuncId>,
    put_str_func_id_chirho: Option<cranelift_module::FuncId>,
    get_line_func_id_chirho: Option<cranelift_module::FuncId>,
    write_file_func_id_chirho: Option<cranelift_module::FuncId>,
    read_file_func_id_chirho: Option<cranelift_module::FuncId>,
    unpack_string_func_id_chirho: Option<cranelift_module::FuncId>,
    string_data_ids_chirho: &HashMap<String, cranelift_module::DataId>,
) -> Result<(), String> {
    let name_chirho = &binding_chirho.binder_chirho.name_chirho;
    let mut next_inline_id_chirho = next_inline_core_id_chirho(core_module_chirho);
    let rewritten_rhs_chirho = rewrite_inline_lambda_values_chirho(
        &binding_chirho.rhs_chirho,
        &mut next_inline_id_chirho,
        InlineLambdaRewriteModeChirho::PreserveLambdaRootChirho,
    );
    let (_param_binders_chirho, body_chirho) = peel_lambdas_chirho(&rewritten_rhs_chirho);
    let (func_id_chirho, _arity_chirho) = func_decl_map_chirho
        .get(&binding_chirho.binder_chirho.id_chirho)
        .ok_or_else(|| format!("function '{name_chirho}' not pre-declared"))?;
    let func_id_chirho = *func_id_chirho;

    let mut next_local_idx_chirho = 0;
    let mut lifted_bindings_chirho = Vec::new();
    collect_local_lambda_bindings_chirho(
        binding_chirho.binder_chirho.id_chirho,
        body_chirho,
        &mut next_local_idx_chirho,
        &mut lifted_bindings_chirho,
    );
    let mut known_func_ids_chirho: HashSet<_> = func_decl_map_chirho.keys().copied().collect();
    known_func_ids_chirho.extend(
        lifted_bindings_chirho
            .iter()
            .map(|lifted_binding_chirho| lifted_binding_chirho.binder_chirho.id_chirho),
    );
    for lifted_binding_chirho in &mut lifted_bindings_chirho {
        lifted_binding_chirho.captured_ids_chirho =
            collect_free_runtime_var_ids_with_known_ids_chirho(
                &lifted_binding_chirho.rhs_chirho,
                &known_func_ids_chirho,
            );
    }
    let local_decl_map_chirho =
        declare_local_lifted_bindings_chirho(module_chirho, &lifted_bindings_chirho)?;

    let mut imported_decl_map_chirho = func_decl_map_chirho.clone();
    imported_decl_map_chirho.extend(local_decl_map_chirho.iter().map(
        |(core_id_chirho, (local_func_id_chirho, arity_chirho))| {
            (*core_id_chirho, (*local_func_id_chirho, *arity_chirho))
        },
    ));
    let lifted_capture_ids_map_chirho: HashMap<_, _> = lifted_bindings_chirho
        .iter()
        .map(|lifted_binding_chirho| {
            (
                lifted_binding_chirho.binder_chirho.id_chirho,
                lifted_binding_chirho.captured_ids_chirho.clone(),
            )
        })
        .collect();
    let mut pap_scan_exprs_chirho = vec![&rewritten_rhs_chirho];
    for lifted_binding_chirho in &lifted_bindings_chirho {
        pap_scan_exprs_chirho.push(&lifted_binding_chirho.rhs_chirho);
    }
    let pap_wrappers_chirho = collect_partial_application_sites_chirho(
        binding_chirho.binder_chirho.id_chirho,
        &pap_scan_exprs_chirho,
        &imported_decl_map_chirho,
        &lifted_capture_ids_map_chirho,
    );
    let pap_wrapper_decl_map_chirho =
        declare_pap_wrappers_chirho(module_chirho, &pap_wrappers_chirho)?;

    let mut toplevel_names_chirho = core_module_chirho.names_chirho.clone();
    for binding_chirho in &core_module_chirho.bindings_chirho {
        toplevel_names_chirho.insert(
            binding_chirho.binder_chirho.id_chirho,
            binding_chirho.binder_chirho.name_chirho.clone(),
        );
    }

    for lifted_binding_chirho in &lifted_bindings_chirho {
        let (lifted_func_id_chirho, _) = local_decl_map_chirho
            .get(&lifted_binding_chirho.binder_chirho.id_chirho)
            .ok_or_else(|| {
                format!(
                    "lifted local function '{}' missing declaration",
                    lifted_binding_chirho.symbol_name_chirho
                )
            })?;
        define_function_body_chirho(
            module_chirho,
            fb_ctx_chirho,
            *lifted_func_id_chirho,
            lifted_binding_chirho.binder_chirho.id_chirho,
            &lifted_binding_chirho.symbol_name_chirho,
            &lifted_binding_chirho.rhs_chirho,
            &lifted_binding_chirho.captured_ids_chirho,
            &imported_decl_map_chirho,
            &lifted_capture_ids_map_chirho,
            &pap_wrapper_decl_map_chirho,
            &toplevel_names_chirho,
            put_str_ln_func_id_chirho,
            print_int_func_id_chirho,
            append_str_func_id_chirho,
            alloc_func_id_chirho,
            show_int_func_id_chirho,
            show_bool_func_id_chirho,
            show_char_func_id_chirho,
            show_float_func_id_chirho,
            put_str_func_id_chirho,
            get_line_func_id_chirho,
            write_file_func_id_chirho,
            read_file_func_id_chirho,
            unpack_string_func_id_chirho,
            string_data_ids_chirho,
        )?;
    }

    define_function_body_chirho(
        module_chirho,
        fb_ctx_chirho,
        func_id_chirho,
        binding_chirho.binder_chirho.id_chirho,
        name_chirho,
        &rewritten_rhs_chirho,
        &[],
        &imported_decl_map_chirho,
        &lifted_capture_ids_map_chirho,
        &pap_wrapper_decl_map_chirho,
        &toplevel_names_chirho,
        put_str_ln_func_id_chirho,
        print_int_func_id_chirho,
        append_str_func_id_chirho,
        alloc_func_id_chirho,
        show_int_func_id_chirho,
        show_bool_func_id_chirho,
        show_char_func_id_chirho,
        show_float_func_id_chirho,
        put_str_func_id_chirho,
        get_line_func_id_chirho,
        write_file_func_id_chirho,
        read_file_func_id_chirho,
        unpack_string_func_id_chirho,
        string_data_ids_chirho,
    )?;

    for pap_wrapper_chirho in &pap_wrappers_chirho {
        let wrapper_func_id_chirho = pap_wrapper_decl_map_chirho
            .get(&(
                pap_wrapper_chirho.target_id_chirho,
                pap_wrapper_chirho.applied_arg_count_chirho,
            ))
            .ok_or_else(|| {
                format!(
                    "partial application wrapper '{}' missing declaration",
                    pap_wrapper_chirho.symbol_name_chirho
                )
            })?;
        define_pap_wrapper_body_chirho(
            module_chirho,
            fb_ctx_chirho,
            *wrapper_func_id_chirho,
            pap_wrapper_chirho,
            &pap_wrapper_decl_map_chirho,
            alloc_func_id_chirho,
        )?;
    }

    Ok(())
}

/// Peel all leading lambda binders from an expression and return them together
/// with the inner body.
///
/// ```text
/// \x -> \y -> body   →   ([x_binder, y_binder], body)
/// ```
fn peel_lambdas_chirho(expr_chirho: &CoreExprChirho) -> (Vec<&BinderChirho>, &CoreExprChirho) {
    let mut binders_chirho: Vec<&BinderChirho> = Vec::new();
    let mut current_chirho = expr_chirho;
    loop {
        match current_chirho {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                binders_chirho.push(binder_chirho);
                current_chirho = body_chirho;
            }
            // Type lambdas are erased — peel through them too.
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                current_chirho = body_chirho;
            }
            other_chirho => break (binders_chirho, other_chirho),
        }
    }
}

// ─── Legacy helper (kept for compatibility) ───────────────────────────────────

/// Count the number of lambda parameters wrapping a Core expression.
///
/// Deprecated — prefer `peel_lambdas_chirho` for full binder information.
#[allow(dead_code)]
fn count_params_chirho(expr_chirho: &CoreExprChirho) -> (usize, &CoreExprChirho) {
    match expr_chirho {
        CoreExprChirho::LamChirho { body_chirho, .. } => {
            let (inner_count_chirho, inner_body_chirho) = count_params_chirho(body_chirho);
            (1 + inner_count_chirho, inner_body_chirho)
        }
        other_chirho => (0, other_chirho),
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::TargetConfigChirho;
    use haskelujah_core_chirho::expr_chirho::{
        AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
        CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
    };
    use haskelujah_span_chirho::SpanChirho;
    use haskelujah_typing_chirho::ty_chirho::TyChirho;
    use std::fs;
    use std::process::Command;

    // ── Shared helpers ────────────────────────────────────────────────────

    fn int_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn single_binding_module_chirho(
        name_chirho: &str,
        rhs_chirho: CoreExprChirho,
    ) -> CoreModuleChirho {
        CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho(name_chirho, 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        }
    }

    fn compile_ok_chirho(module_chirho: &CoreModuleChirho) -> Vec<u8> {
        let config_chirho = TargetConfigChirho::default();
        let result_chirho = compile_core_to_object_chirho(module_chirho, &config_chirho);
        assert!(
            result_chirho.is_ok(),
            "Compilation failed: {:?}",
            result_chirho.err()
        );
        let obj_chirho = result_chirho.unwrap();
        assert!(!obj_chirho.object_bytes_chirho.is_empty());
        obj_chirho.object_bytes_chirho
    }

    fn compile_and_run_exit_code_chirho(module_chirho: &CoreModuleChirho) -> i32 {
        let object_bytes_chirho = compile_ok_chirho(module_chirho);
        let unique_suffix_chirho = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let temp_dir_chirho = std::env::temp_dir().join(format!(
            "haskelujah-cranelift-runtime-test-{}-{unique_suffix_chirho}",
            std::process::id()
        ));
        fs::create_dir_all(&temp_dir_chirho).expect("create temp runtime dir");
        let obj_path_chirho = temp_dir_chirho.join("test.o");
        let exe_path_chirho = temp_dir_chirho.join("test-exe");
        fs::write(&obj_path_chirho, object_bytes_chirho).expect("write object file");

        let workspace_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(std::path::Path::parent)
            .expect("backend crate should live under workspace/crates");
        let cargo_status_chirho = Command::new("cargo")
            .current_dir(workspace_root_chirho)
            .args(["build", "-p", "haskelujah-rts-chirho", "--quiet"])
            .status()
            .expect("build RTS");
        assert!(cargo_status_chirho.success(), "RTS build should succeed");

        let link_status_chirho = Command::new("cc")
            .args([
                "-o",
                exe_path_chirho.to_str().expect("utf8 exe path"),
                obj_path_chirho.to_str().expect("utf8 obj path"),
                "-Wl,-no_fixup_chains",
                "-L",
                workspace_root_chirho
                    .join("target")
                    .join("debug")
                    .to_str()
                    .expect("utf8 rts lib dir"),
                "-lhaskelujah_rts_chirho",
            ])
            .status()
            .expect("link executable");
        assert!(link_status_chirho.success(), "link should succeed");

        let run_status_chirho = Command::new(&exe_path_chirho)
            .status()
            .expect("run executable");
        run_status_chirho.code().unwrap_or(-1)
    }

    fn compile_executable_and_run_exit_code_chirho(module_chirho: &CoreModuleChirho) -> i32 {
        let config_chirho = TargetConfigChirho::default();
        let result_chirho = compile_core_to_object_executable_chirho(module_chirho, &config_chirho);
        assert!(
            result_chirho.is_ok(),
            "Executable compilation failed: {:?}",
            result_chirho.err()
        );
        let object_bytes_chirho = result_chirho.unwrap().object_bytes_chirho;
        let unique_suffix_chirho = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let temp_dir_chirho = std::env::temp_dir().join(format!(
            "haskelujah-cranelift-executable-test-{}-{unique_suffix_chirho}",
            std::process::id()
        ));
        fs::create_dir_all(&temp_dir_chirho).expect("create temp executable dir");
        let obj_path_chirho = temp_dir_chirho.join("test.o");
        let exe_path_chirho = temp_dir_chirho.join("test-exe");
        fs::write(&obj_path_chirho, object_bytes_chirho).expect("write executable object file");

        let workspace_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(std::path::Path::parent)
            .expect("backend crate should live under workspace/crates");
        let cargo_status_chirho = Command::new("cargo")
            .current_dir(workspace_root_chirho)
            .args(["build", "-p", "haskelujah-rts-chirho", "--quiet"])
            .status()
            .expect("build RTS");
        assert!(cargo_status_chirho.success(), "RTS build should succeed");

        let link_status_chirho = Command::new("cc")
            .args([
                "-o",
                exe_path_chirho.to_str().expect("utf8 exe path"),
                obj_path_chirho.to_str().expect("utf8 obj path"),
                "-Wl,-no_fixup_chains",
                "-L",
                workspace_root_chirho
                    .join("target")
                    .join("debug")
                    .to_str()
                    .expect("utf8 rts lib dir"),
                "-lhaskelujah_rts_chirho",
            ])
            .status()
            .expect("link executable");
        assert!(link_status_chirho.success(), "link should succeed");

        let run_status_chirho = Command::new(&exe_path_chirho)
            .status()
            .expect("run executable");
        run_status_chirho.code().unwrap_or(-1)
    }

    fn compile_and_run_stdout_chirho(module_chirho: &CoreModuleChirho) -> String {
        let object_bytes_chirho = compile_ok_chirho(module_chirho);
        let unique_suffix_chirho = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let temp_dir_chirho = std::env::temp_dir().join(format!(
            "haskelujah-cranelift-stdout-test-{}-{unique_suffix_chirho}",
            std::process::id()
        ));
        fs::create_dir_all(&temp_dir_chirho).expect("create temp stdout dir");
        let obj_path_chirho = temp_dir_chirho.join("test.o");
        let exe_path_chirho = temp_dir_chirho.join("test-exe");
        fs::write(&obj_path_chirho, object_bytes_chirho).expect("write object file");

        let workspace_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(std::path::Path::parent)
            .expect("backend crate should live under workspace/crates");
        let cargo_status_chirho = Command::new("cargo")
            .current_dir(workspace_root_chirho)
            .args(["build", "-p", "haskelujah-rts-chirho", "--quiet"])
            .status()
            .expect("build RTS");
        assert!(cargo_status_chirho.success(), "RTS build should succeed");

        let link_status_chirho = Command::new("cc")
            .args([
                "-o",
                exe_path_chirho.to_str().expect("utf8 exe path"),
                obj_path_chirho.to_str().expect("utf8 obj path"),
                "-Wl,-no_fixup_chains",
                "-L",
                workspace_root_chirho
                    .join("target")
                    .join("debug")
                    .to_str()
                    .expect("utf8 rts lib dir"),
                "-lhaskelujah_rts_chirho",
            ])
            .status()
            .expect("link executable");
        assert!(link_status_chirho.success(), "link should succeed");

        let run_output_chirho = Command::new(&exe_path_chirho)
            .output()
            .expect("run executable");
        assert!(
            run_output_chirho.status.success(),
            "executable should succeed: {:?}",
            run_output_chirho.status
        );
        String::from_utf8(run_output_chirho.stdout).expect("stdout should be utf8")
    }

    // ── Previously existing tests (kept passing) ──────────────────────────

    #[test]
    fn compile_empty_module_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = TargetConfigChirho::default();
        let result_chirho = compile_core_to_object_chirho(&module_chirho, &config_chirho);
        assert!(result_chirho.is_ok());
        let obj_chirho = result_chirho.unwrap();
        assert!(!obj_chirho.object_bytes_chirho.is_empty());
    }

    #[test]
    fn compile_simple_binding_chirho() {
        let module_chirho = single_binding_module_chirho(
            "main",
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
        );
        compile_ok_chirho(&module_chirho);
    }

    #[test]
    fn compile_lambda_binding_chirho() {
        // id x = x   (λx.x)
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("id", 0),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("x", 1),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    // ── New tests: integer constant ───────────────────────────────────────

    /// Verify that a binding whose RHS is an integer literal compiles to a
    /// non-empty object file. The function should return `iconst i64 N`.
    #[test]
    fn compile_integer_constant_chirho() {
        for n_chirho in [0_i64, 1, -1, 42, i64::MAX, i64::MIN] {
            let module_chirho = single_binding_module_chirho(
                "const_val",
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(n_chirho)),
            );
            compile_ok_chirho(&module_chirho);
        }
    }

    // ── New tests: simple arithmetic (1 + 2) ─────────────────────────────

    /// `f = 1 + 2`  →  PrimOpChirho "+#" [Lit 1, Lit 2]
    #[test]
    fn compile_simple_addition_chirho() {
        let rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "+#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
            ],
        };
        let module_chirho = single_binding_module_chirho("add_one_two", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    /// `f = 10 - 3`  →  PrimOpChirho "-#"
    #[test]
    fn compile_subtraction_chirho() {
        let rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "-#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
            ],
        };
        let module_chirho = single_binding_module_chirho("sub_chirho", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    /// `f = 6 * 7`  →  PrimOpChirho "*#"
    #[test]
    fn compile_multiplication_chirho() {
        let rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "*#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(6)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
            ],
        };
        let module_chirho = single_binding_module_chirho("mul_chirho", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    /// `f = 10 \`div\` 3`  →  PrimOpChirho "div#"
    #[test]
    fn compile_division_chirho() {
        let rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "div#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
            ],
        };
        let module_chirho = single_binding_module_chirho("div_chirho", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    /// `f = 10 \`mod\` 3`  →  PrimOpChirho "mod#"
    #[test]
    fn compile_modulo_chirho() {
        let rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "mod#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
            ],
        };
        let module_chirho = single_binding_module_chirho("mod_chirho", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    // ── New tests: comparisons (x == 0) ──────────────────────────────────

    /// `f x = x == 0`  →  λx. PrimOpChirho "==#" [Var x, Lit 0]
    #[test]
    fn compile_equality_comparison_chirho() {
        let x_id_chirho = CoreIdChirho(1);
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 1),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            }),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("eq_zero", 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// `f x y = x /= y`  →  PrimOpChirho "/=#"
    #[test]
    fn compile_not_equal_comparison_chirho() {
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 1),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("y", 2),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "/=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                        CoreExprChirho::VarChirho(CoreIdChirho(2)),
                    ],
                }),
            }),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("neq_fn", 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// `f x y = x < y`  →  PrimOpChirho "<#"
    #[test]
    fn compile_less_than_comparison_chirho() {
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 1),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("y", 2),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                        CoreExprChirho::VarChirho(CoreIdChirho(2)),
                    ],
                }),
            }),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("lt_fn", 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    // ── New tests: let binding (let x = 5 in x + 1) ──────────────────────

    /// `f = let x = 5 in x + 1`
    #[test]
    fn compile_let_binding_chirho() {
        let x_id_chirho = CoreIdChirho(10);
        let rhs_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                int_binder_chirho("x", 10),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(5)),
            )],
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            }),
        };
        let module_chirho = single_binding_module_chirho("let_add_chirho", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    /// `f = let x = 3 in let y = x + 2 in y * y`  (nested lets)
    #[test]
    fn compile_nested_let_bindings_chirho() {
        let x_id_chirho = CoreIdChirho(10);
        let y_id_chirho = CoreIdChirho(11);
        let inner_let_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                int_binder_chirho("y", 11),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                    ],
                },
            )],
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "*#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(y_id_chirho),
                    CoreExprChirho::VarChirho(y_id_chirho),
                ],
            }),
        };
        let outer_let_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                int_binder_chirho("x", 10),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
            )],
            body_chirho: Box::new(inner_let_chirho),
        };
        let module_chirho = single_binding_module_chirho("nested_let_chirho", outer_let_chirho);
        compile_ok_chirho(&module_chirho);
    }

    // ── New tests: case on literal integers ───────────────────────────────

    /// `f x = case x of { 0 -> 10; _ -> 20 }`
    #[test]
    fn compile_case_on_literal_chirho() {
        let x_id_chirho = CoreIdChirho(1);
        let scrut_bind_chirho = int_binder_chirho("wild", 99);
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_id_chirho)),
            bind_chirho: scrut_bind_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(0)),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
                },
            ],
        };
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 1),
            body_chirho: Box::new(body_chirho),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("case_zero", 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// `f x = case x of { 1 -> 100; 2 -> 200; 3 -> 300; _ -> 0 }`
    #[test]
    fn compile_case_multi_literal_chirho() {
        let x_id_chirho = CoreIdChirho(1);
        let scrut_bind_chirho = int_binder_chirho("wild", 99);
        let alts_chirho: Vec<CoreAltChirho> = (1..=3_i64)
            .map(|n_chirho| CoreAltChirho {
                con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(n_chirho)),
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(n_chirho * 100)),
            })
            .chain(std::iter::once(CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            }))
            .collect();

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_id_chirho)),
            bind_chirho: scrut_bind_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho,
        };
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 1),
            body_chirho: Box::new(body_chirho),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("multi_case", 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// `f x = case x of { True -> 1; False -> 0 }`
    /// (constructor alts with Bool tags)
    #[test]
    fn compile_case_bool_constructor_chirho() {
        let x_id_chirho = CoreIdChirho(1);
        let scrut_bind_chirho = int_binder_chirho("wild", 99);
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_id_chirho)),
            bind_chirho: scrut_bind_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("False".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                },
            ],
        };
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 1),
            body_chirho: Box::new(body_chirho),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("bool_to_int", 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    // ── New tests: combined let + case ────────────────────────────────────

    /// `f x = let y = x + 1 in case y of { 0 -> 99; _ -> y }`
    #[test]
    fn compile_let_then_case_chirho() {
        let x_id_chirho = CoreIdChirho(1);
        let y_id_chirho = CoreIdChirho(2);
        let scrut_bind_chirho = int_binder_chirho("wild", 99);

        let case_expr_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_id_chirho)),
            bind_chirho: scrut_bind_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(0)),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::VarChirho(y_id_chirho),
                },
            ],
        };

        let let_expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                int_binder_chirho("y", 2),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                },
            )],
            body_chirho: Box::new(case_expr_chirho),
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 1),
            body_chirho: Box::new(let_expr_chirho),
        };

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("let_case_fn", 0),
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    // ── New tests: type-level wrappers are erased ─────────────────────────

    /// Wrap a literal in TyLam/TyApp and verify it still compiles.
    #[test]
    fn compile_ty_lam_app_erased_chirho() {
        use haskelujah_typing_chirho::ty_chirho::TyChirho as TyCh;
        let inner_chirho = CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7));
        let ty_lam_chirho = CoreExprChirho::TyLamChirho {
            ty_var_chirho: "a".to_string(),
            body_chirho: Box::new(CoreExprChirho::TyAppChirho {
                expr_chirho: Box::new(inner_chirho),
                ty_chirho: TyCh::int_chirho(),
            }),
        };
        let module_chirho = single_binding_module_chirho("poly_const_chirho", ty_lam_chirho);
        compile_ok_chirho(&module_chirho);
    }

    // ── New tests: peel_lambdas_chirho correctness ────────────────────────

    #[test]
    fn peel_lambdas_zero_chirho() {
        let expr_chirho = CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1));
        let (binders_chirho, body_chirho) = peel_lambdas_chirho(&expr_chirho);
        assert_eq!(binders_chirho.len(), 0);
        assert!(matches!(body_chirho, CoreExprChirho::LitChirho(_)));
    }

    #[test]
    fn peel_lambdas_two_chirho() {
        let lit_chirho = CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42));
        let lam_y_chirho = CoreExprChirho::LamChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(2),
                name_chirho: "y".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            body_chirho: Box::new(lit_chirho),
        };
        let lam_x_chirho = CoreExprChirho::LamChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(1),
                name_chirho: "x".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            body_chirho: Box::new(lam_y_chirho),
        };
        let (binders_chirho, body_chirho) = peel_lambdas_chirho(&lam_x_chirho);
        assert_eq!(binders_chirho.len(), 2);
        assert_eq!(binders_chirho[0].name_chirho, "x");
        assert_eq!(binders_chirho[1].name_chirho, "y");
        assert!(matches!(body_chirho, CoreExprChirho::LitChirho(_)));
    }

    // ── New tests: direct function calls via App ─────────────────────────

    /// `f x y = x + y; main = f 10 32`
    /// Two bindings: f is a 2-arg function, main calls f directly.
    #[test]
    fn compile_function_call_chirho() {
        let f_id_chirho = CoreIdChirho(0);
        let x_id_chirho = CoreIdChirho(1);
        let y_id_chirho = CoreIdChirho(2);

        let f_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("f", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("x", 1),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("y", 2),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_id_chirho),
                            CoreExprChirho::VarChirho(y_id_chirho),
                        ],
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 3),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10))),
                }),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(32))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![f_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// `inc x = x + 1; main = inc 41`  (single-arg direct call)
    #[test]
    fn compile_single_arg_function_call_chirho() {
        let inc_id_chirho = CoreIdChirho(0);
        let x_id_chirho = CoreIdChirho(1);

        let inc_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("inc", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("x", 1),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 2),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(inc_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(41))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![inc_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// Dict elision: module with $sel/$f dicts that should be pruned.
    #[test]
    fn compile_executable_with_dict_elision_chirho() {
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
                ],
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        // Unreachable binding with dict-like name that should be pruned.
        let dead_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("$fNumInt", 100),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![main_binding_chirho, dead_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let config_chirho = TargetConfigChirho::default();
        let result_chirho =
            compile_core_to_object_executable_chirho(&module_chirho, &config_chirho);
        assert!(
            result_chirho.is_ok(),
            "Dict-elided compilation failed: {:?}",
            result_chirho.err()
        );
        assert!(!result_chirho.unwrap().object_bytes_chirho.is_empty());
    }

    #[test]
    fn compile_executable_print_int_chirho() {
        let print_arg_binder_chirho = int_binder_chirho("printArgChirho", 1);
        let print_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("print", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: print_arg_binder_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 2),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(
                    print_binding_chirho.binder_chirho.id_chirho,
                )),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "PrintInt".to_string(),
            bindings_chirho: vec![print_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = TargetConfigChirho::default();
        let result_chirho =
            compile_core_to_object_executable_chirho(&module_chirho, &config_chirho);
        assert!(
            result_chirho.is_ok(),
            "print executable compilation failed: {:?}",
            result_chirho.err()
        );
    }

    /// `f a b c = a + b + c; main = f 1 2 3` (3-arg direct call)
    #[test]
    fn compile_multi_arg_function_call_chirho() {
        let f_id_chirho = CoreIdChirho(0);
        let a_id_chirho = CoreIdChirho(1);
        let b_id_chirho = CoreIdChirho(2);
        let c_id_chirho = CoreIdChirho(3);

        let f_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("f", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("a", 1),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("b", 2),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: int_binder_chirho("c", 3),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::PrimOpChirho {
                                    name_chirho: "+#".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(a_id_chirho),
                                        CoreExprChirho::VarChirho(b_id_chirho),
                                    ],
                                },
                                CoreExprChirho::VarChirho(c_id_chirho),
                            ],
                        }),
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 4),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            1,
                        ))),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
                }),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![f_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// Recursive function: `fact n = case n of { 0 -> 1; _ -> n * fact (n - 1) }`
    #[test]
    fn compile_recursive_function_chirho() {
        let fact_id_chirho = CoreIdChirho(0);
        let n_id_chirho = CoreIdChirho(1);
        let scrut_bind_chirho = int_binder_chirho("wild", 99);

        let fact_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("fact", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("n", 1),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(n_id_chirho)),
                    bind_chirho: scrut_bind_chirho,
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(0)),
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::PrimOpChirho {
                                name_chirho: "*#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(n_id_chirho),
                                    CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                            fact_id_chirho,
                                        )),
                                        arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                            name_chirho: "-#".to_string(),
                                            args_chirho: vec![
                                                CoreExprChirho::VarChirho(n_id_chirho),
                                                CoreExprChirho::LitChirho(
                                                    CoreLitChirho::IntChirho(1),
                                                ),
                                            ],
                                        }),
                                    },
                                ],
                            },
                        },
                    ],
                }),
            },
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![fact_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        compile_ok_chirho(&module_chirho);
    }

    /// `ConApp` in function body compiles successfully.
    #[test]
    fn compile_constructor_app_chirho() {
        let rhs_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "Just".to_string(),
            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))],
        };
        let module_chirho = single_binding_module_chirho("mk_just", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    #[test]
    fn compile_constructor_case_with_payload_binders_chirho() {
        let scrut_binder_chirho = int_binder_chirho("pair", 1);
        let lhs_binder_chirho = int_binder_chirho("lhs", 2);
        let rhs_binder_chirho = int_binder_chirho("rhs", 3);
        let rhs_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: "Pair".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(22)),
                ],
            }),
            bind_chirho: scrut_binder_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("Pair".to_string()),
                binders_chirho: vec![lhs_binder_chirho.clone(), rhs_binder_chirho.clone()],
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(lhs_binder_chirho.id_chirho),
                        CoreExprChirho::VarChirho(rhs_binder_chirho.id_chirho),
                    ],
                },
            }],
        };
        let module_chirho = single_binding_module_chirho("sum_pair", rhs_chirho);
        compile_ok_chirho(&module_chirho);
    }

    #[test]
    fn run_constructor_case_with_payload_binders_chirho() {
        let scrut_binder_chirho = int_binder_chirho("pair", 1);
        let lhs_binder_chirho = int_binder_chirho("lhs", 2);
        let rhs_binder_chirho = int_binder_chirho("rhs", 3);
        let rhs_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: "Pair".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(22)),
                ],
            }),
            bind_chirho: scrut_binder_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("Pair".to_string()),
                binders_chirho: vec![lhs_binder_chirho.clone(), rhs_binder_chirho.clone()],
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(lhs_binder_chirho.id_chirho),
                        CoreExprChirho::VarChirho(rhs_binder_chirho.id_chirho),
                    ],
                },
            }],
        };
        let module_chirho = single_binding_module_chirho("main", rhs_chirho);
        let exit_code_chirho = compile_and_run_exit_code_chirho(&module_chirho);
        assert_eq!(exit_code_chirho, 42);
    }

    #[test]
    fn run_constructor_payload_function_value_chirho() {
        let inc_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("incChirho", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("xChirho", 1),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let fun_binder_chirho = int_binder_chirho("funChirho", 3);
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 2),
            rhs_chirho: CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "BoxedFunChirho".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(
                        inc_binding_chirho.binder_chirho.id_chirho,
                    )],
                }),
                bind_chirho: int_binder_chirho("boxedFunChirho", 4),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("BoxedFunChirho".to_string()),
                    binders_chirho: vec![fun_binder_chirho.clone()],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            fun_binder_chirho.id_chirho,
                        )),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            41,
                        ))),
                    },
                }],
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let module_chirho = CoreModuleChirho {
            name_chirho: "FunctionField".to_string(),
            bindings_chirho: vec![inc_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let exit_code_chirho = compile_and_run_exit_code_chirho(&module_chirho);
        assert_eq!(exit_code_chirho, 42);
    }

    #[test]
    fn run_overapplied_function_return_value_chirho() {
        let inc_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("incChirho", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("xChirho", 1),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let get_inc_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("getIncChirho", 2),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("dictChirho", 3),
                body_chirho: Box::new(CoreExprChirho::VarChirho(
                    inc_binding_chirho.binder_chirho.id_chirho,
                )),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 4),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                        get_inc_binding_chirho.binder_chirho.id_chirho,
                    )),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))),
                }),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(41))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Overapply".to_string(),
            bindings_chirho: vec![
                inc_binding_chirho,
                get_inc_binding_chirho,
                main_binding_chirho,
            ],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let exit_code_chirho = compile_and_run_exit_code_chirho(&module_chirho);
        assert_eq!(exit_code_chirho, 42);
    }

    #[test]
    fn run_inline_lambda_argument_value_chirho() {
        let apply_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("applyChirho", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("funChirho", 1),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("argChirho", 2),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let inline_arg_binder_chirho = int_binder_chirho("inlineArgChirho", 3);
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 4),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                        apply_binding_chirho.binder_chirho.id_chirho,
                    )),
                    arg_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: inline_arg_binder_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(inline_arg_binder_chirho.id_chirho),
                                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                            ],
                        }),
                    }),
                }),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(41))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "InlineLambdaArg".to_string(),
            bindings_chirho: vec![apply_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let exit_code_chirho = compile_and_run_exit_code_chirho(&module_chirho);
        assert_eq!(exit_code_chirho, 42);
    }

    #[test]
    fn run_put_str_ln_concat_string_chirho() {
        let put_str_ln_arg_binder_chirho = int_binder_chirho("putStrLnArgChirho", 1);
        let put_str_ln_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("putStrLn", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: put_str_ln_arg_binder_chirho,
                body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 2),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(
                    put_str_ln_binding_chirho.binder_chirho.id_chirho,
                )),
                arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: "++#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                                    "Hello".to_string(),
                                )),
                                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                                    " ".to_string(),
                                )),
                            ],
                        },
                        CoreExprChirho::LitChirho(CoreLitChirho::StringChirho("World".to_string())),
                    ],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "ConcatString".to_string(),
            bindings_chirho: vec![put_str_ln_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let stdout_chirho = compile_and_run_stdout_chirho(&module_chirho);
        assert_eq!(stdout_chirho, "Hello World\n");
    }

    #[test]
    fn run_curried_two_arg_function_value_chirho() {
        let apply2_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("apply2Chirho", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("fChirho", 1),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("xChirho", 2),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: int_binder_chirho("yChirho", 3),
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                            }),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(3))),
                        }),
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let add_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("addChirho", 4),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("lhsChirho", 5),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("rhsChirho", 6),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(CoreIdChirho(5)),
                            CoreExprChirho::VarChirho(CoreIdChirho(6)),
                        ],
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 7),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            apply2_binding_chirho.binder_chirho.id_chirho,
                        )),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                            add_binding_chirho.binder_chirho.id_chirho,
                        )),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3))),
                }),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(4))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "CurriedFunctionValue".to_string(),
            bindings_chirho: vec![
                apply2_binding_chirho,
                add_binding_chirho,
                main_binding_chirho,
            ],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let exit_code_chirho = compile_and_run_exit_code_chirho(&module_chirho);
        assert_eq!(exit_code_chirho, 7);
    }

    #[test]
    fn run_executable_selector_value_alias_preserves_dict_arg_chirho() {
        let sub_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("$prim_Num_-_Int", 0),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("lhsChirho", 1),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: int_binder_chirho("rhsChirho", 2),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "-#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(CoreIdChirho(1)),
                            CoreExprChirho::VarChirho(CoreIdChirho(2)),
                        ],
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let dict_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("$fNumInt", 3),
            rhs_chirho: CoreExprChirho::ConAppChirho {
                con_name_chirho: "DictNumIntChirho".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(
                    sub_binding_chirho.binder_chirho.id_chirho,
                )],
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let dict_field_binder_chirho = int_binder_chirho("minusFieldChirho", 5);
        let selector_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("$sel_Num_-", 4),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("dictArgChirho", 6),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(6))),
                    bind_chirho: int_binder_chirho("dictScrutChirho", 7),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("DictNumIntChirho".to_string()),
                        binders_chirho: vec![dict_field_binder_chirho.clone()],
                        rhs_chirho: CoreExprChirho::VarChirho(dict_field_binder_chirho.id_chirho),
                    }],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let alias_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("minusAliasChirho", 8),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(
                    selector_binding_chirho.binder_chirho.id_chirho,
                )),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                    dict_binding_chirho.binder_chirho.id_chirho,
                )),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let main_binding_chirho = CoreBindingChirho {
            binder_chirho: int_binder_chirho("main", 9),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                        alias_binding_chirho.binder_chirho.id_chirho,
                    )),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3))),
                }),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "SelectorValueAlias".to_string(),
            bindings_chirho: vec![
                sub_binding_chirho,
                dict_binding_chirho,
                selector_binding_chirho,
                alias_binding_chirho,
                main_binding_chirho,
            ],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let exit_code_chirho = compile_executable_and_run_exit_code_chirho(&module_chirho);
        assert_eq!(exit_code_chirho, 1);
    }
}
