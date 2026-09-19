// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source ownership controls safe import-name abbreviation.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use crate::{DeclChirho, ModuleChirho, ModuleIfaceChirho};

pub(crate) fn collect_local_type_names_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashSet<String> {
    let mut names_chirho = std::collections::HashSet::new();
    for declaration_chirho in &module_chirho.decls_chirho {
        match declaration_chirho {
            DeclChirho::ClassDeclChirho {
                name_chirho,
                associated_tfs_chirho,
                ..
            } => {
                names_chirho.insert(name_chirho.text_chirho().to_string());
                // Associated heads are declarations in the type namespace,
                // not imports just because they are nested in a class body.
                for family_chirho in associated_tfs_chirho {
                    if family_chirho.head_declared_chirho {
                        names_chirho.insert(family_chirho.name_chirho.text_chirho().to_string());
                    }
                }
            }
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => {
                names_chirho.insert(name_chirho.text_chirho().to_string());
            }
            _ => {}
        }
    }
    names_chirho
}

fn default_safe_unqualified_imported_type_names_chirho() -> std::collections::HashSet<String> {
    // These names are already modeled as shared runtime-facing types across
    // multiple re-export modules, so keeping them bare avoids spurious
    // mismatches like B.ByteString vs ByteString, I.IORef vs IORef, and
    // E.SomeException vs SomeException.
    ["ByteString", "Ordering", "IORef", "SomeException"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

pub(crate) fn collect_safe_unqualified_imported_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashSet<String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut safe_type_names_chirho = default_safe_unqualified_imported_type_names_chirho();
    safe_type_names_chirho.retain(|name_chirho| !local_type_names_chirho.contains(name_chirho));
    safe_type_names_chirho.extend(
        module_chirho
            .imports_chirho
            .iter()
            .filter(|import_chirho| !import_chirho.qualified_chirho)
            .flat_map(|import_chirho| {
                let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
                ifaces_chirho
                    .iter()
                    .rev()
                    .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
                    .into_iter()
                    .flat_map(|iface_chirho| {
                        haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                            &iface_chirho.exports_chirho,
                            &import_chirho.spec_chirho,
                        )
                    })
            })
            .filter_map(|(name_chirho, namespace_chirho, _span_chirho)| {
                (namespace_chirho
                    == haskelujah_naming_chirho::env_chirho::NamespaceChirho::TypeChirho
                    && !local_type_names_chirho.contains(&name_chirho))
                .then_some(name_chirho)
            }),
    );
    safe_type_names_chirho
}

pub(crate) fn collect_preferred_qualified_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashMap<String, String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut exporters_by_type_name_chirho: std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    > = std::collections::HashMap::new();

    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        else {
            continue;
        };
        for exported_type_name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
            if local_type_names_chirho.contains(exported_type_name_chirho) {
                continue;
            }
            exporters_by_type_name_chirho
                .entry(exported_type_name_chirho.clone())
                .or_default()
                .insert(module_name_chirho.clone());
        }
    }

    exporters_by_type_name_chirho
        .into_iter()
        .filter_map(|(type_name_chirho, exporters_chirho)| {
            (exporters_chirho.len() == 1).then(|| {
                (
                    type_name_chirho,
                    exporters_chirho
                        .into_iter()
                        .next()
                        .expect("single exporter should exist"),
                )
            })
        })
        .collect()
}
