// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-driver-chirho
//!
//! Build orchestration for the Haskelujah compiler. Coordinates parsing, checking,
//! and backend lowering across execution modes.

#[path = "project_chirho/cabal_build_chirho.rs"]
mod cabal_build_chirho;
#[path = "backend_chirho/compile_backend_chirho.rs"]
mod compile_backend_chirho;
#[path = "backend_chirho/constructor_maps_chirho.rs"]
mod constructor_maps_chirho;
#[path = "native_link_chirho/core_merge_chirho.rs"]
mod core_merge_chirho;
#[path = "frontend_chirho/cpp_preprocess_chirho.rs"]
mod cpp_preprocess_chirho;
#[path = "project_chirho/dependency_packages_chirho.rs"]
mod dependency_packages_chirho;
#[path = "frontend_chirho/error_deferral_chirho.rs"]
mod error_deferral_chirho;
#[path = "project_chirho/family_exports_chirho.rs"]
mod family_exports_chirho;
pub(crate) use error_deferral_chirho::DeferralPolicyChirho;
pub use error_deferral_chirho::FrontendOptionsChirho;
#[path = "frontend_chirho/hsc_preprocess_chirho.rs"]
mod hsc_preprocess_chirho;
#[path = "project_chirho/import_links_chirho.rs"]
mod import_links_chirho;
#[path = "frontend_chirho/import_qualification_chirho.rs"]
mod import_qualification_chirho;
pub use import_qualification_chirho::qualified_import_alias_scheme_chirho;
#[path = "frontend_chirho/linearity_chirho.rs"]
mod linearity_chirho;
#[path = "project_chirho/module_discovery_chirho.rs"]
mod module_discovery_chirho;
#[path = "frontend_chirho/module_search_chirho.rs"]
mod module_search_chirho;
#[path = "project_chirho/multi_module_chirho.rs"]
mod multi_module_chirho;
#[path = "native_link_chirho/native_provenance_chirho.rs"]
mod native_provenance_chirho;
#[path = "project_chirho/package_install_chirho.rs"]
mod package_install_chirho;
#[path = "frontend_chirho/pipeline_chirho.rs"]
mod pipeline_chirho;
pub use pipeline_chirho::{
    FrontendImportsChirho, run_frontend_with_options_chirho,
    run_frontend_with_type_synonyms_families_and_class_env_chirho,
};
pub(crate) use pipeline_chirho::{
    inject_prelude_import_chirho, run_frontend_under_deferral_chirho,
};
#[path = "project_chirho/project_dir_chirho.rs"]
mod project_dir_chirho;
#[path = "native_link_chirho/rts_snapshot_chirho.rs"]
pub mod rts_snapshot_chirho;
#[cfg(test)]
#[path = "native_link_chirho/rts_snapshot_tests_chirho.rs"]
mod rts_snapshot_tests_chirho;
mod source_chirho;
pub use source_chirho::typecheck_source_chirho;
pub mod splice_chirho;
#[path = "frontend_chirho/stdlib_seed_chirho.rs"]
mod stdlib_seed_chirho;
pub mod stg_lower_chirho;
#[path = "frontend_chirho/type_names_chirho.rs"]
mod type_names_chirho;
// Every public item these modules took from lib.rs keeps its crate-root path (row 634).
pub use cabal_build_chirho::{
    CabalBuildResultChirho, CabalCompileResultChirho, CabalExecutableBuildResultChirho,
    build_cabal_project_chirho, compile_cabal_project_chirho,
};
pub use module_discovery_chirho::discover_modules_chirho;
pub use multi_module_chirho::{
    compile_modules_chirho, compile_modules_incremental_chirho, eval_modules_chirho,
    eval_modules_with_machine_chirho,
};
pub use package_install_chirho::{InstallResultChirho, install_package_chirho};
pub use project_dir_chirho::{ProjectCompileResultChirho, compile_project_dir_chirho};
// The moved project helpers lib.rs itself calls, and those its library tests name (row 634).
#[cfg(test)]
use cabal_build_chirho::{collect_library_package_deps_chirho, collect_package_deps_chirho};
use dependency_packages_chirho::{
    collect_frontend_artifacts_from_module_sources_chirho, find_dependency_packages_dir_chirho,
};
#[cfg(test)]
use dependency_packages_chirho::{
    collect_local_dependency_frontend_artifacts_chirho,
    compile_module_sources_with_extra_ifaces_chirho, find_local_dependency_package_dir_chirho,
    merge_local_dependency_package_index_chirho, scan_dependency_package_ifaces_chirho,
};
use family_exports_chirho::{
    build_family_aware_iface_chirho, build_type_family_scope_chirho,
    family_frontend_diagnostics_chirho, publish_inferred_family_exports_chirho,
    stamp_exported_family_references_chirho,
};
use package_install_chirho::find_cabal_in_dir_chirho;
use project_dir_chirho::{
    discover_hs_files_chirho, extract_module_name_chirho, source_imports_stdlib_chirho,
};
#[cfg(test)]
use project_dir_chirho::{
    extract_imports_chirho, filter_seeded_imported_types_for_source_chirho,
    filter_seeded_type_synonyms_for_source_chirho,
};

