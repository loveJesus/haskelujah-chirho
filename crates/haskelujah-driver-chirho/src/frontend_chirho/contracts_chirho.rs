// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Naming selects visible roots; typed dependencies remain internal contracts.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;
use haskelujah_naming_chirho::env_chirho::NamespaceChirho;
use haskelujah_typing_chirho::kind_chirho::{KindChirho, KindContractChirho};
use std::collections::{HashMap, HashSet, VecDeque};

fn builtin_contract_chirho(iface_chirho: &ModuleIfaceChirho) -> Option<ModuleTypeContractsChirho> {
    // The synthetic interface's authored source span distinguishes it from a
    // source module shadowing this name. Checked source companions take priority.
    if !matches!(
        iface_chirho.name_chirho.as_str(),
        "Control.Monad.Identity" | "Data.Functor.Identity"
    ) || iface_chirho
        .exports_chirho
        .types_chirho
        .get("Identity")?
        .span_chirho
        != haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO
    {
        return None;
    }
    let mut contracts_chirho = ModuleTypeContractsChirho::default();
    contracts_chirho.kinds_chirho.insert(
        "Identity".to_owned(),
        KindContractChirho::monomorphic_nominal_chirho(KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho,
        )),
    );
    Some(contracts_chirho)
}

pub(super) fn select_imported_contracts_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    modules_chirho: Option<&ImportedTypeContractsChirho>,
    values_chirho: &HashMap<String, haskelujah_typing_chirho::SchemeChirho>,
) -> ModuleTypeContractsChirho {
    let mut selected_chirho = ModuleTypeContractsChirho::default();
    let safe_chirho =
        collect_safe_unqualified_imported_type_names_chirho(module_chirho, ifaces_chirho);
    let preferred_chirho =
        collect_preferred_qualified_type_names_chirho(module_chirho, ifaces_chirho);
    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        else {
            continue;
        };
        let builtin_chirho = builtin_contract_chirho(iface_chirho);
        let Some(contracts_chirho) = modules_chirho
            .and_then(|modules_chirho| modules_chirho.get(&module_name_chirho))
            .or(builtin_chirho.as_ref())
        else {
            continue;
        };
        let imported_names_chirho =
            haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                &iface_chirho.exports_chirho,
                &import_chirho.spec_chirho,
            );
        let visible_chirho: HashSet<_> = imported_names_chirho
            .iter()
            .filter_map(|(name_chirho, namespace_chirho, _)| {
                (*namespace_chirho == NamespaceChirho::TypeChirho).then_some(name_chirho.clone())
            })
            .collect();
        let mut roots_chirho: Vec<_> = visible_chirho.iter().cloned().collect();
        for (name_chirho, namespace_chirho, _) in &imported_names_chirho {
            if *namespace_chirho != NamespaceChirho::ValueChirho {
                continue;
            }
            if let Some(scheme_chirho) = values_chirho
                .get(&format!("{module_name_chirho}.{name_chirho}"))
                .or_else(|| values_chirho.get(name_chirho))
            {
                scheme_chirho.map_constructor_names_chirho(&mut |name_chirho| {
                    roots_chirho.push(
                        private_contract_name_chirho(name_chirho, iface_chirho, contracts_chirho)
                            .unwrap_or_else(|| name_chirho.to_owned()),
                    );
                    name_chirho.to_owned()
                });
            }
        }
        // Select before cloning: imports pay for the contracts they use, not
        // every export of a large provider or every previously loaded module.
        let reached_chirho =
            reachable_contracts_chirho(contracts_chirho, roots_chirho, &mut str::to_owned);
        let qualifier_chirho = import_chirho
            .alias_chirho
            .as_ref()
            .map(|alias_chirho| alias_chirho.text_chirho().to_owned())
            .unwrap_or_else(|| module_name_chirho.clone());
        let qualifiable_chirho = contract_type_names_chirho(&reached_chirho);
        let mut rename_chirho = |name_chirho: &str| {
            if let Some(private_chirho) =
                private_contract_name_chirho(name_chirho, iface_chirho, contracts_chirho)
            {
                return private_chirho;
            }
            qualify_imported_type_name_chirho(
                name_chirho,
                &qualifiable_chirho,
                &qualifier_chirho,
                &safe_chirho,
                &preferred_chirho,
            )
        };
        let mut names_chirho: Vec<_> = reached_chirho.kinds_chirho.keys().collect();
        names_chirho.sort();
        for name_chirho in names_chirho {
            let contract_chirho = reached_chirho.kinds_chirho[name_chirho]
                .map_constructor_names_chirho(&mut rename_chirho);
            if name_chirho.contains('.') {
                selected_chirho
                    .kinds_chirho
                    .insert(name_chirho.clone(), contract_chirho);
                continue;
            }
            let bare_chirho = name_chirho.as_str();
            for key_chirho in [
                format!("{qualifier_chirho}.{bare_chirho}"),
                format!("{module_name_chirho}.{bare_chirho}"),
            ] {
                selected_chirho
                    .kinds_chirho
                    .insert(key_chirho, contract_chirho.clone());
            }
            if !import_chirho.qualified_chirho && visible_chirho.contains(bare_chirho) {
                selected_chirho
                    .kinds_chirho
                    .insert(bare_chirho.to_owned(), contract_chirho);
            }
        }
        let mut names_chirho: Vec<_> = reached_chirho.synonyms_chirho.keys().collect();
        names_chirho.sort();
        for name_chirho in names_chirho {
            let contract_chirho = reached_chirho.synonyms_chirho[name_chirho]
                .map_constructor_names_chirho(&mut rename_chirho);
            if name_chirho.contains('.') {
                selected_chirho
                    .synonyms_chirho
                    .insert(name_chirho.clone(), contract_chirho);
                continue;
            }
            let bare_chirho = name_chirho.as_str();
            for key_chirho in [
                format!("{qualifier_chirho}.{bare_chirho}"),
                format!("{module_name_chirho}.{bare_chirho}"),
            ] {
                selected_chirho
                    .synonyms_chirho
                    .insert(key_chirho, contract_chirho.clone());
            }
            if !import_chirho.qualified_chirho && visible_chirho.contains(bare_chirho) {
                selected_chirho
                    .synonyms_chirho
                    .insert(bare_chirho.to_owned(), contract_chirho);
            }
        }
    }
    selected_chirho
}

