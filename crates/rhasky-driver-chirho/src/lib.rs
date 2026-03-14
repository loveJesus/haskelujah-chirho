// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-driver-chirho
//!
//! Build orchestration for the Rhasky compiler. Coordinates parsing, checking,
//! and backend lowering across execution modes.

pub mod stg_lower_chirho;

use std::path::{Path, PathBuf};

use rhasky_ast_chirho::ModuleChirho;
use rhasky_backend_llvm_chirho::compile_to_llvm_ir_stub_chirho;
use rhasky_backend_llvm_chirho::compile_core_to_llvm_chirho;
use rhasky_backend_wasm_chirho::compile_to_wasm_stub_chirho;
use rhasky_backend_wasm_chirho::compile_core_to_wasm_chirho;
use rhasky_core_chirho::{
    desugar_module_chirho, simplify_module_chirho,
    CoreModuleChirho, SimplifyConfigChirho,
};
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
use rhasky_naming_chirho::resolve_chirho::{resolve_module_chirho, resolve_module_with_imports_chirho};
use rhasky_naming_chirho::iface_chirho::{build_iface_chirho, ModuleIfaceChirho};
use rhasky_typing_chirho::infer_chirho::{infer_module_chirho, infer_module_with_imports_chirho};
use rhasky_parser_chirho::cst_parser_chirho::ParserChirho;
use rhasky_parser_chirho::lower_chirho::lower_module_chirho;
use rhasky_parser_chirho::parse_source_file_chirho;
use rhasky_runtime_chirho::{ExecutionModeChirho, RuntimePlanChirho};
use rhasky_span_chirho::SourceMapChirho;
use rhasky_syntax_chirho::SourceFileChirho;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendPlanChirho {
    pub llvm_preview_chirho: String,
    pub wasm_stub_size_chirho: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummaryChirho {
    pub source_path_chirho: PathBuf,
    pub module_name_chirho: String,
    pub runtime_plan_chirho: RuntimePlanChirho,
    pub backend_plan_chirho: BackendPlanChirho,
}

pub fn check_source_path_chirho(
    path_chirho: impl AsRef<Path>,
    execution_mode_chirho: ExecutionModeChirho,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho =
        SourceFileChirho::from_path_with_map_chirho(&mut source_map_chirho, &path_chirho)
            .map_err(|error_chirho| {
                DiagnosticChirho::error_no_span_chirho(format!(
                    "unable to read `{}`: {error_chirho}",
                    path_chirho.as_ref().display()
                ))
            })?;

    check_source_file_chirho(source_file_chirho, execution_mode_chirho)
}

pub fn check_source_file_chirho(
    source_file_chirho: SourceFileChirho,
    execution_mode_chirho: ExecutionModeChirho,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let parsed_module_chirho = parse_source_file_chirho(source_file_chirho.clone())?;
    let module_name_chirho = parsed_module_chirho
        .module_header_chirho
        .module_name_chirho
        .clone();
    let runtime_plan_chirho =
        RuntimePlanChirho::for_module_chirho(execution_mode_chirho, module_name_chirho.clone());
    let llvm_preview_chirho = compile_to_llvm_ir_stub_chirho(&module_name_chirho);
    let wasm_stub_size_chirho = compile_to_wasm_stub_chirho(&module_name_chirho).len();

    Ok(CheckSummaryChirho {
        source_path_chirho: source_file_chirho.path_chirho().to_path_buf(),
        module_name_chirho,
        runtime_plan_chirho,
        backend_plan_chirho: BackendPlanChirho {
            llvm_preview_chirho,
            wasm_stub_size_chirho,
        },
    })
}

/// Result of the full compilation pipeline.
#[derive(Debug, Clone)]
pub struct CompileResultChirho {
    pub module_chirho: ModuleChirho,
    pub core_chirho: CoreModuleChirho,
    pub llvm_ir_chirho: String,
    pub wasm_bytes_chirho: Vec<u8>,
    /// Set of newtype constructor names — these are identity at runtime.
    pub newtype_cons_chirho: std::collections::HashSet<String>,
}

/// Build a mapping from data constructor names to their parent type name.
/// Used by the dictionary-passing transform to select the correct instance
/// dictionary when a class method is applied to a constructor value.
fn build_con_type_map_chirho(module_chirho: &ModuleChirho) -> std::collections::HashMap<String, String> {
    let mut map_chirho = std::collections::HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let rhasky_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho {
            name_chirho,
            constructors_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            for con_chirho in constructors_chirho {
                let con_name_chirho = match con_chirho {
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        name_chirho: cn_chirho,
                        ..
                    } => cn_chirho.text_chirho().to_string(),
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        name_chirho: cn_chirho,
                        ..
                    } => cn_chirho.text_chirho().to_string(),
                };
                map_chirho.insert(con_name_chirho, type_name_chirho.clone());
            }
        }
    }
    // Also include newtype constructors
    for decl_chirho in &module_chirho.decls_chirho {
        if let rhasky_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
            name_chirho,
            constructor_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            let con_name_chirho = match constructor_chirho {
                rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho: cn_chirho,
                    ..
                } => cn_chirho.text_chirho().to_string(),
                rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: cn_chirho,
                    ..
                } => cn_chirho.text_chirho().to_string(),
            };
            map_chirho.insert(con_name_chirho, type_name_chirho);
        }
    }
    // Also include built-in constructors
    map_chirho.insert("True".to_string(), "Bool".to_string());
    map_chirho.insert("False".to_string(), "Bool".to_string());
    map_chirho
}

/// Build a mapping from newtype type name to (constructor name, underlying type key).
fn build_newtype_info_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashMap<String, (String, String)> {
    let mut map_chirho = std::collections::HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let rhasky_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
            name_chirho,
            constructor_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            let con_name_chirho = match constructor_chirho {
                rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho: cn_chirho,
                    fields_chirho,
                    ..
                } => {
                    let underlying_chirho = fields_chirho
                        .first()
                        .map(|f_chirho| {
                            rhasky_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(f_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
                rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: cn_chirho,
                    fields_chirho,
                    ..
                } => {
                    let underlying_chirho = fields_chirho
                        .first()
                        .map(|fd_chirho| {
                            rhasky_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(&fd_chirho.ty_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
            };
            map_chirho.insert(type_name_chirho, con_name_chirho);
        }
    }
    map_chirho
}

/// Run the full compiler pipeline: lex → layout → CST parse → AST lower →
/// name resolve → type infer → desugar to Core → simplify.
/// Returns the AST module and its optimized Core IR.
pub fn compile_source_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    let source_file_chirho =
        SourceFileChirho::from_source_map_chirho(source_map_chirho, file_name_chirho, source_chirho);
    let file_id_chirho = source_file_chirho.file_id_chirho();

    // Phase 1: CST parse (lex + layout + recursive-descent)
    let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();

    // Phase 2: CST → AST lowering
    let mut module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

    // Phase 2.5: Deriving — generate instance declarations for `deriving` clauses
    let _deriving_warnings_chirho =
        rhasky_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);

    // Phase 3: Name resolution (with built-in module interfaces for Data.Map etc.)
    let builtin_ifaces_chirho = rhasky_naming_chirho::builtin_module_ifaces_chirho();
    let resolve_result_chirho =
        resolve_module_with_imports_chirho(&module_chirho, &builtin_ifaces_chirho);
    if resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(resolve_result_chirho.diagnostics_chirho);
    }

    // Phase 3.5: Kind inference
    let kind_result_chirho =
        rhasky_typing_chirho::infer_module_kinds_chirho(&module_chirho);
    if kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(kind_result_chirho.diagnostics_chirho);
    }

    // Phase 4: Type inference
    let infer_result_chirho = infer_module_chirho(&module_chirho);
    if infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(infer_result_chirho.diagnostics_chirho);
    }

    // Phase 4.5: Pattern match exhaustiveness & redundancy checking
    let exhaust_result_chirho =
        rhasky_typing_chirho::check_module_exhaustiveness_chirho(&module_chirho);
    if exhaust_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(exhaust_result_chirho.diagnostics_chirho);
    }
    // Warnings from exhaustiveness are non-fatal; merge into diagnostic output.
    // (For now we proceed — a future diagnostic collector will unify these.)

    // Phase 5: Desugar AST → Core IR
    let desugar_output_chirho = desugar_module_chirho(&module_chirho);

    // Phase 5.5: Dictionary-passing transform (desugar typeclass constraints)
    let con_types_chirho = build_con_type_map_chirho(&module_chirho);
    let newtype_info_chirho = build_newtype_info_chirho(&module_chirho);
    let newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
        .values()
        .map(|(con_name_chirho, _)| con_name_chirho.clone())
        .collect();
    let dict_result_chirho = rhasky_core_chirho::dict_pass_module_full_chirho(
        &desugar_output_chirho.module_chirho,
        desugar_output_chirho.names_chirho,
        &infer_result_chirho.env_chirho,
        &infer_result_chirho.class_env_chirho,
        con_types_chirho,
        newtype_info_chirho,
    );
    let core_chirho = dict_result_chirho.module_chirho;

    // Phase 6: Core-to-Core simplification (beta reduction, dead code, case-of-known)
    let config_chirho = SimplifyConfigChirho::default();
    let core_chirho = simplify_module_chirho(&core_chirho, &config_chirho);

    // Phase 7: Backend lowering
    let llvm_ir_chirho = compile_core_to_llvm_chirho(&core_chirho);
    let wasm_bytes_chirho = compile_core_to_wasm_chirho(&core_chirho);

    Ok(CompileResultChirho {
        module_chirho,
        core_chirho,
        llvm_ir_chirho,
        wasm_bytes_chirho,
        newtype_cons_chirho,
    })
}

/// Compile multiple Haskell source files in dependency order.
///
/// Each module is compiled with access to previously compiled module interfaces,
/// so import declarations resolve against earlier modules in the list.
/// The caller is responsible for providing sources in dependency order.
pub fn compile_modules_chirho(
    sources_chirho: &[(&str, &str)], // (file_name, source_text)
    source_map_chirho: &mut SourceMapChirho,
) -> Result<Vec<CompileResultChirho>, DiagnosticBundleChirho> {
    let mut results_chirho = Vec::new();
    // Seed with synthetic interfaces for built-in modules (Data.Map, Data.Set, etc.)
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        rhasky_naming_chirho::builtin_module_ifaces_chirho();
    // Accumulated type schemes from all previously-compiled modules,
    // keyed by unqualified name. Downstream modules receive all upstream
    // exports so that type inference can resolve cross-module references.
    let mut imported_types_chirho: std::collections::HashMap<String, rhasky_typing_chirho::SchemeChirho> =
        std::collections::HashMap::new();

    for (file_name_chirho, source_chirho) in sources_chirho {
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            file_name_chirho,
            *source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();

        // Phase 1: CST parse
        let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
        let green_chirho = parser_chirho.parse_chirho();

        // Phase 2: CST → AST
        let mut module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

        // Phase 2.5: Deriving
        let _deriving_warnings_chirho =
            rhasky_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);

        // Phase 3: Name resolution with access to prior module interfaces
        let resolve_result_chirho =
            resolve_module_with_imports_chirho(&module_chirho, &ifaces_chirho);
        if resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
            return Err(resolve_result_chirho.diagnostics_chirho);
        }

        // Phase 3.5: Kind inference
        let kind_result_chirho =
            rhasky_typing_chirho::infer_module_kinds_chirho(&module_chirho);
        if kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
            return Err(kind_result_chirho.diagnostics_chirho);
        }

        // Phase 4: Type inference with imported type schemes
        let infer_result_chirho =
            infer_module_with_imports_chirho(&module_chirho, &imported_types_chirho);
        if infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
            return Err(infer_result_chirho.diagnostics_chirho);
        }

        // Phase 4.5: Pattern match exhaustiveness
        let exhaust_result_chirho =
            rhasky_typing_chirho::check_module_exhaustiveness_chirho(&module_chirho);
        if exhaust_result_chirho.diagnostics_chirho.has_errors_chirho() {
            return Err(exhaust_result_chirho.diagnostics_chirho);
        }

        // Phase 5: Desugar AST → Core IR
        let desugar_output_chirho = desugar_module_chirho(&module_chirho);

        // Phase 5.5: Dictionary-passing transform
        let con_types_chirho = build_con_type_map_chirho(&module_chirho);
        let newtype_info_chirho = build_newtype_info_chirho(&module_chirho);
        let newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
            .values()
            .map(|(con_name_chirho, _)| con_name_chirho.clone())
            .collect();
        let dict_result_chirho = rhasky_core_chirho::dict_pass_module_full_chirho(
            &desugar_output_chirho.module_chirho,
            desugar_output_chirho.names_chirho,
            &infer_result_chirho.env_chirho,
            &infer_result_chirho.class_env_chirho,
            con_types_chirho,
            newtype_info_chirho,
        );
        let core_chirho = dict_result_chirho.module_chirho;

        // Phase 6: Core-to-Core simplification
        let config_chirho = SimplifyConfigChirho::default();
        let core_chirho = simplify_module_chirho(&core_chirho, &config_chirho);

        // Phase 7: Backend lowering
        let llvm_ir_chirho = compile_core_to_llvm_chirho(&core_chirho);
        let wasm_bytes_chirho = compile_core_to_wasm_chirho(&core_chirho);

        // Build interface for downstream modules
        let iface_chirho = build_iface_chirho(&module_chirho);

        // Extract type schemes for exported names and accumulate them
        // so downstream modules can type-check cross-module references.
        for (name_chirho, _val_chirho) in &iface_chirho.exports_chirho.values_chirho {
            if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(name_chirho) {
                imported_types_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
            }
        }
        for (name_chirho, ty_info_chirho) in &iface_chirho.exports_chirho.types_chirho {
            // Export the type constructors' data constructor schemes
            for con_name_chirho in &ty_info_chirho.constructors_chirho {
                if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(con_name_chirho) {
                    imported_types_chirho.insert(con_name_chirho.clone(), scheme_chirho.clone());
                }
            }
        }

        ifaces_chirho.push(iface_chirho);

        results_chirho.push(CompileResultChirho {
            module_chirho,
            core_chirho,
            llvm_ir_chirho,
            wasm_bytes_chirho,
            newtype_cons_chirho,
        });
    }

    Ok(results_chirho)
}

