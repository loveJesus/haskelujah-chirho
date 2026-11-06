// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Module interfaces
//!
//! A `ModuleIfaceChirho` represents the public API of a compiled module — the
//! names (values, types, constructors) that other modules may import from it.
//! Building an interface applies the module's export list to its declarations.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportItemChirho, ImportSpecChirho, ModuleChirho,
};
use haskelujah_span_chirho::SpanChirho;

use crate::type_exports_chirho::{
    canonical_type_name_chirho, canonical_value_name_chirho, collect_all_definitions_chirho,
    locally_complete_type_names_chirho, merge_all_associated_type_exports_chirho,
    merge_associated_type_exports_chirho, merged_complete_member_types_chirho,
    remove_associated_type_exports_chirho, selected_associated_type_names_chirho,
};

mod builtin_chirho;
mod class_identity_chirho;
#[cfg(test)]
mod tests_chirho;

pub use builtin_chirho::builtin_module_ifaces_chirho;
pub use class_identity_chirho::build_builtin_class_scope_chirho;

// ---------------------------------------------------------------------------
// Interface types
// ---------------------------------------------------------------------------

/// An exported value (function, variable, data constructor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfaceValueChirho {
    pub name_chirho: String,
    pub span_chirho: SpanChirho,
    /// The source module that defines the value (row 642). It is `Some` only for a source
    /// module's own definition, and a re-export clones it, so it keeps naming the definer however
    /// many modules forward the value. It is `None` for builtin values and for stub interfaces,
    /// whose text cannot say where a value is defined.
    /// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
    pub defining_module_chirho: Option<String>,
}

/// An exported type with its available constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfaceTypeChirho {
    pub name_chirho: String,
    /// Constructor names that are exported alongside this type.
    pub constructors_chirho: Vec<String>,
    /// Associated value names exported alongside this type or class.
    /// For classes this is method names; for record types/newtypes this includes
    /// field selector functions.
    pub methods_chirho: Vec<String>,
    /// Whether this export's producer knows it is a class: a source class declaration, the
    /// builtin class table or an explicit builtin class entry. `false` claims nothing, so a
    /// class whose producer cannot tell, such as a dependency package's, stays unmarked. K01
    /// proves the methodless `Typeable` only from a marked export (#32090).
    /// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
    pub is_class_chirho: bool,
    pub span_chirho: SpanChirho,
}

/// Where an interface came from (row 608). Only a builtin interface lets a qualified alias read
/// the generated GHC agreement table; a source module's never does, whatever its name.
/// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IfaceOriginChirho {
    /// Built from source, or of unknown origin: the fail-closed default.
    #[default]
    SourceChirho,
    /// Built by `builtin_module_ifaces_chirho`, and never merged with a source copy.
    BuiltinChirho,
}

/// Everything a module exports.
#[derive(Debug, Clone, Default)]
pub struct IfaceExportsChirho {
    /// Value-level exports: functions, variables, data constructors.
    pub values_chirho: HashMap<String, IfaceValueChirho>,
    /// Type-level exports: type constructors, type classes.
    pub types_chirho: HashMap<String, IfaceTypeChirho>,
    /// Associated type/data-family names grouped under their parent class.
    /// Each member also has its own entry in `types_chirho`.
    pub associated_types_chirho: HashMap<String, Vec<String>>,
    /// Types and classes whose entry lists every constructor, field and
    /// method that `T(..)` brings. Only these let a `T(..)` item close a
    /// qualified import list. A source module marks the types it declares; a
    /// builtin interface marks a type only when its entry matches GHC's
    /// declaration. Copies of one module merge their marks per type. Losing
    /// a mark only leaves a qualifier open.
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub complete_member_types_chirho: HashSet<String>,
    /// Where this interface came from; source unless proven builtin (row 608).
    pub origin_chirho: IfaceOriginChirho,
}

/// A compiled module's public interface.
#[derive(Debug, Clone)]
pub struct ModuleIfaceChirho {
    /// Fully qualified module name.
    pub name_chirho: String,
    /// The exported names.
    pub exports_chirho: IfaceExportsChirho,
}

