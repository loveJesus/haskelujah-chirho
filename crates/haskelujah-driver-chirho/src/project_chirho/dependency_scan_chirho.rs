// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-aware package scan seeds. Scan order records provenance, never a
//! package-version selection. Typed errors stay attached to their actual source
//! until an import reaches that candidate with selection evidence.

#[cfg(test)]
use crate::module_search_chirho::{
    extract_exported_names_from_source_chirho, extract_module_name_from_source_chirho,
};
#[cfg(test)]
use crate::read_haskell_source_file_chirho;
use haskelujah_ast_chirho::decl_chirho::ConstructorSignatureErrorChirho;
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
use haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho;
#[cfg(test)]
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
#[cfg(test)]
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::LineColChirho;
#[cfg(test)]
use haskelujah_span_chirho::SourceMapChirho;
#[cfg(test)]
use haskelujah_syntax_chirho::SourceFileChirho;
use std::collections::{BTreeMap, BTreeSet};
#[cfg(test)]
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
struct DependencyInterfaceFailureChirho {
    error_chirho: ConstructorSignatureErrorChirho,
    location_chirho: Option<LineColChirho>,
}

/// The original Result is retained, not flattened into a successful interface.
/// Panic recovery is separate candidate provenance, never a typed-error route.
type DependencyInterfaceOutcomeChirho = Result<usize, DependencyInterfaceFailureChirho>;

#[derive(Debug, Clone)]
struct DependencyInterfaceCandidateChirho {
    module_chirho: String,
    package_root_chirho: PathBuf,
    path_chirho: PathBuf,
    imports_chirho: BTreeSet<String>,
    scan_ordinal_chirho: usize,
    outcome_chirho: DependencyInterfaceOutcomeChirho,
    panic_recovery_chirho: bool,
}

/// Source selection is separate from scan order. Package qualifications are not
/// retained by ImportDecl yet, so absent caller evidence cannot select a version.
#[derive(Debug, Default, Clone)]
pub(crate) struct DependencyInterfaceSourcesChirho {
    candidates_chirho: Arc<Vec<DependencyInterfaceCandidateChirho>>,
    selected_paths_chirho: BTreeMap<String, PathBuf>,
    compiled_sources_chirho: BTreeMap<String, (String, BTreeSet<String>)>,
}

#[derive(Debug, Default)]
pub(crate) struct DependencyInterfaceScanChirho {
    ifaces_chirho: Vec<ModuleIfaceChirho>,
    sources_chirho: DependencyInterfaceSourcesChirho,
}

/// Successful interfaces and their retained source authority cross the frontend
/// seed boundary together. A Vec remains the explicit metadata-free legacy API.
pub(crate) struct FrontendInterfaceSeedsChirho {
    pub(super) ifaces_chirho: Vec<ModuleIfaceChirho>,
    pub(super) interface_sources_chirho: Option<Arc<DependencyInterfaceSourcesChirho>>,
}

impl From<Vec<ModuleIfaceChirho>> for FrontendInterfaceSeedsChirho {
    fn from(ifaces_chirho: Vec<ModuleIfaceChirho>) -> Self {
        Self {
            ifaces_chirho,
            interface_sources_chirho: None,
        }
    }
}

impl From<DependencyInterfaceScanChirho> for FrontendInterfaceSeedsChirho {
    fn from(scan_chirho: DependencyInterfaceScanChirho) -> Self {
        let admitted_chirho = scan_chirho
            .sources_chirho
            .candidates_chirho
            .iter()
            .filter_map(|candidate_chirho| {
                if scan_chirho
                    .sources_chirho
                    .selected_paths_chirho
                    .get(&candidate_chirho.module_chirho)
                    .is_some_and(|selected_chirho| selected_chirho != &candidate_chirho.path_chirho)
                {
                    return None;
                }
                match candidate_chirho.outcome_chirho {
                    Ok(interface_ordinal_chirho) => Some(interface_ordinal_chirho),
                    Err(_) => None,
                }
            })
            .collect::<BTreeSet<_>>();
        let ifaces_chirho = scan_chirho
            .ifaces_chirho
            .into_iter()
            .enumerate()
            .filter_map(|(ordinal_chirho, iface_chirho)| {
                admitted_chirho
                    .contains(&ordinal_chirho)
                    .then_some(iface_chirho)
            })
            .collect();
        Self {
            ifaces_chirho,
            interface_sources_chirho: Some(Arc::new(scan_chirho.sources_chirho)),
        }
    }
}

