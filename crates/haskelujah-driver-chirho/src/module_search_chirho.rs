// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded, authority-aware source-module discovery.
//!
//! Files found beneath a search root are only candidates. A hierarchical
//! candidate becomes authoritative when its declared module name matches its
//! path relative to that root. Interface names are unique at this boundary so
//! downstream consumers never disagree merely because one selects the first
//! match while another selects the last.

use std::path::Path;

use haskelujah_naming_chirho::iface_chirho::{
    IfaceExportsChirho, IfaceValueChirho, ModuleIfaceChirho, build_iface_with_imports_chirho,
};
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::{SourceMapChirho, SpanChirho};
use haskelujah_syntax_chirho::SourceFileChirho;

use crate::read_haskell_source_file_chirho;

/// Admit one authoritative owner per module name.
///
/// A source-root candidate replaces a seeded fallback with the same name. This
/// preserves legitimate local modules such as a root-level `Prelude.hs`, while
/// keeping downstream first-match and last-match consumers in agreement.
fn admit_authoritative_iface_chirho(
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    iface_chirho: ModuleIfaceChirho,
) {
    if let Some(existing_chirho) = ifaces_chirho
        .iter()
        .position(|existing_chirho| existing_chirho.name_chirho == iface_chirho.name_chirho)
    {
        ifaces_chirho[existing_chirho] = iface_chirho;
    } else {
        ifaces_chirho.push(iface_chirho);
    }
}

fn sibling_path_matches_declared_module_chirho(
    path_chirho: &Path,
    declared_module_name_chirho: &str,
) -> bool {
    let Some(file_stem_chirho) = path_chirho
        .file_stem()
        .map(|file_stem_chirho| file_stem_chirho.to_string_lossy())
    else {
        return false;
    };
    declared_module_name_chirho
        .rsplit('.')
        .next()
        .is_some_and(|last_component_chirho| last_component_chirho == file_stem_chirho)
}

/// Scan same-directory peers whose file stem matches their declared module's
/// final component. The caller selected the directory, while this final path
/// check keeps unrelated test files in that directory from becoming imports.
///
/// Workflow: `spec-chirho/workflows-chirho/compiler-pipeline-chirho/
/// module-search-authority-chirho.md`.
pub(crate) fn scan_sibling_module_ifaces_chirho(
    search_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    skip_file_chirho: &str,
) {
    if !search_dir_chirho.is_dir() {
        return;
    }
    let Ok(entries_chirho) = std::fs::read_dir(search_dir_chirho) else {
        return;
    };
    for entry_chirho in entries_chirho.flatten() {
        let path_chirho = entry_chirho.path();
        if path_chirho
            .extension()
            .is_none_or(|extension_chirho| extension_chirho != "hs")
        {
            continue;
        }
        if path_chirho
            .file_name()
            .map(|name_chirho| name_chirho.to_string_lossy().to_string())
            == Some(skip_file_chirho.to_string())
        {
            continue;
        }
        let Ok(sibling_source_chirho) = read_haskell_source_file_chirho(&path_chirho) else {
            continue;
        };
        let sibling_name_chirho = path_chirho
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let sibling_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            &sibling_name_chirho,
            &sibling_source_chirho,
        );
        let sibling_fid_chirho = sibling_file_chirho.file_id_chirho();
        let iface_result_chirho = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let parser_chirho =
                ParserChirho::new_chirho(&sibling_source_chirho, sibling_fid_chirho);
            let green_chirho = parser_chirho.parse_chirho();
            let sibling_module_chirho = lower_module_chirho(&green_chirho, sibling_fid_chirho);
            build_iface_with_imports_chirho(&sibling_module_chirho, ifaces_chirho)
        }));
        if let Ok(iface_chirho) = iface_result_chirho
            && sibling_path_matches_declared_module_chirho(&path_chirho, &iface_chirho.name_chirho)
        {
            admit_authoritative_iface_chirho(ifaces_chirho, iface_chirho);
        }
    }
}