fn merge_imported_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    iface_exports_chirho: &IfaceExportsChirho,
    spec_chirho: &Option<ImportSpecChirho>,
) {
    match spec_chirho {
        None => {
            for (name_chirho, value_chirho) in &iface_exports_chirho.values_chirho {
                result_chirho
                    .values_chirho
                    .insert(name_chirho.clone(), value_chirho.clone());
            }
            for (name_chirho, ty_chirho) in &iface_exports_chirho.types_chirho {
                result_chirho
                    .types_chirho
                    .insert(name_chirho.clone(), ty_chirho.clone());
            }
            merge_all_associated_type_exports_chirho(result_chirho, iface_exports_chirho);
        }
        Some(spec_chirho) if spec_chirho.hiding_chirho => {
            merge_imported_exports_chirho(result_chirho, iface_exports_chirho, &None);
            for item_chirho in &spec_chirho.items_chirho {
                remove_import_item_from_exports_chirho(
                    result_chirho,
                    iface_exports_chirho,
                    item_chirho,
                );
            }
        }
        Some(spec_chirho) => {
            for item_chirho in &spec_chirho.items_chirho {
                merge_import_item_into_exports_chirho(
                    result_chirho,
                    iface_exports_chirho,
                    item_chirho,
                );
            }
        }
    }
}

fn merge_import_item_into_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    iface_exports_chirho: &IfaceExportsChirho,
    item_chirho: &ImportItemChirho,
) {
    match item_chirho {
        ImportItemChirho::VarChirho(name_chirho) => {
            let value_name_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
            if let Some(value_chirho) = iface_exports_chirho.values_chirho.get(&value_name_chirho) {
                result_chirho
                    .values_chirho
                    .insert(value_name_chirho, value_chirho.clone());
            }
        }
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            let ty_name_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
            if let Some(ty_chirho) = iface_exports_chirho.types_chirho.get(&ty_name_chirho) {
                let associated_names_chirho = selected_associated_type_names_chirho(
                    iface_exports_chirho,
                    &ty_name_chirho,
                    members_chirho,
                );
                let (constructors_chirho, methods_chirho) = match members_chirho {
                    ExportMembersChirho::AllChirho => (
                        ty_chirho.constructors_chirho.clone(),
                        ty_chirho.methods_chirho.clone(),
                    ),
                    ExportMembersChirho::SomeChirho(names_chirho) => {
                        let selected_names_chirho: Vec<String> = names_chirho
                            .iter()
                            .map(|name_chirho| {
                                canonical_value_name_chirho(name_chirho.text_chirho())
                            })
                            .collect();
                        let constructors_chirho = ty_chirho
                            .constructors_chirho
                            .iter()
                            .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                            .cloned()
                            .collect();
                        let methods_chirho = ty_chirho
                            .methods_chirho
                            .iter()
                            .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                            .cloned()
                            .collect();
                        (constructors_chirho, methods_chirho)
                    }
                    ExportMembersChirho::NoneChirho => (vec![], vec![]),
                };
                for constructor_chirho in &constructors_chirho {
                    if let Some(value_chirho) =
                        iface_exports_chirho.values_chirho.get(constructor_chirho)
                    {
                        result_chirho
                            .values_chirho
                            .insert(constructor_chirho.clone(), value_chirho.clone());
                    }
                }
                for method_chirho in &methods_chirho {
                    if let Some(value_chirho) =
                        iface_exports_chirho.values_chirho.get(method_chirho)
                    {
                        result_chirho
                            .values_chirho
                            .insert(method_chirho.clone(), value_chirho.clone());
                    }
                }
                result_chirho.types_chirho.insert(
                    ty_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: ty_chirho.name_chirho.clone(),
                        constructors_chirho,
                        methods_chirho,
                        is_class_chirho: ty_chirho.is_class_chirho,
                        span_chirho: ty_chirho.span_chirho,
                    },
                );
                merge_associated_type_exports_chirho(
                    result_chirho,
                    iface_exports_chirho,
                    &ty_name_chirho,
                    &associated_names_chirho,
                );
            }
        }
        // An item lowering could not represent names nothing to re-export,
        // as when lowering dropped it.
        // workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
        ImportItemChirho::UnrepresentedChirho(_) => {}
    }
}