impl DependencyInterfaceScanChirho {
    #[cfg(test)]
    pub(crate) fn validate_sources_chirho(
        &self,
        sources_chirho: &[(String, String, String)],
    ) -> Result<(), DiagnosticBundleChirho> {
        self.sources_chirho.validate_sources_chirho(sources_chirho)
    }

    /// Interface inspection does not consume or discard failed candidates.
    #[cfg(test)]
    pub(crate) fn ifaces_chirho(&self) -> impl Iterator<Item = &ModuleIfaceChirho> {
        self.sources_chirho
            .candidates_chirho
            .iter()
            .filter_map(|candidate_chirho| {
                if self
                    .sources_chirho
                    .selected_paths_chirho
                    .get(&candidate_chirho.module_chirho)
                    .is_some_and(|selected_chirho| selected_chirho != &candidate_chirho.path_chirho)
                {
                    return None;
                }
                match &candidate_chirho.outcome_chirho {
                    Ok(interface_ordinal_chirho) => {
                        self.ifaces_chirho.get(*interface_ordinal_chirho)
                    }
                    Err(_) => None,
                }
            })
    }

    /// Only actual package discovery may supply this selection; neither an
    /// ordinal nor a module's spelling is evidence for a source path.
    #[cfg(test)]
    pub(crate) fn select_source_chirho(&mut self, module_chirho: String, path_chirho: PathBuf) {
        self.sources_chirho
            .selected_paths_chirho
            .insert(module_chirho, path_chirho);
    }
}

const INTERFACE_IMPORT_WORK_CHIRHO: usize = 100_000;

impl DependencyInterfaceSourcesChirho {
    pub(super) fn including_compiled_sources_chirho(
        &self,
        sources_chirho: &[(String, String, String)],
    ) -> Self {
        let mut retained_chirho = self.clone();
        for (name_chirho, path_chirho, source_chirho) in sources_chirho {
            retained_chirho.compiled_sources_chirho.insert(
                name_chirho.clone(),
                (
                    path_chirho.clone(),
                    super::extract_imports_chirho(source_chirho)
                        .into_iter()
                        .collect(),
                ),
            );
        }
        retained_chirho
    }