/// How deep the hierarchical module search descends below its root.
///
/// A hierarchical module name contributes one directory per dotted component,
/// so this bounds a module at 64 components — unreachable for real code, while
/// still terminating a pathological tree. Measured 2026-08-02: the deepest
/// directory nesting anywhere in this repo is 10 — a 6.4x margin.
pub(crate) const MAX_MODULE_SEARCH_DEPTH_CHIRHO: usize = 64;

/// How many directories the hierarchical module search will enter.
///
/// This is the bound that actually matters. Measured 2026-08-02: `/private/tmp`
/// holds 80418 directories but only 4274 `.hs` files, so a file-only budget
/// never fires there and the walk is dominated by `read_dir`.
///
/// Margin, measured the same day (a search root is the checked file's PARENT,
/// so for a package build it is one package directory, never the vendor root):
/// the largest in-repo search root is `ghc-lib-parser-9.14.1.20251220` at 170
/// directories — a 12x margin. The 4287 directories under
/// `.haskelujah-packages-chirho` are the SUM across 410 packages and are never
/// walked from a single root.
pub(crate) const MAX_MODULE_SEARCH_DIRS_CHIRHO: usize = 2048;

/// How many `.hs` files the hierarchical module search will parse.
///
/// Exists so a shallow-but-file-dense tree stays bounded too. Margin: the
/// largest in-repo search root by file count is the 938-file GHC corpus
/// directory — a 17x margin.
pub(crate) const MAX_MODULE_SEARCH_FILES_CHIRHO: usize = 16384;

/// Bounds carried through one hierarchical module search.
///
/// `haskelujah check <file>` searches the checked file's PARENT directory,
/// which for a file outside a project is an arbitrary directory such as `/tmp`
/// or `$HOME`. Without these bounds the walk is unbounded in depth, breadth and
/// file count: measured 2026-08-02, a file placed directly in `/private/tmp`
/// did not finish within 60 seconds. That is disqualifying for drop-in use,
/// since a real user's file is never inside our repo.
pub(crate) struct ModuleSearchBoundsChirho {
    /// How many directories have been entered so far.
    pub(crate) dirs_entered_chirho: usize,
    /// How many `.hs` files have been parsed so far.
    pub(crate) files_read_chirho: usize,
    /// Whether the truncation notice has already been emitted.
    pub(crate) warned_chirho: bool,
}

impl ModuleSearchBoundsChirho {
    pub(crate) fn new_chirho() -> Self {
        Self {
            dirs_entered_chirho: 0,
            files_read_chirho: 0,
            warned_chirho: false,
        }
    }

    /// True while the directory budget remains.
    pub(crate) fn may_enter_dir_chirho(&mut self) -> bool {
        if self.dirs_entered_chirho >= MAX_MODULE_SEARCH_DIRS_CHIRHO {
            self.report_truncation_chirho("directories");
            return false;
        }
        self.dirs_entered_chirho += 1;
        true
    }

    /// True while the parse budget remains.
    pub(crate) fn may_read_file_chirho(&mut self) -> bool {
        if self.files_read_chirho >= MAX_MODULE_SEARCH_FILES_CHIRHO {
            self.report_truncation_chirho("source files");
            return false;
        }
        self.files_read_chirho += 1;
        true
    }

    /// Report exhaustion once. Never silent: a truncated search can leave
    /// imports unresolvable, and a silent cap reads as "searched everything".
    fn report_truncation_chirho(&mut self, what_chirho: &str) {
        if self.warned_chirho {
            return;
        }
        self.warned_chirho = true;
        eprintln!(
            "warning: module search stopped after scanning {} {what_chirho}; some imports \
             may be unresolvable — check the file from inside its own project directory",
            if what_chirho == "directories" {
                MAX_MODULE_SEARCH_DIRS_CHIRHO
            } else {
                MAX_MODULE_SEARCH_FILES_CHIRHO
            }
        );
    }
}

