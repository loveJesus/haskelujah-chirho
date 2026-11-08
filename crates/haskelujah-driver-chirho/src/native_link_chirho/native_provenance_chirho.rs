// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The native provenance the driver records for each compiled module (row 628).
//!
//! The dictionary pass marks the bindings it generates. Here the driver adds the rest:
//! - the module's entry candidate, its own top-level `main` (the program keeps only the root
//!   module's);
//! - its builtin references: the references no binding of the module owns whose names its
//!   builtin interfaces (or the builtin environment) supply, and no source module does.
//!
//! A free reference that is neither is a program binding the module did not receive, and
//! the native backends fail closed on it.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use haskelujah_ast_chirho::ModuleChirho;
use haskelujah_core_chirho::expr_chirho::CoreIdChirho;
use haskelujah_core_chirho::{
    CoreModuleChirho, ImportSuppliersChirho, module_free_reference_ids_chirho,
};
use haskelujah_naming_chirho::env_chirho::NamespaceChirho;
use haskelujah_naming_chirho::iface_chirho::IfaceValueChirho;
use haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho;
use haskelujah_naming_chirho::{IfaceOriginChirho, ModuleIfaceChirho};

/// The bare names the builtin type environment declares.
fn builtin_registry_chirho() -> &'static HashSet<String> {
    static REGISTRY_CHIRHO: OnceLock<HashSet<String>> = OnceLock::new();
    REGISTRY_CHIRHO.get_or_init(|| {
        haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho()
            .into_keys()
            .map(|name_chirho| bare_name_chirho(&name_chirho))
            .collect()
    })
}

/// A value name without operator parentheses: `(+)` and `+` are one name.
fn bare_name_chirho(name_chirho: &str) -> String {
    if name_chirho.len() > 2
        && name_chirho.starts_with('(')
        && name_chirho.ends_with(')')
        && !name_chirho.contains(',')
    {
        name_chirho[1..name_chirho.len() - 1].to_string()
    } else {
        name_chirho.to_string()
    }
}

/// How an imported value links, when one of the program's own modules supplies it (row 642).
enum ImportedValueRouteChirho {
    /// A program module defines it, so a reference links by identity to that module's binding.
    DefinedByChirho(String),
    /// A program module forwards a value no program module defines, such as a builtin it
    /// re-exports, so a reference takes the builtin route.
    ForwardedBuiltinChirho,
}

/// The values a source interface exports, by bare name, to read their origins.
fn values_by_bare_name_chirho(
    iface_chirho: &ModuleIfaceChirho,
) -> HashMap<String, &IfaceValueChirho> {
    iface_chirho
        .exports_chirho
        .values_chirho
        .values()
        .map(|value_chirho| (bare_name_chirho(&value_chirho.name_chirho), value_chirho))
        .collect()
}

/// The class methods a source interface exports, which link through their class (row 629).
fn class_methods_chirho(iface_chirho: &ModuleIfaceChirho) -> HashSet<&str> {
    iface_chirho
        .exports_chirho
        .types_chirho
        .values()
        .filter(|type_chirho| type_chirho.constructors_chirho.is_empty())
        .flat_map(|type_chirho| type_chirho.methods_chirho.iter().map(String::as_str))
        .collect()
}

/// How a name a program module's interface supplies links (rows 629 and 642), or `None` when it
/// keeps row 628's source classification: a type, a class method, a constructor, or a value
/// whose definer the program does not compile. The origin is the value's own: a re-export never
/// changes it, so `A.f`, forwarding B's `f`, is defined by B.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
fn imported_value_route_chirho(
    name_chirho: &str,
    namespace_chirho: NamespaceChirho,
    class_methods_chirho: &HashSet<&str>,
    values_chirho: &HashMap<String, &IfaceValueChirho>,
    program_modules_chirho: &HashSet<String>,
) -> Option<ImportedValueRouteChirho> {
    let bare_chirho = bare_name_chirho(name_chirho);
    if namespace_chirho != NamespaceChirho::ValueChirho
        || class_methods_chirho.contains(bare_chirho.as_str())
        || bare_chirho
            .chars()
            .next()
            .is_some_and(|first_chirho| first_chirho.is_uppercase() || first_chirho == ':')
    {
        return None;
    }
    match &values_chirho.get(&bare_chirho)?.defining_module_chirho {
        Some(definer_chirho) if program_modules_chirho.contains(definer_chirho) => Some(
            ImportedValueRouteChirho::DefinedByChirho(definer_chirho.clone()),
        ),
        Some(_) => None,
        None => Some(ImportedValueRouteChirho::ForwardedBuiltinChirho),
    }
}