// The moved frontend helpers that lib.rs and other driver modules reach (row 691).
use cpp_preprocess_chirho::{
    CPP_GHC_VERSION_MAJOR_CHIRHO, CPP_GHC_VERSION_MINOR_CHIRHO, CPP_GHC_VERSION_PATCH_CHIRHO,
    CPP_GLASGOW_HASKELL_VERSION_CHIRHO, cpp_include_dirs_chirho, decode_cpp_stdout_chirho,
    discover_cpp_min_version_macro_options_chirho, has_cpp_directive_residue_chirho,
    preprocess_cpp_source_chirho, preprocess_cpp_source_with_options_chirho,
    source_uses_cpp_chirho, strip_cpp_directives_chirho,
};
use hsc_preprocess_chirho::{
    lower_maybe_like_unboxed_sums_chirho, path_is_hsc_chirho,
    preprocess_hsc_source_with_options_chirho, read_haskell_source_file_chirho,
};
#[cfg(test)]
use import_qualification_chirho::qualify_imported_scheme_for_iface_chirho;
use import_qualification_chirho::{
    ImportQualificationChirho, align_preferred_synonym_keys_chirho,
    collect_preferred_qualified_type_names_chirho,
    collect_safe_unqualified_imported_type_names_chirho, should_override_imported_scheme_chirho,
};
use linearity_chirho::check_module_linearity_chirho;
use stdlib_seed_chirho::{FrontendSeedArtifactsChirho, merge_stdlib_frontend_artifacts_chirho};
use type_names_chirho::{
    exported_type_synonyms_from_module_chirho, extend_imported_type_families_chirho,
    seed_builtin_type_families_chirho,
};

// The moved backend helpers that lib.rs and other driver modules reach (row 691).
use compile_backend_chirho::compile_backend_chirho;
use constructor_maps_chirho::{build_con_type_map_chirho, build_newtype_info_chirho};

