// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The one link decision both multi-module merges make for an imported user reference (row 629).
//!
//! An imported reference names the supplying module's own source binding of its name. A
//! compiler binding is never a candidate. When no supplier defines the name, the reference is a
//! re-export, and until row 642 records where a re-exported value is defined, it links only to
//! the program's one source binding of that name. Every other case is refused, by name: two
//! suppliers defining their own, no definition at all, or several possible origins. Nothing
//! here falls back to the first binding of a name.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use std::collections::{BTreeMap, BTreeSet, HashMap};

#[cfg(test)]
#[path = "import_links_tests_chirho.rs"]
mod tests_chirho;

use haskelujah_core_chirho::expr_chirho::{CoreBindingChirho, CoreIdChirho};
use haskelujah_core_chirho::{CoreProvenanceChirho, ImportedReferenceChirho};

/// The program's source bindings by module, and its imported user references, gathered module
/// by module once each module's ids sit in the program's id space.
#[derive(Debug, Default)]
pub(crate) struct ImportLinkerChirho {
    /// Each module's source bindings, by name. Compiler bindings are left out.
    sources_chirho: BTreeMap<String, BTreeMap<String, CoreIdChirho>>,
    /// The module of every source binding.
    source_modules_chirho: HashMap<CoreIdChirho, String>,
    /// Every imported user reference, with its identity.
    references_chirho: BTreeMap<CoreIdChirho, ImportedReferenceChirho>,
}

impl ImportLinkerChirho {
    /// Join one module: its bindings and provenance, already moved to the program's ids.
    pub(crate) fn add_module_chirho(
        &mut self,
        module_chirho: &str,
        bindings_chirho: &[CoreBindingChirho],
        provenance_chirho: &CoreProvenanceChirho,
    ) {
        for binding_chirho in bindings_chirho {
            let binder_chirho = &binding_chirho.binder_chirho;
            if provenance_chirho
                .compiler_bindings_chirho
                .contains(&binder_chirho.id_chirho)
            {
                continue;
            }
            self.sources_chirho
                .entry(module_chirho.to_string())
                .or_default()
                .entry(binder_chirho.name_chirho.clone())
                .or_insert(binder_chirho.id_chirho);
            self.source_modules_chirho
                .insert(binder_chirho.id_chirho, module_chirho.to_string());
        }
        self.references_chirho.extend(
            provenance_chirho
                .imported_references_chirho
                .iter()
                .map(|(id_chirho, reference_chirho)| (*id_chirho, reference_chirho.clone())),
        );
    }

    /// Whether `id_chirho` is an imported user reference, which only this decision links.
    pub(crate) fn is_imported_reference_chirho(&self, id_chirho: &CoreIdChirho) -> bool {
        self.references_chirho.contains_key(id_chirho)
    }

    /// The binding each imported reference names, as a renaming of its id; or every refusal,
    /// one per line, in a stable order.
    pub(crate) fn link_chirho(&self) -> Result<HashMap<CoreIdChirho, CoreIdChirho>, String> {
        let mut remap_chirho = HashMap::new();
        let mut refusals_chirho = BTreeSet::new();
        for (id_chirho, reference_chirho) in &self.references_chirho {
            match self.target_chirho(reference_chirho) {
                Ok(target_chirho) => {
                    remap_chirho.insert(*id_chirho, target_chirho);
                }
                Err(refusal_chirho) => {
                    refusals_chirho.insert(refusal_chirho);
                }
            }
        }
        if refusals_chirho.is_empty() {
            Ok(remap_chirho)
        } else {
            Err(refusals_chirho.into_iter().collect::<Vec<_>>().join("\n"))
        }
    }

    /// The binding `reference_chirho` names, or why the link would guess.
    fn target_chirho(
        &self,
        reference_chirho: &ImportedReferenceChirho,
    ) -> Result<CoreIdChirho, String> {
        let name_chirho = &reference_chirho.name_chirho;
        let suppliers_chirho = reference_chirho.suppliers_chirho.join(", ");
        let defined_by_suppliers_chirho: Vec<CoreIdChirho> = reference_chirho
            .suppliers_chirho
            .iter()
            .filter_map(|supplier_chirho| {
                self.sources_chirho
                    .get(supplier_chirho)?
                    .get(name_chirho)
                    .copied()
            })
            .collect();
        let definers_chirho: Vec<&String> = self
            .sources_chirho
            .iter()
            .filter(|(_, bindings_chirho)| bindings_chirho.contains_key(name_chirho))
            .map(|(module_chirho, _)| module_chirho)
            .collect();
        match (
            reference_chirho.suppliers_chirho.len(),
            defined_by_suppliers_chirho.as_slice(),
            definers_chirho.as_slice(),
        ) {
            // A direct import: the supplier's own binding.
            (1, [target_chirho], _) => Ok(*target_chirho),
            (_, [_, _, ..], _) => Err(format!(
                "cannot link `{name_chirho}`: it is imported unqualified from {suppliers_chirho}, \
                 and each of them defines its own (row 629)"
            )),
            // A re-export, or one name through several imports: one possible binding.
            (_, _, [definer_chirho]) => Ok(self.sources_chirho[*definer_chirho][name_chirho]),
            (_, _, []) => Err(format!(
                "cannot link `{name_chirho}`, imported from {suppliers_chirho}: no module of the \
                 program defines it (row 629)"
            )),
            (_, _, definers_chirho) => Err(format!(
                "cannot link `{name_chirho}`, imported from {suppliers_chirho}: it is defined by \
                 {}, and without the re-export's origin (row 642) the link would guess (row 629)",
                definers_chirho
                    .iter()
                    .map(|definer_chirho| definer_chirho.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }

    /// `names_chirho` with every source binding spelled by its module, for deduplication keys:
    /// bodies that name different modules' bindings never share a key, while compiler bindings
    /// and builtin references keep their names, so per-module Prelude copies still merge.
    pub(crate) fn identity_names_chirho(
        &self,
        names_chirho: &HashMap<CoreIdChirho, String>,
    ) -> HashMap<CoreIdChirho, String> {
        names_chirho
            .iter()
            .map(|(id_chirho, name_chirho)| {
                let spelled_chirho = match self.source_modules_chirho.get(id_chirho) {
                    Some(module_chirho) => format!("{module_chirho}::{name_chirho}"),
                    None => name_chirho.clone(),
                };
                (*id_chirho, spelled_chirho)
            })
            .collect()
    }
}
