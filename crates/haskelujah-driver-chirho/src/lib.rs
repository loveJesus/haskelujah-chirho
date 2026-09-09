// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-driver-chirho
//!
//! Build orchestration for the Haskelujah compiler. Coordinates parsing, checking,
//! and backend lowering across execution modes.

mod module_search_chirho;
mod source_chirho;
pub use source_chirho::typecheck_source_chirho;
pub mod splice_chirho;
pub mod stg_lower_chirho;

use source_chirho::cpp_chirho::{configure_cpp_command_chirho, temporary_source_file_chirho};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use haskelujah_ast_chirho::ModuleChirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_backend_llvm_chirho::compile_core_to_llvm_chirho;
use haskelujah_backend_llvm_chirho::compile_to_llvm_ir_stub_chirho;
use haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_executable_chirho;
use haskelujah_backend_wasm_chirho::compile_core_to_wasm_chirho;
use haskelujah_backend_wasm_chirho::compile_to_wasm_stub_chirho;
use haskelujah_core_chirho::{CoreModuleChirho, SimplifyConfigChirho, simplify_module_chirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
use haskelujah_naming_chirho::iface_chirho::{ModuleIfaceChirho, build_iface_with_imports_chirho};
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

#[cfg(test)]
use module_search_chirho::{
    extract_exported_names_from_source_chirho, extract_module_name_from_source_chirho,
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
    let splice_result_chirho =
        splice_chirho::expand_splices_chirho(std::mem::take(&mut module_chirho.decls_chirho));
    module_chirho.decls_chirho = splice_result_chirho.decls_chirho;
    let splice_warnings_chirho = splice_result_chirho.warnings_chirho;

    // Phase 2.5: Deriving — generate instance declarations for `deriving` clauses
    let deriving_warnings_chirho =
        haskelujah_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);

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
    for (builtin_name_chirho, builtin_scheme_chirho) in
        haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho()
    {
        merged_imported_types_chirho
            .entry(builtin_name_chirho)
            .or_insert(builtin_scheme_chirho);
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
                                None
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
    let frontend_result_chirho =
        typecheck_source_chirho(source_chirho, source_map_chirho, file_name_chirho)?;

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
    )
}

/// Evidence-threading P2b: standard single-param class methods and constrained
/// Prelude functions whose free references get per-occurrence ids so typing
/// evidence can drive dispatch.
/// Workflow: `spec-chirho/workflows-chirho/print-show-evidence-chirho.md`.
/// Monad-chain operators (>>=, >>, return, pure, fail) are deliberately
/// EXCLUDED — they have dedicated dispatch machinery and INV-001 protection.
const EVIDENCE_METHOD_NAMES_CHIRHO: &[&str] = &[
    "==",
    "/=",
    "<",
    "<=",
    ">",
    ">=",
    "compare",
    "max",
    "min",
    "+",
    "-",
    "*",
    "negate",
    "abs",
    "signum",
    "fromInteger",
    "div",
    "mod",
    "quot",
    "rem",
    "show",
    "print",
    "mfix",
    "read",
];

/// Evidence-threading P2b: join typing occurrence records to desugar occurrence
/// ids by (name, source-order ordinal). Names whose reference counts disagree
/// between the two phases are dropped wholesale (conservative alignment).
/// workflow: monadic-dispatch-chirho (evidence-threading)
fn join_occurrence_evidence_chirho(
    infer_result_chirho: &InferResultChirho,
    method_occurrences_chirho: &std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        (String, haskelujah_core_chirho::CoreIdChirho),
    >,
    literal_occurrence_spans_chirho: &std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        haskelujah_span_chirho::SpanChirho,
    >,
) -> std::collections::HashMap<haskelujah_core_chirho::CoreIdChirho, (String, String)> {
    // Literal evidence joins by span, exactly: the checker solved this very
    // literal. It is inserted first so the per-name ordinal join below never
    // overrides it. The engine's body-backed rows are keyed "Int"; the checker
    // defaults an unconstrained literal to "Integer" per the Report.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    let mut evidence_chirho: std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        (String, String),
    > = std::collections::HashMap::new();
    for (occ_id_chirho, span_chirho) in literal_occurrence_spans_chirho {
        if let Some(record_chirho) = infer_result_chirho.literal_evidence_chirho.get(span_chirho) {
            let ty_key_chirho = if record_chirho.ty_key_chirho == "Integer" {
                "Int".to_string()
            } else {
                record_chirho.ty_key_chirho.clone()
            };
            evidence_chirho.insert(
                *occ_id_chirho,
                (record_chirho.class_name_chirho.clone(), ty_key_chirho),
            );
        }
    }
    let mut occ_ids_by_name_chirho: std::collections::HashMap<
        String,
        Vec<haskelujah_core_chirho::CoreIdChirho>,
    > = std::collections::HashMap::new();
    for (occ_id_chirho, (name_chirho, _canon_chirho)) in method_occurrences_chirho {
        occ_ids_by_name_chirho
            .entry(name_chirho.clone())
            .or_default()
            .push(*occ_id_chirho);
    }
    for ids_chirho in occ_ids_by_name_chirho.values_mut() {
        ids_chirho.sort_by_key(|id_chirho| id_chirho.0);
    }
    for (name_chirho, ids_chirho) in &occ_ids_by_name_chirho {
        if infer_result_chirho
            .method_occurrence_totals_chirho
            .get(name_chirho)
            .copied()
            != Some(ids_chirho.len() as u32)
        {
            continue;
        }
        for record_chirho in infer_result_chirho
            .method_occurrences_chirho
            .iter()
            .filter(|record_chirho| &record_chirho.name_chirho == name_chirho)
        {
            // A record is authoritative only when its class either declares
            // this method (`==` -> Eq) or constrains this function's own
            // scheme (`print :: Show a => ...`). Incidental predicates never
            // drive occurrence dispatch.
            let class_declares_reference_chirho = infer_result_chirho
                .class_env_chirho
                .classes_chirho
                .get(&record_chirho.class_name_chirho)
                .is_some_and(|class_chirho| class_chirho.methods_chirho.contains_key(name_chirho));
            let function_requires_class_chirho = infer_result_chirho
                .env_chirho
                .lookup_chirho(name_chirho)
                .is_some_and(|scheme_chirho| {
                    scheme_chirho.preds_chirho.iter().any(|pred_chirho| {
                        pred_chirho.class_name_chirho == record_chirho.class_name_chirho
                    })
                });
            if !class_declares_reference_chirho && !function_requires_class_chirho {
                continue;
            }
            if let Some(occ_id_chirho) = ids_chirho.get(record_chirho.ordinal_chirho as usize) {
                // The engine's body-backed instance rows are keyed "Int"; typing
                // defaults ambiguous numerics to "Integer" per the Report. Map
                // the evidence key onto the backed row at this boundary only.
                let ty_key_chirho = if record_chirho.ty_key_chirho == "Integer" {
                    "Int".to_string()
                } else {
                    record_chirho.ty_key_chirho.clone()
                };
                evidence_chirho
                    .entry(*occ_id_chirho)
                    .or_insert_with(|| (record_chirho.class_name_chirho.clone(), ty_key_chirho));
            }
        }
    }
    evidence_chirho
}