use source_chirho::cpp_chirho::{configure_cpp_command_chirho, temporary_source_file_chirho};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use haskelujah_ast_chirho::ModuleChirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::promotion_chirho::{PromotionScopeChirho, stamp_promotions_chirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_backend_llvm_chirho::compile_core_to_llvm_chirho;
use haskelujah_backend_llvm_chirho::compile_to_llvm_ir_stub_chirho;
use haskelujah_backend_wasm_chirho::compile_core_to_wasm_chirho;
use haskelujah_backend_wasm_chirho::compile_to_wasm_stub_chirho;
use haskelujah_core_chirho::{CoreModuleChirho, SimplifyConfigChirho, simplify_module_chirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
use haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho;
use haskelujah_naming_chirho::resolve_chirho::resolve_module_with_imports_chirho;
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_runtime_chirho::{ExecutionModeChirho, RuntimePlanChirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;
use haskelujah_typing_chirho::infer_chirho::{
    InferResultChirho, InferenceImportContextChirho, TypeFamilyEnvChirho,
};

use module_search_chirho::{
    DiscoveredFamilyContextChirho, DiscoveredSourceModulesChirho,
    register_discovered_type_families_chirho, scan_hierarchical_modules_with_sources_chirho,
    scan_sibling_modules_with_sources_chirho,
};

type ImportedTypeSynonymsChirho = std::collections::HashMap<String, (Vec<String>, TypeChirho)>;
type ImportedTypeFamiliesChirho = TypeFamilyEnvChirho;

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
    /// Type synonyms that were actually in scope for this module after import
    /// processing. This lets downstream package builds preserve re-exported
    /// aliases such as `GenParser`.
    pub resolved_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
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
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    imported_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    >,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let imported_type_families_chirho = seed_builtin_type_families_chirho();
    run_frontend_with_type_synonyms_and_type_families_chirho(
        source_chirho,
        file_id_chirho,
        ifaces_chirho,
        imported_types_chirho,
        &ImportedTypeSynonymsChirho::new(),
        &imported_type_families_chirho,
    )
}

pub fn run_frontend_with_type_synonyms_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    imported_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    >,
    imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let imported_type_families_chirho = seed_builtin_type_families_chirho();
    run_frontend_with_type_synonyms_and_type_families_chirho(
        source_chirho,
        file_id_chirho,
        ifaces_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        &imported_type_families_chirho,
    )
}

pub fn run_frontend_with_type_synonyms_and_type_families_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    imported_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    >,
    imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
    imported_type_families_chirho: &ImportedTypeFamiliesChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let imported_class_env_chirho =
        haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();
    run_frontend_with_type_synonyms_families_and_class_env_chirho(
        source_chirho,
        file_id_chirho,
        ifaces_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_type_families_chirho,
        &imported_class_env_chirho,
    )
}

pub fn check_source_path_chirho(
    path_chirho: impl AsRef<Path>,
    execution_mode_chirho: ExecutionModeChirho,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    check_source_path_with_source_map_chirho(
        path_chirho,
        execution_mode_chirho,
        &mut source_map_chirho,
    )
}

pub fn check_source_path_with_source_map_chirho(
    path_chirho: impl AsRef<Path>,
    execution_mode_chirho: ExecutionModeChirho,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let source_chirho = read_haskell_source_file_chirho(&path_chirho).map_err(|error_chirho| {
        DiagnosticChirho::error_no_span_chirho(format!(
            "unable to read `{}`: {error_chirho}",
            path_chirho.as_ref().display()
        ))
    })?;
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        path_chirho.as_ref().to_path_buf(),
        source_chirho,
    );

    if let Some(search_dir_chirho) = path_chirho.as_ref().parent() {
        check_source_file_with_search_path_chirho(
            source_file_chirho,
            execution_mode_chirho,
            search_dir_chirho,
        )
    } else {
        check_source_file_chirho(source_file_chirho, execution_mode_chirho)
    }
}