    pub(crate) fn validate_sources_chirho(
        &self,
        sources_chirho: &[(String, String, String)],
    ) -> Result<(), DiagnosticBundleChirho> {
        let mut local_chirho = self
            .compiled_sources_chirho
            .iter()
            .map(|(name_chirho, (_, imports_chirho))| (name_chirho.clone(), imports_chirho.clone()))
            .collect::<BTreeMap<_, _>>();
        for (name_chirho, _, source_chirho) in sources_chirho {
            local_chirho.insert(
                name_chirho.clone(),
                super::extract_imports_chirho(source_chirho)
                    .into_iter()
                    .collect(),
            );
        }
        let mut work_chirho = INTERFACE_IMPORT_WORK_CHIRHO;
        let mut by_module_chirho: BTreeMap<&str, Vec<&DependencyInterfaceCandidateChirho>> =
            BTreeMap::new();
        for candidate_chirho in self.candidates_chirho.iter() {
            work_chirho = work_chirho.checked_sub(1).ok_or_else(|| {
                selection_error_chirho(format!(
                    "package interface import work bound ({INTERFACE_IMPORT_WORK_CHIRHO}) reached"
                ))
            })?;
            by_module_chirho
                .entry(&candidate_chirho.module_chirho)
                .or_default()
                .push(candidate_chirho);
        }
        let mut pending_chirho = sources_chirho
            .iter()
            .map(|(name_chirho, _, _)| (name_chirho.clone(), None::<String>))
            .collect::<Vec<_>>();
        let mut visited_chirho = BTreeSet::new();
        while let Some((name_chirho, uncertainty_chirho)) = pending_chirho.pop() {
            work_chirho = work_chirho.checked_sub(1).ok_or_else(|| {
                selection_error_chirho(format!(
                    "package interface import work bound ({INTERFACE_IMPORT_WORK_CHIRHO}) reached"
                ))
            })?;
            if !visited_chirho.insert((name_chirho.clone(), uncertainty_chirho.clone())) {
                continue;
            }
            if let Some(imports_chirho) = local_chirho.get(&name_chirho) {
                pending_chirho.extend(
                    imports_chirho
                        .iter()
                        .cloned()
                        .map(|name_chirho| (name_chirho, uncertainty_chirho.clone())),
                );
                continue;
            }
            let Some(candidates_chirho) = by_module_chirho.get(name_chirho.as_str()) else {
                continue;
            };
            let admitted_chirho = if let Some(selected_chirho) =
                self.selected_paths_chirho.get(&name_chirho)
            {
                let selected_candidates_chirho = candidates_chirho
                    .iter()
                    .copied()
                    .filter(|candidate_chirho| &candidate_chirho.path_chirho == selected_chirho)
                    .collect::<Vec<_>>();
                if selected_candidates_chirho.len() != 1 {
                    return Err(selection_error_chirho(format!(
                        "package interface selection for {name_chirho} does not identify one scanned source: {}",
                        selected_chirho.display()
                    )));
                }
                selected_candidates_chirho
            } else {
                candidates_chirho.clone()
            };
            let uncertain_chirho = uncertainty_chirho
                .or_else(|| (admitted_chirho.len() != 1).then(|| name_chirho.clone()));
            for candidate_chirho in admitted_chirho {
                work_chirho = work_chirho.checked_sub(candidate_chirho.imports_chirho.len()).and_then(|remaining_chirho| remaining_chirho.checked_sub(1)).ok_or_else(|| selection_error_chirho(
                    format!("package interface import work bound ({INTERFACE_IMPORT_WORK_CHIRHO}) reached")))?;
                if let Err(failure_chirho) = candidate_chirho.outcome_chirho {
                    let DependencyInterfaceFailureChirho {
                        error_chirho,
                        location_chirho,
                    } = failure_chirho;
                    if let Some(uncertain_name_chirho) = &uncertain_chirho {
                        return Err(selection_error_chirho(format!(
                            "package interface selection is unproved for {uncertain_name_chirho}; an import can reach malformed source {} (package {}, scan ordinal {}). Select the source path before reporting a constructor error",
                            candidate_chirho.path_chirho.display(),
                            candidate_chirho.package_root_chirho.display(),
                            candidate_chirho.scan_ordinal_chirho
                        )));
                    }
                    let mut diagnostic_chirho =
                        crate::family_exports_chirho::constructor_signature_diagnostic_chirho(
                            error_chirho,
                            &name_chirho,
                        );
                    diagnostic_chirho.labels_chirho.clear();
                    let location_chirho = location_chirho
                        .map(|location_chirho| {
                            format!(
                                ":{}:{}",
                                location_chirho.line_chirho, location_chirho.col_chirho
                            )
                        })
                        .unwrap_or_default();
                    diagnostic_chirho.notes_chirho.push(format!(
                        "source: {}{location_chirho}; signature byte range {}..{}; package {}",
                        candidate_chirho.path_chirho.display(),
                        error_chirho.span_chirho().start_chirho().as_u32_chirho(),
                        error_chirho.span_chirho().end_chirho().as_u32_chirho(),
                        candidate_chirho.package_root_chirho.display()
                    ));
                    return Err(DiagnosticBundleChirho::new_chirho(vec![diagnostic_chirho]));
                }
                let edge_uncertainty_chirho = uncertain_chirho.clone().or_else(|| {
                    candidate_chirho
                        .panic_recovery_chirho
                        .then(|| name_chirho.clone())
                });
                pending_chirho.extend(
                    candidate_chirho
                        .imports_chirho
                        .iter()
                        .cloned()
                        .map(|name_chirho| (name_chirho, edge_uncertainty_chirho.clone())),
                );
            }
        }
        Ok(())
    }
}

