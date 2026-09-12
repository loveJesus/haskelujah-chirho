// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real implementations and their checked boot promises form separate graph nodes.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::modules_chirho::FrontendSeedArtifactsChirho;
use crate::module_search_chirho::{
    LocalModuleSearchChirho, LocalModuleSourceChirho, MAX_MODULE_SEARCH_FILES_CHIRHO,
    ModuleSourceKeyChirho, module_header_chirho,
};
use crate::*;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::sync::Arc;

pub(super) enum SourceGraphErrorChirho {
    RootChirho(DiagnosticBundleChirho),
    DependencyChirho(String),
}

impl From<String> for SourceGraphErrorChirho {
    fn from(message_chirho: String) -> Self {
        Self::DependencyChirho(message_chirho)
    }
}

impl From<&str> for SourceGraphErrorChirho {
    fn from(message_chirho: &str) -> Self {
        Self::DependencyChirho(message_chirho.to_owned())
    }
}

struct SourceGraphChirho {
    keys_chirho: Vec<ModuleSourceKeyChirho>,
    sources_chirho: Vec<Arc<LocalModuleSourceChirho>>,
    indices_chirho: HashMap<ModuleSourceKeyChirho, usize>,
    dependencies_chirho: Vec<Vec<usize>>,
}

impl SourceGraphChirho {
    fn discover_chirho(
        source_chirho: &str,
        file_name_chirho: &str,
        search_dir_chirho: &Path,
        source_map_chirho: &mut SourceMapChirho,
    ) -> Result<Self, String> {
        let (name_chirho, imports_chirho) =
            module_header_chirho(source_chirho, file_name_chirho, source_map_chirho)?;
        let root_chirho = ModuleSourceKeyChirho {
            name_chirho: name_chirho.clone(),
            boot_chirho: false,
        };
        let mut graph_chirho = Self {
            indices_chirho: HashMap::from([(root_chirho.clone(), 0)]),
            keys_chirho: vec![root_chirho],
            sources_chirho: vec![Arc::new(LocalModuleSourceChirho {
                name_chirho,
                imports_chirho,
                path_chirho: PathBuf::from(file_name_chirho),
                source_chirho: source_chirho.to_owned(),
            })],
            dependencies_chirho: Vec::new(),
        };
        let mut search_chirho = LocalModuleSearchChirho::new_chirho(search_dir_chirho)?;
        let mut missing_chirho = HashSet::new();
        let mut index_chirho = 0;
        while index_chirho < graph_chirho.sources_chirho.len() {
            let mut requests_chirho = graph_chirho.sources_chirho[index_chirho]
                .imports_chirho
                .clone();
            // An implementation behind a SOURCE edge must still be checked.
            // It is scheduled, not a dependency of its own boot declaration.
            if graph_chirho.keys_chirho[index_chirho].boot_chirho {
                requests_chirho.push(ModuleSourceKeyChirho {
                    name_chirho: graph_chirho.keys_chirho[index_chirho].name_chirho.clone(),
                    boot_chirho: false,
                });
            }
            for key_chirho in requests_chirho {
                if graph_chirho.indices_chirho.contains_key(&key_chirho)
                    || missing_chirho.contains(&key_chirho)
                {
                    continue;
                }
                if graph_chirho.indices_chirho.len() + missing_chirho.len()
                    >= MAX_MODULE_SEARCH_FILES_CHIRHO
                {
                    return Err("module dependency budget exhausted".to_owned());
                }
                let provider_chirho = if key_chirho.boot_chirho {
                    Some(
                        search_chirho
                            .lookup_boot_chirho(&key_chirho.name_chirho, source_map_chirho)?,
                    )
                } else {
                    search_chirho.lookup_chirho(&key_chirho.name_chirho, source_map_chirho)?
                };
                if let Some(provider_chirho) = provider_chirho {
                    graph_chirho
                        .indices_chirho
                        .insert(key_chirho.clone(), graph_chirho.sources_chirho.len());
                    graph_chirho.keys_chirho.push(key_chirho);
                    graph_chirho.sources_chirho.push(provider_chirho);
                } else {
                    missing_chirho.insert(key_chirho);
                }
            }
            index_chirho += 1;
        }
        for key_chirho in graph_chirho
            .keys_chirho
            .iter()
            .filter(|key_chirho| key_chirho.boot_chirho)
        {
            let implementation_chirho = ModuleSourceKeyChirho {
                name_chirho: key_chirho.name_chirho.clone(),
                boot_chirho: false,
            };
            if !graph_chirho
                .indices_chirho
                .contains_key(&implementation_chirho)
            {
                return Err(format!(
                    "boot contract for {} has no implementation source",
                    key_chirho.name_chirho
                ));
            }
        }
        graph_chirho.dependencies_chirho = graph_chirho
            .sources_chirho
            .iter()
            .map(|source_chirho| {
                source_chirho
                    .imports_chirho
                    .iter()
                    .filter_map(|key_chirho| graph_chirho.indices_chirho.get(key_chirho).copied())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            })
            .collect();
        Ok(graph_chirho)
    }