fn remove_import_item_from_exports_chirho(
    result_chirho: &mut IfaceExportsChirho,
    iface_exports_chirho: &IfaceExportsChirho,
    item_chirho: &ImportItemChirho,
) {
    match item_chirho {
        ImportItemChirho::VarChirho(name_chirho) => {
            let value_name_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
            result_chirho.values_chirho.remove(&value_name_chirho);
        }
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            let ty_name_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
            match members_chirho {
                ExportMembersChirho::NoneChirho => {
                    result_chirho.types_chirho.remove(&ty_name_chirho);
                    result_chirho
                        .associated_types_chirho
                        .remove(&ty_name_chirho);
                }
                ExportMembersChirho::AllChirho => {
                    result_chirho.types_chirho.remove(&ty_name_chirho);
                    if let Some(ty_chirho) = iface_exports_chirho.types_chirho.get(&ty_name_chirho)
                    {
                        for constructor_chirho in &ty_chirho.constructors_chirho {
                            result_chirho.values_chirho.remove(constructor_chirho);
                        }
                        for method_chirho in &ty_chirho.methods_chirho {
                            result_chirho.values_chirho.remove(method_chirho);
                        }
                    }
                    let associated_names_chirho = selected_associated_type_names_chirho(
                        iface_exports_chirho,
                        &ty_name_chirho,
                        members_chirho,
                    );
                    remove_associated_type_exports_chirho(
                        result_chirho,
                        &ty_name_chirho,
                        &associated_names_chirho,
                    );
                }
                ExportMembersChirho::SomeChirho(names_chirho) => {
                    let selected_names_chirho: Vec<String> = names_chirho
                        .iter()
                        .map(|name_chirho| canonical_value_name_chirho(name_chirho.text_chirho()))
                        .collect();
                    if let Some(ty_chirho) = result_chirho.types_chirho.get_mut(&ty_name_chirho) {
                        ty_chirho
                            .constructors_chirho
                            .retain(|name_chirho| !selected_names_chirho.contains(name_chirho));
                        ty_chirho
                            .methods_chirho
                            .retain(|name_chirho| !selected_names_chirho.contains(name_chirho));
                    }
                    for selected_name_chirho in selected_names_chirho {
                        result_chirho.values_chirho.remove(&selected_name_chirho);
                    }
                    let associated_names_chirho = selected_associated_type_names_chirho(
                        iface_exports_chirho,
                        &ty_name_chirho,
                        members_chirho,
                    );
                    remove_associated_type_exports_chirho(
                        result_chirho,
                        &ty_name_chirho,
                        &associated_names_chirho,
                    );
                }
            }
        }
        // An item lowering could not represent names nothing to hide, as when
        // lowering dropped it.
        // workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
        ImportItemChirho::UnrepresentedChirho(_) => {}
    }
}

fn imported_value_in_scope_chirho(
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
    value_name_chirho: &str,
) -> Option<IfaceValueChirho> {
    for import_chirho in &module_chirho.imports_chirho {
        if import_chirho.qualified_chirho {
            continue;
        }
        let Some(iface_chirho) = imported_ifaces_chirho.iter().find(|iface_chirho| {
            iface_chirho.name_chirho == import_chirho.module_chirho.full_name_chirho()
        }) else {
            continue;
        };
        let imported_names_chirho = crate::resolve_chirho::compute_imported_names_chirho(
            &iface_chirho.exports_chirho,
            &import_chirho.spec_chirho,
        );
        if imported_names_chirho
            .iter()
            .any(|(name_chirho, namespace_chirho, _span_chirho)| {
                name_chirho == value_name_chirho
                    && *namespace_chirho == crate::env_chirho::NamespaceChirho::ValueChirho
            })
        {
            if let Some(value_chirho) = iface_chirho
                .exports_chirho
                .values_chirho
                .get(value_name_chirho)
            {
                return Some(value_chirho.clone());
            }
        }
    }
    None
}

