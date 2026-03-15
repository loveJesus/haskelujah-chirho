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
use rhasky_naming_chirho::resolve_chirho::resolve_module_with_imports_chirho;
use rhasky_naming_chirho::iface_chirho::{build_iface_with_imports_chirho, ModuleIfaceChirho};
use rhasky_typing_chirho::infer_chirho::{
    InferResultChirho, infer_module_chirho, infer_module_with_imports_chirho,
};
use rhasky_parser_chirho::cst_parser_chirho::ParserChirho;
use rhasky_parser_chirho::lower_chirho::lower_module_chirho;
use rhasky_runtime_chirho::{ExecutionModeChirho, RuntimePlanChirho};
use rhasky_span_chirho::SourceMapChirho;
use rhasky_syntax_chirho::SourceFileChirho;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendPlanChirho {
    pub llvm_preview_chirho: String,
    pub wasm_stub_size_chirho: usize,
}

#[derive(Debug, Clone)]
pub struct CheckSummaryChirho {
    pub source_path_chirho: PathBuf,
    pub module_name_chirho: String,
    pub runtime_plan_chirho: RuntimePlanChirho,
    pub backend_plan_chirho: BackendPlanChirho,
    /// Non-fatal warnings collected from the pipeline (deriving, exhaustiveness).
    pub warnings_chirho: Vec<String>,
}

// ---------------------------------------------------------------------------
// Shared front-end result and runner
// ---------------------------------------------------------------------------

/// Holds the results of the shared front-end compiler phases (1 through 4.5):
/// CST parse → AST lower → deriving → name resolve → kind infer → type infer
/// → exhaustiveness check.
///
/// Downstream pipeline stages (desugar, dict pass, backends) consume this
/// together with the original AST module.
pub struct FrontendResultChirho {
    /// The fully lowered and derived AST module.
    pub module_chirho: ModuleChirho,
    /// Results of type inference, including the type environment and class
    /// environment needed by the dictionary-passing transform.
    pub infer_result_chirho: InferResultChirho,
    /// Non-fatal warnings collected from deriving and exhaustiveness checking.
    pub warnings_chirho: Vec<String>,
}

/// Run the shared front-end compiler phases for a single Haskell module.
///
/// Executes phases 1 through 4.5 in order:
/// 1. CST parse (lex + layout + recursive-descent)
/// 2. CST → AST lowering
/// 2.5. Deriving (generate instance declarations)
/// 3. Name resolution (against `ifaces_chirho`)
/// 3.5. Kind inference
/// 4. Type inference (with `imported_types_chirho` if non-empty)
/// 4.5. Pattern match exhaustiveness and redundancy checking
///
/// Returns a [`FrontendResultChirho`] on success, or a
/// [`DiagnosticBundleChirho`] containing the first fatal error.
pub fn run_frontend_chirho(
    source_chirho: &str,
    file_id_chirho: rhasky_span_chirho::FileIdChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    imported_types_chirho: &std::collections::HashMap<String, rhasky_typing_chirho::SchemeChirho>,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    // Phase 1: CST parse (lex + layout + recursive-descent)
    let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();

    // Phase 2: CST → AST lowering
    let mut module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

    // Phase 2.1: Automatic Prelude import
    // Every Haskell module implicitly imports Prelude unless:
    //   - {-# LANGUAGE NoImplicitPrelude #-} is present
    //   - The module already has an explicit `import Prelude`
    inject_prelude_import_chirho(&mut module_chirho);

    // Phase 2.5: Deriving — generate instance declarations for `deriving` clauses
    let deriving_warnings_chirho =
        rhasky_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);

    // Phase 3: Name resolution
    let resolve_result_chirho =
        resolve_module_with_imports_chirho(&module_chirho, ifaces_chirho);
    if resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(resolve_result_chirho.diagnostics_chirho);
    }

    // Phase 3.1: Orphan instance detection
    let orphan_warnings_chirho =
        rhasky_naming_chirho::check_orphan_instances_chirho(&module_chirho);

    // Phase 3.5: Kind inference
    let kind_result_chirho =
        rhasky_typing_chirho::infer_module_kinds_chirho(&module_chirho);
    if kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(kind_result_chirho.diagnostics_chirho);
    }

    // Phase 4: Type inference — use import-aware variant when upstream
    // type schemes are available, plain variant otherwise.
    let infer_result_chirho = if imported_types_chirho.is_empty() {
        infer_module_chirho(&module_chirho)
    } else {
        infer_module_with_imports_chirho(&module_chirho, imported_types_chirho)
    };
    if infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(infer_result_chirho.diagnostics_chirho);
    }

    // Phase 4.5: Pattern match exhaustiveness and redundancy checking
    let exhaust_result_chirho =
        rhasky_typing_chirho::check_module_exhaustiveness_chirho(&module_chirho);
    if exhaust_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(exhaust_result_chirho.diagnostics_chirho);
    }

    // Collect non-fatal warnings from exhaustiveness checking.
    let exhaust_warnings_chirho: Vec<String> = exhaust_result_chirho
        .diagnostics_chirho
        .diagnostics_chirho()
        .iter()
        .filter(|d_chirho| !d_chirho.is_error_chirho())
        .map(|d_chirho| d_chirho.to_string())
        .collect();

    // Collect orphan instance warnings as strings.
    let orphan_warning_strs_chirho: Vec<String> = orphan_warnings_chirho
        .diagnostics_chirho()
        .iter()
        .map(|d_chirho| d_chirho.to_string())
        .collect();

    // Merge deriving, exhaustiveness, and orphan warnings.
    let mut warnings_chirho = deriving_warnings_chirho;
    warnings_chirho.extend(exhaust_warnings_chirho);
    warnings_chirho.extend(orphan_warning_strs_chirho);

    Ok(FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        warnings_chirho,
    })
}

