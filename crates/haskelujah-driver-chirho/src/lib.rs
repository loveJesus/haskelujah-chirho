// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-driver-chirho
//!
//! Build orchestration for the Haskelujah compiler. Coordinates parsing, checking,
//! and backend lowering across execution modes.

#[path = "project_chirho/cabal_build_chirho.rs"]
mod cabal_build_chirho;
#[path = "native_link_chirho/core_merge_chirho.rs"]
mod core_merge_chirho;
#[path = "project_chirho/dependency_packages_chirho.rs"]
mod dependency_packages_chirho;
#[path = "project_chirho/module_discovery_chirho.rs"]
mod module_discovery_chirho;
mod module_search_chirho;
#[path = "project_chirho/multi_module_chirho.rs"]
mod multi_module_chirho;
#[path = "native_link_chirho/native_provenance_chirho.rs"]
mod native_provenance_chirho;
#[path = "project_chirho/package_install_chirho.rs"]
mod package_install_chirho;
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
pub mod stg_lower_chirho;
// Every public item these modules took from lib.rs keeps its crate-root path (row 634).
pub use cabal_build_chirho::{
    CabalBuildResultChirho, CabalCompileResultChirho, CabalExecutableBuildResultChirho,
    build_cabal_project_chirho, compile_cabal_project_chirho,
};
pub use module_discovery_chirho::discover_modules_chirho;
pub use multi_module_chirho::{
    compile_modules_chirho, compile_modules_incremental_chirho, eval_modules_chirho,
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
use package_install_chirho::find_cabal_in_dir_chirho;
use project_dir_chirho::{
    discover_hs_files_chirho, extract_module_name_chirho, source_imports_stdlib_chirho,
};
#[cfg(test)]
use project_dir_chirho::{
    extract_imports_chirho, filter_seeded_imported_types_for_source_chirho,
    filter_seeded_type_synonyms_for_source_chirho,
};

use source_chirho::cpp_chirho::{configure_cpp_command_chirho, temporary_source_file_chirho};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use haskelujah_ast_chirho::ModuleChirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
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
    InferResultChirho, TypeFamilyEnvChirho, infer_module_chirho,
    infer_module_with_imports_type_synonyms_families_and_class_env_chirho,
};

use module_search_chirho::{scan_hierarchical_modules_chirho, scan_sibling_module_ifaces_chirho};

type ImportedTypeSynonymsChirho = std::collections::HashMap<String, (Vec<String>, TypeChirho)>;
type ImportedTypeFamiliesChirho = TypeFamilyEnvChirho;

/// Seed builtin type family equations that packages commonly depend on.
/// PrimState is defined in the `primitive` package but many packages
/// use it transitively (aeson, resourcet, vector-builder, etc.).
fn seed_builtin_type_families_chirho() -> ImportedTypeFamiliesChirho {
    use haskelujah_typing_chirho::TyChirho;
    let mut families_chirho = ImportedTypeFamiliesChirho::new();

    // type instance PrimState (ST s) = s
    let s_var_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::TyVarChirho(9900));
    families_chirho.insert(
        "PrimState".to_string(),
        vec![(
            vec![TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(s_var_chirho.clone()),
            )],
            s_var_chirho.clone(),
        )],
    );

    // type instance PrimState IO = RealWorld
    families_chirho.get_mut("PrimState").unwrap().push((
        vec![TyChirho::ConChirho("IO".to_string())],
        TyChirho::ConChirho("RealWorld".to_string()),
    ));

    families_chirho
}

fn extend_imported_type_families_chirho(
    target_chirho: &mut ImportedTypeFamiliesChirho,
    source_chirho: ImportedTypeFamiliesChirho,
) {
    for (family_name_chirho, equations_chirho) in source_chirho {
        target_chirho
            .entry(family_name_chirho)
            .or_default()
            .extend(equations_chirho);
    }
}

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

#[derive(Debug, Default, Clone)]
struct FrontendSeedArtifactsChirho {
    ifaces_chirho: Vec<ModuleIfaceChirho>,
    imported_types_chirho:
        std::collections::HashMap<String, haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    imported_type_families_chirho: ImportedTypeFamiliesChirho,
}

static STDLIB_FRONTEND_ARTIFACTS_CHIRHO: OnceLock<Result<FrontendSeedArtifactsChirho, String>> =
    OnceLock::new();

fn merge_stdlib_frontend_artifacts_chirho(
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    imported_types_chirho: &mut std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    imported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    imported_type_families_chirho: &mut ImportedTypeFamiliesChirho,
) {
    let stdlib_artifacts_chirho = STDLIB_FRONTEND_ARTIFACTS_CHIRHO
        .get_or_init(collect_stdlib_frontend_artifacts_uncached_chirho);

    match stdlib_artifacts_chirho {
        Ok(stdlib_artifacts_chirho) => {
            ifaces_chirho.extend(stdlib_artifacts_chirho.ifaces_chirho.clone());
            *ifaces_chirho = haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(
                std::mem::take(ifaces_chirho),
            );
            imported_types_chirho.extend(stdlib_artifacts_chirho.imported_types_chirho.clone());
            imported_type_synonyms_chirho.extend(
                stdlib_artifacts_chirho
                    .imported_type_synonyms_chirho
                    .clone(),
            );
            extend_imported_type_families_chirho(
                imported_type_families_chirho,
                stdlib_artifacts_chirho
                    .imported_type_families_chirho
                    .clone(),
            );
        }
        Err(error_chirho) => {
            eprintln!("warning: stdlib frontend seed skipped: {}", error_chirho);
        }
    }
}

fn collect_stdlib_frontend_artifacts_uncached_chirho() -> Result<FrontendSeedArtifactsChirho, String>
{
    let Some(stdlib_dir_chirho) = find_stdlib_dir_chirho() else {
        return Ok(FrontendSeedArtifactsChirho::default());
    };

    let mut module_sources_chirho: Vec<(String, String, String)> = Vec::new();
    for path_chirho in discover_hs_files_chirho(&stdlib_dir_chirho) {
        let source_chirho =
            read_haskell_source_file_chirho(&path_chirho).map_err(|error_chirho| {
                format!(
                    "cannot read stdlib module {}: {}",
                    path_chirho.display(),
                    error_chirho
                )
            })?;
        let module_name_chirho = extract_module_name_chirho(&source_chirho);
        module_sources_chirho.push((
            module_name_chirho,
            path_chirho.to_string_lossy().to_string(),
            source_chirho,
        ));
    }

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        module_sources_chirho,
        &mut source_map_chirho,
        Vec::new(),
        std::collections::HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        seed_builtin_type_families_chirho(),
        false,
    )?;
    artifacts_chirho
        .ifaces_chirho
        .retain(|iface_chirho| iface_chirho.name_chirho.starts_with("Haskelujah."));
    Ok(artifacts_chirho)
}

fn find_stdlib_dir_chirho() -> Option<PathBuf> {
    if let Ok(explicit_dir_chirho) = std::env::var("HASKELUJAH_STDLIB_DIR_CHIRHO") {
        let explicit_path_chirho = PathBuf::from(explicit_dir_chirho);
        if stdlib_dir_is_valid_chirho(&explicit_path_chirho) {
            return Some(explicit_path_chirho);
        }
    }

    let manifest_stdlib_dir_chirho = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .map(|path_chirho| path_chirho.join("stdlib-chirho"));
    if let Some(manifest_stdlib_dir_chirho) = manifest_stdlib_dir_chirho {
        if stdlib_dir_is_valid_chirho(&manifest_stdlib_dir_chirho) {
            return Some(manifest_stdlib_dir_chirho);
        }
    }

    if let Ok(current_exe_chirho) = std::env::current_exe() {
        for ancestor_chirho in current_exe_chirho.ancestors() {
            let candidate_chirho = ancestor_chirho.join("stdlib-chirho");
            if stdlib_dir_is_valid_chirho(&candidate_chirho) {
                return Some(candidate_chirho);
            }
        }
    }

    if let Ok(current_dir_chirho) = std::env::current_dir() {
        for ancestor_chirho in current_dir_chirho.ancestors() {
            let candidate_chirho = ancestor_chirho.join("stdlib-chirho");
            if stdlib_dir_is_valid_chirho(&candidate_chirho) {
                return Some(candidate_chirho);
            }
        }
    }

    None
}

fn stdlib_dir_is_valid_chirho(path_chirho: &Path) -> bool {
    path_chirho.join("Haskelujah").join("Prelude.hs").is_file()
}

fn collect_type_constructor_names_chirho(
    ty_chirho: &TypeChirho,
    names_chirho: &mut std::collections::HashSet<String>,
) {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            names_chirho.insert(name_chirho.full_name_chirho());
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            collect_type_constructor_names_chirho(fun_chirho, names_chirho);
            collect_type_constructor_names_chirho(arg_chirho, names_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            collect_type_constructor_names_chirho(arg_chirho, names_chirho);
            collect_type_constructor_names_chirho(result_chirho, names_chirho);
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                collect_type_constructor_names_chirho(elem_chirho, names_chirho);
            }
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            collect_type_constructor_names_chirho(element_chirho, names_chirho);
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            collect_type_constructor_names_chirho(inner_chirho, names_chirho);
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for constraint_chirho in context_chirho {
                match constraint_chirho {
                    haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                        args_chirho,
                        ..
                    } => {
                        for arg_chirho in args_chirho {
                            collect_type_constructor_names_chirho(arg_chirho, names_chirho);
                        }
                    }
                    haskelujah_ast_chirho::ty_chirho::ConstraintChirho::QuantifiedChirho {
                        context_chirho,
                        body_chirho,
                        ..
                    } => {
                        for inner_constraint_chirho in context_chirho {
                            if let haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                                args_chirho,
                                ..
                            } = inner_constraint_chirho
                            {
                                for arg_chirho in args_chirho {
                                    collect_type_constructor_names_chirho(arg_chirho, names_chirho);
                                }
                            }
                        }
                        if let haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                            args_chirho,
                            ..
                        } = body_chirho.as_ref()
                        {
                            for arg_chirho in args_chirho {
                                collect_type_constructor_names_chirho(arg_chirho, names_chirho);
                            }
                        }
                    }
                }
            }
            collect_type_constructor_names_chirho(body_chirho, names_chirho);
        }
        TypeChirho::ForallChirho { body_chirho, .. }
        | TypeChirho::RequiredForallChirho { body_chirho, .. } => {
            collect_type_constructor_names_chirho(body_chirho, names_chirho);
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                collect_type_constructor_names_chirho(elem_chirho, names_chirho);
            }
        }
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            names_chirho.insert(name_chirho.full_name_chirho());
        }
        TypeChirho::VarChirho(_)
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => {}
    }
}