/// Run the back-end pipeline phases (5 through 7) on an already-front-end-compiled
/// module, producing a [`CompileResultChirho`].
///
/// Phases: desugar → dict pass → simplify → LLVM IR → Wasm bytes.
fn compile_backend_chirho(
    module_chirho: ModuleChirho,
    infer_result_chirho: InferResultChirho,
    extra_dict_param_names_chirho: std::collections::HashSet<String>,
) -> Result<CompileResultChirho, DiagnosticBundleChirho> {
    // Phase 5: Desugar AST → Core IR (evidence-threading P2b: with
    // per-occurrence ids for the standard method-name set).
    // Dictionary evidence: every name whose scheme carries class predicates
    // gets an occurrence id per reference, so the checker's per-reference
    // evidence can reach the dictionary pass.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    // Class methods are not references to constrained functions: the pass
    // dispatches them through selectors keyed on their canonical ids, so they
    // keep the method-occurrence mechanism and are not minted here.
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
            method_names_chirho: EVIDENCE_METHOD_NAMES_CHIRHO
                .iter()
                .map(|name_chirho| name_chirho.to_string())
                .collect(),
            constrained_names_chirho,
            constructor_arities_chirho,
        },
    );
    let reference_evidence_chirho: std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        Vec<(String, Option<String>)>,
    > = desugar_output_chirho
        .reference_occurrence_spans_chirho
        .iter()
        .filter_map(|(occ_id_chirho, span_chirho)| {
            let records_chirho = infer_result_chirho
                .reference_evidence_chirho
                .get(span_chirho)?;
            Some((
                *occ_id_chirho,
                records_chirho
                    .iter()
                    .map(|record_chirho| {
                        let key_chirho = record_chirho.ty_key_chirho.as_deref().map(|key_chirho| {
                            if key_chirho == "Integer" {
                                "Int".to_string()
                            } else {
                                key_chirho.to_string()
                            }
                        });
                        (record_chirho.class_name_chirho.clone(), key_chirho)
                    })
                    .collect(),
            ))
        })
        .collect();
    let occurrence_evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho,
        &desugar_output_chirho.method_occurrences_chirho,
        &desugar_output_chirho.literal_occurrence_spans_chirho,
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
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    // Accumulated type schemes from all previously-compiled modules,
    // keyed by unqualified name. Downstream modules receive all upstream
    // exports so that type inference can resolve cross-module references.
    let mut imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    > = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();

    for (file_name_chirho, source_chirho) in sources_chirho {
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            file_name_chirho,
            *source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();

        // Phases 1–4.5: shared front-end
        let frontend_result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
            source_chirho,
            file_id_chirho,
            &ifaces_chirho,
            &imported_types_chirho,
            &imported_type_synonyms_chirho,
            &imported_type_families_chirho,
        )?;

        let FrontendResultChirho {
            module_chirho,
            infer_result_chirho,
            resolved_imported_type_synonyms_chirho,
            warnings_chirho: _warnings_chirho,
        } = frontend_result_chirho;

        // Build interface for downstream modules before consuming module_chirho.
        // Use import-aware variant so `module Foo` re-exports work.
        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

        let extra_dict_param_names_chirho: std::collections::HashSet<String> =
            imported_types_chirho.keys().cloned().collect();

        // Carry both bare and module-qualified export names forward so
        // downstream modules can resolve selective and qualified imports.
        insert_exported_schemes_into_imports_chirho(
            &iface_chirho,
            &infer_result_chirho,
            &mut imported_types_chirho,
        );

        imported_type_synonyms_chirho.extend(exported_type_synonyms_from_module_chirho(
            &module_chirho,
            &iface_chirho,
            &resolved_imported_type_synonyms_chirho,
        ));
        imported_type_families_chirho = infer_result_chirho.type_families_chirho.clone();

        ifaces_chirho.push(iface_chirho);

        // Phases 5–7: back-end
        let compile_result_chirho = compile_backend_chirho(
            module_chirho,
            infer_result_chirho,
            extra_dict_param_names_chirho,
        )?;
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
) -> Result<haskelujah_runtime_chirho::ValueChirho, String> {
    use haskelujah_core_chirho::expr_chirho::{
        CoreExprChirho, CoreIdChirho, CoreModuleChirho as CoreModChirho,
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

    fn max_id_in_expr_chirho(expr_chirho: &CoreExprChirho, max_id_chirho: &mut u32) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                *max_id_chirho = (*max_id_chirho).max(id_chirho.0);
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                max_id_in_expr_chirho(fun_chirho, max_id_chirho);
                max_id_in_expr_chirho(arg_chirho, max_id_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                max_id_in_expr_chirho(body_chirho, max_id_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                    max_id_in_expr_chirho(rhs_chirho, max_id_chirho);
                }
                max_id_in_expr_chirho(body_chirho, max_id_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                *max_id_chirho = (*max_id_chirho).max(bind_chirho.id_chirho.0);
                max_id_in_expr_chirho(scrutinee_chirho, max_id_chirho);
                for alt_chirho in alts_chirho {
                    for binder_chirho in &alt_chirho.binders_chirho {
                        *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                    }
                    max_id_in_expr_chirho(&alt_chirho.rhs_chirho, max_id_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                max_id_in_expr_chirho(body_chirho, max_id_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                max_id_in_expr_chirho(inner_chirho, max_id_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    max_id_in_expr_chirho(arg_chirho, max_id_chirho);
                }
            }
        }
    }

    fn max_id_in_module_chirho(module_chirho: &CoreModChirho) -> u32 {
        let mut max_id_chirho = module_chirho
            .names_chirho
            .keys()
            .map(|id_chirho| id_chirho.0)
            .max()
            .unwrap_or(0);
        for binding_chirho in &module_chirho.bindings_chirho {
            max_id_chirho = max_id_chirho.max(binding_chirho.binder_chirho.id_chirho.0);
            max_id_in_expr_chirho(&binding_chirho.rhs_chirho, &mut max_id_chirho);
        }
        max_id_chirho
    }

    fn offset_binder_chirho(
        b_chirho: &mut haskelujah_core_chirho::expr_chirho::BinderChirho,
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
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                offset_expr_chirho(fun_chirho, off_chirho);
                offset_expr_chirho(arg_chirho, off_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                offset_binder_chirho(binder_chirho, off_chirho);
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (b_chirho, rhs_chirho) in binds_chirho {
                    offset_binder_chirho(b_chirho, off_chirho);
                    offset_expr_chirho(rhs_chirho, off_chirho);
                }
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
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
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                offset_expr_chirho(inner_chirho, off_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    offset_expr_chirho(a_chirho, off_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    offset_expr_chirho(a_chirho, off_chirho);
                }
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
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                remap_expr_chirho(fun_chirho, remap_chirho);
                remap_expr_chirho(arg_chirho, remap_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_b_chirho, rhs_chirho) in binds_chirho {
                    remap_expr_chirho(rhs_chirho, remap_chirho);
                }
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                remap_expr_chirho(scrutinee_chirho, remap_chirho);
                for alt_chirho in alts_chirho {
                    remap_expr_chirho(&mut alt_chirho.rhs_chirho, remap_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                remap_expr_chirho(inner_chirho, remap_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    remap_expr_chirho(a_chirho, remap_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    remap_expr_chirho(a_chirho, remap_chirho);
                }
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
        let max_id_chirho = max_id_in_module_chirho(&result_chirho.core_chirho);
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
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                collect_binder_ids_chirho(fun_chirho, set_chirho);
                collect_binder_ids_chirho(arg_chirho, set_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                set_chirho.insert(binder_chirho.id_chirho);
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (b_chirho, rhs_chirho) in binds_chirho {
                    set_chirho.insert(b_chirho.id_chirho);
                    collect_binder_ids_chirho(rhs_chirho, set_chirho);
                }
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
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
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                collect_binder_ids_chirho(inner_chirho, set_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    collect_binder_ids_chirho(a_chirho, set_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    collect_binder_ids_chirho(a_chirho, set_chirho);
                }
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
        name_chirho: results_chirho
            .last()
            .unwrap()
            .module_chirho
            .name_chirho
            .text_chirho()
            .to_string(),
        bindings_chirho: merged_bindings_chirho,
        names_chirho: merged_names_chirho,
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
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
    let compile_result_chirho =
        compile_source_chirho(source_chirho, source_map_chirho, file_name_chirho)
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

/// Compile multiple Haskell source files with incremental recompilation avoidance.
///
/// Uses an [`IncrementalSessionChirho`] to track source fingerprints and
/// dependency-interface fingerprints. Only modules whose source or dependencies
/// have changed are recompiled; the rest reuse cached results.
pub fn compile_modules_incremental_chirho(
    sources_chirho: &[(&str, &str)], // (file_name, source_text)
    source_map_chirho: &mut SourceMapChirho,
    session_chirho: &mut haskelujah_incremental_chirho::IncrementalSessionChirho,
) -> Result<Vec<(CompileResultChirho, bool)>, DiagnosticBundleChirho> {
    use haskelujah_incremental_chirho::{FingerprintChirho, PhaseTagChirho};

    // Phase 0: Register all modules and compute fingerprints.
    // Parse module headers to extract names and imports.
    let mut module_infos_chirho: Vec<(String, String, &str, haskelujah_span_chirho::FileIdChirho)> =
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
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
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
            resolved_imported_type_synonyms_chirho: _resolved_imported_type_synonyms_chirho,
            warnings_chirho: _warnings_chirho,
        } = frontend_result_chirho;

        // Build and register the module interface for downstream modules.
        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

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
        let compile_result_chirho = compile_backend_chirho(
            module_chirho,
            infer_result_chirho,
            std::collections::HashSet::new(),
        )?;

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
            } else if path_chirho
                .extension()
                .map_or(false, |ext_chirho| ext_chirho == "hs")
            {
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
            if let Some(module_name_chirho) = parse_import_module_name_chirho(rest_chirho) {
                imports_chirho.push(module_name_chirho);
            }
        }
    }
    imports_chirho
}

fn parse_import_module_name_chirho(rest_chirho: &str) -> Option<String> {
    let mut remaining_chirho = rest_chirho.trim();

    'scan_modifiers_chirho: loop {
        if let Some(pragma_rest_chirho) = remaining_chirho.strip_prefix("{-#") {
            if let Some(pragma_end_chirho) = pragma_rest_chirho.find("#-}") {
                remaining_chirho = pragma_rest_chirho[pragma_end_chirho + 3..].trim_start();
                continue;
            }
        }

        for keyword_chirho in ["qualified", "safe", "unsafe", "interruptible"] {
            if let Some(after_keyword_chirho) = remaining_chirho.strip_prefix(keyword_chirho) {
                if after_keyword_chirho
                    .chars()
                    .next()
                    .is_some_and(|char_chirho| char_chirho.is_whitespace())
                {
                    remaining_chirho = after_keyword_chirho.trim_start();
                    continue 'scan_modifiers_chirho;
                }
            }
        }

        if let Some(package_rest_chirho) = remaining_chirho.strip_prefix('"') {
            if let Some(package_end_chirho) = package_rest_chirho.find('"') {
                remaining_chirho = package_rest_chirho[package_end_chirho + 1..].trim_start();
                continue;
            }
        }

        break;
    }

    let module_name_chirho = remaining_chirho
        .split(|char_chirho: char| char_chirho.is_whitespace() || char_chirho == '(')
        .next()
        .unwrap_or("")
        .trim();
    if module_name_chirho.is_empty() {
        None
    } else {
        Some(module_name_chirho.to_string())
    }
}

fn filter_seeded_type_synonyms_for_source_chirho(
    source_chirho: &str,
    imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
) -> ImportedTypeSynonymsChirho {
    let imported_modules_chirho: std::collections::HashSet<String> =
        extract_imports_chirho(source_chirho).into_iter().collect();
    if imported_modules_chirho.is_empty() {
        return ImportedTypeSynonymsChirho::new();
    }

    imported_type_synonyms_chirho
        .iter()
        .filter(|(name_chirho, _synonym_chirho)| {
            imported_modules_chirho.iter().any(|module_name_chirho| {
                name_chirho
                    .strip_prefix(module_name_chirho)
                    .is_some_and(|suffix_chirho| suffix_chirho.starts_with('.'))
            })
        })
        .map(|(name_chirho, synonym_chirho)| (name_chirho.clone(), synonym_chirho.clone()))
        .collect()
}

fn filter_seeded_imported_types_for_source_chirho(
    source_chirho: &str,
    imported_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    >,
) -> std::collections::HashMap<String, haskelujah_typing_chirho::SchemeChirho> {
    let imported_modules_chirho: std::collections::HashSet<String> =
        extract_imports_chirho(source_chirho).into_iter().collect();
    if imported_modules_chirho.is_empty() {
        return std::collections::HashMap::new();
    }

    let mut qualified_bare_schemes_chirho: std::collections::HashMap<
        String,
        Vec<haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    > = std::collections::HashMap::new();
    for (qualified_name_chirho, qualified_scheme_chirho) in imported_types_chirho {
        for module_name_chirho in &imported_modules_chirho {
            let Some(suffix_chirho) = qualified_name_chirho.strip_prefix(module_name_chirho) else {
                continue;
            };
            if !suffix_chirho.starts_with('.') {
                continue;
            }
            let remainder_chirho = &suffix_chirho[1..];
            if let Some((first_segment_chirho, _rest_chirho)) = remainder_chirho.split_once('.') {
                if first_segment_chirho
                    .chars()
                    .next()
                    .is_some_and(|chirho| chirho.is_uppercase())
                {
                    continue;
                }
            }
            if !remainder_chirho.is_empty() && !remainder_chirho.contains('.') {
                qualified_bare_schemes_chirho
                    .entry(remainder_chirho.to_string())
                    .or_default()
                    .push(qualified_scheme_chirho.clone());
            }
        }
    }

    imported_types_chirho
        .iter()
        .filter(|(name_chirho, scheme_chirho)| {
            if !name_chirho.contains('.') {
                return qualified_bare_schemes_chirho.get(*name_chirho).is_some_and(
                    |schemes_chirho| {
                        schemes_chirho.len() == 1
                            && schemes_chirho
                                .first()
                                .is_some_and(|qualified_scheme_chirho| {
                                    qualified_scheme_chirho == *scheme_chirho
                                })
                    },
                );
            }
            imported_modules_chirho.iter().any(|module_name_chirho| {
                name_chirho
                    .strip_prefix(module_name_chirho)
                    .is_some_and(|suffix_chirho| {
                        if !suffix_chirho.starts_with('.') {
                            return false;
                        }
                        let remainder_chirho = &suffix_chirho[1..];
                        if let Some((first_segment_chirho, _rest_chirho)) =
                            remainder_chirho.split_once('.')
                        {
                            if first_segment_chirho
                                .chars()
                                .next()
                                .is_some_and(|chirho| chirho.is_uppercase())
                            {
                                return false;
                            }
                        }
                        true
                    })
            })
        })
        .map(|(name_chirho, scheme_chirho)| (name_chirho.clone(), scheme_chirho.clone()))
        .collect()
}

fn source_imports_stdlib_chirho(source_chirho: &str) -> bool {
    extract_imports_chirho(source_chirho)
        .iter()
        .any(|module_name_chirho| module_name_chirho.starts_with("Haskelujah."))
}

/// Parse a `.hs-boot` file to extract a minimal module interface.
///
/// Boot files provide enough type and value declarations to break circular
/// import cycles. This function runs the front-end pipeline on the boot file
/// to produce a `ModuleIfaceChirho` and extracted type schemes.
fn parse_boot_iface_chirho(
    module_name_chirho: &str,
    boot_source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    boot_path_chirho: &str,
    existing_ifaces_chirho: &[ModuleIfaceChirho],
    existing_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
) -> Result<
    (
        ModuleIfaceChirho,
        std::collections::HashMap<String, haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    ),
    String,
> {
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        boot_path_chirho,
        boot_source_chirho,
    );
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let frontend_result_chirho = run_frontend_chirho(
        boot_source_chirho,
        file_id_chirho,
        existing_ifaces_chirho,
        existing_types_chirho,
    )
    .map_err(|e_chirho| format!("Boot file {} error: {}", module_name_chirho, e_chirho))?;

    let FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        resolved_imported_type_synonyms_chirho: _resolved_imported_type_synonyms_chirho,
        ..
    } = frontend_result_chirho;

    let iface_chirho = build_iface_with_imports_chirho(&module_chirho, existing_ifaces_chirho);

    // Extract type schemes from the boot file
    let mut types_chirho = std::collections::HashMap::new();
    for (name_chirho, _) in &iface_chirho.exports_chirho.values_chirho {
        if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(name_chirho) {
            types_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
        }
    }
    for (_name_chirho, ty_info_chirho) in &iface_chirho.exports_chirho.types_chirho {
        for con_name_chirho in &ty_info_chirho.constructors_chirho {
            if let Some(scheme_chirho) = infer_result_chirho
                .env_chirho
                .lookup_chirho(con_name_chirho)
            {
                types_chirho.insert(con_name_chirho.clone(), scheme_chirho.clone());
            }
        }
    }

    Ok((iface_chirho, types_chirho))
}

/// Compile a Haskell project from a directory, automatically discovering `.hs`
/// files, computing dependency order from import declarations, and compiling
/// modules in topological order. Circular module imports are handled via
/// `.hs-boot` files.
///
/// Returns an error if:
/// - No `.hs` files are found in the directory
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
        let source_chirho = read_haskell_source_file_chirho(path_chirho).map_err(|e_chirho| {
            format!("Failed to read {}: {}", path_chirho.display(), e_chirho)
        })?;
        let module_name_chirho = extract_module_name_chirho(&source_chirho);
        let file_name_chirho = path_chirho.to_string_lossy().to_string();
        module_sources_chirho.push((module_name_chirho, file_name_chirho, source_chirho));
    }

    // Step 3: Build dependency graph
    let mut dep_graph_chirho = haskelujah_incremental_chirho::DepGraphChirho::new_chirho();
    let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();

    for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
        let fp_chirho =
            haskelujah_incremental_chirho::FingerprintChirho::from_str_chirho(source_chirho);
        dep_graph_chirho.add_module_chirho(module_name_chirho, fp_chirho);
        let imports_chirho = extract_imports_chirho(source_chirho);
        for imported_chirho in &imports_chirho {
            // Only add edges for local modules (skip external like Prelude, Data.Map, etc.)
            if known_modules_chirho.contains(imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, imported_chirho);
            }
        }
    }

    // Step 4: Topological sort via SCCs (handles circular imports)
    let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();

    // Flatten SCC order for the result, handling cycles via .hs-boot files.
    let mut order_chirho: Vec<String> = Vec::new();

    // Step 5: Compile in dependency order, respecting SCCs
    let mut results_chirho: Vec<CompileResultChirho> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut all_warnings_chirho: Vec<String> = Vec::new();
    let mut imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    > = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();
    if module_sources_chirho
        .iter()
        .any(|(_, _, source_chirho)| source_imports_stdlib_chirho(source_chirho))
    {
        merge_stdlib_frontend_artifacts_chirho(
            &mut ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        );
    }

    for scc_chirho in &sccs_chirho {
        if scc_chirho.len() > 1 {
            // Circular import group — look for .hs-boot files to break the cycle.
            // Phase 1: For each SCC member, either compile its .hs-boot file
            // or create a minimal empty interface so that `import M` resolves.
            for module_name_chirho in scc_chirho {
                let (_, file_name_chirho, _source_chirho) = module_sources_chirho
                    .iter()
                    .find(|(n_chirho, _, _)| n_chirho == module_name_chirho)
                    .ok_or_else(|| format!("Module {} not found in sources", module_name_chirho))?;

                // Look for a .hs-boot file alongside the .hs file
                let boot_path_chirho = format!("{}-boot", file_name_chirho);
                if let Ok(boot_source_chirho) = read_haskell_source_file_chirho(&boot_path_chirho) {
                    // Parse the boot file to extract a minimal interface
                    let boot_iface_chirho = parse_boot_iface_chirho(
                        module_name_chirho,
                        &boot_source_chirho,
                        source_map_chirho,
                        &boot_path_chirho,
                        &ifaces_chirho,
                        &imported_types_chirho,
                    );
                    match boot_iface_chirho {
                        Ok((iface_chirho, types_chirho)) => {
                            ifaces_chirho.push(iface_chirho);
                            imported_types_chirho.extend(types_chirho);
                        }
                        Err(e_chirho) => {
                            all_warnings_chirho.push(format!(
                                "Warning: failed to parse {}: {}",
                                boot_path_chirho, e_chirho
                            ));
                            // Fall back to empty interface
                            ifaces_chirho.push(ModuleIfaceChirho {
                                name_chirho: module_name_chirho.clone(),
                                exports_chirho: Default::default(),
                            });
                        }
                    }
                } else {
                    // No boot file — create an empty interface so `import M`
                    // at least resolves (names won't be in scope without a boot
                    // file, matching GHC's behavior).
                    ifaces_chirho.push(ModuleIfaceChirho {
                        name_chirho: module_name_chirho.clone(),
                        exports_chirho: Default::default(),
                    });
                }
            }

            // Phase 2: Compile all modules in the SCC using boot interfaces.
            // The order within an SCC doesn't matter (boot files break the cycle).
        }

        // Compile each module in the SCC (or the single module if no cycle)
        for module_name_chirho in scc_chirho {
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
            let filtered_imported_types_chirho = filter_seeded_imported_types_for_source_chirho(
                source_chirho,
                &imported_types_chirho,
            );
            let filtered_imported_type_synonyms_chirho =
                filter_seeded_type_synonyms_for_source_chirho(
                    source_chirho,
                    &imported_type_synonyms_chirho,
                );

            let frontend_result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
                source_chirho,
                file_id_chirho,
                &ifaces_chirho,
                &filtered_imported_types_chirho,
                &filtered_imported_type_synonyms_chirho,
                &imported_type_families_chirho,
            )
            .map_err(|e_chirho| format!("Error compiling {}: {}", module_name_chirho, e_chirho))?;

            let FrontendResultChirho {
                module_chirho,
                infer_result_chirho,
                resolved_imported_type_synonyms_chirho,
                warnings_chirho,
            } = frontend_result_chirho;

            all_warnings_chirho.extend(warnings_chirho);

            // Build interface for downstream modules
            let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

            let extra_dict_param_names_chirho: std::collections::HashSet<String> =
                imported_types_chirho.keys().cloned().collect();

            // Carry both bare and module-qualified export names forward so
            // downstream package modules can resolve imported signatures without
            // falling back to polymorphic placeholders.
            insert_exported_schemes_into_imports_chirho(
                &iface_chirho,
                &infer_result_chirho,
                &mut imported_types_chirho,
            );

            imported_type_synonyms_chirho.extend(exported_type_synonyms_from_module_chirho(
                &module_chirho,
                &iface_chirho,
                &resolved_imported_type_synonyms_chirho,
            ));
            imported_type_families_chirho = infer_result_chirho.type_families_chirho.clone();

            ifaces_chirho.push(iface_chirho);

            // Backend compilation
            let compile_result_chirho = compile_backend_chirho(
                module_chirho,
                infer_result_chirho,
                extra_dict_param_names_chirho,
            )
            .map_err(|e_chirho| {
                format!("Backend error for {}: {}", module_name_chirho, e_chirho)
            })?;

            results_chirho.push(compile_result_chirho);
            order_chirho.push(module_name_chirho.clone());
        }
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
    pub package_chirho: haskelujah_package_chirho::PackageDescChirho,
    /// The resolved build plan (external dependencies in topo order).
    pub build_plan_chirho: haskelujah_package_chirho::BuildPlanChirho,
    /// Per-module compilation results, in compilation order.
    pub module_results_chirho: Vec<CompileResultChirho>,
    /// Module names in compilation order.
    pub compilation_order_chirho: Vec<String>,
    /// Non-fatal warnings across all compiled modules.
    pub warnings_chirho: Vec<String>,
}

#[derive(Debug)]
pub struct CabalExecutableBuildResultChirho {
    pub name_chirho: String,
    pub core_chirho: CoreModuleChirho,
    pub llvm_ir_chirho: String,
    pub compilation_order_chirho: Vec<String>,
    pub warnings_chirho: Vec<String>,
}

#[derive(Debug)]
pub struct CabalBuildResultChirho {
    pub package_chirho: haskelujah_package_chirho::PackageDescChirho,
    pub build_plan_chirho: haskelujah_package_chirho::BuildPlanChirho,
    pub executables_chirho: Vec<CabalExecutableBuildResultChirho>,
}

#[derive(Debug, Default, Clone)]
struct FrontendSeedArtifactsChirho {
    ifaces_chirho: Vec<ModuleIfaceChirho>,
    imported_types_chirho:
        std::collections::HashMap<String, haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    imported_type_families_chirho: ImportedTypeFamiliesChirho,
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
    index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<CabalCompileResultChirho, String> {
    use haskelujah_package_chirho::{parse_cabal_chirho, resolve_deps_chirho};

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
    let project_dir_chirho = cabal_path_chirho
        .as_ref()
        .parent()
        .unwrap_or_else(|| Path::new("."));

    let builtins_chirho = builtin_dependency_names_chirho()
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    let merged_index_chirho = merge_local_dependency_package_index_chirho(
        project_dir_chirho,
        &all_deps_chirho,
        index_chirho,
    )?;

    // Resolve dependencies.
    let build_plan_chirho =
        resolve_deps_chirho(&all_deps_chirho, &merged_index_chirho, &builtins_chirho)
            .map_err(|e_chirho| format!("dependency resolution failed: {}", e_chirho))?;

    // Discover Haskell source files.
    let source_files_chirho = discover_modules_chirho(&package_chirho, project_dir_chirho);
    let module_cpp_options_chirho =
        package_module_cpp_options_map_chirho(&package_chirho, project_dir_chirho);

    let dep_frontend_artifacts_chirho =
        collect_local_dependency_frontend_artifacts_chirho(project_dir_chirho, &all_deps_chirho)?;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let project_compile_result_chirho = compile_module_files_with_frontend_seed_chirho(
        &source_files_chirho,
        &module_cpp_options_chirho,
        &mut source_map_chirho,
        dep_frontend_artifacts_chirho.ifaces_chirho.clone(),
        dep_frontend_artifacts_chirho.imported_types_chirho,
        dep_frontend_artifacts_chirho.imported_type_synonyms_chirho,
        dep_frontend_artifacts_chirho.imported_type_families_chirho,
    )?;

    Ok(CabalCompileResultChirho {
        package_chirho,
        build_plan_chirho,
        module_results_chirho: project_compile_result_chirho.module_results_chirho,
        compilation_order_chirho: project_compile_result_chirho.compilation_order_chirho,
        warnings_chirho: project_compile_result_chirho.warnings_chirho,
    })
}

pub fn build_cabal_project_chirho(
    cabal_path_chirho: impl AsRef<Path>,
    index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<CabalBuildResultChirho, String> {
    use haskelujah_package_chirho::{parse_cabal_chirho, resolve_deps_chirho};

    let cabal_content_chirho =
        std::fs::read_to_string(cabal_path_chirho.as_ref()).map_err(|e_chirho| {
            format!(
                "cannot read {}: {}",
                cabal_path_chirho.as_ref().display(),
                e_chirho
            )
        })?;
    let package_chirho = parse_cabal_chirho(&cabal_content_chirho);

    let all_deps_chirho = collect_package_deps_chirho(&package_chirho);
    let project_dir_chirho = cabal_path_chirho
        .as_ref()
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let builtins_chirho = builtin_dependency_names_chirho()
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    let merged_index_chirho = merge_local_dependency_package_index_chirho(
        project_dir_chirho,
        &all_deps_chirho,
        index_chirho,
    )?;
    let build_plan_chirho =
        resolve_deps_chirho(&all_deps_chirho, &merged_index_chirho, &builtins_chirho)
            .map_err(|e_chirho| format!("dependency resolution failed: {}", e_chirho))?;

    let mut executables_chirho = Vec::new();
    let dep_frontend_artifacts_chirho =
        collect_local_dependency_frontend_artifacts_chirho(project_dir_chirho, &all_deps_chirho)?;
    for executable_chirho in &package_chirho.executables_chirho {
        let target_modules_chirho = discover_executable_modules_chirho(
            &package_chirho,
            executable_chirho,
            project_dir_chirho,
        );
        let target_module_cpp_options_chirho = module_cpp_options_map_for_module_files_chirho(
            &target_modules_chirho,
            &executable_chirho.build_info_chirho.cpp_options_chirho,
        );
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let project_compile_result_chirho = compile_module_files_with_frontend_seed_chirho(
            &target_modules_chirho,
            &target_module_cpp_options_chirho,
            &mut source_map_chirho,
            dep_frontend_artifacts_chirho.ifaces_chirho.clone(),
            dep_frontend_artifacts_chirho.imported_types_chirho.clone(),
            dep_frontend_artifacts_chirho
                .imported_type_synonyms_chirho
                .clone(),
            dep_frontend_artifacts_chirho
                .imported_type_families_chirho
                .clone(),
        )?;
        let merged_core_chirho = merge_compile_results_core_chirho(
            &project_compile_result_chirho.module_results_chirho,
        )?;
        let llvm_ir_chirho = try_compile_core_to_llvm_executable_chirho(&merged_core_chirho)?;
        executables_chirho.push(CabalExecutableBuildResultChirho {
            name_chirho: executable_chirho.name_chirho.clone(),
            core_chirho: merged_core_chirho,
            llvm_ir_chirho,
            compilation_order_chirho: project_compile_result_chirho.compilation_order_chirho,
            warnings_chirho: project_compile_result_chirho.warnings_chirho,
        });
    }

    Ok(CabalBuildResultChirho {
        package_chirho,
        build_plan_chirho,
        executables_chirho,
    })
}

fn collect_library_package_deps_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
) -> Vec<haskelujah_package_chirho::DependencyChirho> {
    let mut deps_chirho = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();
    for lib_chirho in iter_package_libraries_chirho(package_chirho) {
        for dep_chirho in &lib_chirho.build_info_chirho.build_depends_chirho {
            if is_self_or_internal_library_dep_chirho(package_chirho, &dep_chirho.package_chirho) {
                continue;
            }
            if seen_chirho.insert(dep_chirho.package_chirho.clone()) {
                deps_chirho.push(dep_chirho.clone());
            }
        }
    }

    deps_chirho
}

/// Collect build-depends for the package targets we compile directly.
fn collect_package_deps_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
) -> Vec<haskelujah_package_chirho::DependencyChirho> {
    let mut deps_chirho = collect_library_package_deps_chirho(package_chirho);
    let mut seen_chirho = deps_chirho
        .iter()
        .map(|dep_chirho| dep_chirho.package_chirho.clone())
        .collect::<std::collections::HashSet<_>>();
    for exe_chirho in &package_chirho.executables_chirho {
        for dep_chirho in &exe_chirho.build_info_chirho.build_depends_chirho {
            if is_self_or_internal_library_dep_chirho(package_chirho, &dep_chirho.package_chirho) {
                continue;
            }
            if seen_chirho.insert(dep_chirho.package_chirho.clone()) {
                deps_chirho.push(dep_chirho.clone());
            }
        }
    }
    // Skip test-suite and benchmark dependencies here. They should not affect
    // package resolution for direct library/executable compilation, and they
    // especially must not leak into recursive local dependency indexing.

    deps_chirho
}

fn discover_executable_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    executable_chirho: &haskelujah_package_chirho::ExecutableChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    let mut target_package_chirho = package_chirho.clone();
    target_package_chirho.executables_chirho = vec![executable_chirho.clone()];
    target_package_chirho.test_suites_chirho.clear();
    target_package_chirho.benchmarks_chirho.clear();
    discover_modules_chirho(&target_package_chirho, project_dir_chirho)
}

fn module_cpp_options_map_for_module_files_chirho(
    module_files_chirho: &[(String, PathBuf)],
    cpp_options_chirho: &[String],
) -> std::collections::HashMap<PathBuf, Vec<String>> {
    let mut module_cpp_options_chirho = std::collections::HashMap::new();
    for (_, path_chirho) in module_files_chirho {
        module_cpp_options_chirho.insert(path_chirho.clone(), cpp_options_chirho.to_vec());
    }
    module_cpp_options_chirho
}

fn package_module_cpp_options_map_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> std::collections::HashMap<PathBuf, Vec<String>> {
    let mut module_cpp_options_chirho = std::collections::HashMap::new();

    for library_chirho in iter_package_libraries_chirho(package_chirho) {
        let library_modules_chirho =
            discover_modules_for_library_chirho(library_chirho, project_dir_chirho);
        for (_, path_chirho) in library_modules_chirho {
            module_cpp_options_chirho.insert(
                path_chirho,
                library_chirho.build_info_chirho.cpp_options_chirho.clone(),
            );
        }
    }

    for executable_chirho in &package_chirho.executables_chirho {
        let executable_modules_chirho = discover_executable_modules_chirho(
            package_chirho,
            executable_chirho,
            project_dir_chirho,
        );
        for (_, path_chirho) in executable_modules_chirho {
            module_cpp_options_chirho.insert(
                path_chirho,
                executable_chirho
                    .build_info_chirho
                    .cpp_options_chirho
                    .clone(),
            );
        }
    }

    module_cpp_options_chirho
}

fn compile_module_files_with_frontend_seed_chirho(
    module_files_chirho: &[(String, PathBuf)],
    module_cpp_options_chirho: &std::collections::HashMap<PathBuf, Vec<String>>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
) -> Result<ProjectCompileResultChirho, String> {
    let mut module_sources_chirho: Vec<(String, String, String)> = Vec::new();
    for (module_name_chirho, path_chirho) in module_files_chirho {
        let raw_source_chirho =
            read_haskell_source_file_chirho(path_chirho).map_err(|e_chirho| {
                format!(
                    "cannot read module {} at {}: {}",
                    module_name_chirho,
                    path_chirho.display(),
                    e_chirho
                )
            })?;
        let cpp_options_for_module_chirho = module_cpp_options_chirho
            .get(path_chirho)
            .cloned()
            .unwrap_or_default();
        let source_chirho = preprocess_cpp_source_with_options_chirho(
            path_chirho,
            &raw_source_chirho,
            &cpp_options_for_module_chirho,
        )
        .map_err(|e_chirho| {
            format!(
                "cannot preprocess module {} at {}: {}",
                module_name_chirho,
                path_chirho.display(),
                e_chirho
            )
        })?;
        module_sources_chirho.push((
            module_name_chirho.clone(),
            path_chirho.to_string_lossy().to_string(),
            source_chirho,
        ));
    }
    compile_module_sources_with_extra_ifaces_chirho(
        module_sources_chirho,
        source_map_chirho,
        extra_ifaces_chirho,
        initial_imported_types_chirho,
        initial_imported_type_synonyms_chirho,
        initial_imported_type_families_chirho,
    )
}

/// Scan downloaded dependency packages in .haskelujah-packages-chirho/ for stub
/// module interfaces. This enables cross-package module resolution.
#[cfg(test)]
fn scan_dependency_package_ifaces_chirho(project_dir_chirho: &Path) -> Vec<ModuleIfaceChirho> {
    let mut ifaces_chirho = Vec::new();
    let current_package_dir_chirho = std::fs::canonicalize(project_dir_chirho)
        .unwrap_or_else(|_| project_dir_chirho.to_path_buf());

    // Look for .haskelujah-packages-chirho/ relative to project dir ancestors
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        return ifaces_chirho;
    };

    // Seed the current package first when the caller itself lives under
    // .haskelujah-packages-chirho/. This gives tests and package-scan callers
    // forward module stubs for same-package imports such as containers'
    // Utils.Containers.Internal.BitUtil.
    scan_package_hs_files_chirho(project_dir_chirho, project_dir_chirho, &mut ifaces_chirho);

    // Scan each package directory
    let entries_chirho = match std::fs::read_dir(&packages_dir_chirho) {
        Ok(e_chirho) => e_chirho,
        Err(_) => return ifaces_chirho,
    };

    for entry_chirho in entries_chirho.flatten() {
        let pkg_dir_chirho = entry_chirho.path();
        if !pkg_dir_chirho.is_dir() {
            continue;
        }
        let canonical_pkg_dir_chirho =
            std::fs::canonicalize(&pkg_dir_chirho).unwrap_or_else(|_| pkg_dir_chirho.clone());
        if canonical_pkg_dir_chirho == current_package_dir_chirho {
            continue;
        }
        // Recursively find .hs files and generate stub interfaces
        scan_package_hs_files_chirho(&pkg_dir_chirho, &pkg_dir_chirho, &mut ifaces_chirho);
    }

    ifaces_chirho
}