fn imported_type_in_scope_chirho(
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
    type_name_chirho: &str,
    members_chirho: &ExportMembersChirho,
) -> Option<(IfaceTypeChirho, Vec<IfaceValueChirho>, IfaceExportsChirho)> {
    for import_chirho in &module_chirho.imports_chirho {
        if import_chirho.qualified_chirho {
            continue;
        }
        let Some(iface_chirho) = imported_ifaces_chirho.iter().find(|iface_chirho| {
            iface_chirho.name_chirho == import_chirho.module_chirho.full_name_chirho()
        }) else {
            continue;
        };
        let imported_names_chirho = crate::resolve_chirho::compute_imported_names_chirho(
            &iface_chirho.exports_chirho,
            &import_chirho.spec_chirho,
        );
        let imported_type_visible_chirho =
            imported_names_chirho
                .iter()
                .any(|(name_chirho, namespace_chirho, _span_chirho)| {
                    name_chirho == type_name_chirho
                        && *namespace_chirho == crate::env_chirho::NamespaceChirho::TypeChirho
                });
        if !imported_type_visible_chirho {
            continue;
        }
        let Some(ty_chirho) = iface_chirho
            .exports_chirho
            .types_chirho
            .get(type_name_chirho)
        else {
            continue;
        };
        let (constructors_chirho, methods_chirho) = match members_chirho {
            ExportMembersChirho::AllChirho => (
                ty_chirho.constructors_chirho.clone(),
                ty_chirho.methods_chirho.clone(),
            ),
            ExportMembersChirho::SomeChirho(names_chirho) => {
                let selected_names_chirho: Vec<String> = names_chirho
                    .iter()
                    .map(|name_chirho| canonical_value_name_chirho(name_chirho.text_chirho()))
                    .collect();
                let constructors_chirho = ty_chirho
                    .constructors_chirho
                    .iter()
                    .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                    .cloned()
                    .collect();
                let methods_chirho = ty_chirho
                    .methods_chirho
                    .iter()
                    .filter(|name_chirho| selected_names_chirho.contains(name_chirho))
                    .cloned()
                    .collect();
                (constructors_chirho, methods_chirho)
            }
            ExportMembersChirho::NoneChirho => (vec![], vec![]),
        };
        let mut accompanying_values_chirho = Vec::new();
        for constructor_name_chirho in &constructors_chirho {
            if let Some(value_chirho) = iface_chirho
                .exports_chirho
                .values_chirho
                .get(constructor_name_chirho)
            {
                accompanying_values_chirho.push(value_chirho.clone());
            }
        }
        for method_name_chirho in &methods_chirho {
            if let Some(value_chirho) = iface_chirho
                .exports_chirho
                .values_chirho
                .get(method_name_chirho)
            {
                accompanying_values_chirho.push(value_chirho.clone());
            }
        }
        let associated_names_chirho = selected_associated_type_names_chirho(
            &iface_chirho.exports_chirho,
            type_name_chirho,
            members_chirho,
        )
        .into_iter()
        .filter(|associated_name_chirho| {
            imported_names_chirho
                .iter()
                .any(|(name_chirho, namespace_chirho, _span_chirho)| {
                    name_chirho == associated_name_chirho
                        && *namespace_chirho == crate::env_chirho::NamespaceChirho::TypeChirho
                })
        })
        .collect::<Vec<_>>();
        let mut accompanying_associated_chirho = IfaceExportsChirho::default();
        merge_associated_type_exports_chirho(
            &mut accompanying_associated_chirho,
            &iface_chirho.exports_chirho,
            type_name_chirho,
            &associated_names_chirho,
        );
        return Some((
            IfaceTypeChirho {
                name_chirho: ty_chirho.name_chirho.clone(),
                constructors_chirho,
                methods_chirho,
                is_class_chirho: ty_chirho.is_class_chirho,
                span_chirho: ty_chirho.span_chirho,
            },
            accompanying_values_chirho,
            accompanying_associated_chirho,
        ));
    }
    None
}

// ---------------------------------------------------------------------------
// Building an interface from a module
// ---------------------------------------------------------------------------

/// Build a module interface from a parsed module. Applies the export list
/// to determine which names are publicly visible.
pub fn build_iface_chirho(module_chirho: &ModuleChirho) -> ModuleIfaceChirho {
    build_iface_with_imports_chirho(module_chirho, &[])
}