fn insert_type_synonym_with_module_alias_chirho(
    exported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    module_name_chirho: &str,
    alias_name_chirho: &str,
    params_chirho: Vec<String>,
    rhs_chirho: TypeChirho,
) {
    exported_type_synonyms_chirho.insert(
        alias_name_chirho.to_string(),
        (params_chirho.clone(), rhs_chirho.clone()),
    );
    exported_type_synonyms_chirho.insert(
        format!("{module_name_chirho}.{alias_name_chirho}"),
        (params_chirho, rhs_chirho),
    );
}

fn seed_transitive_type_synonyms_chirho(
    exported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    module_name_chirho: &str,
    rhs_chirho: &TypeChirho,
    resolved_imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
    seen_chirho: &mut std::collections::HashSet<String>,
) {
    let mut referenced_names_chirho = std::collections::HashSet::new();
    collect_type_constructor_names_chirho(rhs_chirho, &mut referenced_names_chirho);
    for referenced_name_chirho in referenced_names_chirho {
        let bare_name_chirho = referenced_name_chirho
            .rsplit_once('.')
            .map(|(_prefix_chirho, bare_name_chirho)| bare_name_chirho)
            .unwrap_or(referenced_name_chirho.as_str())
            .to_string();
        if !seen_chirho.insert(bare_name_chirho.clone()) {
            continue;
        }
        let candidate_keys_chirho = [
            referenced_name_chirho.clone(),
            format!("{module_name_chirho}.{bare_name_chirho}"),
            bare_name_chirho.clone(),
        ];
        let Some((params_chirho, transitive_rhs_chirho)) = candidate_keys_chirho
            .iter()
            .find_map(|key_chirho| resolved_imported_type_synonyms_chirho.get(key_chirho))
            .cloned()
        else {
            continue;
        };
        insert_type_synonym_with_module_alias_chirho(
            exported_type_synonyms_chirho,
            module_name_chirho,
            &bare_name_chirho,
            params_chirho,
            transitive_rhs_chirho.clone(),
        );
        seed_transitive_type_synonyms_chirho(
            exported_type_synonyms_chirho,
            module_name_chirho,
            &transitive_rhs_chirho,
            resolved_imported_type_synonyms_chirho,
            seen_chirho,
        );
    }
}

fn exported_type_synonyms_from_module_chirho(
    module_chirho: &ModuleChirho,
    iface_chirho: &ModuleIfaceChirho,
    resolved_imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
) -> ImportedTypeSynonymsChirho {
    let mut exported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut local_type_alias_names_chirho = std::collections::HashSet::new();
    let mut transitive_seen_chirho = std::collections::HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::TypeAliasDeclChirho {
            name_chirho,
            type_vars_chirho,
            rhs_chirho,
            ..
        } = decl_chirho
        {
            let alias_name_chirho = name_chirho.text_chirho().to_string();
            local_type_alias_names_chirho.insert(alias_name_chirho.clone());
            if iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key(&alias_name_chirho)
            {
                insert_type_synonym_with_module_alias_chirho(
                    &mut exported_type_synonyms_chirho,
                    &iface_chirho.name_chirho,
                    &alias_name_chirho,
                    type_vars_chirho
                        .iter()
                        .map(|ty_var_chirho| ty_var_chirho.text_chirho().to_string())
                        .collect(),
                    rhs_chirho.clone(),
                );
                seed_transitive_type_synonyms_chirho(
                    &mut exported_type_synonyms_chirho,
                    &iface_chirho.name_chirho,
                    rhs_chirho,
                    resolved_imported_type_synonyms_chirho,
                    &mut transitive_seen_chirho,
                );
            }
        }
    }
    for exported_type_name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
        if local_type_alias_names_chirho.contains(exported_type_name_chirho)
            || exported_type_synonyms_chirho.contains_key(exported_type_name_chirho)
        {
            continue;
        }
        if let Some((params_chirho, rhs_chirho)) =
            resolved_imported_type_synonyms_chirho.get(exported_type_name_chirho)
        {
            insert_type_synonym_with_module_alias_chirho(
                &mut exported_type_synonyms_chirho,
                &iface_chirho.name_chirho,
                exported_type_name_chirho,
                params_chirho.clone(),
                rhs_chirho.clone(),
            );
            seed_transitive_type_synonyms_chirho(
                &mut exported_type_synonyms_chirho,
                &iface_chirho.name_chirho,
                rhs_chirho,
                resolved_imported_type_synonyms_chirho,
                &mut transitive_seen_chirho,
            );
        }
    }
    exported_type_synonyms_chirho
}

const CPP_GLASGOW_HASKELL_VERSION_CHIRHO: &str = "810";
const CPP_GHC_VERSION_MAJOR_CHIRHO: u32 = 8;
const CPP_GHC_VERSION_MINOR_CHIRHO: u32 = 10;
const CPP_GHC_VERSION_PATCH_CHIRHO: u32 = 0;

fn cpp_macro_package_name_chirho(package_name_chirho: &str) -> String {
    package_name_chirho
        .chars()
        .map(|char_chirho| {
            if char_chirho.is_ascii_alphanumeric() {
                char_chirho
            } else {
                '_'
            }
        })
        .collect()
}

fn cpp_version_triplet_chirho(
    version_chirho: &haskelujah_package_chirho::VersionChirho,
) -> (u32, u32, u32) {
    (
        version_chirho
            .components_chirho
            .first()
            .copied()
            .unwrap_or(0),
        version_chirho
            .components_chirho
            .get(1)
            .copied()
            .unwrap_or(0),
        version_chirho
            .components_chirho
            .get(2)
            .copied()
            .unwrap_or(0),
    )
}

fn cpp_min_version_macro_option_chirho(
    package_name_chirho: &str,
    version_chirho: &haskelujah_package_chirho::VersionChirho,
) -> String {
    let macro_name_chirho = cpp_macro_package_name_chirho(package_name_chirho);
    let (major_chirho, minor_chirho, patch_chirho) = cpp_version_triplet_chirho(version_chirho);
    format!(
        "-DMIN_VERSION_{macro_name_chirho}(x,y,z)=((x)<{major_chirho}||((x)=={major_chirho}&&((y)<{minor_chirho}||((y)=={minor_chirho}&&(z)<={patch_chirho}))))"
    )
}

fn parse_package_name_and_version_from_dir_chirho(
    dir_name_chirho: &str,
) -> Option<(String, haskelujah_package_chirho::VersionChirho)> {
    let split_index_chirho =
        dir_name_chirho
            .char_indices()
            .rev()
            .find_map(|(index_chirho, char_chirho)| {
                if char_chirho != '-' {
                    return None;
                }
                let suffix_chirho = dir_name_chirho.get(index_chirho + 1..)?;
                suffix_chirho
                    .chars()
                    .next()
                    .is_some_and(|suffix_char_chirho| suffix_char_chirho.is_ascii_digit())
                    .then_some(index_chirho)
            })?;
    let package_name_chirho = dir_name_chirho[..split_index_chirho].to_string();
    let version_text_chirho = &dir_name_chirho[split_index_chirho + 1..];
    let version_chirho = haskelujah_package_chirho::parse_version_chirho(version_text_chirho)?;
    Some((package_name_chirho, version_chirho))
}

fn find_enclosing_cabal_dir_chirho(path_chirho: &Path) -> Option<PathBuf> {
    let start_dir_chirho = if path_chirho.is_dir() {
        path_chirho
    } else {
        path_chirho.parent()?
    };
    for ancestor_chirho in start_dir_chirho.ancestors() {
        if find_cabal_in_dir_chirho(ancestor_chirho).is_some() {
            return Some(ancestor_chirho.to_path_buf());
        }
    }
    None
}

fn insert_cpp_package_version_chirho(
    package_versions_chirho: &mut std::collections::BTreeMap<
        String,
        haskelujah_package_chirho::VersionChirho,
    >,
    package_name_chirho: &str,
    version_chirho: haskelujah_package_chirho::VersionChirho,
) {
    match package_versions_chirho.get(package_name_chirho) {
        Some(existing_version_chirho) if existing_version_chirho >= &version_chirho => {}
        _ => {
            package_versions_chirho.insert(package_name_chirho.to_string(), version_chirho);
        }
    }
}

fn seed_cpp_package_versions_chirho()
-> std::collections::BTreeMap<String, haskelujah_package_chirho::VersionChirho> {
    let mut package_versions_chirho = std::collections::BTreeMap::new();
    for (package_name_chirho, version_text_chirho) in [
        ("base", "4.14.0"),
        ("template-haskell", "2.16.0"),
        ("ghc", "8.10.0"),
        ("process", "1.6.0"),
        ("time", "1.15.0"),
    ] {
        if let Some(version_chirho) =
            haskelujah_package_chirho::parse_version_chirho(version_text_chirho)
        {
            insert_cpp_package_version_chirho(
                &mut package_versions_chirho,
                package_name_chirho,
                version_chirho,
            );
        }
    }
    package_versions_chirho
}