fn find_dependency_packages_dir_chirho(project_dir_chirho: &Path) -> Option<PathBuf> {
    let mut search_dir_chirho = project_dir_chirho.to_path_buf();
    loop {
        let candidate_chirho = search_dir_chirho.join(".haskelujah-packages-chirho");
        if candidate_chirho.is_dir() {
            return Some(candidate_chirho);
        }
        if !search_dir_chirho.pop() {
            return None;
        }
    }
}

fn find_local_dependency_package_dir_chirho(
    packages_dir_chirho: &Path,
    package_name_chirho: &str,
) -> Option<PathBuf> {
    let mut candidates_chirho =
        find_local_dependency_package_dirs_chirho(packages_dir_chirho, package_name_chirho);
    candidates_chirho.sort();
    candidates_chirho.pop()
}

fn find_local_dependency_package_dirs_chirho(
    packages_dir_chirho: &Path,
    package_name_chirho: &str,
) -> Vec<PathBuf> {
    std::fs::read_dir(packages_dir_chirho)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry_chirho| entry_chirho.path())
        .filter(|path_chirho| {
            path_chirho.is_dir()
                && path_chirho.file_name().is_some_and(|file_name_chirho| {
                    package_dir_matches_dependency_chirho(
                        &file_name_chirho.to_string_lossy(),
                        package_name_chirho,
                    )
                })
        })
        .collect()
}