pub fn check_source_file_chirho(
    source_file_chirho: SourceFileChirho,
    execution_mode_chirho: ExecutionModeChirho,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let source_path_chirho = source_file_chirho.path_chirho().to_path_buf();
    let raw_source_chirho = source_file_chirho.contents_chirho().to_string();
    let source_chirho = preprocess_cpp_chirho(&raw_source_chirho);
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
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

fn check_source_file_with_search_path_chirho(
    source_file_chirho: SourceFileChirho,
    execution_mode_chirho: ExecutionModeChirho,
    search_dir_chirho: &Path,
) -> Result<CheckSummaryChirho, DiagnosticBundleChirho> {
    let source_path_chirho = source_file_chirho.path_chirho().to_path_buf();
    let raw_source_chirho = source_file_chirho.contents_chirho().to_string();
    let source_chirho = preprocess_cpp_chirho(&raw_source_chirho);
    let file_id_chirho = source_file_chirho.file_id_chirho();
    let file_name_chirho = source_path_chirho
        .file_name()
        .map(|name_chirho| name_chirho.to_string_lossy().to_string())
        .unwrap_or_else(|| source_path_chirho.to_string_lossy().to_string());

    let mut sibling_source_map_chirho = SourceMapChirho::new_chirho();
    let mut discovered_sources_chirho = DiscoveredSourceModulesChirho::new();
    let mut all_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut imported_types_chirho = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();
    if source_imports_stdlib_chirho(&source_chirho) {
        merge_stdlib_frontend_artifacts_chirho(
            &mut all_ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        )
        .map_err(|error_chirho| {
            family_frontend_diagnostics_chirho(
                error_chirho,
                haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
            )
        })?;
    }

    scan_sibling_modules_with_sources_chirho(
        search_dir_chirho,
        &mut sibling_source_map_chirho,
        &mut all_ifaces_chirho,
        &file_name_chirho,
        &mut discovered_sources_chirho,
    );
    for _round_chirho in 0..5 {
        let prev_count_chirho = all_ifaces_chirho.len();
        scan_hierarchical_modules_with_sources_chirho(
            search_dir_chirho,
            search_dir_chirho,
            &mut sibling_source_map_chirho,
            &mut all_ifaces_chirho,
            &file_name_chirho,
            &mut discovered_sources_chirho,
        );
        if all_ifaces_chirho.len() == prev_count_chirho {
            break;
        }
    }

    let discovery_parser_chirho = ParserChirho::new_chirho(&source_chirho, file_id_chirho);
    let discovery_green_chirho = discovery_parser_chirho.parse_chirho();
    let mut discovery_module_chirho = lower_module_chirho(&discovery_green_chirho, file_id_chirho);
    inject_prelude_import_chirho(&mut discovery_module_chirho);
    register_discovered_type_families_chirho(
        &discovery_module_chirho,
        DiscoveredFamilyContextChirho {
            modules_chirho: &discovered_sources_chirho,
            ifaces_chirho: &mut all_ifaces_chirho,
            registry_chirho: &mut imported_type_families_chirho,
            imported_synonyms_chirho: &imported_type_synonyms_chirho,
            checked_path_chirho: Some(&source_path_chirho),
        },
    )?;

    let frontend_result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
        &source_chirho,
        file_id_chirho,
        &all_ifaces_chirho,
        &imported_types_chirho,
        &imported_type_synonyms_chirho,
        &imported_type_families_chirho,
    )?;

    let module_name_chirho = frontend_result_chirho
        .module_chirho
        .name_chirho
        .text_chirho()
        .to_string();
    let runtime_plan_chirho =
        RuntimePlanChirho::for_module_chirho(execution_mode_chirho, module_name_chirho.clone());
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

/// Result of the full compilation pipeline. Not `Clone`, because the module it
/// holds is not.
#[derive(Debug)]
pub struct CompileResultChirho {
    pub module_chirho: ModuleChirho,
    pub core_chirho: CoreModuleChirho,
    pub llvm_ir_chirho: String,
    pub wasm_bytes_chirho: Vec<u8>,
    /// Set of newtype constructor names — these are identity at runtime.
    pub newtype_cons_chirho: std::collections::HashSet<String>,
}

/// Run only the frontend pipeline and return non-fatal warnings.
/// Useful for testing diagnostic output (orphan instances, exhaustiveness, etc.)
pub fn frontend_warnings_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
) -> Result<Vec<String>, DiagnosticBundleChirho> {
    frontend_warnings_with_options_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
        &FrontendOptionsChirho::default(),
    )
}

/// [`frontend_warnings_chirho`] under the caller's GHC options, which apply before the
/// module header's own `OPTIONS_GHC` pragmas (row 695).
/// workflow: compiler-pipeline-chirho/error-deferral-chirho
pub fn frontend_warnings_with_options_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    options_chirho: &FrontendOptionsChirho,
) -> Result<Vec<String>, DiagnosticBundleChirho> {
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        file_name_chirho,
        source_chirho,
    );
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let empty_imported_types_chirho = std::collections::HashMap::new();
    let imported_type_families_chirho = seed_builtin_type_families_chirho();
    let imported_class_env_chirho =
        haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();

    let frontend_result_chirho = run_frontend_with_options_chirho(
        source_chirho,
        file_id_chirho,
        FrontendImportsChirho {
            ifaces_chirho: &builtin_ifaces_chirho,
            types_chirho: &empty_imported_types_chirho,
            type_synonyms_chirho: &ImportedTypeSynonymsChirho::new(),
            type_families_chirho: &imported_type_families_chirho,
            class_env_chirho: &imported_class_env_chirho,
        },
        options_chirho,
    )?;

    Ok(frontend_result_chirho.warnings_chirho)
}

