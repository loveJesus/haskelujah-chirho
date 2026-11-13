// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_hir_chirho::{CompletenessChirho, UnitIdentityChirho};
use haskelujah_parser_chirho::lower_chirho::LoweredModuleChirho;
use std::path::{Path, PathBuf};

pub(super) const MAX_MODULES_CHIRHO: usize = 64;
const MAX_SOURCE_BYTES_CHIRHO: usize = 16 << 20;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ModuleKeyChirho {
    pub unit_chirho: UnitIdentityChirho,
    pub module_chirho: String,
}
impl ModuleKeyChirho {
    pub(super) fn home_chirho(module_chirho: &str) -> Self {
        Self {
            unit_chirho: UnitIdentityChirho::StandaloneHomeChirho,
            module_chirho: module_chirho.into(),
        }
    }
    fn library_chirho(module_chirho: &str) -> Self {
        Self {
            unit_chirho: UnitIdentityChirho::CompilerChirho,
            module_chirho: module_chirho.into(),
        }
    }
}
pub(super) struct SourceInputChirho {
    pub parsed_chirho: LoweredModuleChirho,
    pub numeric_chirho: bool,
    pub completeness_chirho: CompletenessChirho,
}
pub(super) type SourcesChirho = BTreeMap<ModuleKeyChirho, SourceInputChirho>;

pub(super) fn parse_sources_chirho(
    sources_chirho: &[&str],
) -> Result<SourcesChirho, NextErrorChirho> {
    if sources_chirho.len() > MAX_MODULES_CHIRHO {
        return Err(resource_chirho("source graph module ceiling exhausted"));
    }
    let mut result_chirho = BTreeMap::new();
    let mut left_chirho = MAX_SOURCE_BYTES_CHIRHO;
    for source_chirho in sources_chirho {
        left_chirho = left_chirho
            .checked_sub(source_chirho.len())
            .ok_or_else(|| resource_chirho("source graph byte ceiling exhausted"))?;
        let (parsed_chirho, numeric_chirho) =
            super::super::parse_chirho::parse_with_dependencies_chirho(
                source_chirho,
                haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
            )?;
        let key_chirho = ModuleKeyChirho::home_chirho(
            &parsed_chirho.module_chirho.name_chirho.full_name_chirho(),
        );
        if result_chirho
            .insert(
                key_chirho,
                SourceInputChirho {
                    parsed_chirho,
                    numeric_chirho,
                    completeness_chirho: CompletenessChirho::CompleteChirho,
                },
            )
            .is_some()
        {
            return Err(invalid_chirho("duplicate defining source module"));
        }
    }
    Ok(result_chirho)
}

/// Explicit imports select supplied home source first; known syntax always selects its defining unit.
pub(super) fn dependency_chirho(
    sources_chirho: &SourcesChirho,
    owner_chirho: &ModuleKeyChirho,
    module_chirho: &str,
) -> Option<ModuleKeyChirho> {
    if owner_chirho.unit_chirho == UnitIdentityChirho::StandaloneHomeChirho {
        let home_chirho = ModuleKeyChirho::home_chirho(module_chirho);
        if sources_chirho.contains_key(&home_chirho) {
            return Some(home_chirho);
        }
    }
    if super::super::library_chirho::manifest_chirho(module_chirho).is_some()
        || module_chirho == super::super::library_chirho::INTEGER_MODULE_CHIRHO
    {
        Some(ModuleKeyChirho::library_chirho(module_chirho))
    } else if module_chirho == "Prelude"
        && owner_chirho.unit_chirho == UnitIdentityChirho::StandaloneHomeChirho
    {
        None
    } else if owner_chirho.unit_chirho == UnitIdentityChirho::CompilerChirho {
        Some(ModuleKeyChirho::library_chirho(module_chirho))
    } else {
        Some(ModuleKeyChirho::home_chirho(module_chirho))
    }
}