fn package_dir_matches_dependency_chirho(dir_name_chirho: &str, package_name_chirho: &str) -> bool {
    if dir_name_chirho == package_name_chirho {
        return true;
    }
    let prefix_chirho = format!("{package_name_chirho}-");
    let Some(version_suffix_chirho) = dir_name_chirho.strip_prefix(&prefix_chirho) else {
        return false;
    };
    version_suffix_chirho
        .chars()
        .next()
        .is_some_and(|char_chirho| char_chirho.is_ascii_digit())
}

fn builtin_dependency_names_chirho() -> std::collections::HashSet<String> {
    [
        "base",
        "ghc-prim",
        "ghc-bignum",
        "ghc-internal",
        "rts",
        "deepseq",
        "array",
        "bytestring",
        "containers",
        "text",
        "filepath",
        "directory",
        "process",
        "time",
        "template-haskell",
        "pretty",
        "binary",
        "integer-gmp",
        "ghc-boot-th",
        "ghc-boot",
        "os-string",
        "unix",
        "Win32",
    ]
    .into_iter()
    .map(|name_chirho| name_chirho.to_string())
    .collect()
}

fn merge_local_dependency_package_index_chirho(
    project_dir_chirho: &Path,
    _root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
    base_index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<haskelujah_package_chirho::PackageIndexChirho, String> {
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        return Ok(base_index_chirho.clone());
    };

    let builtin_deps_chirho = builtin_dependency_names_chirho();
    let mut merged_index_chirho = base_index_chirho.clone();
    let mut package_dirs_chirho: Vec<PathBuf> = std::fs::read_dir(&packages_dir_chirho)
        .map_err(|error_chirho| {
            format!(
                "cannot read local dependency directory {}: {}",
                packages_dir_chirho.display(),
                error_chirho
            )
        })?
        .filter_map(|entry_chirho| entry_chirho.ok().map(|entry_chirho| entry_chirho.path()))
        .filter(|path_chirho| path_chirho.is_dir())
        .collect();
    package_dirs_chirho.sort();

    for package_dir_chirho in package_dirs_chirho {
        let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) else {
            continue;
        };

        let Ok(cabal_content_chirho) = std::fs::read_to_string(&cabal_path_chirho) else {
            continue;
        };
        let package_chirho = haskelujah_package_chirho::parse_cabal_chirho(&cabal_content_chirho);
        if package_chirho.name_chirho.is_empty() {
            continue;
        }
        if builtin_deps_chirho.contains(&package_chirho.name_chirho) {
            continue;
        }

        let Some(package_version_chirho) = package_chirho.version_chirho.clone().or_else(|| {
            package_dir_chirho
                .file_name()
                .and_then(|file_name_chirho| file_name_chirho.to_str())
                .and_then(|dir_name_chirho| {
                    dir_name_chirho
                        .strip_prefix(&format!("{}-", package_chirho.name_chirho))
                        .and_then(haskelujah_package_chirho::parse_version_chirho)
                })
        }) else {
            continue;
        };
        let package_deps_chirho = collect_library_package_deps_chirho(&package_chirho);

        let already_present_chirho = merged_index_chirho
            .packages_chirho
            .get(&package_chirho.name_chirho)
            .is_some_and(|versions_chirho| {
                versions_chirho
                    .iter()
                    .any(|meta_chirho| meta_chirho.version_chirho == package_version_chirho)
            });
        if !already_present_chirho {
            merged_index_chirho.add_package_chirho(
                &package_chirho.name_chirho,
                package_version_chirho,
                package_deps_chirho,
            );
        }
    }

    Ok(merged_index_chirho)
}

