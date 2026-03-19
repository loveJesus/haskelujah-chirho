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

use std::collections::HashMap;

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
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho, CoreModuleChirho,
};

use crate::lower_chirho::{LowerCtxChirho, VarEnvChirho, ensure_i64_chirho, lower_expr_chirho};
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
    compile_core_to_object_chirho(&filtered_module_chirho, config_chirho)
}

/// Compile a `CoreModuleChirho` to a native object file via Cranelift.
pub fn compile_core_to_object_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &TargetConfigChirho,
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

        let linkage_chirho = if name_chirho == "main" {
            LinkageChirho::Export
        } else {
            LinkageChirho::Local
        };
        let func_id_chirho = obj_module_chirho
            .declare_function(name_chirho, linkage_chirho, &sig_chirho)
            .map_err(|e_chirho| {
                format!("failed to declare function '{name_chirho}': {e_chirho}")
            })?;
        func_decl_map_chirho.insert(
            binding_chirho.binder_chirho.id_chirho,
            (func_id_chirho, param_count_chirho),
        );
    }

    // ── Import libc functions for Prelude IO ─────────────────────────────
    let (libc_puts_id_chirho, print_int_func_id_chirho, alloc_func_id_chirho) = {
        // puts(ptr) -> i32
        let mut puts_sig_chirho = obj_module_chirho.make_signature();
        puts_sig_chirho.params.push(AbiParamChirho::new(
            obj_module_chirho.target_config().pointer_type(),
        ));
        puts_sig_chirho
            .returns
            .push(AbiParamChirho::new(cl_types_chirho::I32));
        let libc_puts_id_chirho = obj_module_chirho
            .declare_function("puts", LinkageChirho::Import, &puts_sig_chirho)
            .ok();

        // Cranelift does not model libc varargs robustly on Apple AArch64,
        // so use a fixed-signature RTS helper for integer printing.
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

        (
            libc_puts_id_chirho,
            print_int_func_id_chirho,
            alloc_func_id_chirho,
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
            libc_puts_id_chirho,
            print_int_func_id_chirho,
            alloc_func_id_chirho,
            &string_data_ids_chirho,
        )?;
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
/// parameters. The body is then lowered by `lower_expr_chirho`.
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
    libc_puts_id_chirho: Option<cranelift_module::FuncId>,
    print_int_func_id_chirho: Option<cranelift_module::FuncId>,
    alloc_func_id_chirho: Option<cranelift_module::FuncId>,
    string_data_ids_chirho: &HashMap<String, cranelift_module::DataId>,
) -> Result<(), String> {
    let name_chirho = &binding_chirho.binder_chirho.name_chirho;

    // ── Count parameters from nested lambdas ──────────────────────────────
    let (param_binders_chirho, body_chirho) = peel_lambdas_chirho(&binding_chirho.rhs_chirho);

    // ── Look up the pre-declared function ID ──────────────────────────────
    let (func_id_chirho, _arity_chirho) = func_decl_map_chirho
        .get(&binding_chirho.binder_chirho.id_chirho)
        .ok_or_else(|| format!("function '{name_chirho}' not pre-declared"))?;
    let func_id_chirho = *func_id_chirho;

    // ── Build function signature (matching declaration) ───────────────────
    let param_count_chirho = param_binders_chirho.len();
    let mut sig_chirho = module_chirho.make_signature();
    sig_chirho
        .returns
        .push(AbiParamChirho::new(cl_types_chirho::I64));
    for _ in 0..param_count_chirho {
        sig_chirho
            .params
            .push(AbiParamChirho::new(cl_types_chirho::I64));
    }

    // ── Build function body ────────────────────────────────────────────────
    let mut func_chirho = ClFunctionChirho::with_name_signature(
        cranelift_codegen::ir::UserFuncName::user(0, func_id_chirho.as_u32()),
        sig_chirho,
    );

    {
        let mut builder_chirho = FuncBuilderChirho::new(&mut func_chirho, fb_ctx_chirho);
        let entry_block_chirho = builder_chirho.create_block();
        builder_chirho.append_block_params_for_function_params(entry_block_chirho);
        builder_chirho.switch_to_block(entry_block_chirho);
        builder_chirho.seal_block(entry_block_chirho);

        // ── Import all declared functions into this function for direct calls ─
        let mut func_ref_map_chirho: HashMap<
            haskelujah_core_chirho::expr_chirho::CoreIdChirho,
            (cranelift_codegen::ir::FuncRef, usize),
        > = HashMap::new();
        for (core_id_chirho, (decl_func_id_chirho, arity_chirho)) in func_decl_map_chirho {
            let fref_chirho =
                module_chirho.declare_func_in_func(*decl_func_id_chirho, builder_chirho.func);
            func_ref_map_chirho.insert(*core_id_chirho, (fref_chirho, *arity_chirho));
        }

        // ── Bind lambda parameters to SSA values ──────────────────────────
        let mut env_chirho = VarEnvChirho::new_chirho();
        let block_params_chirho = builder_chirho.block_params(entry_block_chirho).to_vec();
        for (binder_chirho, param_val_chirho) in
            param_binders_chirho.iter().zip(block_params_chirho.iter())
        {
            env_chirho.bind_chirho(binder_chirho.id_chirho, *param_val_chirho);
        }

        // ── Set up lowering context ────────────────────────────────────────
        let mut next_var_idx_chirho: u32 = 0;
        let mut cl_vars_chirho: HashMap<
            haskelujah_core_chirho::expr_chirho::CoreIdChirho,
            cranelift_frontend::Variable,
        > = HashMap::new();
        // Import puts and RTS print helper if available.
        let puts_fref_chirho = libc_puts_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let print_int_fref_chirho = print_int_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));
        let alloc_fref_chirho = alloc_func_id_chirho
            .map(|fid_chirho| module_chirho.declare_func_in_func(fid_chirho, builder_chirho.func));

        // Import string data globals into this function
        let mut string_globals_chirho: HashMap<String, cranelift_codegen::ir::GlobalValue> =
            HashMap::new();
        for (s_chirho, data_id_chirho) in string_data_ids_chirho {
            let gv_chirho =
                module_chirho.declare_data_in_func(*data_id_chirho, builder_chirho.func);
            string_globals_chirho.insert(s_chirho.clone(), gv_chirho);
        }

        // Build name map for detecting Prelude functions
        let toplevel_names_chirho: HashMap<
            haskelujah_core_chirho::expr_chirho::CoreIdChirho,
            String,
        > = core_module_chirho
            .bindings_chirho
            .iter()
            .map(|b_chirho| {
                (
                    b_chirho.binder_chirho.id_chirho,
                    b_chirho.binder_chirho.name_chirho.clone(),
                )
            })
            .collect();

        let mut ctx_chirho = LowerCtxChirho {
            env_chirho: &mut env_chirho,
            next_var_idx_chirho: &mut next_var_idx_chirho,
            cl_vars_chirho: &mut cl_vars_chirho,
            func_ref_map_chirho: &func_ref_map_chirho,
            toplevel_names_chirho: &toplevel_names_chirho,
            puts_ref_chirho: puts_fref_chirho,
            print_int_ref_chirho: print_int_fref_chirho,
            alloc_ref_chirho: alloc_fref_chirho,
            string_globals_chirho,
        };

        // ── Lower the body expression ──────────────────────────────────────
        let result_val_chirho =
            lower_expr_chirho(&mut builder_chirho, &mut ctx_chirho, body_chirho);
        let result_i64_chirho = ensure_i64_chirho(&mut builder_chirho, result_val_chirho, false);
        builder_chirho.ins().return_(&[result_i64_chirho]);

        builder_chirho.finalize();
    }

    // ── Define the function in the object module ───────────────────────────
    let mut ctx_chirho = cranelift_codegen::Context::for_function(func_chirho);
    if let Err(verifier_error_chirho) = ctx_chirho.verify(module_chirho.isa()) {
        return Err(format!(
            "failed to verify function '{name_chirho}': {verifier_error_chirho}\n{}",
            ctx_chirho.func.display()
        ));
    }
    module_chirho
        .define_function(func_id_chirho, &mut ctx_chirho)
        .map_err(|e_chirho| format!("failed to define function '{name_chirho}': {e_chirho}"))?;

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
    use std::fs;
    use std::process::Command;
    use haskelujah_core_chirho::expr_chirho::{
        AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
        CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
    };
    use haskelujah_span_chirho::SpanChirho;
    use haskelujah_typing_chirho::ty_chirho::TyChirho;

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
        let temp_dir_chirho =
            std::env::temp_dir().join("haskelujah-cranelift-runtime-test-chirho");
        let _ = fs::remove_dir_all(&temp_dir_chirho);
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
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_binder_chirho.id_chirho)),
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
            bindings_chirho: vec![inc_binding_chirho, get_inc_binding_chirho, main_binding_chirho],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };

        let exit_code_chirho = compile_and_run_exit_code_chirho(&module_chirho);
        assert_eq!(exit_code_chirho, 42);
    }
}
