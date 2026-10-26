// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `toList` imported from Data.List.NonEmpty or Data.Foldable, routed to one dual-shape
//! implementation (unit 517). The bare `toList` every module sees is the IsList identity,
//! which returns a real `x :| xs` unchanged. Each spelling an import really exposes is
//! routed instead to `TO_LIST_ROUTE_CHIRHO`, which returns every other value unchanged and
//! turns `x :| xs` into `x : xs`.
//!
//! Exposure is read from each import's written list through the AST's syntax classifier
//! (row 536):
//! - an unrestricted import exposes `toList`;
//! - a `hiding` list exposes it unless an item names it;
//! - an explicit list exposes it only when an item names it.
//!
//! Which items name it is module-specific and stays here. `Foldable(..)` and
//! `Foldable(toList)` name Data.Foldable's `toList`, while `NonEmpty(..)` never names
//! Data.List.NonEmpty's: its only member is the constructor.
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho, ImportListModeChirho, ModuleChirho,
};

use super::DesugarCtxChirho;
use crate::dict_chirho::prelude_chirho::TO_LIST_ROUTE_CHIRHO;

/// The export every route stands for.
const TO_LIST_CHIRHO: &str = "toList";

/// The routed modules, each with the class whose member list can also name `toList`.
const ROUTED_MODULES_CHIRHO: [(&str, Option<&str>); 2] = [
    ("Data.List.NonEmpty", None),
    ("Data.Foldable", Some("Foldable")),
];

impl DesugarCtxChirho {
    /// Bind `<qualifier>.toList` to the route for every import of a routed module that
    /// exposes `toList`, and the bare `toList` too when that import is unqualified.
    ///
    /// The seeds go into the module scope before the module's own top-level names, so a
    /// top-level `toList` rebinds the bare name, and every lexical binder shadows it.
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    pub(super) fn seed_to_list_routes_chirho(&mut self, module_chirho: &ModuleChirho) {
        for import_chirho in &module_chirho.imports_chirho {
            if !exposes_routed_to_list_chirho(import_chirho) {
                continue;
            }
            let route_id_chirho = self.resolve_var_chirho(TO_LIST_ROUTE_CHIRHO);
            self.bind_in_scope_chirho(
                &format!(
                    "{}.{TO_LIST_CHIRHO}",
                    import_chirho.effective_qualifier_chirho()
                ),
                route_id_chirho,
            );
            if !import_chirho.qualified_chirho {
                self.bind_in_scope_chirho(TO_LIST_CHIRHO, route_id_chirho);
            }
        }
    }
}

/// Whether this import is of a routed module, and its written list lets `toList` through.
fn exposes_routed_to_list_chirho(import_chirho: &ImportDeclChirho) -> bool {
    let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
    let Some((_, class_chirho)) = ROUTED_MODULES_CHIRHO
        .iter()
        .find(|(routed_chirho, _)| *routed_chirho == module_name_chirho)
    else {
        return false;
    };
    match import_chirho.list_mode_chirho() {
        ImportListModeChirho::UnrestrictedChirho => true,
        ImportListModeChirho::HidingChirho(items_chirho) => !items_chirho
            .iter()
            .any(|item_chirho| names_to_list_chirho(item_chirho, *class_chirho)),
        ImportListModeChirho::ExplicitChirho(items_chirho) => items_chirho
            .iter()
            .any(|item_chirho| names_to_list_chirho(item_chirho, *class_chirho)),
    }
}

/// Whether a written item names the module's `toList`: the value itself or, where the
/// module exports it as a class member, that class with `(..)` or with `toList` listed.
/// An unrepresented item names nothing.
fn names_to_list_chirho(item_chirho: &ImportItemChirho, class_chirho: Option<&str>) -> bool {
    match item_chirho {
        ImportItemChirho::VarChirho(name_chirho) => name_chirho.text_chirho() == TO_LIST_CHIRHO,
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho,
        } => {
            class_chirho == Some(name_chirho.text_chirho())
                && match members_chirho {
                    ExportMembersChirho::AllChirho => true,
                    ExportMembersChirho::SomeChirho(listed_chirho) => listed_chirho
                        .iter()
                        .any(|member_chirho| member_chirho.text_chirho() == TO_LIST_CHIRHO),
                    ExportMembersChirho::NoneChirho => false,
                }
        }
        ImportItemChirho::UnrepresentedChirho(_) => false,
    }
}

#[cfg(test)]
#[path = "builtin_routes_tests_chirho.rs"]
mod tests_chirho;