fn collect_local_dependency_frontend_artifacts_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
) -> Result<FrontendSeedArtifactsChirho, String> {
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        return Ok(FrontendSeedArtifactsChirho::default());
    };

    let builtin_deps_chirho = builtin_dependency_names_chirho();
    let mut artifacts_chirho = FrontendSeedArtifactsChirho::default();
    // Seed PrimState type family equations so dependency packages like
    // primitive can resolve PrimState (ST s) = s during compilation.
    artifacts_chirho.imported_type_families_chirho = seed_builtin_type_families_chirho();
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut visited_chirho = std::collections::HashSet::new();
    let mut active_chirho = std::collections::HashSet::new();

    for dep_chirho in root_deps_chirho {
        if let Err(error_chirho) = compile_local_dependency_package_frontend_recursive_chirho(
            &dep_chirho.package_chirho,
            &packages_dir_chirho,
            &builtin_deps_chirho,
            &mut visited_chirho,
            &mut active_chirho,
            &mut artifacts_chirho,
            &mut source_map_chirho,
        ) {
            eprintln!(
                "warning: local dependency '{}' frontend compile skipped: {}",
                dep_chirho.package_chirho, error_chirho
            );
        }
    }

    Ok(artifacts_chirho)
}

fn compile_local_dependency_package_frontend_recursive_chirho(
    package_name_chirho: &str,
    packages_dir_chirho: &Path,
    builtin_deps_chirho: &std::collections::HashSet<String>,
    visited_chirho: &mut std::collections::HashSet<String>,
    active_chirho: &mut std::collections::HashSet<String>,
    artifacts_chirho: &mut FrontendSeedArtifactsChirho,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<(), String> {
    if builtin_deps_chirho.contains(package_name_chirho)
        || visited_chirho.contains(package_name_chirho)
    {
        return Ok(());
    }
    if !active_chirho.insert(package_name_chirho.to_string()) {
        return Ok(());
    }

    let Some(package_dir_chirho) =
        find_local_dependency_package_dir_chirho(packages_dir_chirho, package_name_chirho)
    else {
        active_chirho.remove(package_name_chirho);
        return Ok(());
    };

    let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) else {
        active_chirho.remove(package_name_chirho);
        return Ok(());
    };

    let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho).map_err(|e_chirho| {
        format!(
            "cannot read dependency cabal {}: {}",
            cabal_path_chirho.display(),
            e_chirho
        )
    })?;
    let package_chirho = haskelujah_package_chirho::parse_cabal_chirho(&cabal_content_chirho);

    for dep_chirho in collect_library_package_deps_chirho(&package_chirho) {
        if let Err(error_chirho) = compile_local_dependency_package_frontend_recursive_chirho(
            &dep_chirho.package_chirho,
            packages_dir_chirho,
            builtin_deps_chirho,
            visited_chirho,
            active_chirho,
            artifacts_chirho,
            source_map_chirho,
        ) {
            eprintln!(
                "warning: transitive local dependency '{}' frontend compile skipped: {}",
                dep_chirho.package_chirho, error_chirho
            );
        }
    }

    let source_files_chirho = discover_library_modules_chirho(&package_chirho, &package_dir_chirho);
    let source_file_cpp_options_chirho =
        package_module_cpp_options_map_chirho(&package_chirho, &package_dir_chirho);
    let mut module_sources_chirho = Vec::new();
    for (module_name_chirho, path_chirho) in &source_files_chirho {
        let raw_source_chirho =
            read_haskell_source_file_chirho(path_chirho).map_err(|e_chirho| {
                format!(
                    "cannot read dependency module {} at {}: {}",
                    module_name_chirho,
                    path_chirho.display(),
                    e_chirho
                )
            })?;
        let cpp_options_for_module_chirho = source_file_cpp_options_chirho
            .get(path_chirho)
            .cloned()
            .unwrap_or_default();
        let source_chirho = preprocess_cpp_source_with_options_chirho(
            path_chirho,
            &raw_source_chirho,
            &cpp_options_for_module_chirho,
        )
        .map_err(|e_chirho| {
            format!(
                "cannot preprocess dependency module {} at {}: {}",
                module_name_chirho,
                path_chirho.display(),
                e_chirho
            )
        })?;
        module_sources_chirho.push((
            module_name_chirho.clone(),
            path_chirho.to_string_lossy().to_string(),
            source_chirho,
        ));
    }

    let compiled_artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        module_sources_chirho,
        source_map_chirho,
        artifacts_chirho.ifaces_chirho.clone(),
        artifacts_chirho.imported_types_chirho.clone(),
        artifacts_chirho.imported_type_synonyms_chirho.clone(),
        artifacts_chirho.imported_type_families_chirho.clone(),
        true,
    )
    .map_err(|error_chirho| {
        format!(
            "dependency package '{}': {}",
            package_name_chirho, error_chirho
        )
    })?;

    artifacts_chirho.ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(
            compiled_artifacts_chirho.ifaces_chirho,
        );
    artifacts_chirho
        .imported_types_chirho
        .extend(compiled_artifacts_chirho.imported_types_chirho);
    artifacts_chirho
        .imported_type_synonyms_chirho
        .extend(compiled_artifacts_chirho.imported_type_synonyms_chirho);
    extend_imported_type_families_chirho(
        &mut artifacts_chirho.imported_type_families_chirho,
        compiled_artifacts_chirho.imported_type_families_chirho,
    );

    active_chirho.remove(package_name_chirho);
    visited_chirho.insert(package_name_chirho.to_string());
    Ok(())
}

/// Recursively scan a package directory for .hs files and generate stub interfaces.
#[cfg(test)]
fn scan_package_hs_files_chirho(
    root_chirho: &Path,
    dir_chirho: &Path,
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
) {
    let entries_chirho = match std::fs::read_dir(dir_chirho) {
        Ok(e_chirho) => e_chirho,
        Err(_) => return,
    };

    for entry_chirho in entries_chirho.flatten() {
        let path_chirho = entry_chirho.path();
        if path_chirho.is_dir() {
            let dir_name_chirho = path_chirho
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            // Skip hidden dirs, dist, test, bench directories
            if dir_name_chirho.starts_with('.')
                || dir_name_chirho == "dist-chirho"
                || dir_name_chirho == "tests"
                || dir_name_chirho == "test"
                || dir_name_chirho == "bench"
                || dir_name_chirho == "benchmarks"
            {
                continue;
            }
            scan_package_hs_files_chirho(root_chirho, &path_chirho, ifaces_chirho);
        } else if path_chirho.extension().is_some_and(|extension_chirho| {
            extension_chirho == "hs" || extension_chirho == "lhs" || extension_chirho == "hsc"
        }) {
            let file_name_chirho = path_chirho
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            if file_name_chirho == "Setup.hs"
                || file_name_chirho == "Setup.lhs"
                || file_name_chirho == "Setup.hsc"
            {
                continue;
            }
            // Read source and extract module name + exports
            if let Ok(preprocessed_source_chirho) = read_haskell_source_file_chirho(&path_chirho) {
                let mut stub_iface_chirho = None;
                if let Some(mod_name_chirho) =
                    extract_module_name_from_source_chirho(&preprocessed_source_chirho)
                {
                    use haskelujah_naming_chirho::iface_chirho::{
                        IfaceExportsChirho, IfaceTypeChirho, IfaceValueChirho,
                    };
                    let span_chirho = haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO;
                    let mut exports_chirho = IfaceExportsChirho::default();
                    for name_chirho in
                        extract_exported_names_from_source_chirho(&preprocessed_source_chirho)
                    {
                        let first_char_chirho = name_chirho.chars().next().unwrap_or('a');
                        if first_char_chirho.is_uppercase() {
                            exports_chirho.types_chirho.insert(
                                name_chirho.clone(),
                                IfaceTypeChirho {
                                    name_chirho: name_chirho.clone(),
                                    constructors_chirho: vec![],
                                    methods_chirho: vec![],
                                    span_chirho,
                                },
                            );
                            exports_chirho.values_chirho.insert(
                                name_chirho.clone(),
                                IfaceValueChirho {
                                    name_chirho: name_chirho.clone(),
                                    span_chirho,
                                },
                            );
                        } else {
                            exports_chirho.values_chirho.insert(
                                name_chirho.clone(),
                                IfaceValueChirho {
                                    name_chirho: name_chirho.clone(),
                                    span_chirho,
                                },
                            );
                        }
                    }
                    stub_iface_chirho = Some(ModuleIfaceChirho {
                        name_chirho: mod_name_chirho,
                        exports_chirho,
                    });
                }

                let real_iface_result_chirho =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut source_map_chirho = SourceMapChirho::new_chirho();
                        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                            &mut source_map_chirho,
                            &path_chirho,
                            &preprocessed_source_chirho,
                        );
                        let file_id_chirho = source_file_chirho.file_id_chirho();
                        let parser_chirho =
                            ParserChirho::new_chirho(&preprocessed_source_chirho, file_id_chirho);
                        let green_chirho = parser_chirho.parse_chirho();
                        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
                        build_iface_with_imports_chirho(&module_chirho, ifaces_chirho)
                    }));

                if let Ok(real_iface_chirho) = real_iface_result_chirho {
                    ifaces_chirho.push(real_iface_chirho);
                } else if let Some(stub_iface_chirho) = stub_iface_chirho {
                    ifaces_chirho.push(stub_iface_chirho);
                }
            }
        }
    }
}

fn insert_exported_schemes_into_imports_chirho(
    iface_chirho: &ModuleIfaceChirho,
    infer_result_chirho: &InferResultChirho,
    imported_types_chirho: &mut std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
) {
    for (name_chirho, _val_chirho) in &iface_chirho.exports_chirho.values_chirho {
        if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(name_chirho) {
            imported_types_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
            imported_types_chirho.insert(
                format!("{}.{}", iface_chirho.name_chirho, name_chirho),
                scheme_chirho.clone(),
            );
        }
    }
    for (_name_chirho, ty_info_chirho) in &iface_chirho.exports_chirho.types_chirho {
        for con_name_chirho in &ty_info_chirho.constructors_chirho {
            if let Some(scheme_chirho) = infer_result_chirho
                .env_chirho
                .lookup_chirho(con_name_chirho)
            {
                imported_types_chirho.insert(con_name_chirho.clone(), scheme_chirho.clone());
                imported_types_chirho.insert(
                    format!("{}.{}", iface_chirho.name_chirho, con_name_chirho),
                    scheme_chirho.clone(),
                );
            }
        }
        for method_name_chirho in &ty_info_chirho.methods_chirho {
            if let Some(scheme_chirho) = infer_result_chirho
                .env_chirho
                .lookup_chirho(method_name_chirho)
            {
                imported_types_chirho.insert(method_name_chirho.clone(), scheme_chirho.clone());
                imported_types_chirho.insert(
                    format!("{}.{}", iface_chirho.name_chirho, method_name_chirho),
                    scheme_chirho.clone(),
                );
            }
        }
    }
}