/// Preprocess source code with CPP if `{-# LANGUAGE CPP #-}` is present.
/// Shells out to the system C preprocessor, defining `__GLASGOW_HASKELL__`
/// for compatibility with packages that conditionally compile for GHC versions.
pub fn preprocess_cpp_chirho(source_chirho: &str) -> String {
    if !source_uses_cpp_chirho(source_chirho) {
        return lower_maybe_like_unboxed_sums_chirho(source_chirho);
    }

    match temporary_source_file_chirho(source_chirho, None, ".hs")
        .and_then(|file_chirho| preprocess_cpp_source_chirho(file_chirho.path(), source_chirho))
    {
        Ok(processed_chirho) => {
            let cleaned_chirho = if has_cpp_directive_residue_chirho(&processed_chirho) {
                strip_cpp_directives_chirho(&processed_chirho)
            } else {
                processed_chirho
            };
            lower_maybe_like_unboxed_sums_chirho(&cleaned_chirho)
        }
        _ => lower_maybe_like_unboxed_sums_chirho(&strip_cpp_directives_chirho(source_chirho)),
    }
}

/// Run the full compiler pipeline: lex → layout → CST parse → AST lower →
/// name resolve → type infer → desugar to Core → simplify.
/// Returns the AST module and its optimized Core IR.
pub fn compile_source_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    compile_source_with_options_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
        &FrontendOptionsChirho::default(),
    )
}

/// [`compile_source_chirho`] under the caller's GHC options, which apply before the module
/// header's own `OPTIONS_GHC` pragmas (row 695).
/// workflow: compiler-pipeline-chirho/error-deferral-chirho
pub fn compile_source_with_options_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    options_chirho: &FrontendOptionsChirho,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    let (frontend_result_chirho, ifaces_chirho) =
        source_chirho::typecheck_source_with_ifaces_and_options_chirho(
            source_chirho,
            source_map_chirho,
            file_name_chirho,
            options_chirho,
        )?;

    let FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        resolved_imported_type_synonyms_chirho: _resolved_imported_type_synonyms_chirho,
        warnings_chirho: _warnings_chirho,
    } = frontend_result_chirho;

    compile_backend_chirho(
        module_chirho,
        infer_result_chirho,
        std::collections::HashSet::new(),
        &ifaces_chirho,
        // A single module links no other module of the program.
        &std::collections::HashSet::new(),
    )
}

/// Compile a source file, also searching sibling `.hs` files in the same directory
/// to build module interfaces for cross-module imports (like GHC test companion files).
pub fn compile_source_with_search_path_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    search_dir_chirho: &Path,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    compile_source_with_search_context_chirho(
        source_chirho,
        source_map_chirho,
        SourceSearchContextChirho {
            file_name_chirho,
            search_dir_chirho,
            checked_path_chirho: None,
        },
    )
}

#[path = "backend_chirho/evidence_join_chirho.rs"]
mod evidence_join_chirho;
#[cfg(test)]
pub(crate) use evidence_join_chirho::join_occurrence_evidence_chirho;

/// Compile and evaluate a Haskell source program through the full pipeline,
/// returning the final runtime value of the named entry point.
pub fn eval_source_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
) -> Result<haskelujah_runtime_chirho::ValueChirho, String> {
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
) -> Result<
    (
        haskelujah_runtime_chirho::ValueChirho,
        haskelujah_runtime_chirho::eval_chirho::MachineChirho,
    ),
    String,
> {
    eval_source_outcome_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
        entry_name_chirho,
    )?
    .into_result_chirho()
}

/// `eval_source_with_machine_chirho`, keeping the machine when the run fails
/// at runtime, so `haskelujah run` can emit what the program wrote before the
/// failure. `Err` is a program that never ran: a compile error, or no entry.
/// workflow: testing-chirho/execution-oracles-chirho
pub fn eval_source_outcome_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
) -> Result<stg_lower_chirho::RunOutcomeChirho, String> {
    eval_source_outcome_with_options_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
        entry_name_chirho,
        &FrontendOptionsChirho::default(),
    )
}

