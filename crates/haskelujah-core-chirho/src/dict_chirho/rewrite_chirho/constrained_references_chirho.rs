// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dictionary arguments of constrained references: a call to a binding whose
//! scheme has predicates receives one dictionary per predicate, chosen from the
//! checker's evidence for that reference before any argument-shape guess.
//! These helpers moved here from `rewrite_chirho.rs`, which keeps only the
//! dispatch that calls them (gpt_chirho #27285). Two rules hold here: a call
//! receives a dictionary argument for EVERY predicate, an explicit lazy error
//! where none can be built, never a gap its next term argument would fill; and
//! a whole ground list key the checker proved is never replaced by a guess read
//! off the argument's syntax.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use super::{CoreExprChirho, CoreIdChirho, CoreLitChirho, DictPassCtxChirho};

impl DictPassCtxChirho {
    /// The head of a call to a constrained binding, with one dictionary argument
    /// per predicate in scheme order: the checker's evidence for the reference
    /// first, then the argument-shape key `syntactic_type_key_chirho` when the
    /// caller has one, then the enclosing binding's own dictionary for the
    /// class. A whole ground list key (`[Int]`, `[[Int]]`) is the checker's
    /// proof of the exact type: when it names no dictionary, neither later path
    /// may stand in for it, because the argument-shape guess reads the element
    /// off the syntax and collapses `[[Int]]` to `[Int]`. A predicate that no
    /// path serves gets `missing_dictionary_chirho`.
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
            let evidence_key_chirho = Self::take_reference_evidence_chirho(
                &mut reference_evidence_chirho,
                class_name_chirho,
            )
            .flatten();
            let evidence_dict_chirho = self.evidence_dict_for_class_chirho(
                class_name_chirho,
                evidence_key_chirho.as_deref(),
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
            );
            let proven_ground_chirho = evidence_key_chirho
                .as_deref()
                .is_some_and(Self::is_whole_list_key_chirho);
            let dict_id_chirho = evidence_dict_chirho
                .or_else(|| {
                    syntactic_type_key_chirho
                        .filter(|_| !proven_ground_chirho)
                        .and_then(|type_key_chirho| {
                            self.instance_dicts_chirho
                                .get(&(class_name_chirho.clone(), type_key_chirho.to_string()))
                                .copied()
                        })
                })
                .or_else(|| {
                    (!proven_ground_chirho)
                        .then(|| {
                            Self::fallback_dict_for_class_chirho(
                                class_name_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                            )
                        })
                        .flatten()
                });
            let dict_arg_chirho = match dict_id_chirho {
                Some(dict_id_chirho) => CoreExprChirho::VarChirho(dict_id_chirho),
                None => self.missing_dictionary_chirho(
                    class_name_chirho,
                    evidence_key_chirho.as_deref(),
                    written_id_chirho,
                ),
            };
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(dict_arg_chirho),
            };
        }
        result_chirho
    }

    /// The argument for a predicate that no dictionary serves: an explicit
    /// error naming the obligation and the binding that needs it. As any
    /// non-atomic argument it is lowered to a thunk, so a dictionary the callee
    /// never uses never fails. It used to be left out, and the call's next term
    /// argument then took the dictionary's place: T6c's `sameChirho nested`
    /// printed nothing more and exited 0.
    fn missing_dictionary_chirho(
        &self,
        class_name_chirho: &str,
        key_chirho: Option<&str>,
        written_id_chirho: CoreIdChirho,
    ) -> CoreExprChirho {
        let callee_chirho = self
            .names_chirho
            .get(&self.canonical_id_chirho(written_id_chirho))
            .map(String::as_str)
            .unwrap_or("a constrained binding");
        let obligation_chirho = match key_chirho {
            Some(key_chirho)
                if !haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho(
                    key_chirho,
                ) =>
            {
                format!("{class_name_chirho} {key_chirho}")
            }
            _ => class_name_chirho.to_string(),
        };
        CoreExprChirho::PrimOpChirho {
            name_chirho: "error".to_string(),
            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                format!(
                    "no instance dictionary for `{obligation_chirho}`, required by `{callee_chirho}`"
                ),
            ))],
        }
    }

    /// A whole list key the checker proved (`[Int]`, `[[Int]]`, `[Char]`),
    /// as opposed to the head key `[]` of a list over a variable.
    fn is_whole_list_key_chirho(key_chirho: &str) -> bool {
        key_chirho.len() > 2 && key_chirho.starts_with('[') && key_chirho.ends_with(']')
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
        // A whole list key (`[Int]`) keeps the head `[]` as its fallback, the
        // key a list reference carried before.
        if Self::should_normalize_instance_head_for_class_chirho(class_name_chirho)
            || Self::is_whole_list_key_chirho(key_chirho)
        {
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
