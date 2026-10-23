// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Where a proven method occurrence may land (gpt_chirho #28111). Three rules
//! hold here, beside the two entries that dispatch an occurrence
//! (`occurrence_head_replacement_chirho`, `try_rewrite_method_var_chirho`):
//! - Self agreement: an occurrence the checker proved only by its head (`[]`
//!   for `[a]`, `TreeChirho` for `TreeChirho a`) takes the enclosing
//!   instance's own dictionary when the caller's key has that head and names a
//!   dictionary this binding holds locally. That is the dictionary the call's
//!   syntax found before proofs (`describeChirho xs` in
//!   `instance DescribableChirho a => DescribableChirho [a]`).
//! - No ambient answer: the entries never serve a proof that is not the
//!   binding's own dictionary from the context dictionary. Unresolved, the
//!   occurrence returns to its canonical method and the caller's own route.
//!   These two mechanisms answer the same recursive calls, so each masks the
//!   other; only reverting both brings back the context dictionary.
//! - Context-leading rows: a `$prim` row whose instance has a context takes
//!   those dictionaries first (`\$dShow -> \_ -> 7`), so a proof is served by
//!   it only with each one applied. Rows the pass generates for ground keys
//!   (`$prim_C_m_[Int]`) take none and are served as they are.
//!
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use std::collections::HashMap;

use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, PredChirho};
use haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::{CoreExprChirho, CoreIdChirho, CoreModuleChirho, DictPassCtxChirho};

impl DictPassCtxChirho {
    /// The enclosing instance's own dictionary for an occurrence proved only
    /// by its head: the class's selector applied to the dictionary this
    /// binding holds under the caller's candidate key, when that key is longer
    /// than the proof and has the proof as its head. A `[b]` or ground key is
    /// no local dictionary here, so nothing else borrows the self dictionary.
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    pub(super) fn agreeing_self_dictionary_chirho(
        &self,
        occurrence_id_chirho: CoreIdChirho,
        candidate_key_chirho: &dyn Fn() -> Option<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (_method_name_chirho, canonical_id_chirho) = self
            .method_occurrence_canon_chirho
            .get(&occurrence_id_chirho)?;
        let (class_name_chirho, proven_key_chirho) =
            self.occurrence_evidence_chirho.get(&occurrence_id_chirho)?;
        if !Self::is_head_only_key_chirho(proven_key_chirho) {
            return None;
        }
        let candidate_chirho = candidate_key_chirho()?;
        if candidate_chirho == *proven_key_chirho
            || Self::normalize_instance_head_key_chirho(&candidate_chirho) != *proven_key_chirho
        {
            return None;
        }
        let self_dict_chirho =
            local_instance_dicts_chirho.get(&(class_name_chirho.clone(), candidate_chirho))?;
        let name_chirho = self.names_chirho.get(canonical_id_chirho)?;
        let (selector_class_chirho, selector_id_chirho) =
            self.class_method_selector_for_name_chirho(name_chirho)?;
        (selector_class_chirho == *class_name_chirho).then(|| CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(*self_dict_chirho)),
        })
    }

    /// A key that names only a type's head: `[]`, the checker's key for a list
    /// over a variable, or a bare constructor. Never a whole spelling, a tuple
    /// or the binding's own dictionary.
    fn is_head_only_key_chirho(key_chirho: &str) -> bool {
        key_chirho == "[]"
            || !(key_chirho.starts_with('[')
                || key_chirho.starts_with('(')
                || key_chirho.contains(' ')
                || is_own_dictionary_key_chirho(key_chirho))
    }

    /// Whether a proof key may ever be served by the ambient context
    /// dictionary: only the binding's own dictionary key is.
    pub(super) fn proof_admits_context_chirho(key_chirho: &str) -> bool {
        is_own_dictionary_key_chirho(key_chirho)
    }

    /// The `$prim_<class>_<method>_<key>` row a proof names, as the method it
    /// stands for. A context-leading row is served only with every context
    /// dictionary applied, in context order, each predicate one-parameter,
    /// ground and registered exactly; otherwise it is not served at all and
    /// the caller's keyed route decides.
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    pub(super) fn proof_row_chirho(&self, row_name_chirho: &str) -> Option<CoreExprChirho> {
        let row_id_chirho = self.find_global_id_by_name_chirho(row_name_chirho)?;
        let Some(context_chirho) = self.context_leading_rows_chirho.get(&row_id_chirho) else {
            return Some(CoreExprChirho::VarChirho(row_id_chirho));
        };
        context_chirho.iter().try_fold(
            CoreExprChirho::VarChirho(row_id_chirho),
            |row_chirho, pred_chirho| {
                if !pred_chirho.extra_tys_chirho.is_empty()
                    || pred_chirho.ty_chirho.contains_var_chirho()
                {
                    return None;
                }
                let dict_id_chirho = self.instance_dicts_chirho.get(&(
                    pred_chirho.class_name_chirho.clone(),
                    format!("{}", pred_chirho.ty_chirho),
                ))?;
                Some(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(row_chirho),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                })
            },
        )
    }

    /// Record, before any binding is rewritten, each module `$prim` row whose
    /// instance has a context, with that context: exactly the rows the pass
    /// gives leading dictionary parameters, whatever order it rewrites them in.
    pub(super) fn record_context_leading_rows_chirho(
        &mut self,
        module_chirho: &CoreModuleChirho,
        class_env_chirho: &ClassEnvChirho,
    ) {
        for binding_chirho in &module_chirho.bindings_chirho {
            let Some((class_name_chirho, _method_name_chirho, type_key_chirho)) =
                self.parse_prim_binding_info_chirho(&binding_chirho.binder_chirho.name_chirho)
            else {
                continue;
            };
            if let Some(context_chirho) = Self::conditional_context_preds_chirho(
                class_env_chirho,
                &class_name_chirho,
                &type_key_chirho,
            ) {
                self.context_leading_rows_chirho
                    .insert(binding_chirho.binder_chirho.id_chirho, context_chirho);
            }
        }
    }

    /// The context of the instance a `$prim` row belongs to, when it has one:
    /// the first instance with a context whose head is the row's key, or whose
    /// head is `[a]` for any list key (its ground rows). This one rule both
    /// gives a module row its leading dictionary parameters and marks it.
    pub(super) fn conditional_context_preds_chirho(
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        parsed_type_key_chirho: &str,
    ) -> Option<Vec<PredChirho>> {
        class_env_chirho
            .instances_chirho
            .get(class_name_chirho)?
            .iter()
            .find(|inst_chirho| {
                !inst_chirho.context_chirho.is_empty()
                    && (format!("{}", inst_chirho.head_ty_chirho) == parsed_type_key_chirho
                        || matches!(
                            &inst_chirho.head_ty_chirho,
                            TyChirho::ListChirho(elem_ty_chirho)
                                if matches!(elem_ty_chirho.as_ref(), TyChirho::VarChirho(_))
                                    && parsed_type_key_chirho.starts_with('[')
                                    && parsed_type_key_chirho.ends_with(']')
                        ))
            })
            .map(|inst_chirho| inst_chirho.context_chirho.clone())
    }
}

#[cfg(test)]
#[path = "occurrence_rows_tests_chirho.rs"]
mod tests_chirho;