pub(super) fn order_chirho(
    sources_chirho: &mut SourcesChirho,
    entry_chirho: &str,
) -> Result<Vec<ModuleKeyChirho>, NextErrorChirho> {
    fn visit_chirho(
        key_chirho: ModuleKeyChirho,
        sources_chirho: &mut SourcesChirho,
        visiting_chirho: &mut BTreeSet<ModuleKeyChirho>,
        done_chirho: &mut BTreeSet<ModuleKeyChirho>,
        order_chirho: &mut Vec<ModuleKeyChirho>,
    ) -> Result<(), NextErrorChirho> {
        if done_chirho.contains(&key_chirho) {
            return Ok(());
        }
        if !visiting_chirho.insert(key_chirho.clone()) {
            return Err(unsupported_chirho(
                "cyclic source interfaces require boot/recursive-module staging",
            ));
        }
        if !sources_chirho.contains_key(&key_chirho) {
            if key_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho {
                return Err(unsupported_chirho(
                    "source import has no supplied checked producer",
                ));
            }
            if sources_chirho.len() >= MAX_MODULES_CHIRHO {
                return Err(resource_chirho("source graph module ceiling exhausted"));
            }
            if key_chirho.module_chirho == super::super::library_chirho::INTEGER_MODULE_CHIRHO {
                visiting_chirho.remove(&key_chirho);
                done_chirho.insert(key_chirho.clone());
                order_chirho.push(key_chirho);
                return Ok(());
            }
            let manifest_chirho =
                super::super::library_chirho::manifest_chirho(&key_chirho.module_chirho)
                    .ok_or_else(|| unsupported_chirho("installed unit has no source manifest"))?;
            let source_chirho = manifest_chirho.source_chirho()?;
            let (parsed_chirho, numeric_chirho) =
                super::super::parse_chirho::parse_with_dependencies_chirho(
                    &source_chirho,
                    haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
                )?;
            sources_chirho.insert(
                key_chirho.clone(),
                SourceInputChirho {
                    parsed_chirho,
                    numeric_chirho,
                    completeness_chirho: CompletenessChirho::SliceChirho,
                },
            );
        }
        let source_chirho = &sources_chirho[&key_chirho];
        if key_chirho.unit_chirho == UnitIdentityChirho::StandaloneHomeChirho
            && key_chirho.module_chirho != "Prelude"
            && implicit_prelude_chirho(&source_chirho.parsed_chirho)
            && sources_chirho.contains_key(&ModuleKeyChirho::home_chirho("Prelude"))
        {
            return Err(unsupported_chirho(
                "a home Prelude needs checked implicit-import selection; wired names cannot replace it",
            ));
        }
        let mut dependencies_chirho = source_chirho
            .parsed_chirho
            .module_chirho
            .imports_chirho
            .iter()
            .filter_map(|import_chirho| {
                dependency_chirho(
                    sources_chirho,
                    &key_chirho,
                    &import_chirho.module_chirho.full_name_chirho(),
                )
            })
            .collect::<BTreeSet<_>>();
        if source_chirho.numeric_chirho
            && !(key_chirho.unit_chirho == UnitIdentityChirho::CompilerChirho
                && key_chirho.module_chirho == super::super::library_chirho::NUM_MODULE_CHIRHO)
        {
            dependencies_chirho.insert(ModuleKeyChirho::library_chirho(
                super::super::library_chirho::NUM_MODULE_CHIRHO,
            ));
        }
        for dependency_chirho in dependencies_chirho {
            visit_chirho(
                dependency_chirho,
                sources_chirho,
                visiting_chirho,
                done_chirho,
                order_chirho,
            )?;
        }
        visiting_chirho.remove(&key_chirho);
        done_chirho.insert(key_chirho.clone());
        order_chirho.push(key_chirho);
        Ok(())
    }
    let entry_chirho = ModuleKeyChirho::home_chirho(entry_chirho);
    if !sources_chirho.contains_key(&entry_chirho) {
        return Err(unsupported_chirho("entry source module is not supplied"));
    }
    let mut result_chirho = Vec::new();
    visit_chirho(
        entry_chirho,
        sources_chirho,
        &mut BTreeSet::new(),
        &mut BTreeSet::new(),
        &mut result_chirho,
    )?;
    Ok(result_chirho)
}