    fn order_chirho(&self) -> Result<Vec<usize>, String> {
        let mut pending_chirho: Vec<_> = self.dependencies_chirho.iter().map(Vec::len).collect();
        let mut dependents_chirho = vec![Vec::new(); pending_chirho.len()];
        for (index_chirho, dependencies_chirho) in self.dependencies_chirho.iter().enumerate() {
            for dependency_chirho in dependencies_chirho {
                dependents_chirho[*dependency_chirho].push(index_chirho);
            }
        }
        let mut ready_chirho: BTreeSet<_> = pending_chirho
            .iter()
            .enumerate()
            .filter(|(_, count_chirho)| **count_chirho == 0)
            .map(|(index_chirho, _)| (&self.keys_chirho[index_chirho], index_chirho))
            .collect();
        let mut order_chirho = Vec::new();
        while let Some((_, index_chirho)) = ready_chirho.pop_first() {
            order_chirho.push(index_chirho);
            for dependent_chirho in &dependents_chirho[index_chirho] {
                pending_chirho[*dependent_chirho] -= 1;
                if pending_chirho[*dependent_chirho] == 0 {
                    ready_chirho.insert((&self.keys_chirho[*dependent_chirho], *dependent_chirho));
                }
            }
        }
        if order_chirho.len() != self.sources_chirho.len() {
            let blocked_chirho: Vec<_> = pending_chirho
                .iter()
                .enumerate()
                .filter(|(_, count_chirho)| **count_chirho > 0)
                .take(4)
                .map(|(index_chirho, _)| {
                    self.sources_chirho[index_chirho]
                        .path_chirho
                        .display()
                        .to_string()
                })
                .collect();
            return Err(format!(
                "source import cycle not broken by checked boot contracts: {}",
                blocked_chirho.join(", ")
            ));
        }
        Ok(order_chirho)
    }

    /// Select a version before reading its semantic output. Direct requests take
    /// priority over a transitive view of the same nominal module. In particular,
    /// an implementation checked later never replaces an explicit SOURCE edge.
    fn selected_chirho(
        &self,
        index_chirho: usize,
        budget_chirho: &mut usize,
    ) -> Result<HashSet<usize>, String> {
        let mut selected_chirho = HashMap::new();
        let mut pending_chirho = VecDeque::new();
        for dependency_chirho in &self.dependencies_chirho[index_chirho] {
            let key_chirho = &self.keys_chirho[*dependency_chirho];
            if let Some(previous_chirho) =
                selected_chirho.insert(key_chirho.name_chirho.as_str(), *dependency_chirho)
                && previous_chirho != *dependency_chirho
            {
                return Err(format!(
                    "mixed boot and implementation imports of {} require per-import interface selection",
                    key_chirho.name_chirho
                ));
            }
            pending_chirho.push_back(*dependency_chirho);
        }
        while let Some(dependency_chirho) = pending_chirho.pop_front() {
            for child_chirho in &self.dependencies_chirho[dependency_chirho] {
                *budget_chirho = budget_chirho
                    .checked_sub(1)
                    .ok_or("module contract closure budget exhausted")?;
                let key_chirho = &self.keys_chirho[*child_chirho];
                if let std::collections::hash_map::Entry::Vacant(entry_chirho) =
                    selected_chirho.entry(key_chirho.name_chirho.as_str())
                {
                    entry_chirho.insert(*child_chirho);
                    pending_chirho.push_back(*child_chirho);
                }
            }
        }
        Ok(selected_chirho.into_values().collect())
    }
}