/// Recursively scan a source root for hierarchical module peers.
///
/// Workflow: `spec-chirho/workflows-chirho/compiler-pipeline-chirho/
/// module-search-authority-chirho.md`.
pub(crate) fn scan_hierarchical_modules_chirho(
    dir_chirho: &Path,
    root_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    skip_file_chirho: &str,
) {
    let mut bounds_chirho = ModuleSearchBoundsChirho::new_chirho();
    scan_hierarchical_modules_bounded_chirho(
        dir_chirho,
        root_dir_chirho,
        source_map_chirho,
        ifaces_chirho,
        skip_file_chirho,
        0,
        &mut bounds_chirho,
    );
}

fn scan_hierarchical_modules_bounded_chirho(
    dir_chirho: &Path,
    root_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    skip_file_chirho: &str,
    depth_chirho: usize,
    bounds_chirho: &mut ModuleSearchBoundsChirho,
) {
    let entries_chirho = match std::fs::read_dir(dir_chirho) {
        Ok(entries_chirho) => entries_chirho,
        Err(_) => return,
    };
    for entry_chirho in entries_chirho.flatten() {
        let path_chirho = entry_chirho.path();
        // Never follow a symlink. This is what makes a cycle impossible —
        // `sub/up -> ..` is simply not descended into — and it costs nothing,
        // because `read_dir` already knows the entry type. Verified 2026-08-02:
        // there is not one symlinked DIRECTORY in this repo, in either GHC
        // corpus, or under the vendored Hackage packages, so nothing legitimate
        // is lost. (Before this, `is_dir()` followed symlinks and termination
        // depended on the OS `ELOOP` limit.)
        let is_symlink_chirho = entry_chirho
            .file_type()
            .map(|file_type_chirho| file_type_chirho.is_symlink())
            .unwrap_or(true);
        if is_symlink_chirho {
            continue;
        }
        if path_chirho.is_dir() {
            if depth_chirho >= MAX_MODULE_SEARCH_DEPTH_CHIRHO
                || !bounds_chirho.may_enter_dir_chirho()
            {
                continue;
            }
            scan_hierarchical_modules_bounded_chirho(
                &path_chirho,
                root_dir_chirho,
                source_map_chirho,
                ifaces_chirho,
                skip_file_chirho,
                depth_chirho + 1,
                bounds_chirho,
            );
            continue;
        }
        if path_chirho
            .extension()
            .is_none_or(|extension_chirho| extension_chirho != "hs")
        {
            continue;
        }

        let file_name_chirho = path_chirho
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if file_name_chirho == skip_file_chirho
            || file_name_chirho == "Setup.hs"
            || file_name_chirho == "Setup.lhs"
        {
            continue;
        }

        let relative_path_chirho = path_chirho
            .strip_prefix(root_dir_chirho)
            .unwrap_or(&path_chirho);
        let expected_module_name_chirho = relative_path_chirho
            .with_extension("")
            .to_string_lossy()
            .replace(['/', '\\'], ".");
        if ifaces_chirho
            .iter()
            .any(|iface_chirho| iface_chirho.name_chirho == expected_module_name_chirho)
        {
            continue;
        }

        // Parsing is the real cost of this search, so the budget is spent here
        // rather than per directory entry.
        if !bounds_chirho.may_read_file_chirho() {
            continue;
        }
        let Ok(source_chirho) = read_haskell_source_file_chirho(&path_chirho) else {
            continue;
        };
        let sibling_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            &file_name_chirho,
            &source_chirho,
        );
        let file_id_chirho = sibling_file_chirho.file_id_chirho();
        let iface_result_chirho = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let parser_chirho = ParserChirho::new_chirho(&source_chirho, file_id_chirho);
            let green_chirho = parser_chirho.parse_chirho();
            let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
            if module_chirho.name_chirho.full_name_chirho() != expected_module_name_chirho {
                return None;
            }
            Some(build_iface_with_imports_chirho(
                &module_chirho,
                ifaces_chirho,
            ))
        }));
        match iface_result_chirho {
            Ok(Some(iface_chirho)) => {
                admit_authoritative_iface_chirho(ifaces_chirho, iface_chirho);
            }
            Ok(None) => {}
            Err(_) => {
                // A recovery stub is subject to the same authority rule as a
                // fully parsed interface. A header found under an unrelated
                // descendant directory is not a module from this source root.
                let Some(module_name_chirho) =
                    extract_module_name_from_source_chirho(&source_chirho)
                else {
                    continue;
                };
                if module_name_chirho != expected_module_name_chirho {
                    continue;
                }
                let mut exports_chirho = IfaceExportsChirho::default();
                for name_chirho in extract_exported_names_from_source_chirho(&source_chirho) {
                    exports_chirho.values_chirho.insert(
                        name_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    );
                }
                admit_authoritative_iface_chirho(
                    ifaces_chirho,
                    ModuleIfaceChirho {
                        name_chirho: module_name_chirho,
                        exports_chirho,
                    },
                );
            }
        }
    }
}