/// Inject an implicit `import Prelude` into a module unless:
/// - `{-# LANGUAGE NoImplicitPrelude #-}` is present, or
/// - the module already has an explicit `import Prelude`.
fn inject_prelude_import_chirho(module_chirho: &mut ModuleChirho) {
    // Check for NoImplicitPrelude extension
    if module_chirho.extensions_chirho.iter().any(|e_chirho| e_chirho == "NoImplicitPrelude") {
        return;
    }

    // Check if Prelude is already explicitly imported
    let has_prelude_import_chirho = module_chirho.imports_chirho.iter().any(|imp_chirho| {
        imp_chirho.module_chirho.text_chirho() == "Prelude"
    });
    if has_prelude_import_chirho {
        return;
    }

    // Inject implicit Prelude import (unqualified, import everything)
    module_chirho.imports_chirho.push(rhasky_ast_chirho::module_chirho::ImportDeclChirho {
        module_chirho: rhasky_ast_chirho::name_chirho::NameChirho::RawChirho(
            rhasky_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                "Prelude",
                rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
            ),
        ),
        qualified_chirho: false,
        alias_chirho: None,
        spec_chirho: None,
        span_chirho: rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
    });
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
    let source_path_chirho = source_file_chirho.path_chirho().to_path_buf();
    let source_chirho = source_file_chirho.contents_chirho().to_string();
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let builtin_ifaces_chirho = rhasky_naming_chirho::builtin_module_ifaces_chirho();
    let empty_imported_types_chirho = std::collections::HashMap::new();

    let frontend_result_chirho = run_frontend_chirho(
        &source_chirho,
        file_id_chirho,
        &builtin_ifaces_chirho,
        &empty_imported_types_chirho,
    )?;

    // Extract module name from the AST (produced by the real parser)
    let module_name_chirho = frontend_result_chirho
        .module_chirho
        .name_chirho
        .text_chirho()
        .to_string();
    let runtime_plan_chirho =
        RuntimePlanChirho::for_module_chirho(execution_mode_chirho, module_name_chirho.clone());

    // Skip backend generation — use lightweight stubs for the summary.
    let llvm_preview_chirho = compile_to_llvm_ir_stub_chirho(&module_name_chirho);
    let wasm_stub_size_chirho = compile_to_wasm_stub_chirho(&module_name_chirho).len();

    Ok(CheckSummaryChirho {
        source_path_chirho,
        module_name_chirho,
        runtime_plan_chirho,
        backend_plan_chirho: BackendPlanChirho {
            llvm_preview_chirho,
            wasm_stub_size_chirho,
        },
        warnings_chirho: frontend_result_chirho.warnings_chirho,
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

/// Run only the frontend pipeline and return non-fatal warnings.
/// Useful for testing diagnostic output (orphan instances, exhaustiveness, etc.)
pub fn frontend_warnings_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
) -> Result<Vec<String>, DiagnosticBundleChirho> {
    let source_file_chirho =
        SourceFileChirho::from_source_map_chirho(source_map_chirho, file_name_chirho, source_chirho);
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let builtin_ifaces_chirho = rhasky_naming_chirho::builtin_module_ifaces_chirho();
    let empty_imported_types_chirho = std::collections::HashMap::new();

    let frontend_result_chirho = run_frontend_chirho(
        source_chirho,
        file_id_chirho,
        &builtin_ifaces_chirho,
        &empty_imported_types_chirho,
    )?;

    Ok(frontend_result_chirho.warnings_chirho)
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

    let builtin_ifaces_chirho = rhasky_naming_chirho::builtin_module_ifaces_chirho();
    let empty_imported_types_chirho = std::collections::HashMap::new();

    let frontend_result_chirho = run_frontend_chirho(
        source_chirho,
        file_id_chirho,
        &builtin_ifaces_chirho,
        &empty_imported_types_chirho,
    )?;

    let FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        warnings_chirho: _warnings_chirho,
    } = frontend_result_chirho;

    compile_backend_chirho(module_chirho, infer_result_chirho)
}

