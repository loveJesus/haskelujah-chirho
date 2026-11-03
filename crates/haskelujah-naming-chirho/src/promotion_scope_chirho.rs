// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Promoted constructors' identity for one module (row 686).
//!
//! Every constructor spelling the module can write (`T`, `M.T`) is bound to the
//! constructor it names. Proof comes only from an interface's constructor membership and
//! its recorded defining module, kept through re-exports, or from this module's own data
//! and newtype declarations. A function, field or pattern synonym is not a constructor.
//! A builtin key also needs a trusted builtin interface and a parent in the compiler's
//! fixed table, so a source module spelling `True` gains nothing. A spelling bound to two
//! different constructors, or proven by nothing, is Unknown. The type-namespace spellings
//! are kept too, because an unquoted name the type namespace has is the type.
//! workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::promotion_chirho::{
    BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, PromotedKeyChirho, PromotionResolutionChirho,
    PromotionScopeChirho,
};

use crate::env_chirho::NamespaceChirho;
use crate::iface_chirho::{IfaceExportsChirho, IfaceOriginChirho, ModuleIfaceChirho};
use crate::resolve_chirho::compute_imported_names_chirho;

/// The constructors the compiler itself defines, by parent type: the only constructors a
/// builtin interface can give a builtin key. List, unit and tuple constructors are
/// written with their own syntax and keyed by the converters.
const BUILTIN_PROMOTED_CONSTRUCTORS_CHIRHO: &[(&str, &[&str])] = &[
    ("Bool", &["False", "True"]),
    ("Ordering", &["LT", "EQ", "GT"]),
];

/// One module's promoted-constructor scope, built from its declarations and the
/// interfaces its imports name, the same ones resolution read.
pub fn build_promotion_scope_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> PromotionScopeChirho {
    let mut builder_chirho = ScopeBuilderChirho::default();
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
    for decl_chirho in &module_chirho.decls_chirho {
        for type_name_chirho in declared_type_names_chirho(decl_chirho) {
            builder_chirho.bind_type_chirho(None, type_name_chirho);
            builder_chirho.bind_type_chirho(Some(&module_name_chirho), type_name_chirho);
        }
        for constructor_chirho in declared_constructors_chirho(decl_chirho) {
            let resolution_chirho = PromotionResolutionChirho::ResolvedChirho(
                PromotedKeyChirho::source_chirho(&module_name_chirho, constructor_chirho),
            );
            builder_chirho.bind_constructor_chirho(None, constructor_chirho, &resolution_chirho);
            builder_chirho.bind_constructor_chirho(
                Some(&module_name_chirho),
                constructor_chirho,
                &resolution_chirho,
            );
        }
    }
    for import_chirho in &module_chirho.imports_chirho {
        let imported_module_chirho = import_chirho.module_chirho.full_name_chirho();
        // Resolution reports a missing interface; its names stay unbound here.
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == imported_module_chirho)
        else {
            continue;
        };
        let qualifier_chirho = import_chirho
            .alias_chirho
            .as_ref()
            .map_or(imported_module_chirho.clone(), |alias_chirho| {
                alias_chirho.text_chirho().to_string()
            });
        let unqualified_chirho = !import_chirho.qualified_chirho;
        for (name_chirho, namespace_chirho, _) in
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho)
        {
            match namespace_chirho {
                NamespaceChirho::TypeChirho => {
                    builder_chirho.bind_type_chirho(Some(&qualifier_chirho), &name_chirho);
                    if unqualified_chirho {
                        builder_chirho.bind_type_chirho(None, &name_chirho);
                    }
                }
                NamespaceChirho::ValueChirho => {
                    let Some(resolution_chirho) =
                        imported_constructor_chirho(&iface_chirho.exports_chirho, &name_chirho)
                    else {
                        continue;
                    };
                    builder_chirho.bind_constructor_chirho(
                        Some(&qualifier_chirho),
                        &name_chirho,
                        &resolution_chirho,
                    );
                    if unqualified_chirho {
                        builder_chirho.bind_constructor_chirho(
                            None,
                            &name_chirho,
                            &resolution_chirho,
                        );
                    }
                }
            }
        }
    }
    PromotionScopeChirho::new_chirho(
        builder_chirho.constructors_chirho,
        builder_chirho.types_chirho,
    )
}

