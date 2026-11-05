// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declared types and signature headers retained by authoritative discovery.
//! The cold pass checks headers with the common engine; it visits no value body.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.md.

use super::boot_contract_chirho::{
    BootContractContextChirho, validate_reached_boot_contracts_chirho,
};
use super::{
    DiscoveredSourceModuleChirho, DiscoveredSourceModulesChirho, admit_authoritative_iface_chirho,
};
use crate::ImportedTypeSynonymsChirho;
use crate::family_exports_chirho::{
    FamilyExportsErrorChirho, build_family_aware_iface_chirho, build_type_family_scope_chirho,
    constructor_signature_diagnostic_chirho, family_frontend_diagnostics_chirho,
    publish_type_family_exports_chirho,
};
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_diagnostics_chirho::DiagnosticBundleChirho;
use haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho;
use haskelujah_typing_chirho::infer_chirho::{
    SourceFamilyRegistrationContextChirho, TypeFamilyAliasResolutionChirho,
    TypeFamilyRegistryChirho, TypeFamilyScopeResolutionChirho,
    predeclare_source_open_families_chirho, register_source_module_families_chirho,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

pub(crate) struct DiscoveredFamilyContextChirho<'source_chirho> {
    pub(crate) modules_chirho: &'source_chirho DiscoveredSourceModulesChirho,
    pub(crate) ifaces_chirho: &'source_chirho mut Vec<ModuleIfaceChirho>,
    pub(crate) registry_chirho: &'source_chirho mut TypeFamilyRegistryChirho,
    pub(crate) imported_types_chirho: Option<
        &'source_chirho mut std::collections::HashMap<
            String,
            haskelujah_typing_chirho::SchemeChirho,
        >,
    >,
    pub(crate) imported_synonyms_chirho: &'source_chirho ImportedTypeSynonymsChirho,
    /// A synthetic source name has no authority to locate an hs-boot file.
    pub(crate) checked_path_chirho: Option<&'source_chirho Path>,
}

fn has_closed_families_chirho(module_chirho: &ModuleChirho) -> bool {
    module_chirho.decls_chirho.iter().any(|declaration_chirho| {
        matches!(
            declaration_chirho,
            DeclChirho::TypeFamilyDeclChirho {
                closed_chirho: true,
                data_family_chirho: false,
                ..
            }
        )
    })
}

const DISCOVERY_WORK_CHIRHO: usize = 100_000;

/// Discovery may own a different SourceMap from the checked source. Keep the
/// foreign producer's span as metadata, but render its actual path explicitly
/// instead of interpreting its FileId in the importing caller's SourceMap.
fn discovered_signature_diagnostics_chirho(
    error_chirho: haskelujah_ast_chirho::decl_chirho::ConstructorSignatureErrorChirho,
    module_name_chirho: &str,
    source_chirho: &DiscoveredSourceModuleChirho,
) -> DiagnosticBundleChirho {
    let mut diagnostic_chirho =
        constructor_signature_diagnostic_chirho(error_chirho, module_name_chirho);
    diagnostic_chirho.labels_chirho.clear();
    let location_chirho = source_chirho
        .interface_error_chirho
        .as_ref()
        .filter(|record_chirho| {
            record_chirho.error_chirho.span_chirho() == error_chirho.span_chirho()
        })
        .and_then(|record_chirho| record_chirho.location_chirho)
        .map(|location_chirho| {
            format!(
                ":{}:{}",
                location_chirho.line_chirho, location_chirho.col_chirho
            )
        })
        .unwrap_or_default();
    diagnostic_chirho.notes_chirho.push(format!(
        "source: {}{location_chirho}; constructor signature byte range {}..{}",
        source_chirho.path_chirho.display(),
        error_chirho.span_chirho().start_chirho().as_u32_chirho(),
        error_chirho.span_chirho().end_chirho().as_u32_chirho(),
    ));
    DiagnosticBundleChirho::new_chirho(vec![diagnostic_chirho])
}

struct SourceInterfaceDiagnosticsChirho<'source_chirho> {
    sources_chirho: &'source_chirho DiscoveredSourceModulesChirho,
    checked_name_chirho: &'source_chirho str,
}