fn selection_error_chirho(message_chirho: String) -> DiagnosticBundleChirho {
    let mut diagnostic_chirho = DiagnosticChirho::error_chirho(
        message_chirho,
        haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
    );
    diagnostic_chirho.labels_chirho.clear();
    DiagnosticBundleChirho::new_chirho(vec![diagnostic_chirho])
}

impl super::FrontendSeedArtifactsChirho {
    /// Direct frontend follow-ups retain and check the same scanned-source seed.
    #[cfg(test)]
    pub(crate) fn validate_scanned_source_chirho(
        &self,
        name_chirho: &str,
        path_chirho: &str,
        source_chirho: &str,
    ) -> Result<(), DiagnosticBundleChirho> {
        if let Some(sources_chirho) = &self.interface_sources_chirho {
            sources_chirho.validate_sources_chirho(&[(
                name_chirho.to_owned(),
                path_chirho.to_owned(),
                source_chirho.to_owned(),
            )])?;
        }
        Ok(())
    }
}

/// Scan downloaded dependency packages in .haskelujah-packages-chirho/ for stub
/// module interfaces. This enables cross-package module resolution.
#[cfg(test)]
pub(crate) fn scan_dependency_package_ifaces_chirho(
    project_dir_chirho: &Path,
) -> DependencyInterfaceScanChirho {
    let mut scan_chirho = DependencyInterfaceScanChirho::default();
    let current_package_dir_chirho = std::fs::canonicalize(project_dir_chirho)
        .unwrap_or_else(|_| project_dir_chirho.to_path_buf());

    // Look for .haskelujah-packages-chirho/ relative to project dir ancestors
    let Some(packages_dir_chirho) = super::find_dependency_packages_dir_chirho(project_dir_chirho)
    else {
        return scan_chirho;
    };

    // Seed the current package first when the caller itself lives under
    // .haskelujah-packages-chirho/. This gives tests and package-scan callers
    // forward module stubs for same-package imports such as containers'
    // Utils.Containers.Internal.BitUtil.
    scan_package_hs_files_chirho(
        project_dir_chirho,
        &current_package_dir_chirho,
        &mut scan_chirho,
    );

    // Scan each package directory
    let entries_chirho = match std::fs::read_dir(&packages_dir_chirho) {
        Ok(e_chirho) => e_chirho,
        Err(_) => return scan_chirho,
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
        scan_package_hs_files_chirho(&pkg_dir_chirho, &canonical_pkg_dir_chirho, &mut scan_chirho);
    }

    scan_chirho
}