/// What an imported value names when it is a constructor; `None` when no exported type
/// lists it, so it is no promotion at all.
fn imported_constructor_chirho(
    exports_chirho: &IfaceExportsChirho,
    name_chirho: &str,
) -> Option<PromotionResolutionChirho> {
    let parent_chirho = exports_chirho.types_chirho.values().find(|ty_chirho| {
        ty_chirho
            .constructors_chirho
            .iter()
            .any(|constructor_chirho| constructor_chirho == name_chirho)
    })?;
    let defining_module_chirho = exports_chirho
        .values_chirho
        .get(name_chirho)
        .and_then(|value_chirho| value_chirho.defining_module_chirho.as_deref());
    let key_chirho = if let Some(module_chirho) = defining_module_chirho {
        PromotedKeyChirho::source_chirho(module_chirho, name_chirho)
    } else if exports_chirho.origin_chirho == IfaceOriginChirho::BuiltinChirho
        && is_builtin_constructor_chirho(&parent_chirho.name_chirho, name_chirho)
    {
        PromotedKeyChirho::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, name_chirho)
    } else {
        // A stub interface, or a builtin constructor outside the compiler's table.
        return Some(PromotionResolutionChirho::UnresolvedChirho(
            PromotedKeyChirho::unknown_chirho("", name_chirho),
        ));
    };
    Some(PromotionResolutionChirho::ResolvedChirho(key_chirho))
}

/// Whether the compiler itself defines `constructor` as a constructor of `parent`.
fn is_builtin_constructor_chirho(parent_chirho: &str, constructor_chirho: &str) -> bool {
    BUILTIN_PROMOTED_CONSTRUCTORS_CHIRHO.iter().any(
        |(builtin_parent_chirho, constructors_chirho)| {
            *builtin_parent_chirho == parent_chirho
                && constructors_chirho.contains(&constructor_chirho)
        },
    )
}

/// The names a declaration introduces into the type namespace: its type, family or
/// class, and a class's associated families.
fn declared_type_names_chirho(decl_chirho: &DeclChirho) -> Vec<&str> {
    match decl_chirho {
        DeclChirho::DataDeclChirho { name_chirho, .. }
        | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
        | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
        | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => vec![name_chirho.text_chirho()],
        DeclChirho::ClassDeclChirho {
            name_chirho,
            associated_tfs_chirho,
            ..
        } => std::iter::once(name_chirho.text_chirho())
            .chain(
                associated_tfs_chirho
                    .iter()
                    .map(|family_chirho| family_chirho.name_chirho.text_chirho()),
            )
            .collect(),
        _ => Vec::new(),
    }
}

/// The constructors a data or newtype declaration writes.
fn declared_constructors_chirho(decl_chirho: &DeclChirho) -> Vec<&str> {
    let constructors_chirho: Vec<&ConDeclChirho> = match decl_chirho {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => constructors_chirho.iter().collect(),
        DeclChirho::NewtypeDeclChirho {
            constructor_chirho, ..
        } => vec![constructor_chirho],
        _ => Vec::new(),
    };
    constructors_chirho
        .into_iter()
        .map(|constructor_chirho| match constructor_chirho {
            ConDeclChirho::OrdinaryChirho { name_chirho, .. }
            | ConDeclChirho::RecordChirho { name_chirho, .. }
            | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.text_chirho(),
        })
        // Lowering's placeholder for a constructor it could not read names nothing.
        .filter(|name_chirho| !name_chirho.is_empty())
        .collect()
}

/// Spellings bound so far. A spelling bound twice to different constructors is Unknown.
#[derive(Default)]
struct ScopeBuilderChirho {
    constructors_chirho: HashMap<String, PromotionResolutionChirho>,
    types_chirho: HashSet<String>,
}

impl ScopeBuilderChirho {
    fn bind_constructor_chirho(
        &mut self,
        qualifier_chirho: Option<&str>,
        text_chirho: &str,
        resolution_chirho: &PromotionResolutionChirho,
    ) {
        match self
            .constructors_chirho
            .entry(spelling_chirho(qualifier_chirho, text_chirho))
        {
            Entry::Vacant(vacant_chirho) => {
                vacant_chirho.insert(resolution_chirho.clone());
            }
            Entry::Occupied(mut occupied_chirho) => {
                if occupied_chirho.get() != resolution_chirho {
                    occupied_chirho.insert(PromotionResolutionChirho::UnresolvedChirho(
                        PromotedKeyChirho::unknown_chirho(
                            qualifier_chirho.unwrap_or_default(),
                            text_chirho,
                        ),
                    ));
                }
            }
        }
    }

    fn bind_type_chirho(&mut self, qualifier_chirho: Option<&str>, text_chirho: &str) {
        self.types_chirho
            .insert(spelling_chirho(qualifier_chirho, text_chirho));
    }
}

/// A spelling as a name writes it: `M.T`, or `T` unqualified.
fn spelling_chirho(qualifier_chirho: Option<&str>, text_chirho: &str) -> String {
    match qualifier_chirho {
        Some(qualifier_chirho) => format!("{qualifier_chirho}.{text_chirho}"),
        None => text_chirho.to_string(),
    }
}

#[cfg(test)]
#[path = "promotion_scope_tests_chirho.rs"]
mod tests_chirho;