/// Cold modules have their own SourceMap. Their byte ranges remain producer
/// coordinates and must never be rendered as importing-caller locations.
fn source_bundle_diagnostics_chirho(
    bundle_chirho: DiagnosticBundleChirho,
    module_chirho: &ModuleChirho,
    context_chirho: &SourceInterfaceDiagnosticsChirho<'_>,
) -> DiagnosticBundleChirho {
    let name_chirho = module_chirho.name_chirho.full_name_chirho();
    if name_chirho == context_chirho.checked_name_chirho {
        return bundle_chirho;
    }
    let Some(source_chirho) = context_chirho.sources_chirho.get(&name_chirho) else {
        return bundle_chirho;
    };
    let mut diagnostics_chirho = bundle_chirho.into_vec_chirho();
    for diagnostic_chirho in &mut diagnostics_chirho {
        diagnostic_chirho.notes_chirho.push(format!(
            "source: {} (module {name_chirho})",
            source_chirho.path_chirho.display(),
        ));
        for label_chirho in std::mem::take(&mut diagnostic_chirho.labels_chirho) {
            diagnostic_chirho.notes_chirho.push(format!(
                "producer byte range {}..{}: {}",
                label_chirho.span_chirho.start_chirho().as_u32_chirho(),
                label_chirho.span_chirho.end_chirho().as_u32_chirho(),
                label_chirho.message_chirho.as_deref().unwrap_or(""),
            ));
        }
        for suggestion_chirho in std::mem::take(&mut diagnostic_chirho.suggestions_chirho) {
            diagnostic_chirho.notes_chirho.push(format!(
                "producer suggestion at byte range {}..{}: {}",
                suggestion_chirho.span_chirho.start_chirho().as_u32_chirho(),
                suggestion_chirho.span_chirho.end_chirho().as_u32_chirho(),
                suggestion_chirho.description_chirho,
            ));
        }
    }
    DiagnosticBundleChirho::new_chirho(diagnostics_chirho)
}

fn source_family_diagnostics_chirho(
    error_chirho: FamilyExportsErrorChirho,
    module_chirho: &ModuleChirho,
    context_chirho: &SourceInterfaceDiagnosticsChirho<'_>,
) -> DiagnosticBundleChirho {
    if let FamilyExportsErrorChirho::ConstructorSignatureChirho {
        module_chirho: name_chirho,
        error_chirho,
    } = &error_chirho
        && name_chirho != context_chirho.checked_name_chirho
        && let Some(source_chirho) = context_chirho.sources_chirho.get(name_chirho)
    {
        return discovered_signature_diagnostics_chirho(*error_chirho, name_chirho, source_chirho);
    }
    error_chirho.into_diagnostics_chirho(module_chirho.span_chirho)
}

fn reachable_chirho(
    name_chirho: &str,
    modules_chirho: &BTreeMap<String, &ModuleChirho>,
    work_chirho: &mut usize,
) -> Result<BTreeSet<String>, String> {
    let mut reached_chirho = BTreeSet::new();
    let mut pending_chirho = vec![name_chirho.to_owned()];
    while let Some(name_chirho) = pending_chirho.pop() {
        *work_chirho = work_chirho
            .checked_sub(1)
            .ok_or_else(|| "source family import work budget exhausted".to_owned())?;
        if reached_chirho.contains(&name_chirho) {
            continue;
        }
        let Some(module_chirho) = modules_chirho.get(&name_chirho) else {
            continue;
        };
        reached_chirho.insert(name_chirho);
        pending_chirho.extend(
            module_chirho
                .imports_chirho
                .iter()
                .map(|import_chirho| import_chirho.module_chirho.full_name_chirho()),
        );
    }
    Ok(reached_chirho)
}

