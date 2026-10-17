// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dictionary arguments of constrained references: a call to a binding whose
//! scheme has predicates receives one dictionary per predicate, chosen from the
//! checker's evidence for that reference before any argument-shape guess.
//! These helpers moved here unchanged from `rewrite_chirho.rs`, which keeps only
//! the dispatch that calls them (gpt_chirho #27285).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use super::{CoreExprChirho, CoreIdChirho, DictPassCtxChirho};

impl DictPassCtxChirho {
    /// The head of a call to a constrained binding, with one dictionary argument
    /// per predicate in scheme order: the checker's evidence for the reference
    /// first, then the argument-shape key `syntactic_type_key_chirho` when the
    /// caller has one, then the enclosing binding's own dictionary for the
    /// class. A predicate that no path serves gets no argument.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn apply_reference_dicts_chirho(
        &self,
        written_id_chirho: CoreIdChirho,
        classes_chirho: &[String],
        syntactic_type_key_chirho: Option<&str>,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> CoreExprChirho {
        let mut reference_evidence_chirho = self
            .reference_evidence_chirho
            .get(&written_id_chirho)
            .cloned()
            .unwrap_or_default();
        let mut result_chirho =
            CoreExprChirho::VarChirho(self.canonical_id_chirho(written_id_chirho));
        for class_name_chirho in classes_chirho {
            let evidence_dict_chirho = Self::take_reference_evidence_chirho(
                &mut reference_evidence_chirho,
                class_name_chirho,
            )
            .and_then(|key_chirho| {
                self.evidence_dict_for_class_chirho(
                    class_name_chirho,
                    key_chirho.as_deref(),
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                )
            });
            let dict_id_chirho = evidence_dict_chirho
                .or_else(|| {
                    syntactic_type_key_chirho.and_then(|type_key_chirho| {
                        self.instance_dicts_chirho
                            .get(&(class_name_chirho.clone(), type_key_chirho.to_string()))
                            .copied()
                    })
                })
                .or_else(|| {
                    Self::fallback_dict_for_class_chirho(
                        class_name_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                    )
                });
            if let Some(dict_id_chirho) = dict_id_chirho {
                result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                };
            }
        }
        result_chirho
    }

    /// The dictionary a constrained reference's predicate is served by, from
    /// the checker's evidence for that position: an instance dictionary for a
    /// proven key, the enclosing binding's own parameter for
    /// `OWN_DICTIONARY_KEY_CHIRHO`, and nothing when nothing was proved.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn evidence_dict_for_class_chirho(
        &self,
        class_name_chirho: &str,
        ty_key_chirho: Option<&str>,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreIdChirho> {
        let key_chirho = ty_key_chirho?;
        if haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho(key_chirho) {
            return Self::own_dict_chirho(
                class_name_chirho,
                key_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
            );
        }
        let mut candidate_keys_chirho = vec![key_chirho.to_string()];
        if Self::should_normalize_instance_head_for_class_chirho(class_name_chirho) {
            let normalized_chirho = Self::normalize_instance_head_key_chirho(key_chirho);
            if normalized_chirho != key_chirho {
                candidate_keys_chirho.push(normalized_chirho);
            }
        }
        candidate_keys_chirho
            .into_iter()
            .find_map(|candidate_chirho| {
                let lookup_chirho = (class_name_chirho.to_string(), candidate_chirho);
                local_instance_dicts_chirho
                    .get(&lookup_chirho)
                    .or_else(|| self.instance_dicts_chirho.get(&lookup_chirho))
                    .copied()
            })
    }

    /// The binding's own dictionary parameter an own-parameter key names: the
    /// parameter of that predicate index when the key carries one, else the
    /// binding's parameter for the class (superclass extractions included).
    pub(super) fn own_dict_chirho(
        class_name_chirho: &str,
        key_chirho: &str,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreIdChirho> {
        dict_vars_chirho.get(key_chirho).copied().or_else(|| {
            Self::fallback_dict_for_class_chirho(
                class_name_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
            )
        })
    }

    /// Take the evidence for the next predicate of `class_name` from a
    /// reference's evidence list (scheme order), if the checker recorded one.
    pub(super) fn take_reference_evidence_chirho(
        evidence_chirho: &mut Vec<(String, Option<String>)>,
        class_name_chirho: &str,
    ) -> Option<Option<String>> {
        let position_chirho = evidence_chirho
            .iter()
            .position(|(class_chirho, _key_chirho)| class_chirho == class_name_chirho)?;
        Some(evidence_chirho.remove(position_chirho).1)
    }

    pub(super) fn fallback_dict_for_class_chirho(
        class_name_chirho: &str,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreIdChirho> {
        if evidence_classes_chirho.contains(class_name_chirho) {
            dict_vars_chirho.get(class_name_chirho).copied()
        } else {
            None
        }
    }

    /// Normalize a value-shaped type key (`Maybe Int`, `Either e a`, `[Char]`)
    /// to the instance head key (`Maybe`, `Either`, `[]`) used to name the
    /// generated per-type instance bodies. Non-higher-kinded keys pass through.
    pub(super) fn normalize_instance_head_key_chirho(value_key_chirho: &str) -> String {
        let trimmed_chirho = value_key_chirho.trim();
        if trimmed_chirho.starts_with('[') {
            return "[]".to_string();
        }
        match trimmed_chirho.split_whitespace().next() {
            Some(head_chirho) => head_chirho.to_string(),
            None => trimmed_chirho.to_string(),
        }
    }

    pub(super) fn should_normalize_instance_head_for_class_chirho(class_name_chirho: &str) -> bool {
        matches!(
            class_name_chirho,
            "Functor"
                | "Applicative"
                | "Monad"
                | "Foldable"
                | "Traversable"
                | "Alternative"
                | "MonadPlus"
                | "MonadFix"
        )
    }
}
