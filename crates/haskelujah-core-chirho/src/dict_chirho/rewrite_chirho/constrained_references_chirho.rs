// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dictionary arguments of constrained references: a call to a binding whose
//! scheme has predicates receives one dictionary per predicate, chosen from the
//! checker's evidence for that reference before any argument-shape guess.
//! These helpers moved here from `rewrite_chirho.rs`, which keeps only the
//! dispatch that calls them (gpt_chirho #27285, #27303). Two rules hold here: a
//! call, evidenced or not, receives a dictionary argument for EVERY predicate, an explicit lazy error
//! where none can be built, never a gap its next term argument would fill; and
//! a whole ground list key the checker proved is never replaced by a guess read
//! off the argument's syntax. The head replacement of a method occurrence moved
//! here too, since an `Eq` or `Ord` occurrence at such a key shares those
//! lookups: an argument-shape key may serve it only when it agrees with the
//! proof (`proven_list_dispatch_chirho`).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use super::{CoreExprChirho, CoreIdChirho, CoreLitChirho, DictPassCtxChirho};

/// The classes whose method occurrences `proven_list_dispatch_chirho` governs
/// (gpt_chirho #27488): their list dictionaries are ground specializations per
/// element type (`$fEq[Int]`, `$fOrd[Double]`), so a key read off the syntax
/// names another element type's dictionary. `Show` keeps its documented
/// dictionary path for shapes without a generated row (`show [Just RedChirho]`
/// prints GHC's answer through it); every other class keeps its path, unmeasured.
const PROVEN_LIST_CLASSES_CHIRHO: [&str; 2] = ["Eq", "Ord"];

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

    /// A constrained call reached with no evidence for its reference, only the
    /// surrounding call's type key (`try_rewrite_typed_dict_param_arg_chirho`;
    /// an infix use mints no reference occurrence): each predicate's dictionary
    /// at that key, else the enclosing binding's own. `None` when NONE is
    /// served, so the general evidence-first path decides; otherwise EVERY slot
    /// is filled, a missing one with `missing_dictionary_chirho`. A partial
    /// spine let the term arguments slide into the dictionary slots: `3.0
    /// `halfChirho` 2.0` with `(Show a, Fractional a)` under an `Int` key printed
    /// `$PAP halfChirho …` where GHC prints `"1.5"`, in either predicate order.
    /// The key comes from the surrounding call, so a served dictionary can be
    /// at the wrong type; giving infix uses reference evidence is the cure.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn typed_reference_dicts_chirho(
        &self,
        written_id_chirho: CoreIdChirho,
        classes_chirho: &[String],
        type_key_chirho: &str,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreExprChirho> {
        let slots_chirho: Vec<Option<CoreIdChirho>> = classes_chirho
            .iter()
            .map(|class_name_chirho| {
                self.instance_dicts_chirho
                    .get(&(class_name_chirho.clone(), type_key_chirho.to_string()))
                    .copied()
                    .or_else(|| {
                        Self::fallback_dict_for_class_chirho(
                            class_name_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                        )
                    })
            })
            .collect();
        if slots_chirho.iter().all(Option::is_none) {
            return None;
        }
        let mut result_chirho =
            CoreExprChirho::VarChirho(self.canonical_id_chirho(written_id_chirho));
        for (class_name_chirho, slot_chirho) in classes_chirho.iter().zip(slots_chirho) {
            let dict_arg_chirho = match slot_chirho {
                Some(dict_id_chirho) => CoreExprChirho::VarChirho(dict_id_chirho),
                None => self.missing_dictionary_chirho(
                    class_name_chirho,
                    Some(type_key_chirho),
                    written_id_chirho,
                ),
            };
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(dict_arg_chirho),
            };
        }
        Some(result_chirho)
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

    /// The spellings one evidence key can have in the registry, exact first:
    /// the key, then the same key with each whole-word `String` spelled `[Char]`
    /// (`string_spelled_out_chirho`), since an instance head is registered as it
    /// displays (`instance TagChirho [[Char]]` is `$fTagChirho[[Char]]`) while
    /// the evidence spells a nested `String` as `String` (`[String]`), the
    /// spelling `Show`'s own rows use. For an exact method row
    /// (`$prim_C_m_<key>`), where the head key never stood.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn key_spellings_chirho(key_chirho: &str) -> Vec<String> {
        let spelled_chirho = Self::string_spelled_out_chirho(key_chirho);
        if spelled_chirho == key_chirho {
            vec![key_chirho.to_string()]
        } else {
            vec![key_chirho.to_string(), spelled_chirho]
        }
    }

    /// `key_spellings_chirho`, then the head key: for a higher-kinded class
    /// (`Maybe Int` → `Maybe`), as before, and for a whole list key (`[Int]` →
    /// `[]`), the key a list carried before. For dictionary lookups.
    pub(super) fn registry_spellings_chirho(
        class_name_chirho: &str,
        key_chirho: &str,
    ) -> Vec<String> {
        let mut spellings_chirho = Self::key_spellings_chirho(key_chirho);
        if Self::should_normalize_instance_head_for_class_chirho(class_name_chirho)
            || Self::is_whole_list_key_chirho(key_chirho)
        {
            let normalized_chirho = Self::normalize_instance_head_key_chirho(key_chirho);
            if !spellings_chirho.contains(&normalized_chirho) {
                spellings_chirho.push(normalized_chirho);
            }
        }
        spellings_chirho
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
        self.registry_dict_chirho(class_name_chirho, key_chirho, local_instance_dicts_chirho)
    }

    /// The dictionary the registry holds for `key_chirho` under any of its
    /// spellings (`registry_spellings_chirho`), the enclosing binding's local
    /// dictionaries first.
    fn registry_dict_chirho(
        &self,
        class_name_chirho: &str,
        key_chirho: &str,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreIdChirho> {
        Self::registry_spellings_chirho(class_name_chirho, key_chirho)
            .into_iter()
            .find_map(|candidate_chirho| {
                let lookup_chirho = (class_name_chirho.to_string(), candidate_chirho);
                local_instance_dicts_chirho
                    .get(&lookup_chirho)
                    .or_else(|| self.instance_dicts_chirho.get(&lookup_chirho))
                    .copied()
            })
    }

    /// Evidence-threading P1/P2b: if `head_id` is a method OCCURRENCE id, return
    /// the replacement head expression — the evidence-dispatched `$prim` row when
    /// one exists; for an `Eq` or `Ord` method at a proven whole list key, the
    /// dispatch of `proven_list_dispatch_chirho`; else the existing keyed
    /// selector/instance dispatch driven by the PROVEN type key, else the
    /// canonical shared id (today's path). `syntactic_key_chirho` reads the
    /// call's argument-shape key, and only a proven list key asks for it.
    pub(super) fn occurrence_head_replacement_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        syntactic_key_chirho: &dyn Fn() -> Option<String>,
    ) -> Option<CoreExprChirho> {
        let (method_name_chirho, canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&head_id_chirho)?;
        // A constrained reference with evidence keeps its occurrence id: the
        // dictionary-parameter call-site path reads the evidence off it and
        // canonicalizes the head itself.
        // workflow: language-features-chirho/dictionary-evidence-chirho
        if self.reference_evidence_chirho.contains_key(&head_id_chirho) {
            return None;
        }
        if let Some((class_name_chirho, ty_key_chirho)) =
            self.occurrence_evidence_chirho.get(&head_id_chirho)
        {
            // A literal's wrapper takes the selector form: the simplifier
            // reduces `$sel_Num_fromInteger dict lit` to the literal, which
            // every backend already handles; the prim row is an interpreter
            // shape (Cranelift returned garbage for `Box 42` through it).
            // workflow: language-features-chirho/dictionary-evidence-chirho
            let literal_wrapper_chirho =
                matches!(method_name_chirho.as_str(), "fromInteger" | "fromString");
            let prim_name_chirho = format!(
                "$prim_{}_{}_{}",
                class_name_chirho, method_name_chirho, ty_key_chirho
            );
            if !literal_wrapper_chirho {
                if let Some(prim_id_chirho) = self.find_global_id_by_name_chirho(&prim_name_chirho)
                {
                    return Some(CoreExprChirho::VarChirho(prim_id_chirho));
                }
            }
            // A proven list key that names no row never comes back as the
            // canonical id, whose recursion re-keyed the call off the syntax.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            if let Some(selector_id_chirho) = self.proven_list_selector_chirho(
                *canonical_id_chirho,
                class_name_chirho,
                ty_key_chirho,
            ) {
                return Some(self.proven_list_dispatch_chirho(
                    head_id_chirho,
                    selector_id_chirho,
                    class_name_chirho,
                    ty_key_chirho,
                    syntactic_key_chirho,
                    local_instance_dicts_chirho,
                ));
            }
            // Keyed fallback: reuse the existing selector/instance dispatch with
            // the proven key so selector-backed instances (Eq/Num Int, ...) work.
            return Some(self.try_rewrite_method_var_chirho(
                *canonical_id_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                Some(ty_key_chirho),
            ));
        }
        Some(CoreExprChirho::VarChirho(*canonical_id_chirho))
    }

    /// The selector an occurrence dispatches through when R2 governs it: an
    /// `Eq` or `Ord` method (`PROVEN_LIST_CLASSES_CHIRHO`) at a whole list key,
    /// whose canonical name selects through that same class.
    fn proven_list_selector_chirho(
        &self,
        canonical_id_chirho: CoreIdChirho,
        class_name_chirho: &str,
        key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        if !PROVEN_LIST_CLASSES_CHIRHO.contains(&class_name_chirho)
            || !Self::is_whole_list_key_chirho(key_chirho)
        {
            return None;
        }
        let name_chirho = self.names_chirho.get(&canonical_id_chirho)?;
        let (selector_class_chirho, selector_id_chirho) =
            self.class_method_selector_for_name_chirho(name_chirho)?;
        (selector_class_chirho == class_name_chirho).then_some(selector_id_chirho)
    }

    /// The head of an `Eq` or `Ord` method occurrence at a whole list key the
    /// checker proved, once that key names no method row (R2, gpt_chirho
    /// #27481, #27488): the selector applied to the dictionary the registry
    /// holds for the proven key; else to the one for the call's argument-shape
    /// key, only when that key agrees with the proof
    /// (`key_agrees_with_proof_chirho`); else to `missing_dictionary_chirho`,
    /// which names the obligation and fails only if the comparison is forced.
    /// The canonical id this used to return recursed with no evidence and was
    /// keyed off its first argument: `[[1]] == [[2 :: Int]]` took `[Int]` and
    /// printed `True`, and inside `sameChirho :: Eq a => ..` the enclosing
    /// `Eq a` dictionary answered for `Eq [[Int]]`. Neither may serve now.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn proven_list_dispatch_chirho(
        &self,
        occurrence_id_chirho: CoreIdChirho,
        selector_id_chirho: CoreIdChirho,
        class_name_chirho: &str,
        proven_key_chirho: &str,
        syntactic_key_chirho: &dyn Fn() -> Option<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> CoreExprChirho {
        let dict_arg_chirho = self
            .registry_dict_chirho(
                class_name_chirho,
                proven_key_chirho,
                local_instance_dicts_chirho,
            )
            .or_else(|| {
                syntactic_key_chirho()
                    .filter(|syntactic_chirho| {
                        Self::key_agrees_with_proof_chirho(proven_key_chirho, syntactic_chirho)
                    })
                    .and_then(|syntactic_chirho| {
                        self.registry_dict_chirho(
                            class_name_chirho,
                            &syntactic_chirho,
                            local_instance_dicts_chirho,
                        )
                    })
            })
            .map(CoreExprChirho::VarChirho)
            .unwrap_or_else(|| {
                self.missing_dictionary_chirho(
                    class_name_chirho,
                    Some(proven_key_chirho),
                    occurrence_id_chirho,
                )
            });
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_id_chirho)),
            arg_chirho: Box::new(dict_arg_chirho),
        }
    }

    /// Whether an argument-shape key may stand in for a proven one: the same
    /// key once both are spelled as the engine's rows are
    /// (`engine_spelled_key_chirho`). `[Int]` agrees with a proven `[Integer]`
    /// (the defaulted `[1] == [2]`) and `[[Char]]` with `[String]`; `[Int]`
    /// never agrees with `[[Integer]]`, whose nesting the syntax collapsed.
    fn key_agrees_with_proof_chirho(proven_key_chirho: &str, syntactic_key_chirho: &str) -> bool {
        Self::engine_spelled_key_chirho(proven_key_chirho)
            == Self::engine_spelled_key_chirho(syntactic_key_chirho)
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