fn alias_snapshot_chirho(
    modules_chirho: &[&ModuleChirho],
    ifaces_chirho: &[ModuleIfaceChirho],
    registry_chirho: &TypeFamilyRegistryChirho,
) -> Vec<(String, BTreeSet<String>)> {
    let mut snapshot_chirho = Vec::new();
    for module_chirho in modules_chirho {
        let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
        if let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        {
            for name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
                let alias_chirho = format!("{module_name_chirho}.{name_chirho}");
                let identities_chirho = match registry_chirho.resolve_export_chirho(&alias_chirho) {
                    TypeFamilyAliasResolutionChirho::MissingChirho => BTreeSet::new(),
                    TypeFamilyAliasResolutionChirho::UniqueChirho(definition_chirho) => {
                        BTreeSet::from([definition_chirho.id_chirho().key_chirho()])
                    }
                    TypeFamilyAliasResolutionChirho::AmbiguousChirho(identities_chirho) => {
                        identities_chirho
                            .iter()
                            .map(|identity_chirho| identity_chirho.key_chirho())
                            .collect()
                    }
                };
                snapshot_chirho.push((alias_chirho, identities_chirho));
            }
        }
    }
    snapshot_chirho.sort();
    snapshot_chirho
}

/// Header publication is private and provisional. Its interface omits only a
/// family whose real definition has not been registered. The ordinary strict
/// publisher still verifies every selected target before changing aliases.
fn publish_headers_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    registry_chirho: &mut TypeFamilyRegistryChirho,
    diagnostics_chirho: &SourceInterfaceDiagnosticsChirho<'_>,
) -> Result<(), DiagnosticBundleChirho> {
    let scope_chirho =
        build_type_family_scope_chirho(module_chirho, ifaces_chirho, registry_chirho).map_err(
            |error_chirho| {
                family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
            },
        )?;
    let mut iface_chirho =
        build_family_aware_iface_chirho(module_chirho, ifaces_chirho, registry_chirho).map_err(
            |error_chirho| {
                source_family_diagnostics_chirho(error_chirho, module_chirho, diagnostics_chirho)
            },
        )?;
    iface_chirho
        .exports_chirho
        .types_chirho
        .retain(|name_chirho, _type_chirho| {
            match scope_chirho.resolve_chirho(name_chirho, registry_chirho) {
                TypeFamilyScopeResolutionChirho::UniqueChirho(key_chirho) => {
                    registry_chirho.definition_chirho(key_chirho).is_some()
                }
                TypeFamilyScopeResolutionChirho::AmbiguousChirho(keys_chirho) => keys_chirho
                    .iter()
                    .all(|key_chirho| registry_chirho.definition_chirho(key_chirho).is_some()),
                TypeFamilyScopeResolutionChirho::MissingChirho => true,
            }
        });
    publish_type_family_exports_chirho(
        module_chirho,
        &iface_chirho,
        ifaces_chirho,
        &scope_chirho,
        registry_chirho,
    )
    .map_err(|error_chirho| {
        family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
    })?;
    admit_authoritative_iface_chirho(ifaces_chirho, iface_chirho);
    Ok(())
}

fn propagate_headers_chirho(
    modules_chirho: &[&ModuleChirho],
    ifaces_chirho: &mut Vec<ModuleIfaceChirho>,
    registry_chirho: &mut TypeFamilyRegistryChirho,
    work_chirho: &mut usize,
    diagnostics_chirho: &SourceInterfaceDiagnosticsChirho<'_>,
) -> Result<(), DiagnosticBundleChirho> {
    for _round_chirho in 0..=modules_chirho.len() {
        let before_chirho = alias_snapshot_chirho(modules_chirho, ifaces_chirho, registry_chirho);
        for module_chirho in modules_chirho {
            *work_chirho = work_chirho.checked_sub(1).ok_or_else(|| {
                family_frontend_diagnostics_chirho(
                    "source family publication work budget exhausted",
                    module_chirho.span_chirho,
                )
            })?;
            publish_headers_chirho(
                module_chirho,
                ifaces_chirho,
                registry_chirho,
                diagnostics_chirho,
            )?;
        }
        if alias_snapshot_chirho(modules_chirho, ifaces_chirho, registry_chirho) == before_chirho {
            return Ok(());
        }
    }
    Err(family_frontend_diagnostics_chirho(
        "source family header export cycle did not converge",
        modules_chirho[0].span_chirho,
    ))
}

