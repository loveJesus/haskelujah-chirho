// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An inferred top-level binding owns the dictionary parameters of its final scheme. Two
//! parameters of the same class retain their predicate indices; a shared type variable whose
//! top-level schemes disagree proves no ownership. Nested generalization never registers here.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use super::{InferCtxChirho, own_dictionary_key_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

impl InferCtxChirho {
    /// Captures and their owner keys follow the same substitution. Collapsing two variables
    /// intersects their claims, including empty veto maps; a concrete target owns no parameter.
    pub(super) fn substitute_inferred_dictionary_owners_chirho(
        &mut self,
        subst_chirho: &SubstChirho,
    ) {
        let mut updated_chirho: HashMap<TyVarChirho, HashMap<String, usize>> = HashMap::new();
        for (var_chirho, indices_chirho) in self.inferred_dictionary_owners_chirho.drain() {
            let TyChirho::VarChirho(target_chirho) =
                subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(var_chirho))
            else {
                continue;
            };
            updated_chirho
                .entry(target_chirho)
                .and_modify(|previous_chirho| {
                    previous_chirho.retain(|class_chirho, index_chirho| {
                        indices_chirho.get(class_chirho) == Some(index_chirho)
                    });
                })
                .or_insert(indices_chirho);
        }
        self.inferred_dictionary_owners_chirho = updated_chirho;
    }

    /// Record the final predicate order, after context reduction and defaulting. Every scheme
    /// mentioning the same variable must expose the same class at the same parameter index.
    /// Empty entries are retained: a sibling without that parameter vetoes a later claim.
    pub(super) fn record_inferred_dictionary_owners_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) {
        let mut mentioned_chirho: HashSet<TyVarChirho> = scheme_chirho
            .ty_chirho
            .free_vars_chirho()
            .into_iter()
            .collect();
        mentioned_chirho.extend(scheme_chirho.vars_chirho.iter().copied());
        for pred_chirho in &scheme_chirho.preds_chirho {
            mentioned_chirho.extend(pred_chirho.ty_chirho.free_vars_chirho());
            for extra_chirho in &pred_chirho.extra_tys_chirho {
                mentioned_chirho.extend(extra_chirho.free_vars_chirho());
            }
        }
        for var_chirho in mentioned_chirho {
            let mut indices_chirho: HashMap<String, Option<usize>> = HashMap::new();
            if scheme_chirho.vars_chirho.contains(&var_chirho) {
                for (index_chirho, pred_chirho) in scheme_chirho.preds_chirho.iter().enumerate() {
                    if pred_chirho.ty_chirho != TyChirho::VarChirho(var_chirho) {
                        continue;
                    }
                    indices_chirho
                        .entry(pred_chirho.class_name_chirho.clone())
                        .and_modify(|index_chirho| *index_chirho = None)
                        .or_insert(Some(index_chirho));
                }
            }
            let agreed_chirho: HashMap<String, usize> = indices_chirho
                .into_iter()
                .filter_map(|(class_chirho, index_chirho)| {
                    index_chirho.map(|index_chirho| (class_chirho, index_chirho))
                })
                .collect();
            self.inferred_dictionary_owners_chirho
                .entry(var_chirho)
                .and_modify(|previous_chirho| {
                    previous_chirho.retain(|class_chirho, index_chirho| {
                        agreed_chirho.get(class_chirho) == Some(index_chirho)
                    });
                })
                .or_insert(agreed_chirho);
        }
    }

    /// The exact dictionary parameter a generalized occurrence can use. Unregistered or
    /// conflicting ownership yields no proof, rather than the class's last dictionary.
    pub(super) fn inferred_dictionary_owner_key_chirho(
        &self,
        class_chirho: &str,
        var_chirho: TyVarChirho,
    ) -> Option<String> {
        self.inferred_dictionary_owners_chirho
            .get(&var_chirho)?
            .get(class_chirho)
            .copied()
            .map(|index_chirho| own_dictionary_key_chirho(Some(index_chirho)))
    }
}
