// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family declarations retained by authoritative source discovery. This is a
//! cold declaration pass; siblings' expressions are not inferred or linked.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.md.

use super::boot_contract_chirho::{
    BootContractContextChirho, validate_reached_boot_contracts_chirho,
};
use super::{DiscoveredSourceModulesChirho, admit_authoritative_iface_chirho};
use crate::ImportedTypeSynonymsChirho;
use crate::family_exports_chirho::{
    build_family_aware_iface_chirho, build_type_family_scope_chirho,
    family_frontend_diagnostics_chirho, publish_type_family_exports_chirho,
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
) -> Result<(), DiagnosticBundleChirho> {
    let scope_chirho =
        build_type_family_scope_chirho(module_chirho, ifaces_chirho, registry_chirho).map_err(
            |error_chirho| {
                family_frontend_diagnostics_chirho(error_chirho, module_chirho.span_chirho)
            },
        )?;
    let mut iface_chirho =
        build_family_aware_iface_chirho(module_chirho, ifaces_chirho, registry_chirho);
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
            publish_headers_chirho(module_chirho, ifaces_chirho, registry_chirho)?;
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
    let mut work_chirho = DISCOVERY_WORK_CHIRHO;
    let reached_chirho = reachable_chirho(&checked_name_chirho, &modules_chirho, &mut work_chirho)
        .map_err(|error_chirho| {
            family_frontend_diagnostics_chirho(error_chirho, checked_chirho.span_chirho)
        })?;
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
    for module_chirho in &ordered_chirho {
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
                        &ifaces_chirho,
                    ),
                ),
                imported_synonyms_chirho: context_chirho.imported_synonyms_chirho,
                safe_unqualified_type_names_chirho: &safe_names_chirho,
                preferred_qualified_type_names_chirho: &preferred_names_chirho,
            },
        )?;
        registry_chirho = result_chirho.registry_chirho;
    }
    propagate_headers_chirho(
        &ordered_chirho,
        &mut ifaces_chirho,
        &mut registry_chirho,
        &mut work_chirho,
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
                        &ifaces_chirho,
                    ),
                ),
                imported_synonyms_chirho: context_chirho.imported_synonyms_chirho,
                safe_unqualified_type_names_chirho: &safe_names_chirho,
                preferred_qualified_type_names_chirho: &preferred_names_chirho,
            },
        )?;
        registry_chirho = result_chirho.registry_chirho;
        let iface_chirho =
            build_family_aware_iface_chirho(module_chirho, &ifaces_chirho, &registry_chirho);
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
            build_family_aware_iface_chirho(module_chirho, &ifaces_chirho, &registry_chirho);
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
    *context_chirho.registry_chirho = registry_chirho;
    *context_chirho.ifaces_chirho = ifaces_chirho;
    Ok(())
}

#[cfg(test)]
#[path = "family_discovery_tests_chirho.rs"]
mod tests_chirho;