/// Recursively scan a package directory for .hs files and generate stub interfaces.
#[cfg(test)]
fn scan_package_hs_files_chirho(
    dir_chirho: &Path,
    package_root_chirho: &Path,
    scan_chirho: &mut DependencyInterfaceScanChirho,
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
            scan_package_hs_files_chirho(&path_chirho, package_root_chirho, scan_chirho);
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
                                    declaration_chirho: None,
                                    is_class_chirho: false,
                                    name_chirho: name_chirho.clone(),
                                    constructors_chirho: vec![],
                                    methods_chirho: vec![],
                                    span_chirho,
                                },
                            );
                            exports_chirho.values_chirho.insert(
                                name_chirho.clone(),
                                IfaceValueChirho {
                            availability_chirho: haskelujah_naming_chirho::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                                    name_chirho: name_chirho.clone(),
                                    span_chirho,
                                    defining_module_chirho: None,
                                },
                            );
                        } else {
                            exports_chirho.values_chirho.insert(
                                name_chirho.clone(),
                                IfaceValueChirho {
                            availability_chirho: haskelujah_naming_chirho::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                                    name_chirho: name_chirho.clone(),
                                    span_chirho,
                                    defining_module_chirho: None,
                                },
                            );
                        }
                    }
                    stub_iface_chirho = Some(ModuleIfaceChirho {
                        exposure_chirho: None,
                        name_chirho: mod_name_chirho,
                        exports_chirho,
                    });
                }

                let mut source_map_chirho = SourceMapChirho::new_chirho();
                let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                    &mut source_map_chirho,
                    &path_chirho,
                    &preprocessed_source_chirho,
                );
                let file_id_chirho = source_file_chirho.file_id_chirho();
                let imports_chirho = super::extract_imports_chirho(&preprocessed_source_chirho)
                    .into_iter()
                    .collect::<BTreeSet<_>>();
                let real_iface_result_chirho =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let parser_chirho =
                            ParserChirho::new_chirho(&preprocessed_source_chirho, file_id_chirho);
                        let green_chirho = parser_chirho.parse_chirho();
                        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
                        let result_chirho =
                            haskelujah_naming_chirho::build_iface_with_imports_chirho(
                                &module_chirho,
                                &scan_chirho.ifaces_chirho,
                            );
                        (module_chirho.name_chirho.full_name_chirho(), result_chirho)
                    }));
                let (module_chirho, outcome_chirho, panic_recovery_chirho) =
                    match real_iface_result_chirho {
                        Ok((name_chirho, Ok(iface_chirho))) => {
                            let interface_ordinal_chirho = scan_chirho.ifaces_chirho.len();
                            scan_chirho.ifaces_chirho.push(iface_chirho);
                            (name_chirho, Ok(interface_ordinal_chirho), false)
                        }
                        Ok((name_chirho, Err(error_chirho))) => {
                            let location_chirho = source_map_chirho
                                .line_index_chirho(error_chirho.span_chirho().file_id_chirho())
                                .map(|index_chirho| {
                                    index_chirho
                                        .line_col_chirho(error_chirho.span_chirho().start_chirho())
                                });
                            (
                                name_chirho,
                                Err(DependencyInterfaceFailureChirho {
                                    error_chirho,
                                    location_chirho,
                                }),
                                false,
                            )
                        }
                        Err(_) => {
                            let Some(iface_chirho) = stub_iface_chirho else {
                                continue;
                            };
                            let name_chirho = iface_chirho.name_chirho.clone();
                            let interface_ordinal_chirho = scan_chirho.ifaces_chirho.len();
                            scan_chirho.ifaces_chirho.push(iface_chirho);
                            (name_chirho, Ok(interface_ordinal_chirho), true)
                        }
                    };
                let scan_ordinal_chirho = scan_chirho.sources_chirho.candidates_chirho.len();
                Arc::make_mut(&mut scan_chirho.sources_chirho.candidates_chirho).push(
                    DependencyInterfaceCandidateChirho {
                        module_chirho,
                        package_root_chirho: package_root_chirho.to_path_buf(),
                        path_chirho: std::fs::canonicalize(&path_chirho).unwrap_or(path_chirho),
                        imports_chirho,
                        scan_ordinal_chirho,
                        outcome_chirho,
                        panic_recovery_chirho,
                    },
                );
            }
        }
    }
}

#[cfg(test)]
#[path = "dependency_scan_tests_chirho.rs"]
mod tests_chirho;
