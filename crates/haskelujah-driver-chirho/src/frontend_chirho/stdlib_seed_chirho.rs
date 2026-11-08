// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Haskelujah stdlib frontend seed, moved from the driver root (row 691, L2): finding the
//! stdlib directory, collecting its module interfaces, imported types, synonyms and families
//! once per process, and merging that seed into each frontend run.

use super::*;

#[derive(Debug, Default, Clone)]
pub(crate) struct FrontendSeedArtifactsChirho {
    pub(crate) ifaces_chirho: Vec<ModuleIfaceChirho>,
    pub(crate) interface_sources_chirho:
        Option<std::sync::Arc<dependency_packages_chirho::DependencyInterfaceSourcesChirho>>,
    pub(crate) imported_types_chirho:
        std::collections::HashMap<String, haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    pub(crate) imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    pub(crate) imported_type_families_chirho: ImportedTypeFamiliesChirho,
}

static STDLIB_FRONTEND_ARTIFACTS_CHIRHO: OnceLock<Result<FrontendSeedArtifactsChirho, String>> =
    OnceLock::new();

pub(crate) fn merge_stdlib_frontend_artifacts_chirho(
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    imported_types_chirho: &mut std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    imported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    imported_type_families_chirho: &mut ImportedTypeFamiliesChirho,
) -> Result<(), String> {
    let stdlib_artifacts_chirho = STDLIB_FRONTEND_ARTIFACTS_CHIRHO
        .get_or_init(collect_stdlib_frontend_artifacts_uncached_chirho);

    match stdlib_artifacts_chirho {
        Ok(stdlib_artifacts_chirho) => {
            // Registry merge validates its complete replacement plan before
            // interfaces or schemes from this independent seed are installed.
            extend_imported_type_families_chirho(
                imported_type_families_chirho,
                stdlib_artifacts_chirho
                    .imported_type_families_chirho
                    .clone(),
            )?;
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
            Ok(())
        }
        Err(error_chirho) => Err(format!("cannot load stdlib frontend seed: {error_chirho}")),
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
    for iface_chirho in &mut artifacts_chirho.ifaces_chirho {
        iface_chirho.exposure_chirho = None;
    }
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
    if let Some(manifest_stdlib_dir_chirho) = manifest_stdlib_dir_chirho
        && stdlib_dir_is_valid_chirho(&manifest_stdlib_dir_chirho)
    {
        return Some(manifest_stdlib_dir_chirho);
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