pub(super) fn check_source_graph_chirho(
    source_chirho: &str,
    root_file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    file_name_chirho: &str,
    search_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<FrontendResultChirho, SourceGraphErrorChirho> {
    let graph_chirho = SourceGraphChirho::discover_chirho(
        source_chirho,
        file_name_chirho,
        search_dir_chirho,
        source_map_chirho,
    )?;
    let order_chirho = graph_chirho.order_chirho()?;
    let mut ranks_chirho = vec![0; order_chirho.len()];
    for (rank_chirho, index_chirho) in order_chirho.iter().enumerate() {
        ranks_chirho[*index_chirho] = rank_chirho;
    }
    let mut checked_chirho: HashMap<usize, FrontendResultChirho> = HashMap::new();
    let mut ifaces_chirho = HashMap::new();
    let mut budget_chirho = 1_048_576usize;
    for index_chirho in &order_chirho {
        let mut selected_chirho: Vec<_> = graph_chirho
            .selected_chirho(*index_chirho, &mut budget_chirho)?
            .into_iter()
            .collect();
        selected_chirho.sort_unstable_by_key(|index_chirho| ranks_chirho[*index_chirho]);
        let source_chirho = &graph_chirho.sources_chirho[*index_chirho];
        let mut seed_chirho =
            FrontendSeedArtifactsChirho::for_source_chirho(&source_chirho.source_chirho);
        let mut slots_chirho = seed_chirho
            .ifaces_chirho
            .iter()
            .enumerate()
            .map(|(index_chirho, iface_chirho)| (iface_chirho.name_chirho.clone(), index_chirho))
            .collect();
        for dependency_chirho in &selected_chirho {
            let frontend_chirho = checked_chirho
                .get(dependency_chirho)
                .ok_or("dependency was not checked before publication")?;
            let iface_chirho = ifaces_chirho
                .get(dependency_chirho)
                .ok_or("dependency has no checked interface")?;
            seed_chirho.publish_chirho(frontend_chirho, iface_chirho, &mut slots_chirho);
        }
        let file_id_chirho = if *index_chirho == 0 {
            root_file_id_chirho
        } else {
            SourceFileChirho::from_source_map_chirho(
                source_map_chirho,
                &source_chirho.path_chirho,
                &source_chirho.source_chirho,
            )
            .file_id_chirho()
        };
        let boot_chirho = graph_chirho.keys_chirho[*index_chirho].boot_chirho;
        let frontend_chirho = seed_chirho
            .check_chirho(&source_chirho.source_chirho, file_id_chirho, boot_chirho)
            .map_err(|error_chirho| {
                if *index_chirho == 0 {
                    SourceGraphErrorChirho::RootChirho(error_chirho)
                } else {
                    SourceGraphErrorChirho::DependencyChirho(format!(
                        "Error checking dependency {} ({}): {error_chirho}",
                        source_chirho.name_chirho,
                        source_chirho.path_chirho.display()
                    ))
                }
            })?;
        if graph_chirho.keys_chirho[*index_chirho].boot_chirho {
            super::boot_chirho::validate_boot_declarations_chirho(&frontend_chirho)?;
        }
        let iface_chirho = if boot_chirho {
            haskelujah_naming_chirho::iface_chirho::build_boot_iface_with_imports_chirho(
                &frontend_chirho.module_chirho,
                &seed_chirho.ifaces_chirho,
            )
        } else {
            build_iface_with_imports_chirho(
                &frontend_chirho.module_chirho,
                &seed_chirho.ifaces_chirho,
            )
        };
        ifaces_chirho.insert(*index_chirho, iface_chirho);
        checked_chirho.insert(*index_chirho, frontend_chirho);
    }
    // Compatibility is checked only after both sides have independently passed;
    // boot signatures never become assumptions while checking their definition.
    for (index_chirho, key_chirho) in graph_chirho
        .keys_chirho
        .iter()
        .enumerate()
        .filter(|(_, key_chirho)| key_chirho.boot_chirho)
    {
        let implementation_key_chirho = ModuleSourceKeyChirho {
            name_chirho: key_chirho.name_chirho.clone(),
            boot_chirho: false,
        };
        let implementation_chirho = graph_chirho
            .indices_chirho
            .get(&implementation_key_chirho)
            .ok_or("boot contract has no implementation source")?;
        super::boot_chirho::check_boot_agreement_chirho(
            &checked_chirho[&index_chirho],
            &checked_chirho[implementation_chirho],
            &ifaces_chirho[&index_chirho],
            &ifaces_chirho[implementation_chirho],
        )?;
    }
    checked_chirho.remove(&0).ok_or_else(|| {
        SourceGraphErrorChirho::from("root module did not complete frontend checking")
    })
}