/// [`eval_source_outcome_chirho`] under the caller's GHC options, which apply before the
/// module header's own `OPTIONS_GHC` pragmas (row 695).
/// workflow: compiler-pipeline-chirho/error-deferral-chirho
pub fn eval_source_outcome_with_options_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
    options_chirho: &FrontendOptionsChirho,
) -> Result<stg_lower_chirho::RunOutcomeChirho, String> {
    let compile_result_chirho = compile_source_with_options_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
        options_chirho,
    )
    .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    stg_lower_chirho::lower_and_run_outcome_chirho(
        &compile_result_chirho.core_chirho,
        entry_name_chirho,
        compile_result_chirho.newtype_cons_chirho,
    )
}

/// [`eval_source_with_machine_chirho`] under the caller's GHC options (row 695).
pub fn eval_source_with_options_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
    options_chirho: &FrontendOptionsChirho,
) -> Result<
    (
        haskelujah_runtime_chirho::ValueChirho,
        haskelujah_runtime_chirho::eval_chirho::MachineChirho,
    ),
    String,
> {
    eval_source_outcome_with_options_chirho(
        source_chirho,
        source_map_chirho,
        file_name_chirho,
        entry_name_chirho,
        options_chirho,
    )?
    .into_result_chirho()
}

/// Like `eval_source_with_machine_chirho` but with stdin input lines pre-loaded.
pub fn eval_source_with_input_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    file_name_chirho: &str,
    entry_name_chirho: Option<&str>,
    input_lines_chirho: &[&str],
) -> Result<
    (
        haskelujah_runtime_chirho::ValueChirho,
        haskelujah_runtime_chirho::eval_chirho::MachineChirho,
    ),
    String,
> {
    let compile_result_chirho =
        compile_source_chirho(source_chirho, source_map_chirho, file_name_chirho)
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
) -> Result<
    (
        haskelujah_runtime_chirho::ValueChirho,
        haskelujah_runtime_chirho::eval_chirho::MachineChirho,
    ),
    String,
> {
    let compile_result_chirho =
        compile_source_chirho(source_chirho, source_map_chirho, file_name_chirho)
            .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    stg_lower_chirho::lower_and_run_with_step_limit_chirho(
        &compile_result_chirho.core_chirho,
        entry_name_chirho,
        compile_result_chirho.newtype_cons_chirho,
        step_limit_chirho,
    )
}

/// Render a diagnostic bundle as a human-readable error report with source
/// code snippets, underline annotations, and optional ANSI colors.
pub fn render_diagnostics_chirho(
    diagnostics_chirho: &DiagnosticBundleChirho,
    source_map_chirho: &SourceMapChirho,
    color_chirho: bool,
) -> String {
    let config_chirho = if color_chirho {
        haskelujah_diagnostics_chirho::render_chirho::RenderConfigChirho::default()
    } else {
        haskelujah_diagnostics_chirho::render_chirho::RenderConfigChirho::plain_chirho()
    };
    haskelujah_diagnostics_chirho::render_chirho::render_bundle_chirho(
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
        check_summary_chirho
            .runtime_plan_chirho
            .execution_mode_chirho,
        check_summary_chirho
            .runtime_plan_chirho
            .incremental_session_chirho,
        check_summary_chirho
            .backend_plan_chirho
            .llvm_preview_chirho
            .trim_end(),
        check_summary_chirho
            .backend_plan_chirho
            .wasm_stub_size_chirho,
    )
}

#[cfg(test)]
#[path = "frontend_chirho/synonym_keys_tests_chirho.rs"]
mod synonym_keys_tests_chirho;
#[cfg(test)]
mod tests_chirho;

pub fn compile_source_file_with_search_path_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    checked_path_chirho: &Path,
    search_dir_chirho: &Path,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    let file_name_chirho = checked_path_chirho
        .file_name()
        .unwrap_or(checked_path_chirho.as_os_str())
        .to_string_lossy();
    compile_source_with_search_context_chirho(
        source_chirho,
        source_map_chirho,
        SourceSearchContextChirho {
            file_name_chirho: &file_name_chirho,
            search_dir_chirho,
            checked_path_chirho: Some(checked_path_chirho),
        },
    )
}

