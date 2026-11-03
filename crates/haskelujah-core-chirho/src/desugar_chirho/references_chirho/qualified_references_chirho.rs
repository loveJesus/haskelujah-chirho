// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which binding a written reference names (row 598).
//!
//! An unqualified name resolves through the scope chain, innermost first. A
//! qualified `Q.x` never takes a local binder, since no `let`, `where`, lambda
//! or `case` binder can be written with a qualifier. Resolving it by its bare
//! text let a local `reverse` capture `L.reverse`, and a local `foo` capture
//! `Main.foo`.
//!
//! A reference to a value a program module defines takes its definer's identity (rows 629 and
//! 642): `A.limit` and `B.limit` are two references when A and B each define `limit`, never one
//! id by their shared text, and one reference when A only re-exports B's.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use std::collections::{BTreeSet, HashMap};

use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::DesugarCtxChirho;
use crate::expr_chirho::{CoreIdChirho, ImportedReferenceChirho};

/// Which program modules define each imported value name, by the qualifier a reference writes:
/// `None` for an unqualified reference (rows 629 and 642). The driver builds it from the
/// module's imports and the origins their interfaces record. A value with no program definer,
/// such as a builtin a program module re-exports, is never recorded here.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportSuppliersChirho {
    suppliers_chirho: HashMap<(Option<String>, String), BTreeSet<String>>,
}

impl ImportSuppliersChirho {
    /// User-value spellings supplied by program modules, under any qualifier or alias.
    /// The producer excludes class selectors, constructors, builtins and dependency stubs.
    pub fn value_names_chirho(&self) -> impl Iterator<Item = &str> {
        self.suppliers_chirho
            .keys()
            .map(|(_, name_chirho)| name_chirho.as_str())
    }

    /// Record that program module `module_chirho` defines a value `name_chirho` reaches when
    /// written under `qualifier_chirho` (or unqualified).
    pub fn add_chirho(
        &mut self,
        qualifier_chirho: Option<&str>,
        name_chirho: &str,
        module_chirho: &str,
    ) {
        self.suppliers_chirho
            .entry((
                qualifier_chirho.map(str::to_string),
                name_chirho.to_string(),
            ))
            .or_default()
            .insert(module_chirho.to_string());
    }

    /// The identity of a reference written `qualifier.name` (or unqualified `name`), when it
    /// reaches a value a program module defines.
    pub fn reference_chirho(
        &self,
        qualifier_chirho: Option<&str>,
        name_chirho: &str,
    ) -> Option<ImportedReferenceChirho> {
        let suppliers_chirho = self.suppliers_chirho.get(&(
            qualifier_chirho.map(str::to_string),
            name_chirho.to_string(),
        ))?;
        Some(ImportedReferenceChirho {
            name_chirho: name_chirho.to_string(),
            defining_modules_chirho: suppliers_chirho.iter().cloned().collect(),
        })
    }
}

impl DesugarCtxChirho {
    /// The name a written reference resolves by, and the id it resolves to.
    ///
    /// - Unqualified: the scope chain.
    /// - Bound under its full spelling, as the seeded Data.Map and QualifiedDo
    ///   aliases are: that binding.
    /// - Qualified by the module's own name: its top-level binding, else a name
    ///   no scope binds, such as one of its own class methods.
    /// - Any other qualifier names an import. The linker addresses an import by
    ///   its bare export name, so the reference skips every scope. Beside a
    ///   top-level binding of the same name, the dictionary pass spells the
    ///   import's body apart (row 597), so both reach the natives.
    ///
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub(super) fn resolve_written_reference_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> (String, CoreIdChirho) {
        let text_chirho = name_chirho.text_chirho().to_string();
        let Some(qualifier_chirho) = name_chirho.qualifier_chirho() else {
            if self.lookup_scope_chirho(&text_chirho).is_none()
                && let Some(reference_chirho) = self
                    .import_suppliers_chirho
                    .reference_chirho(None, &text_chirho)
            {
                let id_chirho = self.resolve_imported_reference_chirho(reference_chirho);
                return (text_chirho, id_chirho);
            }
            let id_chirho = self.resolve_var_chirho(&text_chirho);
            return (text_chirho, id_chirho);
        };
        let full_name_chirho = name_chirho.full_name_chirho();
        if self.lookup_scope_chirho(&full_name_chirho).is_some() {
            let id_chirho = self.resolve_var_chirho(&full_name_chirho);
            return (full_name_chirho, id_chirho);
        }
        if let Some(id_chirho) = self.own_top_level_id_chirho(qualifier_chirho, &text_chirho) {
            return (text_chirho, id_chirho);
        }
        if let Some(reference_chirho) = self
            .import_suppliers_chirho
            .reference_chirho(Some(qualifier_chirho), &text_chirho)
        {
            let id_chirho = self.resolve_imported_reference_chirho(reference_chirho);
            return (text_chirho, id_chirho);
        }
        let id_chirho = self.resolve_unscoped_var_chirho(&text_chirho);
        (text_chirho, id_chirho)
    }

    /// The id of an imported user reference: one per identity, whatever text names it, and
    /// recorded with that identity for the dictionary pass and the multi-module link (row 629).
    /// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
    fn resolve_imported_reference_chirho(
        &mut self,
        reference_chirho: ImportedReferenceChirho,
    ) -> CoreIdChirho {
        if let Some(&id_chirho) = self.imported_reference_ids_chirho.get(&reference_chirho) {
            return id_chirho;
        }
        let id_chirho = self.fresh_id_chirho(&reference_chirho.name_chirho);
        self.imported_reference_ids_chirho
            .insert(reference_chirho.clone(), id_chirho);
        self.imported_references_chirho
            .insert(id_chirho, reference_chirho);
        id_chirho
    }

    /// The module's own top-level binding of `text_chirho`, when `qualifier_chirho` is the
    /// module's own name.
    fn own_top_level_id_chirho(
        &self,
        qualifier_chirho: &str,
        text_chirho: &str,
    ) -> Option<CoreIdChirho> {
        if self.current_module_name_chirho.as_deref() != Some(qualifier_chirho) {
            return None;
        }
        self.scope_chirho
            .first()
            .and_then(|scope_chirho| scope_chirho.get(text_chirho))
            .copied()
    }

    /// Whether written reference `name_chirho` names a binding this module owns. That is an
    /// unqualified name bound in scope, at top level or locally, or a name qualified by the
    /// module's own name that names its top-level binding. Every Prelude shortcut the
    /// desugarer takes for a written operator yields to such a binding (row 614).
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub(super) fn names_owned_binding_chirho(&self, name_chirho: &NameChirho) -> bool {
        match name_chirho.qualifier_chirho() {
            None => self
                .lookup_scope_chirho(name_chirho.text_chirho())
                .is_some(),
            Some(qualifier_chirho) => self
                .own_top_level_id_chirho(qualifier_chirho, name_chirho.text_chirho())
                .is_some(),
        }
    }

    /// Whether `id_chirho`, resolved under `name_chirho`, is a binding this module owns:
    /// one that some scope frame binds under that name (row 614).
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub(super) fn is_owned_binding_id_chirho(
        &self,
        name_chirho: &str,
        id_chirho: CoreIdChirho,
    ) -> bool {
        self.scope_chirho
            .iter()
            .any(|scope_chirho| scope_chirho.get(name_chirho) == Some(&id_chirho))
    }
}