/// Run the back-end pipeline phases (5 through 7) on an already-front-end-compiled
/// module, producing a [`CompileResultChirho`].
///
/// Phases: desugar → dict pass → simplify → LLVM IR → Wasm bytes.
fn compile_backend_chirho(
    module_chirho: ModuleChirho,
    infer_result_chirho: InferResultChirho,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
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

        // Phases 1–4.5: shared front-end
        let frontend_result_chirho = run_frontend_chirho(
            source_chirho,
            file_id_chirho,
            &ifaces_chirho,
            &imported_types_chirho,
        )?;

        let FrontendResultChirho {
            module_chirho,
            infer_result_chirho,
            warnings_chirho: _warnings_chirho,
        } = frontend_result_chirho;

        // Build interface for downstream modules before consuming module_chirho.
        // Use import-aware variant so `module Foo` re-exports work.
        let iface_chirho =
            build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

        // Extract type schemes for exported names and accumulate them
        // so downstream modules can type-check cross-module references.
        for (name_chirho, _val_chirho) in &iface_chirho.exports_chirho.values_chirho {
            if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(name_chirho) {
                imported_types_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
            }
        }
        for (_name_chirho, ty_info_chirho) in &iface_chirho.exports_chirho.types_chirho {
            // Export the type constructors' data constructor schemes
            for con_name_chirho in &ty_info_chirho.constructors_chirho {
                if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(con_name_chirho) {
                    imported_types_chirho.insert(con_name_chirho.clone(), scheme_chirho.clone());
                }
            }
        }

        ifaces_chirho.push(iface_chirho);

        // Phases 5–7: back-end
        let compile_result_chirho = compile_backend_chirho(module_chirho, infer_result_chirho)?;
        results_chirho.push(compile_result_chirho);
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
        CoreExprChirho,
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

/// Like `eval_source_with_machine_chirho` but with a custom step limit.
/// Used for algorithmic tests that require more than the default 100,000 steps.
pub fn eval_source_with_step_limit_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
    step_limit_chirho: u64,
) -> Result<(rhasky_runtime_chirho::ValueChirho, rhasky_runtime_chirho::eval_chirho::MachineChirho), String> {
    let compile_result_chirho = compile_source_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
    )
    .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    stg_lower_chirho::lower_and_run_with_step_limit_chirho(
        &compile_result_chirho.core_chirho,
        entry_name_chirho,
        compile_result_chirho.newtype_cons_chirho,
        step_limit_chirho,
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

        // Phases 1–4.5: shared front-end (same path for both recompile and cache-hit,
        // since Core/AST are not yet serialised to the artifact store).
        let empty_imported_types_chirho = std::collections::HashMap::new();
        let frontend_result_chirho = run_frontend_chirho(
            source_chirho,
            *file_id_chirho,
            &ifaces_chirho,
            &empty_imported_types_chirho,
        )?;
        let FrontendResultChirho {
            module_chirho,
            infer_result_chirho,
            warnings_chirho: _warnings_chirho,
        } = frontend_result_chirho;

        // Build and register the module interface for downstream modules.
        let iface_chirho =
            build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

        if recompiled_chirho {
            // Record compilation fingerprint for incremental tracking.
            let iface_fp_chirho =
                FingerprintChirho::from_str_chirho(&format!("{:?}", iface_chirho));
            let record_chirho =
                session_chirho.record_compilation_chirho(module_name_chirho, source_fp_chirho);
            record_chirho.set_phase_chirho(PhaseTagChirho::IfaceChirho, iface_fp_chirho);
        }
        // Cache-hit branch skips fingerprint bookkeeping but still pushes the
        // interface so downstream modules resolve correctly.

        ifaces_chirho.push(iface_chirho);

        // Phases 5–7: back-end
        let compile_result_chirho = compile_backend_chirho(module_chirho, infer_result_chirho)?;

        results_chirho.push((compile_result_chirho, recompiled_chirho));
    }

    Ok(results_chirho)
}