/// Build a module interface from a parsed module, with access to imported
/// module interfaces for handling `module Foo` re-exports in the export list.
pub fn build_iface_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
) -> ModuleIfaceChirho {
    // Use full_name_chirho to preserve qualified module names (e.g., "Sub.Helper"
    // instead of just "Helper"). This is critical for hierarchical module imports.
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();

    // First, collect ALL definitions in the module.
    let all_exports_chirho = collect_all_definitions_chirho(module_chirho);

    // Then filter by the export list.
    let mut exports_chirho = match &module_chirho.exports_chirho {
        None => {
            // No export list means export everything.
            all_exports_chirho
        }
        Some(specs_chirho) => filter_exports_chirho(
            &all_exports_chirho,
            specs_chirho,
            module_chirho,
            imported_ifaces_chirho,
        ),
    };
    // The types this module declares are complete as exported.
    // workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    exports_chirho.complete_member_types_chirho =
        locally_complete_type_names_chirho(module_chirho, &exports_chirho);
    ModuleIfaceChirho {
        name_chirho: module_name_chirho,
        exports_chirho,
    }
}

/// Merge module interfaces that share the same module name, preserving the
/// union of exported values and type members across all copies. A type's
/// completeness mark survives only where every copy exporting it marks it.
pub fn merge_module_ifaces_chirho(
    modules_chirho: Vec<ModuleIfaceChirho>,
) -> Vec<ModuleIfaceChirho> {
    let mut deduped_chirho: Vec<ModuleIfaceChirho> = Vec::with_capacity(modules_chirho.len());
    let mut index_chirho: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for module_chirho in modules_chirho {
        if let Some(&idx_chirho) = index_chirho.get(&module_chirho.name_chirho) {
            let existing_chirho = &mut deduped_chirho[idx_chirho];
            // A merge stays builtin only when both copies are (row 608).
            // workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
            if module_chirho.exports_chirho.origin_chirho != IfaceOriginChirho::BuiltinChirho {
                existing_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::SourceChirho;
            }
            // Marks merge per type, read before this copy's types join the
            // entry. workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
            existing_chirho.exports_chirho.complete_member_types_chirho =
                merged_complete_member_types_chirho(
                    &existing_chirho.exports_chirho,
                    &module_chirho.exports_chirho,
                );
            for (name_chirho, value_chirho) in module_chirho.exports_chirho.values_chirho {
                existing_chirho
                    .exports_chirho
                    .values_chirho
                    .entry(name_chirho)
                    .or_insert(value_chirho);
            }
            for (name_chirho, type_chirho) in module_chirho.exports_chirho.types_chirho {
                match existing_chirho
                    .exports_chirho
                    .types_chirho
                    .get_mut(&name_chirho)
                {
                    Some(existing_type_chirho) => {
                        existing_type_chirho.is_class_chirho |= type_chirho.is_class_chirho;
                        for constructor_chirho in type_chirho.constructors_chirho {
                            if !existing_type_chirho
                                .constructors_chirho
                                .contains(&constructor_chirho)
                            {
                                existing_type_chirho
                                    .constructors_chirho
                                    .push(constructor_chirho);
                            }
                        }
                        for method_chirho in type_chirho.methods_chirho {
                            if !existing_type_chirho.methods_chirho.contains(&method_chirho) {
                                existing_type_chirho.methods_chirho.push(method_chirho);
                            }
                        }
                    }
                    None => {
                        existing_chirho
                            .exports_chirho
                            .types_chirho
                            .insert(name_chirho, type_chirho);
                    }
                }
            }
            for (parent_chirho, members_chirho) in
                module_chirho.exports_chirho.associated_types_chirho
            {
                let existing_members_chirho = existing_chirho
                    .exports_chirho
                    .associated_types_chirho
                    .entry(parent_chirho)
                    .or_default();
                for member_chirho in members_chirho {
                    if !existing_members_chirho.contains(&member_chirho) {
                        existing_members_chirho.push(member_chirho);
                    }
                }
            }
        } else {
            index_chirho.insert(module_chirho.name_chirho.clone(), deduped_chirho.len());
            deduped_chirho.push(module_chirho);
        }
    }

    deduped_chirho
}

