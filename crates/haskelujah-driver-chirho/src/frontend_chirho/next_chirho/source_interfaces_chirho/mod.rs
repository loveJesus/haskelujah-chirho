// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Separate checked arenas, closed interfaces and original-name Core linking.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho
use super::{CoreModuleChirho, NextErrorChirho, NextErrorKindChirho};
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_hir_chirho::OriginalNameChirho;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};
mod graph_chirho;
mod link_chirho;
mod rename_chirho;
#[cfg(test)]
mod tests_chirho;

fn invalid_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::InvalidChirho, message_chirho)
}
fn resource_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::ResourceChirho, message_chirho)
}
fn unsupported_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}

struct ObjectChirho {
    core_chirho: CoreModuleChirho,
    definitions_chirho: BTreeMap<CoreIdChirho, OriginalNameChirho>,
    imports_chirho: BTreeMap<CoreIdChirho, OriginalNameChirho>,
}

struct ProgramChirho {
    entry_chirho: haskelujah_tc_chirho::CheckedModuleChirho,
    core_chirho: Option<CoreModuleChirho>,
}

fn program_chirho(
    mut sources_chirho: graph_chirho::SourcesChirho,
    entry_chirho: &str,
    emit_chirho: bool,
) -> Result<ProgramChirho, NextErrorChirho> {
    let order_chirho = graph_chirho::order_chirho(&mut sources_chirho, entry_chirho)?;
    let home_keys_chirho: BTreeSet<_> = sources_chirho.keys().cloned().collect();
    let mut interfaces_chirho = BTreeMap::new();
    let mut objects_chirho = Vec::new();
    let mut output_left_chirho = 1 << 20;
    let mut entry_checked_chirho = None;
    for key_chirho in order_chirho {
        if key_chirho.unit_chirho == haskelujah_hir_chirho::UnitIdentityChirho::CompilerChirho
            && key_chirho.module_chirho == super::library_chirho::INTEGER_MODULE_CHIRHO
        {
            interfaces_chirho.insert(
                key_chirho,
                Arc::new(super::library_chirho::integer_interface_chirho()),
            );
            continue;
        }
        let mut selected_chirho = BTreeMap::new();
        let input_chirho = sources_chirho
            .get(&key_chirho)
            .ok_or_else(|| invalid_chirho("source graph lost a producer"))?;
        for import_chirho in &input_chirho.parsed_chirho.module_chirho.imports_chirho {
            let name_chirho = import_chirho.module_chirho.full_name_chirho();
            let home_key_chirho = graph_chirho::ModuleKeyChirho::home_chirho(&name_chirho);
            let dependency_chirho = if key_chirho.unit_chirho
                == haskelujah_hir_chirho::UnitIdentityChirho::StandaloneHomeChirho
                && home_keys_chirho.contains(&home_key_chirho)
            {
                Some(home_key_chirho)
            } else {
                graph_chirho::dependency_chirho(&sources_chirho, &key_chirho, &name_chirho)
            };
            if let Some(dependency_chirho) = dependency_chirho {
                let interface_chirho =
                    interfaces_chirho.get(&dependency_chirho).ok_or_else(|| {
                        invalid_chirho("source dependency was not checked before its consumer")
                    })?;
                selected_chirho.insert(name_chirho, Arc::clone(interface_chirho));
            }
        }
        let mut policy_chirho = haskelujah_hir_chirho::LoweringPolicyChirho {
            completeness_chirho: input_chirho.completeness_chirho,
            prelude_exports_chirho: Some(super::library_chirho::prelude_exports_chirho()?),
            ..Default::default()
        };
        if input_chirho.numeric_chirho
            && !(key_chirho.unit_chirho
                == haskelujah_hir_chirho::UnitIdentityChirho::CompilerChirho
                && key_chirho.module_chirho == super::library_chirho::NUM_MODULE_CHIRHO)
        {
            let numeric_key_chirho = graph_chirho::ModuleKeyChirho {
                unit_chirho: haskelujah_hir_chirho::UnitIdentityChirho::CompilerChirho,
                module_chirho: super::library_chirho::NUM_MODULE_CHIRHO.into(),
            };
            policy_chirho.implicit_interfaces_chirho.push(Arc::clone(
                interfaces_chirho
                    .get(&numeric_key_chirho)
                    .ok_or_else(|| invalid_chirho("known Num dependency was not checked"))?,
            ));
        }
        let input_chirho = sources_chirho
            .remove(&key_chirho)
            .ok_or_else(|| invalid_chirho("source graph lost a producer"))?;
        let checked_chirho = super::check_parsed_with_policy_chirho(
            input_chirho.parsed_chirho,
            key_chirho.unit_chirho.clone(),
            &selected_chirho,
            &policy_chirho,
        )
        .map_err(|error_chirho| {
            if key_chirho == graph_chirho::ModuleKeyChirho::home_chirho(entry_chirho)
                || error_chirho.kind_chirho == NextErrorKindChirho::ResourceChirho
            {
                error_chirho
            } else {
                unsupported_chirho(&format!(
                    "producer {:?} publishes no checked interface: {error_chirho}",
                    key_chirho
                ))
            }
        })?;
        let interface_chirho = haskelujah_tc_chirho::project_interface_chirho(
            &checked_chirho,
            &mut output_left_chirho,
        )?;
        if emit_chirho {
            let core_chirho = super::core_chirho::elaborate_chirho(&checked_chirho)?;
            let mut definitions_chirho = BTreeMap::new();
            for binding_chirho in &core_chirho.bindings_chirho {
                let info_chirho = checked_chirho
                    .module_chirho
                    .names_chirho
                    .get(binding_chirho.binder_chirho.id_chirho.0 as usize)
                    .ok_or_else(|| invalid_chirho("Core global has no producing original name"))?;
                definitions_chirho.insert(
                    binding_chirho.binder_chirho.id_chirho,
                    info_chirho.original_chirho(),
                );
            }
            let mut imports_chirho = BTreeMap::new();
            for imported_chirho in &checked_chirho.module_chirho.imports_chirho {
                for (original_chirho, id_chirho) in &imported_chirho.names_chirho {
                    imports_chirho.insert(CoreIdChirho(id_chirho.0), original_chirho.clone());
                }
            }
            objects_chirho.push(ObjectChirho {
                core_chirho,
                definitions_chirho,
                imports_chirho,
            });
        }
        // The producer's arena and solver drop here. Only original names and closed data remain.
        interfaces_chirho.insert(key_chirho.clone(), Arc::new(interface_chirho));
        if key_chirho == graph_chirho::ModuleKeyChirho::home_chirho(entry_chirho) {
            entry_checked_chirho = Some(checked_chirho);
        }
    }
    Ok(ProgramChirho {
        entry_chirho: entry_checked_chirho
            .ok_or_else(|| invalid_chirho("checked graph has no entry"))?,
        core_chirho: if emit_chirho {
            Some(link_chirho::link_chirho(objects_chirho, entry_chirho)?)
        } else {
            None
        },
    })
}