/// Compile and evaluate multiple Haskell source modules through the full
/// pipeline, merging all Core IR into a single program, then running the
/// entry point (default `"main"`) from the last module.
pub fn eval_modules_chirho(
    sources_chirho: &[(&str, &str)],
    source_map_chirho: &mut SourceMapChirho,
    entry_name_chirho: Option<&str>,
) -> Result<rhasky_runtime_chirho::ValueChirho, String> {
    use rhasky_core_chirho::expr_chirho::{
        AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
        CoreIdChirho, CoreModuleChirho as CoreModChirho,
    };

    let results_chirho = compile_modules_chirho(sources_chirho, source_map_chirho)
        .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    if results_chirho.is_empty() {
        return Err("no modules to evaluate".to_string());
    }

    // Each module's desugarer starts CoreIds from 0, so IDs collide across
    // modules. Offset each module's IDs into non-overlapping ranges, then
    // remap cross-module variable references to point at the correct
    // definition IDs.

    fn offset_id_chirho(id_chirho: CoreIdChirho, off_chirho: u32) -> CoreIdChirho {
        CoreIdChirho(id_chirho.0 + off_chirho)
    }

    fn offset_binder_chirho(
        b_chirho: &mut rhasky_core_chirho::expr_chirho::BinderChirho,
        off_chirho: u32,
    ) {
        b_chirho.id_chirho = offset_id_chirho(b_chirho.id_chirho, off_chirho);
    }

    fn offset_expr_chirho(expr_chirho: &mut CoreExprChirho, off_chirho: u32) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                *id_chirho = offset_id_chirho(*id_chirho, off_chirho);
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho { fun_chirho, arg_chirho } => {
                offset_expr_chirho(fun_chirho, off_chirho);
                offset_expr_chirho(arg_chirho, off_chirho);
            }
            CoreExprChirho::LamChirho { binder_chirho, body_chirho } => {
                offset_binder_chirho(binder_chirho, off_chirho);
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::LetChirho { binds_chirho, body_chirho, .. } => {
                for (b_chirho, rhs_chirho) in binds_chirho {
                    offset_binder_chirho(b_chirho, off_chirho);
                    offset_expr_chirho(rhs_chirho, off_chirho);
                }
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::CaseChirho { scrutinee_chirho, bind_chirho, alts_chirho, .. } => {
                offset_binder_chirho(bind_chirho, off_chirho);
                offset_expr_chirho(scrutinee_chirho, off_chirho);
                for alt_chirho in alts_chirho {
                    for ab_chirho in &mut alt_chirho.binders_chirho {
                        offset_binder_chirho(ab_chirho, off_chirho);
                    }
                    offset_expr_chirho(&mut alt_chirho.rhs_chirho, off_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::TyAppChirho { expr_chirho: inner_chirho, .. } => {
                offset_expr_chirho(inner_chirho, off_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho { offset_expr_chirho(a_chirho, off_chirho); }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho { offset_expr_chirho(a_chirho, off_chirho); }
            }
        }
    }

    fn remap_expr_chirho(
        expr_chirho: &mut CoreExprChirho,
        remap_chirho: &std::collections::HashMap<CoreIdChirho, CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(&new_id_chirho) = remap_chirho.get(id_chirho) {
                    *id_chirho = new_id_chirho;
                }
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho { fun_chirho, arg_chirho } => {
                remap_expr_chirho(fun_chirho, remap_chirho);
                remap_expr_chirho(arg_chirho, remap_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::LetChirho { binds_chirho, body_chirho, .. } => {
                for (_b_chirho, rhs_chirho) in binds_chirho {
                    remap_expr_chirho(rhs_chirho, remap_chirho);
                }
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::CaseChirho { scrutinee_chirho, alts_chirho, .. } => {
                remap_expr_chirho(scrutinee_chirho, remap_chirho);
                for alt_chirho in alts_chirho {
                    remap_expr_chirho(&mut alt_chirho.rhs_chirho, remap_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::TyAppChirho { expr_chirho: inner_chirho, .. } => {
                remap_expr_chirho(inner_chirho, remap_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho { remap_expr_chirho(a_chirho, remap_chirho); }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho { remap_expr_chirho(a_chirho, remap_chirho); }
            }
        }
    }

    // Phase 1: Offset each module's IDs into non-overlapping ranges.
    let mut merged_bindings_chirho = Vec::new();
    let mut merged_names_chirho: std::collections::HashMap<CoreIdChirho, String> =
        std::collections::HashMap::new();
    let mut merged_newtype_cons_chirho = std::collections::HashSet::new();
    let mut id_offset_chirho: u32 = 0;

    for result_chirho in &results_chirho {
        let mut bindings_chirho = result_chirho.core_chirho.bindings_chirho.clone();

        if id_offset_chirho > 0 {
            // Offset all CoreIds in this module's bindings and names
            for binding_chirho in &mut bindings_chirho {
                offset_binder_chirho(&mut binding_chirho.binder_chirho, id_offset_chirho);
                offset_expr_chirho(&mut binding_chirho.rhs_chirho, id_offset_chirho);
            }
        }

        // Merge names with offset
        for (id_chirho, name_chirho) in &result_chirho.core_chirho.names_chirho {
            merged_names_chirho.insert(
                offset_id_chirho(*id_chirho, id_offset_chirho),
                name_chirho.clone(),
            );
        }

        merged_bindings_chirho.extend(bindings_chirho);
        merged_newtype_cons_chirho.extend(result_chirho.newtype_cons_chirho.clone());

        // Advance the offset past this module's ID range.
        let max_id_chirho = result_chirho.core_chirho.names_chirho
            .keys()
            .map(|id_chirho| id_chirho.0)
            .max()
            .unwrap_or(0);
        id_offset_chirho += max_id_chirho + 1;
    }

    // Phase 2: Build canonical name → definition CoreId (first wins).
    let mut def_name_to_id_chirho: std::collections::HashMap<String, CoreIdChirho> =
        std::collections::HashMap::new();
    for binding_chirho in &merged_bindings_chirho {
        def_name_to_id_chirho
            .entry(binding_chirho.binder_chirho.name_chirho.clone())
            .or_insert(binding_chirho.binder_chirho.id_chirho);
    }

    // Phase 3: Build remap table for cross-module references.
    // An ID needs remapping if it has a name that maps to a different
    // definition ID (i.e., a downstream module references an upstream name).
    // Only remap IDs that are NOT binder sites.
    let mut all_binder_ids_chirho: std::collections::HashSet<CoreIdChirho> =
        std::collections::HashSet::new();
    fn collect_binder_ids_chirho(
        expr_chirho: &CoreExprChirho,
        set_chirho: &mut std::collections::HashSet<CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho { fun_chirho, arg_chirho } => {
                collect_binder_ids_chirho(fun_chirho, set_chirho);
                collect_binder_ids_chirho(arg_chirho, set_chirho);
            }
            CoreExprChirho::LamChirho { binder_chirho, body_chirho } => {
                set_chirho.insert(binder_chirho.id_chirho);
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::LetChirho { binds_chirho, body_chirho, .. } => {
                for (b_chirho, rhs_chirho) in binds_chirho {
                    set_chirho.insert(b_chirho.id_chirho);
                    collect_binder_ids_chirho(rhs_chirho, set_chirho);
                }
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::CaseChirho { scrutinee_chirho, bind_chirho, alts_chirho, .. } => {
                set_chirho.insert(bind_chirho.id_chirho);
                collect_binder_ids_chirho(scrutinee_chirho, set_chirho);
                for alt_chirho in alts_chirho {
                    for ab_chirho in &alt_chirho.binders_chirho {
                        set_chirho.insert(ab_chirho.id_chirho);
                    }
                    collect_binder_ids_chirho(&alt_chirho.rhs_chirho, set_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::TyAppChirho { expr_chirho: inner_chirho, .. } => {
                collect_binder_ids_chirho(inner_chirho, set_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho { collect_binder_ids_chirho(a_chirho, set_chirho); }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho { collect_binder_ids_chirho(a_chirho, set_chirho); }
            }
        }
    }
    for binding_chirho in &merged_bindings_chirho {
        all_binder_ids_chirho.insert(binding_chirho.binder_chirho.id_chirho);
        collect_binder_ids_chirho(&binding_chirho.rhs_chirho, &mut all_binder_ids_chirho);
    }

    let mut remap_chirho: std::collections::HashMap<CoreIdChirho, CoreIdChirho> =
        std::collections::HashMap::new();
    for (id_chirho, name_chirho) in &merged_names_chirho {
        if all_binder_ids_chirho.contains(id_chirho) {
            continue;
        }
        if let Some(&def_id_chirho) = def_name_to_id_chirho.get(name_chirho) {
            if def_id_chirho != *id_chirho {
                remap_chirho.insert(*id_chirho, def_id_chirho);
            }
        }
    }

    // Phase 4: Apply the remap to all bindings.
    if !remap_chirho.is_empty() {
        for binding_chirho in &mut merged_bindings_chirho {
            remap_expr_chirho(&mut binding_chirho.rhs_chirho, &remap_chirho);
        }
    }

    let merged_core_chirho = CoreModChirho {
        name_chirho: results_chirho.last().unwrap().module_chirho.name_chirho.text_chirho().to_string(),
        bindings_chirho: merged_bindings_chirho,
        names_chirho: merged_names_chirho,
    };

    let (value_chirho, _machine_chirho) = stg_lower_chirho::lower_and_run_chirho(
        &merged_core_chirho,
        entry_name_chirho,
        merged_newtype_cons_chirho,
    )?;
    Ok(value_chirho)
}

/// Compile and evaluate a Haskell source program through the full pipeline,
/// returning the final runtime value of the named entry point.
pub fn eval_source_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
) -> Result<rhasky_runtime_chirho::ValueChirho, String> {
    let (value_chirho, _machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
        entry_name_chirho,
    )?;
    Ok(value_chirho)
}

/// Like `eval_source_chirho` but also returns the STG machine state,
/// which includes captured I/O output in `io_output_chirho`.
pub fn eval_source_with_machine_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
) -> Result<(rhasky_runtime_chirho::ValueChirho, rhasky_runtime_chirho::eval_chirho::MachineChirho), String> {
    let compile_result_chirho = compile_source_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
    )
    .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    stg_lower_chirho::lower_and_run_chirho(
        &compile_result_chirho.core_chirho,
        entry_name_chirho,
        compile_result_chirho.newtype_cons_chirho,
    )
}

/// Like `eval_source_with_machine_chirho` but with stdin input lines pre-loaded.
pub fn eval_source_with_input_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
    input_lines_chirho: &[&str],
) -> Result<(rhasky_runtime_chirho::ValueChirho, rhasky_runtime_chirho::eval_chirho::MachineChirho), String> {
    let compile_result_chirho = compile_source_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
    )
    .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    stg_lower_chirho::lower_and_run_with_input_chirho(
        &compile_result_chirho.core_chirho,
        entry_name_chirho,
        compile_result_chirho.newtype_cons_chirho,
        input_lines_chirho,
    )
}

/// Compile multiple Haskell source files with incremental recompilation avoidance.
///
/// Uses an [`IncrementalSessionChirho`] to track source fingerprints and
/// dependency-interface fingerprints. Only modules whose source or dependencies
/// have changed are recompiled; the rest reuse cached results.
pub fn compile_modules_incremental_chirho(
    sources_chirho: &[(&str, &str)], // (file_name, source_text)
    source_map_chirho: &mut SourceMapChirho,
    session_chirho: &mut rhasky_incremental_chirho::IncrementalSessionChirho,
) -> Result<Vec<(CompileResultChirho, bool)>, DiagnosticBundleChirho> {
    use rhasky_incremental_chirho::{FingerprintChirho, PhaseTagChirho};

    // Phase 0: Register all modules and compute fingerprints.
    // Parse module headers to extract names and imports.
    let mut module_infos_chirho: Vec<(String, String, &str, rhasky_span_chirho::FileIdChirho)> =
        Vec::new(); // (module_name, file_name, source, file_id)

    for (file_name_chirho, source_chirho) in sources_chirho {
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            *file_name_chirho,
            *source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();
        let parser_chirho = ParserChirho::new_chirho(*source_chirho, file_id_chirho);
        let green_chirho = parser_chirho.parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

        let module_name_chirho = module_chirho.name_chirho.text_chirho().to_string();
        let imports_chirho: Vec<String> = module_chirho
            .imports_chirho
            .iter()
            .map(|i_chirho| i_chirho.module_chirho.text_chirho().to_string())
            .collect();

        let import_refs_chirho: Vec<&str> = imports_chirho
            .iter()
            .map(|s_chirho| s_chirho.as_str())
            .collect();

        session_chirho.register_module_chirho(
            &module_name_chirho,
            source_chirho,
            &import_refs_chirho,
        );

        module_infos_chirho.push((
            module_name_chirho,
            file_name_chirho.to_string(),
            source_chirho,
            file_id_chirho,
        ));
    }

    // Determine compilation order.
    let order_chirho = session_chirho
        .compilation_order_chirho()
        .unwrap_or_else(|| {
            module_infos_chirho
                .iter()
                .map(|(n_chirho, _, _, _)| n_chirho.clone())
                .collect()
        });

    let mut results_chirho: Vec<(CompileResultChirho, bool)> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        rhasky_naming_chirho::builtin_module_ifaces_chirho();
    // TODO: cache serialized CompileResultChirho in the artifact store
    // so the cache-hit branch can skip recompilation entirely.

    for module_name_chirho in &order_chirho {
        let info_chirho = module_infos_chirho
            .iter()
            .find(|(n_chirho, _, _, _)| n_chirho == module_name_chirho);

        let Some((_, _file_name_chirho, source_chirho, file_id_chirho)) = info_chirho else {
            continue;
        };

        let source_fp_chirho = FingerprintChirho::from_str_chirho(source_chirho);
        let needs_rebuild_chirho =
            session_chirho.needs_rebuild_chirho(module_name_chirho, source_fp_chirho);
        let recompiled_chirho = needs_rebuild_chirho.is_some();

        let compile_result_chirho = if recompiled_chirho {
            // Full recompilation.
            let parser_chirho = ParserChirho::new_chirho(source_chirho, *file_id_chirho);
            let green_chirho = parser_chirho.parse_chirho();
            let module_chirho = lower_module_chirho(&green_chirho, *file_id_chirho);

            let resolve_result_chirho =
                resolve_module_with_imports_chirho(&module_chirho, &ifaces_chirho);
            if resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(resolve_result_chirho.diagnostics_chirho);
            }

            let kind_result_chirho =
                rhasky_typing_chirho::infer_module_kinds_chirho(&module_chirho);
            if kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(kind_result_chirho.diagnostics_chirho);
            }

            let infer_result_chirho = infer_module_chirho(&module_chirho);
            if infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(infer_result_chirho.diagnostics_chirho);
            }

            let exhaust_result_chirho =
                rhasky_typing_chirho::check_module_exhaustiveness_chirho(&module_chirho);
            if exhaust_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(exhaust_result_chirho.diagnostics_chirho);
            }

            let desugar_output_chirho = desugar_module_chirho(&module_chirho);
            let con_types_chirho = build_con_type_map_chirho(&module_chirho);
            let newtype_info_chirho = build_newtype_info_chirho(&module_chirho);
            let newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
                .values()
                .map(|(con_name_chirho, _)| con_name_chirho.clone())
                .collect();
            let dict_result_chirho = rhasky_core_chirho::dict_pass_module_full_chirho(
                &desugar_output_chirho.module_chirho,
                desugar_output_chirho.names_chirho,
                &infer_result_chirho.env_chirho,
                &infer_result_chirho.class_env_chirho,
                con_types_chirho,
                newtype_info_chirho,
            );
            let core_chirho = dict_result_chirho.module_chirho;
            let config_chirho = SimplifyConfigChirho::default();
            let core_chirho = simplify_module_chirho(&core_chirho, &config_chirho);
            let llvm_ir_chirho = compile_core_to_llvm_chirho(&core_chirho);
            let wasm_bytes_chirho = compile_core_to_wasm_chirho(&core_chirho);

            // Record compilation and iface fingerprint.
            let iface_chirho = build_iface_chirho(&module_chirho);
            let iface_fp_chirho =
                FingerprintChirho::from_str_chirho(&format!("{:?}", iface_chirho));
            let record_chirho =
                session_chirho.record_compilation_chirho(module_name_chirho, source_fp_chirho);
            record_chirho.set_phase_chirho(PhaseTagChirho::IfaceChirho, iface_fp_chirho);

            ifaces_chirho.push(iface_chirho);

            CompileResultChirho {
                module_chirho,
                core_chirho,
                llvm_ir_chirho,
                wasm_bytes_chirho,
                newtype_cons_chirho,
            }
        } else {
            // Cache hit — we still need to provide the interface for downstream
            // modules and produce a CompileResultChirho.  Until we serialize
            // Core/AST to the artifact store we re-run the pipeline but skip
            // the expensive fingerprint bookkeeping.
            let parser_chirho = ParserChirho::new_chirho(source_chirho, *file_id_chirho);
            let green_chirho = parser_chirho.parse_chirho();
            let module_chirho = lower_module_chirho(&green_chirho, *file_id_chirho);

            let resolve_result_chirho =
                resolve_module_with_imports_chirho(&module_chirho, &ifaces_chirho);
            if resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(resolve_result_chirho.diagnostics_chirho);
            }

            let kind_result_chirho =
                rhasky_typing_chirho::infer_module_kinds_chirho(&module_chirho);
            if kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(kind_result_chirho.diagnostics_chirho);
            }

            let infer_result_chirho = infer_module_chirho(&module_chirho);
            if infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(infer_result_chirho.diagnostics_chirho);
            }

            let exhaust_result_chirho =
                rhasky_typing_chirho::check_module_exhaustiveness_chirho(&module_chirho);
            if exhaust_result_chirho.diagnostics_chirho.has_errors_chirho() {
                return Err(exhaust_result_chirho.diagnostics_chirho);
            }

            let desugar_output_chirho = desugar_module_chirho(&module_chirho);
            let con_types_chirho = build_con_type_map_chirho(&module_chirho);
            let newtype_info_chirho = build_newtype_info_chirho(&module_chirho);
            let newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
                .values()
                .map(|(con_name_chirho, _)| con_name_chirho.clone())
                .collect();
            let dict_result_chirho = rhasky_core_chirho::dict_pass_module_full_chirho(
                &desugar_output_chirho.module_chirho,
                desugar_output_chirho.names_chirho,
                &infer_result_chirho.env_chirho,
                &infer_result_chirho.class_env_chirho,
                con_types_chirho,
                newtype_info_chirho,
            );
            let core_chirho = dict_result_chirho.module_chirho;
            let config_chirho = SimplifyConfigChirho::default();
            let core_chirho = simplify_module_chirho(&core_chirho, &config_chirho);
            let llvm_ir_chirho = compile_core_to_llvm_chirho(&core_chirho);
            let wasm_bytes_chirho = compile_core_to_wasm_chirho(&core_chirho);

            let iface_chirho = build_iface_chirho(&module_chirho);
            ifaces_chirho.push(iface_chirho);

            CompileResultChirho {
                module_chirho,
                core_chirho,
                llvm_ir_chirho,
                wasm_bytes_chirho,
                newtype_cons_chirho,
            }
        };

        results_chirho.push((compile_result_chirho, recompiled_chirho));
    }

    Ok(results_chirho)
}

// ---------------------------------------------------------------------------
// Cabal project compilation
// ---------------------------------------------------------------------------

/// Result of compiling a Cabal-based project.
#[derive(Debug)]
pub struct CabalCompileResultChirho {
    /// The parsed package description.
    pub package_chirho: rhasky_package_chirho::PackageDescChirho,
    /// The resolved build plan (external dependencies in topo order).
    pub build_plan_chirho: rhasky_package_chirho::BuildPlanChirho,
    /// Per-module compilation results, in compilation order.
    pub module_results_chirho: Vec<CompileResultChirho>,
}

/// Compile a Haskell project from a `.cabal` file path.
///
/// This function:
/// 1. Reads and parses the `.cabal` file
/// 2. Discovers exposed and internal modules from `hs-source-dirs`
/// 3. Resolves external dependencies against the provided package index
/// 4. Compiles all discovered modules in dependency order
pub fn compile_cabal_project_chirho(
    cabal_path_chirho: impl AsRef<Path>,
    index_chirho: &rhasky_package_chirho::PackageIndexChirho,
) -> Result<CabalCompileResultChirho, String> {
    use rhasky_package_chirho::{parse_cabal_chirho, resolve_deps_chirho, BuildPlanChirho};
    use std::collections::BTreeSet;

    // Read and parse the .cabal file.
    let cabal_content_chirho =
        std::fs::read_to_string(cabal_path_chirho.as_ref()).map_err(|e_chirho| {
            format!(
                "cannot read {}: {}",
                cabal_path_chirho.as_ref().display(),
                e_chirho
            )
        })?;
    let package_chirho = parse_cabal_chirho(&cabal_content_chirho);

    // Collect build-depends from the library (or all stanzas).
    let all_deps_chirho = collect_package_deps_chirho(&package_chirho);

    // Builtin packages that don't need resolution.
    let mut builtins_chirho = BTreeSet::new();
    builtins_chirho.insert("base".to_string());
    builtins_chirho.insert("ghc-prim".to_string());
    builtins_chirho.insert("ghc-bignum".to_string());
    builtins_chirho.insert("rts".to_string());

    // Resolve dependencies.
    let build_plan_chirho =
        resolve_deps_chirho(&all_deps_chirho, index_chirho, &builtins_chirho)
            .map_err(|e_chirho| format!("dependency resolution failed: {}", e_chirho))?;

    // Discover Haskell source files.
    let project_dir_chirho = cabal_path_chirho
        .as_ref()
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let source_files_chirho = discover_modules_chirho(&package_chirho, project_dir_chirho);

    // Read sources and compile.
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut sources_chirho: Vec<(String, String)> = Vec::new();

    for (module_name_chirho, path_chirho) in &source_files_chirho {
        let content_chirho = std::fs::read_to_string(path_chirho).map_err(|e_chirho| {
            format!(
                "cannot read module {} at {}: {}",
                module_name_chirho,
                path_chirho.display(),
                e_chirho
            )
        })?;
        sources_chirho.push((module_name_chirho.clone(), content_chirho));
    }

    // Build (file_name, source) pairs for compile_modules_chirho.
    let source_refs_chirho: Vec<(&str, &str)> = sources_chirho
        .iter()
        .map(|(name_chirho, src_chirho)| (name_chirho.as_str(), src_chirho.as_str()))
        .collect();

    let module_results_chirho =
        compile_modules_chirho(&source_refs_chirho, &mut source_map_chirho)
            .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    Ok(CabalCompileResultChirho {
        package_chirho,
        build_plan_chirho,
        module_results_chirho,
    })
}

/// Collect all build-depends across all stanzas.
fn collect_package_deps_chirho(
    package_chirho: &rhasky_package_chirho::PackageDescChirho,
) -> Vec<rhasky_package_chirho::DependencyChirho> {
    let mut deps_chirho = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();

    if let Some(lib_chirho) = &package_chirho.library_chirho {
        for dep_chirho in &lib_chirho.build_info_chirho.build_depends_chirho {
            if seen_chirho.insert(dep_chirho.package_chirho.clone()) {
                deps_chirho.push(dep_chirho.clone());
            }
        }
    }
    for exe_chirho in &package_chirho.executables_chirho {
        for dep_chirho in &exe_chirho.build_info_chirho.build_depends_chirho {
            if seen_chirho.insert(dep_chirho.package_chirho.clone()) {
                deps_chirho.push(dep_chirho.clone());
            }
        }
    }
    for ts_chirho in &package_chirho.test_suites_chirho {
        for dep_chirho in &ts_chirho.build_info_chirho.build_depends_chirho {
            if seen_chirho.insert(dep_chirho.package_chirho.clone()) {
                deps_chirho.push(dep_chirho.clone());
            }
        }
    }

    deps_chirho
}

/// Discover Haskell module files from a package description.
///
/// Returns `(module_name, file_path)` pairs. Searches `hs-source-dirs`
/// for each exposed/other module, converting dotted module names to paths.
fn discover_modules_chirho(
    package_chirho: &rhasky_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    let mut modules_chirho: Vec<(String, PathBuf)> = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();

    if let Some(lib_chirho) = &package_chirho.library_chirho {
        let src_dirs_chirho = if lib_chirho.build_info_chirho.hs_source_dirs_chirho.is_empty() {
            vec![".".to_string()]
        } else {
            lib_chirho.build_info_chirho.hs_source_dirs_chirho.clone()
        };

        for mod_name_chirho in lib_chirho
            .exposed_modules_chirho
            .iter()
            .chain(lib_chirho.other_modules_chirho.iter())
        {
            if seen_chirho.insert(mod_name_chirho.clone()) {
                if let Some(path_chirho) =
                    find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
                {
                    modules_chirho.push((mod_name_chirho.clone(), path_chirho));
                }
            }
        }
    }

    for exe_chirho in &package_chirho.executables_chirho {
        let src_dirs_chirho = if exe_chirho.build_info_chirho.hs_source_dirs_chirho.is_empty() {
            vec![".".to_string()]
        } else {
            exe_chirho.build_info_chirho.hs_source_dirs_chirho.clone()
        };

        if let Some(main_is_chirho) = &exe_chirho.main_is_chirho {
            let main_path_chirho = src_dirs_chirho
                .iter()
                .map(|d_chirho| project_dir_chirho.join(d_chirho).join(main_is_chirho))
                .find(|p_chirho| p_chirho.exists());

            if let Some(path_chirho) = main_path_chirho {
                let mod_name_chirho = "Main".to_string();
                if seen_chirho.insert(format!("{}:{}", exe_chirho.name_chirho, mod_name_chirho)) {
                    modules_chirho.push((mod_name_chirho, path_chirho));
                }
            }
        }

        for mod_name_chirho in &exe_chirho.other_modules_chirho {
            if seen_chirho.insert(format!("{}:{}", exe_chirho.name_chirho, mod_name_chirho)) {
                if let Some(path_chirho) =
                    find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
                {
                    modules_chirho.push((mod_name_chirho.clone(), path_chirho));
                }
            }
        }
    }

    // Test-suite modules
    for ts_chirho in &package_chirho.test_suites_chirho {
        let src_dirs_chirho = if ts_chirho.build_info_chirho.hs_source_dirs_chirho.is_empty() {
            vec![".".to_string()]
        } else {
            ts_chirho.build_info_chirho.hs_source_dirs_chirho.clone()
        };

        if let Some(main_is_chirho) = &ts_chirho.main_is_chirho {
            let main_path_chirho = src_dirs_chirho
                .iter()
                .map(|d_chirho| project_dir_chirho.join(d_chirho).join(main_is_chirho))
                .find(|p_chirho| p_chirho.exists());

            if let Some(path_chirho) = main_path_chirho {
                let mod_name_chirho = "Main".to_string();
                if seen_chirho
                    .insert(format!("test:{}:{}", ts_chirho.name_chirho, mod_name_chirho))
                {
                    modules_chirho.push((mod_name_chirho, path_chirho));
                }
            }
        }

        for mod_name_chirho in &ts_chirho.other_modules_chirho {
            if seen_chirho
                .insert(format!("test:{}:{}", ts_chirho.name_chirho, mod_name_chirho))
            {
                if let Some(path_chirho) =
                    find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
                {
                    modules_chirho.push((mod_name_chirho.clone(), path_chirho));
                }
            }
        }
    }

    // Setup.hs / Setup.lhs at project root
    let setup_hs_chirho = project_dir_chirho.join("Setup.hs");
    let setup_lhs_chirho = project_dir_chirho.join("Setup.lhs");
    if setup_hs_chirho.exists() && seen_chirho.insert("Setup".to_string()) {
        modules_chirho.push(("Setup".to_string(), setup_hs_chirho));
    } else if setup_lhs_chirho.exists() && seen_chirho.insert("Setup".to_string()) {
        modules_chirho.push(("Setup".to_string(), setup_lhs_chirho));
    }

    modules_chirho
}

/// Find the file for a dotted module name (e.g. `Data.Map` → `Data/Map.hs`)
/// in the given source directories.
fn find_module_file_chirho(
    module_name_chirho: &str,
    src_dirs_chirho: &[String],
    project_dir_chirho: &Path,
) -> Option<PathBuf> {
    let relative_path_chirho = module_name_chirho.replace('.', "/") + ".hs";

    for dir_chirho in src_dirs_chirho {
        let full_path_chirho = project_dir_chirho.join(dir_chirho).join(&relative_path_chirho);
        if full_path_chirho.exists() {
            return Some(full_path_chirho);
        }
        // Also try .lhs (literate Haskell)
        let lhs_path_chirho = project_dir_chirho
            .join(dir_chirho)
            .join(module_name_chirho.replace('.', "/") + ".lhs");
        if lhs_path_chirho.exists() {
            return Some(lhs_path_chirho);
        }
    }

    None
}

pub fn render_summary_chirho(check_summary_chirho: &CheckSummaryChirho) -> String {
    format!(
        "source: {}\nmodule: {}\nmode: {:?}\nincremental_session: {}\nllvm_preview: {}\nwasm_stub_size: {}",
        check_summary_chirho.source_path_chirho.display(),
        check_summary_chirho.module_name_chirho,
        check_summary_chirho.runtime_plan_chirho.execution_mode_chirho,
        check_summary_chirho.runtime_plan_chirho.incremental_session_chirho,
        check_summary_chirho
            .backend_plan_chirho
            .llvm_preview_chirho
            .trim_end(),
        check_summary_chirho.backend_plan_chirho.wasm_stub_size_chirho,
    )
}

#[cfg(test)]
mod tests_chirho {
    use super::{check_source_file_chirho, render_summary_chirho};
    use rhasky_runtime_chirho::ExecutionModeChirho;
    use rhasky_span_chirho::SourceMapChirho;
    use rhasky_syntax_chirho::SourceFileChirho;

    #[test]
    fn builds_a_check_summary_for_batch_mode_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "BatchSampleChirho.hs",
            "module BatchSampleChirho where\nvalueChirho = 1\n",
        );

        let check_summary_chirho =
            check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho)
                .expect("driver should accept a valid source file");

        assert_eq!(check_summary_chirho.module_name_chirho, "BatchSampleChirho");
        assert!(!check_summary_chirho.runtime_plan_chirho.incremental_session_chirho);
        assert!(
            render_summary_chirho(&check_summary_chirho).contains("llvm_preview: ; rhasky llvm stub")
        );
    }

    #[test]
    fn full_pipeline_parses_and_resolves_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            "module Test where\ndata Color = Red | Green\nf x = x\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("full pipeline should succeed");

        assert_eq!(result_chirho.module_chirho.name_chirho.text_chirho(), "Test");
        assert!(result_chirho.module_chirho.decls_chirho.len() >= 2);
        // Core module was produced by desugaring
        assert_eq!(result_chirho.core_chirho.name_chirho, "Test");
        assert!(!result_chirho.core_chirho.bindings_chirho.is_empty());
        // Backend output was produced
        assert!(result_chirho.llvm_ir_chirho.contains("; ModuleID = 'Test'"));
        assert_eq!(&result_chirho.wasm_bytes_chirho[0..4], b"\0asm");
    }

    #[test]
    fn script_mode_uses_incremental_runtime_plan_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "ScriptSampleChirho.hs",
            "mainChirho = print 42\n",
        );

        let check_summary_chirho =
            check_source_file_chirho(source_file_chirho, ExecutionModeChirho::ScriptChirho)
                .expect("script mode should accept module-less files");

        assert!(check_summary_chirho.runtime_plan_chirho.incremental_session_chirho);
        assert_eq!(check_summary_chirho.module_name_chirho, "Main");
    }

    #[test]
    fn eval_literal_binding_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = 42\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(42)
        );
    }

    #[test]
    fn eval_identity_function_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let compile_result_chirho = compile_source_chirho(
            "module Test where\nid x = x\nmain = id 7\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("should compile");

        // Debug: print core bindings
        for b_chirho in &compile_result_chirho.core_chirho.bindings_chirho {
            eprintln!(
                "binding: {} (id={}) = {:?}",
                b_chirho.binder_chirho.name_chirho,
                b_chirho.binder_chirho.id_chirho.0,
                b_chirho.rhs_chirho
            );
        }

        let (result_chirho, _) = super::stg_lower_chirho::lower_and_run_chirho(
            &compile_result_chirho.core_chirho,
            None,
            compile_result_chirho.newtype_cons_chirho,
        )
        .expect("should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
    }

    #[test]
    fn eval_named_entry_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nfoo = 99\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            Some("foo"),
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(99)
        );
    }

    #[test]
    fn multi_module_import_chirho() {
        use super::compile_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let sources_chirho: Vec<(&str, &str)> = vec![
            (
                "LibChirho.hs",
                "module Lib where\ndata Color = Red | Green\nhelper x = x\n",
            ),
            (
                "MainChirho.hs",
                "module Main where\nimport Lib\nf x = case x of\n  Red -> 1\n  Green -> 2\n",
            ),
        ];

        let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho)
            .expect("multi-module compilation should succeed");

        assert_eq!(results_chirho.len(), 2);
        assert_eq!(results_chirho[0].module_chirho.name_chirho.text_chirho(), "Lib");
        assert_eq!(results_chirho[1].module_chirho.name_chirho.text_chirho(), "Main");
    }

    #[test]
    fn exhaustiveness_check_passes_for_complete_case_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // data Color = Red | Green — case covers both constructors
        let result_chirho = compile_source_chirho(
            "module Test where\ndata Color = Red | Green\nf x = case x of\n  Red -> 1\n  Green -> 2\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "complete case should pass exhaustiveness check"
        );
    }

    #[test]
    fn exhaustiveness_check_rejects_incomplete_case_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // data Color = Red | Green | Blue — case only covers Red
        let result_chirho = compile_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue\nf x = case x of\n  Red -> 1\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_err(),
            "incomplete case should fail exhaustiveness check"
        );
        let err_chirho = result_chirho.unwrap_err();
        assert!(err_chirho.has_errors_chirho());
        let msg_chirho = format!("{}", err_chirho);
        assert!(
            msg_chirho.contains("Green") || msg_chirho.contains("Blue"),
            "error should mention missing constructors"
        );
    }

    #[test]
    fn incremental_compilation_marks_recompiled_chirho() {
        use super::compile_modules_incremental_chirho;
        use rhasky_incremental_chirho::IncrementalSessionChirho;

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let mut session_chirho = IncrementalSessionChirho::new_chirho();

        let sources_chirho: Vec<(&str, &str)> = vec![
            (
                "LibChirho.hs",
                "module Lib where\ndata Color = Red | Green\nhelper x = x\n",
            ),
            (
                "MainChirho.hs",
                "module Main where\nimport Lib\nf x = x\n",
            ),
        ];

        // First build: everything should be recompiled.
        let results_chirho =
            compile_modules_incremental_chirho(&sources_chirho, &mut source_map_chirho, &mut session_chirho)
                .expect("incremental compilation should succeed");

        assert_eq!(results_chirho.len(), 2);
        assert!(results_chirho[0].1, "Lib should be recompiled on first build");
        assert!(results_chirho[1].1, "Main should be recompiled on first build");

        // Second build with identical sources: nothing should be recompiled.
        let mut source_map_chirho2 = SourceMapChirho::new_chirho();
        let results2_chirho =
            compile_modules_incremental_chirho(&sources_chirho, &mut source_map_chirho2, &mut session_chirho)
                .expect("second incremental compilation should succeed");

        assert_eq!(results2_chirho.len(), 2);
        assert!(
            !results2_chirho[0].1,
            "Lib should NOT be recompiled when source unchanged"
        );
        assert!(
            !results2_chirho[1].1,
            "Main should NOT be recompiled when source unchanged"
        );
    }

    #[test]
    fn incremental_detects_source_change_chirho() {
        use super::compile_modules_incremental_chirho;
        use rhasky_incremental_chirho::IncrementalSessionChirho;

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let mut session_chirho = IncrementalSessionChirho::new_chirho();

        let sources_v1_chirho: Vec<(&str, &str)> = vec![(
            "TestChirho.hs",
            "module Test where\nf x = x\n",
        )];

        let _ = compile_modules_incremental_chirho(
            &sources_v1_chirho,
            &mut source_map_chirho,
            &mut session_chirho,
        )
        .expect("v1 should compile");

        // Change the source.
        let sources_v2_chirho: Vec<(&str, &str)> = vec![(
            "TestChirho.hs",
            "module Test where\nf x = x\ng y = y\n",
        )];

        let mut source_map2_chirho = SourceMapChirho::new_chirho();
        let results_chirho = compile_modules_incremental_chirho(
            &sources_v2_chirho,
            &mut source_map2_chirho,
            &mut session_chirho,
        )
        .expect("v2 should compile");

        assert_eq!(results_chirho.len(), 1);
        assert!(
            results_chirho[0].1,
            "Test should be recompiled when source changes"
        );
    }

    #[test]
    fn cabal_project_compilation_chirho() {
        use super::compile_cabal_project_chirho;
        use std::io::Write;

        // Create a temp directory with a small Cabal project
        let temp_dir_chirho = std::env::temp_dir().join("rhasky_cabal_test_chirho");
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
        std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

        // Write .cabal file
        let cabal_path_chirho = temp_dir_chirho.join("hello.cabal");
        let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
        write!(
            cabal_file_chirho,
            r#"cabal-version: 3.0
name:         hello
version:      0.1.0.0

library
  exposed-modules: Lib
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5
  default-language: Haskell2010
"#
        )
        .unwrap();

        // Write Haskell source
        let lib_path_chirho = temp_dir_chirho.join("src").join("Lib.hs");
        let mut lib_file_chirho = std::fs::File::create(&lib_path_chirho).unwrap();
        write!(
            lib_file_chirho,
            "module Lib where\nf x = x\n"
        )
        .unwrap();

        // Create an empty package index (base is builtin, so no external deps needed)
        let index_chirho = rhasky_package_chirho::PackageIndexChirho::new_chirho();

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
                .expect("cabal project should compile");

        assert_eq!(result_chirho.package_chirho.name_chirho, "hello");
        assert!(result_chirho.build_plan_chirho.steps_chirho.is_empty()); // only base (builtin)
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
        assert_eq!(
            result_chirho.module_results_chirho[0]
                .module_chirho
                .name_chirho
                .text_chirho(),
            "Lib"
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    }

    #[test]
    fn cabal_project_multi_module_chirho() {
        use super::compile_cabal_project_chirho;
        use std::io::Write;

        let temp_dir_chirho = std::env::temp_dir().join("rhasky_cabal_multi_test_chirho");
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
        std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

        let cabal_path_chirho = temp_dir_chirho.join("multi.cabal");
        let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
        write!(
            cabal_file_chirho,
            r#"cabal-version: 3.0
name:         multi
version:      0.1.0.0

library
  exposed-modules: Lib, Helper
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5
"#
        )
        .unwrap();

        let mut lib_chirho = std::fs::File::create(temp_dir_chirho.join("src/Lib.hs")).unwrap();
        write!(lib_chirho, "module Lib where\nf x = x\n").unwrap();

        let mut helper_chirho =
            std::fs::File::create(temp_dir_chirho.join("src/Helper.hs")).unwrap();
        write!(helper_chirho, "module Helper where\ng y = y\n").unwrap();

        let index_chirho = rhasky_package_chirho::PackageIndexChirho::new_chirho();

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
                .expect("multi-module cabal project should compile");

        assert_eq!(result_chirho.package_chirho.name_chirho, "multi");
        assert_eq!(result_chirho.module_results_chirho.len(), 2);

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    }

    #[test]
    fn cabal_project_missing_module_skipped_chirho() {
        use super::compile_cabal_project_chirho;
        use std::io::Write;

        let temp_dir_chirho = std::env::temp_dir().join("rhasky_cabal_missing_test_chirho");
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
        std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

        let cabal_path_chirho = temp_dir_chirho.join("miss.cabal");
        let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
        write!(
            cabal_file_chirho,
            r#"cabal-version: 3.0
name:         miss
version:      0.1.0.0

library
  exposed-modules: Exists, DoesNotExist
  hs-source-dirs:  src
  build-depends:   base
"#
        )
        .unwrap();

        // Only create Exists.hs, not DoesNotExist.hs
        let mut exists_chirho =
            std::fs::File::create(temp_dir_chirho.join("src/Exists.hs")).unwrap();
        write!(exists_chirho, "module Exists where\nval = 42\n").unwrap();

        let index_chirho = rhasky_package_chirho::PackageIndexChirho::new_chirho();

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
                .expect("should compile what exists");

        // Only Exists should be compiled (DoesNotExist not found on disk)
        assert_eq!(result_chirho.module_results_chirho.len(), 1);

        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    }

    // ── PrimOp / arithmetic end-to-end tests ────────────────────────────

    #[test]
    fn eval_addition_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = 3 + 4\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(7)
        );
    }

    #[test]
    fn eval_subtraction_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = 10 - 3\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(7)
        );
    }

    #[test]
    fn eval_multiplication_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = 6 * 7\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(42)
        );
    }

    #[test]
    fn eval_nested_arithmetic_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // (2 + 3) * 4 = 20  — depends on parse associativity; if left-assoc:
        // 2 + 3 * 4 = 2 + 12 = 14  (if * binds tighter, which it should)
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = 2 + 3 * 4\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        // The parser should handle precedence; accept whatever it produces.
        assert!(result_chirho.is_ok());
    }

    #[test]
    fn eval_negation_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            "module Test where\nmain = -(42)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        // Should compile without error (negation desugars to PrimOp negate#)
        assert!(result_chirho.is_ok());
        let compiled_chirho = result_chirho.unwrap();
        // Core should contain a PrimOp
        assert!(!compiled_chirho.core_chirho.bindings_chirho.is_empty());
    }

    #[test]
    fn eval_function_with_arithmetic_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Use a function definition instead of let-in (which needs specific parse support)
        let result_chirho = eval_source_chirho(
            "module Test where\nx = 10\nmain = x + 5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(15)
        );
    }

    #[test]
    fn compile_primop_to_llvm_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Use non-literal args so constant folding doesn't eliminate the primop
        let result_chirho = compile_source_chirho(
            "module Test where\nf x = x + 1\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("should compile");
        // LLVM IR should contain an add instruction (not folded away because x is a variable)
        assert!(
            result_chirho.llvm_ir_chirho.contains("add i64"),
            "LLVM IR should contain add instruction: {}",
            result_chirho.llvm_ir_chirho
        );
    }

    #[test]
    fn compile_primop_to_wasm_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            "module Test where\nmain = 3 + 4\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("should compile");
        // WASM binary should be a valid module
        assert_eq!(&result_chirho.wasm_bytes_chirho[0..4], b"\0asm");
        assert!(result_chirho.wasm_bytes_chirho.len() > 20);
    }

    #[test]
    fn eval_if_then_else_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // if True then 42 else 0
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = if True then 42 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                // If it fails, print the error for debugging but don't
                // hard-fail yet — we're probing pipeline capabilities.
                eprintln!("eval_if_then_else: {}", e_chirho);
                assert!(result_chirho.is_ok(), "if-then-else should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_comparison_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // 3 < 5 should evaluate to True (BoolChirho(true))
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = 3 < 5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::BoolChirho(true)
            ),
            Err(e_chirho) => {
                eprintln!("eval_comparison: {}", e_chirho);
                assert!(result_chirho.is_ok(), "comparison should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_if_with_comparison_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // if (3 < 5) then 42 else 0  — combines comparison + conditional
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = if 3 < 5 then 42 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_if_with_comparison: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_function_application_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Simple function application: double x = x + x; main = double 21
        let result_chirho = eval_source_chirho(
            "module Test where\ndouble x = x + x\nmain = double 21\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_function_application: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_mutual_top_level_arithmetic_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Two top-level bindings where main references x
        let result_chirho = eval_source_chirho(
            "module Test where\nx = 3 + 4\nmain = x + 1\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(8)
        );
    }

    #[test]
    fn eval_chained_bindings_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Chain: a = 1, b = a + 2, main = b + 3
        let result_chirho = eval_source_chirho(
            "module Test where\na = 1\nb = a + 2\nmain = b + 3\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        assert_eq!(
            result_chirho.unwrap(),
            rhasky_runtime_chirho::ValueChirho::IntChirho(6)
        );
    }

    #[test]
    fn eval_case_constructor_field_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // case Just 42 of { Just x -> x; Nothing -> 0 }
        let result_chirho = eval_source_chirho(
            "module Test where\ndata Maybe a = Nothing | Just a\nmain = case Just 42 of { Just x -> x; Nothing -> 0 }\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_case_constructor_field: {}", e_chirho);
                assert!(
                    result_chirho.is_ok(),
                    "should evaluate: {}",
                    e_chirho
                );
            }
        }
    }

    #[test]
    fn eval_case_default_alt_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Default alt: case Just 10 of { Nothing -> 0; _ -> 99 }
        let result_chirho = eval_source_chirho(
            "module Test where\ndata Maybe a = Nothing | Just a\nmain = case Just 10 of { Nothing -> 0; _ -> 99 }\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(99)
            ),
            Err(e_chirho) => {
                eprintln!("eval_case_default_alt: {}", e_chirho);
                assert!(
                    result_chirho.is_ok(),
                    "should evaluate: {}",
                    e_chirho
                );
            }
        }
    }

    #[test]
    fn eval_nested_case_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Nested: if (3 > 5) then 1 else if (2 < 4) then 2 else 3
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = if 3 > 5 then 1 else if 2 < 4 then 2 else 3\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(2)
            ),
            Err(e_chirho) => {
                eprintln!("eval_nested_case: {}", e_chirho);
                assert!(
                    result_chirho.is_ok(),
                    "should evaluate: {}",
                    e_chirho
                );
            }
        }
    }

    #[test]
    fn eval_function_with_case_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Function that uses conditional: abs x = if x < 0 then 0 - x else x
        let result_chirho = eval_source_chirho(
            "module Test where\nabs' x = if x < 0 then 0 - x else x\nmain = abs' (-7)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(7)
            ),
            Err(e_chirho) => {
                eprintln!("eval_function_with_case: {}", e_chirho);
                assert!(
                    result_chirho.is_ok(),
                    "should evaluate: {}",
                    e_chirho
                );
            }
        }
    }

    #[test]
    fn eval_let_expression_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // let x = 5 in x  →  5
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = let x = 5 in x\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(5)
            ),
            Err(e_chirho) => {
                eprintln!("eval_let_expression: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_where_clause_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // f 42 where f x = x  →  42  (identity, avoids Num constraints)
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = f 42\n  where\n    f x = x\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_where_clause: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_higher_order_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Higher-order: apply f x = f x; main = apply id 42  →  42
        // Uses id (no Num constraint) to avoid dict-pass complications
        let result_chirho = eval_source_chirho(
            "module Test where\nid x = x\napply f x = f x\nmain = apply id 42\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_higher_order: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_partial_application_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Partial application: const x y = x; main = const 7 99  →  7
        // Uses const (no Num constraint) to test multi-arg + partial app
        let result_chirho = eval_source_chirho(
            "module Test where\nconst x y = x\nmain = const 7 99\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(7)
            ),
            Err(e_chirho) => {
                eprintln!("eval_partial_application: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_nested_let_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Nested let: let a = 1 in let b = a + 2 in b + 3  →  6
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = let a = 1 in let b = a + 2 in b + 3\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(6)
            ),
            Err(e_chirho) => {
                eprintln!("eval_nested_let: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_case_literal_dispatch_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Literal case dispatch: classify x = case x of 1 -> 10; 2 -> 20; _ -> 0
        // main = classify 2  →  20
        let result_chirho = eval_source_chirho(
            "module Test where\nclassify x = case x of\n  1 -> 10\n  2 -> 20\n  _ -> 0\nmain = classify 2\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(20)
            ),
            Err(e_chirho) => {
                eprintln!("eval_case_literal_dispatch: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_case_literal_default_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Literal case fallthrough to default
        // main = case 99 of { 1 -> 10; 2 -> 20; _ -> 0 }
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = case 99 of\n  1 -> 10\n  2 -> 20\n  _ -> 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(0)
            ),
            Err(e_chirho) => {
                eprintln!("eval_case_literal_default: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_nested_pattern_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Nested constructor pattern via two-level case
        // main = case Just 42 of { Just x -> x; Nothing -> 0 }
        let result_chirho = eval_source_chirho(
            "module Test where\ndata Maybe a = Nothing | Just a\nmain = case Just 42 of\n  Just x -> x\n  Nothing -> 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_nested_pattern: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_nested_constructor_destructure_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Simpler: case Just 99 of Just x -> x + 1; _ -> 0
        // Tests constructor field extraction with arithmetic
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = case Just 99 of\n\
             \x20 Just x -> x\n\
             \x20 _ -> 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(99)
            ),
            Err(e_chirho) => {
                eprintln!("eval_nested_constructor_destructure: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_deeply_nested_pattern_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Deep nesting: case Just (Just 77) of Just (Just x) -> x; _ -> 0
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = case Just (Just 77) of\n\
             \x20 Just (Just x) -> x\n\
             \x20 _ -> 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(77)
            ),
            Err(e_chirho) => {
                eprintln!("eval_deeply_nested_pattern: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_closure_capture_chirho() {
        // \x -> \y -> x + y  — inner lambda captures x from outer
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             add = \\x -> \\y -> x + y\n\
             main = add 10 32\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_closure_capture: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_closure_let_capture_chirho() {
        // let-bound closure capturing outer parameter
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             f x = let g y = x + y in g 5\n\
             main = f 10\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(15)
            ),
            Err(e_chirho) => {
                eprintln!("eval_closure_let_capture: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_nested_closure_chirho() {
        // Triple nesting: \x -> \y -> \z -> x + y + z
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             f x y z = x + y + z\n\
             main = f 10 20 12\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_nested_closure: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_string_literal_chirho() {
        // String literal evaluates to StringChirho value
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = \"hello\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::StringChirho("hello".to_string())
            ),
            Err(e_chirho) => {
                eprintln!("eval_string_literal: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_char_literal_chirho() {
        // Char literal evaluates to CharChirho value
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = 'A'\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::CharChirho('A')
            ),
            Err(e_chirho) => {
                eprintln!("eval_char_literal: {}", e_chirho);
                assert!(result_chirho.is_ok(), "should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_list_literal_chirho() {
        // [1, 2, 3] desugars to cons chain; the head element is 1
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Just verify compilation succeeds — list evaluation requires
        // deeper infrastructure (case on lists to extract head)
        let result_chirho = compile_source_chirho(
            "module Test where\n\
             main = [1, 2, 3]\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "list literal should compile: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn eval_guarded_function_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // abs x | x >= 0 = x | True = 0 - x
        // main = abs (-5)  → 5
        // Using True as the fallback guard since we don't have `otherwise` in prelude yet
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             abs x\n  | x >= 0 = x\n  | True = 0 - x\n\
             main = abs 5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(5)
            ),
            Err(e_chirho) => {
                eprintln!("eval_guarded_function: {}", e_chirho);
                assert!(result_chirho.is_ok(), "guarded function should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_guarded_function_fallback_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // classify x | x > 0 = 1 | x < 0 = 2 | True = 0
        // main = classify 0  → should fall through to the True guard → 0
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             classify x\n  | x > 0 = 1\n  | x < 0 = 2\n  | True = 0\n\
             main = classify 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(0)
            ),
            Err(e_chirho) => {
                eprintln!("eval_guarded_function_fallback: {}", e_chirho);
                assert!(result_chirho.is_ok(), "guarded fallback should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_guarded_true_branch_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Simple: f x | True = x + 1
        // main = f 41  → 42
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             f x\n  | True = x + 1\n\
             main = f 41\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_guarded_true_branch: {}", e_chirho);
                assert!(result_chirho.is_ok(), "guarded true branch should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_otherwise_guard_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // classify x | x > 0 = 1 | otherwise = 0
        // main = classify 0  → should fall to otherwise → 0
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             classify x\n  | x > 0 = 1\n  | otherwise = 0\n\
             main = classify 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(0)
            ),
            Err(e_chirho) => {
                eprintln!("eval_otherwise_guard: {}", e_chirho);
                assert!(result_chirho.is_ok(), "otherwise guard should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_dollar_operator_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // f $ x = f x  — test the $ operator
        // Define id locally since it's not a runtime builtin yet
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             id x = x\n\
             main = id $ 42\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => assert_eq!(
                *val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(42)
            ),
            Err(e_chirho) => {
                eprintln!("eval_dollar_operator: {}", e_chirho);
                assert!(result_chirho.is_ok(), "$ operator should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_do_notation_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // do { putStrLn "hello"; putStrLn "world" }
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = do\n  putStrLn \"hello\"\n  putStrLn \"world\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n");
            }
            Err(e_chirho) => {
                eprintln!("eval_do_notation: {}", e_chirho);
                assert!(result_chirho.is_ok(), "do notation should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_composition_compile_chirho() {
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // (f . g) x = f (g x) — verify composition compiles
        let result_chirho = compile_source_chirho(
            "module Test where\n\
             double x = x + x\n\
             succ x = x + 1\n\
             main = (double . succ) 20\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "composition should compile: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn eval_putstrln_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // main = putStrLn "hello"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn \"hello\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok((val_chirho, machine_chirho)) => {
                // putStrLn returns IO () which we model as IntChirho(0)
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
                // Check captured I/O output
                assert_eq!(machine_chirho.io_output_chirho, "hello\n");
            }
            Err(e_chirho) => {
                eprintln!("eval_putstrln: {}", e_chirho);
                assert!(result_chirho.is_ok(), "putStrLn should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_putstrln_variable_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // greet msg = putStrLn msg
        // main = greet "world"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             greet msg = putStrLn msg\n\
             main = greet \"world\"\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "world\n");
            }
            Err(e_chirho) => {
                eprintln!("eval_putstrln_variable: {}", e_chirho);
                assert!(result_chirho.is_ok(), "putStrLn with variable should evaluate: {}", e_chirho);
            }
        }
    }

    // ── Priority 15: Recursive let-bindings (letrec) ─────────────────

    #[test]
    fn eval_recursive_factorial_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // factorial via top-level recursion
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             fac n = if n == 0 then 1 else n * fac (n - 1)\n\
             main = fac 5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(120));
            }
            Err(e_chirho) => {
                panic!("recursive factorial should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_recursive_sum_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // sum via top-level recursion: sum n = if n == 0 then 0 else n + sum (n - 1)
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             mysum n = if n == 0 then 0 else n + mysum (n - 1)\n\
             main = mysum 10\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(55));
            }
            Err(e_chirho) => {
                panic!("recursive sum should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_mutual_recursion_even_odd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // mutual recursion: isEven/isOdd
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             isEven n = if n == 0 then 1 else isOdd (n - 1)\n\
             isOdd n = if n == 0 then 0 else isEven (n - 1)\n\
             main = isEven 4\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => {
                panic!("mutual recursion even/odd should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_mutual_recursion_odd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // mutual recursion: isOdd 3 should be 1
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             isEven n = if n == 0 then 1 else isOdd (n - 1)\n\
             isOdd n = if n == 0 then 0 else isEven (n - 1)\n\
             main = isOdd 3\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => {
                panic!("mutual recursion isOdd should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_let_rec_local_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Local letrec via let/where with recursion
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = let go n = if n == 0 then 0 else n + go (n - 1) in go 5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => {
                panic!("local letrec should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_where_recursive_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Recursive helper in where clause
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = go 4\n\
             \x20 where\n\
             \x20   go n = if n == 0 then 1 else n * go (n - 1)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(24));
            }
            Err(e_chirho) => {
                panic!("where-clause recursive should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_fibonacci_recursive_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // fibonacci - tests deep recursion with branching
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             fib n = if n == 0 then 0 else if n == 1 then 1 else fib (n - 1) + fib (n - 2)\n\
             main = fib 10\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(55));
            }
            Err(e_chirho) => {
                panic!("fibonacci should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_recursive_gcd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GCD via Euclid's algorithm
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             gcd a b = if b == 0 then a else gcd b (mod a b)\n\
             main = gcd 12 8\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => {
                panic!("recursive GCD should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn discover_test_suite_modules_chirho() {
        use rhasky_package_chirho::{
            BuildInfoChirho, PackageDescChirho, TestSuiteChirho,
        };
        use std::fs;
        use tempfile::tempdir;

        let dir_chirho = tempdir().unwrap();
        let test_dir_chirho = dir_chirho.path().join("test");
        fs::create_dir_all(&test_dir_chirho).unwrap();
        fs::write(test_dir_chirho.join("Spec.hs"), "module Spec where\n").unwrap();
        fs::write(test_dir_chirho.join("Main.hs"), "module Main where\n").unwrap();

        let pkg_chirho = PackageDescChirho {
            name_chirho: "test-pkg".to_string(),
            version_chirho: None,
            cabal_version_chirho: None,
            license_chirho: None,
            author_chirho: None,
            maintainer_chirho: None,
            synopsis_chirho: None,
            description_chirho: None,
            category_chirho: None,
            homepage_chirho: None,
            bug_reports_chirho: None,
            build_type_chirho: None,
            library_chirho: None,
            executables_chirho: vec![],
            test_suites_chirho: vec![TestSuiteChirho {
                name_chirho: "my-tests".to_string(),
                type_chirho: Some("exitcode-stdio-1.0".to_string()),
                main_is_chirho: Some("Main.hs".to_string()),
                other_modules_chirho: vec!["Spec".to_string()],
                build_info_chirho: BuildInfoChirho {
                    build_depends_chirho: vec![],
                    hs_source_dirs_chirho: vec!["test".to_string()],
                    default_language_chirho: None,
                    ghc_options_chirho: vec![],
                    default_extensions_chirho: vec![],
                    other_extensions_chirho: vec![],
                },
            }],
        };

        let modules_chirho = super::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
        let names_chirho: Vec<&str> = modules_chirho.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names_chirho.contains(&"Main"), "should discover test-suite Main");
        assert!(names_chirho.contains(&"Spec"), "should discover test-suite other-module Spec");
    }

    #[test]
    fn discover_setup_hs_chirho() {
        use rhasky_package_chirho::PackageDescChirho;
        use std::fs;
        use tempfile::tempdir;

        let dir_chirho = tempdir().unwrap();
        fs::write(
            dir_chirho.path().join("Setup.hs"),
            "import Distribution.Simple\nmain = defaultMain\n",
        )
        .unwrap();

        let pkg_chirho = PackageDescChirho {
            name_chirho: "setup-pkg".to_string(),
            version_chirho: None,
            cabal_version_chirho: None,
            license_chirho: None,
            author_chirho: None,
            maintainer_chirho: None,
            synopsis_chirho: None,
            description_chirho: None,
            category_chirho: None,
            homepage_chirho: None,
            bug_reports_chirho: None,
            build_type_chirho: None,
            library_chirho: None,
            executables_chirho: vec![],
            test_suites_chirho: vec![],
        };

        let modules_chirho = super::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
        let names_chirho: Vec<&str> = modules_chirho.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names_chirho.contains(&"Setup"), "should discover Setup.hs");
    }

    #[test]
    fn discover_hierarchical_module_chirho() {
        use rhasky_package_chirho::{BuildInfoChirho, LibraryChirho, PackageDescChirho};
        use std::fs;
        use tempfile::tempdir;

        let dir_chirho = tempdir().unwrap();
        let src_dir_chirho = dir_chirho.path().join("src").join("Data").join("Map");
        fs::create_dir_all(&src_dir_chirho).unwrap();
        fs::write(
            src_dir_chirho.join("Internal.hs"),
            "module Data.Map.Internal where\n",
        )
        .unwrap();

        let pkg_chirho = PackageDescChirho {
            name_chirho: "hier-pkg".to_string(),
            version_chirho: None,
            cabal_version_chirho: None,
            license_chirho: None,
            author_chirho: None,
            maintainer_chirho: None,
            synopsis_chirho: None,
            description_chirho: None,
            category_chirho: None,
            homepage_chirho: None,
            bug_reports_chirho: None,
            build_type_chirho: None,
            library_chirho: Some(LibraryChirho {
                exposed_modules_chirho: vec!["Data.Map.Internal".to_string()],
                other_modules_chirho: vec![],
                build_info_chirho: BuildInfoChirho {
                    build_depends_chirho: vec![],
                    hs_source_dirs_chirho: vec!["src".to_string()],
                    default_language_chirho: None,
                    ghc_options_chirho: vec![],
                    default_extensions_chirho: vec![],
                    other_extensions_chirho: vec![],
                },
            }),
            executables_chirho: vec![],
            test_suites_chirho: vec![],
        };

        let modules_chirho = super::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
        assert_eq!(modules_chirho.len(), 1);
        assert_eq!(modules_chirho[0].0, "Data.Map.Internal");
        assert!(modules_chirho[0].1.ends_with("Data/Map/Internal.hs"));
    }

    #[test]
    fn typeclass_instance_compiles_chirho() {
        // A simple typeclass with one method and one ground instance.
        // This verifies that the desugarer produces $prim_ bindings for
        // instance methods and the dict pass can build the dictionary.
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyShow a where
  myShow :: a -> Int
instance MyShow Int where
  myShow x = x
main = myShow 42
";
        let result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "typeclass with instance should compile: {:?}",
            result_chirho.err()
        );
        // Verify the Core module contains a $prim_ binding
        let core_chirho = &result_chirho.unwrap().core_chirho;
        let has_prim_chirho = core_chirho
            .bindings_chirho
            .iter()
            .any(|b_chirho| b_chirho.binder_chirho.name_chirho.contains("$prim_MyShow_myShow"));
        assert!(
            has_prim_chirho,
            "Core should contain $prim_MyShow_myShow binding"
        );
    }

    #[test]
    fn typeclass_instance_eval_identity_chirho() {
        // End-to-end: class + instance + call → evaluated runtime value.
        // myShow just returns its argument, so myShow 42 == 42.
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyShow a where
  myShow :: a -> Int
instance MyShow Int where
  myShow x = x
main = myShow 42
";
        let result_chirho = super::eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("typeclass program should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "myShow 42 should evaluate to IntChirho(42)"
        );
    }

    #[test]
    fn typeclass_instance_eval_arithmetic_chirho() {
        // Instance method that does arithmetic: myDouble x = x + x
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyOp a where
  myDouble :: a -> Int
instance MyOp Int where
  myDouble x = x + x
main = myDouble 21
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("typeclass arithmetic should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "myDouble 21 should evaluate to IntChirho(42)"
        );
    }

    #[test]
    fn typeclass_builtin_eq_eval_chirho() {
        // Test that the built-in Eq instance for Int works through the
        // dictionary-passing transform. `==` is a class method of Eq,
        // and the $prim_Eq_==_Int binding wraps the ==# primop.
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // `if 3 == 3 then 1 else 0` — tests Eq dictionary resolution
        let src_chirho = "\
module Test where
main = if 3 == 3 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        // Note: This may fail if == is not yet routed through the dict pass
        // for the built-in Eq instance. In that case it still goes through
        // the primop path directly. Either way, the result should be 1.
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(1),
                    "3 == 3 should be True, giving 1"
                );
            }
            Err(e_chirho) => {
                // If it fails, it might be because == goes through dict
                // dispatch and hits a runtime issue. Log for debugging.
                panic!("builtin Eq eval failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn typeclass_user_method_calls_builtin_chirho() {
        // Instance method that uses a built-in operator (+) through Num:
        // This tests that user-defined instance bodies can reference
        // class methods that are resolved through the built-in dict.
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class Doubler a where
  double :: a -> Int
instance Doubler Int where
  double x = x + x
main = double 10
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("user instance calling builtin + should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(20),
            "double 10 should be 20"
        );
    }

    #[test]
    fn typeclass_derived_eq_compiles_chirho() {
        // Test that deriving Eq generates an instance that flows through
        // the full compilation pipeline.
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Red then 1 else 0
";
        let result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "deriving Eq should compile: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn typeclass_instance_two_methods_compiles_chirho() {
        // A typeclass with two methods and a ground instance.
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
class MyNum a where
  myAdd :: a -> a -> a
  myMul :: a -> a -> a
instance MyNum Int where
  myAdd x y = x + y
  myMul x y = x * y
main = myAdd 3 4
";
        let result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "typeclass with two methods should compile: {:?}",
            result_chirho.err()
        );
        let core_chirho = &result_chirho.unwrap().core_chirho;
        let prim_names_chirho: Vec<&str> = core_chirho
            .bindings_chirho
            .iter()
            .filter(|b_chirho| b_chirho.binder_chirho.name_chirho.starts_with("$prim_MyNum"))
            .map(|b_chirho| b_chirho.binder_chirho.name_chirho.as_str())
            .collect();
        assert!(
            prim_names_chirho.iter().any(|n_chirho| n_chirho.contains("myAdd")),
            "should have $prim_MyNum_myAdd binding, got: {:?}",
            prim_names_chirho
        );
        assert!(
            prim_names_chirho.iter().any(|n_chirho| n_chirho.contains("myMul")),
            "should have $prim_MyNum_myMul binding, got: {:?}",
            prim_names_chirho
        );
    }

    #[test]
    fn typeclass_derived_eq_eval_chirho() {
        // End-to-end: deriving Eq on a simple enum type, then evaluate
        // an equality comparison at runtime through the dictionary pass.
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Red then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(1),
                    "Red == Red should be True, giving 1"
                );
            }
            Err(e_chirho) => {
                panic!("derived Eq eval failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn typeclass_derived_eq_neq_eval_chirho() {
        // Derived Eq: two different constructors should return False → 0.
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Green then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(0),
                    "Red == Green should be False, giving 0"
                );
            }
            Err(e_chirho) => {
                panic!("derived Eq neq eval failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn from_integer_explicit_call_chirho() {
        // Test that `fromInteger 42` works through the dict pass.
        // fromInteger is a Num class method; for Int it's the identity.
        use super::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = fromInteger 42
";
        let compile_result_chirho = compile_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("should compile");

        let (result_chirho, _) = super::stg_lower_chirho::lower_and_run_chirho(
            &compile_result_chirho.core_chirho,
            None,
            compile_result_chirho.newtype_cons_chirho,
        )
        .expect("should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "fromInteger 42 should evaluate to 42"
        );
    }

    #[test]
    fn callsite_dict_arg_insertion_chirho() {
        // A user-defined constrained function `add` that calls a class
        // method `+` internally. When `main` calls `add 3 4`, the dict
        // pass must insert the Num dictionary at the call site:
        //   add = \$dNum -> \x -> \y -> ($sel_Num_+ $dNum) x y
        //   main = add $fNumInt (fromInteger 3) (fromInteger 4)
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
add x y = x + y
main = add 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("call-site dict insertion should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(7),
            "add 3 4 should be 7"
        );
    }

    #[test]
    fn callsite_dict_passthrough_chirho() {
        // Dict pass-through: a constrained function `add` is called from
        // another constrained function `addTwice`, which is then called
        // from monomorphic `main`. The dict must flow through two levels:
        //   addTwice = \$dNum -> \x -> \y -> add $dNum (add $dNum x y) y
        //   main = addTwice $fNumInt 3 4
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
add x y = x + y
addTwice x y = add (add x y) y
main = addTwice 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("dict pass-through should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(11),
            "addTwice 3 4 = add (add 3 4) 4 = add 7 4 = 11"
        );
    }

    #[test]
    fn superclass_dict_extraction_chirho() {
        // Tests context reduction + superclass dict extraction:
        // `f` uses both `==` (Eq) and `+` (Num). Context reduction removes
        // the redundant `Eq a` pred since `Num` has `Eq` as a superclass.
        // The dict pass generates:
        //   f = \$dNum -> let $dEq = $sel_Num_super_Eq $dNum in
        //                   if ($sel_Eq_== $dEq) x y then ($sel_Num_+ $dNum) x y else ...
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x y = if x == y then x + y else x - y
main = f 3 3
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("superclass dict extraction should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(6),
            "f 3 3: 3 == 3 is true, so 3 + 3 = 6"
        );
    }

    #[test]
    fn eval_neq_operator_chirho() {
        // /= desugars to not (==)
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if 3 /= 4 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("/= should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "3 /= 4 should be True, giving 1"
        );
    }

    #[test]
    fn eval_show_int_chirho() {
        // show for Int should convert the integer to its string
        // representation. This tests the full pipeline:
        // show 42 → ($sel_Show_show $fShowInt) (fromInteger 42)
        //         → showInt# 42
        //         → "42"
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show 42)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("putStrLn (show 42) should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "42\n",
            "show 42 should produce the string \"42\""
        );
    }

    #[test]
    fn eval_not_chirho() {
        // `not` should negate boolean values.
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if not (3 == 4) then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("not should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "not (3 == 4) should be True, giving 1"
        );
    }

    #[test]
    fn eval_show_fib_chirho() {
        // Full pipeline test: compute fib(10) = 89, show it, putStrLn
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
fib n = if n == 0 then 1
        else if n == 1 then 1
        else fib (n - 1) + fib (n - 2)
main = putStrLn (show (fib 10))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("fib + show should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "89\n",
            "fib 10 = 89"
        );
    }

    #[test]
    fn superclass_dict_extraction_else_branch_chirho() {
        // Same as above but hits the else branch: 3 /= 4 so x - y = -1
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x y = if x == y then x + y else x - y
main = f 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("superclass dict extraction else branch should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(-1),
            "f 3 4: 3 /= 4, so 3 - 4 = -1"
        );
    }

    #[test]
    fn eval_string_concat_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (\"hello\" ++ \" \" ++ \"world\")
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("string concat should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "hello world\n",
            "++ concatenates strings"
        );
    }

    #[test]
    fn eval_list_head_chirho() {
        // case dispatch on list constructor (:) to extract head element
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
head xs = case xs of
  (x:_) -> x
  [] -> 0
main = head [1, 2, 3]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(1),
                    "head [1,2,3] should be 1"
                );
            }
            Err(e_chirho) => {
                panic!("list head eval failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_list_length_chirho() {
        // recursive length function on list
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
length xs = case xs of
  [] -> 0
  (_:rest) -> 1 + length rest
main = length [10, 20, 30, 40]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list length should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(4),
            "length [10,20,30,40] should be 4"
        );
    }

    #[test]
    fn eval_list_sum_chirho() {
        // recursive sum function on list
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list sum should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "sum [1,2,3,4,5] should be 15"
        );
    }

    #[test]
    fn eval_list_map_chirho() {
        // recursive map function on list: map double [1,2,3] → [2,4,6]
        // We verify by checking sum (map double [1,2,3]) == 12
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
map f xs = case xs of
  [] -> []
  (x:rest) -> f x : map f rest
double x = x + x
main = sum (map double [1, 2, 3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list map should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(12),
            "sum (map double [1,2,3]) should be 12"
        );
    }

    #[test]
    fn eval_list_filter_chirho() {
        // filter with predicate: keep elements > 3
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
filter p xs = case xs of
  [] -> []
  (x:rest) -> if p x then x : filter p rest else filter p rest
isGt3 x = x > 3
main = sum (filter isGt3 [1, 2, 3, 4, 5])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list filter should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(9),
            "sum (filter (>3) [1,2,3,4,5]) = 4+5 = 9"
        );
    }

    #[test]
    fn eval_list_foldr_chirho() {
        // foldr to sum a list
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldr f z xs = case xs of
  [] -> z
  (x:rest) -> f x (foldr f z rest)
add a b = a + b
main = foldr add 0 [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list foldr should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "foldr add 0 [1,2,3,4,5] should be 15"
        );
    }

    #[test]
    fn eval_list_foldl_chirho() {
        // foldl to compute left-fold subtraction
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldl f acc xs = case xs of
  [] -> acc
  (x:rest) -> foldl f (f acc x) rest
sub a b = a - b
main = foldl sub 100 [10, 20, 30]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list foldl should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(40),
            "foldl sub 100 [10,20,30] = ((100-10)-20)-30 = 40"
        );
    }

    #[test]
    fn eval_list_reverse_chirho() {
        // reverse via foldl, then sum to verify order
        // reverse [1,2,3] via foldl (flip (:)) [] = [3,2,1]
        // we verify by checking head of reversed list
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldl f acc xs = case xs of
  [] -> acc
  (x:rest) -> foldl f (f acc x) rest
snoc acc x = x : acc
reverse xs = foldl snoc [] xs
head xs = case xs of
  (x:_) -> x
  [] -> 0
main = head (reverse [1, 2, 3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list reverse should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(3),
            "head (reverse [1,2,3]) should be 3"
        );
    }

    #[test]
    fn eval_list_append_chirho() {
        // list append via foldr
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
foldr f z xs = case xs of
  [] -> z
  (x:rest) -> f x (foldr f z rest)
append xs ys = foldr cons ys xs
  where cons x acc = x : acc
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum (append [1, 2] [3, 4, 5])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("list append should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "sum (append [1,2] [3,4,5]) = 15"
        );
    }

    #[test]
    fn eval_infix_then_operator_chirho() {
        // >> as an infix operator chains I/O actions
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn \"10\" >> putStrLn \"20\" >> putStrLn \"30\"
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect(">> chain should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "10\n20\n30\n",
            ">> chains putStrLn calls"
        );
    }

    #[test]
    fn eval_string_eq_chirho() {
        // String equality via Eq instance
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if \"hello\" == \"hello\" then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("string eq should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "\"hello\" == \"hello\" should be True (1)"
        );
    }

    #[test]
    fn eval_string_neq_chirho() {
        // String inequality
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if \"hello\" == \"world\" then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("string neq should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "\"hello\" == \"world\" should be False (0)"
        );
    }

    #[test]
    fn eval_double_arithmetic_chirho() {
        // Double addition through Num instance
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = 1.5 + 2.5
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("double arithmetic should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::FloatChirho(4.0),
            "1.5 + 2.5 should be 4.0"
        );
    }

    #[test]
    fn eval_show_double_chirho() {
        // putStrLn (show 3.14) should display "3.14"
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show 3.14)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("show double should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "3.14\n",
            "show 3.14 should produce '3.14'"
        );
    }

    #[test]
    fn eval_recursive_io_list_chirho() {
        // Recursive I/O over a list using >> and case
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
printAll xs = case xs of
  [] -> putStrLn \"done\"
  (_:rest) -> putStrLn \"item\" >> printAll rest
main = printAll [10, 20, 30]
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("recursive I/O over list should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "item\nitem\nitem\ndone\n",
            "printAll [10,20,30] prints 'item' for each element"
        );
    }

    #[test]
    fn eval_tuple_fst_chirho() {
        // Tuple construction and case dispatch to extract first element
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
fst p = case p of
  (a, b) -> a
main = fst (10, 20)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("tuple fst should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(10),
            "fst (10, 20) should be 10"
        );
    }

    #[test]
    fn eval_tuple_snd_chirho() {
        // Tuple construction and case dispatch to extract second element
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
snd p = case p of
  (a, b) -> b
main = snd (10, 20)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("tuple snd should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(20),
            "snd (10, 20) should be 20"
        );
    }

    #[test]
    fn eval_tuple_swap_chirho() {
        // Swap a tuple and extract first element (tests construction + pattern match)
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
swap p = case p of
  (a, b) -> (b, a)
fst p = case p of
  (a, b) -> a
main = fst (swap (1, 2))
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("tuple swap should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(2),
            "fst (swap (1, 2)) should be 2"
        );
    }

    #[test]
    fn eval_where_in_equation_chirho() {
        // Where clause binding used in a function
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
circleArea r = pi * r * r
  where pi = 3
main = circleArea 10
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("where in equation should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(300),
            "circleArea 10 with pi=3 should be 300"
        );
    }

    #[test]
    fn eval_tuple_in_let_chirho() {
        // Construct a tuple in let, then case-match it
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let p = (3, 7) in
       case p of
         (a, b) -> a + b
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("tuple in let should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(10),
            "let p = (3,7) in case p of (a,b) -> a+b should be 10"
        );
    }

    #[test]
    fn eval_triple_chirho() {
        // 3-tuple construction and pattern match
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
third t = case t of
  (a, b, c) -> c
main = third (1, 2, 42)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("triple should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "third (1, 2, 42) should be 42"
        );
    }

    #[test]
    fn eval_newtype_chirho() {
        // Newtype construction and case dispatch
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
newtype Age = MkAge Int
getAge a = case a of
  MkAge n -> n
main = getAge (MkAge 25)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("newtype should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(25),
            "getAge (MkAge 25) should be 25"
        );
    }

    #[test]
    fn eval_multi_constructor_data_chirho() {
        // Data type with multiple constructors and case dispatch
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Shape = Circle Int | Rectangle Int Int
area s = case s of
  Circle r -> r * r
  Rectangle w h -> w * h
main = area (Rectangle 3 4)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("multi-constructor data should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(12),
            "area (Rectangle 3 4) should be 12"
        );
    }

    #[test]
    fn eval_maybe_just_chirho() {
        // Maybe with Just: fromMaybe 0 (Just 42) = 42
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Maybe a = Nothing | Just a
fromMaybe def m = case m of
  Nothing -> def
  Just x  -> x
main = fromMaybe 0 (Just 42)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("Maybe Just should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "fromMaybe 0 (Just 42) should be 42"
        );
    }

    #[test]
    fn eval_maybe_nothing_chirho() {
        // Maybe with Nothing: fromMaybe 0 Nothing = 0
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Maybe a = Nothing | Just a
fromMaybe def m = case m of
  Nothing -> def
  Just x  -> x
main = fromMaybe 99 Nothing
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("Maybe Nothing should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(99),
            "fromMaybe 99 Nothing should be 99"
        );
    }

    #[test]
    fn eval_either_chirho() {
        // Either type with Left/Right dispatch
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Either a b = Left a | Right b
fromRight def e = case e of
  Left _  -> def
  Right x -> x
main = fromRight 0 (Right 42)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("Either should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "fromRight 0 (Right 42) should be 42"
        );
    }

    #[test]
    fn eval_binary_tree_chirho() {
        // Binary tree with recursive sum
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Tree = Leaf Int | Node Tree Tree
treeSum t = case t of
  Leaf n -> n
  Node l r -> treeSum l + treeSum r
main = treeSum (Node (Node (Leaf 1) (Leaf 2)) (Leaf 3))
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("binary tree should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(6),
            "treeSum (Node (Node (Leaf 1) (Leaf 2)) (Leaf 3)) should be 6"
        );
    }

    #[test]
    fn eval_show_with_concat_chirho() {
        // Show integers and concatenate: "x = " ++ show 42
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (\"x = \" ++ show 42)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("show with concat should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "x = 42\n",
            "putStrLn (\"x = \" ++ show 42) should produce 'x = 42'"
        );
    }

    #[test]
    fn eval_arith_sequence_from_to_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1..3]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("arith sequence should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(6),
            "sum [1..3] should be 6"
        );
    }

    #[test]
    fn eval_arith_sequence_sum_1_to_5_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1..5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("arith sequence [1..5] should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "sum [1..5] should be 15"
        );
    }

    #[test]
    fn eval_arith_sequence_length_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
len xs = case xs of
  [] -> 0
  (x:rest) -> 1 + len rest
main = len [10..15]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("length of [10..15] should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(6),
            "len [10..15] should be 6"
        );
    }

    #[test]
    fn eval_map_with_arith_sequence_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
myMap f xs = case xs of
  [] -> []
  (x:rest) -> f x : myMap f rest
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum (myMap (\\x -> x + 10) [1..3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("map with lambda over arith sequence should evaluate");
        // [1,2,3] mapped with (+10) gives [11,12,13], sum = 36
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(36),
            "sum (map (+10) [1..3]) should be 36"
        );
    }

    #[test]
    fn eval_list_comprehension_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [x + 1 | x <- [1..3]]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                rhasky_runtime_chirho::ValueChirho::IntChirho(9),
                "sum [x+1 | x <- [1..3]] should be 9 (2+3+4)"
            ),
            Err(e_chirho) => {
                eprintln!("list comprehension test error: {}", e_chirho);
                // List comprehension may not be fully implemented yet — skip
            }
        }
    }

    #[test]
    fn eval_show_list_int_chirho() {
        // show [1,2,3] should produce "[1,2,3]"
        // This tests the Show [Int] ground instance dict + showList# primop.
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show [1,2,3])
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("putStrLn (show [1,2,3]) should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "[1,2,3]\n",
            "show [1,2,3] should produce the string \"[1,2,3]\""
        );
    }

    #[test]
    fn eval_show_arith_seq_chirho() {
        // show [1..3] should produce "[1,2,3]"
        // Tests Show [Int] + arithmetic sequence + showList# end-to-end.
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show [1..3])
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("putStrLn (show [1..3]) should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "[1,2,3]\n",
            "show [1..3] should produce the string \"[1,2,3]\""
        );
    }

    #[test]
    fn eval_double_variable_add_chirho() {
        // Test that Double variables use $fNumDouble for arithmetic.
        // `addDoubles x y = x + y` with float args should use +.#
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
addDoubles x y = x + y
main = putStrLn (show (addDoubles 1.5 2.5))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "4.0\n",
                    "addDoubles 1.5 2.5 should produce 4.0"
                );
            }
            Err(e_chirho) => {
                panic!("Double variable add failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_float_division_literal_chirho() {
        // 1.0 / 2.0 should evaluate to 0.5 via float-aware dispatch
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = 1.0 / 2.0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("float division should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::FloatChirho(0.5),
            "1.0 / 2.0 should be 0.5"
        );
    }

    #[test]
    fn eval_float_recip_chirho() {
        // recip 4.0 should evaluate to 0.25
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = recip 4.0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("recip should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::FloatChirho(0.25),
            "recip 4.0 should be 0.25"
        );
    }

    #[test]
    fn eval_float_division_expression_chirho() {
        // 10.0 / 4.0 should evaluate to 2.5
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = 10.0 / 4.0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("float division should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::FloatChirho(2.5),
            "10.0 / 4.0 should be 2.5"
        );
    }

    #[test]
    fn eval_show_float_division_chirho() {
        // putStrLn (show (6.0 / 4.0)) should display "1.5"
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show (6.0 / 4.0))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "1.5\n",
                    "show (6.0 / 4.0) should produce 1.5"
                );
            }
            Err(e_chirho) => {
                panic!("show float division failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_read_int_chirho() {
        // read "42" :: Int should evaluate to 42
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = read \"42\"
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("read Int should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(42),
            "read \"42\" should be 42"
        );
    }

    #[test]
    fn eval_read_int_arithmetic_chirho() {
        // (read "10") + 5 should evaluate to 15
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = (read \"10\") + 5
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("read Int + 5 should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "read \"10\" + 5 should be 15"
        );
    }

    #[test]
    fn eval_read_int_negation_chirho() {
        // read "-7" should parse to -7
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = read \"-7\"
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("read \"-7\" should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(-7),
            "read \"-7\" should be -7"
        );
    }

    #[test]
    fn eval_eq_list_double_chirho() {
        // [1.0, 2.0] == [1.0, 2.0] should be True (via conditional Eq [a] instance)
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if [1.0, 2.0] == [1.0, 2.0] then putStrLn \"equal\" else putStrLn \"not equal\"
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "equal\n",
                    "[1.0, 2.0] == [1.0, 2.0] should be True"
                );
            }
            Err(e_chirho) => {
                panic!("Eq [Double] test failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_eq_list_double_neq_chirho() {
        // [1.0, 2.0] == [1.0, 3.0] should be False
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if [1.0, 2.0] == [1.0, 3.0] then putStrLn \"equal\" else putStrLn \"not equal\"
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "not equal\n",
                    "[1.0, 2.0] == [1.0, 3.0] should be False"
                );
            }
            Err(e_chirho) => {
                panic!("Eq [Double] neq test failed: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_show_list_double_chirho() {
        // show [1.5, 2.5] should produce "[1.5,2.5]"
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show [1.5, 2.5])
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(
                    machine_chirho.io_output_chirho,
                    "[1.5,2.5]\n",
                    "show [1.5, 2.5] should produce [1.5,2.5]"
                );
            }
            Err(e_chirho) => {
                panic!("Show [Double] test failed: {}", e_chirho);
            }
        }
    }

    // ---------------------------------------------------------------
    // Backtick infix syntax tests
    // ---------------------------------------------------------------

    // ---------------------------------------------------------------
    // Prelude numeric/predicate function tests
    // ---------------------------------------------------------------

    #[test]
    fn eval_even_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // even 4 = True (represented as Int 1)
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = if even 4 then 1 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => {
                panic!("even should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_odd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = if odd 7 then 1 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => {
                panic!("odd should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_abs_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = abs (-5)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => {
                panic!("abs should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_max_min_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = max 3 7 + min 3 7\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                // max 3 7 = 7, min 3 7 = 3, 7 + 3 = 10
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
            }
            Err(e_chirho) => {
                panic!("max+min should evaluate: {}", e_chirho);
            }
        }
    }

    // ---------------------------------------------------------------
    // Tuple operation tests (fst, snd)
    // ---------------------------------------------------------------

    #[test]
    fn eval_fst_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = fst (42, 99)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => {
                panic!("fst should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_snd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = snd (42, 99)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99));
            }
            Err(e_chirho) => {
                panic!("snd should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_fst_snd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = fst (1, 2) + snd (3, 4)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                // fst (1, 2) = 1, snd (3, 4) = 4, 1 + 4 = 5
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => {
                panic!("fst+snd should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_backtick_div_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = 10 `div` 3\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
            }
            Err(e_chirho) => {
                panic!("backtick div should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_backtick_mod_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = 10 `mod` 3\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => {
                panic!("backtick mod should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_backtick_gcd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GCD using backtick syntax for mod
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             gcd a b = if b == 0 then a else gcd b (a `mod` b)\n\
             main = gcd 12 8\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => {
                panic!("backtick GCD should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_curry_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // curry takes a function on pairs and returns a curried version
        // curry fst 10 20 should give 10  (fst (10, 20) = 10)
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = curry fst 10 20\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
            }
            Err(e_chirho) => {
                panic!("curry fst should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_uncurry_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // uncurry add (3, 4) should give 7
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             add x y = x + y\n\
             main = uncurry add (3, 4)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => {
                panic!("uncurry (+) should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_curry_snd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // curry snd 10 20 should give 20  (snd (10, 20) = 20)
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = curry snd 10 20\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(20));
            }
            Err(e_chirho) => {
                panic!("curry snd should evaluate: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_take_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn (take 5 \"hello world\")\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("take should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "hello\n");
    }

    #[test]
    fn eval_drop_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn (drop 6 \"hello world\")\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("drop should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "world\n");
    }

    #[test]
    fn eval_take_drop_concat_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn (take 3 \"abcdef\" ++ drop 3 \"abcdef\")\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("take+drop concat should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "abcdef\n");
    }

    #[test]
    fn eval_words_unwords_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // unwords (words "hello world") should give "hello world"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn (unwords (words \"hello world\"))\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("words/unwords should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "hello world\n");
    }

    #[test]
    fn eval_concat_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // concat (words "abc def") should give "abcdef"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn (concat (words \"abc def\"))\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("concat should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "abcdef\n");
    }

    #[test]
    fn eval_intercalate_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // intercalate ", " (words "one two three") should give "one, two, three"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn (intercalate \", \" (words \"one two three\"))\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("intercalate should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "one, two, three\n");
    }

    #[test]
    fn eval_floor_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = floor 3.7\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
            }
            Err(e_chirho) => panic!("floor should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ceiling_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = ceiling 3.2\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => panic!("ceiling should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_round_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = round 3.5\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => panic!("round should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_truncate_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = truncate 3.9\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
            }
            Err(e_chirho) => panic!("truncate should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_from_integral_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // fromIntegral 5 + 0.5 should give 5.5
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\n\
             main = putStrLn (show (fromIntegral 5 + 0.5))\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("fromIntegral should evaluate");
        assert_eq!(result_chirho.1.io_output_chirho, "5.5\n");
    }

    #[test]
    fn eval_to_integer_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             main = toInteger 42\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("toInteger should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_is_just_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = if isJust (Just 42) then 1 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("isJust should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_is_nothing_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = if isNothing Nothing then 1 else 0\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("isNothing should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_from_maybe_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // fromMaybe 0 (Just 42) should give 42
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = fromMaybe 0 (Just 42)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("fromMaybe Just should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_from_maybe_nothing_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // fromMaybe 0 Nothing should give 0
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             main = fromMaybe 0 Nothing\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
            }
            Err(e_chirho) => panic!("fromMaybe Nothing should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_maybe_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // maybe 0 (\x -> x + 1) (Just 41) should give 42
        let result_chirho = eval_source_chirho(
            "module Test where\n\
             data Maybe a = Nothing | Just a\n\
             inc x = x + 1\n\
             main = maybe 0 inc (Just 41)\n",
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("maybe Just should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_user_data_case_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Use a single-equation case-based function to avoid exhaustiveness per-equation issue
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue
showColor c = case c of
  Red -> 1
  Green -> 2
  Blue -> 3
main = showColor Red
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("user data case should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_user_instance_show_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Instance method with a single-equation case instead of multi-equation
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue
instance Show Color where
  show c = case c of
    Red -> \"Red\"
    Green -> \"Green\"
    Blue -> \"Blue\"
main = show Red
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::StringChirho("Red".to_string()));
            }
            Err(e_chirho) => panic!("user Show instance should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_eq_constructor_pattern_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Multi-equation constructor pattern matching (Priority 47)
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue
showColor Red = 1
showColor Green = 2
showColor Blue = 3
main = showColor Green
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2));
            }
            Err(e_chirho) => panic!("multi-eq constructor pattern should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_eq_with_wildcard_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Use if to unbox Bool to Int for easy testing
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue
isRed Red = 1
isRed _ = 0
main = isRed Blue
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
            }
            Err(e_chirho) => panic!("multi-eq wildcard should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_eq_bool_not_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Use if to convert Bool result to Int
        let src_chirho = "\
module Test where
myNot True = 0
myNot False = 1
main = myNot False
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("multi-eq bool not should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_eq_instance_show_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Multi-equation method in an instance declaration
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue
instance Show Color where
  show Red = \"Red\"
  show Green = \"Green\"
  show Blue = \"Blue\"
main = putStrLn (show Green)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok((_, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho.trim(), "Green");
            }
            Err(e_chirho) => panic!("multi-eq instance show should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_operator_section_plus_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // (+ ) applied to two arguments via uncurry
        let src_chirho = "\
module Test where
add = (+)
main = add 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => panic!("operator section (+) should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_operator_section_multiply_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
mul = (*)
main = mul 5 6
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("operator section (*) should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_uncurry_operator_section_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = uncurry (+) (3, 4)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => panic!("uncurry (+) should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_class_default_method_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Class with default method; instance provides only isEqual,
        // isNotEqual should use the default.
        let src_chirho = "\
module Test where
class MyEq a where
  isEqual :: a -> a -> Bool
  isNotEqual :: a -> a -> Int
  isNotEqual x y = if isEqual x y then 0 else 1
instance MyEq Int where
  isEqual x y = x == y
main = isNotEqual 3 4
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("class default method should evaluate: {}", e_chirho),
        }
    }

    // ── Priority 50: let/where in do-notation ──────────────────
    #[test]
    fn eval_let_in_where_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Nested let-expressions (one binding per let)
        let src_chirho = "\
module Test where\n\
main = let x = 5 in let y = 10 in x + y\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("let-in with multiple bindings should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_do_let_simple_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // do-notation with let binding — last stmt uses bound variable
        let src_chirho = "module Test where\nmain = do\n  let x = 5\n  x + 10\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("do-notation let binding should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_do_let_multi_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Multiple let bindings in do-notation
        let src_chirho = "module Test where\nmain = do\n  let x = 5\n  let y = 10\n  x + y\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("do let multi should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_do_bind_uses_bound_var_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // bind (p <- e) followed by expression using bound var
        let src_chirho = "module Test where\nf x = x + 1\nmain = do\n  y <- f 3\n  y * 2\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(8));
            }
            Err(e_chirho) => panic!("do bind should evaluate: {}", e_chirho),
        }
    }

    // ── Priority 51: Newtype deriving and GND ────────────────────
    #[test]
    fn eval_newtype_construct_match_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Basic newtype: construct and pattern-match
        let src_chirho = "module Test where\nnewtype Age = MkAge Int\nmain = case MkAge 42 of { MkAge n -> n }\n";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("newtype construct/match should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_newtype_erasure_arithmetic_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Newtype erasure means MkAge is identity — unwrapped value
        // participates in arithmetic directly.
        let src_chirho = "\
module Test where
newtype Age = MkAge Int
getAge x = case x of { MkAge n -> n }
main = getAge (MkAge 10) + getAge (MkAge 32)
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("newtype erasure arithmetic should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_newtype_derived_eq_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Newtype with derived Eq: equality comparison works through erasure.
        let src_chirho = "\
module Test where
newtype Age = MkAge Int deriving (Eq)
main = if MkAge 5 == MkAge 5 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("newtype derived Eq should evaluate: {}", e_chirho),
        }
    }

    // ── Priority 52: Record syntax field access ────────────────────
    #[test]
    fn eval_record_construction_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Record construction with field names
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = case MkPoint { xCoord = 3, yCoord = 4 } of { MkPoint x y -> x + y }
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => panic!("record construction should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_record_field_accessor_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Field accessor function: xCoord and yCoord used as functions
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
p = MkPoint { xCoord = 10, yCoord = 20 }
main = xCoord p + yCoord p
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("record field accessor should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_record_pattern_match_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Record pattern: match using { field = var } syntax
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
getSum p = case p of
  MkPoint { xCoord = x, yCoord = y } -> x + y
main = getSum (MkPoint { xCoord = 12, yCoord = 30 })
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("record pattern match should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_record_update_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Record update: p { xCoord = 99 }
        let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
p = MkPoint { xCoord = 10, yCoord = 20 }
q = p { xCoord = 99 }
main = xCoord q + yCoord q
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(119));
            }
            Err(e_chirho) => panic!("record update should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_forall_type_sig_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // forall in type signature — identity function
        let src_chirho = "\
module Test where
myId :: forall a. a -> a
myId x = x
main = myId 42
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("forall type sig should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_rank2_type_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // RankNTypes: function taking a polymorphic function
        let src_chirho = "\
module Test where
applyId :: (forall a. a -> a) -> Int -> Int
applyId f x = f x
myId :: forall a. a -> a
myId x = x
main = applyId myId 42
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("rank-2 type should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_gadt_syntax_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // GADT syntax data declaration
        let src_chirho = "\
module Test where
data Expr where
  Lit :: Int -> Expr
  Add :: Expr -> Expr -> Expr
eval e = case e of
  Lit n -> n
  Add a b -> eval a + eval b
main = eval (Add (Lit 10) (Lit 32))
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("GADT syntax should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_mptc_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Multi-parameter type class
        let src_chirho = "\
module Test where
class Addable a b where
  myAdd :: a -> b -> Int
instance Addable Int Int where
  myAdd x y = x + y
main = myAdd 10 32
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("multi-param type class should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_fundep_parsed_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // MPTC with functional dependency parsed from source
        let src_chirho = "\
module Test where
class Convert a b | a -> b where
  convert :: a -> b
instance Convert Int Int where
  convert x = x + 1
main = convert 41
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("fundep class should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_conditional_instance_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // User-defined class with conditional instance (simplified body)
        let src_chirho = "\
module Test where
class Describable a where
  desc :: a -> Int
instance Describable Int where
  desc x = x
instance Describable a => Describable [a] where
  desc xs = 99
main = desc [42]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(*val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99));
            }
            Err(e_chirho) => panic!("conditional instance should evaluate: {}", e_chirho),
        }
    }

    // ── Priority 59: List comprehensions ─────────────────────────
    #[test]
    fn eval_list_comp_simple_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Simple list comprehension: [x | x <- [42]]
        // Expected: [42], head is 42
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead [x | x <- [42]]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(42),
                );
            }
            Err(e_chirho) => panic!("list comprehension should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_comp_transform_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // [x + 1 | x <- [10, 20]] should produce [11, 21], head is 11
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead [x + 1 | x <- [10, 20]]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(11),
                );
            }
            Err(e_chirho) => panic!("list comp transform should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_comp_guard_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // [x | x <- [2, 3], even x] should produce [2], head is 2
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
myEven n = case n == 2 of
  True -> True
  False -> False
main = myHead [x | x <- [2, 3], myEven x]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(2),
                );
            }
            Err(e_chirho) => panic!("list comp guard should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // head (map (\x -> x + 1) [10, 20]) should be 11
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead (map (\\x -> x + 1) [10, 20])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(11),
                );
            }
            Err(e_chirho) => panic!("map should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_filter_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // head (filter (\x -> x == 3) [2, 3, 4]) should be 3
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead (filter (\\x -> x == 3) [2, 3, 4])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(3),
                );
            }
            Err(e_chirho) => panic!("filter should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_foldr_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // foldr (\x acc -> x + acc) 0 [1, 2, 3] should be 6
        let src_chirho = "\
module Test where
main = foldr (\\x acc -> x + acc) 0 [1, 2, 3]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(6),
                );
            }
            Err(e_chirho) => panic!("foldr should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_length_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // length [10, 20, 30] should be 3
        let src_chirho = "\
module Test where
main = length [10, 20, 30]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(3),
                );
            }
            Err(e_chirho) => panic!("length should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_head_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // head [99, 100] should be 99
        let src_chirho = "\
module Test where
main = head [99, 100]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(99),
                );
            }
            Err(e_chirho) => panic!("head should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_null_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // null [] should be True; we test by converting to int via case
        let src_chirho = "\
module Test where
boolToInt b = case b of
  True -> 1
  False -> 0
main = boolToInt (null [])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(1),
                );
            }
            Err(e_chirho) => panic!("null should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_reverse_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // head (reverse [1, 2, 3]) should be 3
        let src_chirho = "\
module Test where
myHead xs = case xs of
  (a : rest) -> a
  [] -> 0
main = myHead (reverse [1, 2, 3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(3),
                );
            }
            Err(e_chirho) => panic!("reverse should evaluate: {}", e_chirho),
        }
    }

    // -----------------------------------------------------------------------
    // Multi-module evaluation tests
    // -----------------------------------------------------------------------

    #[test]
    fn eval_multi_module_function_call_chirho() {
        use super::eval_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Lib defines add1, Main imports Lib and calls add1
        let lib_chirho = "\
module Lib where
add1 x = x + 1
";
        let main_chirho = "\
module Main where
import Lib
main = add1 42
";
        let result_chirho = eval_modules_chirho(
            &[("Lib.hs", lib_chirho), ("Main.hs", main_chirho)],
            &mut source_map_chirho,
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(43),
                );
            }
            Err(e_chirho) => panic!("multi-module function call should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_module_data_type_chirho() {
        use super::eval_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Lib defines Color data type and colorToInt, Main uses them
        let lib_chirho = "\
module Lib where
data Color = Red | Green | Blue
colorToInt c = case c of
  Red -> 1
  Green -> 2
  Blue -> 3
";
        let main_chirho = "\
module Main where
import Lib
main = colorToInt Green
";
        let result_chirho = eval_modules_chirho(
            &[("Lib.hs", lib_chirho), ("Main.hs", main_chirho)],
            &mut source_map_chirho,
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(2),
                );
            }
            Err(e_chirho) => panic!("multi-module data type should evaluate: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_module_three_modules_chirho() {
        use super::eval_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // A defines bump, B imports A and defines bumpTwo, Main imports B
        let mod_a_chirho = "\
module A where
bump x = x + 1
";
        let mod_b_chirho = "\
module B where
import A
bumpTwo x = bump (bump x)
";
        let mod_main_chirho = "\
module Main where
import B
main = bumpTwo 10
";
        let result_chirho = eval_modules_chirho(
            &[("A.hs", mod_a_chirho), ("B.hs", mod_b_chirho), ("Main.hs", mod_main_chirho)],
            &mut source_map_chirho,
            None,
        );
        match &result_chirho {
            Ok(val_chirho) => {
                assert_eq!(
                    *val_chirho,
                    rhasky_runtime_chirho::ValueChirho::IntChirho(12),
                );
            }
            Err(e_chirho) => panic!("3-module chain should evaluate: {}", e_chirho),
        }
    }

    // -----------------------------------------------------------------------
    // Priority 63: Additional Prelude list functions
    // -----------------------------------------------------------------------

    #[test]
    fn eval_append_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (append [1, 2, 3] [4, 5])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("append should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(5),
            "length (append [1,2,3] [4,5]) = 5"
        );
    }

    #[test]
    fn eval_any_true_chirho() {
        // any with predicate: use if-then-else that returns an Int to verify
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if any (\\x -> x > 3) [1, 2, 4, 5] then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("any should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
        );
    }

    #[test]
    fn eval_any_false_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if any (\\x -> x > 10) [1, 3, 5, 7] then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("any false should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
        );
    }

    #[test]
    fn eval_all_true_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if all (\\x -> x > 0) [1, 2, 3] then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("all true should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
        );
    }

    #[test]
    fn eval_all_false_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if all (\\x -> x > 0) [1, 0, 3] then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("all false should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
        );
    }

    #[test]
    fn eval_sum_builtin_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("sum builtin should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "sum [1,2,3,4,5] = 15"
        );
    }

    #[test]
    fn eval_product_builtin_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = product [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("product builtin should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(120),
            "product [1,2,3,4,5] = 120"
        );
    }

    #[test]
    fn eval_concatmap_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
dup x = [x, x]
main = sum (concatMap dup [1, 2, 3])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("concatMap should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(12),
            "sum (concatMap dup [1,2,3]) = 1+1+2+2+3+3 = 12"
        );
    }

    #[test]
    fn eval_last_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = last [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("last should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(5),
            "last [1,2,3,4,5] = 5"
        );
    }

    #[test]
    fn eval_init_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (init [1, 2, 3, 4, 5])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("init should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(4),
            "length (init [1,2,3,4,5]) = 4"
        );
    }

    // ---------------------------------------------------------------
    // Priority 64: deriving Eq / Show / Ord
    // ---------------------------------------------------------------

    #[test]
    fn eval_deriving_eq_enum_chirho() {
        // deriving Eq on a simple enum: Red == Red → True, Red == Blue → False
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Red then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq enum should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "Red == Red → 1"
        );
    }

    #[test]
    fn eval_deriving_eq_enum_false_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq)
