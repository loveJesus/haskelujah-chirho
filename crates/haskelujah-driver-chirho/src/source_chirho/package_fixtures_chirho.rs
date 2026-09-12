// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Legacy package-fixture interface scanning, compiled only for unit tests.
//! These header fallbacks do not supply checked contracts to the file frontend.

use crate::{
    ModuleIfaceChirho, ParserChirho, SourceFileChirho, SourceMapChirho,
    build_iface_with_imports_chirho, lower_module_chirho, read_haskell_source_file_chirho,
};
use std::path::Path;

/// Recursively scan a package directory for .hs files and generate stub interfaces.
pub(crate) fn scan_package_hs_files_chirho(
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

/// Extract exported names from a module header and top-level type signatures.
fn extract_exported_names_from_source_chirho(source_chirho: &str) -> Vec<String> {
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
fn extract_module_name_from_source_chirho(source_chirho: &str) -> Option<String> {
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
