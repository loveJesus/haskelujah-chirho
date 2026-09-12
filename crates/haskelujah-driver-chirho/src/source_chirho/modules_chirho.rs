// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source dependency frontend orchestration.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use crate::*;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub(crate) struct FrontendSeedArtifactsChirho {
    pub(crate) type_contracts_chirho: ImportedTypeContractsChirho,
    pub(crate) ifaces_chirho: Vec<ModuleIfaceChirho>,
    pub(crate) imported_types_chirho:
        std::collections::HashMap<String, haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    pub(crate) imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    pub(crate) imported_type_families_chirho: ImportedTypeFamiliesChirho,
}

impl FrontendSeedArtifactsChirho {
    pub(crate) fn for_source_chirho(source_chirho: &str) -> Self {
        let mut seed_chirho = Self {
            ifaces_chirho: haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
            imported_type_families_chirho: seed_builtin_type_families_chirho(),
            ..Self::default()
        };
        if source_imports_stdlib_chirho(source_chirho) {
            merge_stdlib_frontend_artifacts_chirho(
                &mut seed_chirho.ifaces_chirho,
                &mut seed_chirho.imported_types_chirho,
                &mut seed_chirho.imported_type_synonyms_chirho,
                &mut seed_chirho.imported_type_families_chirho,
                &mut seed_chirho.type_contracts_chirho,
            );
        }
        seed_chirho
    }
}

/// Iterative dependency ordering with deterministic independent-module order.
/// A cycle is not a checked interface; this frontend-only producer has no boot
/// contract input. Each source index and dependency is visited a bounded number
/// of times rather than recursively searching the source vector at every edge.
fn source_order_chirho(
    sources_chirho: &[(String, String, String)],
    source_map_chirho: &mut SourceMapChirho,
) -> Result<Vec<usize>, String> {
    let mut indices_chirho = HashMap::new();
    for (index_chirho, (name_chirho, _, _)) in sources_chirho.iter().enumerate() {
        if indices_chirho
            .insert(name_chirho.as_str(), index_chirho)
            .is_some()
        {
            return Err(format!("multiple source owners for module {name_chirho}"));
        }
    }
    let mut pending_chirho = vec![0; sources_chirho.len()];
    let mut dependents_chirho = vec![Vec::new(); sources_chirho.len()];
    for (index_chirho, (name_chirho, path_chirho, source_chirho)) in
        sources_chirho.iter().enumerate()
    {
        let (declared_chirho, imports_chirho) = crate::module_search_chirho::module_header_chirho(
            source_chirho,
            path_chirho,
            source_map_chirho,
        )?;
        if declared_chirho != *name_chirho {
            return Err(format!(
                "source owner {path_chirho} declares {declared_chirho}, expected {name_chirho}"
            ));
        }
        let dependencies_chirho: HashSet<_> = imports_chirho
            .iter()
            .filter_map(|name_chirho| indices_chirho.get(name_chirho.as_str()).copied())
            .collect();
        pending_chirho[index_chirho] = dependencies_chirho.len();
        for dependency_chirho in dependencies_chirho {
            dependents_chirho[dependency_chirho].push(index_chirho);
        }
    }
    let mut ready_chirho: BTreeSet<_> = pending_chirho
        .iter()
        .enumerate()
        .filter(|(_, count_chirho)| **count_chirho == 0)
        .map(|(index_chirho, _)| (sources_chirho[index_chirho].0.as_str(), index_chirho))
        .collect();
    let mut order_chirho = Vec::with_capacity(sources_chirho.len());
    while let Some((_, index_chirho)) = ready_chirho.pop_first() {
        order_chirho.push(index_chirho);
        for dependent_chirho in &dependents_chirho[index_chirho] {
            pending_chirho[*dependent_chirho] -= 1;
            if pending_chirho[*dependent_chirho] == 0 {
                ready_chirho.insert((
                    sources_chirho[*dependent_chirho].0.as_str(),
                    *dependent_chirho,
                ));
            }
        }
    }
    if order_chirho.len() != sources_chirho.len() {
        let blocked_chirho: Vec<_> = pending_chirho
            .iter()
            .enumerate()
            .filter(|(_, count_chirho)| **count_chirho > 0)
            .take(4)
            .map(|(index_chirho, _)| sources_chirho[index_chirho].0.as_str())
            .collect();
        return Err(format!(
            "source import cycle involving {}; a checked hs-boot contract is required",
            blocked_chirho.join(", ")
        ));
    }
    Ok(order_chirho)
}

pub(crate) fn collect_frontend_artifacts_from_module_sources_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
    initial_type_contracts_chirho: ImportedTypeContractsChirho,
    seed_stdlib_frontend_artifacts_chirho: bool,
) -> Result<FrontendSeedArtifactsChirho, String> {
    let order_chirho = source_order_chirho(&module_sources_chirho, source_map_chirho)?;
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    // Add cross-package dependency interfaces
    ifaces_chirho.extend(extra_ifaces_chirho);
    let mut ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(ifaces_chirho);
    let mut imported_types_chirho = initial_imported_types_chirho;
    let mut imported_type_synonyms_chirho = initial_imported_type_synonyms_chirho;
    let mut imported_type_families_chirho = initial_imported_type_families_chirho;
    let mut imported_type_contracts_chirho = initial_type_contracts_chirho;
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
            &mut imported_type_contracts_chirho,
        );
    }

    let mut iface_slots_chirho: HashMap<_, _> = ifaces_chirho
        .iter()
        .enumerate()
        .map(|(index_chirho, iface_chirho)| (iface_chirho.name_chirho.clone(), index_chirho))
        .collect();
    for index_chirho in order_chirho {
        let (module_name_chirho, file_name_chirho, source_chirho) =
            &module_sources_chirho[index_chirho];

        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            file_name_chirho,
            source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();
        let filtered_imported_types_chirho =
            filter_seeded_imported_types_for_source_chirho(source_chirho, &imported_types_chirho);
        let filtered_imported_type_synonyms_chirho = filter_seeded_type_synonyms_for_source_chirho(
            source_chirho,
            &imported_type_synonyms_chirho,
        );

        let frontend_result_chirho = run_frontend_with_inputs_chirho(
            source_chirho,
            file_id_chirho,
            FrontendInputsChirho::new_chirho(
                &ifaces_chirho,
                &filtered_imported_types_chirho,
                &filtered_imported_type_synonyms_chirho,
                &imported_type_families_chirho,
            )
            .with_class_env_chirho(&imported_class_env_chirho)
            .with_type_contracts_chirho(&imported_type_contracts_chirho),
        )
        .map_err(|e_chirho| {
            format!(
                "Error checking dependency {} ({}): {}",
                module_name_chirho, file_name_chirho, e_chirho
            )
        })?;

        let FrontendResultChirho {
            module_chirho,
            infer_result_chirho,
            resolved_imported_type_synonyms_chirho,
            warnings_chirho: _warnings_chirho,
            type_contracts_chirho,
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
        imported_type_contracts_chirho
            .insert(iface_chirho.name_chirho.clone(), type_contracts_chirho);
        if let Some(slot_chirho) = iface_slots_chirho.get(&iface_chirho.name_chirho) {
            ifaces_chirho[*slot_chirho] = iface_chirho;
        } else {
            iface_slots_chirho.insert(iface_chirho.name_chirho.clone(), ifaces_chirho.len());
            ifaces_chirho.push(iface_chirho);
        }
    }

    Ok(FrontendSeedArtifactsChirho {
        ifaces_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_type_families_chirho,
        type_contracts_chirho: imported_type_contracts_chirho,
    })
}