/// The value names `module_chirho`'s imports supply, split by origin: (source, builtin).
/// Qualified imports count, since a qualified reference reaches Core by its bare name. A name a
/// program module defines links by identity, so it blocks neither set (row 642); a builtin a
/// program module forwards counts as builtin.
fn imported_names_by_origin_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    program_modules_chirho: &HashSet<String>,
) -> (HashSet<String>, HashSet<String>) {
    let mut source_chirho = HashSet::new();
    let mut builtin_chirho = HashSet::new();
    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = haskelujah_naming_chirho::iface_chirho::select_import_iface_chirho(
            ifaces_chirho,
            import_chirho,
        ) else {
            continue;
        };
        let supplied_chirho =
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho);
        if iface_chirho.exports_chirho.origin_chirho == IfaceOriginChirho::BuiltinChirho {
            builtin_chirho.extend(
                supplied_chirho
                    .into_iter()
                    .map(|(name_chirho, _, _)| bare_name_chirho(&name_chirho)),
            );
            continue;
        }
        let program_iface_chirho = program_modules_chirho.contains(&module_name_chirho);
        let class_methods_chirho = class_methods_chirho(iface_chirho);
        let values_chirho = values_by_bare_name_chirho(iface_chirho);
        for (name_chirho, namespace_chirho, _) in supplied_chirho {
            let route_chirho = program_iface_chirho
                .then(|| {
                    imported_value_route_chirho(
                        &name_chirho,
                        namespace_chirho,
                        &class_methods_chirho,
                        &values_chirho,
                        program_modules_chirho,
                    )
                })
                .flatten();
            match route_chirho {
                Some(ImportedValueRouteChirho::DefinedByChirho(_)) => {}
                Some(ImportedValueRouteChirho::ForwardedBuiltinChirho) => {
                    builtin_chirho.insert(bare_name_chirho(&name_chirho));
                }
                None => {
                    source_chirho.insert(bare_name_chirho(&name_chirho));
                }
            }
        }
    }
    (source_chirho, builtin_chirho)
}