/// Atomically add only the checked module's import-reachable authoritative ASTs.
/// The checked AST participates so a SOURCE companion can instance its family.
pub(crate) fn register_discovered_type_families_chirho(
    checked_chirho: &ModuleChirho,
    context_chirho: DiscoveredFamilyContextChirho<'_>,
) -> Result<(), DiagnosticBundleChirho> {
    let mut modules_chirho = context_chirho
        .modules_chirho
        .iter()
        .map(|(name_chirho, source_chirho)| (name_chirho.clone(), &source_chirho.module_chirho))
        .collect::<BTreeMap<_, _>>();
    let checked_name_chirho = checked_chirho.name_chirho.full_name_chirho();
    modules_chirho.insert(checked_name_chirho.clone(), checked_chirho);
    let diagnostics_chirho = SourceInterfaceDiagnosticsChirho {
        sources_chirho: context_chirho.modules_chirho,
        checked_name_chirho: &checked_name_chirho,
    };
    let mut work_chirho = DISCOVERY_WORK_CHIRHO;
    let reached_chirho = reachable_chirho(&checked_name_chirho, &modules_chirho, &mut work_chirho)
        .map_err(|error_chirho| {
            family_frontend_diagnostics_chirho(error_chirho, checked_chirho.span_chirho)
        })?;
    for name_chirho in &reached_chirho {
        // The checked AST overrides a candidate with the same declared name.
        if name_chirho == &checked_name_chirho {
            continue;
        }
        if let Some(source_chirho) = context_chirho.modules_chirho.get(name_chirho)
            && let Some(failure_chirho) = &source_chirho.interface_error_chirho
        {
            return Err(discovered_signature_diagnostics_chirho(
                failure_chirho.error_chirho,
                name_chirho,
                source_chirho,
            ));
        }
    }
    let mut paths_chirho = context_chirho
        .modules_chirho
        .iter()
        .map(|(name_chirho, source_chirho)| {
            (name_chirho.clone(), source_chirho.path_chirho.as_path())
        })
        .collect::<BTreeMap<_, _>>();
    if let Some(path_chirho) = context_chirho.checked_path_chirho {
        paths_chirho.insert(checked_name_chirho.clone(), path_chirho);
    } else {
        // The checked source overrides any candidate sharing its declared name.
        paths_chirho.remove(&checked_name_chirho);
    }
    validate_reached_boot_contracts_chirho(
        &modules_chirho,
        BootContractContextChirho {
            paths_chirho: &paths_chirho,
            reached_chirho: &reached_chirho,
        },
    )?;
    let ordered_chirho = reached_chirho
        .iter()
        .filter_map(|name_chirho| modules_chirho.get(name_chirho).copied())
        .collect::<Vec<_>>();
    let mut dependencies_chirho = BTreeMap::new();
    for module_chirho in &ordered_chirho {
        let name_chirho = module_chirho.name_chirho.full_name_chirho();
        let dependencies_for_module_chirho =
            reachable_chirho(&name_chirho, &modules_chirho, &mut work_chirho)
                .map_err(|error_chirho| {
                    family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
                })?
                .into_iter()
                .filter(|dependency_chirho| {
                    dependency_chirho != &name_chirho
                        && modules_chirho
                            .get(dependency_chirho)
                            .is_some_and(|module_chirho| has_closed_families_chirho(module_chirho))
                })
                .collect::<BTreeSet<_>>();
        dependencies_chirho.insert(name_chirho, dependencies_for_module_chirho);
    }
    let mut registry_chirho = context_chirho.registry_chirho.clone();
    let mut ifaces_chirho = context_chirho.ifaces_chirho.clone();
    // Preserve naming's positive declaration identities while private family
    // publication temporarily omits definitions whose reduction bodies are pending.
    let mut declaration_ifaces_chirho = ifaces_chirho.clone();
    for _round_chirho in 0..ordered_chirho.len() {
        for module_chirho in &ordered_chirho {
            work_chirho = work_chirho.checked_sub(1).ok_or_else(|| {
                family_frontend_diagnostics_chirho(
                    "source declaration interface work bound exhausted",
                    module_chirho.span_chirho,
                )
            })?;
            let iface_chirho = haskelujah_naming_chirho::build_iface_with_imports_chirho(
                module_chirho,
                &declaration_ifaces_chirho,
            )
            .map_err(|error_chirho| {
                DiagnosticBundleChirho::new_chirho(vec![constructor_signature_diagnostic_chirho(
                    error_chirho,
                    &module_chirho.name_chirho.full_name_chirho(),
                )])
            })?;
            admit_authoritative_iface_chirho(&mut declaration_ifaces_chirho, iface_chirho);
        }
    }
    let mut kind_dependencies_chirho = BTreeMap::new();
    for module_chirho in &ordered_chirho {
        let type_scope_chirho = haskelujah_naming_chirho::build_type_constructor_scope_chirho(
            module_chirho,
            &declaration_ifaces_chirho,
        );
        let family_scope_chirho =
            build_type_family_scope_chirho(module_chirho, &ifaces_chirho, &registry_chirho)
                .map_err(|error_chirho| {
                    family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
                })?;
        let promotion_scope_chirho = haskelujah_naming_chirho::build_promotion_scope_chirho(
            module_chirho,
            &declaration_ifaces_chirho,
        );
        let builtin_classes_chirho = haskelujah_naming_chirho::build_builtin_class_scope_chirho(
            module_chirho,
            &declaration_ifaces_chirho,
        );
        let dependencies_chirho = haskelujah_typing_chirho::kind_chirho::elaboration_chirho::source_declaration_dependencies_chirho(module_chirho, haskelujah_typing_chirho::kind_chirho::elaboration_chirho::ElaborationImportsChirho { registry_chirho: &registry_chirho, type_scope_chirho: &type_scope_chirho, family_scope_chirho: &family_scope_chirho, promotion_scope_chirho: &promotion_scope_chirho, builtin_classes_chirho: &builtin_classes_chirho }, &ordered_chirho).map_err(|error_chirho| family_frontend_diagnostics_chirho(error_chirho.message_chirho, error_chirho.span_chirho))?;
        kind_dependencies_chirho.insert(
            module_chirho.name_chirho.full_name_chirho(),
            dependencies_chirho,
        );
    }
    let mut signature_exports_chirho = Vec::new();
    let mut signature_budget_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::with_limits_chirho(
            haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitsChirho {
                work_chirho: 1 << 25,
                output_chirho: 1 << 24,
                depth_chirho: 1024,
            },
        );
    let empty_names_chirho = std::collections::HashSet::new();
    let empty_preferred_chirho = std::collections::HashMap::new();
    let mut staged_chirho = BTreeSet::new();
    let mut unstaged_chirho = ordered_chirho.clone();
    while !unstaged_chirho.is_empty() {
        let Some(index_chirho) = unstaged_chirho.iter().position(|module_chirho| {
            kind_dependencies_chirho[&module_chirho.name_chirho.full_name_chirho()]
                .is_subset(&staged_chirho)
        }) else {
            return Err(family_frontend_diagnostics_chirho(
                "source declaration kind cycle has no checked telescope contract",
                checked_chirho.span_chirho,
            ));
        };
        let module_chirho = unstaged_chirho.remove(index_chirho);
        let scope_chirho = Arc::new(
            build_type_family_scope_chirho(module_chirho, &ifaces_chirho, &registry_chirho)
                .map_err(|error_chirho| {
                    family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
                })?,
        );
        let safe_names_chirho = crate::collect_safe_unqualified_imported_type_names_chirho(
            module_chirho,
            &ifaces_chirho,
        );
        let preferred_names_chirho =
            crate::collect_preferred_qualified_type_names_chirho(module_chirho, &ifaces_chirho);
        let result_chirho = predeclare_source_open_families_chirho(
            module_chirho,
            SourceFamilyRegistrationContextChirho {
                registry_chirho: &registry_chirho,
                family_scope_chirho: scope_chirho,
                promotion_scope_chirho: Arc::new(
                    haskelujah_naming_chirho::build_promotion_scope_chirho(
                        module_chirho,
                        &declaration_ifaces_chirho,
                    ),
                ),
                type_scope_chirho: Arc::new(
                    haskelujah_naming_chirho::build_type_constructor_scope_chirho(
                        module_chirho,
                        &declaration_ifaces_chirho,
                    ),
                ),
                builtin_classes_chirho: haskelujah_naming_chirho::build_builtin_class_scope_chirho(
                    module_chirho,
                    &declaration_ifaces_chirho,
                ),
                imported_synonyms_chirho: context_chirho.imported_synonyms_chirho,
                safe_unqualified_type_names_chirho: &safe_names_chirho,
                preferred_qualified_type_names_chirho: &preferred_names_chirho,
            },
        )
        .map_err(|bundle_chirho| {
            source_bundle_diagnostics_chirho(bundle_chirho, module_chirho, &diagnostics_chirho)
        })?;
        registry_chirho = result_chirho.registry_chirho;
        staged_chirho.insert(module_chirho.name_chirho.full_name_chirho());
        propagate_headers_chirho(
            &ordered_chirho,
            &mut ifaces_chirho,
            &mut registry_chirho,
            &mut work_chirho,
            &diagnostics_chirho,
        )?;
    }
    propagate_headers_chirho(
        &ordered_chirho,
        &mut ifaces_chirho,
        &mut registry_chirho,
        &mut work_chirho,
        &diagnostics_chirho,
    )?;
    let mut pending_chirho = ordered_chirho.clone();
    let mut closed_registered_chirho = BTreeSet::new();
    while !pending_chirho.is_empty() {
        let Some(index_chirho) = pending_chirho.iter().position(|module_chirho| {
            dependencies_chirho[&module_chirho.name_chirho.full_name_chirho()]
                .is_subset(&closed_registered_chirho)
        }) else {
            let cycle_chirho = pending_chirho
                .iter()
                .map(|module_chirho| module_chirho.name_chirho.full_name_chirho())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(family_frontend_diagnostics_chirho(
                format!("closed type-family source dependency cycle: {cycle_chirho}"),
                checked_chirho.span_chirho,
            ));
        };
        let module_chirho = pending_chirho.remove(index_chirho);
        let scope_chirho = Arc::new(
            build_type_family_scope_chirho(module_chirho, &ifaces_chirho, &registry_chirho)
                .map_err(|error_chirho| {
                    family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
                })?,
        );
        let safe_names_chirho = crate::collect_safe_unqualified_imported_type_names_chirho(
            module_chirho,
            &ifaces_chirho,
        );
        let preferred_names_chirho =
            crate::collect_preferred_qualified_type_names_chirho(module_chirho, &ifaces_chirho);
        let result_chirho = register_source_module_families_chirho(
            module_chirho,
            SourceFamilyRegistrationContextChirho {
                registry_chirho: &registry_chirho,
                family_scope_chirho: scope_chirho,
                promotion_scope_chirho: Arc::new(
                    haskelujah_naming_chirho::build_promotion_scope_chirho(
                        module_chirho,
                        &declaration_ifaces_chirho,
                    ),
                ),
                type_scope_chirho: Arc::new(
                    haskelujah_naming_chirho::build_type_constructor_scope_chirho(
                        module_chirho,
                        &declaration_ifaces_chirho,
                    ),
                ),
                builtin_classes_chirho: haskelujah_naming_chirho::build_builtin_class_scope_chirho(
                    module_chirho,
                    &declaration_ifaces_chirho,
                ),
                imported_synonyms_chirho: context_chirho.imported_synonyms_chirho,
                safe_unqualified_type_names_chirho: &safe_names_chirho,
                preferred_qualified_type_names_chirho: &preferred_names_chirho,
            },
        )
        .map_err(|bundle_chirho| {
            source_bundle_diagnostics_chirho(bundle_chirho, module_chirho, &diagnostics_chirho)
        })?;
        registry_chirho = result_chirho.registry_chirho;
        let iface_chirho =
            build_family_aware_iface_chirho(module_chirho, &ifaces_chirho, &registry_chirho)
                .map_err(|error_chirho| {
                    source_family_diagnostics_chirho(
                        error_chirho,
                        module_chirho,
                        &diagnostics_chirho,
                    )
                })?;
        if module_chirho.name_chirho.full_name_chirho() != checked_name_chirho
            && context_chirho.imported_types_chirho.is_some()
        {
            let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
            let mut copy_chirho = crate::ImportQualificationChirho {
                iface_chirho: None,
                type_names_chirho: &empty_names_chirho,
                qualifier_chirho: "",
                safe_chirho: &empty_names_chirho,
                preferred_chirho: &empty_preferred_chirho,
                budget_chirho: &mut signature_budget_chirho,
                span_chirho: module_chirho.span_chirho,
            };
            for declaration_chirho in &module_chirho.decls_chirho {
                if let DeclChirho::TypeSigChirho { name_chirho, .. } = declaration_chirho {
                    // Naming selects the real defining export. A private/stub
                    // spelling cannot become a callable imported value here.
                    if !iface_chirho
                        .exports_chirho
                        .values_chirho
                        .get(name_chirho.text_chirho())
                        .is_some_and(|value_chirho| {
                            value_chirho.defining_module_chirho.as_deref()
                                == Some(module_name_chirho.as_str())
                        })
                    {
                        continue;
                    }
                    if let Some(scheme_chirho) = result_chirho
                        .env_chirho
                        .lookup_chirho(name_chirho.text_chirho())
                    {
                        let producer_chirho = copy_chirho.key_chirho(&module_name_chirho).map_err(
                            |bundle_chirho| {
                                source_bundle_diagnostics_chirho(
                                    bundle_chirho,
                                    module_chirho,
                                    &diagnostics_chirho,
                                )
                            },
                        )?;
                        let name_chirho = copy_chirho
                            .key_chirho(name_chirho.text_chirho())
                            .map_err(|bundle_chirho| {
                                source_bundle_diagnostics_chirho(
                                    bundle_chirho,
                                    module_chirho,
                                    &diagnostics_chirho,
                                )
                            })?;
                        let scheme_chirho = copy_chirho
                            .scheme_chirho(scheme_chirho, false)
                            .map_err(|bundle_chirho| {
                                source_bundle_diagnostics_chirho(
                                    bundle_chirho,
                                    module_chirho,
                                    &diagnostics_chirho,
                                )
                            })?;
                        signature_exports_chirho
                            .push((format!("{producer_chirho}.{name_chirho}"), scheme_chirho));
                    }
                }
            }
        }
        publish_type_family_exports_chirho(
            module_chirho,
            &iface_chirho,
            &ifaces_chirho,
            &result_chirho.family_scope_chirho,
            &mut registry_chirho,
        )
        .map_err(|error_chirho| {
            family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
        })?;
        admit_authoritative_iface_chirho(&mut ifaces_chirho, iface_chirho);
        if has_closed_families_chirho(module_chirho) {
            closed_registered_chirho.insert(module_chirho.name_chirho.full_name_chirho());
            propagate_headers_chirho(
                &ordered_chirho,
                &mut ifaces_chirho,
                &mut registry_chirho,
                &mut work_chirho,
                &diagnostics_chirho,
            )?;
        }
    }
    // Strict final publication is mandatory after every real body exists.
    for module_chirho in &ordered_chirho {
        let scope_chirho =
            build_type_family_scope_chirho(module_chirho, &ifaces_chirho, &registry_chirho)
                .map_err(|error_chirho| {
                    family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
                })?;
        let iface_chirho =
            build_family_aware_iface_chirho(module_chirho, &ifaces_chirho, &registry_chirho)
                .map_err(|error_chirho| {
                    source_family_diagnostics_chirho(
                        error_chirho,
                        module_chirho,
                        &diagnostics_chirho,
                    )
                })?;
        publish_type_family_exports_chirho(
            module_chirho,
            &iface_chirho,
            &ifaces_chirho,
            &scope_chirho,
            &mut registry_chirho,
        )
        .map_err(|error_chirho| {
            family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
        })?;
        admit_authoritative_iface_chirho(&mut ifaces_chirho, iface_chirho);
    }
    if let Some(imported_types_chirho) = context_chirho.imported_types_chirho {
        for (key_chirho, scheme_chirho) in signature_exports_chirho {
            imported_types_chirho.insert(key_chirho, scheme_chirho);
        }
    }
    *context_chirho.registry_chirho = registry_chirho;
    *context_chirho.ifaces_chirho = ifaces_chirho;
    Ok(())
}

#[cfg(test)]
#[path = "family_discovery_tests_chirho.rs"]
mod tests_chirho;