// ---------------------------------------------------------------------------
// Hierarchical project compilation from directory
// ---------------------------------------------------------------------------

/// Result of compiling a directory-based Haskell project.
#[derive(Debug)]
pub struct ProjectCompileResultChirho {
    /// Per-module compilation results, in dependency order.
    pub module_results_chirho: Vec<CompileResultChirho>,
    /// Module names in compilation order.
    pub compilation_order_chirho: Vec<String>,
    /// Non-fatal warnings across all modules.
    pub warnings_chirho: Vec<String>,
}

/// Walk a directory tree and find all `.hs` files recursively.
fn discover_hs_files_chirho(dir_chirho: &Path) -> Vec<PathBuf> {
    let mut files_chirho = Vec::new();
    if !dir_chirho.is_dir() {
        return files_chirho;
    }
    let mut stack_chirho = vec![dir_chirho.to_path_buf()];
    while let Some(current_chirho) = stack_chirho.pop() {
        let entries_chirho = match std::fs::read_dir(&current_chirho) {
            Ok(e_chirho) => e_chirho,
            Err(_) => continue,
        };
        for entry_chirho in entries_chirho.flatten() {
            let path_chirho = entry_chirho.path();
            if path_chirho.is_dir() {
                // Skip hidden directories and common non-source directories
                let name_chirho = entry_chirho.file_name();
                let name_str_chirho = name_chirho.to_string_lossy();
                if !name_str_chirho.starts_with('.')
                    && name_str_chirho != "dist-newstyle"
                    && name_str_chirho != "node_modules"
                    && name_str_chirho != ".stack-work"
                {
                    stack_chirho.push(path_chirho);
                }
            } else if path_chirho.extension().map_or(false, |ext_chirho| ext_chirho == "hs") {
                files_chirho.push(path_chirho);
            }
        }
    }
    files_chirho.sort();
    files_chirho
}

/// Extract module name from a Haskell source string by scanning for `module Name where`.
/// Falls back to "Main" if no module header is found.
fn extract_module_name_chirho(source_chirho: &str) -> String {
    for line_chirho in source_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();
        if trimmed_chirho.is_empty() || trimmed_chirho.starts_with("--") {
            continue;
        }
        if trimmed_chirho.starts_with("{-") {
            continue; // skip block comment starts (simplified)
        }
        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("module ") {
            let rest_chirho = rest_chirho.trim();
            // Module name is everything before "(" or "where"
            // Check "(" first since export lists appear before "where"
            let name_chirho = rest_chirho
                .split_once('(')
                .or_else(|| rest_chirho.split_once(" where"))
                .or_else(|| rest_chirho.split_once("where"))
                .map_or(rest_chirho, |(n_chirho, _)| n_chirho);
            return name_chirho.trim().to_string();
        }
        break; // First non-comment, non-module line means implicit Main
    }
    "Main".to_string()
}

/// Extract imported module names from a Haskell source string.
fn extract_imports_chirho(source_chirho: &str) -> Vec<String> {
    let mut imports_chirho = Vec::new();
    for line_chirho in source_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();
        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("import ") {
            let rest_chirho = rest_chirho.trim();
            // Skip "qualified" keyword if present
            let rest_chirho = rest_chirho
                .strip_prefix("qualified ")
                .unwrap_or(rest_chirho)
                .trim();
            // Module name is the first word (dotted identifier)
            let module_name_chirho = rest_chirho
                .split(|c_chirho: char| c_chirho.is_whitespace() || c_chirho == '(')
                .next()
                .unwrap_or("")
                .to_string();
            if !module_name_chirho.is_empty() {
                imports_chirho.push(module_name_chirho);
            }
        }
    }
    imports_chirho
}