fn compile_source_with_search_context_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    context_chirho: SourceSearchContextChirho<'_>,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    let SourceSearchContextChirho {
        file_name_chirho,
        search_dir_chirho,
        checked_path_chirho,
    } = context_chirho;
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        file_name_chirho,
        source_chirho,
    );
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let mut all_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut discovered_sources_chirho = DiscoveredSourceModulesChirho::new();
    let mut imported_types_chirho = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();
    if source_imports_stdlib_chirho(source_chirho) {
        merge_stdlib_frontend_artifacts_chirho(
            &mut all_ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        )
        .map_err(|error_chirho| {
            family_frontend_diagnostics_chirho(
                error_chirho,
                haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
            )
        })?;
    }

    scan_sibling_modules_with_sources_chirho(
        search_dir_chirho,
        source_map_chirho,
        &mut all_ifaces_chirho,
        file_name_chirho,
        &mut discovered_sources_chirho,
    );

    // Scan ALL subdirectories recursively for hierarchical module imports
    // (e.g., Debug/SimpleReflect/Expr.hs for `import Debug.SimpleReflect.Expr`).
    // Fixed-point iteration handles dependency chains between modules.
    for _round_chirho in 0..5 {
        let prev_count_chirho = all_ifaces_chirho.len();
        scan_hierarchical_modules_with_sources_chirho(
            search_dir_chirho,
            search_dir_chirho,
            source_map_chirho,
            &mut all_ifaces_chirho,
            file_name_chirho,
            &mut discovered_sources_chirho,
        );
        if all_ifaces_chirho.len() == prev_count_chirho {
            break;
        }
    }

    let discovery_parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let discovery_green_chirho = discovery_parser_chirho.parse_chirho();
    let mut discovery_module_chirho = lower_module_chirho(&discovery_green_chirho, file_id_chirho);
    inject_prelude_import_chirho(&mut discovery_module_chirho);
    register_discovered_type_families_chirho(
        &discovery_module_chirho,
        DiscoveredFamilyContextChirho {
            modules_chirho: &discovered_sources_chirho,
            ifaces_chirho: &mut all_ifaces_chirho,
            registry_chirho: &mut imported_type_families_chirho,
            imported_synonyms_chirho: &imported_type_synonyms_chirho,
            checked_path_chirho,
        },
    )?;

    let frontend_result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
        source_chirho,
        file_id_chirho,
        &all_ifaces_chirho,
        &imported_types_chirho,
        &imported_type_synonyms_chirho,
        &imported_type_families_chirho,
    )?;

    let FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        resolved_imported_type_synonyms_chirho: _resolved_imported_type_synonyms_chirho,
        warnings_chirho: _warnings_chirho,
    } = frontend_result_chirho;

    compile_backend_chirho(
        module_chirho,
        infer_result_chirho,
        std::collections::HashSet::new(),
        &all_ifaces_chirho,
        // Siblings lend their interfaces; their Core is not linked here.
        &std::collections::HashSet::new(),
    )
}

pub fn eval_source_file_outcome_chirho(
    source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    checked_path_chirho: &Path,
    entry_name_chirho: Option<&str>,
) -> Result<stg_lower_chirho::RunOutcomeChirho, String> {
    let search_dir_chirho = checked_path_chirho
        .parent()
        .filter(|parent_chirho| !parent_chirho.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let compile_result_chirho = compile_source_file_with_search_path_chirho(
        source_chirho,
        source_map_chirho,
        checked_path_chirho,
        search_dir_chirho,
    )
    .map_err(|diagnostics_chirho| format!("compilation error: {diagnostics_chirho:?}"))?;
    stg_lower_chirho::lower_and_run_outcome_chirho(
        &compile_result_chirho.core_chirho,
        entry_name_chirho,
        compile_result_chirho.newtype_cons_chirho,
    )
}

struct SourceSearchContextChirho<'source_chirho> {
    file_name_chirho: &'source_chirho str,
    search_dir_chirho: &'source_chirho Path,
    checked_path_chirho: Option<&'source_chirho Path>,
}