fn discover_cpp_package_versions_chirho(
    path_chirho: &Path,
) -> std::collections::BTreeMap<String, haskelujah_package_chirho::VersionChirho> {
    let mut package_versions_chirho = seed_cpp_package_versions_chirho();

    // Canonicalise so that paths with ../../ etc. resolve before walking up
    let canonical_path_chirho = path_chirho
        .canonicalize()
        .unwrap_or_else(|_| path_chirho.to_path_buf());
    if let Some(project_dir_chirho) = canonical_path_chirho.parent() {
        if let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) {
            if let Ok(entries_chirho) = std::fs::read_dir(packages_dir_chirho) {
                for entry_chirho in entries_chirho.flatten() {
                    let package_dir_chirho = entry_chirho.path();
                    if !package_dir_chirho.is_dir() {
                        continue;
                    }
                    let Some(dir_name_chirho) = package_dir_chirho
                        .file_name()
                        .and_then(|file_name_chirho| file_name_chirho.to_str())
                    else {
                        continue;
                    };
                    let Some((package_name_chirho, version_chirho)) =
                        parse_package_name_and_version_from_dir_chirho(dir_name_chirho)
                    else {
                        continue;
                    };
                    insert_cpp_package_version_chirho(
                        &mut package_versions_chirho,
                        &package_name_chirho,
                        version_chirho,
                    );
                }
            }
        }
    }

    if let Some(package_dir_chirho) = find_enclosing_cabal_dir_chirho(path_chirho) {
        if let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) {
            if let Ok(cabal_text_chirho) = std::fs::read_to_string(cabal_path_chirho) {
                let package_chirho =
                    haskelujah_package_chirho::parse_cabal_chirho(&cabal_text_chirho);
                if !package_chirho.name_chirho.is_empty() {
                    let package_version_chirho =
                        package_chirho.version_chirho.clone().or_else(|| {
                            package_dir_chirho
                                .file_name()
                                .and_then(|file_name_chirho| file_name_chirho.to_str())
                                .and_then(parse_package_name_and_version_from_dir_chirho)
                                .map(|(_, version_chirho)| version_chirho)
                        });
                    if let Some(version_chirho) = package_version_chirho {
                        insert_cpp_package_version_chirho(
                            &mut package_versions_chirho,
                            &package_chirho.name_chirho,
                            version_chirho,
                        );
                    }
                }
            }
        }
    }

    package_versions_chirho
}

fn discover_cpp_min_version_macro_options_chirho(path_chirho: &Path) -> Vec<String> {
    // Keep the existing bounded set of per-command flags. There is no shared
    // header: the command never read that file, and concurrent calls rewrote it.
    let versions_chirho = discover_cpp_package_versions_chirho(path_chirho);
    let important_packages_chirho: std::collections::HashSet<&str> = [
        "base",
        "ghc-prim",
        "text",
        "bytestring",
        "containers",
        "deepseq",
        "transformers",
        "mtl",
        "stm",
        "primitive",
        "vector",
        "hashable",
        "template-haskell",
        "random",
        "parsec",
        "array",
        "time",
        "unix",
        "integer-gmp",
        "scientific",
        "filepath",
        "os_string",
        "binary",
        "directory",
        "process",
        "exceptions",
    ]
    .into_iter()
    .collect();
    versions_chirho
        .into_iter()
        .filter(|(name_chirho, _)| important_packages_chirho.contains(name_chirho.as_str()))
        .map(|(name_chirho, ver_chirho)| {
            cpp_min_version_macro_option_chirho(&name_chirho, &ver_chirho)
        })
        .collect()
}

fn cpp_include_dirs_chirho(path_chirho: &Path) -> Vec<PathBuf> {
    let mut include_dirs_chirho = Vec::new();
    if let Some(parent_chirho) = path_chirho.parent() {
        include_dirs_chirho.push(parent_chirho.to_path_buf());
    }

    for ancestor_chirho in path_chirho.ancestors().skip(1) {
        for candidate_dir_name_chirho in ["include", "internal", "autogen"] {
            let include_dir_chirho = ancestor_chirho.join(candidate_dir_name_chirho);
            if include_dir_chirho.is_dir() {
                include_dirs_chirho.push(include_dir_chirho);
            }
        }
        let dist_autogen_dir_chirho = ancestor_chirho.join("dist/build/autogen");
        if dist_autogen_dir_chirho.is_dir() {
            include_dirs_chirho.push(dist_autogen_dir_chirho);
        }
        if find_cabal_in_dir_chirho(ancestor_chirho).is_some() {
            include_dirs_chirho.push(ancestor_chirho.to_path_buf());
        }
    }

    include_dirs_chirho.sort();
    include_dirs_chirho.dedup();
    include_dirs_chirho
}

fn source_uses_cpp_chirho(source_chirho: &str) -> bool {
    source_chirho
        .lines()
        .take(64)
        .any(line_uses_cpp_pragma_chirho)
}

fn line_uses_cpp_pragma_chirho(line_chirho: &str) -> bool {
    let folded_line_chirho = line_chirho.to_ascii_uppercase();
    (folded_line_chirho.contains("{-#")
        && folded_line_chirho.contains("LANGUAGE")
        && folded_line_chirho.contains("CPP"))
        || (folded_line_chirho.contains("{-#")
            && folded_line_chirho.contains("OPTIONS_GHC")
            && folded_line_chirho.contains("-CPP"))
}

fn path_is_hsc_chirho(path_chirho: &Path) -> bool {
    path_chirho
        .extension()
        .and_then(|ext_chirho| ext_chirho.to_str())
        .is_some_and(|ext_chirho| ext_chirho.eq_ignore_ascii_case("hsc"))
}

fn decode_cpp_stdout_chirho(path_chirho: &Path, stdout_chirho: Vec<u8>) -> io::Result<String> {
    String::from_utf8(stdout_chirho).map_err(|error_chirho| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "cpp output for {} was not valid UTF-8: {}",
                path_chirho.display(),
                error_chirho
            ),
        )
    })
}

fn sanitize_hsc_source_chirho(source_chirho: &str) -> String {
    let mut sanitized_chirho = String::with_capacity(source_chirho.len());
    let mut cursor_chirho = 0usize;

    while let Some(rel_start_chirho) = source_chirho[cursor_chirho..].find("#{") {
        let start_chirho = cursor_chirho + rel_start_chirho;
        sanitized_chirho.push_str(&source_chirho[cursor_chirho..start_chirho]);
        let body_start_chirho = start_chirho + 2;
        let Some(rel_end_chirho) = source_chirho[body_start_chirho..].find('}') else {
            sanitized_chirho.push_str(&source_chirho[start_chirho..]);
            return sanitized_chirho;
        };
        let end_chirho = body_start_chirho + rel_end_chirho;
        let body_chirho = source_chirho[body_start_chirho..end_chirho].trim();
        let replacement_chirho = if body_chirho.starts_with("type ") {
            "CInt"
        } else if body_chirho.starts_with("size ")
            || body_chirho.starts_with("alignment ")
            || body_chirho.starts_with("const ")
        {
            "0"
        } else if body_chirho.starts_with("peek ") {
            "peek"
        } else if body_chirho.starts_with("poke ") {
            "poke"
        } else if body_chirho.starts_with("ptr ") {
            "nullPtr"
        } else {
            "undefined"
        };
        sanitized_chirho.push_str(replacement_chirho);
        cursor_chirho = end_chirho + 1;
    }

    sanitized_chirho.push_str(&source_chirho[cursor_chirho..]);
    sanitized_chirho
}