/// Compile a Haskell project from a directory, automatically discovering `.hs`
/// files, computing dependency order from import declarations, and compiling
/// modules in topological order.
///
/// Returns an error if:
/// - No `.hs` files are found in the directory
/// - Circular module imports are detected
/// - Any module fails to compile
pub fn compile_project_dir_chirho(
    project_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<ProjectCompileResultChirho, String> {
    // Step 1: Discover all .hs files
    let hs_files_chirho = discover_hs_files_chirho(project_dir_chirho);
    if hs_files_chirho.is_empty() {
        return Err(format!(
            "No .hs files found in {}",
            project_dir_chirho.display()
        ));
    }

    // Step 2: Read sources and extract module names + imports
    let mut module_sources_chirho: Vec<(String, String, String)> = Vec::new(); // (module_name, file_path, source)
    for path_chirho in &hs_files_chirho {
        let source_chirho = std::fs::read_to_string(path_chirho).map_err(|e_chirho| {
            format!("Failed to read {}: {}", path_chirho.display(), e_chirho)
        })?;
        let module_name_chirho = extract_module_name_chirho(&source_chirho);
        let file_name_chirho = path_chirho.to_string_lossy().to_string();
        module_sources_chirho.push((module_name_chirho, file_name_chirho, source_chirho));
    }

    // Step 3: Build dependency graph
    let mut dep_graph_chirho = rhasky_incremental_chirho::DepGraphChirho::new_chirho();
    let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();

    for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
        let fp_chirho = rhasky_incremental_chirho::FingerprintChirho::from_str_chirho(source_chirho);
        dep_graph_chirho.add_module_chirho(module_name_chirho, fp_chirho);
        let imports_chirho = extract_imports_chirho(source_chirho);
        for imported_chirho in &imports_chirho {
            // Only add edges for local modules (skip external like Prelude, Data.Map, etc.)
            if known_modules_chirho.contains(imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, imported_chirho);
            }
        }
    }

    // Step 4: Topological sort
    let order_chirho = dep_graph_chirho.topo_sort_chirho().ok_or_else(|| {
        "Circular module imports detected. Cannot determine compilation order.".to_string()
    })?;

    // Step 5: Compile in dependency order
    let mut results_chirho: Vec<CompileResultChirho> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        rhasky_naming_chirho::builtin_module_ifaces_chirho();
    let mut all_warnings_chirho: Vec<String> = Vec::new();
    let mut imported_types_chirho: std::collections::HashMap<
        String,
        rhasky_typing_chirho::ty_chirho::SchemeChirho,
    > = std::collections::HashMap::new();

    for module_name_chirho in &order_chirho {
        let (_, file_name_chirho, source_chirho) = module_sources_chirho
            .iter()
            .find(|(n_chirho, _, _)| n_chirho == module_name_chirho)
            .ok_or_else(|| format!("Module {} not found in sources", module_name_chirho))?;

        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            file_name_chirho,
            source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();

        let frontend_result_chirho =
            run_frontend_chirho(source_chirho, file_id_chirho, &ifaces_chirho, &imported_types_chirho)
                .map_err(|e_chirho| format!("Error compiling {}: {}", module_name_chirho, e_chirho))?;

        let FrontendResultChirho {
            module_chirho,
            infer_result_chirho,
            warnings_chirho,
        } = frontend_result_chirho;

        all_warnings_chirho.extend(warnings_chirho);

        // Build interface for downstream modules
        let iface_chirho =
            build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

        // Accumulate exported type schemes
        for (name_chirho, val_chirho) in &iface_chirho.exports_chirho.values_chirho {
            if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(name_chirho) {
                imported_types_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
            }
            // Also try the iface value name directly
            let _ = val_chirho;
        }

        ifaces_chirho.push(iface_chirho);

        // Backend compilation
        let compile_result_chirho =
            compile_backend_chirho(module_chirho, infer_result_chirho)
                .map_err(|e_chirho| format!("Backend error for {}: {}", module_name_chirho, e_chirho))?;

        results_chirho.push(compile_result_chirho);
    }

    Ok(ProjectCompileResultChirho {
        module_results_chirho: results_chirho,
        compilation_order_chirho: order_chirho,
        warnings_chirho: all_warnings_chirho,
    })
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
    use rhasky_package_chirho::{parse_cabal_chirho, resolve_deps_chirho};
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
pub fn discover_modules_chirho(
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

/// Render a diagnostic bundle as a human-readable error report with source
/// code snippets, underline annotations, and optional ANSI colors.
pub fn render_diagnostics_chirho(
    diagnostics_chirho: &DiagnosticBundleChirho,
    source_map_chirho: &SourceMapChirho,
    color_chirho: bool,
) -> String {
    let config_chirho = if color_chirho {
        rhasky_diagnostics_chirho::render_chirho::RenderConfigChirho::default()
    } else {
        rhasky_diagnostics_chirho::render_chirho::RenderConfigChirho::plain_chirho()
    };
    rhasky_diagnostics_chirho::render_chirho::render_bundle_chirho(
        diagnostics_chirho,
        source_map_chirho,
        &config_chirho,
    )
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
mod tests_chirho;
