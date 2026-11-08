// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The explicit hs-boot promises selected by reached SOURCE import edges.
//! This stage checks module identity and the proven DATA-versus-synonym
//! contradiction only; it does not claim complete boot interface equivalence.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.md.

use crate::read_haskell_source_file_chirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::{SourceMapChirho, SpanChirho};
use haskelujah_syntax_chirho::SourceFileChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::GreenElementChirho;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(super) struct BootContractContextChirho<'source_chirho> {
    pub(super) paths_chirho: &'source_chirho BTreeMap<String, &'source_chirho Path>,
    pub(super) reached_chirho: &'source_chirho BTreeSet<String>,
}

fn error_chirho(message_chirho: String, span_chirho: SpanChirho) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    diagnostics_chirho.push_chirho(DiagnosticChirho::error_with_code_chirho(
        ErrorCodeChirho::error_chirho(209),
        message_chirho,
        span_chirho,
    ));
    diagnostics_chirho
}

fn read_boot_chirho(
    target_chirho: &str,
    path_chirho: &Path,
    span_chirho: SpanChirho,
) -> Result<ModuleChirho, DiagnosticBundleChirho> {
    let mut boot_name_chirho = path_chirho.as_os_str().to_os_string();
    boot_name_chirho.push("-boot");
    let boot_path_chirho = std::path::PathBuf::from(boot_name_chirho);
    let source_chirho =
        read_haskell_source_file_chirho(&boot_path_chirho).map_err(|failure_chirho| {
            error_chirho(
                format!(
                    "cannot load reached SOURCE contract {}: {failure_chirho}",
                    boot_path_chirho.display()
                ),
                span_chirho,
            )
        })?;
    let result_chirho = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            boot_path_chirho.as_path(),
            &source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();
        let tree_chirho = ParserChirho::new_chirho(&source_chirho, file_id_chirho).parse_chirho();
        let mut pending_chirho = vec![(tree_chirho.as_ref(), 0usize)];
        let mut budget_chirho = 65_536usize;
        while let Some((node_chirho, depth_chirho)) = pending_chirho.pop() {
            if depth_chirho > 128
                || budget_chirho == 0
                || node_chirho.kind_chirho() == SyntaxKindChirho::ErrorNodeChirho
            {
                return None;
            }
            budget_chirho -= 1;
            for element_chirho in node_chirho.children_chirho() {
                if let GreenElementChirho::NodeChirho(child_chirho) = element_chirho {
                    pending_chirho.push((child_chirho.as_ref(), depth_chirho + 1));
                }
            }
        }
        Some(lower_module_chirho(&tree_chirho, file_id_chirho))
    }));
    let Ok(Some(module_chirho)) = result_chirho else {
        return Err(error_chirho(
            format!(
                "cannot validate reached SOURCE contract {}: parse failed or exceeded the contract budget",
                boot_path_chirho.display()
            ),
            span_chirho,
        ));
    };
    if module_chirho.name_chirho.full_name_chirho() != target_chirho {
        return Err(error_chirho(
            format!(
                "reached SOURCE contract {} declares module {}, expected {target_chirho}",
                boot_path_chirho.display(),
                module_chirho.name_chirho.full_name_chirho()
            ),
            span_chirho,
        ));
    }
    Ok(module_chirho)
}

/// Only reached SOURCE edges select boots. A nearby unused boot has no effect.
/// No absent declaration is a contradiction: lowering/splices may be incomplete.
pub(super) fn validate_reached_boot_contracts_chirho(
    modules_chirho: &BTreeMap<String, &ModuleChirho>,
    context_chirho: BootContractContextChirho<'_>,
) -> Result<(), DiagnosticBundleChirho> {
    let mut validated_chirho = BTreeSet::new();
    for importer_chirho in context_chirho.reached_chirho {
        let Some(module_chirho) = modules_chirho.get(importer_chirho) else {
            continue;
        };
        for import_chirho in module_chirho.imports_chirho.iter().filter(|import_chirho| {
            import_chirho.source_chirho && import_chirho.selects_home_source_chirho()
        }) {
            let target_chirho = import_chirho.module_chirho.full_name_chirho();
            if !validated_chirho.insert(target_chirho.clone()) {
                continue;
            }
            let Some(actual_chirho) = modules_chirho.get(&target_chirho) else {
                return Err(error_chirho(
                    format!(
                        "cannot validate reached SOURCE import {target_chirho}: no authoritative source AST"
                    ),
                    import_chirho.span_chirho,
                ));
            };
            let Some(path_chirho) = context_chirho.paths_chirho.get(&target_chirho) else {
                return Err(error_chirho(
                    format!(
                        "cannot validate reached SOURCE import {target_chirho}: no authoritative source path"
                    ),
                    import_chirho.span_chirho,
                ));
            };
            let boot_chirho =
                read_boot_chirho(&target_chirho, path_chirho, import_chirho.span_chirho)?;
            let aliases_chirho = actual_chirho
                .decls_chirho
                .iter()
                .filter_map(|declaration_chirho| match declaration_chirho {
                    DeclChirho::TypeAliasDeclChirho { name_chirho, .. } => {
                        Some(name_chirho.text_chirho())
                    }
                    _ => None,
                })
                .collect::<BTreeSet<_>>();
            for declaration_chirho in &boot_chirho.decls_chirho {
                if let DeclChirho::DataDeclChirho {
                    name_chirho,
                    constructors_chirho,
                    ..
                } = declaration_chirho
                    && constructors_chirho.is_empty()
                    && aliases_chirho.contains(name_chirho.text_chirho())
                {
                    return Err(error_chirho(
                        format!(
                            "type constructor {}.{} conflicts with its reached hs-boot contract: the boot explicitly promises abstract data, but the module explicitly defines a type synonym (GHC-15843)",
                            target_chirho,
                            name_chirho.text_chirho()
                        ),
                        import_chirho.span_chirho,
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "boot_contract_tests_chirho.rs"]
mod tests_chirho;