fn collect_frontend_artifacts_from_module_sources_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
    seed_stdlib_frontend_artifacts_chirho: bool,
) -> Result<FrontendSeedArtifactsChirho, String> {
    if module_sources_chirho.is_empty() {
        return Ok(FrontendSeedArtifactsChirho {
            ifaces_chirho: haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(
                haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
            ),
            imported_types_chirho: initial_imported_types_chirho,
            imported_type_synonyms_chirho: initial_imported_type_synonyms_chirho,
            imported_type_families_chirho: initial_imported_type_families_chirho,
        });
    }

    let mut dep_graph_chirho = haskelujah_incremental_chirho::DepGraphChirho::new_chirho();
    let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();

    for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
        let fp_chirho =
            haskelujah_incremental_chirho::FingerprintChirho::from_str_chirho(source_chirho);
        dep_graph_chirho.add_module_chirho(module_name_chirho, fp_chirho);
        for imported_chirho in extract_imports_chirho(source_chirho) {
            if known_modules_chirho.contains(&imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_chirho);
            }
        }
    }

    let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    // Add cross-package dependency interfaces
    ifaces_chirho.extend(extra_ifaces_chirho);
    let mut ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(ifaces_chirho);
    let mut imported_types_chirho = initial_imported_types_chirho;
    let mut imported_type_synonyms_chirho = initial_imported_type_synonyms_chirho;
    let mut imported_type_families_chirho = initial_imported_type_families_chirho;
    let mut imported_class_env_chirho =
        haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();
    if seed_stdlib_frontend_artifacts_chirho
        && module_sources_chirho
            .iter()
            .any(|(_, _, source_chirho)| source_imports_stdlib_chirho(source_chirho))
    {
        merge_stdlib_frontend_artifacts_chirho(
            &mut ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        );
    }

    for scc_chirho in &sccs_chirho {
        for module_name_chirho in scc_chirho {
            let (_, file_name_chirho, source_chirho) = module_sources_chirho
                .iter()
                .find(|(name_chirho, _, _)| name_chirho == module_name_chirho)
                .ok_or_else(|| format!("Module {} not found in sources", module_name_chirho))?;

            let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                source_map_chirho,
                file_name_chirho,
                source_chirho,
            );
            let file_id_chirho = source_file_chirho.file_id_chirho();
            let filtered_imported_types_chirho = filter_seeded_imported_types_for_source_chirho(
                source_chirho,
                &imported_types_chirho,
            );
            let filtered_imported_type_synonyms_chirho =
                filter_seeded_type_synonyms_for_source_chirho(
                    source_chirho,
                    &imported_type_synonyms_chirho,
                );

            let frontend_result_chirho =
                run_frontend_with_type_synonyms_families_and_class_env_chirho(
                    source_chirho,
                    file_id_chirho,
                    &ifaces_chirho,
                    &filtered_imported_types_chirho,
                    &filtered_imported_type_synonyms_chirho,
                    &imported_type_families_chirho,
                    &imported_class_env_chirho,
                )
                .map_err(|e_chirho| {
                    format!("Error compiling {}: {}", module_name_chirho, e_chirho)
                })?;

            let FrontendResultChirho {
                module_chirho,
                infer_result_chirho,
                resolved_imported_type_synonyms_chirho,
                warnings_chirho: _warnings_chirho,
            } = frontend_result_chirho;

            let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);
            insert_exported_schemes_into_imports_chirho(
                &iface_chirho,
                &infer_result_chirho,
                &mut imported_types_chirho,
            );
            imported_type_synonyms_chirho.extend(exported_type_synonyms_from_module_chirho(
                &module_chirho,
                &iface_chirho,
                &resolved_imported_type_synonyms_chirho,
            ));
            imported_type_families_chirho = infer_result_chirho.type_families_chirho.clone();
            imported_class_env_chirho = infer_result_chirho.class_env_chirho.clone();
            ifaces_chirho.push(iface_chirho);
        }
    }

    Ok(FrontendSeedArtifactsChirho {
        ifaces_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_type_families_chirho,
    })
}

fn compile_module_sources_with_extra_ifaces_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
) -> Result<ProjectCompileResultChirho, String> {
    if module_sources_chirho.is_empty() {
        return Ok(ProjectCompileResultChirho {
            module_results_chirho: Vec::new(),
            compilation_order_chirho: Vec::new(),
            warnings_chirho: Vec::new(),
        });
    }

    let mut dep_graph_chirho = haskelujah_incremental_chirho::DepGraphChirho::new_chirho();
    let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();

    for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
        let fp_chirho =
            haskelujah_incremental_chirho::FingerprintChirho::from_str_chirho(source_chirho);
        dep_graph_chirho.add_module_chirho(module_name_chirho, fp_chirho);
        for imported_chirho in extract_imports_chirho(source_chirho) {
            if known_modules_chirho.contains(&imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_chirho);
            }
        }
    }

    let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();
    let mut results_chirho: Vec<CompileResultChirho> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    ifaces_chirho.extend(extra_ifaces_chirho);
    let mut ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(ifaces_chirho);
    let mut all_warnings_chirho: Vec<String> = Vec::new();
    let mut imported_types_chirho = initial_imported_types_chirho;
    let mut imported_type_synonyms_chirho = initial_imported_type_synonyms_chirho;
    let mut imported_type_families_chirho = initial_imported_type_families_chirho;
    let mut imported_class_env_chirho =
        haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();
    if module_sources_chirho
        .iter()
        .any(|(_, _, source_chirho)| source_imports_stdlib_chirho(source_chirho))
    {
        merge_stdlib_frontend_artifacts_chirho(
            &mut ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        );
    }
    let mut order_chirho: Vec<String> = Vec::new();

    for scc_chirho in &sccs_chirho {
        for module_name_chirho in scc_chirho {
            let (_, file_name_chirho, source_chirho) = module_sources_chirho
                .iter()
                .find(|(name_chirho, _, _)| name_chirho == module_name_chirho)
                .ok_or_else(|| format!("Module {} not found in sources", module_name_chirho))?;

            let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                source_map_chirho,
                file_name_chirho,
                source_chirho,
            );
            let file_id_chirho = source_file_chirho.file_id_chirho();
            let filtered_imported_types_chirho = filter_seeded_imported_types_for_source_chirho(
                source_chirho,
                &imported_types_chirho,
            );
            let filtered_imported_type_synonyms_chirho =
                filter_seeded_type_synonyms_for_source_chirho(
                    source_chirho,
                    &imported_type_synonyms_chirho,
                );

            let frontend_result_chirho =
                run_frontend_with_type_synonyms_families_and_class_env_chirho(
                    source_chirho,
                    file_id_chirho,
                    &ifaces_chirho,
                    &filtered_imported_types_chirho,
                    &filtered_imported_type_synonyms_chirho,
                    &imported_type_families_chirho,
                    &imported_class_env_chirho,
                )
                .map_err(|e_chirho| {
                    format!("Error compiling {}: {}", module_name_chirho, e_chirho)
                })?;

            let FrontendResultChirho {
                module_chirho,
                infer_result_chirho,
                resolved_imported_type_synonyms_chirho,
                warnings_chirho,
            } = frontend_result_chirho;

            all_warnings_chirho.extend(warnings_chirho);

            let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);
            let extra_dict_param_names_chirho: std::collections::HashSet<String> =
                imported_types_chirho.keys().cloned().collect();
            insert_exported_schemes_into_imports_chirho(
                &iface_chirho,
                &infer_result_chirho,
                &mut imported_types_chirho,
            );
            imported_type_synonyms_chirho.extend(exported_type_synonyms_from_module_chirho(
                &module_chirho,
                &iface_chirho,
                &resolved_imported_type_synonyms_chirho,
            ));
            imported_type_families_chirho = infer_result_chirho.type_families_chirho.clone();
            imported_class_env_chirho = infer_result_chirho.class_env_chirho.clone();
            ifaces_chirho.push(iface_chirho);

            let compile_result_chirho = compile_backend_chirho(
                module_chirho,
                infer_result_chirho,
                extra_dict_param_names_chirho,
            )
            .map_err(|e_chirho| {
                format!("Backend error for {}: {}", module_name_chirho, e_chirho)
            })?;

            results_chirho.push(compile_result_chirho);
            order_chirho.push(module_name_chirho.clone());
        }
    }

    Ok(ProjectCompileResultChirho {
        module_results_chirho: results_chirho,
        compilation_order_chirho: order_chirho,
        warnings_chirho: all_warnings_chirho,
    })
}