pub fn check_sources_chirho(
    sources_chirho: &[&str],
    entry_module_chirho: &str,
) -> Result<(), NextErrorChirho> {
    program_chirho(
        graph_chirho::parse_sources_chirho(sources_chirho)?,
        entry_module_chirho,
        false,
    )
    .map(|_| ())
}
pub fn compile_sources_chirho(
    sources_chirho: &[&str],
    entry_module_chirho: &str,
) -> Result<CoreModuleChirho, NextErrorChirho> {
    program_chirho(
        graph_chirho::parse_sources_chirho(sources_chirho)?,
        entry_module_chirho,
        true,
    )?
    .core_chirho
    .ok_or_else(|| invalid_chirho("source compilation produced no Core"))
}
pub fn eval_sources_binding_chirho(
    sources_chirho: &[&str],
    entry_module_chirho: &str,
    entry_binding_chirho: &str,
    step_limit_chirho: u64,
) -> Result<
    (
        haskelujah_runtime_chirho::ValueChirho,
        haskelujah_runtime_chirho::MachineChirho,
    ),
    NextErrorChirho,
> {
    super::eval_core_binding_chirho(
        compile_sources_chirho(sources_chirho, entry_module_chirho)?,
        entry_binding_chirho,
        step_limit_chirho,
    )
}
pub fn check_source_file_chirho(path_chirho: &Path) -> Result<(), NextErrorChirho> {
    let (sources_chirho, entry_chirho) = graph_chirho::load_chirho(path_chirho)?;
    program_chirho(sources_chirho, &entry_chirho, false).map(|_| ())
}
pub fn compile_source_file_chirho(path_chirho: &Path) -> Result<CoreModuleChirho, NextErrorChirho> {
    let (sources_chirho, entry_chirho) = graph_chirho::load_chirho(path_chirho)?;
    program_chirho(sources_chirho, &entry_chirho, true)?
        .core_chirho
        .ok_or_else(|| invalid_chirho("source compilation produced no Core"))
}

pub(super) fn single_chirho(
    source_chirho: &str,
    emit_chirho: bool,
) -> Result<
    (
        haskelujah_tc_chirho::CheckedModuleChirho,
        Option<CoreModuleChirho>,
    ),
    NextErrorChirho,
> {
    let sources_chirho = graph_chirho::parse_sources_chirho(&[source_chirho])?;
    let entry_chirho = sources_chirho
        .keys()
        .next()
        .ok_or_else(|| invalid_chirho("missing source"))?
        .module_chirho
        .clone();
    let result_chirho = program_chirho(sources_chirho, &entry_chirho, emit_chirho)?;
    Ok((result_chirho.entry_chirho, result_chirho.core_chirho))
}
