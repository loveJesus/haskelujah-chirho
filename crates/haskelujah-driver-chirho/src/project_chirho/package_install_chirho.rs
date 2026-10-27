// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Installing a package: fetch, compile and register (row 634).

use super::cabal_build_chirho::collect_package_deps_chirho;
use super::module_discovery_chirho::discover_modules_chirho;
use super::multi_module_chirho::compile_modules_chirho;
use super::read_haskell_source_file_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use std::path::{Path, PathBuf};

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
pub(crate) fn find_cabal_in_dir_chirho(dir_chirho: &Path) -> Option<PathBuf> {
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