/// Extract exported names from a module header and top-level type signatures.
pub(crate) fn extract_exported_names_from_source_chirho(source_chirho: &str) -> Vec<String> {
    let mut names_chirho = Vec::new();
    let mut in_export_list_chirho = false;
    let mut past_where_chirho = false;

    for line_chirho in source_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();
        if trimmed_chirho.starts_with("--") {
            continue;
        }
        if trimmed_chirho.starts_with("module ") && trimmed_chirho.contains('(') {
            in_export_list_chirho = true;
        }
        if in_export_list_chirho {
            for part_chirho in trimmed_chirho.split(',') {
                let clean_chirho = part_chirho
                    .trim()
                    .trim_start_matches("module ")
                    .trim_start_matches("type ")
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .trim();
                if let Some(name_chirho) = clean_chirho
                    .split(|character_chirho: char| {
                        character_chirho == '(' || character_chirho.is_whitespace()
                    })
                    .next()
                    && !name_chirho.is_empty()
                    && name_chirho != "where"
                    && name_chirho.chars().next().is_some_and(|character_chirho| {
                        character_chirho.is_alphanumeric() || character_chirho == '_'
                    })
                {
                    names_chirho.push(name_chirho.to_string());
                }
            }
            if trimmed_chirho.contains("where") && !trimmed_chirho.starts_with("module") {
                in_export_list_chirho = false;
                past_where_chirho = true;
            }
            continue;
        }
        if trimmed_chirho == "where" || trimmed_chirho.ends_with(") where") {
            past_where_chirho = true;
            continue;
        }
        if past_where_chirho
            && !line_chirho.starts_with(' ')
            && !line_chirho.starts_with('\t')
            && trimmed_chirho.contains(" :: ")
            && let Some(name_chirho) = trimmed_chirho.split(" :: ").next()
        {
            let name_chirho = name_chirho.trim();
            if !name_chirho.is_empty() && !name_chirho.contains(' ') {
                names_chirho.push(name_chirho.to_string());
            }
        }
    }
    names_chirho.sort();
    names_chirho.dedup();
    names_chirho
}

/// Extract a module name from a `module Foo.Bar.Baz` header.
pub(crate) fn extract_module_name_from_source_chirho(source_chirho: &str) -> Option<String> {
    for line_chirho in source_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();
        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("module ") {
            let rest_chirho = rest_chirho.trim();
            let name_chirho = rest_chirho
                .split(|character_chirho: char| {
                    character_chirho.is_whitespace() || character_chirho == '('
                })
                .next()?;
            if !name_chirho.is_empty() {
                return Some(name_chirho.to_string());
            }
        }
    }
    None
}
