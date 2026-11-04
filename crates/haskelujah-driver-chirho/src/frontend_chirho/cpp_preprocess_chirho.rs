// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! C preprocessing for Haskell sources, moved from the driver root (row 691, L1): deciding
//! when a source needs cpp, the GHC and package version macros and include directories it
//! gets, running it with pragma closers protected, and stripping leftover directives.

use super::*;

pub(crate) const CPP_GLASGOW_HASKELL_VERSION_CHIRHO: &str = "810";
pub(crate) const CPP_GHC_VERSION_MAJOR_CHIRHO: u32 = 8;
pub(crate) const CPP_GHC_VERSION_MINOR_CHIRHO: u32 = 10;
pub(crate) const CPP_GHC_VERSION_PATCH_CHIRHO: u32 = 0;

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

pub(crate) fn discover_cpp_min_version_macro_options_chirho(path_chirho: &Path) -> Vec<String> {
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

pub(crate) fn cpp_include_dirs_chirho(path_chirho: &Path) -> Vec<PathBuf> {
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

pub(crate) fn source_uses_cpp_chirho(source_chirho: &str) -> bool {
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

pub(crate) fn decode_cpp_stdout_chirho(
    path_chirho: &Path,
    stdout_chirho: Vec<u8>,
) -> io::Result<String> {
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

pub(crate) fn preprocess_cpp_source_with_options_chirho(
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

pub(crate) fn preprocess_cpp_source_chirho(
    path_chirho: &Path,
    source_chirho: &str,
) -> io::Result<String> {
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

pub(crate) fn has_cpp_directive_residue_chirho(source_chirho: &str) -> bool {
    source_chirho
        .lines()
        .any(|line_chirho| cpp_directive_head_for_line_chirho(line_chirho).is_some())
}

pub(crate) fn strip_cpp_directives_chirho(source_chirho: &str) -> String {
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