fn merge_compile_results_core_chirho(
    results_chirho: &[CompileResultChirho],
) -> Result<CoreModuleChirho, String> {
    use haskelujah_core_chirho::expr_chirho::{CoreExprChirho, CoreIdChirho};
    use std::collections::{HashMap, HashSet};

    if results_chirho.is_empty() {
        return Err("no compiled modules to merge".to_string());
    }

    fn offset_id_chirho(id_chirho: CoreIdChirho, off_chirho: u32) -> CoreIdChirho {
        CoreIdChirho(id_chirho.0 + off_chirho)
    }

    fn offset_binder_chirho(
        binder_chirho: &mut haskelujah_core_chirho::expr_chirho::BinderChirho,
        off_chirho: u32,
    ) {
        binder_chirho.id_chirho = offset_id_chirho(binder_chirho.id_chirho, off_chirho);
    }

    fn offset_expr_chirho(expr_chirho: &mut CoreExprChirho, off_chirho: u32) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                *id_chirho = offset_id_chirho(*id_chirho, off_chirho);
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                offset_expr_chirho(fun_chirho, off_chirho);
                offset_expr_chirho(arg_chirho, off_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                offset_binder_chirho(binder_chirho, off_chirho);
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    offset_binder_chirho(binder_chirho, off_chirho);
                    offset_expr_chirho(rhs_chirho, off_chirho);
                }
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                offset_binder_chirho(bind_chirho, off_chirho);
                offset_expr_chirho(scrutinee_chirho, off_chirho);
                for alt_chirho in alts_chirho {
                    for binder_chirho in &mut alt_chirho.binders_chirho {
                        offset_binder_chirho(binder_chirho, off_chirho);
                    }
                    offset_expr_chirho(&mut alt_chirho.rhs_chirho, off_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                offset_expr_chirho(inner_chirho, off_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    offset_expr_chirho(arg_chirho, off_chirho);
                }
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
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                remap_expr_chirho(fun_chirho, remap_chirho);
                remap_expr_chirho(arg_chirho, remap_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_binder_chirho, rhs_chirho) in binds_chirho {
                    remap_expr_chirho(rhs_chirho, remap_chirho);
                }
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                remap_expr_chirho(scrutinee_chirho, remap_chirho);
                for alt_chirho in alts_chirho {
                    remap_expr_chirho(&mut alt_chirho.rhs_chirho, remap_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                remap_expr_chirho(inner_chirho, remap_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    remap_expr_chirho(arg_chirho, remap_chirho);
                }
            }
        }
    }

    fn collect_binder_ids_chirho(
        expr_chirho: &CoreExprChirho,
        ids_chirho: &mut HashSet<CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                collect_binder_ids_chirho(fun_chirho, ids_chirho);
                collect_binder_ids_chirho(arg_chirho, ids_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                ids_chirho.insert(binder_chirho.id_chirho);
                collect_binder_ids_chirho(body_chirho, ids_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    ids_chirho.insert(binder_chirho.id_chirho);
                    collect_binder_ids_chirho(rhs_chirho, ids_chirho);
                }
                collect_binder_ids_chirho(body_chirho, ids_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                ids_chirho.insert(bind_chirho.id_chirho);
                collect_binder_ids_chirho(scrutinee_chirho, ids_chirho);
                for alt_chirho in alts_chirho {
                    for binder_chirho in &alt_chirho.binders_chirho {
                        ids_chirho.insert(binder_chirho.id_chirho);
                    }
                    collect_binder_ids_chirho(&alt_chirho.rhs_chirho, ids_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                collect_binder_ids_chirho(body_chirho, ids_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                collect_binder_ids_chirho(inner_chirho, ids_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    collect_binder_ids_chirho(arg_chirho, ids_chirho);
                }
            }
        }
    }

    fn canonical_binder_ty_chirho(
        binder_chirho: &haskelujah_core_chirho::expr_chirho::BinderChirho,
    ) -> String {
        binder_chirho.ty_chirho.to_string()
    }

    fn canonical_expr_chirho(
        expr_chirho: &CoreExprChirho,
        names_chirho: &HashMap<CoreIdChirho, String>,
        local_ids_chirho: &mut HashMap<CoreIdChirho, usize>,
        next_local_id_chirho: &mut usize,
    ) -> String {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(local_id_chirho) = local_ids_chirho.get(id_chirho) {
                    format!("local:{local_id_chirho}")
                } else if let Some(name_chirho) = names_chirho.get(id_chirho) {
                    format!("global:{name_chirho}")
                } else {
                    format!("unresolved:{}", id_chirho.0)
                }
            }
            CoreExprChirho::LitChirho(lit_chirho) => format!("lit:{lit_chirho:?}"),
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => format!(
                "app({},{})",
                canonical_expr_chirho(
                    fun_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                ),
                canonical_expr_chirho(
                    arg_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                )
            ),
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let local_slot_chirho = *next_local_id_chirho;
                *next_local_id_chirho += 1;
                let previous_local_chirho =
                    local_ids_chirho.insert(binder_chirho.id_chirho, local_slot_chirho);
                let body_key_chirho = canonical_expr_chirho(
                    body_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                );
                if let Some(prev_local_chirho) = previous_local_chirho {
                    local_ids_chirho.insert(binder_chirho.id_chirho, prev_local_chirho);
                } else {
                    local_ids_chirho.remove(&binder_chirho.id_chirho);
                }
                format!(
                    "lam({}:{})",
                    canonical_binder_ty_chirho(binder_chirho),
                    body_key_chirho
                )
            }
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                let let_binder_ids_chirho: Vec<CoreIdChirho> = binds_chirho
                    .iter()
                    .map(|(binder_chirho, _)| binder_chirho.id_chirho)
                    .collect();
                let mut previous_ids_chirho = Vec::with_capacity(let_binder_ids_chirho.len());
                for binder_id_chirho in &let_binder_ids_chirho {
                    let local_slot_chirho = *next_local_id_chirho;
                    *next_local_id_chirho += 1;
                    previous_ids_chirho.push((
                        *binder_id_chirho,
                        local_ids_chirho.insert(*binder_id_chirho, local_slot_chirho),
                    ));
                }
                let binds_key_chirho = binds_chirho
                    .iter()
                    .map(|(binder_chirho, rhs_chirho)| {
                        format!(
                            "{}={}",
                            canonical_binder_ty_chirho(binder_chirho),
                            canonical_expr_chirho(
                                rhs_chirho,
                                names_chirho,
                                local_ids_chirho,
                                next_local_id_chirho,
                            )
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";");
                let body_key_chirho = canonical_expr_chirho(
                    body_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                );
                for (binder_id_chirho, previous_local_chirho) in previous_ids_chirho {
                    if let Some(prev_local_chirho) = previous_local_chirho {
                        local_ids_chirho.insert(binder_id_chirho, prev_local_chirho);
                    } else {
                        local_ids_chirho.remove(&binder_id_chirho);
                    }
                }
                format!("let(rec={rec_chirho};binds=[{binds_key_chirho}];body={body_key_chirho})")
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => {
                let scrut_key_chirho = canonical_expr_chirho(
                    scrutinee_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                );
                let bind_slot_chirho = *next_local_id_chirho;
                *next_local_id_chirho += 1;
                let previous_bind_chirho =
                    local_ids_chirho.insert(bind_chirho.id_chirho, bind_slot_chirho);
                let alt_keys_chirho = alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let alt_binder_ids_chirho: Vec<CoreIdChirho> = alt_chirho
                            .binders_chirho
                            .iter()
                            .map(|binder_chirho| binder_chirho.id_chirho)
                            .collect();
                        let mut previous_alt_ids_chirho =
                            Vec::with_capacity(alt_binder_ids_chirho.len());
                        for binder_id_chirho in &alt_binder_ids_chirho {
                            let local_slot_chirho = *next_local_id_chirho;
                            *next_local_id_chirho += 1;
                            previous_alt_ids_chirho.push((
                                *binder_id_chirho,
                                local_ids_chirho.insert(*binder_id_chirho, local_slot_chirho),
                            ));
                        }
                        let binders_key_chirho = alt_chirho
                            .binders_chirho
                            .iter()
                            .map(canonical_binder_ty_chirho)
                            .collect::<Vec<_>>()
                            .join(",");
                        let rhs_key_chirho = canonical_expr_chirho(
                            &alt_chirho.rhs_chirho,
                            names_chirho,
                            local_ids_chirho,
                            next_local_id_chirho,
                        );
                        for (binder_id_chirho, previous_local_chirho) in previous_alt_ids_chirho {
                            if let Some(prev_local_chirho) = previous_local_chirho {
                                local_ids_chirho.insert(binder_id_chirho, prev_local_chirho);
                            } else {
                                local_ids_chirho.remove(&binder_id_chirho);
                            }
                        }
                        format!(
                            "alt({:?};{};{})",
                            alt_chirho.con_chirho, binders_key_chirho, rhs_key_chirho
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";");
                if let Some(prev_local_chirho) = previous_bind_chirho {
                    local_ids_chirho.insert(bind_chirho.id_chirho, prev_local_chirho);
                } else {
                    local_ids_chirho.remove(&bind_chirho.id_chirho);
                }
                format!(
                    "case({};{};{};alts=[{}])",
                    scrut_key_chirho, bind_slot_chirho, result_ty_chirho, alt_keys_chirho
                )
            }
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => format!(
                "tylam({};{})",
                ty_var_chirho,
                canonical_expr_chirho(
                    body_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                )
            ),
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => format!(
                "tyapp({};{})",
                canonical_expr_chirho(
                    inner_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                ),
                ty_chirho
            ),
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => format!(
                "primop({};[{}])",
                name_chirho,
                args_chirho
                    .iter()
                    .map(|arg_chirho| canonical_expr_chirho(
                        arg_chirho,
                        names_chirho,
                        local_ids_chirho,
                        next_local_id_chirho,
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => format!(
                "con({};[{}])",
                con_name_chirho,
                args_chirho
                    .iter()
                    .map(|arg_chirho| canonical_expr_chirho(
                        arg_chirho,
                        names_chirho,
                        local_ids_chirho,
                        next_local_id_chirho,
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }

    fn canonical_binding_key_chirho(
        binding_chirho: &haskelujah_core_chirho::expr_chirho::CoreBindingChirho,
        names_chirho: &HashMap<CoreIdChirho, String>,
    ) -> String {
        let mut local_ids_chirho = HashMap::new();
        let mut next_local_id_chirho = 0usize;
        format!(
            "binding({};{};rec={};inline={:?};{})",
            binding_chirho.binder_chirho.name_chirho,
            canonical_binder_ty_chirho(&binding_chirho.binder_chirho),
            binding_chirho.is_rec_chirho,
            binding_chirho.inline_chirho,
            canonical_expr_chirho(
                &binding_chirho.rhs_chirho,
                names_chirho,
                &mut local_ids_chirho,
                &mut next_local_id_chirho,
            )
        )
    }

    let mut merged_bindings_chirho = Vec::new();
    let mut merged_names_chirho: HashMap<CoreIdChirho, String> = HashMap::new();
    let mut id_offset_chirho: u32 = 0;

    for result_chirho in results_chirho {
        let mut bindings_chirho = result_chirho.core_chirho.bindings_chirho.clone();
        if id_offset_chirho > 0 {
            for binding_chirho in &mut bindings_chirho {
                offset_binder_chirho(&mut binding_chirho.binder_chirho, id_offset_chirho);
                offset_expr_chirho(&mut binding_chirho.rhs_chirho, id_offset_chirho);
            }
        }
        for (id_chirho, name_chirho) in &result_chirho.core_chirho.names_chirho {
            merged_names_chirho.insert(
                offset_id_chirho(*id_chirho, id_offset_chirho),
                name_chirho.clone(),
            );
        }
        merged_bindings_chirho.extend(bindings_chirho);
        let max_id_chirho = result_chirho
            .core_chirho
            .names_chirho
            .keys()
            .map(|id_chirho| id_chirho.0)
            .max()
            .unwrap_or(0);
        id_offset_chirho += max_id_chirho + 1;
    }

    let mut def_name_to_id_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
    for binding_chirho in &merged_bindings_chirho {
        def_name_to_id_chirho
            .entry(binding_chirho.binder_chirho.name_chirho.clone())
            .or_insert(binding_chirho.binder_chirho.id_chirho);
    }

    let mut all_binder_ids_chirho: HashSet<CoreIdChirho> = HashSet::new();
    for binding_chirho in &merged_bindings_chirho {
        all_binder_ids_chirho.insert(binding_chirho.binder_chirho.id_chirho);
        collect_binder_ids_chirho(&binding_chirho.rhs_chirho, &mut all_binder_ids_chirho);
    }

    let mut remap_chirho: HashMap<CoreIdChirho, CoreIdChirho> = HashMap::new();
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

    if !remap_chirho.is_empty() {
        for binding_chirho in &mut merged_bindings_chirho {
            remap_expr_chirho(&mut binding_chirho.rhs_chirho, &remap_chirho);
        }
    }

    let mut canonical_binding_to_id_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
    let mut deduped_bindings_chirho = Vec::with_capacity(merged_bindings_chirho.len());
    let mut removed_binding_ids_chirho = HashSet::new();
    let mut duplicate_binding_remap_chirho: HashMap<CoreIdChirho, CoreIdChirho> = HashMap::new();
    for binding_chirho in merged_bindings_chirho {
        let canonical_key_chirho =
            canonical_binding_key_chirho(&binding_chirho, &merged_names_chirho);
        if let Some(&kept_id_chirho) = canonical_binding_to_id_chirho.get(&canonical_key_chirho) {
            duplicate_binding_remap_chirho
                .insert(binding_chirho.binder_chirho.id_chirho, kept_id_chirho);
            removed_binding_ids_chirho.insert(binding_chirho.binder_chirho.id_chirho);
        } else {
            canonical_binding_to_id_chirho
                .insert(canonical_key_chirho, binding_chirho.binder_chirho.id_chirho);
            deduped_bindings_chirho.push(binding_chirho);
        }
    }

    if !duplicate_binding_remap_chirho.is_empty() {
        for binding_chirho in &mut deduped_bindings_chirho {
            remap_expr_chirho(
                &mut binding_chirho.rhs_chirho,
                &duplicate_binding_remap_chirho,
            );
        }
        merged_names_chirho.retain(|id_chirho, _| !removed_binding_ids_chirho.contains(id_chirho));
    }

    Ok(CoreModuleChirho {
        name_chirho: results_chirho
            .last()
            .map(|result_chirho| {
                result_chirho
                    .module_chirho
                    .name_chirho
                    .text_chirho()
                    .to_string()
            })
            .unwrap_or_else(|| "Main".to_string()),
        bindings_chirho: deduped_bindings_chirho,
        names_chirho: merged_names_chirho,
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
    })
}

/// Discover Haskell module files from a package description.
///
/// Returns `(module_name, file_path)` pairs. Searches `hs-source-dirs`
/// for each exposed/other module, converting dotted module names to paths.
pub fn discover_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    let mut modules_chirho = discover_library_modules_chirho(package_chirho, project_dir_chirho);
    let mut seen_chirho = modules_chirho
        .iter()
        .map(|(module_name_chirho, _path_chirho)| module_name_chirho.clone())
        .collect::<std::collections::HashSet<_>>();

    for exe_chirho in &package_chirho.executables_chirho {
        let src_dirs_chirho = if exe_chirho
            .build_info_chirho
            .hs_source_dirs_chirho
            .is_empty()
        {
            vec![".".to_string()]
        } else {
            exe_chirho.build_info_chirho.hs_source_dirs_chirho.clone()
        };

        // Compile other-modules BEFORE main-is so dependencies are available
        for mod_name_chirho in &exe_chirho.other_modules_chirho {
            if seen_chirho.insert(format!("{}:{}", exe_chirho.name_chirho, mod_name_chirho)) {
                if let Some(path_chirho) =
                    find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
                {
                    modules_chirho.push((mod_name_chirho.clone(), path_chirho));
                }
            }
        }

        // Now add main-is AFTER other-modules
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
    }

    for test_chirho in &package_chirho.test_suites_chirho {
        let src_dirs_chirho = if test_chirho
            .build_info_chirho
            .hs_source_dirs_chirho
            .is_empty()
        {
            vec![".".to_string()]
        } else {
            test_chirho.build_info_chirho.hs_source_dirs_chirho.clone()
        };

        for mod_name_chirho in &test_chirho.other_modules_chirho {
            if seen_chirho.insert(format!(
                "test:{}:{}",
                test_chirho.name_chirho, mod_name_chirho
            )) {
                if let Some(path_chirho) =
                    find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
                {
                    modules_chirho.push((mod_name_chirho.clone(), path_chirho));
                }
            }
        }

        if let Some(main_is_chirho) = &test_chirho.main_is_chirho {
            let main_path_chirho = src_dirs_chirho
                .iter()
                .map(|d_chirho| project_dir_chirho.join(d_chirho).join(main_is_chirho))
                .find(|p_chirho| p_chirho.exists());

            if let Some(path_chirho) = main_path_chirho {
                let mod_name_chirho = "Main".to_string();
                if seen_chirho.insert(format!(
                    "test:{}:{}",
                    test_chirho.name_chirho, mod_name_chirho
                )) {
                    modules_chirho.push((mod_name_chirho, path_chirho));
                }
            }
        }
    }

    if package_chirho.library_chirho.is_none()
        && package_chirho.internal_libraries_chirho.is_empty()
        && package_chirho.executables_chirho.is_empty()
        && package_chirho.test_suites_chirho.is_empty()
        && package_chirho.benchmarks_chirho.is_empty()
    {
        for setup_file_chirho in ["Setup.hs", "Setup.lhs"] {
            let setup_path_chirho = project_dir_chirho.join(setup_file_chirho);
            if setup_path_chirho.exists() && seen_chirho.insert("setup:Setup".to_string()) {
                modules_chirho.push(("Setup".to_string(), setup_path_chirho));
                break;
            }
        }
    }

    modules_chirho
}

fn discover_library_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    let mut modules_chirho: Vec<(String, PathBuf)> = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();

    for library_chirho in iter_package_libraries_chirho(package_chirho) {
        for (module_name_chirho, path_chirho) in
            discover_modules_for_library_chirho(library_chirho, project_dir_chirho)
        {
            if seen_chirho.insert(module_name_chirho.clone()) {
                modules_chirho.push((module_name_chirho, path_chirho));
            }
        }
    }

    modules_chirho
}

fn iter_package_libraries_chirho<'a>(
    package_chirho: &'a haskelujah_package_chirho::PackageDescChirho,
) -> impl Iterator<Item = &'a haskelujah_package_chirho::LibraryChirho> + 'a {
    package_chirho
        .library_chirho
        .iter()
        .chain(package_chirho.internal_libraries_chirho.iter())
}

fn is_self_or_internal_library_dep_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    dep_name_chirho: &str,
) -> bool {
    dep_name_chirho == package_chirho.name_chirho
        || package_chirho
            .internal_libraries_chirho
            .iter()
            .any(|library_chirho| library_chirho.name_chirho.as_deref() == Some(dep_name_chirho))
}

fn discover_modules_for_library_chirho(
    library_chirho: &haskelujah_package_chirho::LibraryChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    let src_dirs_chirho = if library_chirho
        .build_info_chirho
        .hs_source_dirs_chirho
        .is_empty()
    {
        vec![".".to_string()]
    } else {
        library_chirho
            .build_info_chirho
            .hs_source_dirs_chirho
            .clone()
    };

    let mut modules_chirho = Vec::new();
    for mod_name_chirho in library_chirho
        .exposed_modules_chirho
        .iter()
        .chain(library_chirho.other_modules_chirho.iter())
    {
        if let Some(path_chirho) =
            find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
        {
            modules_chirho.push((mod_name_chirho.clone(), path_chirho));
        }
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
    let module_rel_dir_chirho = module_name_chirho.replace('.', "/");

    for dir_chirho in src_dirs_chirho {
        for extension_chirho in ["hs", "lhs", "hsc"] {
            let candidate_path_chirho = project_dir_chirho
                .join(dir_chirho)
                .join(format!("{module_rel_dir_chirho}.{extension_chirho}"));
            if candidate_path_chirho.exists() {
                return Some(candidate_path_chirho);
            }
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

// ---------------------------------------------------------------------------
// Package installation (Hackage → compile → register)
// ---------------------------------------------------------------------------

/// Result of installing a package from Hackage.
#[derive(Debug)]
pub struct InstallResultChirho {
    /// The parsed package description.
    pub package_chirho: haskelujah_package_chirho::PackageDescChirho,
    /// The resolved build plan for dependencies.
    pub build_plan_chirho: haskelujah_package_chirho::BuildPlanChirho,
    /// Number of modules compiled.
    pub modules_compiled_chirho: usize,
    /// The package database entry that was registered.
    pub installed_pkg_chirho: haskelujah_package_chirho::InstalledPkgChirho,
}

/// Download a package from Hackage, compile it, and register it in the
/// package database.
///
/// This function:
/// 1. Downloads the package tarball from Hackage
/// 2. Extracts it to `install_dir_chirho`
/// 3. Parses the `.cabal` file
/// 4. Resolves dependencies against the provided index
/// 5. Discovers and compiles all modules in dependency order
/// 6. Registers the package in the provided package database
pub fn install_package_chirho(
    name_chirho: &str,
    version_chirho: &haskelujah_package_chirho::VersionChirho,
    install_dir_chirho: &Path,
    index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
    db_chirho: &mut haskelujah_package_chirho::InstalledPkgDbChirho,
) -> Result<InstallResultChirho, String> {
    use haskelujah_package_chirho::{
        hackage_chirho::{download_tarball_chirho, extract_tarball_chirho},
        resolve_deps_chirho,
    };
    use std::collections::BTreeSet;

    // Step 1: Download the tarball.
    let tarball_chirho =
        download_tarball_chirho(name_chirho, version_chirho).map_err(|e_chirho| {
            format!(
                "failed to download {}-{}: {}",
                name_chirho, version_chirho, e_chirho
            )
        })?;

    // Step 2: Extract.
    extract_tarball_chirho(&tarball_chirho, install_dir_chirho).map_err(|e_chirho| {
        format!(
            "failed to extract {}-{}: {}",
            name_chirho, version_chirho, e_chirho
        )
    })?;

    // Step 3: Find and parse .cabal file.
    let pkg_dir_chirho = install_dir_chirho.join(format!("{}-{}", name_chirho, version_chirho));
    let cabal_path_chirho = find_cabal_in_dir_chirho(&pkg_dir_chirho)
        .ok_or_else(|| format!("no .cabal file found in {}", pkg_dir_chirho.display()))?;

    let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho)
        .map_err(|e_chirho| format!("cannot read {}: {}", cabal_path_chirho.display(), e_chirho))?;
    let package_chirho = haskelujah_package_chirho::parse_cabal_chirho(&cabal_content_chirho);

    // Step 4: Resolve dependencies.
    let all_deps_chirho = collect_package_deps_chirho(&package_chirho);

    // Treat common GHC boot/platform packages as builtins (satisfied by
    // our synthetic module interfaces). This lets simple packages install
    // without pulling the entire boot library set.
    let mut builtins_chirho = BTreeSet::new();
    for builtin_chirho in &[
        "base",
        "ghc-prim",
        "ghc-bignum",
        "ghc-internal",
        "rts",
        "deepseq",
        "array",
        "bytestring",
        "containers",
        "text",
        "filepath",
        "directory",
        "process",
        "time",
        "transformers",
        "mtl",
        "parsec",
        "template-haskell",
        "pretty",
        "binary",
        "integer-gmp",
        "ghc-boot-th",
        "ghc-boot",
        "exceptions",
        "stm",
        "unix",
        "Win32",
        "hashable",
        "unordered-containers",
        "vector",
        "primitive",
        "tagged",
        "distributive",
        "comonad",
    ] {
        builtins_chirho.insert(builtin_chirho.to_string());
    }

    let build_plan_chirho =
        resolve_deps_chirho(&all_deps_chirho, index_chirho, &builtins_chirho)
            .map_err(|e_chirho| format!("dependency resolution failed: {}", e_chirho))?;

    // Step 5: Discover and compile modules.
    let source_files_chirho = discover_modules_chirho(&package_chirho, &pkg_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut sources_chirho: Vec<(String, String)> = Vec::new();

    for (module_name_chirho, path_chirho) in &source_files_chirho {
        match read_haskell_source_file_chirho(path_chirho) {
            Ok(content_chirho) => {
                sources_chirho.push((module_name_chirho.clone(), content_chirho));
            }
            Err(e_chirho) => {
                // Warn but continue — some listed modules may not exist.
                eprintln!(
                    "warning: cannot read module {} at {}: {}",
                    module_name_chirho,
                    path_chirho.display(),
                    e_chirho,
                );
            }
        }
    }

    let modules_compiled_chirho = sources_chirho.len();

    // Attempt compilation — if modules exist.
    if !sources_chirho.is_empty() {
        let source_refs_chirho: Vec<(&str, &str)> = sources_chirho
            .iter()
            .map(|(n_chirho, s_chirho)| (n_chirho.as_str(), s_chirho.as_str()))
            .collect();

        // Best-effort compilation: log errors but don't fail the install.
        match compile_modules_chirho(&source_refs_chirho, &mut source_map_chirho) {
            Ok(_results_chirho) => {}
            Err(_diag_chirho) => {
                eprintln!(
                    "warning: compilation of {}-{} produced errors (package still registered)",
                    name_chirho, version_chirho,
                );
            }
        }
    }

    // Step 6: Register in package database.
    let exposed_modules_chirho: Vec<haskelujah_package_chirho::InstalledModuleChirho> =
        if let Some(lib_chirho) = &package_chirho.library_chirho {
            lib_chirho
                .exposed_modules_chirho
                .iter()
                .map(
                    |m_chirho| haskelujah_package_chirho::InstalledModuleChirho {
                        module_name_chirho: m_chirho.clone(),
                        iface_path_chirho: PathBuf::from(format!(
                            "{}.rhi",
                            m_chirho.replace('.', "/")
                        )),
                        object_path_chirho: None,
                    },
                )
                .collect()
        } else {
            Vec::new()
        };

    let dep_tuples_chirho: Vec<(String, haskelujah_package_chirho::VersionChirho)> =
        all_deps_chirho
            .iter()
            .map(|d_chirho| (d_chirho.package_chirho.clone(), version_chirho.clone()))
            .collect();

    let installed_pkg_chirho = haskelujah_package_chirho::InstalledPkgChirho {
        name_chirho: name_chirho.to_string(),
        version_chirho: version_chirho.clone(),
        exposed_modules_chirho,
        depends_chirho: dep_tuples_chirho,
        install_dir_chirho: pkg_dir_chirho.clone(),
    };

    db_chirho.register_chirho(installed_pkg_chirho.clone());

    Ok(InstallResultChirho {
        package_chirho,
        build_plan_chirho,
        modules_compiled_chirho,
        installed_pkg_chirho,
    })
}

/// Find any `.cabal` file in a directory.
fn find_cabal_in_dir_chirho(dir_chirho: &Path) -> Option<PathBuf> {
    if let Ok(entries_chirho) = std::fs::read_dir(dir_chirho) {
        for entry_chirho in entries_chirho.flatten() {
            if entry_chirho
                .path()
                .extension()
                .is_some_and(|ext_chirho| ext_chirho == "cabal")
            {
                return Some(entry_chirho.path());
            }
        }
    }
    None
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