/// Read only the entry's source dependency closure. No package iface or filesystem path mints a unit.
pub(super) fn load_chirho(path_chirho: &Path) -> Result<(SourcesChirho, String), NextErrorChirho> {
    let root_chirho = path_chirho.parent().unwrap_or_else(|| Path::new("."));
    let mut pending_chirho = vec![(path_chirho.to_path_buf(), None::<String>)];
    let mut result_chirho = BTreeMap::new();
    let mut left_chirho = MAX_SOURCE_BYTES_CHIRHO;
    let mut entry_chirho = None;
    let mut queued_chirho = std::collections::BTreeSet::new();
    while let Some((path_chirho, expected_chirho)) = pending_chirho.pop() {
        if expected_chirho.as_ref().is_some_and(|name_chirho| {
            result_chirho.contains_key(&ModuleKeyChirho::home_chirho(name_chirho))
        }) {
            continue;
        }
        if result_chirho.len() >= MAX_MODULES_CHIRHO {
            return Err(resource_chirho("source graph module ceiling exhausted"));
        }
        let file_chirho = std::fs::File::open(&path_chirho).map_err(|error_chirho| {
            unsupported_chirho(&format!(
                "cannot read source producer {}: {error_chirho}",
                path_chirho.display()
            ))
        })?;
        use std::io::Read;
        let mut bytes_chirho = Vec::new();
        file_chirho
            .take((left_chirho + 1) as u64)
            .read_to_end(&mut bytes_chirho)
            .map_err(|error_chirho| invalid_chirho(&error_chirho.to_string()))?;
        left_chirho = left_chirho
            .checked_sub(bytes_chirho.len())
            .ok_or_else(|| resource_chirho("source graph byte ceiling exhausted"))?;
        let source_chirho = String::from_utf8(bytes_chirho)
            .map_err(|_| invalid_chirho("source module is not UTF-8"))?;
        let (parsed_chirho, numeric_chirho) =
            super::super::parse_chirho::parse_with_dependencies_chirho(
                &source_chirho,
                haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
            )?;
        let name_chirho = parsed_chirho.module_chirho.name_chirho.full_name_chirho();
        if name_chirho != "Prelude"
            && implicit_prelude_chirho(&parsed_chirho)
            && root_chirho.join("Prelude.hs").exists()
        {
            return Err(unsupported_chirho(
                "a home Prelude needs checked implicit-import selection; wired names cannot replace it",
            ));
        }
        if expected_chirho
            .as_ref()
            .is_some_and(|expected_chirho| *expected_chirho != name_chirho)
        {
            return Err(invalid_chirho(
                "selected source path declares a different module",
            ));
        }
        if entry_chirho.is_none() {
            entry_chirho = Some(name_chirho.clone());
        }
        for import_chirho in &parsed_chirho.module_chirho.imports_chirho {
            let module_chirho = import_chirho.module_chirho.full_name_chirho();
            let mut relative_chirho = PathBuf::new();
            for component_chirho in module_chirho.split('.') {
                if component_chirho.is_empty()
                    || !component_chirho.chars().all(|character_chirho| {
                        character_chirho.is_alphanumeric()
                            || character_chirho == '_'
                            || character_chirho == char::from(39)
                    })
                {
                    return Err(unsupported_chirho("unrepresented source import path"));
                }
                relative_chirho.push(component_chirho);
            }
            relative_chirho.set_extension("hs");
            let candidate_chirho = root_chirho.join(relative_chirho);
            if !candidate_chirho.exists()
                && (module_chirho == "Prelude"
                    || super::super::library_chirho::manifest_chirho(&module_chirho).is_some())
            {
                continue;
            }
            if !result_chirho.contains_key(&ModuleKeyChirho::home_chirho(&module_chirho))
                && queued_chirho.insert(module_chirho.clone())
            {
                if queued_chirho.len() > MAX_MODULES_CHIRHO {
                    return Err(resource_chirho("source graph module ceiling exhausted"));
                }
                pending_chirho.push((candidate_chirho, Some(module_chirho)));
            }
        }
        if result_chirho
            .insert(
                ModuleKeyChirho::home_chirho(&name_chirho),
                SourceInputChirho {
                    parsed_chirho,
                    numeric_chirho,
                    completeness_chirho: CompletenessChirho::CompleteChirho,
                },
            )
            .is_some()
        {
            return Err(invalid_chirho("duplicate source module"));
        }
    }
    Ok((
        result_chirho,
        entry_chirho.ok_or_else(|| invalid_chirho("source graph has no entry"))?,
    ))
}

fn implicit_prelude_chirho(parsed_chirho: &LoweredModuleChirho) -> bool {
    !parsed_chirho
        .module_chirho
        .extensions_chirho
        .iter()
        .any(|flag_chirho| {
            matches!(
                flag_chirho.as_str(),
                "NoImplicitPrelude" | "RebindableSyntax"
            )
        })
}
