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

use std::collections::HashSet;
use std::sync::OnceLock;

use haskelujah_ast_chirho::ModuleChirho;
use haskelujah_core_chirho::expr_chirho::CoreIdChirho;
use haskelujah_core_chirho::{
    CoreModuleChirho, ImportSuppliersChirho, module_free_reference_ids_chirho,
};
use haskelujah_naming_chirho::env_chirho::NamespaceChirho;
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

/// The value names `module_chirho`'s imports supply, split by the origin of the interface
/// they come from: (source modules, builtin interfaces). Qualified imports count, since a
/// qualified reference reaches Core by its bare name.
fn imported_names_by_origin_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> (HashSet<String>, HashSet<String>) {
    let mut source_chirho = HashSet::new();
    let mut builtin_chirho = HashSet::new();
    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        else {
            continue;
        };
        let supplied_chirho =
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho)
                .into_iter()
                .map(|(name_chirho, _, _)| bare_name_chirho(&name_chirho));
        match iface_chirho.exports_chirho.origin_chirho {
            IfaceOriginChirho::SourceChirho => source_chirho.extend(supplied_chirho),
            IfaceOriginChirho::BuiltinChirho => builtin_chirho.extend(supplied_chirho),
        }
    }
    (source_chirho, builtin_chirho)
}

/// Which modules of the program being built supply each value name `module_chirho` imports, by
/// the qualifier a reference writes (row 629). Only `program_modules_chirho` count: a dependency
/// package or a stdlib module brings an interface but no Core to link. A class method or a
/// constructor is left out, since those link through their class and type.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
pub(crate) fn import_suppliers_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    program_modules_chirho: &HashSet<String>,
) -> ImportSuppliersChirho {
    let mut suppliers_chirho = ImportSuppliersChirho::default();
    for import_chirho in &module_chirho.imports_chirho {
        let supplier_chirho = import_chirho.module_chirho.full_name_chirho();
        if !program_modules_chirho.contains(&supplier_chirho) {
            continue;
        }
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == supplier_chirho)
        else {
            continue;
        };
        if iface_chirho.exports_chirho.origin_chirho != IfaceOriginChirho::SourceChirho {
            continue;
        }
        let class_methods_chirho: HashSet<&str> = iface_chirho
            .exports_chirho
            .types_chirho
            .values()
            .filter(|type_chirho| type_chirho.constructors_chirho.is_empty())
            .flat_map(|type_chirho| type_chirho.methods_chirho.iter().map(String::as_str))
            .collect();
        let qualifier_chirho = import_chirho.alias_chirho.as_ref().map_or_else(
            || supplier_chirho.clone(),
            |alias_chirho| alias_chirho.full_name_chirho(),
        );
        for (name_chirho, namespace_chirho, _) in
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho)
        {
            let bare_chirho = bare_name_chirho(&name_chirho);
            if namespace_chirho != NamespaceChirho::ValueChirho
                || class_methods_chirho.contains(bare_chirho.as_str())
                || bare_chirho
                    .chars()
                    .next()
                    .is_some_and(|first_chirho| first_chirho.is_uppercase() || first_chirho == ':')
            {
                continue;
            }
            suppliers_chirho.add_chirho(Some(&qualifier_chirho), &bare_chirho, &supplier_chirho);
            if !import_chirho.qualified_chirho {
                suppliers_chirho.add_chirho(None, &bare_chirho, &supplier_chirho);
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
) {
    let (source_names_chirho, builtin_names_chirho) =
        imported_names_by_origin_chirho(module_chirho, ifaces_chirho);
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
    let builtin_references_chirho: Vec<CoreIdChirho> =
        module_free_reference_ids_chirho(core_chirho)
            .into_iter()
            .filter(|id_chirho| {
                core_chirho
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
