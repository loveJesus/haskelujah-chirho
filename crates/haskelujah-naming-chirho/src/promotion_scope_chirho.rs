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
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, TypeConstructorIdentityChirho,
};
use haskelujah_ast_chirho::promotion_chirho::{
    BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, PromotedKeyChirho, PromotionResolutionChirho,
    PromotionScopeChirho,
};

use crate::env_chirho::NamespaceChirho;
use crate::iface_chirho::{IfaceOriginChirho, ModuleIfaceChirho};
use crate::resolve_chirho::compute_imported_names_chirho;
use crate::type_exports_chirho::{LEVITY_CONSTRUCTORS_CHIRHO, RUNTIME_REP_CONSTRUCTORS_CHIRHO};

/// A datatype the compiler itself defines, whose constructors a trusted builtin interface
/// gives the compiler's identity.
struct BuiltinParentChirho {
    /// The parent type as interfaces list it.
    parent_chirho: &'static str,
    /// The module the key names: where GHC defines the type.
    module_chirho: &'static str,
    /// Its constructors.
    constructors_chirho: &'static [&'static str],
    /// The builtin interfaces trusted to provide it. `None` is any builtin interface, the
    /// Bool and Ordering rule since 686b. Every later entry names its providers exactly,
    /// so a builtin homonym anywhere else stays unproven.
    providers_chirho: Option<&'static [&'static str]>,
    declaration_chirho: Option<CompilerTypeConstructorChirho>,
}

/// The constructors the compiler itself defines: the only constructors a builtin
/// interface can give a builtin key. List, unit and tuple constructors are written with
/// their own syntax and keyed by the converters.
/// workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho
const BUILTIN_PROMOTED_PARENTS_CHIRHO: &[BuiltinParentChirho] = &[
    BuiltinParentChirho {
        parent_chirho: "Bool",
        declaration_chirho: Some(CompilerTypeConstructorChirho::BoolChirho),
        module_chirho: BUILTIN_CONSTRUCTOR_MODULE_CHIRHO,
        constructors_chirho: &["False", "True"],
        providers_chirho: None,
    },
    BuiltinParentChirho {
        parent_chirho: "Ordering",
        declaration_chirho: Some(CompilerTypeConstructorChirho::OrderingChirho),
        module_chirho: BUILTIN_CONSTRUCTOR_MODULE_CHIRHO,
        constructors_chirho: &["LT", "EQ", "GT"],
        providers_chirho: None,
    },
    BuiltinParentChirho {
        parent_chirho: "Maybe",
        declaration_chirho: Some(CompilerTypeConstructorChirho::MaybeChirho),
        module_chirho: "GHC.Maybe",
        constructors_chirho: &["Nothing", "Just"],
        providers_chirho: Some(&["Prelude", "Data.Maybe"]),
    },
    BuiltinParentChirho {
        parent_chirho: "Either",
        declaration_chirho: Some(CompilerTypeConstructorChirho::EitherChirho),
        module_chirho: "Data.Either",
        constructors_chirho: &["Left", "Right"],
        providers_chirho: Some(&["Prelude", "Data.Either"]),
    },
    BuiltinParentChirho {
        parent_chirho: "Proxy",
        declaration_chirho: Some(CompilerTypeConstructorChirho::ProxyChirho),
        module_chirho: "Data.Proxy",
        constructors_chirho: &["Proxy"],
        providers_chirho: Some(&["Data.Proxy", "Data.Data", "Data.Typeable"]),
    },
    BuiltinParentChirho {
        parent_chirho: "Levity",
        declaration_chirho: None,
        module_chirho: BUILTIN_CONSTRUCTOR_MODULE_CHIRHO,
        constructors_chirho: LEVITY_CONSTRUCTORS_CHIRHO,
        providers_chirho: Some(&["GHC.Exts"]),
    },
    BuiltinParentChirho {
        parent_chirho: "RuntimeRep",
        declaration_chirho: None,
        module_chirho: BUILTIN_CONSTRUCTOR_MODULE_CHIRHO,
        constructors_chirho: RUNTIME_REP_CONSTRUCTORS_CHIRHO,
        providers_chirho: Some(&["GHC.Exts"]),
    },
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
                        imported_constructor_chirho(iface_chirho, &name_chirho)
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
    iface_chirho: &ModuleIfaceChirho,
    name_chirho: &str,
) -> Option<PromotionResolutionChirho> {
    let exports_chirho = &iface_chirho.exports_chirho;
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
    } else if let Some(builtin_chirho) =
        trusted_builtin_parent_chirho(iface_chirho, &parent_chirho.name_chirho, name_chirho)
    {
        PromotedKeyChirho::builtin_chirho(builtin_chirho.module_chirho, name_chirho)
    } else {
        // A stub interface, a source forwarder, an untrusted provider, or a builtin
        // constructor outside the compiler's table.
        return Some(PromotionResolutionChirho::UnresolvedChirho(
            PromotedKeyChirho::unknown_chirho("", name_chirho),
        ));
    };
    Some(PromotionResolutionChirho::ResolvedChirho(key_chirho))
}

/// The table entry that trusts `iface_chirho` to give `constructor_chirho` of
/// `parent_chirho` the compiler's identity: a builtin interface, a parent in the table that
/// has the constructor and, where the entry names its providers, one of them.
fn trusted_builtin_parent_chirho(
    iface_chirho: &ModuleIfaceChirho,
    parent_chirho: &str,
    constructor_chirho: &str,
) -> Option<&'static BuiltinParentChirho> {
    if iface_chirho.exports_chirho.origin_chirho != IfaceOriginChirho::BuiltinChirho {
        return None;
    }
    BUILTIN_PROMOTED_PARENTS_CHIRHO.iter().find(|entry_chirho| {
        entry_chirho.parent_chirho == parent_chirho
            && entry_chirho
                .declaration_chirho
                .is_none_or(|expected_chirho| {
                    iface_chirho
                        .exports_chirho
                        .types_chirho
                        .get(parent_chirho)
                        .and_then(|parent_chirho| parent_chirho.declaration_chirho.as_ref())
                        .is_some_and(|declaration_chirho| {
                            declaration_chirho.identity_chirho
                                == TypeConstructorIdentityChirho::CompilerChirho(expected_chirho)
                        })
                })
            && entry_chirho
                .constructors_chirho
                .contains(&constructor_chirho)
            && entry_chirho
                .providers_chirho
                .is_none_or(|providers_chirho| {
                    providers_chirho.contains(&iface_chirho.name_chirho.as_str())
                })
    })
}

/// The names a declaration introduces into the type namespace: its type, family or
/// class, and a class's associated families.
pub(crate) fn declared_type_names_chirho(decl_chirho: &DeclChirho) -> Vec<&str> {
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
pub(crate) fn spelling_chirho(qualifier_chirho: Option<&str>, text_chirho: &str) -> String {
    match qualifier_chirho {
        Some(qualifier_chirho) => format!("{qualifier_chirho}.{text_chirho}"),
        None => text_chirho.to_string(),
    }
}

#[cfg(test)]
#[path = "promotion_scope_tests_chirho.rs"]
mod tests_chirho;