pub(super) fn contract_type_names_chirho(
    contracts_chirho: &ModuleTypeContractsChirho,
) -> HashSet<String> {
    contracts_chirho
        .kinds_chirho
        .keys()
        .chain(contracts_chirho.synonyms_chirho.keys())
        .map(|name_chirho| {
            name_chirho
                .rsplit('.')
                .next()
                .unwrap_or(name_chirho)
                .to_owned()
        })
        .collect()
}

/// A private dependency keeps its defining identity through re-exports. It is
/// never renamed to the importing alias merely because its basename is equal.
pub(super) fn private_contract_name_chirho(
    name_chirho: &str,
    iface_chirho: &ModuleIfaceChirho,
    contracts_chirho: &ModuleTypeContractsChirho,
) -> Option<String> {
    let bare_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
    if iface_chirho
        .exports_chirho
        .types_chirho
        .contains_key(bare_chirho)
    {
        return None;
    }
    let canonical_chirho = if name_chirho.contains('.') {
        name_chirho.to_owned()
    } else {
        format!("{}.{name_chirho}", iface_chirho.name_chirho)
    };
    (contracts_chirho
        .kinds_chirho
        .contains_key(&canonical_chirho)
        || contracts_chirho
            .synonyms_chirho
            .contains_key(&canonical_chirho))
    .then_some(canonical_chirho)
}

/// Retain only contracts reached by actual exports (including value schemes).
/// Each name is visited once; re-exports do not clone the entire dependency fleet.
pub(super) fn export_contracts_chirho(
    module_chirho: &ModuleChirho,
    iface_chirho: &ModuleIfaceChirho,
    scope_chirho: &ModuleTypeContractsChirho,
    inferred_chirho: &InferResultChirho,
) -> ModuleTypeContractsChirho {
    let mut roots_chirho: Vec<String> = iface_chirho
        .exports_chirho
        .types_chirho
        .keys()
        .cloned()
        .collect();
    for name_chirho in iface_chirho.exports_chirho.values_chirho.keys() {
        if let Some(scheme_chirho) = inferred_chirho.env_chirho.lookup_chirho(name_chirho) {
            scheme_chirho.map_constructor_names_chirho(&mut |name_chirho| {
                roots_chirho.push(name_chirho.to_owned());
                name_chirho.to_owned()
            });
        }
    }
    let local_chirho = collect_local_type_names_chirho(module_chirho);
    reachable_contracts_chirho(scope_chirho, roots_chirho, &mut |name_chirho| {
        if local_chirho.contains(name_chirho)
            && !iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key(name_chirho)
        {
            format!("{}.{name_chirho}", iface_chirho.name_chirho)
        } else {
            name_chirho.to_owned()
        }
    })
}

fn reachable_contracts_chirho(
    scope_chirho: &ModuleTypeContractsChirho,
    roots_chirho: Vec<String>,
    rename_chirho: &mut impl FnMut(&str) -> String,
) -> ModuleTypeContractsChirho {
    let mut result_chirho = ModuleTypeContractsChirho::default();
    let mut pending_chirho: VecDeque<_> = roots_chirho.into();
    let mut visited_chirho = HashSet::new();
    while let Some(name_chirho) = pending_chirho.pop_front() {
        if !visited_chirho.insert(name_chirho.clone()) {
            continue;
        }
        let key_chirho = rename_chirho(&name_chirho);
        let mut enqueue_chirho = |dependency_chirho: &str| {
            pending_chirho.push_back(dependency_chirho.to_owned());
            rename_chirho(dependency_chirho)
        };
        if let Some(contract_chirho) = scope_chirho.kinds_chirho.get(&name_chirho) {
            result_chirho.kinds_chirho.insert(
                key_chirho.clone(),
                contract_chirho.map_constructor_names_chirho(&mut enqueue_chirho),
            );
        }
        if let Some(contract_chirho) = scope_chirho.synonyms_chirho.get(&name_chirho) {
            result_chirho.synonyms_chirho.insert(
                key_chirho,
                contract_chirho.map_constructor_names_chirho(&mut enqueue_chirho),
            );
        }
    }
    result_chirho
}
