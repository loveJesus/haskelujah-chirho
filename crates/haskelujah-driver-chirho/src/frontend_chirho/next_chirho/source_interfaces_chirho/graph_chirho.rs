// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_parser_chirho::lower_chirho::LoweredModuleChirho;
use std::path::{Path, PathBuf};

pub(super) const MAX_MODULES_CHIRHO: usize = 64;
const MAX_SOURCE_BYTES_CHIRHO: usize = 16 << 20;
pub(super) type SourcesChirho = BTreeMap<String, LoweredModuleChirho>;

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
        let parsed_chirho = super::super::parse_chirho::parse_chirho(
            source_chirho,
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
        )?;
        let name_chirho = parsed_chirho.module_chirho.name_chirho.full_name_chirho();
        if result_chirho.insert(name_chirho, parsed_chirho).is_some() {
            return Err(invalid_chirho("duplicate defining source module"));
        }
    }
    Ok(result_chirho)
}

pub(super) fn order_chirho(
    sources_chirho: &SourcesChirho,
    entry_chirho: &str,
) -> Result<Vec<String>, NextErrorChirho> {
    fn visit_chirho(
        name_chirho: &str,
        sources_chirho: &SourcesChirho,
        visiting_chirho: &mut std::collections::BTreeSet<String>,
        done_chirho: &mut std::collections::BTreeSet<String>,
        order_chirho: &mut Vec<String>,
    ) -> Result<(), NextErrorChirho> {
        if done_chirho.contains(name_chirho) {
            return Ok(());
        }
        if !visiting_chirho.insert(name_chirho.into()) {
            return Err(unsupported_chirho(
                "cyclic source interfaces require boot/recursive-module staging",
            ));
        }
        let source_chirho = sources_chirho
            .get(name_chirho)
            .ok_or_else(|| unsupported_chirho("source import has no supplied checked producer"))?;
        for import_chirho in &source_chirho.module_chirho.imports_chirho {
            let dependency_chirho = import_chirho.module_chirho.full_name_chirho();
            if dependency_chirho == "Prelude" && !sources_chirho.contains_key(&dependency_chirho) {
                continue;
            }
            visit_chirho(
                &dependency_chirho,
                sources_chirho,
                visiting_chirho,
                done_chirho,
                order_chirho,
            )?;
        }
        visiting_chirho.remove(name_chirho);
        done_chirho.insert(name_chirho.into());
        order_chirho.push(name_chirho.into());
        Ok(())
    }
    if !sources_chirho.contains_key(entry_chirho) {
        return Err(unsupported_chirho("entry source module is not supplied"));
    }
    let mut result_chirho = Vec::new();
    visit_chirho(
        entry_chirho,
        sources_chirho,
        &mut Default::default(),
        &mut Default::default(),
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
        if expected_chirho
            .as_ref()
            .is_some_and(|name_chirho| result_chirho.contains_key(name_chirho))
        {
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
        let parsed_chirho = super::super::parse_chirho::parse_chirho(
            &source_chirho,
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
        )?;
        let name_chirho = parsed_chirho.module_chirho.name_chirho.full_name_chirho();
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
            if module_chirho == "Prelude" && !candidate_chirho.exists() {
                continue;
            }
            if !result_chirho.contains_key(&module_chirho)
                && queued_chirho.insert(module_chirho.clone())
            {
                if queued_chirho.len() > MAX_MODULES_CHIRHO {
                    return Err(resource_chirho("source graph module ceiling exhausted"));
                }
                pending_chirho.push((candidate_chirho, Some(module_chirho)));
            }
        }
        if result_chirho.insert(name_chirho, parsed_chirho).is_some() {
            return Err(invalid_chirho("duplicate source module"));
        }
    }
    Ok((
        result_chirho,
        entry_chirho.ok_or_else(|| invalid_chirho("source graph has no entry"))?,
    ))
}