/// Apply an explicit export list to filter the module's definitions.
/// `module_chirho` is the parsed module (for checking imports of re-exported modules).
/// `imported_ifaces_chirho` provides the interfaces of imported modules for re-exports.
fn filter_exports_chirho(
    all_chirho: &IfaceExportsChirho,
    specs_chirho: &[ExportSpecChirho],
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
) -> IfaceExportsChirho {
    let mut result_chirho = IfaceExportsChirho::default();

    for spec_chirho in specs_chirho {
        match spec_chirho {
            ExportSpecChirho::VarChirho(name_chirho) => {
                let value_text_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
                if let Some(val_chirho) = all_chirho.values_chirho.get(&value_text_chirho) {
                    result_chirho
                        .values_chirho
                        .insert(value_text_chirho.clone(), val_chirho.clone());
                } else if let Some(imported_value_chirho) = imported_value_in_scope_chirho(
                    module_chirho,
                    imported_ifaces_chirho,
                    &value_text_chirho,
                ) {
                    result_chirho
                        .values_chirho
                        .insert(value_text_chirho.clone(), imported_value_chirho);
                }
                let text_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
                // A bare name in an export list can also refer to a type
                if let Some(ty_chirho) = all_chirho.types_chirho.get(&text_chirho) {
                    result_chirho.types_chirho.insert(
                        text_chirho.clone(),
                        IfaceTypeChirho {
                            name_chirho: ty_chirho.name_chirho.clone(),
                            constructors_chirho: vec![], // bare name = no constructors
                            methods_chirho: vec![],
                            is_class_chirho: ty_chirho.is_class_chirho,
                            span_chirho: ty_chirho.span_chirho,
                        },
                    );
                } else if let Some((
                    imported_type_chirho,
                    _accompanying_values_chirho,
                    _accompanying_associated_chirho,
                )) = imported_type_in_scope_chirho(
                    module_chirho,
                    imported_ifaces_chirho,
                    &text_chirho,
                    &ExportMembersChirho::NoneChirho,
                ) {
                    result_chirho
                        .types_chirho
                        .insert(text_chirho, imported_type_chirho);
                }
            }
            ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho,
            } => {
                let text_chirho = canonical_type_name_chirho(name_chirho.text_chirho());
                if let Some(ty_chirho) = all_chirho.types_chirho.get(&text_chirho) {
                    let (cons_chirho, methods_chirho) = match members_chirho {
                        ExportMembersChirho::AllChirho => (
                            ty_chirho.constructors_chirho.clone(),
                            ty_chirho.methods_chirho.clone(),
                        ),
                        ExportMembersChirho::SomeChirho(names_chirho) => {
                            let selected_chirho: Vec<String> = names_chirho
                                .iter()
                                .map(|n_chirho| canonical_value_name_chirho(n_chirho.text_chirho()))
                                .collect();
                            let sel_cons_chirho: Vec<String> = ty_chirho
                                .constructors_chirho
                                .iter()
                                .filter(|c_chirho| selected_chirho.contains(c_chirho))
                                .cloned()
                                .collect();
                            let sel_methods_chirho: Vec<String> = ty_chirho
                                .methods_chirho
                                .iter()
                                .filter(|m_chirho| selected_chirho.contains(m_chirho))
                                .cloned()
                                .collect();
                            (sel_cons_chirho, sel_methods_chirho)
                        }
                        ExportMembersChirho::NoneChirho => (vec![], vec![]),
                    };

                    // Export the selected constructors/methods as values
                    for cn_chirho in &cons_chirho {
                        if let Some(val_chirho) = all_chirho.values_chirho.get(cn_chirho) {
                            result_chirho
                                .values_chirho
                                .insert(cn_chirho.clone(), val_chirho.clone());
                        }
                    }
                    for mn_chirho in &methods_chirho {
                        if let Some(val_chirho) = all_chirho.values_chirho.get(mn_chirho) {
                            result_chirho
                                .values_chirho
                                .insert(mn_chirho.clone(), val_chirho.clone());
                        }
                    }

                    result_chirho.types_chirho.insert(
                        text_chirho.clone(),
                        IfaceTypeChirho {
                            name_chirho: ty_chirho.name_chirho.clone(),
                            constructors_chirho: cons_chirho,
                            methods_chirho,
                            is_class_chirho: ty_chirho.is_class_chirho,
                            span_chirho: ty_chirho.span_chirho,
                        },
                    );
                    let associated_names_chirho = selected_associated_type_names_chirho(
                        all_chirho,
                        &text_chirho,
                        members_chirho,
                    );
                    merge_associated_type_exports_chirho(
                        &mut result_chirho,
                        all_chirho,
                        &text_chirho,
                        &associated_names_chirho,
                    );
                } else if let Some((
                    imported_type_chirho,
                    accompanying_values_chirho,
                    accompanying_associated_chirho,
                )) = imported_type_in_scope_chirho(
                    module_chirho,
                    imported_ifaces_chirho,
                    &text_chirho,
                    members_chirho,
                ) {
                    for value_chirho in accompanying_values_chirho {
                        result_chirho
                            .values_chirho
                            .insert(value_chirho.name_chirho.clone(), value_chirho);
                    }
                    result_chirho
                        .types_chirho
                        .insert(text_chirho, imported_type_chirho);
                    merge_all_associated_type_exports_chirho(
                        &mut result_chirho,
                        &accompanying_associated_chirho,
                    );
                }
            }
            ExportSpecChirho::ModuleChirho(re_export_name_chirho) => {
                // `module M` exports every entity in scope both unqualified (`e`) and qualified
                // by M (`M.e`), whichever imports bring it (Haskell 2010, 5.2; row 621). Each
                // import is weighed on its own, so a qualified-only import never leaks into the
                // unqualified scope, and one import's `hiding` never removes another's names.
                // workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
                let target_mod_chirho = re_export_name_chirho.full_name_chirho();
                let is_self_chirho =
                    module_chirho.name_chirho.full_name_chirho() == target_mod_chirho;
                let mut unqualified_chirho = all_chirho.clone();
                let mut qualified_chirho = if is_self_chirho {
                    all_chirho.clone()
                } else {
                    IfaceExportsChirho::default()
                };
                for import_chirho in &module_chirho.imports_chirho {
                    let Some(iface_chirho) = imported_ifaces_chirho.iter().find(|iface_chirho| {
                        iface_chirho.name_chirho == import_chirho.module_chirho.full_name_chirho()
                    }) else {
                        continue;
                    };
                    let mut supplied_chirho = IfaceExportsChirho::default();
                    merge_imported_exports_chirho(
                        &mut supplied_chirho,
                        &iface_chirho.exports_chirho,
                        &import_chirho.spec_chirho,
                    );
                    if !import_chirho.qualified_chirho {
                        merge_imported_exports_chirho(
                            &mut unqualified_chirho,
                            &supplied_chirho,
                            &None,
                        );
                    }
                    if import_chirho.effective_qualifier_chirho() == target_mod_chirho {
                        merge_imported_exports_chirho(
                            &mut qualified_chirho,
                            &supplied_chirho,
                            &None,
                        );
                    }
                }
                for (key_chirho, value_chirho) in &qualified_chirho.values_chirho {
                    if unqualified_chirho.values_chirho.contains_key(key_chirho) {
                        result_chirho
                            .values_chirho
                            .insert(key_chirho.clone(), value_chirho.clone());
                    }
                }
                for (key_chirho, ty_chirho) in &qualified_chirho.types_chirho {
                    if unqualified_chirho.types_chirho.contains_key(key_chirho) {
                        result_chirho
                            .types_chirho
                            .insert(key_chirho.clone(), ty_chirho.clone());
                        if let Some(associated_chirho) =
                            qualified_chirho.associated_types_chirho.get(key_chirho)
                        {
                            result_chirho
                                .associated_types_chirho
                                .entry(key_chirho.clone())
                                .or_default()
                                .extend(associated_chirho.iter().cloned());
                        }
                        if qualified_chirho
                            .complete_member_types_chirho
                            .contains(key_chirho)
                        {
                            result_chirho
                                .complete_member_types_chirho
                                .insert(key_chirho.clone());
                        }
                    }
                }
            }
        }
    }

    result_chirho
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