fn strip_hsc_include_directives_chirho(source_chirho: &str) -> String {
    source_chirho
        .lines()
        .filter(|line_chirho| {
            let trimmed_chirho = line_chirho.trim_start();
            let directive_chirho = trimmed_chirho
                .strip_prefix('#')
                .map(|rest_chirho| rest_chirho.trim_start())
                .unwrap_or_default();
            !(directive_chirho.starts_with("include") || directive_chirho.starts_with("let"))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn preprocess_hsc_source_with_options_chirho(
    path_chirho: &Path,
    source_chirho: &str,
    extra_cpp_options_chirho: &[String],
) -> io::Result<String> {
    let stripped_source_chirho = strip_hsc_include_directives_chirho(source_chirho);
    let sanitized_source_chirho = sanitize_hsc_source_chirho(&stripped_source_chirho);
    let temporary_chirho =
        temporary_source_file_chirho(&sanitized_source_chirho, path_chirho.parent(), ".hsc")?;
    let output_chirho =
        configure_cpp_command_chirho(temporary_chirho.path(), true, extra_cpp_options_chirho)?
            .output_chirho()?;
    if !output_chirho.status.success() {
        let stderr_chirho = String::from_utf8_lossy(&output_chirho.stderr)
            .trim()
            .to_string();
        return Err(io::Error::other(format!(
            "hsc preprocessing failed for {}: {}",
            path_chirho.display(),
            stderr_chirho
        )));
    }
    let preprocessed_chirho = decode_cpp_stdout_chirho(path_chirho, output_chirho.stdout)?;
    Ok(sanitize_hsc_source_chirho(&preprocessed_chirho))
}

/// Placeholder used to protect Haskell pragma close tokens (`#-}`) from the
/// C preprocessor, which would otherwise reject them as invalid directives.
const PRAGMA_CLOSE_PLACEHOLDER_CHIRHO: &str = "HASKELUJAH_PRAGMA_CLOSE_CHIRHO";

/// Returns `true` when the source contains `#-}` that would confuse `cpp`.
fn source_has_pragma_close_chirho(source_chirho: &str) -> bool {
    source_chirho.lines().any(|line_chirho| {
        let trimmed_chirho = line_chirho.trim_start();
        trimmed_chirho == "#-}" || trimmed_chirho.starts_with("#-}")
    })
}

/// Replace `#-}` lines with a safe placeholder before handing to `cpp`.
fn protect_pragma_close_chirho(source_chirho: &str) -> String {
    source_chirho
        .lines()
        .map(|line_chirho| {
            let trimmed_chirho = line_chirho.trim_start();
            if trimmed_chirho == "#-}" || trimmed_chirho.starts_with("#-}") {
                line_chirho.replace("#-}", PRAGMA_CLOSE_PLACEHOLDER_CHIRHO)
            } else {
                line_chirho.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Restore `#-}` from the placeholder in cpp output.
fn restore_pragma_close_chirho(source_chirho: &str) -> String {
    source_chirho.replace(PRAGMA_CLOSE_PLACEHOLDER_CHIRHO, "#-}")
}

fn preprocess_cpp_source_with_options_chirho(
    path_chirho: &Path,
    source_chirho: &str,
    extra_cpp_options_chirho: &[String],
) -> io::Result<String> {
    if path_is_hsc_chirho(path_chirho) {
        return preprocess_hsc_source_with_options_chirho(
            path_chirho,
            source_chirho,
            extra_cpp_options_chirho,
        );
    }

    if !source_uses_cpp_chirho(source_chirho) {
        return Ok(source_chirho.to_string());
    }

    // If the file contains #-} (Haskell pragma close), we must write a
    // sanitised temp file so the C preprocessor does not choke on it.
    let needs_pragma_close_fix_chirho = source_has_pragma_close_chirho(source_chirho);
    let temporary_chirho = if needs_pragma_close_fix_chirho {
        Some(temporary_source_file_chirho(
            &protect_pragma_close_chirho(source_chirho),
            path_chirho.parent(),
            ".hs",
        )?)
    } else {
        None
    };
    let cpp_path_chirho = temporary_chirho
        .as_ref()
        .map(|file_chirho| file_chirho.path())
        .unwrap_or(path_chirho);
    let output_chirho =
        configure_cpp_command_chirho(cpp_path_chirho, true, extra_cpp_options_chirho)?
            .output_chirho()?;
    if !output_chirho.status.success() {
        let stderr_chirho = String::from_utf8_lossy(&output_chirho.stderr)
            .trim()
            .to_string();
        return Err(io::Error::other(format!(
            "cpp preprocessing failed for {}: {}",
            path_chirho.display(),
            stderr_chirho
        )));
    }

    let mut result_chirho = decode_cpp_stdout_chirho(path_chirho, output_chirho.stdout)?;
    if needs_pragma_close_fix_chirho {
        result_chirho = restore_pragma_close_chirho(&result_chirho);
    }
    Ok(result_chirho)
}

fn preprocess_cpp_source_chirho(path_chirho: &Path, source_chirho: &str) -> io::Result<String> {
    preprocess_cpp_source_with_options_chirho(path_chirho, source_chirho, &[])
}

fn cpp_directive_head_for_line_chirho(line_chirho: &str) -> Option<&str> {
    let trimmed_chirho = line_chirho.trim_start();
    let directive_text_chirho = trimmed_chirho.strip_prefix('#')?.trim_start();
    let directive_head_chirho = directive_text_chirho
        .split_whitespace()
        .next()
        .unwrap_or_default();
    if directive_head_chirho.is_empty()
        || directive_head_chirho
            .chars()
            .all(|char_chirho| char_chirho.is_ascii_digit())
        || matches!(
            directive_head_chirho,
            "if" | "ifdef"
                | "ifndef"
                | "else"
                | "elif"
                | "endif"
                | "define"
                | "undef"
                | "include"
                | "line"
                | "pragma"
        )
    {
        Some(directive_head_chirho)
    } else {
        None
    }
}

fn has_cpp_directive_residue_chirho(source_chirho: &str) -> bool {
    source_chirho
        .lines()
        .any(|line_chirho| cpp_directive_head_for_line_chirho(line_chirho).is_some())
}

fn strip_cpp_directives_chirho(source_chirho: &str) -> String {
    #[derive(Clone, Copy)]
    struct CppStripFrameChirho {
        parent_active_chirho: bool,
    }

    let mut output_lines_chirho = Vec::new();
    let mut conditional_stack_chirho: Vec<CppStripFrameChirho> = Vec::new();
    let mut active_branch_chirho = true;

    for line_chirho in source_chirho.lines() {
        if let Some(directive_head_chirho) = cpp_directive_head_for_line_chirho(line_chirho) {
            match directive_head_chirho {
                "if" | "ifdef" | "ifndef" => {
                    conditional_stack_chirho.push(CppStripFrameChirho {
                        parent_active_chirho: active_branch_chirho,
                    });
                }
                "else" | "elif" => {
                    active_branch_chirho = false;
                }
                "endif" => {
                    active_branch_chirho = conditional_stack_chirho
                        .pop()
                        .map(|frame_chirho| frame_chirho.parent_active_chirho)
                        .unwrap_or(true);
                }
                _ => {}
            }
            output_lines_chirho.push(String::new());
            continue;
        }

        if active_branch_chirho {
            output_lines_chirho.push(line_chirho.to_string());
        } else {
            output_lines_chirho.push(String::new());
        }
    }

    output_lines_chirho.join("\n")
}

fn is_maybe_absent_unboxed_sum_alt_chirho(alt_chirho: &str) -> bool {
    matches!(alt_chirho.trim(), "" | "(# #)" | "(##)")
}

fn maybe_like_unboxed_sum_parts_chirho(
    line_chirho: &str,
    start_chirho: usize,
) -> Option<(usize, &str, &str)> {
    if !line_chirho[start_chirho..].starts_with("(#") {
        return None;
    }

    let bytes_chirho = line_chirho.as_bytes();
    let mut idx_chirho = start_chirho + 2;
    let mut depth_chirho = 1usize;
    let mut pipe_idx_chirho = None;

    while idx_chirho + 1 < bytes_chirho.len() {
        if line_chirho[idx_chirho..].starts_with("(#") {
            depth_chirho += 1;
            idx_chirho += 2;
            continue;
        }
        if line_chirho[idx_chirho..].starts_with("#)") {
            depth_chirho = depth_chirho.saturating_sub(1);
            if depth_chirho == 0 {
                let pipe_idx_chirho = pipe_idx_chirho?;
                let body_start_chirho = start_chirho + 2;
                let body_end_chirho = idx_chirho;
                let left_chirho = &line_chirho[body_start_chirho..pipe_idx_chirho];
                let right_chirho = &line_chirho[pipe_idx_chirho + 1..body_end_chirho];
                return Some((idx_chirho + 2, left_chirho, right_chirho));
            }
            idx_chirho += 2;
            continue;
        }
        if depth_chirho == 1 && bytes_chirho[idx_chirho] == b'|' {
            pipe_idx_chirho = Some(idx_chirho);
        }
        idx_chirho += 1;
    }

    None
}

fn rewrite_maybe_like_unboxed_sum_line_chirho(
    line_chirho: &str,
    type_context_chirho: bool,
) -> String {
    if !line_chirho.contains("(#") || !line_chirho.contains('|') {
        return line_chirho.to_string();
    }

    let mut rewritten_chirho = String::with_capacity(line_chirho.len());
    let mut cursor_chirho = 0usize;

    while let Some(rel_start_chirho) = line_chirho[cursor_chirho..].find("(#") {
        let start_chirho = cursor_chirho + rel_start_chirho;
        rewritten_chirho.push_str(&line_chirho[cursor_chirho..start_chirho]);

        let Some((end_chirho, left_alt_chirho, right_alt_chirho)) =
            maybe_like_unboxed_sum_parts_chirho(line_chirho, start_chirho)
        else {
            rewritten_chirho.push_str("(#");
            cursor_chirho = start_chirho + 2;
            continue;
        };

        let left_trimmed_chirho = left_alt_chirho.trim();
        let right_trimmed_chirho = right_alt_chirho.trim();

        let replacement_chirho = if type_context_chirho {
            if is_maybe_absent_unboxed_sum_alt_chirho(left_trimmed_chirho)
                && !right_trimmed_chirho.is_empty()
            {
                Some(format!("Maybe {right_trimmed_chirho}"))
            } else {
                None
            }
        } else if is_maybe_absent_unboxed_sum_alt_chirho(left_trimmed_chirho) {
            if right_trimmed_chirho.is_empty() {
                Some("(Nothing)".to_string())
            } else {
                Some(format!("(Just {right_trimmed_chirho})"))
            }
        } else {
            None
        };

        if let Some(replacement_chirho) = replacement_chirho {
            rewritten_chirho.push_str(&replacement_chirho);
            cursor_chirho = end_chirho;
        } else {
            rewritten_chirho.push_str(&line_chirho[start_chirho..end_chirho]);
            cursor_chirho = end_chirho;
        }
    }

    rewritten_chirho.push_str(&line_chirho[cursor_chirho..]);
    rewritten_chirho
}

fn lower_maybe_like_unboxed_sums_chirho(source_chirho: &str) -> String {
    if !source_chirho.contains("(#") || !source_chirho.contains('|') {
        return source_chirho.to_string();
    }

    source_chirho
        .lines()
        .map(|line_chirho| {
            let type_rewritten_chirho = if line_chirho.contains("::") {
                rewrite_maybe_like_unboxed_sum_line_chirho(line_chirho, true)
            } else {
                line_chirho.to_string()
            };
            rewrite_maybe_like_unboxed_sum_line_chirho(&type_rewritten_chirho, false)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_haskell_source_file_chirho(path_chirho: impl AsRef<Path>) -> io::Result<String> {
    let path_ref_chirho = path_chirho.as_ref();
    let source_chirho = std::fs::read_to_string(path_ref_chirho)?;
    // Try CPP preprocessing; fall back to raw source (with directives
    // stripped) if cpp fails (e.g., tick characters in Haskell identifiers).
    match preprocess_cpp_source_chirho(path_ref_chirho, &source_chirho) {
        Ok(processed_chirho) => {
            let cleaned_chirho = if has_cpp_directive_residue_chirho(&processed_chirho) {
                strip_cpp_directives_chirho(&processed_chirho)
            } else {
                processed_chirho
            };
            Ok(lower_maybe_like_unboxed_sums_chirho(&cleaned_chirho))
        }
        Err(_) if path_is_hsc_chirho(path_ref_chirho) => Ok(sanitize_hsc_source_chirho(
            &lower_maybe_like_unboxed_sums_chirho(&strip_cpp_directives_chirho(&source_chirho)),
        )),
        Err(_) => Ok(lower_maybe_like_unboxed_sums_chirho(
            &strip_cpp_directives_chirho(&source_chirho),
        )),
    }
}

fn is_placeholder_import_scheme_chirho(
    scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
) -> bool {
    if !scheme_chirho.preds_chirho.is_empty() || scheme_chirho.vars_chirho.len() != 1 {
        return false;
    }
    match &scheme_chirho.ty_chirho {
        haskelujah_typing_chirho::TyChirho::VarChirho(var_chirho) => {
            *var_chirho == scheme_chirho.vars_chirho[0] && var_chirho.0 >= 9000
        }
        _ => false,
    }
}

/// The scheme a qualified import gives `name_chirho` when the interface it selected has no
/// `module.name` key (row 608). Only an interface proven builtin reads the generated GHC agreement
/// table. A source module's interface, whatever its name, and any interface of unknown origin
/// keep today's placeholder.
/// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
pub fn qualified_import_alias_scheme_chirho(
    iface_chirho: &ModuleIfaceChirho,
    name_chirho: &str,
    builtins_chirho: &std::collections::HashMap<String, haskelujah_typing_chirho::SchemeChirho>,
) -> Option<haskelujah_typing_chirho::SchemeChirho> {
    if iface_chirho.exports_chirho.origin_chirho
        != haskelujah_naming_chirho::IfaceOriginChirho::BuiltinChirho
    {
        return None;
    }
    haskelujah_typing_chirho::infer_chirho::qualified_builtin_alias_scheme_chirho(
        &iface_chirho.name_chirho,
        name_chirho,
        builtins_chirho,
    )
}

fn should_override_imported_scheme_chirho(
    existing_scheme_chirho: Option<&haskelujah_typing_chirho::SchemeChirho>,
    new_scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
) -> bool {
    match existing_scheme_chirho {
        None => true,
        Some(existing_scheme_chirho) => {
            is_placeholder_import_scheme_chirho(existing_scheme_chirho)
                && !is_placeholder_import_scheme_chirho(new_scheme_chirho)
        }
    }
}

fn collect_local_type_names_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashSet<String> {
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
            | DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect()
}

fn default_safe_unqualified_imported_type_names_chirho() -> std::collections::HashSet<String> {
    // These names are already modeled as shared runtime-facing types across
    // multiple re-export modules, so keeping them bare avoids spurious
    // mismatches like B.ByteString vs ByteString, I.IORef vs IORef, and
    // E.SomeException vs SomeException.
    ["ByteString", "Ordering", "IORef", "SomeException"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn collect_safe_unqualified_imported_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashSet<String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut safe_type_names_chirho = default_safe_unqualified_imported_type_names_chirho();
    safe_type_names_chirho.retain(|name_chirho| !local_type_names_chirho.contains(name_chirho));
    safe_type_names_chirho.extend(
        module_chirho
            .imports_chirho
            .iter()
            .filter(|import_chirho| !import_chirho.qualified_chirho)
            .flat_map(|import_chirho| {
                let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
                ifaces_chirho
                    .iter()
                    .rev()
                    .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
                    .into_iter()
                    .flat_map(|iface_chirho| {
                        haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                            &iface_chirho.exports_chirho,
                            &import_chirho.spec_chirho,
                        )
                    })
            })
            .filter_map(|(name_chirho, namespace_chirho, _span_chirho)| {
                (namespace_chirho
                    == haskelujah_naming_chirho::env_chirho::NamespaceChirho::TypeChirho
                    && !local_type_names_chirho.contains(&name_chirho))
                .then_some(name_chirho)
            }),
    );
    safe_type_names_chirho
}

fn collect_preferred_qualified_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashMap<String, String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut exporters_by_type_name_chirho: std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    > = std::collections::HashMap::new();

    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        else {
            continue;
        };
        for exported_type_name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
            if local_type_names_chirho.contains(exported_type_name_chirho) {
                continue;
            }
            exporters_by_type_name_chirho
                .entry(exported_type_name_chirho.clone())
                .or_default()
                .insert(module_name_chirho.clone());
        }
    }

    exporters_by_type_name_chirho
        .into_iter()
        .filter_map(|(type_name_chirho, exporters_chirho)| {
            (exporters_chirho.len() == 1).then(|| {
                (
                    type_name_chirho,
                    exporters_chirho
                        .into_iter()
                        .next()
                        .expect("single exporter should exist"),
                )
            })
        })
        .collect()
}

fn qualify_imported_ast_type_chirho(
    ty_chirho: &TypeChirho,
    qualifiable_type_names_chirho: &std::collections::HashSet<String>,
    qualifier_chirho: &str,
    unqualified_type_names_chirho: &std::collections::HashSet<String>,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) -> TypeChirho {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => {
            let bare_name_chirho = name_chirho.text_chirho();
            let should_canonicalize_unqualified_chirho = unqualified_type_names_chirho
                .contains(bare_name_chirho)
                || preferred_qualified_type_names_chirho.contains_key(bare_name_chirho);
            if should_canonicalize_unqualified_chirho {
                TypeChirho::ConChirho(haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                        bare_name_chirho.to_string(),
                        name_chirho.span_chirho(),
                    ),
                ))
            } else if qualifiable_type_names_chirho.contains(bare_name_chirho) {
                let span_chirho = name_chirho.span_chirho();
                TypeChirho::ConChirho(haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                        qualifier_chirho.to_string(),
                        bare_name_chirho.to_string(),
                        span_chirho,
                    ),
                ))
            } else {
                ty_chirho.clone()
            }
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            span_chirho,
        } => TypeChirho::AppChirho {
            fun_chirho: Box::new(qualify_imported_ast_type_chirho(
                fun_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            arg_chirho: Box::new(qualify_imported_ast_type_chirho(
                arg_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::FunChirho {
            arg_chirho,
            mult_chirho,
            result_chirho,
            span_chirho,
        } => TypeChirho::FunChirho {
            arg_chirho: Box::new(qualify_imported_ast_type_chirho(
                arg_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            mult_chirho: mult_chirho.clone(),
            result_chirho: Box::new(qualify_imported_ast_type_chirho(
                result_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::TupleChirho {
            elements_chirho,
            span_chirho,
        } => TypeChirho::TupleChirho {
            elements_chirho: elements_chirho
                .iter()
                .map(|element_chirho| {
                    qualify_imported_ast_type_chirho(
                        element_chirho,
                        qualifiable_type_names_chirho,
                        qualifier_chirho,
                        unqualified_type_names_chirho,
                        preferred_qualified_type_names_chirho,
                    )
                })
                .collect(),
            span_chirho: *span_chirho,
        },
        TypeChirho::ListChirho {
            element_chirho,
            span_chirho,
        } => TypeChirho::ListChirho {
            element_chirho: Box::new(qualify_imported_ast_type_chirho(
                element_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::ParenChirho {
            inner_chirho,
            span_chirho,
        } => TypeChirho::ParenChirho {
            inner_chirho: Box::new(qualify_imported_ast_type_chirho(
                inner_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            span_chirho,
        } => TypeChirho::QualChirho {
            context_chirho: context_chirho.clone(),
            body_chirho: Box::new(qualify_imported_ast_type_chirho(
                body_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            span_chirho,
        } => TypeChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(qualify_imported_ast_type_chirho(
                body_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
            span_chirho: *span_chirho,
        },
        _ => ty_chirho.clone(),
    }
}

fn qualify_imported_ty_chirho(
    ty_chirho: &haskelujah_typing_chirho::TyChirho,
    qualifiable_type_names_chirho: &std::collections::HashSet<String>,
    qualifier_chirho: &str,
    unqualified_type_names_chirho: &std::collections::HashSet<String>,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) -> haskelujah_typing_chirho::TyChirho {
    match ty_chirho {
        haskelujah_typing_chirho::TyChirho::ConChirho(name_chirho) => {
            let bare_name_chirho = name_chirho
                .rsplit('.')
                .next()
                .unwrap_or(name_chirho.as_str());
            if unqualified_type_names_chirho.contains(bare_name_chirho)
                || preferred_qualified_type_names_chirho.contains_key(bare_name_chirho)
            {
                haskelujah_typing_chirho::TyChirho::ConChirho(bare_name_chirho.to_string())
            } else if qualifiable_type_names_chirho.contains(bare_name_chirho) {
                haskelujah_typing_chirho::TyChirho::ConChirho(format!(
                    "{qualifier_chirho}.{bare_name_chirho}"
                ))
            } else {
                haskelujah_typing_chirho::TyChirho::ConChirho(name_chirho.clone())
            }
        }
        haskelujah_typing_chirho::TyChirho::AppChirho(fun_chirho, arg_chirho) => {
            haskelujah_typing_chirho::TyChirho::AppChirho(
                Box::new(qualify_imported_ty_chirho(
                    fun_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
                Box::new(qualify_imported_ty_chirho(
                    arg_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
            )
        }
        haskelujah_typing_chirho::TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => {
            haskelujah_typing_chirho::TyChirho::FunChirho(
                Box::new(qualify_imported_ty_chirho(
                    arg_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
                Box::new(qualify_imported_ty_chirho(
                    result_chirho,
                    qualifiable_type_names_chirho,
                    qualifier_chirho,
                    unqualified_type_names_chirho,
                    preferred_qualified_type_names_chirho,
                )),
                *mult_chirho,
            )
        }
        haskelujah_typing_chirho::TyChirho::TupleChirho(elements_chirho) => {
            haskelujah_typing_chirho::TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        qualify_imported_ty_chirho(
                            element_chirho,
                            qualifiable_type_names_chirho,
                            qualifier_chirho,
                            unqualified_type_names_chirho,
                            preferred_qualified_type_names_chirho,
                        )
                    })
                    .collect(),
            )
        }
        haskelujah_typing_chirho::TyChirho::ListChirho(element_chirho) => {
            haskelujah_typing_chirho::TyChirho::ListChirho(Box::new(qualify_imported_ty_chirho(
                element_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )))
        }
        haskelujah_typing_chirho::TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => haskelujah_typing_chirho::TyChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(qualify_imported_ty_chirho(
                body_chirho,
                qualifiable_type_names_chirho,
                qualifier_chirho,
                unqualified_type_names_chirho,
                preferred_qualified_type_names_chirho,
            )),
        },
        other_chirho => other_chirho.clone(),
    }
}

fn qualify_imported_scheme_for_iface_chirho(
    scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
    qualifiable_type_names_chirho: &std::collections::HashSet<String>,
    qualifier_chirho: &str,
    unqualified_type_names_chirho: &std::collections::HashSet<String>,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) -> haskelujah_typing_chirho::SchemeChirho {
    if qualifiable_type_names_chirho.is_empty() {
        return scheme_chirho.clone();
    }
    haskelujah_typing_chirho::SchemeChirho {
        vars_chirho: scheme_chirho.vars_chirho.clone(),
        preds_chirho: scheme_chirho
            .preds_chirho
            .iter()
            .map(
                |pred_chirho| haskelujah_typing_chirho::ty_chirho::SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho.clone(),
                    ty_chirho: qualify_imported_ty_chirho(
                        &pred_chirho.ty_chirho,
                        qualifiable_type_names_chirho,
                        qualifier_chirho,
                        unqualified_type_names_chirho,
                        preferred_qualified_type_names_chirho,
                    ),
                    extra_tys_chirho: pred_chirho
                        .extra_tys_chirho
                        .iter()
                        .map(|ty_chirho| {
                            qualify_imported_ty_chirho(
                                ty_chirho,
                                qualifiable_type_names_chirho,
                                qualifier_chirho,
                                unqualified_type_names_chirho,
                                preferred_qualified_type_names_chirho,
                            )
                        })
                        .collect(),
                },
            )
            .collect(),
        ty_chirho: qualify_imported_ty_chirho(
            &scheme_chirho.ty_chirho,
            qualifiable_type_names_chirho,
            qualifier_chirho,
            unqualified_type_names_chirho,
            preferred_qualified_type_names_chirho,
        ),
    }
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

pub fn run_frontend_with_type_synonyms_families_and_class_env_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    imported_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    >,
    imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
    imported_type_families_chirho: &ImportedTypeFamiliesChirho,
    imported_class_env_chirho: &haskelujah_typing_chirho::class_chirho::ClassEnvChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    // Check for -fdefer-type-errors / -fdefer-out-of-scope-variables
    // These GHC flags cause type errors to be deferred as warnings.
    let defer_errors_chirho = source_chirho.contains("-fdefer-type-errors")
        || source_chirho.contains("-fdefer-out-of-scope-variables");

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

    // Phase 2.2: Template Haskell splice expansion
    // Process SpliceDeclChirho entries before name resolution so that
    // generated declarations participate in the normal compilation pipeline.
    let splice_result_chirho = splice_chirho::expand_splices_chirho(
        std::mem::take(&mut module_chirho.decls_chirho),
        &mut module_chirho.origin_supply_chirho,
    );
    module_chirho.decls_chirho = splice_result_chirho.decls_chirho;
    let splice_warnings_chirho = splice_result_chirho.warnings_chirho;

    // Phase 2.5: Deriving — generate instance declarations for `deriving` clauses
    let deriving_warnings_chirho =
        haskelujah_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);

    // Phase 2.6: Do-statement failability — clear the `fail` lowering RESERVED on
    // every bind wherever the pattern cannot fail, so what survives is a
    // selection that was really selected. Runs here because the module's own
    // declarations are final after splices and deriving, and it needs to know how
    // many constructors a pattern's type has. A constructor from another module
    // is not in this environment and stays failable, which is the safe direction.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    haskelujah_typing_chirho::do_failability_chirho::refine_do_selections_chirho(
        &mut module_chirho,
    );

    // Phase 3: Name resolution
    let resolve_result_chirho = resolve_module_with_imports_chirho(&module_chirho, ifaces_chirho);
    if !defer_errors_chirho && resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(resolve_result_chirho.diagnostics_chirho);
    }

    // Phase 3.1: Orphan instance detection
    let orphan_warnings_chirho =
        haskelujah_naming_chirho::check_orphan_instances_chirho(&module_chirho);

    // Phase 3.5: Kind inference
    let kind_result_chirho = haskelujah_typing_chirho::infer_module_kinds_chirho(&module_chirho);
    if !defer_errors_chirho && kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(kind_result_chirho.diagnostics_chirho);
    }

    // Phase 3.9: Seed the type environment with placeholder types for
    // imported values that don't have explicit type schemes. This allows
    // the type checker to see imported names as polymorphic variables
    // rather than reporting them as "unbound variable" (E0202).
    let mut merged_imported_types_chirho = imported_types_chirho.clone();
    let mut merged_imported_type_synonyms_chirho = imported_type_synonyms_chirho.clone();
    let merged_imported_type_families_chirho = imported_type_families_chirho.clone();
    let mut merged_imported_record_field_names_chirho: std::collections::HashMap<
        String,
        Vec<String>,
    > = std::collections::HashMap::new();
    let builtin_value_schemes_chirho =
        haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho();
    for (builtin_name_chirho, builtin_scheme_chirho) in &builtin_value_schemes_chirho {
        merged_imported_types_chirho
            .entry(builtin_name_chirho.clone())
            .or_insert_with(|| builtin_scheme_chirho.clone());
    }
    let safe_unqualified_imported_type_names_chirho =
        collect_safe_unqualified_imported_type_names_chirho(&module_chirho, ifaces_chirho);
    let preferred_qualified_type_names_chirho =
        collect_preferred_qualified_type_names_chirho(&module_chirho, ifaces_chirho);
    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        if let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|m_chirho| m_chirho.name_chirho == module_name_chirho)
        {
            let qualifiable_type_names_chirho: std::collections::HashSet<String> = iface_chirho
                .exports_chirho
                .types_chirho
                .keys()
                .cloned()
                .chain(
                    imported_type_synonyms_chirho
                        .keys()
                        .filter_map(|name_chirho| {
                            name_chirho
                                .strip_prefix(&format!("{module_name_chirho}."))
                                .map(|suffix_chirho| suffix_chirho.to_string())
                        }),
                )
                .collect();
            // Collect which names this import brings in.
            let names_chirho =
                haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                    &iface_chirho.exports_chirho,
                    &import_chirho.spec_chirho,
                );
            let qualifier_chirho = import_chirho
                .alias_chirho
                .as_ref()
                .map(|a_chirho| a_chirho.text_chirho().to_string())
                .unwrap_or_else(|| module_name_chirho.clone());
            if !import_chirho.qualified_chirho {
                for type_info_chirho in iface_chirho.exports_chirho.types_chirho.values() {
                    if type_info_chirho.methods_chirho.is_empty()
                        || type_info_chirho.constructors_chirho.is_empty()
                    {
                        continue;
                    }
                    for constructor_name_chirho in &type_info_chirho.constructors_chirho {
                        merged_imported_record_field_names_chirho
                            .entry(constructor_name_chirho.clone())
                            .or_insert_with(|| type_info_chirho.methods_chirho.clone());
                        merged_imported_record_field_names_chirho
                            .entry(format!(
                                "{}.{}",
                                module_name_chirho, constructor_name_chirho
                            ))
                            .or_insert_with(|| type_info_chirho.methods_chirho.clone());
                        if qualifier_chirho != module_name_chirho {
                            merged_imported_record_field_names_chirho
                                .entry(format!("{}.{}", qualifier_chirho, constructor_name_chirho))
                                .or_insert_with(|| type_info_chirho.methods_chirho.clone());
                        }
                    }
                }
            }
            for (name_chirho, ns_chirho, _span_chirho) in &names_chirho {
                if ns_chirho == &haskelujah_naming_chirho::env_chirho::NamespaceChirho::TypeChirho {
                    let module_qualified_name_chirho =
                        format!("{module_name_chirho}.{name_chirho}");
                    if let Some((params_chirho, rhs_chirho)) = imported_type_synonyms_chirho
                        .get(&module_qualified_name_chirho)
                        .cloned()
                        .or_else(|| imported_type_synonyms_chirho.get(name_chirho).cloned())
                    {
                        let rewritten_rhs_chirho = qualify_imported_ast_type_chirho(
                            &rhs_chirho,
                            &qualifiable_type_names_chirho,
                            &qualifier_chirho,
                            &safe_unqualified_imported_type_names_chirho,
                            &preferred_qualified_type_names_chirho,
                        );
                        if !import_chirho.qualified_chirho {
                            merged_imported_type_synonyms_chirho.insert(
                                name_chirho.clone(),
                                (params_chirho.clone(), rewritten_rhs_chirho.clone()),
                            );
                        }
                        merged_imported_type_synonyms_chirho.insert(
                            format!("{qualifier_chirho}.{name_chirho}"),
                            (params_chirho, rewritten_rhs_chirho),
                        );
                    }
                }
            }
            for (name_chirho, ns_chirho, _span_chirho) in &names_chirho {
                if ns_chirho == &haskelujah_naming_chirho::env_chirho::NamespaceChirho::ValueChirho
                {
                    let module_qualified_name_chirho =
                        format!("{module_name_chirho}.{name_chirho}");
                    let base_scheme_chirho = merged_imported_types_chirho
                        .get(&module_qualified_name_chirho)
                        .cloned()
                        .or_else(|| {
                            if import_chirho.qualified_chirho {
                                // Row 608: a builtin alias takes its bare scheme only where
                                // GHC 9.14.1 types the export exactly so; otherwise the placeholder.
                                // workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
                                qualified_import_alias_scheme_chirho(
                                    iface_chirho,
                                    name_chirho,
                                    &builtin_value_schemes_chirho,
                                )
                            } else {
                                merged_imported_types_chirho.get(name_chirho).cloned()
                            }
                        })
                        .unwrap_or_else(|| {
                            // Assign a fully polymorphic type: forall a. a
                            // This allows the type checker to accept the name
                            // without knowing the precise type.
                            let fresh_var_chirho = haskelujah_typing_chirho::TyVarChirho(
                                9000 + merged_imported_types_chirho.len() as u32,
                            );
                            haskelujah_typing_chirho::SchemeChirho {
                                vars_chirho: vec![fresh_var_chirho],
                                preds_chirho: vec![],
                                ty_chirho: haskelujah_typing_chirho::TyChirho::VarChirho(
                                    fresh_var_chirho,
                                ),
                            }
                        });
                    let in_scope_seed_scheme_chirho = qualify_imported_scheme_for_iface_chirho(
                        &base_scheme_chirho,
                        &qualifiable_type_names_chirho,
                        &qualifier_chirho,
                        &safe_unqualified_imported_type_names_chirho,
                        &preferred_qualified_type_names_chirho,
                    );
                    if !import_chirho.qualified_chirho {
                        // Current-module imports should win over broadly seeded
                        // dependency names. Otherwise a previously compiled
                        // module can pin an unqualified name like `choice` or
                        // `pack` to the wrong specialized scheme in later
                        // modules that explicitly import a different source.
                        merged_imported_types_chirho
                            .insert(name_chirho.clone(), in_scope_seed_scheme_chirho.clone());
                    }
                    let qualified_name_chirho = format!("{qualifier_chirho}.{name_chirho}");
                    if should_override_imported_scheme_chirho(
                        merged_imported_types_chirho.get(&qualified_name_chirho),
                        &in_scope_seed_scheme_chirho,
                    ) {
                        merged_imported_types_chirho
                            .insert(qualified_name_chirho, in_scope_seed_scheme_chirho);
                    }
                }
            }
        }
    }

    // Phase 4: Type inference — use import-aware variant when upstream
    // type schemes are available, plain variant otherwise.
    let infer_result_chirho = if merged_imported_types_chirho.is_empty()
        && merged_imported_type_synonyms_chirho.is_empty()
        && merged_imported_type_families_chirho.is_empty()
        && merged_imported_record_field_names_chirho.is_empty()
        && imported_class_env_chirho.classes_chirho.is_empty()
        && imported_class_env_chirho.instances_chirho.is_empty()
    {
        infer_module_chirho(&module_chirho)
    } else {
        infer_module_with_imports_type_synonyms_families_and_class_env_chirho(
            &module_chirho,
            &merged_imported_types_chirho,
            &merged_imported_type_synonyms_chirho,
            &merged_imported_type_families_chirho,
            imported_class_env_chirho,
            &merged_imported_record_field_names_chirho,
            &safe_unqualified_imported_type_names_chirho,
            &preferred_qualified_type_names_chirho,
        )
    };
    if !defer_errors_chirho && infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(infer_result_chirho.diagnostics_chirho);
    }

    // Phase 4.4: Source-type validity — reject type forms whose licensing
    // extension is not enabled (GHC's GHC-91510: illegal polymorphic /
    // qualified type).
    let validity_result_chirho =
        haskelujah_typing_chirho::check_module_type_validity_diagnostics_chirho(&module_chirho);
    if !defer_errors_chirho
        && validity_result_chirho
            .diagnostics_chirho
            .has_errors_chirho()
    {
        return Err(validity_result_chirho.diagnostics_chirho);
    }

    // Phase 4.5: Pattern match exhaustiveness and redundancy checking
    let exhaust_result_chirho =
        haskelujah_typing_chirho::check_module_exhaustiveness_chirho(&module_chirho);
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

    // Phase 4.6: Linearity checking (only when LinearTypes extension is enabled)
    let linearity_warnings_chirho = if module_chirho
        .extensions_chirho
        .iter()
        .any(|e_chirho| e_chirho == "LinearTypes")
    {
        check_module_linearity_chirho(&module_chirho, &infer_result_chirho)
    } else {
        Vec::new()
    };

    // Collect non-fatal warnings from type inference (e.g. typed holes W4200).
    let infer_warnings_chirho: Vec<String> = infer_result_chirho
        .diagnostics_chirho
        .diagnostics_chirho()
        .iter()
        .filter(|d_chirho| !d_chirho.is_error_chirho())
        .map(|d_chirho| d_chirho.to_string())
        .collect();

    // Merge splice, deriving, exhaustiveness, orphan, linearity, and type inference warnings.
    let mut warnings_chirho = splice_warnings_chirho;
    warnings_chirho.extend(deriving_warnings_chirho);
    warnings_chirho.extend(exhaust_warnings_chirho);
    warnings_chirho.extend(orphan_warning_strs_chirho);
    warnings_chirho.extend(linearity_warnings_chirho);
    warnings_chirho.extend(infer_warnings_chirho);

    Ok(FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        resolved_imported_type_synonyms_chirho: merged_imported_type_synonyms_chirho,
        warnings_chirho,
    })
}

/// Inject an implicit `import Prelude` into a module unless:
/// - `{-# LANGUAGE NoImplicitPrelude #-}` is present, or
/// - the module already has an explicit `import Prelude`.
fn inject_prelude_import_chirho(module_chirho: &mut ModuleChirho) {
    // Check for NoImplicitPrelude extension
    if module_chirho
        .extensions_chirho
        .iter()
        .any(|e_chirho| e_chirho == "NoImplicitPrelude")
    {
        return;
    }

    // Check if Prelude is already explicitly imported
    let has_prelude_import_chirho = module_chirho
        .imports_chirho
        .iter()
        .any(|imp_chirho| imp_chirho.module_chirho.text_chirho() == "Prelude");
    if has_prelude_import_chirho {
        return;
    }

    // Inject implicit Prelude import (unqualified, import everything)
    module_chirho
        .imports_chirho
        .push(haskelujah_ast_chirho::module_chirho::ImportDeclChirho {
            module_chirho: haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                    "Prelude",
                    haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
                ),
            ),
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        });
}

/// Check linearity constraints when the `LinearTypes` extension is enabled.
///
/// For each function with a type signature containing a linear arrow (`%1 ->` or `⊸`),
/// verifies that each linear parameter is used exactly once in the body.
/// Returns warning strings for any violations found.
fn check_module_linearity_chirho(
    module_chirho: &ModuleChirho,
    _infer_result_chirho: &InferResultChirho,
) -> Vec<String> {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_ast_chirho::ty_chirho::{MultiplicityChirho, TypeChirho as AstTypeChirho};
    use haskelujah_typing_chirho::linearity_chirho::{
        check_linearity_chirho, pat_bound_names_chirho,
    };

    let mut warnings_chirho = Vec::new();

    // Collect type signatures: name → AST type
    let mut type_sigs_chirho: std::collections::HashMap<String, &AstTypeChirho> =
        std::collections::HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::TypeSigChirho {
            name_chirho,
            ty_chirho,
            ..
        } = decl_chirho
        {
            type_sigs_chirho.insert(name_chirho.text_chirho().to_string(), ty_chirho);
        }
    }

    // Check each function binding that has a type sig with linear arrows
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::FunBindChirho {
            name_chirho,
            matches_chirho,
            span_chirho,
        } = decl_chirho
        {
            let fn_name_chirho = name_chirho.text_chirho().to_string();
            let sig_ty_chirho = match type_sigs_chirho.get(&fn_name_chirho) {
                Some(ty_chirho) => *ty_chirho,
                None => continue,
            };

            // Extract linear parameter positions from the AST type signature
            // Walk FunChirho chain, checking mult_chirho field
            let mut linear_positions_chirho = Vec::new();
            let mut ty_cursor_chirho = sig_ty_chirho;
            loop {
                match ty_cursor_chirho {
                    AstTypeChirho::FunChirho {
                        mult_chirho,
                        result_chirho,
                        ..
                    } => {
                        let is_linear_chirho = match mult_chirho {
                            Some(MultiplicityChirho::OneChirho) => true,
                            _ => false,
                        };
                        linear_positions_chirho.push(is_linear_chirho);
                        ty_cursor_chirho = result_chirho;
                    }
                    // Skip forall, context, and parentheses to find inner FunChirho
                    AstTypeChirho::ForallChirho { body_chirho, .. } => {
                        ty_cursor_chirho = body_chirho;
                    }
                    AstTypeChirho::QualChirho { body_chirho, .. } => {
                        ty_cursor_chirho = body_chirho;
                    }
                    AstTypeChirho::ParenChirho { inner_chirho, .. } => {
                        ty_cursor_chirho = inner_chirho;
                    }
                    _ => break,
                }
            }

            // If no linear parameters, skip this function
            if !linear_positions_chirho.iter().any(|b_chirho| *b_chirho) {
                continue;
            }

            // Check each match arm
            for arm_chirho in matches_chirho {
                let mut param_names_chirho: Vec<(String, bool)> = Vec::new();
                for (i_chirho, pat_chirho) in arm_chirho.pats_chirho.iter().enumerate() {
                    let is_linear_chirho = linear_positions_chirho
                        .get(i_chirho)
                        .copied()
                        .unwrap_or(false);
                    for name_chirho in pat_bound_names_chirho(pat_chirho) {
                        param_names_chirho.push((name_chirho, is_linear_chirho));
                    }
                }

                let body_expr_chirho = match &arm_chirho.rhs_chirho {
                    haskelujah_ast_chirho::expr_chirho::RhsChirho::UnguardedChirho(e_chirho) => {
                        e_chirho
                    }
                    haskelujah_ast_chirho::expr_chirho::RhsChirho::GuardedChirho(_) => {
                        continue;
                    }
                };

                let violations_chirho =
                    check_linearity_chirho(&param_names_chirho, body_expr_chirho, *span_chirho);

                for v_chirho in &violations_chirho {
                    match v_chirho {
                        haskelujah_typing_chirho::linearity_chirho::LinearityViolationChirho::UsedMultipleChirho {
                            name_chirho: var_name_chirho,
                            count_chirho,
                            ..
                        } => {
                            warnings_chirho.push(format!(
                                "Linearity violation: linear variable `{var_name_chirho}` \
                                 is used {count_chirho} times in `{fn_name_chirho}`"
                            ));
                        }
                        haskelujah_typing_chirho::linearity_chirho::LinearityViolationChirho::UnusedLinearChirho {
                            name_chirho: var_name_chirho,
                            ..
                        } => {
                            warnings_chirho.push(format!(
                                "Linearity violation: linear variable `{var_name_chirho}` \
                                 is unused in `{fn_name_chirho}`"
                            ));
                        }
                    }
                }
            }
        }
    }

    warnings_chirho
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
        );
    }

    scan_sibling_module_ifaces_chirho(
        search_dir_chirho,
        &mut sibling_source_map_chirho,
        &mut all_ifaces_chirho,
        &file_name_chirho,
    );
    for _round_chirho in 0..5 {
        let prev_count_chirho = all_ifaces_chirho.len();
        scan_hierarchical_modules_chirho(
            search_dir_chirho,
            search_dir_chirho,
            &mut sibling_source_map_chirho,
            &mut all_ifaces_chirho,
            &file_name_chirho,
        );
        if all_ifaces_chirho.len() == prev_count_chirho {
            break;
        }
    }

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

/// Extract the first argument type from a GADT type signature.
/// Peels through forall and qualified types, then returns the first `arg` from
/// `arg -> result`.  Returns `None` for non-function types (nullary GADT ctors).
fn extract_first_fun_arg_chirho(
    ty_chirho: &haskelujah_ast_chirho::ty_chirho::TypeChirho,
) -> Option<haskelujah_ast_chirho::ty_chirho::TypeChirho> {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    match ty_chirho {
        TypeChirho::FunChirho { arg_chirho, .. } => Some((**arg_chirho).clone()),
        TypeChirho::ForallChirho { body_chirho, .. } => extract_first_fun_arg_chirho(body_chirho),
        TypeChirho::QualChirho { body_chirho, .. } => extract_first_fun_arg_chirho(body_chirho),
        TypeChirho::ParenChirho { inner_chirho, .. } => extract_first_fun_arg_chirho(inner_chirho),
        _ => None,
    }
}

/// Build a mapping from data constructor names to their parent type name.
/// Used by the dictionary-passing transform to select the correct instance
/// dictionary when a class method is applied to a constructor value.
fn build_con_type_map_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashMap<String, String> {
    let mut map_chirho = std::collections::HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho {
            name_chirho,
            constructors_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            for con_chirho in constructors_chirho {
                let con_name_chirho = match con_chirho {
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        name_chirho: cn_chirho,
                        ..
                    } => cn_chirho.text_chirho().to_string(),
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        name_chirho: cn_chirho,
                        ..
                    } => cn_chirho.text_chirho().to_string(),
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
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
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
            name_chirho,
            constructor_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            let con_name_chirho = match constructor_chirho {
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho: cn_chirho,
                    ..
                } => cn_chirho.text_chirho().to_string(),
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: cn_chirho,
                    ..
                } => cn_chirho.text_chirho().to_string(),
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
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
        if let haskelujah_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
            name_chirho,
            constructor_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = name_chirho.text_chirho().to_string();
            let con_name_chirho = match constructor_chirho {
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho: cn_chirho,
                    fields_chirho,
                    ..
                } => {
                    let underlying_chirho = fields_chirho
                        .first()
                        .map(|(_s_chirho, ty_chirho)| {
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(ty_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: cn_chirho,
                    fields_chirho,
                    ..
                } => {
                    let underlying_chirho = fields_chirho
                        .first()
                        .map(|fd_chirho| {
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(&fd_chirho.ty_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                    name_chirho: cn_chirho,
                    ty_chirho,
                    ..
                } => {
                    // Extract the first argument type from the GADT type signature.
                    // E.g. `forall a. Ctx => Int -> T a` → underlying is `Int`.
                    let underlying_chirho = extract_first_fun_arg_chirho(ty_chirho)
                        .map(|arg_ty_chirho| {
                            haskelujah_core_chirho::desugar_chirho::DesugarCtxChirho::type_key_from_ast_chirho(&arg_ty_chirho)
                        })
                        .unwrap_or_else(|| "()".to_string());
                    (cn_chirho.text_chirho().to_string(), underlying_chirho)
                }
            };
            map_chirho.insert(type_name_chirho, con_name_chirho);
        }
    }
    for (type_name_chirho, con_name_chirho, underlying_chirho) in [
        ("Sum", "Sum", "Int"),
        ("Product", "Product", "Int"),
        ("All", "All", "Bool"),
        ("Any", "Any", "Bool"),
    ] {
        map_chirho
            .entry(type_name_chirho.to_string())
            .or_insert((con_name_chirho.to_string(), underlying_chirho.to_string()));
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
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        file_name_chirho,
        source_chirho,
    );
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let empty_imported_types_chirho = std::collections::HashMap::new();

    let frontend_result_chirho = run_frontend_chirho(
        source_chirho,
        file_id_chirho,
        &builtin_ifaces_chirho,
        &empty_imported_types_chirho,
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
    let (frontend_result_chirho, ifaces_chirho) =
        source_chirho::typecheck_source_with_ifaces_chirho(
            source_chirho,
            source_map_chirho,
            file_name_chirho,
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
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        file_name_chirho,
        source_chirho,
    );
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let mut all_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut imported_types_chirho = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();
    if source_imports_stdlib_chirho(source_chirho) {
        merge_stdlib_frontend_artifacts_chirho(
            &mut all_ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        );
    }

    scan_sibling_module_ifaces_chirho(
        search_dir_chirho,
        source_map_chirho,
        &mut all_ifaces_chirho,
        file_name_chirho,
    );

    // Scan ALL subdirectories recursively for hierarchical module imports
    // (e.g., Debug/SimpleReflect/Expr.hs for `import Debug.SimpleReflect.Expr`).
    // Fixed-point iteration handles dependency chains between modules.
    for _round_chirho in 0..5 {
        let prev_count_chirho = all_ifaces_chirho.len();
        scan_hierarchical_modules_chirho(
            search_dir_chirho,
            search_dir_chirho,
            source_map_chirho,
            &mut all_ifaces_chirho,
            file_name_chirho,
        );
        if all_ifaces_chirho.len() == prev_count_chirho {
            break;
        }
    }

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
    )
}

mod evidence_join_chirho;
pub(crate) use evidence_join_chirho::join_occurrence_evidence_chirho;

/// Run the back-end pipeline phases (5 through 7) on an already-front-end-compiled
/// module, producing a [`CompileResultChirho`].
///
/// Phases: desugar → dict pass → simplify → LLVM IR → Wasm bytes.
fn compile_backend_chirho(
    module_chirho: ModuleChirho,
    infer_result_chirho: InferResultChirho,
    extra_dict_param_names_chirho: std::collections::HashSet<String>,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    // Phase 5: Desugar AST → Core IR (evidence-threading P2b: with
    // per-occurrence ids for the method-occurrence names).
    // Dictionary evidence: every name whose scheme carries class predicates
    // gets an occurrence id per reference, so the checker's per-reference
    // evidence can reach the dictionary pass.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    // Class methods are not references to constrained functions and are not
    // minted here: the method-occurrence names (the standard set and this
    // module's own single-parameter class methods) get their ids from the
    // method-occurrence mechanism, and every other method keeps its canonical id.
    // workflow: language-features-chirho/own-class-method-occurrences-chirho
    let class_method_names_chirho: std::collections::HashSet<&String> = infer_result_chirho
        .class_env_chirho
        .classes_chirho
        .values()
        .flat_map(|class_chirho| class_chirho.methods_chirho.keys())
        .collect();
    let constrained_names_chirho: std::collections::HashSet<String> = infer_result_chirho
        .env_chirho
        .all_bindings_chirho()
        .into_iter()
        .filter(|(name_chirho, scheme_chirho)| {
            !scheme_chirho.preds_chirho.is_empty()
                && !class_method_names_chirho.contains(name_chirho)
        })
        .map(|(name_chirho, _scheme_chirho)| name_chirho.clone())
        .collect();
    // Constructor arities, imported constructors included: `C {}` on a
    // positional constructor fills every argument with a missing-field thunk.
    // workflow: language-features-chirho/rigid-type-variables-chirho (records)
    let constructor_arities_chirho: std::collections::HashMap<String, usize> = infer_result_chirho
        .env_chirho
        .all_bindings_chirho()
        .into_iter()
        .filter(|(name_chirho, _scheme_chirho)| {
            name_chirho
                .chars()
                .next()
                .is_some_and(|first_chirho| first_chirho.is_uppercase())
        })
        .map(|(name_chirho, scheme_chirho)| {
            (name_chirho.clone(), scheme_chirho.spine_arity_chirho())
        })
        .collect();
    let desugar_output_chirho = haskelujah_core_chirho::desugar_module_with_inputs_chirho(
        &module_chirho,
        haskelujah_core_chirho::DesugarInputsChirho {
            method_names_chirho: evidence_join_chirho::method_occurrence_names_chirho(
                &module_chirho,
            ),
            constrained_names_chirho,
            constructor_arities_chirho,
        },
    );
    let reference_evidence_chirho = evidence_join_chirho::join_reference_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.reference_occurrence_spans_chirho,
        &desugar_output_chirho.occurrence_provenance_chirho,
    );
    let occurrence_evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.method_occurrences_chirho,
        &desugar_output_chirho.literal_occurrence_spans_chirho,
        &desugar_output_chirho.method_occurrence_spans_chirho,
        &desugar_output_chirho.occurrence_provenance_chirho,
    );

    // Phase 5.5: Dictionary-passing transform (desugar typeclass constraints)
    let con_types_chirho = build_con_type_map_chirho(&module_chirho);
    let newtype_info_chirho = build_newtype_info_chirho(&module_chirho);
    let mut newtype_cons_chirho: std::collections::HashSet<String> = newtype_info_chirho
        .values()
        .map(|(con_name_chirho, _)| con_name_chirho.clone())
        .collect();
    if !con_types_chirho.contains_key("Identity")
        && module_chirho.imports_chirho.iter().any(|import_chirho| {
            matches!(
                import_chirho.module_chirho.full_name_chirho().as_str(),
                "Data.Functor.Identity" | "Control.Monad.Identity"
            )
        })
    {
        newtype_cons_chirho.insert("Identity".to_string());
    }
    if !con_types_chirho.contains_key("Const")
        && module_chirho.imports_chirho.iter().any(|import_chirho| {
            import_chirho.module_chirho.full_name_chirho() == "Data.Functor.Const"
        })
    {
        newtype_cons_chirho.insert("Const".to_string());
    }
    if !con_types_chirho.contains_key("Down")
        && module_chirho
            .imports_chirho
            .iter()
            .any(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Data.Ord")
    {
        newtype_cons_chirho.insert("Down".to_string());
    }
    if module_chirho
        .imports_chirho
        .iter()
        .any(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Data.Semigroup")
    {
        for con_name_chirho in ["Min", "Max", "Endo"] {
            if !con_types_chirho.contains_key(con_name_chirho) {
                newtype_cons_chirho.insert(con_name_chirho.to_string());
            }
        }
    }
    if module_chirho
        .imports_chirho
        .iter()
        .any(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Data.Monoid")
    {
        for con_name_chirho in ["First", "Last", "Endo"] {
            if !con_types_chirho.contains_key(con_name_chirho) {
                newtype_cons_chirho.insert(con_name_chirho.to_string());
            }
        }
    }
    // Constructor metadata for every type that derives Show (row 533).
    // workflow: print-show-evidence-chirho
    let derived_shows_chirho =
        haskelujah_typing_chirho::deriving_chirho::derived_show_metadata_chirho(&module_chirho);
    let dict_result_chirho = haskelujah_core_chirho::dict_pass_module_full_with_evidence_chirho(
        &desugar_output_chirho.module_chirho,
        desugar_output_chirho.names_chirho,
        &infer_result_chirho.env_chirho,
        &infer_result_chirho.class_env_chirho,
        con_types_chirho,
        newtype_info_chirho,
        extra_dict_param_names_chirho,
        desugar_output_chirho.method_occurrences_chirho,
        occurrence_evidence_chirho,
        reference_evidence_chirho,
        derived_shows_chirho,
        desugar_output_chirho.operator_occurrences_chirho,
    );
    let core_chirho = dict_result_chirho.module_chirho;

    // Phase 6: Core-to-Core simplification (beta reduction, dead code, case-of-known)
    let config_chirho = SimplifyConfigChirho::default();
    let mut core_chirho = simplify_module_chirho(&core_chirho, &config_chirho);

    // Phase 6.5: Native provenance — the module's entry candidate and its builtin
    // references, from its imports (row 628).
    // workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
    native_provenance_chirho::record_module_provenance_chirho(
        &module_chirho,
        &mut core_chirho,
        ifaces_chirho,
    );

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
    let compile_result_chirho =
        compile_source_chirho(source_chirho, source_map_chirho, file_name_chirho)
            .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    stg_lower_chirho::lower_and_run_outcome_chirho(
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
mod tests_chirho;