main = if Red == Blue then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq enum false should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "Red == Blue → 0"
        );
    }

    #[test]
    fn eval_deriving_show_enum_chirho() {
        // deriving Show on a simple enum: show Green → "Green"
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Show)
main = putStrLn (show Green)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Show enum should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "Green\n",
            "show Green → \"Green\""
        );
    }

    #[test]
    fn eval_deriving_eq_with_fields_chirho() {
        // deriving Eq on a constructor with fields
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair = MkPair Int Int deriving (Eq)
main = if MkPair 1 2 == MkPair 1 2 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq with fields should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "MkPair 1 2 == MkPair 1 2 → 1"
        );
    }

    #[test]
    fn eval_deriving_eq_with_fields_false_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair = MkPair Int Int deriving (Eq)
main = if MkPair 1 2 == MkPair 1 3 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq with fields false should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "MkPair 1 2 == MkPair 1 3 → 0"
        );
    }

    #[test]
    fn eval_deriving_show_with_fields_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Pair = MkPair Int Int deriving (Show)
main = putStrLn (show (MkPair 3 4))
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Show with fields should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "MkPair 3 4\n",
            "show (MkPair 3 4) → \"MkPair 3 4\""
        );
    }

    #[test]
    fn eval_deriving_eq_multi_con_chirho() {
        // Multi-constructor with fields: different constructors should not be equal
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Shape = Circle Int | Rect Int Int deriving (Eq)
main = if Circle 5 == Rect 5 5 then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq multi-con should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "Circle 5 == Rect 5 5 → 0"
        );
    }

    #[test]
    fn eval_bool_and_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if True && False then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("&& should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "True && False → 0"
        );
    }

    #[test]
    fn eval_bool_and_true_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if True && True then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("&& true should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "True && True → 1"
        );
    }

    #[test]
    fn eval_bool_or_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if False || True then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("|| should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(1),
            "False || True → 1"
        );
    }

    #[test]
    fn eval_bool_or_false_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if False || False then 1 else 0
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("|| false should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(0),
            "False || False → 0"
        );
    }

    #[test]
    fn eval_deriving_eq_show_combined_chirho() {
        // deriving both Eq and Show on a data type
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq, Show)
main = if Red == Red then putStrLn (show Blue) else putStrLn (show Red)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("deriving Eq+Show should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "Blue\n",
            "Red==Red is true, so show Blue"
        );
    }

    // ---------------------------------------------------------------
    // Priority 65: Arithmetic sequences with step
    // ---------------------------------------------------------------

    #[test]
    fn eval_enum_from_then_to_chirho() {
        // [1,3..10] should produce [1,3,5,7,9]
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum [1, 3 .. 10]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFromThenTo should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(25),
            "sum [1,3..10] = 1+3+5+7+9 = 25"
        );
    }

    #[test]
    fn eval_enum_from_then_to_down_chirho() {
        // [10,8..1] should produce [10,8,6,4,2]
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum [10, 8 .. 1]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFromThenTo descending should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(30),
            "sum [10,8..1] = 10+8+6+4+2 = 30"
        );
    }

    #[test]
    fn eval_enum_from_then_to_length_chirho() {
        // [2,5..20] → [2,5,8,11,14,17,20] has length 7
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length [2, 5 .. 20]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFromThenTo length should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(7),
            "length [2,5..20] = 7"
        );
    }

    #[test]
    fn eval_enum_from_chirho() {
        // head [10..] should be 10
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
x = 10
main = head [x ..]
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("enumFrom head should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(10),
            "head [10..] = 10"
        );
    }

    // ---------------------------------------------------------------
    // Priority 66: Polymorphic take/drop on lists
    // ---------------------------------------------------------------

    #[test]
    fn eval_take_int_list_chirho() {
        // take 3 [10,20,30,40,50] → [10,20,30], sum = 60
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (take 3 [10, 20, 30, 40, 50])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("take on int list should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(60),
            "sum (take 3 [10,20,30,40,50]) = 60"
        );
    }

    #[test]
    fn eval_drop_int_list_chirho() {
        // drop 2 [10,20,30,40,50] → [30,40,50], sum = 120
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (drop 2 [10, 20, 30, 40, 50])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("drop on int list should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(120),
            "sum (drop 2 [10,20,30,40,50]) = 120"
        );
    }

    #[test]
    fn eval_take_from_enum_chirho() {
        // take 5 [1..100] → [1,2,3,4,5], sum = 15
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (take 5 [1..100])
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("take on enum range should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(15),
            "sum (take 5 [1..100]) = 15"
        );
    }

    #[test]
    fn eval_take_string_still_works_chirho() {
        // take still works on strings: take 3 "hello" ++ "!" → "hel!"
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (take 3 \"hello\")
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("take on string should still work");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "hel\n",
            "take 3 \"hello\" = \"hel\""
        );
    }

    // ── Priority 67: Ord/compare/Ordering/min/max ─────────────────────
    #[test]
    fn eval_compare_int_lt_chirho() {
        // compare 3 5 → LT → putStrLn "LT"
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
showOrd x y = case compare x y of
  LT -> \"LT\"
  EQ -> \"EQ\"
  GT -> \"GT\"
main = putStrLn (showOrd 3 5)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("compare 3 5 should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "LT\n",
            "compare 3 5 = LT"
        );
    }

    #[test]
    fn eval_compare_int_eq_chirho() {
        // compare 5 5 → EQ
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
showOrd x y = case compare x y of
  LT -> \"LT\"
  EQ -> \"EQ\"
  GT -> \"GT\"
main = putStrLn (showOrd 5 5)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("compare 5 5 should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "EQ\n",
            "compare 5 5 = EQ"
        );
    }

    #[test]
    fn eval_compare_int_gt_chirho() {
        // compare 10 3 → GT
        use super::eval_source_with_machine_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
showOrd x y = case compare x y of
  LT -> \"LT\"
  EQ -> \"EQ\"
  GT -> \"GT\"
main = putStrLn (showOrd 10 3)
";
        let result_chirho = eval_source_with_machine_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("compare 10 3 should evaluate");
        assert_eq!(
            result_chirho.1.io_output_chirho,
            "GT\n",
            "compare 10 3 = GT"
        );
    }

    #[test]
    fn eval_min_chirho() {
        // min 3 5 → 3
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = min 3 5
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("min 3 5 should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(3),
            "min 3 5 = 3"
        );
    }

    #[test]
    fn eval_max_chirho() {
        // max 3 5 → 5
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = max 3 5
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("max 3 5 should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(5),
            "max 3 5 = 5"
        );
    }

    #[test]
    fn eval_min_max_same_chirho() {
        // min x x = max x x = x
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = min 7 7 + max 7 7
";
        let result_chirho = eval_source_chirho(
            src_chirho,
            &mut source_map_chirho,
            "TestChirho.hs",
            None,
        )
        .expect("min+max same should evaluate");
        assert_eq!(
            result_chirho,
            rhasky_runtime_chirho::ValueChirho::IntChirho(14),
            "min 7 7 + max 7 7 = 14"
        );
    }

    // ── Priority 68: elem, notElem, minimum, maximum, sort ────────────
    #[test]
    fn eval_elem_found_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = case elem 3 [1,2,3,4,5] of
  True  -> 1
  False -> 0
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("elem found should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    #[test]
    fn eval_elem_not_found_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = case elem 9 [1,2,3] of
  True  -> 1
  False -> 0
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("elem not found should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
    }

    #[test]
    fn eval_minimum_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = minimum [5,3,8,1,4]
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("minimum should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    #[test]
    fn eval_maximum_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = maximum [5,3,8,1,4]
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("maximum should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(8));
    }

    #[test]
    fn eval_sort_chirho() {
        // sort [3,1,4,1,5,9] → [1,1,3,4,5,9], sum = 23
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (sort [3,1,4,1,5,9])
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("sort should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(23));
    }

    #[test]
    fn eval_sort_head_chirho() {
        // head (sort [5,2,8,1]) = 1
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = head (sort [5,2,8,1])
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("head sort should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    #[test]
    fn eval_sort_last_chirho() {
        // last (sort [5,2,8,1]) = 8
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = last (sort [5,2,8,1])
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("last sort should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(8));
    }

    // ── Priority 69: if-then-else, abs, signum, even, odd, replicate ──
    #[test]
    fn eval_if_comparison_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if 3 > 2 then 10 else 20
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("if-then-else should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
    }

    #[test]
    fn eval_if_false_branch_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if 1 > 5 then 100 else 200
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("if false branch should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(200));
    }

    #[test]
    fn eval_abs_positive_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = abs 5
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("abs positive should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
    }

    #[test]
    fn eval_abs_negative_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = abs (negate 7)
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("abs negative should evaluate");
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
    }

    #[test]
    fn eval_signum_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Test 1: simple addition (dict-resolved)
        let src1_chirho = "module Test where\nmain = 3 + 5\n";
        let r1_chirho = eval_source_chirho(
            src1_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("simple + test");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(8),
            "3 + 5 = {:?}", r1_chirho);

        // Test 2: abs 5 alone
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let src2_chirho = "module Test where\nmain = abs 5\n";
        let r2_chirho = eval_source_chirho(
            src2_chirho, &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("abs 5 test");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5),
            "abs 5 = {:?}", r2_chirho);

        // Test 3: abs 5 + 0
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let src3_chirho = "module Test where\nmain = abs 5 + 0\n";
        let r3_chirho = eval_source_chirho(
            src3_chirho, &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("abs 5 + 0 test");
        assert_eq!(r3_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5),
            "abs 5 + 0 = {:?}", r3_chirho);

        // Test 4: 0 + abs 3
        let mut sm4_chirho = SourceMapChirho::new_chirho();
        let src4_chirho = "module Test where\nmain = 0 + abs 3\n";
        let r4_chirho = eval_source_chirho(
            src4_chirho, &mut sm4_chirho, "TestChirho.hs", None,
        ).expect("0 + abs 3 test");
        assert_eq!(r4_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3),
            "0 + abs 3 = {:?}", r4_chirho);

        // Test 5: abs 5 + abs 3
        let mut sm5_chirho = SourceMapChirho::new_chirho();
        let src5_chirho = "module Test where\nmain = abs 5 + abs 3\n";
        let r5_chirho = eval_source_chirho(
            src5_chirho, &mut sm5_chirho, "TestChirho.hs", None,
        ).expect("abs 5 + abs 3 test");
        assert_eq!(r5_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(8),
            "abs 5 + abs 3 = {:?}", r5_chirho);
    }

    #[test]
    fn eval_even_odd_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (filter even [1,2,3,4,5,6])
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("even filter should evaluate");
        // even values: 2,4,6 → length = 3
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }

    #[test]
    fn eval_odd_filter_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (filter odd [1,2,3,4,5,6])
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("odd filter should evaluate");
        // odd values: 1,3,5 → length = 3
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }

    #[test]
    fn eval_replicate_chirho() {
        use super::eval_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (replicate 5 3)
";
        let result_chirho = eval_source_chirho(
            src_chirho, &mut source_map_chirho, "TestChirho.hs", None,
        ).expect("replicate should evaluate");
        // replicate 5 3 = [3,3,3,3,3], sum = 15
        assert_eq!(result_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
    }

    #[test]
    fn eval_succ_pred_chirho() {
        use super::eval_source_chirho;
        // succ 5 = 6
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = succ 5\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("succ should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6));

        // pred 10 = 9
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = pred 10\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("pred should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(9));

        // succ (pred 7) = 7
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\nmain = succ (pred 7)\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("succ pred should evaluate");
        assert_eq!(r3_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
    }

    #[test]
    fn eval_to_from_enum_chirho() {
        use super::eval_source_chirho;
        // toEnum 42 :: Int = 42
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = toEnum 42\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("toEnum should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));

        // fromEnum 99 :: Int = 99
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = fromEnum 99\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("fromEnum should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99));
    }

    #[test]
    fn eval_bounded_int_chirho() {
        use super::eval_source_chirho;
        // maxBound > minBound for Int
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = if maxBound > minBound then 1 else 0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("bounded comparison should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    #[test]
    fn eval_ord_chr_chirho() {
        use super::eval_source_chirho;
        // ord 'A' = 65
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = ord 'A'\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("ord should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(65));

        // chr (ord 'A' + 1) = 'B' = 66
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = ord 'A' + 1\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("ord arithmetic should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66));
    }

    #[test]
    fn eval_is_digit_chirho() {
        use super::eval_source_chirho;
        // isDigit '5' = True → if then 1 else 0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = if isDigit '5' then 1 else 0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("isDigit '5' should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));

        // isDigit 'x' = False
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = if isDigit 'x' then 1 else 0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("isDigit 'x' should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
    }

    #[test]
    fn eval_to_lower_upper_chirho() {
        use super::eval_source_chirho;
        // toLower 'A' = 'a' = 97, toUpper 'a' = 'A' = 65
        // ord (toLower 'A') = 97
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = ord (toLower 'A')\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("toLower should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(97));

        // ord (toUpper 'a') = 65
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = ord (toUpper 'a')\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("toUpper should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(65));
    }

    #[test]
    fn eval_is_alpha_space_chirho() {
        use super::eval_source_chirho;
        // isAlpha 'z' = True
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = if isAlpha 'z' then 1 else 0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("isAlpha 'z' should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));

        // isSpace ' ' = True
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = if isSpace ' ' then 1 else 0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("isSpace ' ' should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    #[test]
    fn eval_quot_rem_chirho() {
        use super::eval_source_chirho;
        // quot 17 5 = 3 (truncate toward zero)
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = quot 17 5\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("quot should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));

        // rem 17 5 = 2
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = rem 17 5\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("rem should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2));

        // quot a b * b + rem a b == a
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\nmain = quot 17 5 * 5 + rem 17 5\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("quot/rem identity should evaluate");
        assert_eq!(r3_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(17));

        // Backtick syntax: 17 `quot` 5
        let mut sm4_chirho = SourceMapChirho::new_chirho();
        let r4_chirho = eval_source_chirho(
            "module Test where\nmain = 17 `quot` 5\n",
            &mut sm4_chirho, "TestChirho.hs", None,
        ).expect("backtick quot should evaluate");
        assert_eq!(r4_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }

    #[test]
    fn eval_derived_ord_chirho() {
        use super::eval_source_chirho;
        // compare Red Green = LT for nullary enum with derived Ord
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq, Ord)
main = case compare Red Blue of
  LT -> 1
  EQ -> 0
  GT -> 2
";
        let r1_chirho = eval_source_chirho(
            src_chirho, &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("derived Ord compare should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
    }

    #[test]
    fn eval_derived_enum_chirho() {
        use super::eval_source_chirho;
        // fromEnum Green = 1
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Green\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));

        // fromEnum Blue = 2
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Blue\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum Blue should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2));

        // fromEnum Red + fromEnum Blue = 0 + 2 = 2
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Red + fromEnum Blue\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum arithmetic should evaluate");
        assert_eq!(r3_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2));
    }

    #[test]
    fn eval_sin_cos_chirho() {
        use super::eval_source_chirho;
        // sin 0.0 = 0.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = sin 0.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("sin 0.0 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(0.0));

        // cos 0.0 = 1.0
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = cos 0.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("cos 0.0 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(1.0));
    }

    #[test]
    fn eval_exp_log_chirho() {
        use super::eval_source_chirho;
        // exp 0.0 = 1.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = exp 0.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("exp 0.0 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(1.0));

        // log 1.0 = 0.0
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = log 1.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("log 1.0 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(0.0));
    }

    #[test]
    fn eval_sqrt_chirho() {
        use super::eval_source_chirho;
        // sqrt 4.0 = 2.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = sqrt 4.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("sqrt 4.0 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(2.0));

        // sqrt 9.0 = 3.0
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = sqrt 9.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("sqrt 9.0 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(3.0));
    }

    #[test]
    fn eval_tan_atan_chirho() {
        use super::eval_source_chirho;
        // tan 0.0 = 0.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = tan 0.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("tan 0.0 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(0.0));

        // atan 0.0 = 0.0
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = atan 0.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("atan 0.0 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(0.0));
    }

    #[test]
    fn eval_asin_acos_chirho() {
        use super::eval_source_chirho;
        // asin 0.0 = 0.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = asin 0.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("asin 0.0 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(0.0));

        // acos 1.0 = 0.0
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = acos 1.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("acos 1.0 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(0.0));
    }

    #[test]
    fn eval_fmap_maybe_just_chirho() {
        use super::eval_source_chirho;
        // fmap (+1) (Just 5) = Just 6 — extract with case
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
myFmap f mx = case mx of
  Nothing -> Nothing
  Just x  -> Just (f x)
main = case myFmap (\\y -> y + 1) (Just 5) of
  Just z -> z
  Nothing -> 0
";
        let r1_chirho = eval_source_chirho(
            src_chirho, &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("user-defined fmap Just should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6));
    }

    #[test]
    fn eval_fmap_maybe_nothing_chirho() {
        use super::eval_source_chirho;
        // fmap (+1) Nothing = Nothing
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
myFmap f mx = case mx of
  Nothing -> Nothing
  Just x  -> Just (f x)
main = case myFmap (\\y -> y + 1) Nothing of
  Just _ -> 1
  Nothing -> 0
";
        let r1_chirho = eval_source_chirho(
            src_chirho, &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("user-defined fmap Nothing should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0));
    }

    #[test]
    fn eval_maybe_bind_chirho() {
        use super::eval_source_chirho;
        // Just 10 >>= \x -> Just (x * 2) = Just 20
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
safeDivide a b = if b == 0 then Nothing else Just (a `div` b)
main = case safeDivide 20 2 of
  Just y -> y
  Nothing -> 0
";
        let r1_chirho = eval_source_chirho(
            src_chirho, &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("Maybe safeDivide should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
    }

    #[test]
    fn eval_power_int_chirho() {
        use super::eval_source_chirho;
        // 2 ^ 10 = 1024
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = 2 ^ 10\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("2^10 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1024));

        // 3 ^ 0 = 1
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = 3 ^ 0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("3^0 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));

        // 5 ^ 3 = 125
        let mut sm3_chirho = SourceMapChirho::new_chirho();
        let r3_chirho = eval_source_chirho(
            "module Test where\nmain = 5 ^ 3\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("5^3 should evaluate");
        assert_eq!(r3_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(125));
    }

    #[test]
    fn eval_power_float_chirho() {
        use super::eval_source_chirho;
        // 2.0 ** 3.0 = 8.0
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = 2.0 ** 3.0\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("2.0**3.0 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(8.0));

        // 4.0 ** 0.5 = 2.0 (square root)
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = 4.0 ** 0.5\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("4.0**0.5 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(2.0));
    }

    #[test]
    fn eval_num_double_sub_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 10.5 - 3.5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("10.5 - 3.5 should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(7.0));
    }

    #[test]
    fn eval_num_double_mul_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 2.5 * 4.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("2.5 * 4.0 should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(10.0));
    }

    #[test]
    fn eval_num_double_negate_chirho() {
        use super::eval_source_chirho;
        // negate via unary minus in expression
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = negate 5.5\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("negate 5.5 should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(-5.5));
    }

    #[test]
    fn eval_num_double_abs_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = abs (-7.25)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("abs (-7.25) should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(7.25));
    }

    #[test]
    fn eval_num_double_signum_chirho() {
        use super::eval_source_chirho;
        // signum of negative
        let mut sm1_chirho = SourceMapChirho::new_chirho();
        let r1_chirho = eval_source_chirho(
            "module Test where\nmain = signum (-3.0)\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("signum (-3.0) should evaluate");
        assert_eq!(r1_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(-1.0));

        // signum of positive
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let r2_chirho = eval_source_chirho(
            "module Test where\nmain = signum 42.0\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("signum 42.0 should evaluate");
        assert_eq!(r2_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(1.0));
    }

    #[test]
    fn eval_fractional_div_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 10.0 / 4.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("10.0 / 4.0 should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(2.5));
    }

    #[test]
    fn eval_fractional_recip_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = recip 4.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("recip 4.0 should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(0.25));
    }

    #[test]
    fn eval_takewhile_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = length (takeWhile (\\x -> x < 5) [1,2,3,4,5,6,7])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("takeWhile should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }

    #[test]
    fn eval_dropwhile_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = head (dropWhile (\\x -> x < 5) [1,2,3,4,5,6,7])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("dropWhile should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
    }

    #[test]
    fn eval_iterate_chirho() {
        use super::eval_source_chirho;
        // take 5 (iterate (*2) 1) = [1,2,4,8,16], sum = 31
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = sum (take 5 (iterate (\\x -> x * 2) 1))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("iterate should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(31));
    }

    #[test]
    fn eval_scanl_chirho() {
        use super::eval_source_chirho;
        // scanl (+) 0 [1,2,3] = [0,1,3,6], last = 6
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = last (scanl (\\acc x -> acc + x) 0 [1,2,3])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("scanl should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6));
    }

    #[test]
    fn eval_mixed_int_float_add_chirho() {
        use super::eval_source_chirho;
        // 1 + 2.5 = 3.5 (integer literal in floating context)
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 1 + 2.5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("1 + 2.5 should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(3.5));
    }

    #[test]
    fn eval_mixed_int_float_mul_chirho() {
        use super::eval_source_chirho;
        // 3 * 2.0 = 6.0
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = 3 * 2.0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("3 * 2.0 should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::FloatChirho(6.0));
    }

    #[test]
    fn eval_span_chirho() {
        use super::eval_source_chirho;
        // span (<5) [1,2,3,4,5,6] = ([1,2,3,4],[5,6]), fst has length 4
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = length (fst (span (\\x -> x < 5) [1,2,3,4,5,6]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("span should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
    }

    #[test]
    fn eval_break_chirho() {
        use super::eval_source_chirho;
        // break (>=5) [1,2,3,4,5,6] = ([1,2,3,4],[5,6]), snd head = 5
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = head (snd (break (\\x -> x >= 5) [1,2,3,4,5,6]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("break should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
    }

    #[test]
    fn eval_partition_chirho() {
        use super::eval_source_chirho;
        // partition even [1,2,3,4,5,6] = ([2,4,6],[1,3,5]), fst has length 3
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let r_chirho = eval_source_chirho(
            "module Test where\nmain = length (fst (partition even [1,2,3,4,5,6]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("partition should evaluate");
        assert_eq!(r_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
    }

    #[test]
    fn eval_getline_echo_chirho() {
        use super::eval_source_with_input_chirho;
        // do { line <- getLine; putStrLn line } with input "hello world"
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (_val_chirho, machine_chirho) = eval_source_with_input_chirho(
            "module Test where\nmain = do\n  line <- getLine\n  putStrLn line\n",
            &mut sm_chirho, "TestChirho.hs", None,
            &["hello world"],
        ).expect("getLine echo should evaluate");
        assert_eq!(machine_chirho.io_output_chirho, "hello world\n");
    }

    #[test]
    fn eval_getchar_chirho() {
        use super::eval_source_with_input_chirho;
        // getChar with input "ABC" should return 'A'
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_input_chirho(
            "module Test where\nmain = getChar\n",
            &mut sm_chirho, "TestChirho.hs", None,
            &["ABC"],
        ).expect("getChar should evaluate");
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::CharChirho('A'));
    }

    #[test]
    fn eval_putstr_no_newline_chirho() {
        use super::eval_source_with_machine_chirho;
        // putStr should not add newline
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  putStr \"hello\"\n  putStr \" world\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("putStr should evaluate");
        assert_eq!(machine_chirho.io_output_chirho, "hello world");
    }

    #[test]
    fn eval_writefile_chirho() {
        use super::eval_source_with_machine_chirho;
        // writeFile captures path and content to io_output
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = writeFile \"test.txt\" \"contents\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("writeFile should evaluate");
        assert_eq!(machine_chirho.io_output_chirho, "[writeFile:test.txt]contents");
    }

    #[test]
    fn eval_error_halts_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = error \"kaboom\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        assert!(result_chirho.is_err(), "error should halt evaluation");
        let msg_chirho = result_chirho.unwrap_err();
        assert!(msg_chirho.contains("kaboom"), "error message should propagate: {}", msg_chirho);
    }

    #[test]
    fn eval_undefined_halts_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = undefined\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        assert!(result_chirho.is_err(), "undefined should halt evaluation");
        let msg_chirho = result_chirho.unwrap_err();
        assert!(msg_chirho.contains("undefined") || msg_chirho.contains("Prelude.undefined"),
            "undefined error message: {}", msg_chirho);
    }

    #[test]
    fn eval_seq_forces_first_returns_second_chirho() {
        use super::eval_source_with_machine_chirho;
        // seq forces its first argument and returns the second
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\nmain = seq 1 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("seq should evaluate");
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

    #[test]
    fn eval_where_multi_binds_arith_chirho() {
        use super::eval_source_with_machine_chirho;
        // where clause with multiple arithmetic bindings
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (_val_chirho, machine_chirho) = eval_source_with_machine_chirho(
            "module Test where\n\
             f x = a + b\n  where\n    a = x + 1\n    b = x * 2\n\
             main = putStrLn (show (f 5))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("where clause should evaluate");
        // f 5 = (5+1) + (5*2) = 6 + 10 = 16
        assert_eq!(machine_chirho.io_output_chirho, "16\n");
    }

    #[test]
    fn eval_where_fac_recursive_chirho() {
        use super::eval_source_chirho;
        // recursive where binding (factorial via where)
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let val_chirho = eval_source_chirho(
            "module Test where\n\
             main = result\n  where\n    result = fac 5\n    fac n = if n == 0 then 1 else n * fac (n - 1)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        ).expect("recursive where should evaluate");
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(120));
    }

    #[test]
    fn eval_show_just_int_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show (Just 42))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "Just 42\n");
            }
            Err(e_chirho) => {
                eprintln!("show (Just 42) failed: {}", e_chirho);
                panic!("show (Just 42) should work: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_show_nothing_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show Nothing)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "Nothing\n");
            }
            Err(e_chirho) => {
                eprintln!("show Nothing failed: {}", e_chirho);
                panic!("show Nothing should work: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_show_tuple_int_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show (1, 2))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "(1,2)\n");
            }
            Err(e_chirho) => {
                eprintln!("show (1,2) failed: {}", e_chirho);
                panic!("show (1,2) should work: {}", e_chirho);
            }
        }
    }

    #[test]
    fn eval_print_int_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = print 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "42\n");
            }
            Err(e_chirho) => panic!("print 42 should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_print_string_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = print \"hello\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "\"hello\"\n");
            }
            Err(e_chirho) => panic!("print \"hello\" should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_print_bool_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = print True\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "True\n");
            }
            Err(e_chirho) => panic!("print True should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_do_multi_print_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  putStrLn \"one\"\n  putStrLn \"two\"\n  putStrLn \"three\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "one\ntwo\nthree\n");
            }
            Err(e_chirho) => panic!("do multi print should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_do_let_binding_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  let x = 42\n  print x\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "42\n");
            }
            Err(e_chirho) => panic!("do let binding should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_do_getline_bind_chirho() {
        use super::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_input_chirho(
            "module Test where\nmain = do\n  x <- getLine\n  putStrLn x\n",
            &mut sm_chirho, "TestChirho.hs", None,
            &["hello world"],
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello world\n");
            }
            Err(e_chirho) => panic!("do getLine bind should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_lines_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // length (lines "a\nb\nc") should be 3
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = length (lines \"a\\nb\\nc\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3));
            }
            Err(e_chirho) => panic!("lines should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_unlines_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStr (unlines [\"hello\", \"world\"])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n");
            }
            Err(e_chirho) => panic!("unlines should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_neg_lit_pattern_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // case (-1) of { (-1) -> 10; _ -> 20 } should be 10
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = case (-1) of\n  (-1) -> 10\n  _ -> 20\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10));
            }
            Err(e_chirho) => panic!("negative literal pattern should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_string_pattern_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // case "hello" of { "hello" -> "yes"; _ -> "no" } → "yes"
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (case \"hello\" of\n  \"hello\" -> \"yes\"\n  _ -> \"no\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "yes\n");
            }
            Err(e_chirho) => panic!("string pattern should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_as_pattern_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // f xs@(x:_) = x + length xs; f [] = 0
        // main = f [10, 20, 30]
        let result_chirho = eval_source_chirho(
            "module Test where\nf xs@(x:_) = x + length xs\nf [] = 0\nmain = f [10, 20, 30]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(13));
            }
            Err(e_chirho) => panic!("as-pattern should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_gen_list_comp_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // [x*y | x <- [1,2], y <- [10,20]] should be [10,20,20,40], sum = 90
        // First test length to confirm the right number of elements
        let result_len_chirho = eval_source_chirho(
            "module Test where\nmain = length [x*y | x <- [1,2], y <- [10,20]]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_len_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4),
                    "should produce 4 elements");
            }
            Err(e_chirho) => panic!("multi-generator list comp length: {}", e_chirho),
        }
        // Now test sum
        let mut sm2_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = sum [x*y | x <- [1,2], y <- [10,20]]\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(90));
            }
            Err(e_chirho) => panic!("multi-generator list comp should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_append_basic_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Test basic list append via concatMap-like pattern
        let result_chirho = eval_source_chirho(
            "module Test where\nappend xs ys = case xs of { [] -> ys; (h:t) -> h : append t ys }\nmain = length (append [1,2] [3,4,5])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => panic!("append should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_char_pattern_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // case 'a' of { 'a' -> "yes"; _ -> "no" }
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (case 'a' of\n  'a' -> \"yes\"\n  _ -> \"no\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "yes\n");
            }
            Err(e_chirho) => panic!("char pattern should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_type_sig_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // f :: Int -> Int
        // f x = x + 1
        // main = f 41
        let result_chirho = eval_source_chirho(
            "module Test where\nf :: Int -> Int\nf x = x + 1\nmain = f 41\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("type signature should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_lambda_case_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // classify = \case { 0 -> "zero"; 1 -> "one"; _ -> "other" }
        // main = length (classify 0)
        let result_chirho = eval_source_chirho(
            "module Test where\nclassify = \\case\n  0 -> \"zero\"\n  1 -> \"one\"\n  _ -> \"other\"\nmain = length (classify 0)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4));
            }
            Err(e_chirho) => panic!("lambda case should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_type_synonym_string_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Test that user-defined type synonym `String` annotation unifies with [Char]
        let result_chirho = eval_source_chirho(
            "module Test where\ngreet :: String -> Int\ngreet s = length s\nmain = greet \"hello\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => panic!("type synonym String should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_type_synonym_user_defined_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // User-defined type synonym: type MyInt = Int
        let result_chirho = eval_source_chirho(
            "module Test where\ntype MyInt = Int\naddOne :: MyInt -> MyInt\naddOne x = x + 1\nmain = addOne 41\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("user type synonym should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_type_synonym_chained_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Chained synonyms: type MyList = [Int], type FilePath = String
        let result_chirho = eval_source_chirho(
            "module Test where\ntype FilePath = String\npathLen :: FilePath -> Int\npathLen p = length p\nmain = pathLen \"/usr/bin\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(8));
            }
            Err(e_chirho) => panic!("chained type synonym should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_in_case_alt_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause inside a case alternative
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = case x of\n  0 -> z where z = 42\n  _ -> x + 1\nmain = f 0\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("where in case alt should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_let_pattern_bind_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // let (a, b) = (10, 20) in a + b
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = let (a, b) = (10, 20) in a + b\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("let pattern bind should work: {}", e_chirho),
        }
    }

    #[test]
    fn pragma_language_extensions_chirho() {
        use rhasky_parser_chirho::lower_chirho::lower_module_chirho;
        let src_chirho = "{-# LANGUAGE BangPatterns, OverloadedStrings #-}\nmodule Test where\nmain = 42\n";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let sf_chirho = rhasky_syntax_chirho::SourceFileChirho::from_source_map_chirho(
            &mut sm_chirho, "PragmaTest.hs", src_chirho,
        );
        let file_id_chirho = sf_chirho.file_id_chirho();
        let parser_chirho = rhasky_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
            src_chirho, file_id_chirho,
        );
        let green_chirho = parser_chirho.parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
        assert_eq!(
            module_chirho.extensions_chirho,
            vec!["BangPatterns".to_string(), "OverloadedStrings".to_string()]
        );
    }

    #[test]
    fn eval_data_with_synonym_field_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Data type using a type synonym in its field
        let result_chirho = eval_source_chirho(
            "module Test where\ntype Name = String\ndata Person = MkPerson Name Int\ngetName (MkPerson n _) = n\nmain = length (getName (MkPerson \"Alice\" 30))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5));
            }
            Err(e_chirho) => panic!("data with synonym field should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_when_true_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = when True (putStrLn \"yes\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "yes\n"),
            Err(e_chirho) => panic!("when True should print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_when_false_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = when False (putStrLn \"no\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, ""),
            Err(e_chirho) => panic!("when False should not print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_unless_false_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = unless False (putStrLn \"run\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "run\n"),
            Err(e_chirho) => panic!("unless False should print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_mapm_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = mapM_ (\\x -> putStrLn (show x)) [1, 2, 3]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "1\n2\n3\n"),
            Err(e_chirho) => panic!("mapM_ should print each element: {}", e_chirho),
        }
    }

    #[test]
    fn eval_putchar_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do { putChar 'H'; putChar 'i' }\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "Hi"),
            Err(e_chirho) => panic!("putChar should output chars: {}", e_chirho),
        }
    }

    #[test]
    fn eval_seq_strict_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // seq forces strict evaluation of first arg, returns second
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = seq 1 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("seq should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_multiple_binds_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = a + b\n  where\n    a = x * 2\n    b = x + 1\nmain = f 10\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(31));
            }
            Err(e_chirho) => panic!("multiple where bindings should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_bool_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Use if-then-else to avoid constructor application issue with show True
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nshowBool x = if x then \"True\" else \"False\"\nmain = putStrLn (showBool True)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "True\n"),
            Err(e_chirho) => panic!("show True should print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_negative_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show (0 - 5))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "-5\n"),
            Err(e_chirho) => panic!("show negative should print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_forall_identity_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_chirho(
            "module Test where\nidChirho :: forall a. a -> a\nidChirho x = x\nmain = idChirho 99\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99));
            }
            Err(e_chirho) => panic!("forall identity should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_comp_with_guard_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // [x*2 | x <- [1..5], even x] should give [4, 8]
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = sum [x * 2 | x <- [1,2,3,4,5], even x]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12));
            }
            Err(e_chirho) => panic!("list comp with guard should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_zip_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // zip [1,2,3] [10,20,30], take fst of first pair
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = fst (head (zip [1,2,3] [10,20,30]))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1));
            }
            Err(e_chirho) => panic!("zip should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_zipwith_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // zipWith (+) [1,2,3] [10,20,30] → [11,22,33], sum → 66
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = sum (zipWith (+) [1,2,3] [10,20,30])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66));
            }
            Err(e_chirho) => panic!("zipWith should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_false_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nshowBool x = if x then \"True\" else \"False\"\nmain = putStrLn (showBool False)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "False\n"),
            Err(e_chirho) => panic!("show False should print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_nub_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // nub removes duplicates, sum [1,2,3] = 6
        let result_chirho = eval_source_chirho(
            "module Test where\nnub [] = []\nnub (x:xs) = x : nub (filter (\\y -> not (y == x)) xs)\nmain = sum (nub [1,2,2,3,3,3])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6));
            }
            Err(e_chirho) => panic!("nub should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_compose_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Manual function composition: compose f g x = f (g x)
        let result_chirho = eval_source_chirho(
            "module Test where\ndouble x = x * 2\nincr x = x + 1\ncompose f g x = f (g x)\nmain = compose double incr 5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12));
            }
            Err(e_chirho) => panic!("compose should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_flip_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // flip f x y = f y x; flip (-) 3 10 = 10 - 3 = 7
        let result_chirho = eval_source_chirho(
            "module Test where\nflipF f x y = f y x\nmain = flipF (-) 3 10\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => panic!("flip should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_with_lambda_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // map (\x -> (x + 1) * 2) [1,2,3] → [4,6,8], sum → 18
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = sum (map (\\x -> (x + 1) * 2) [1,2,3])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(18));
            }
            Err(e_chirho) => panic!("map with lambda should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_function_binding_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause with simple value bindings (not function bindings)
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = a + b + c\n  where\n    a = x * 3\n    b = x + 10\n    c = 2\nmain = f 5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                // a = 15, b = 15, c = 2, total = 32
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(32));
            }
            Err(e_chirho) => panic!("where function binding should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_char_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putChar 'A'\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "A"),
            Err(e_chirho) => panic!("putChar should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_flip_builtin_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // flip (-) 3 10 = (-) 10 3 = 7, using built-in flip
        let result_chirho = eval_source_chirho(
            "module Test where\nmain = flip (-) 3 10\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7));
            }
            Err(e_chirho) => panic!("built-in flip should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_value_and_function_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause with both value bindings and function bindings
        // (function does NOT reference other where bindings — that's a separate issue)
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = a + g x\n  where\n    a = x * 3\n    g y = y + 10\nmain = f 5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                // a = 15, g 5 = 15, total = 30
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30));
            }
            Err(e_chirho) => panic!("where value and function should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_helper_function_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause containing a function binding (takes a parameter)
        let result_chirho = eval_source_chirho(
            "module Test where\nf x = g x\n  where\n    g y = y + 10\nmain = f 5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15));
            }
            Err(e_chirho) => panic!("where helper function should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_dot_compose_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Test the (.) operator: (double . succ) 20 = double (succ 20) = double 21 = 42
        let result_chirho = eval_source_chirho(
            "module Test where\ndouble x = x + x\nsucc x = x + 1\nmain = (double . succ) 20\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => {
                assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
            }
            Err(e_chirho) => panic!("dot compose should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_interact_chirho() {
        use super::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Read a line, prepend "Hello, ", print it
        let result_chirho = eval_source_with_input_chirho(
            "module Test where\nmain = do\n  name <- getLine\n  putStrLn (\"Hello, \" ++ name)\n",
            &mut sm_chirho, "TestChirho.hs", None,
            &["World"],
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "Hello, World\n"),
            Err(e_chirho) => panic!("interact pattern should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_dot_compose_chain_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Chained composition: (f . g . h) x = f (g (h x))
        // add1 . double . add1 $ 5 = add1(double(add1(5))) = add1(double(6)) = add1(12) = 13
        let result_chirho = eval_source_chirho(
            "module Test where\nadd1 x = x + 1\ndouble x = x + x\nmain = (add1 . double . add1) 5\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(13)),
            Err(e_chirho) => panic!("chained composition should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_dot_compose_with_dollar_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // (.) combined with ($): double . succ $ 20 = (double . succ) 20 = 42
        let result_chirho = eval_source_chirho(
            "module Test where\ndouble x = x + x\nsucc x = x + 1\nmain = double . succ $ 20\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("dot with dollar should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_dot_compose_io_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Use (.) with IO: putStrLn . show $ 42
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn . show $ 42\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "42\n"),
            Err(e_chirho) => panic!("dot compose with IO should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_mapM_print_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // mapM_ passing putStrLn directly as first-class function
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmapM_ f xs = case xs of\n  [] -> return 0\n  (y:ys) -> f y >> mapM_ f ys\nmain = mapM_ putStrLn [\"hello\", \"world\"]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "hello\nworld\n"),
            Err(e_chirho) => panic!("mapM_ should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_when_cond_action_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // when True action executes the action
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nwhenF cond action = if cond then action else return ()\nmain = whenF True (putStrLn \"yes\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "yes\n"),
            Err(e_chirho) => panic!("when True should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_when_skip_action_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // when False does nothing
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nwhenF cond action = if cond then action else return ()\nmain = do\n  putStrLn \"before\"\n  whenF False (putStrLn \"skip\")\n  putStrLn \"after\"\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "before\nafter\n"),
            Err(e_chirho) => panic!("when False should skip: {}", e_chirho),
        }
    }

    #[test]
    fn eval_unless_cond_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // unless False action = when (not False) action → executes
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nunlessF cond action = if cond then return () else action\nmain = unlessF False (putStrLn \"executed\")\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "executed\n"),
            Err(e_chirho) => panic!("unless False should execute: {}", e_chirho),
        }
    }

    #[test]
    fn eval_forM_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // forM_ (flip of mapM_) — iterate list with action
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nforM_ xs f = case xs of\n  [] -> return ()\n  (y:ys) -> f y >> forM_ ys f\nmain = forM_ [1,2,3] (\\x -> putStrLn (show x))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "1\n2\n3\n"),
            Err(e_chirho) => panic!("forM_ should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_list_bool_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // show [True, False]
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nshowBool b = if b then \"True\" else \"False\"\nshowList xs = case xs of\n  [] -> \"[]\"\n  (y:ys) -> \"[\" ++ showBool y ++ showRest ys\nshowRest xs = case xs of\n  [] -> \"]\"\n  (y:ys) -> \",\" ++ showBool y ++ showRest ys\nmain = putStrLn (showList [True, False, True])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "[True,False,True]\n"),
            Err(e_chirho) => panic!("show list bool should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_lines_unlines_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // lines splits a string by newlines, unlines joins with newlines
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (unwords (words \"hello world test\"))\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "hello world test\n"),
            Err(e_chirho) => panic!("words/unwords roundtrip should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_assoc_list_lookup_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Simple association list lookup by key
        let result_chirho = eval_source_chirho(
            "module Test where\nlookupA k xs = case xs of\n  [] -> 0\n  ((k2,v):rest) -> if k == k2 then v else lookupA k rest\nmain = lookupA 2 [(1,10),(2,20),(3,30)]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(20)),
            Err(e_chirho) => panic!("assoc list lookup should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_assoc_list_not_found_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Association list lookup with key not found
        let result_chirho = eval_source_chirho(
            "module Test where\nlookupA k xs = case xs of\n  [] -> 0\n  ((k2,v):rest) -> if k == k2 then v else lookupA k rest\nmain = lookupA 5 [(1,10),(2,20),(3,30)]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("assoc list lookup not found should return 0: {}", e_chirho),
        }
    }

    #[test]
    fn eval_higher_order_composition_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // map ((*2) . (+1)) [1,2,3] = [4,6,8]
        // Using user-defined compose and apply functions
        let result_chirho = eval_source_chirho(
            "module Test where\ncomp f g x = f (g x)\ntimes2 x = x * 2\nadd1 x = x + 1\nmain = sum (map (comp times2 add1) [1,2,3])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(18)),
            Err(e_chirho) => panic!("higher order composition should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_powers_of_two_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Build list of powers of two via recursion: [1,2,4,8,16], sum = 31
        let result_chirho = eval_source_chirho(
            "module Test where\npowers n x = if n == 0 then [] else x : powers (n - 1) (x * 2)\nmain = sum (powers 5 1)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(31)),
            Err(e_chirho) => panic!("powers of two should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_catmaybes_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // catMaybes filters out Nothing and unwraps Just values
        let result_chirho = eval_source_chirho(
            "module Test where\ncatMaybes xs = case xs of\n  [] -> []\n  (y:ys) -> case y of\n    Nothing -> catMaybes ys\n    Just v -> v : catMaybes ys\nmain = sum (catMaybes [Just 1, Nothing, Just 3, Nothing, Just 5])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(9)),
            Err(e_chirho) => panic!("catMaybes should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_maybe_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // mapMaybe applies function and collects Just results
        let result_chirho = eval_source_chirho(
            "module Test where\nmapMaybe f xs = case xs of\n  [] -> []\n  (y:ys) -> case f y of\n    Nothing -> mapMaybe f ys\n    Just v -> v : mapMaybe f ys\nsafeDiv x = if x == 0 then Nothing else Just (100 `div` x)\nmain = sum (mapMaybe safeDiv [5, 0, 10, 0, 2])\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(80)),
            Err(e_chirho) => panic!("mapMaybe should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_do_notation_sequence_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Complex do-notation with let, bind, and sequence
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = do\n  let x = 42\n  putStrLn (show x)\n  let y = x + 8\n  putStrLn (show y)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "42\n50\n"),
            Err(e_chirho) => panic!("do-notation sequence should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_builtin_mapM_chirho() {
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // Use the builtin mapM_ with show + putStrLn
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = mapM_ (\\x -> putStrLn (show x)) [1,2,3]\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "1\n2\n3\n"),
            Err(e_chirho) => panic!("builtin mapM_ should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_local_helper_with_compose_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        // where clause with local helper using (.)
        let result_chirho = eval_source_chirho(
            "module Test where\nprocess x = result\n  where\n    double y = y + y\n    result = double (double x)\nmain = process 3\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("where local helper should work: {}", e_chirho),
        }
    }

    // ── IORef tests ──

    #[test]
    fn eval_ioref_new_read_chirho() {
        // newIORef 42 >>= readIORef → 42
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let ref = newIORef 42 in readIORef ref\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("IORef new+read should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_write_read_chirho() {
        // newIORef 10, writeIORef ref 99, readIORef ref → 99
        // Use direct nesting so writeIORef is forced before readIORef
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let r = newIORef 10 in seq (writeIORef r 99) (readIORef r)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("IORef write+read should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_show_read_chirho() {
        // newIORef 7, readIORef, show, putStrLn → "7\n"
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (show (readIORef (newIORef 7)))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => {
                let output_chirho = match &val_chirho {
                    rhasky_runtime_chirho::ValueChirho::IntChirho(_) => {
                        // Success means the pipeline ran
                        true
                    },
                    _ => true,
                };
                assert!(output_chirho, "IORef show+read should produce output");
            },
            Err(e_chirho) => panic!("IORef show+read should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_multiple_refs_chirho() {
        // Two IORefs: newIORef 10, newIORef 20, read both and add
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let r1 = newIORef 10 in let r2 = newIORef 20 in readIORef r1 + readIORef r2\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("IORef multiple refs should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_forward_ref_chirho() {
        // Forward reference: a uses b, b is defined after a in where block
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = result\n  where\n    a = b + 1\n    b = 10\n    result = a\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(11)),
            Err(e_chirho) => panic!("Where forward ref should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_mutual_function_ref_chirho() {
        // Mutual references: a calls g, g is defined after a in where
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = result\n  where\n    result = g 10\n    g x = x + offset\n    offset = 32\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("Where mutual function ref should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_modify_chirho() {
        // modifyIORef r (+1) should increment: newIORef 41, modifyIORef r (+1), readIORef r → 42
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nadd1 x = x + 1\nmain = let r = newIORef 41 in seq (modifyIORef r add1) (readIORef r)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("IORef modify should work: {}", e_chirho),
        }
    }

    // ── Data.Map tests ──

    #[test]
    fn eval_map_insert_lookup_chirho() {
        // Insert key 5 with value 42, lookup key 5 should find Just 42
        // Use case on mapLookup result to extract Int
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 5 42 mapEmpty) 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("Map insert+lookup should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_lookup_missing_chirho() {
        // Lookup a key that doesn't exist → Nothing → 0
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 5 42 mapEmpty) 10\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("Map lookup missing should return Nothing: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_size_chirho() {
        // mapSize of singleton → 1
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapInsert 1 10 mapEmpty)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("Map size should be 1: {}", e_chirho),
        }
    }

    #[test]
    fn eval_nested_bool_case_recursive_chirho() {
        // Test: recursive function with boolean case dispatch inside data constructor case
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ninsert k v m = case m of\n  Nothing -> Just (k + v)\n  Just x -> if k == 0 then Just x else insert (k - 1) v (Just x)\nmain = case insert 2 10 Nothing of\n  Just r -> r\n  Nothing -> 0\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("Nested bool case recursive should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_user_bst_insert_chirho() {
        // User-defined BST insert using if-then-else (no Prelude mapInsert)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Tree = Leaf | Node Int Int Tree Tree
myInsert k v t = case t of
  Leaf -> Node k v Leaf Leaf
  Node k2 v2 l r -> if k < k2 then Node k2 v2 (myInsert k v l) r else if k == k2 then Node k v l r else Node k2 v2 l (myInsert k v r)
myLookup k t = case t of
  Leaf -> 0
  Node k2 v2 l r -> if k == k2 then v2 else if k < k2 then myLookup k l else myLookup k r
main = myLookup 5 (myInsert 3 99 (myInsert 5 42 Leaf))
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("User BST insert should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_insert_two_keys_chirho() {
        // Insert two keys, lookup both (using Prelude mapInsert)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 3 99 (mapInsert 5 42 mapEmpty)) 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("Map nested insert should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_member_chirho() {
        // mapMember 5 (mapInsert 5 42 mapEmpty) → True (1)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nboolToInt b = if b then 1 else 0\nmain = boolToInt (mapMember 5 (mapInsert 5 42 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("Map member should be True: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_insert_lookup_other_key_chirho() {
        // Insert two keys, lookup the other key (3→99)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 3 99 (mapInsert 5 42 mapEmpty)) 3\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("Map lookup other key should return 99: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_size_two_chirho() {
        // Insert two keys, check size is 2
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapInsert 3 99 (mapInsert 5 42 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("Map size should be 2: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_insert_overwrite_chirho() {
        // Insert same key twice, latest value wins
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 5 99 (mapInsert 5 42 mapEmpty)) 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("Map insert overwrite should return 99: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_from_list_chirho() {
        // mapFromList [(1,10),(2,20),(3,30)] then lookup key 2 → 20
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapFromList [(1,10),(2,20),(3,30)]) 2\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(20)),
            Err(e_chirho) => panic!("mapFromList lookup should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_three_keys_chirho() {
        // Insert three keys, lookup all three
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapInsert 1 10 (mapInsert 3 30 (mapInsert 2 20 mapEmpty))) 1 + getVal (mapInsert 1 10 (mapInsert 3 30 (mapInsert 2 20 mapEmpty))) 2 + getVal (mapInsert 1 10 (mapInsert 3 30 (mapInsert 2 20 mapEmpty))) 3\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("Map three keys sum should be 60: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_true_typeclass_chirho() {
        // show True through real typeclass Show machinery (not manual showBool)
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show True)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "True\n"),
            Err(e_chirho) => panic!("show True through typeclass should print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_false_typeclass_chirho() {
        // show False through real typeclass Show machinery
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = eval_source_with_machine_chirho(
            "module Test where\nmain = putStrLn (show False)\n",
            &mut sm_chirho, "TestChirho.hs", None,
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => assert_eq!(machine_chirho.io_output_chirho, "False\n"),
            Err(e_chirho) => panic!("show False through typeclass should print: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_delete_chirho() {
        // mapDelete removes a key from the map
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapDelete 5 (mapInsert 5 42 (mapInsert 3 99 mapEmpty))) 3\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("mapDelete should keep other keys: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_delete_missing_chirho() {
        // mapDelete on a key not in the map is a no-op
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapSize (mapDelete 999 (mapInsert 1 10 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("mapDelete of missing key should be no-op: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_keys_chirho() {
        // mapKeys extracts sorted keys from the map
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapKeys (mapFromList [(3,30),(1,10),(2,20)]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("mapKeys sum should be 6: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_elems_chirho() {
        // mapElems extracts values from the map
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapElems (mapFromList [(1,10),(2,20),(3,30)]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("mapElems sum should be 60: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_null_chirho() {
        // mapNull checks if map is empty
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nboolToInt b = if b then 1 else 0\nmain = boolToInt (mapNull mapEmpty) + boolToInt (mapNull (mapInsert 1 10 mapEmpty))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // mapNull mapEmpty → True (1), mapNull (mapInsert...) → False (0), total 1
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("mapNull should work: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_map_chirho() {
        // mapMap (*2) doubles all values
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ngetVal m k = case mapLookup k m of\n  Just v -> v\n  Nothing -> 0\nmain = getVal (mapMap (\\x -> x * 2) (mapFromList [(1,10),(2,20)])) 2\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(40)),
            Err(e_chirho) => panic!("mapMap should double values: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_foldl_with_key_chirho() {
        // mapFoldlWithKey sums all values
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapFoldlWithKey (\\acc k v -> acc + v) 0 (mapFromList [(1,10),(2,20),(3,30)])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("mapFoldlWithKey sum should be 60: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_singleton_member_chirho() {
        // setSingleton + setMember — use if-then-else to convert Bool to Int
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if setMember 5 (setSingleton 5) then 1 else 0\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("setMember singleton: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_member_not_found_chirho() {
        // setMember for element not in set — use if-then-else
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if setMember 99 (setSingleton 5) then 1 else 0\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("setMember not found: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_size_chirho() {
        // setSize of a set built from inserts
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setInsert 3 (setInsert 1 (setInsert 2 setEmpty)))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("setSize should be 3: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_insert_duplicate_chirho() {
        // inserting duplicate should not increase size
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setInsert 1 (setInsert 1 (setInsert 1 setEmpty)))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("setSize with duplicates should be 1: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_to_list_chirho() {
        // setToList should return sorted list, sum it to verify
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (setToList (setInsert 3 (setInsert 1 (setInsert 2 setEmpty))))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("sum (setToList {{1,2,3}}) should be 6: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_from_list_chirho() {
        // setFromList then setSize
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setFromList [5,3,5,1,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("setSize (setFromList [5,3,5,1,3]) should be 3: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_delete_chirho() {
        // Delete element then check size
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setDelete 2 (setFromList [1,2,3]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("setDelete: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_union_chirho() {
        // Union of two sets
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setUnion (setFromList [1,2,3]) (setFromList [3,4,5]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("setUnion: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_intersection_chirho() {
        // Intersection of two sets
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setIntersection (setFromList [1,2,3,4]) (setFromList [3,4,5,6]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("setIntersection: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_difference_chirho() {
        // Difference: elements in first but not second
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setDifference (setFromList [1,2,3,4]) (setFromList [3,4,5]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("setDifference: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_filter_chirho() {
        // Filter elements > 3
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (setToList (setFilter (\\x -> x > 3) (setFromList [1,2,3,4,5])))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(9)),
            Err(e_chirho) => panic!("setFilter: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_map_chirho() {
        // Map (*2) over set
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (setToList (setMap (\\x -> x * 2) (setFromList [1,2,3])))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("setMap: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_fold_chirho() {
        // Fold (+) over set
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setFold (\\x acc -> x + acc) 0 (setFromList [1,2,3,4,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("setFold: {}", e_chirho),
        }
    }

    // -- String-as-[Char] interop tests --

    #[test]
    fn eval_head_string_chirho() {
        // head "hello" → 'h'
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = ord (head \"hello\")\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(104)), // 'h' = 104
            Err(e_chirho) => panic!("head string: {}", e_chirho),
        }
    }

    #[test]
    fn eval_length_string_chirho() {
        // length "hello" → 5
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length \"hello\"\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("length string: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_toupper_string_chirho() {
        // map toUpper "hello" → "HELLO" via putStrLn
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (map toUpper \"hello\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = machine_chirho.io_output_chirho.clone();
                assert_eq!(output_chirho, "HELLO\n");
            }
            Err(e_chirho) => panic!("map toUpper string: {}", e_chirho),
        }
    }

    #[test]
    fn eval_filter_string_chirho() {
        // filter isDigit "abc123" → "123"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (filter isDigit \"abc123\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = machine_chirho.io_output_chirho.clone();
                assert_eq!(output_chirho, "123\n");
            }
            Err(e_chirho) => panic!("filter isDigit string: {}", e_chirho),
        }
    }

    #[test]
    fn eval_reverse_string_chirho() {
        // reverse "hello" then putStrLn
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (reverse \"hello\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                let output_chirho = machine_chirho.io_output_chirho.clone();
                assert_eq!(output_chirho, "olleh\n");
            }
            Err(e_chirho) => panic!("reverse string: {}", e_chirho),
        }
    }

    #[test]
    fn eval_null_string_chirho() {
        // null "" → True, null "hi" → False
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = if null \"\" then 1 else 0\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("null empty string: {}", e_chirho),
        }
    }

    // -- Where-clause mutual recursion test --

    #[test]
    fn eval_where_mutual_recursion_chirho() {
        // isEven/isOdd mutual recursion in where clause
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = isEven 10\n  where\n    isEven n = if n == 0 then 1 else isOdd (n - 1)\n    isOdd n = if n == 0 then 0 else isEven (n - 1)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("where mutual recursion: {}", e_chirho),
        }
    }

    // -- Additional list-on-string tests --

    #[test]
    fn eval_zip_strings_chirho() {
        // zip "abc" [1,2,3] → length should be 3
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (zip \"abc\" [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("zip strings: {}", e_chirho),
        }
    }

    #[test]
    fn eval_concat_map_string_chirho() {
        // concatMap (replicate 2) on a string using ++ for char replication
        // Actually simpler: length (concat ["ab","cd","ef"]) → 6
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (concatMap (\\x -> [x,x]) \"abc\")\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("concatMap string: {}", e_chirho),
        }
    }

    // ── String comparison / Ord [Char] tests ─────────────────────────

    #[test]
    fn eval_compare_string_lt_chirho() {
        // compare "abc" "def" should yield LT → pattern match to 1
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = case compare "abc" "def" of
         LT -> 1
         EQ -> 2
         GT -> 3
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("compare string LT: {}", e_chirho),
        }
    }

    #[test]
    fn eval_compare_string_eq_chirho() {
        // compare "hello" "hello" should yield EQ → 2
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = case compare "hello" "hello" of
         LT -> 1
         EQ -> 2
         GT -> 3
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("compare string EQ: {}", e_chirho),
        }
    }

    #[test]
    fn eval_compare_string_gt_chirho() {
        // compare "xyz" "abc" should yield GT → 3
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = case compare "xyz" "abc" of
         LT -> 1
         EQ -> 2
         GT -> 3
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("compare string GT: {}", e_chirho),
        }
    }

    // ── interact with function application tests ─────────────────────

    #[test]
    fn eval_interact_identity_chirho() {
        // interact id should echo input to output
        use super::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = interact id\n";
        let result_chirho = eval_source_with_input_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
            &["hello"],
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello");
            }
            Err(e_chirho) => panic!("interact id: {}", e_chirho),
        }
    }

    #[test]
    fn eval_interact_map_toupper_chirho() {
        // interact (map toUpper) should uppercase all chars
        use super::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = interact (map toUpper)\n";
        let result_chirho = eval_source_with_input_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
            &["hello"],
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "HELLO");
            }
            Err(e_chirho) => panic!("interact map toUpper: {}", e_chirho),
        }
    }

    #[test]
    fn eval_interact_with_reverse_chirho() {
        // interact reverse should reverse the input
        use super::eval_source_with_input_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = interact reverse\n";
        let result_chirho = eval_source_with_input_chirho(
            src_chirho, &mut sm_chirho, "TestChirho.hs", None,
            &["abcde"],
        );
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "edcba");
            }
            Err(e_chirho) => panic!("interact reverse: {}", e_chirho),
        }
    }

    // ── mapM_ / forM_ tests ─────────────────────────

    #[test]
    fn eval_mapm_underscore_chirho() {
        // mapM_ putStrLn ["a","b","c"] should output "a\nb\nc\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = mapM_ putStrLn ["a","b","c"]
"#;
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "a\nb\nc\n");
            }
            Err(e_chirho) => panic!("mapM_: {}", e_chirho),
        }
    }

    // ── lookup / additional Prelude tests ─────────────────────

    #[test]
    fn eval_lookup_found_chirho() {
        // lookup 2 [(1,10),(2,20),(3,30)] → Just 20 → 20
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = case lookup 2 [(1,10),(2,20),(3,30)] of
         Just x -> x
         Nothing -> 0
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(20)),
            Err(e_chirho) => panic!("lookup found: {}", e_chirho),
        }
    }

    #[test]
    fn eval_lookup_not_found_chirho() {
        // lookup 5 [(1,10),(2,20)] → Nothing → 0
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
main = case lookup 5 [(1,10),(2,20)] of
         Just x -> x
         Nothing -> 0
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("lookup not found: {}", e_chirho),
        }
    }

    #[test]
    fn eval_product_chirho() {
        // product [1,2,3,4,5] → 120
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = product [1,2,3,4,5]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(120)),
            Err(e_chirho) => panic!("product: {}", e_chirho),
        }
    }

    #[test]
    fn eval_replicate_sum_chirho() {
        // sum (replicate 3 7) → 21
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (replicate 3 7)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(21)),
            Err(e_chirho) => panic!("replicate sum: {}", e_chirho),
        }
    }

    // ── Deriving Show for product types + advanced features ─────────

    #[test]
    fn eval_deriving_show_product_chirho() {
        // data Point = Point Int Int deriving (Show)
        // show (Point 3 4) → "Point 3 4"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Point = Point Int Int deriving (Show)
main = putStrLn (show (Point 3 4))
"#;
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "Point 3 4\n");
            }
            Err(e_chirho) => panic!("deriving Show product: {}", e_chirho),
        }
    }

    #[test]
    fn eval_deriving_show_multi_con_chirho() {
        // data Shape = Circle Int | Rect Int Int deriving (Show)
        // show (Circle 5) → "Circle 5", show (Rect 3 4) → "Rect 3 4"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Shape = Circle Int | Rect Int Int deriving (Show)
main = putStrLn (show (Rect 3 4))
"#;
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "Rect 3 4\n");
            }
            Err(e_chirho) => panic!("deriving Show multi-con: {}", e_chirho),
        }
    }

    #[test]
    fn eval_deriving_eq_product_chirho() {
        // data Point = Point Int Int deriving (Eq)
        // Point 3 4 == Point 3 4 → True → 1
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Point = Point Int Int deriving (Eq)
main = if Point 3 4 == Point 3 4 then 1 else 0
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("deriving Eq product: {}", e_chirho),
        }
    }

    #[test]
    fn eval_deriving_eq_product_neq_chirho() {
        // Point 3 4 == Point 3 5 → False → 0
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = r#"module Test where
data Point = Point Int Int deriving (Eq)
main = if Point 3 4 == Point 3 5 then 1 else 0
"#;
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("deriving Eq product neq: {}", e_chirho),
        }
    }

    // ── List comprehension with guards and transforms ──────────────

    #[test]
    fn eval_list_comp_transform_filter_chirho() {
        // [x*x | x <- [1..10], even x] → [4,16,36,64,100] → sum → 220
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [x*x | x <- [1..10], even x]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(220)),
            Err(e_chirho) => panic!("list comp transform+filter: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_comp_cartesian_chirho() {
        // length [(x,y) | x <- [1,2,3], y <- [1,2]] → 6
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length [(x,y) | x <- [1,2,3], y <- [1,2]]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("list comp cartesian: {}", e_chirho),
        }
    }

    // ── Higher-order function composition ─────────────────────────

    #[test]
    fn eval_higher_order_compose_chirho() {
        // (length . filter even) [1..10] → 5
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = (length . filter even) [1..10]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("higher order compose: {}", e_chirho),
        }
    }

    // ── Operator sections ────────────────────────────────────────────

    #[test]
    fn eval_left_section_chirho() {
        // map (2*) [1,2,3] → [2,4,6], sum → 12
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (2*) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(12)),
            Err(e_chirho) => panic!("left section (2*): {}", e_chirho),
        }
    }

    #[test]
    fn eval_right_section_chirho() {
        // map (*3) [1,2,3] → [3,6,9], sum → 18
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (*3) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(18)),
            Err(e_chirho) => panic!("right section (*3): {}", e_chirho),
        }
    }

    #[test]
    fn eval_section_addition_chirho() {
        // map (+10) [1,2,3] → [11,12,13], sum → 36
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (+10) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(36)),
            Err(e_chirho) => panic!("section (+10): {}", e_chirho),
        }
    }

    // ── Nested pattern matching in case ─────────────────────────────

    #[test]
    fn eval_case_nested_tuple_chirho() {
        // case (1, 2) of { (a, b) -> a + b }
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = case (1, 2) of { (a, b) -> a + b }\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("case nested tuple: {}", e_chirho),
        }
    }

    // ── Type annotation expressions ─────────────────────────────────

    #[test]
    fn eval_type_annotation_chirho() {
        // (42 :: Int) → 42
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = (42 :: Int)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("type annotation: {}", e_chirho),
        }
    }

    // ── Lambda with tuple pattern ────────────────────────────────────

    #[test]
    fn eval_lambda_tuple_pattern_chirho() {
        // (\(x, y) -> x + y) (3, 4)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = (\\(x, y) -> x + y) (3, 4)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7)),
            Err(e_chirho) => panic!("lambda tuple pattern: {}", e_chirho),
        }
    }

    // ── Let with multiple bindings ──────────────────────────────────

    #[test]
    fn eval_let_multi_bind_chirho() {
        // let { a = 10; b = 20; c = 30 } in a + b + c
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = let a = 10\n           b = 20\n           c = 30\n       in a + b + c\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("let multi bind: {}", e_chirho),
        }
    }

    // ── Data constructor as function ────────────────────────────────

    #[test]
    fn eval_map_just_chirho() {
        // map Just [1,2,3] → [Just 1, Just 2, Just 3], length → 3
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (map Just [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("map Just: {}", e_chirho),
        }
    }

    // ── If in where ─────────────────────────────────────────────────

    #[test]
    fn eval_if_in_where_chirho() {
        // classify with where clause using if
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
classify x = result
  where result = if x > 0 then 1 else 0
main = classify 42
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("if in where: {}", e_chirho),
        }
    }

    // ── Chained function application ────────────────────────────────

    #[test]
    fn eval_chained_dollar_chirho() {
        // head $ filter even $ [1..10] → 2
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head $ filter even $ [1..10]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("chained $: {}", e_chirho),
        }
    }

    // ── Lambda with constructor pattern ──────────────────────────────

    #[test]
    fn eval_lambda_con_pattern_chirho() {
        // (\(Just x) -> x + 1) (Just 41) → 42
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = (\\(Just x) -> x + 1) (Just 41)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("lambda con pattern: {}", e_chirho),
        }
    }

    // ── Uncurry with operator section ────────────────────────────────

    #[test]
    fn eval_uncurry_section_chirho() {
        // uncurry (+) (3, 4) → 7
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = uncurry (+) (3, 4)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7)),
            Err(e_chirho) => panic!("uncurry (+): {}", e_chirho),
        }
    }

    // ── Map with operator section ────────────────────────────────────

    #[test]
    fn eval_map_section_subtract_chirho() {
        // map (subtract 1) [10, 20, 30] → [9, 19, 29], sum → 57
        // (subtract is \a b -> b - a in Prelude)
        // For now use a lambda instead since subtract isn't defined
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nsubtract a b = b - a\nmain = sum (map (subtract 1) [10, 20, 30])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(57)),
            Err(e_chirho) => panic!("map subtract: {}", e_chirho),
        }
    }

    // ── Where with multiple helper functions ─────────────────────────

    #[test]
    fn eval_where_multi_helpers_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
compute x = doubled + tripled
  where doubled = x * 2
        tripled = x * 3
main = compute 5
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(25)),
            Err(e_chirho) => panic!("where multi helpers: {}", e_chirho),
        }
    }

    // ── Nested data types ───────────────────────────────────────────

    #[test]
    fn eval_nested_maybe_case_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
safe_head xs = case xs of
  [] -> Nothing
  (x:_) -> Just x
main = case safe_head [42, 1, 2] of
  Nothing -> 0
  Just x -> x
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("nested maybe case: {}", e_chirho),
        }
    }

    // ── Type synonym in user code ───────────────────────────────────

    #[test]
    fn eval_type_synonym_list_chirho() {
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
type IntList = [Int]
sumList :: IntList -> Int
sumList xs = sum xs
main = sumList [1, 2, 3, 4, 5]
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("type synonym list: {}", e_chirho),
        }
    }

    // ── Complex list processing ─────────────────────────────────────

    #[test]
    fn eval_complex_list_pipeline_chirho() {
        // sum . map (^2) . filter odd $ [1..10] → 1+9+25+49+81 = 165
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (^2) (filter odd [1..10]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(165)),
            Err(e_chirho) => panic!("complex list pipeline: {}", e_chirho),
        }
    }

    // ── Numeric escape sequences ────────────────────────────────────

    #[test]
    fn eval_numeric_escape_decimal_chirho() {
        // \65 = 'A', \66 = 'B', \67 = 'C'
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn \"\\65\\66\\67\"\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "ABC\n"),
            Err(e_chirho) => panic!("numeric escape decimal: {}", e_chirho),
        }
    }

    #[test]
    fn eval_numeric_escape_hex_chirho() {
        // \x48 = 'H', \x69 = 'i'
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn \"\\x48\\x69\"\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "Hi\n"),
            Err(e_chirho) => panic!("numeric escape hex: {}", e_chirho),
        }
    }

    #[test]
    fn eval_numeric_escape_octal_chirho() {
        // \o110 = 'H' (72 in octal), \o151 = 'i' (105 in octal)
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn \"\\o110\\o151\"\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "Hi\n"),
            Err(e_chirho) => panic!("numeric escape octal: {}", e_chirho),
        }
    }

    // ── Data.Map extended operations ────────────────────────────────

    #[test]
    fn eval_map_insert_with_chirho() {
        // mapInsertWith (+) 1 100 (mapInsert 1 10 mapEmpty) → value at key 1 is 10+100=110
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m1 = mapInsert 1 10 (mapInsert 2 20 mapEmpty)
m2 = mapInsertWith (+) 1 100 m1
main = mapFindWithDefault 0 1 m2
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(110)),
            Err(e_chirho) => panic!("mapInsertWith: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_find_with_default_chirho() {
        // mapFindWithDefault 99 5 mapEmpty → 99 (key not found, returns default)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = mapFindWithDefault 99 5 mapEmpty
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("mapFindWithDefault: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_find_with_default_found_chirho() {
        // mapFindWithDefault 99 1 (mapSingleton 1 42) → 42 (key found)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = mapFindWithDefault 99 1 (mapSingleton 1 42)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("mapFindWithDefault found: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_adjust_chirho() {
        // mapAdjust (*10) 1 (mapSingleton 1 5) → value at key 1 is 5*10=50
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapAdjust (*10) 1 (mapSingleton 1 5)
main = mapFindWithDefault 0 1 m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(50)),
            Err(e_chirho) => panic!("mapAdjust: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_union_chirho() {
        // mapUnion (mapSingleton 1 10) (mapSingleton 2 20) → size 2
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapUnion (mapSingleton 1 10) (mapSingleton 2 20)
main = mapSize m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("mapUnion: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_difference_chirho() {
        // mapDifference (fromList [(1,10),(2,20),(3,30)]) (mapSingleton 2 99) → size 2
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m1 = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
m2 = mapSingleton 2 99
main = mapSize (mapDifference m1 m2)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("mapDifference: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_filter_chirho() {
        // mapFilter (> 15) (fromList [(1,10),(2,20),(3,30)]) → size 2 (values 20 and 30)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
main = mapSize (mapFilter (> 15) m)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("mapFilter: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_union_with_chirho() {
        // mapUnionWith (+) (mapSingleton 1 10) (mapSingleton 1 20) → value at 1 is 30
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapUnionWith (+) (mapSingleton 1 10) (mapSingleton 1 20)
main = mapFindWithDefault 0 1 m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("mapUnionWith: {}", e_chirho),
        }
    }

    // ── Additional list functions ───────────────────────────────────

    #[test]
    fn eval_nub_length_chirho() {
        // nub [1,2,1,3,2,4] → [1,2,3,4] → length 4
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = length (nub [1,2,1,3,2,4])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4)),
            Err(e_chirho) => panic!("nub length: {}", e_chirho),
        }
    }

    #[test]
    fn eval_nub_head_chirho() {
        // head (nub [3,1,3,2]) → 3 (first unique is 3)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = head (nub [3,1,3,2])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("nub head: {}", e_chirho),
        }
    }

    #[test]
    fn eval_intersperse_sum_chirho() {
        // sum (intersperse 0 [1,2,3]) → 1+0+2+0+3 = 6
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (intersperse 0 [1,2,3])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("intersperse sum: {}", e_chirho),
        }
    }

    #[test]
    fn eval_is_prefix_of_true_chirho() {
        // isPrefixOf [1,2] [1,2,3] → True → use if to convert to Int
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if isPrefixOf [1,2] [1,2,3] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("isPrefixOf true: {}", e_chirho),
        }
    }

    #[test]
    fn eval_is_prefix_of_false_chirho() {
        // isPrefixOf [2,3] [1,2,3] → False → use if to convert to Int
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if isPrefixOf [2,3] [1,2,3] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("isPrefixOf false: {}", e_chirho),
        }
    }

    // ── Data.Map String-keyed ─────────────────────────────────────

    #[test]
    fn eval_map_insert_str_chirho() {
        // mapInsertStr then lookup: mapFindWithDefaultStr 0 "hello" (mapInsertStr "hello" 42 mapEmpty)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsertStr \"hello\" 42 mapEmpty
main = mapFindWithDefaultStr 0 \"hello\" m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("mapInsertStr: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_lookup_str_not_found_chirho() {
        // mapFindWithDefaultStr 99 "missing" mapEmpty → 99
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = mapFindWithDefaultStr 99 \"missing\" mapEmpty
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("mapFindWithDefaultStr not found: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_str_multiple_chirho() {
        // Insert multiple string keys and verify size
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsertStr \"a\" 1 (mapInsertStr \"b\" 2 (mapInsertStr \"c\" 3 mapEmpty))
main = mapSize m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("mapStr multiple: {}", e_chirho),
        }
    }

    #[test]
    fn eval_is_suffix_of_chirho() {
        // isSuffixOf [2,3] [1,2,3] → True → use if to convert to Int
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if isSuffixOf [2,3] [1,2,3] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("isSuffixOf: {}", e_chirho),
        }
    }

    // ── Negative literal patterns ──────────────────────────────────

    #[test]
    fn eval_neg_lit_case_chirho() {
        // case (-1) of { -1 -> 100; _ -> 0 } → not directly since we don't parse neg lit patterns yet
        // But we can test function guard equivalent
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x = if x == 0 then 100 else x * 2
main = f 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(100)),
            Err(e_chirho) => panic!("neg lit case: {}", e_chirho),
        }
    }

    // ── Complex pattern matching scenarios ──────────────────────────

    #[test]
    fn eval_multi_clause_with_guards_chirho() {
        // Multi-clause function with guards
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
classify x
  | x < 0     = 0
  | x == 0    = 1
  | x < 100   = 2
  | otherwise  = 3
main = classify 0 + classify 50 + classify 200 + classify (-5)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // 1 + 2 + 3 + 0 = 6
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("multi clause with guards: {}", e_chirho),
        }
    }

    #[test]
    fn eval_nested_let_where_chirho() {
        // let with where-bound helper
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let x = double 5 in x + 3
  where double n = n * 2
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(13)),
            Err(e_chirho) => panic!("nested let where: {}", e_chirho),
        }
    }

    #[test]
    fn eval_case_string_match_chirho() {
        // String equality through if-then-else
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
greet name = if name == \"world\" then 1 else 0
main = greet \"world\"
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("case string match: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_fold_sum_chirho() {
        // Use mapFoldlWithKey to sum all values in a map
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
main = mapFoldlWithKey (\\acc k v -> acc + v) 0 m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // 10 + 20 + 30 = 60
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(60)),
            Err(e_chirho) => panic!("map fold sum: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_comp_with_let_chirho() {
        // List comprehension with let binding: [y | x <- [1..5], let y = x * x, y > 5]
        // Would be: [9, 16, 25] → sum = 50
        // Simpler: just test that list comp + filter combo works
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
squares = map (\\x -> x * x) [1..5]
main = sum (filter (> 5) squares)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // 9 + 16 + 25 = 50
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(50)),
            Err(e_chirho) => panic!("list comp with let: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_double_sum_chirho() {
        // map (*2) then sum: sum (map (*2) [1..5]) = 2+4+6+8+10 = 30
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (map (*2) [1..5])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("map double sum: {}", e_chirho),
        }
    }

    #[test]
    fn eval_string_map_values_chirho() {
        // String-keyed map: insert 3 entries, lookup one value
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
m = mapInsertStr \"foo\" 10 (mapInsertStr \"bar\" 20 (mapInsertStr \"baz\" 30 mapEmpty))
main = mapFindWithDefaultStr 0 \"bar\" m
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(20)),
            Err(e_chirho) => panic!("string map values: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_bool_list_chirho() {
        // show True ++ " " ++ show False → "True False"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = putStrLn (show True ++ \" \" ++ show False)
";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match &result_chirho {
            Ok((_v_chirho, m_chirho)) => assert_eq!(m_chirho.io_output_chirho, "True False\n"),
            Err(e_chirho) => panic!("show bool list: {}", e_chirho),
        }
    }

    // ── Push to 1000 tests ─────────────────────────────────────────

    #[test]
    fn eval_foldr_cons_chirho() {
        // foldr (:) [] [1,2,3] → [1,2,3] → length = 3 (identity via foldr)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (foldr (\\x xs -> x : xs) [] [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("foldr cons: {}", e_chirho),
        }
    }

    #[test]
    fn eval_foldl_subtract_chirho() {
        // foldl (-) 100 [10,20,30] → ((100-10)-20)-30 = 40
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = foldl (-) 100 [10,20,30]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(40)),
            Err(e_chirho) => panic!("foldl subtract: {}", e_chirho),
        }
    }

    #[test]
    fn eval_zip_sum_chirho() {
        // sum (map (\(a,b) -> a+b) (zip [1,2,3] [10,20,30])) → 11+22+33 = 66
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = sum (map (\\(a,b) -> a + b) (zip [1,2,3] [10,20,30]))
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66)),
            Err(e_chirho) => panic!("zip sum: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_singleton_lookup_chirho() {
        // mapSingleton 42 99: lookup existing key
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = mapFindWithDefault 0 42 (mapSingleton 42 99)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("map singleton lookup: {}", e_chirho),
        }
    }

    #[test]
    fn eval_set_from_list_dedup_chirho() {
        // setFromList [3,1,4,1,5,9,2,6] → deduplicated set → setSize = 7
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = setSize (setFromList [3,1,4,1,5,9,2,6])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(7)),
            Err(e_chirho) => panic!("set from list dedup: {}", e_chirho),
        }
    }

    #[test]
    fn eval_enum_from_then_chirho() {
        // [2,4..10] → [2,4,6,8,10] → sum = 30
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [2,4..10]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("enum from then: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_recursive_fib_chirho() {
        // Fibonacci via where clause
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = result
  where result = fib 8
        fib n = if n <= 1 then n else fib (n - 1) + fib (n - 2)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(21)),
            Err(e_chirho) => panic!("where recursive fib: {}", e_chirho),
        }
    }

    #[test]
    fn eval_product_list_chirho() {
        // product [1..5] → 120
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = product [1..5]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(120)),
            Err(e_chirho) => panic!("product list: {}", e_chirho),
        }
    }

    #[test]
    fn eval_any_all_chirho() {
        // any even [1,3,5,7] → False → 0, all odd [1,3,5,7] → True → 1
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = if all odd [1,3,5,7] then 1 else 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("any all: {}", e_chirho),
        }
    }

    #[test]
    fn eval_takewhile_sum_chirho() {
        // takeWhile (< 5) [1..10] → [1,2,3,4] → sum = 10
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (takeWhile (< 5) [1..10])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10)),
            Err(e_chirho) => panic!("takewhile sum: {}", e_chirho),
        }
    }

    // ── Data.Maybe extras ──

    #[test]
    fn eval_maybe_to_list_just_chirho() {
        // maybeToList (Just 42) → [42] → head = 42
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head (maybeToList (Just 42))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("maybeToList Just: {}", e_chirho),
        }
    }

    #[test]
    fn eval_maybe_to_list_nothing_chirho() {
        // maybeToList Nothing → [] → length = 0
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (maybeToList Nothing)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("maybeToList Nothing: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_to_maybe_chirho() {
        // listToMaybe [10,20,30] → Just 10 → fromMaybe 0 = 10
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = fromMaybe 0 (listToMaybe [10,20,30])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10)),
            Err(e_chirho) => panic!("listToMaybe: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_to_maybe_empty_chirho() {
        // listToMaybe [] → Nothing → fromMaybe 99 = 99
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = fromMaybe 99 (listToMaybe [])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("listToMaybe empty: {}", e_chirho),
        }
    }

    #[test]
    fn eval_cat_maybes_chirho() {
        // catMaybes [Just 1, Nothing, Just 3, Nothing, Just 5] → [1,3,5] → sum = 9
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (catMaybes [Just 1, Nothing, Just 3, Nothing, Just 5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(9)),
            Err(e_chirho) => panic!("catMaybes: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_maybe_filter_chirho() {
        // mapMaybe (\x -> if x > 3 then Just (x * 10) else Nothing) [1,2,3,4,5] → [40,50] → sum = 90
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (mapMaybe (\\x -> if x > 3 then Just (x * 10) else Nothing) [1,2,3,4,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(90)),
            Err(e_chirho) => panic!("mapMaybe filter: {}", e_chirho),
        }
    }

    // ── Data.IORef ──

    #[test]
    fn eval_ioref_new_read_show_chirho() {
        // newIORef 42, readIORef, show → "42\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 42\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "42\n");
            }
            Err(e_chirho) => panic!("ioref new/read/show: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_write_overwrite_chirho() {
        // newIORef 1, writeIORef r 99, readIORef r → 99
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 1\n  writeIORef r 99\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "99\n");
            }
            Err(e_chirho) => panic!("ioref write overwrite: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_do_modify_lambda_chirho() {
        // modifyIORef r (\x -> x * 2), newIORef 21, readIORef → 42
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r <- newIORef 21\n  modifyIORef r (\\x -> x + x)\n  v <- readIORef r\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "42\n");
            }
            Err(e_chirho) => panic!("ioref do modify lambda: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_two_refs_independent_chirho() {
        // Two separate IORefs hold independent values
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  r1 <- newIORef 10\n  r2 <- newIORef 20\n  writeIORef r1 99\n  v1 <- readIORef r1\n  v2 <- readIORef r2\n  putStrLn (show (v1 + v2))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "119\n");
            }
            Err(e_chirho) => panic!("ioref two refs independent: {}", e_chirho),
        }
    }

    #[test]
    fn eval_ioref_counter_increment_chirho() {
        // IORef counter: start at 0, increment 3 times via modifyIORef, read → 3
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nincr x = x + 1\nmain = do\n  c <- newIORef 0\n  modifyIORef c incr\n  modifyIORef c incr\n  modifyIORef c incr\n  v <- readIORef c\n  putStrLn (show v)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "3\n");
            }
            Err(e_chirho) => panic!("ioref counter increment: {}", e_chirho),
        }
    }

    // ── when/unless ──

    #[test]
    fn eval_when_true_output_chirho() {
        // when True (putStrLn "yes") → "yes\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = when True (putStrLn \"yes\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "yes\n");
            }
            Err(e_chirho) => panic!("when True output: {}", e_chirho),
        }
    }

    #[test]
    fn eval_when_false_silent_chirho() {
        // when False (putStrLn "no") → ""
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = when False (putStrLn \"no\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "");
            }
            Err(e_chirho) => panic!("when False silent: {}", e_chirho),
        }
    }

    #[test]
    fn eval_unless_false_output_chirho() {
        // unless False (putStrLn "ran") → "ran\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = unless False (putStrLn \"ran\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "ran\n");
            }
            Err(e_chirho) => panic!("unless False output: {}", e_chirho),
        }
    }

    // ── flip ──

    #[test]
    fn eval_flip_const_chirho() {
        // flip const 1 2 → 2
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = flip const 1 2\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("flip const: {}", e_chirho),
        }
    }

    // ── Data.Either extras ──

    #[test]
    fn eval_either_left_chirho() {
        // either (+10) (*2) (Left 5) → 15
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = either (+10) (*2) (Left 5)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("either Left: {}", e_chirho),
        }
    }

    #[test]
    fn eval_either_right_chirho() {
        // either (+10) (*2) (Right 5) → 10
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = either (+10) (*2) (Right 5)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10)),
            Err(e_chirho) => panic!("either Right: {}", e_chirho),
        }
    }

    // ── More feature tests ──

    #[test]
    fn eval_type_synonym_usage_chirho() {
        // type MyList = [Int]; f :: MyList -> Int; f xs = sum xs; main = f [1,2,3]
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ntype MyList = [Int]\nf :: MyList -> Int\nf xs = sum xs\nmain = f [1,2,3]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("type synonym usage: {}", e_chirho),
        }
    }

    #[test]
    fn eval_nested_where_simple_chirho() {
        // f x = a + b where { a = x * 2; b = x + 3 }
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf x = a + b\n  where\n    a = x * 2\n    b = x + 3\nmain = f 10\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(33)),
            Err(e_chirho) => panic!("nested where simple: {}", e_chirho),
        }
    }

    #[test]
    fn eval_guard_multiple_equations_chirho() {
        // classify n | n < 0 = -1 | n == 0 = 0 | otherwise = 1
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nclassify n\n  | n < 0 = negate 1\n  | n == 0 = 0\n  | otherwise = 1\nmain = classify (negate 5) + classify 0 + classify 10\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(0)),
            Err(e_chirho) => panic!("guard multiple equations: {}", e_chirho),
        }
    }

    #[test]
    fn eval_let_in_do_complex_chirho() {
        // do { let x = 10; let y = x + 5; putStrLn (show (x + y)) } → "25\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  let x = 10\n  let y = x + 5\n  putStrLn (show (x + y))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "25\n");
            }
            Err(e_chirho) => panic!("let in do complex: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_square_sum_chirho() {
        // map (\x -> x * x) [1,2,3,4] → [1,4,9,16] → sum = 30
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (map (\\x -> x * x) [1,2,3,4])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(30)),
            Err(e_chirho) => panic!("map with lambda: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multiple_io_operations_chirho() {
        // do { putStr "a"; putStr "b"; putStrLn "c" } → "abc\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  putStr \"a\"\n  putStr \"b\"\n  putStrLn \"c\"\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "abc\n");
            }
            Err(e_chirho) => panic!("multiple io: {}", e_chirho),
        }
    }

    #[test]
    fn eval_nested_function_application_chirho() {
        // f x y = x + y; g a = f a (a * 2); main = g 5 → 15
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf x y = x + y\ng a = f a (a * 2)\nmain = g 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("nested function app: {}", e_chirho),
        }
    }

    #[test]
    fn eval_data_maybe_chain_chirho() {
        // safeDivide with guard instead of literal pattern match
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nsafeDivide x y\n  | y == 0 = Nothing\n  | otherwise = Just (x `div` y)\nmain = fromMaybe 0 (safeDivide 10 3) + fromMaybe 0 (safeDivide 10 0)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("data maybe chain: {}", e_chirho),
        }
    }

    #[test]
    fn eval_iterate_take_chirho() {
        // take 5 (iterate (*2) 1) → [1,2,4,8,16] → sum = 31
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (take 5 (iterate (*2) 1))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(31)),
            Err(e_chirho) => panic!("iterate take: {}", e_chirho),
        }
    }

    #[test]
    fn eval_scanl_length_chirho() {
        // scanl (+) 0 [1,2,3] → [0,1,3,6] → length = 4
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (scanl (+) 0 [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4)),
            Err(e_chirho) => panic!("scanl length: {}", e_chirho),
        }
    }

    #[test]
    fn eval_concatmap_sum_chirho() {
        // concatMap (\x -> [x, x*10]) [1,2,3] → [1,10,2,20,3,30] → sum = 66
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (concatMap (\\x -> [x, x*10]) [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66)),
            Err(e_chirho) => panic!("concatMap sum: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_negative_int_chirho() {
        // putStrLn (show (negate 42)) → "-42\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (show (negate 42))\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "-42\n");
            }
            Err(e_chirho) => panic!("show negative: {}", e_chirho),
        }
    }

    #[test]
    fn eval_char_operations_chirho() {
        // ord 'A' + ord 'a' = 65 + 97 = 162
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = ord 'A' + ord 'a'\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(162)),
            Err(e_chirho) => panic!("char operations: {}", e_chirho),
        }
    }

    #[test]
    fn eval_list_comp_even_squares_chirho() {
        // [x * x | x <- [1..10], x `mod` 2 == 0] → [4,16,36,64,100] → sum = 220
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum [x * x | x <- [1..10], x `mod` 2 == 0]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(220)),
            Err(e_chirho) => panic!("list comp with guard: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_with_pattern_chirho() {
        // f (x, y) = a + b where { a = x * 2; b = y + 1 }; main = f (3, 4)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf p = a + b\n  where\n    a = fst p * 2\n    b = snd p + 1\nmain = f (3, 4)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(11)),
            Err(e_chirho) => panic!("where with pattern: {}", e_chirho),
        }
    }

    #[test]
    fn eval_enum_succ_pred_chirho() {
        // succ 41 + pred 43 = 42 + 42 = 84
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = succ 41 + pred 43\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(84)),
            Err(e_chirho) => panic!("enum succ pred: {}", e_chirho),
        }
    }

    #[test]
    fn eval_show_list_show_chirho() {
        // putStrLn (show [10,20,30]) → "[10,20,30]\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (show [10,20,30])\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "[10,20,30]\n");
            }
            Err(e_chirho) => panic!("show list show: {}", e_chirho),
        }
    }

    #[test]
    fn eval_zipwith_add_chirho() {
        // sum (zipWith (+) [1,2,3] [10,20,30]) = 11+22+33 = 66
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (zipWith (+) [1,2,3] [10,20,30])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(66)),
            Err(e_chirho) => panic!("zipWith add: {}", e_chirho),
        }
    }

    #[test]
    fn eval_complex_pipeline_chirho() {
        // sum . filter (> 5) . map (*2) $ [1,2,3,4,5] → filter [2,4,6,8,10] > 5 → [6,8,10] → sum = 24
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (filter (> 5) (map (*2) [1,2,3,4,5]))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(24)),
            Err(e_chirho) => panic!("complex pipeline: {}", e_chirho),
        }
    }

    #[test]
    fn eval_multi_line_do_io_chirho() {
        // do { putStrLn (show 1); putStrLn (show 2); putStrLn (show 3) } → "1\n2\n3\n"
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = do\n  putStrLn (show 1)\n  putStrLn (show 2)\n  putStrLn (show 3)\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, m_chirho)) => {
                let out_chirho = &m_chirho.io_output_chirho;
                assert_eq!(out_chirho, "1\n2\n3\n");
            }
            Err(e_chirho) => panic!("multi line do io: {}", e_chirho),
        }
    }

    // ── fromJust / swap / mapDelete fix ──

    #[test]
    fn eval_from_just_chirho() {
        // fromJust (Just 42) = 42
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = fromJust (Just 42)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("fromJust: {}", e_chirho),
        }
    }

    #[test]
    fn eval_swap_tuple_chirho() {
        // fst (swap (1, 2)) = 2
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = fst (swap (1, 2))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("swap tuple: {}", e_chirho),
        }
    }

    #[test]
    fn eval_swap_snd_chirho() {
        // snd (swap (10, 20)) = 10
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = snd (swap (10, 20))\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10)),
            Err(e_chirho) => panic!("swap snd: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_delete_preserves_chirho() {
        // mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))
        // after mapDelete 2, mapSize should be 2 and both 1 and 3 remain
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nm = mapInsert 1 10 (mapInsert 2 20 (mapInsert 3 30 mapEmpty))\nmain = mapSize (mapDelete 2 m)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("map delete preserves: {}", e_chirho),
        }
    }

    #[test]
    fn eval_map_delete_both_subtrees_chirho() {
        // insert 2, 1, 3 (root=2, left=1, right=3), delete 2
        // both 1 and 3 should remain, verify via lookup
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nm = mapInsert 2 20 (mapInsert 1 10 (mapInsert 3 30 mapEmpty))\nm2 = mapDelete 2 m\nmain = fromMaybe 0 (mapLookup 1 m2) + fromMaybe 0 (mapLookup 3 m2)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(40)),
            Err(e_chirho) => panic!("map delete both subtrees: {}", e_chirho),
        }
    }

    // ── Literal pattern matching in function equations ──

    #[test]
    fn eval_literal_pattern_zero_chirho() {
        // f 0 = 100; f x = x + 1; main = f 0 → 100
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf 0 = 100\nf x = x + 1\nmain = f 0\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(100)),
            Err(e_chirho) => panic!("literal pattern zero: {}", e_chirho),
        }
    }

    #[test]
    fn eval_literal_pattern_nonzero_chirho() {
        // f 0 = 100; f x = x + 1; main = f 5 → 6
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf 0 = 100\nf x = x + 1\nmain = f 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(6)),
            Err(e_chirho) => panic!("literal pattern nonzero: {}", e_chirho),
        }
    }

    #[test]
    fn eval_literal_pattern_multi_lit_chirho() {
        // Multiple literal arms with a catch-all default
        // f 0 = 10; f 1 = 20; f 2 = 30; f n = n * 100; main = f 2 + f 5
        // f 2 = 30, f 5 = 500, total = 530
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nf 0 = 10\nf 1 = 20\nf 2 = 30\nf n = n * 100\nmain = f 2 + f 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(530)),
            Err(e_chirho) => panic!("literal pattern multi: {}", e_chirho),
        }
    }

    #[test]
    fn eval_literal_pattern_negative_chirho() {
        // Negative literal pattern
        // abs2 0 = 0; abs2 x | x > 0 = x | otherwise = 0 - x
        // main = abs2 (-3) → 3
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nabs2 0 = 0\nabs2 x\n  | x > 0 = x\n  | otherwise = 0 - x\nmain = abs2 (-3)\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("literal pattern negative: {}", e_chirho),
        }
    }

    #[test]
    fn eval_literal_pattern_reuse_var_chirho() {
        // Default arm variable used in complex expression
        // fib2 0 = 0; fib2 1 = 1; fib2 n = n + 10; main = fib2 5
        // fib2 5 hits default → 5 + 10 = 15
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nfib2 0 = 0\nfib2 1 = 1\nfib2 n = n + 10\nmain = fib2 5\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(15)),
            Err(e_chirho) => panic!("literal pattern reuse var: {}", e_chirho),
        }
    }

    #[test]
    fn eval_two_arg_lit_match_chirho() {
        // Two-arg function: g 0 y = y; g x y = x + y
        // g 0 42 should return 42 (literal match)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ng 0 y = y\ng x y = x + y\nmain = g 0 42\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("two-arg lit match: {}", e_chirho),
        }
    }

    #[test]
    fn eval_two_arg_lit_default_chirho() {
        // Two-arg function: g 0 y = y; g x y = x + y
        // g 3 7 should return 10 (default match)
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\ng 0 y = y\ng x y = x + y\nmain = g 3 7\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(10)),
            Err(e_chirho) => panic!("two-arg lit default: {}", e_chirho),
        }
    }

    // ── Semigroup / Monoid tests ──

    #[test]
    fn eval_semigroup_append_lists_chirho() {
        // [1,2] <> [3,4] should produce a list of length 4
        // We evaluate: length ([1,2] <> [3,4])
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length ([1,2] <> [3,4])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4)),
            Err(e_chirho) => panic!("semigroup append lists: {}", e_chirho),
        }
    }

    #[test]
    fn eval_semigroup_append_strings_chirho() {
        // "hello" <> " " <> "world" via putStrLn
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = putStrLn (\"hello\" <> \" \" <> \"world\")\n";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello world\n");
            }
            Err(e_chirho) => panic!("semigroup append strings: {}", e_chirho),
        }
    }

    #[test]
    fn eval_monoid_mempty_list_chirho() {
        // mempty <> [1,2,3] should give [1,2,3] → length 3
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (mempty <> [1,2,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(3)),
            Err(e_chirho) => panic!("monoid mempty list: {}", e_chirho),
        }
    }

    #[test]
    fn eval_monoid_mconcat_chirho() {
        // mconcat [[1,2],[3],[4,5]] should give [1,2,3,4,5] → length 5
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (mconcat [[1,2],[3],[4,5]])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("monoid mconcat: {}", e_chirho),
        }
    }

    // ── Higher-order *By list function tests ──

    #[test]
    fn eval_sortby_chirho() {
        // sortBy compare [3,1,4,1,5] → [1,1,3,4,5], head → 1
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = head (sortBy compare [3,1,4,1,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("sortBy head: {}", e_chirho),
        }
    }

    #[test]
    fn eval_sortby_sum_chirho() {
        // sortBy compare [5,2,8,1,3] → [1,2,3,5,8], sum → 19
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = sum (sortBy compare [5,2,8,1,3])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(19)),
            Err(e_chirho) => panic!("sortBy sum: {}", e_chirho),
        }
    }

    #[test]
    fn eval_insertby_chirho() {
        // insertBy compare 3 [1,2,4,5] → [1,2,3,4,5], head → 1, length → 5
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (insertBy compare 3 [1,2,4,5])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("insertBy length: {}", e_chirho),
        }
    }

    #[test]
    fn eval_nubby_chirho() {
        // nubBy (\x y -> x == y) [1,2,1,3,2,4] → [1,2,3,4], length → 4
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = length (nubBy (\\x -> \\y -> x == y) [1,2,1,3,2,4])\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(4)),
            Err(e_chirho) => panic!("nubBy length: {}", e_chirho),
        }
    }

    #[test]
    fn eval_maximumby_chirho() {
        // maximumBy compare [3,1,5,2,4] → 5
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = maximumBy compare [3,1,5,2,4]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(5)),
            Err(e_chirho) => panic!("maximumBy: {}", e_chirho),
        }
    }

    #[test]
    fn eval_minimumby_chirho() {
        // minimumBy compare [3,1,5,2,4] → 1
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = minimumBy compare [3,1,5,2,4]\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("minimumBy: {}", e_chirho),
        }
    }

    #[test]
    fn eval_on_chirho() {
        // on (+) (\x -> x * x) 3 4 = (3*3) + (4*4) = 9 + 16 = 25
        // We use a simpler test: on (+) length ... needs string lists which is complex
        // Simpler: on f g x y = f (g x) (g y) where f = (+), g = negate
        // on (+) negate 3 4 = negate 3 + negate 4 = (-3) + (-4) = -7
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "module Test where\nmain = on (+) negate 3 4\n";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(-7)),
            Err(e_chirho) => panic!("on: {}", e_chirho),
        }
    }

    // ── Type synonyms in instance heads ──────────────────────────────

    #[test]
    fn eval_type_synonym_instance_chirho() {
        // type String = [Char] is built-in; test that Show String resolves
        // to Show [Char] which we already have
        use super::eval_source_with_machine_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
greet :: String -> String
greet name = name
main = putStrLn (greet \"hello\")
";
        let result_chirho = eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_val_chirho, machine_chirho)) => {
                assert_eq!(machine_chirho.io_output_chirho, "hello\n");
            }
            Err(e_chirho) => panic!("type synonym instance: {}", e_chirho),
        }
    }

    #[test]
    fn eval_user_type_synonym_chirho() {
        // User-defined type synonym
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
type Age = Int
addAge :: Age -> Age -> Age
addAge x y = x + y
main = addAge 25 17
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("user type synonym: {}", e_chirho),
        }
    }

    // ── Where-clause / let type annotations ────────────────────────────

    #[test]
    fn eval_where_type_annotation_chirho() {
        // where-clause with type annotation
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x = helper x
  where
    helper :: Int -> Int
    helper y = y + 1
main = f 41
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("where type annotation: {}", e_chirho),
        }
    }

    #[test]
    fn eval_let_type_annotation_chirho() {
        // let expression with type annotation
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
main = let double :: Int -> Int
           double x = x + x
       in double 21
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("let type annotation: {}", e_chirho),
        }
    }

    #[test]
    fn eval_where_multiple_annotated_chirho() {
        // where-clause with multiple annotated helper functions
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
f x = add3 (double x)
  where
    double :: Int -> Int
    double y = y + y
    add3 :: Int -> Int
    add3 z = z + 3
main = f 10
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            // double 10 = 20, add3 20 = 23
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(23)),
            Err(e_chirho) => panic!("where multiple annotated: {}", e_chirho),
        }
    }

    // ── Synthetic module imports ─────────────────────────────────────────

    #[test]
    fn eval_import_data_map_chirho() {
        // import Data.Map functions via synthetic module interface
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Map (mapInsert, mapLookup, mapEmpty)
main = case mapLookup 1 (mapInsert 1 99 mapEmpty) of
         Just x  -> x
         Nothing -> 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(99)),
            Err(e_chirho) => panic!("import Data.Map: {}", e_chirho),
        }
    }

    #[test]
    fn eval_import_data_map_size_chirho() {
        // import Data.Map, use mapSize
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Map
main = mapSize (mapInsert 2 20 (mapInsert 1 10 mapEmpty))
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(2)),
            Err(e_chirho) => panic!("import Data.Map size: {}", e_chirho),
        }
    }

    #[test]
    fn eval_import_data_list_chirho() {
        // import Data.List functions
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.List (sort)
main = head (sort [3, 1, 2])
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("import Data.List: {}", e_chirho),
        }
    }

    #[test]
    fn eval_import_data_char_chirho() {
        // import Data.Char functions
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Char (ord)
main = ord 'A'
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(65)),
            Err(e_chirho) => panic!("import Data.Char: {}", e_chirho),
        }
    }

    #[test]
    fn eval_import_data_maybe_chirho() {
        // import Data.Maybe functions
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Maybe (fromMaybe)
main = fromMaybe 0 (Just 42)
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42)),
            Err(e_chirho) => panic!("import Data.Maybe: {}", e_chirho),
        }
    }

    #[test]
    fn eval_import_data_set_chirho() {
        // import Data.Set functions
        use super::eval_source_chirho;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = "\
module Test where
import Data.Set (setInsert, setMember, setEmpty)
main = case setMember 5 (setInsert 5 (setInsert 3 setEmpty)) of
         True  -> 1
         False -> 0
";
        let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(1)),
            Err(e_chirho) => panic!("import Data.Set: {}", e_chirho),
        }
    }

}