/// Which of the program's own modules define each value name `module_chirho` imports, by the
/// qualifier a reference writes (rows 629 and 642). The definer is the value's origin, never the
/// module it is imported through, so several paths to one definition are one identity. A value
/// no program module defines (a builtin, a dependency's) is left out and keeps its own route, as
/// is a class method or a constructor, which link through their class and type.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
pub(crate) fn import_suppliers_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    program_modules_chirho: &HashSet<String>,
) -> ImportSuppliersChirho {
    let mut suppliers_chirho = ImportSuppliersChirho::default();
    for import_chirho in &module_chirho.imports_chirho {
        let supplier_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = haskelujah_naming_chirho::iface_chirho::select_import_iface_chirho(
            ifaces_chirho,
            import_chirho,
        ) else {
            continue;
        };
        let class_methods_chirho = class_methods_chirho(iface_chirho);
        let values_chirho = values_by_bare_name_chirho(iface_chirho);
        let qualifier_chirho = import_chirho.alias_chirho.as_ref().map_or_else(
            || supplier_chirho.clone(),
            |alias_chirho| alias_chirho.full_name_chirho(),
        );
        for (name_chirho, namespace_chirho, _) in
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho)
        {
            let bare_chirho = bare_name_chirho(&name_chirho);
            let selected_value_chirho = values_chirho.get(&bare_chirho);
            if namespace_chirho == NamespaceChirho::ValueChirho
                && let Some(value_chirho) = selected_value_chirho
                && value_chirho.defining_module_chirho.is_none()
                && value_chirho.availability_chirho
                    == haskelujah_naming_chirho::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho
                && let Some(compiler_chirho) = value_chirho.compiler_value_chirho
            {
                suppliers_chirho.add_compiler_chirho(
                    Some(&qualifier_chirho),
                    &bare_chirho,
                    compiler_chirho,
                );
                if !import_chirho.qualified_chirho {
                    suppliers_chirho.add_compiler_chirho(None, &bare_chirho, compiler_chirho);
                }
                continue;
            }
            if !program_modules_chirho.contains(&supplier_chirho)
                || iface_chirho.exports_chirho.origin_chirho != IfaceOriginChirho::SourceChirho
            {
                if namespace_chirho == NamespaceChirho::ValueChirho {
                    suppliers_chirho.add_unproven_chirho(Some(&qualifier_chirho), &bare_chirho);
                    if !import_chirho.qualified_chirho {
                        suppliers_chirho.add_unproven_chirho(None, &bare_chirho);
                    }
                }
                continue;
            }
            let Some(ImportedValueRouteChirho::DefinedByChirho(definer_chirho)) =
                imported_value_route_chirho(
                    &name_chirho,
                    namespace_chirho,
                    &class_methods_chirho,
                    &values_chirho,
                    program_modules_chirho,
                )
            else {
                if namespace_chirho == NamespaceChirho::ValueChirho {
                    suppliers_chirho.add_unproven_chirho(Some(&qualifier_chirho), &bare_chirho);
                    if !import_chirho.qualified_chirho {
                        suppliers_chirho.add_unproven_chirho(None, &bare_chirho);
                    }
                }
                continue;
            };
            suppliers_chirho.add_chirho(Some(&qualifier_chirho), &bare_chirho, &definer_chirho);
            if !import_chirho.qualified_chirho {
                suppliers_chirho.add_chirho(None, &bare_chirho, &definer_chirho);
            }
        }
    }
    suppliers_chirho
}

/// Record `core_chirho`'s entry candidate and builtin references, from `module_chirho`'s
/// imports resolved against `ifaces_chirho`.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
pub(crate) fn record_module_provenance_chirho(
    module_chirho: &ModuleChirho,
    core_chirho: &mut CoreModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    program_modules_chirho: &HashSet<String>,
) {
    let (source_names_chirho, builtin_names_chirho) =
        imported_names_by_origin_chirho(module_chirho, ifaces_chirho, program_modules_chirho);
    let registry_chirho = builtin_registry_chirho();
    let compiler_chirho = &core_chirho.provenance_chirho.compiler_bindings_chirho;
    let entry_chirho = core_chirho
        .bindings_chirho
        .iter()
        .map(|binding_chirho| &binding_chirho.binder_chirho)
        .find(|binder_chirho| {
            binder_chirho.name_chirho == "main"
                && !compiler_chirho.contains(&binder_chirho.id_chirho)
        })
        .map(|binder_chirho| binder_chirho.id_chirho);
    // A reference with a user identity links by that identity alone, never as a builtin, even
    // when a forwarded builtin shares its spelling (row 642).
    let imported_chirho = &core_chirho.provenance_chirho.imported_references_chirho;
    let builtin_references_chirho: Vec<CoreIdChirho> =
        module_free_reference_ids_chirho(core_chirho)
            .into_iter()
            .filter(|id_chirho| {
                !imported_chirho.contains_key(id_chirho)
                    && core_chirho
                        .names_chirho
                        .get(id_chirho)
                        .is_some_and(|name_chirho| {
                            let bare_chirho = bare_name_chirho(name_chirho);
                            !source_names_chirho.contains(&bare_chirho)
                                && (builtin_names_chirho.contains(&bare_chirho)
                                    || registry_chirho.contains(&bare_chirho))
                        })
            })
            .collect();
    let provenance_chirho = &mut core_chirho.provenance_chirho;
    provenance_chirho.entry_chirho = entry_chirho;
    provenance_chirho
        .builtin_references_chirho
        .extend(builtin_references_chirho);
}

#[cfg(test)]
#[path = "native_provenance_tests_chirho.rs"]
mod tests_chirho;
