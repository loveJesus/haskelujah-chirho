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
//! proof (`proven_list_dispatch_chirho`), at every entry that dispatches a
//! method occurrence (`proven_list_occurrence_chirho`).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, PredChirho};
use haskelujah_typing_chirho::infer_chirho::ReferenceEvidenceChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{CoreExprChirho, CoreIdChirho, CoreLitChirho, DictPassCtxChirho};

/// How deep a conditional dictionary's context may nest before the pass gives up on it.
const CONDITIONAL_DICTIONARY_DEPTH_CHIRHO: usize = 16;

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
            let proof_chirho = Self::take_reference_evidence_chirho(
                &mut reference_evidence_chirho,
                class_name_chirho,
            );
            let evidence_key_chirho = proof_chirho
                .as_ref()
                .and_then(|proof_chirho| proof_chirho.ty_key_chirho.clone());
            // A displaced compiler class's ground proof takes its own namespace's dictionary
            // (row 692, C10).
            let dictionary_class_chirho = proof_chirho
                .as_ref()
                .and_then(|proof_chirho| proof_chirho.ground_pred_chirho.as_ref())
                .map_or(class_name_chirho.as_str(), |predicate_chirho| {
                    self.dispatch_class_name_chirho(
                        class_name_chirho,
                        predicate_chirho.class_identity_chirho(),
                    )
                });
            let evidence_dict_chirho = self.evidence_dict_for_class_chirho(
                dictionary_class_chirho,
                evidence_key_chirho.as_deref(),
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
            );
            let proven_ground_chirho = evidence_key_chirho
                .as_deref()
                .is_some_and(Self::is_whole_list_key_chirho);
            // A ground application the checker proved (`BoxChirho Int`) is served by its
            // conditional instance's factory before any guess (row 694).
            // workflow: compiler-pipeline-chirho/deriving-strategies-chirho
            let conditional_chirho = || {
                proof_chirho
                    .as_ref()
                    .and_then(|proof_chirho| proof_chirho.applied_ty_chirho.as_ref())
                    .and_then(|applied_chirho| {
                        self.conditional_dictionary_chirho(
                            class_name_chirho,
                            applied_chirho,
                            local_instance_dicts_chirho,
                        )
                    })
            };
            // A multi-parameter predicate the checker proved at a complete ground head takes the
            // one registry entry whose instance matches that head, before any route keyed by its
            // first argument (row 694, #31909).
            let head_dictionary_chirho = proof_chirho
                .as_ref()
                .and_then(|proof_chirho| proof_chirho.applied_head_chirho.as_ref())
                .and_then(|head_chirho| {
                    self.head_dictionary_chirho(
                        class_name_chirho,
                        head_chirho,
                        local_instance_dicts_chirho,
                    )
                });
            let dict_arg_chirho = head_dictionary_chirho
                .or_else(|| evidence_dict_chirho.map(CoreExprChirho::VarChirho))
                .or_else(conditional_chirho)
                .or_else(|| {
                    syntactic_type_key_chirho
                        .filter(|_| !proven_ground_chirho)
                        .and_then(|type_key_chirho| {
                            self.instance_dicts_chirho
                                .get(&(class_name_chirho.clone(), type_key_chirho.to_string()))
                                .copied()
                        })
                        .map(CoreExprChirho::VarChirho)
                })
                // A ground goal of a conditional instance (S1), after every keyed route and
                // before the ambient one.
                // workflow: language-features-chirho/dictionary-evidence-chirho
                .or_else(|| {
                    self.ground_goal_dict_chirho(written_id_chirho, class_name_chirho)
                        .map(CoreExprChirho::VarChirho)
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
                        .map(CoreExprChirho::VarChirho)
                })
                .unwrap_or_else(|| {
                    self.note_unserved_slot_chirho(written_id_chirho, class_name_chirho);
                    self.missing_dictionary_chirho(
                        class_name_chirho,
                        evidence_key_chirho.as_deref(),
                        written_id_chirho,
                    )
                });
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
        // A GND instance over a parametric newtype has a dictionary only through its factory,
        // at a ground application whose context dictionaries exist; a use that needs one the
        // factory cannot give is refused when the module compiles (row 694, P1, P2).
        // workflow: compiler-pipeline-chirho/deriving-strategies-chirho
        if let Some(key_chirho) = key_chirho
            && self
                .gnd_parametric_chirho
                .contains(&(class_name_chirho.to_string(), key_chirho.to_string()))
        {
            let reason_chirho = self
                .gnd_factory_gaps_chirho
                .get(&(class_name_chirho.to_string(), key_chirho.to_string()))
                .map_or_else(
                    || "whose context dictionaries this pass cannot build".to_string(),
                    |gap_chirho| {
                        format!("whose dictionary factory this pass refuses: {gap_chirho}")
                    },
                );
            self.gnd_unbuilt_uses_chirho.borrow_mut().insert(format!(
                "deriving {class_name_chirho} for {key_chirho} through the newtype strategy: `{callee_chirho}` needs its dictionary at an application of the newtype {reason_chirho}"
            ));
        }
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
    pub(in crate::dict_chirho) fn evidence_dict_for_class_chirho(
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
    /// dispatch of `proven_list_occurrence_chirho`; else the existing keyed
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
        // `(^^)` is a constrained Prelude function, not a class method. Its occurrence still
        // shares this canonicalization path because desugaring gives every constrained reference
        // an occurrence id. Consume its full two-predicate proof before the single-method proof
        // path can restore the missing canonical binding (row 577).
        // workflow: language-features-chirho/dictionary-evidence-chirho
        if let Some(power_chirho) = self.evidenced_fractional_power_chirho(head_id_chirho) {
            return Some(power_chirho);
        }
        let (method_name_chirho, canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&head_id_chirho)?;
        // A constrained reference with evidence keeps its occurrence id only when
        // the canonical target takes dictionary parameters; a builtin alias such
        // as `M.fromList` -> `mapFromList` canonicalizes here before class-method
        // rewriting can strip the qualifier and dispatch `IsList.fromList`.
        // workflow: language-features-chirho/own-class-method-occurrences-chirho
        if self.reference_evidence_chirho.contains_key(&head_id_chirho)
            && self
                .dict_param_bindings_chirho
                .contains_key(canonical_id_chirho)
        {
            return None;
        }
        // A constrained reference to the program's own method-named value inherits its
        // canonical target's ownership. Ground schemes may take no dictionary parameters.
        if self
            .local_shadow_ids_chirho
            .borrow()
            .contains(canonical_id_chirho)
        {
            return Some(CoreExprChirho::VarChirho(*canonical_id_chirho));
        }
        if let Some(evidence_chirho) = self.occurrence_evidence_chirho.get(&head_id_chirho) {
            let (class_name_chirho, ty_key_chirho) = evidence_chirho.keys_chirho();
            // A displaced compiler class's method takes its own selector and dictionary before
            // any spelled route, the enclosing source instance's own dictionary included: that
            // spelling is the source class's (row 692, C10).
            // workflow: language-features-chirho/dictionary-evidence-chirho
            if let Some(head_chirho) = self.compiler_method_head_chirho(
                &evidence_chirho.class_identity_chirho,
                method_name_chirho,
                ty_key_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
            ) {
                return Some(head_chirho);
            }
            // A recursive call proved only by its head keeps the enclosing
            // instance's own dictionary, the one its argument's key names.
            // workflow: language-features-chirho/own-class-method-occurrences-chirho
            if let Some(self_dict_head_chirho) = self.agreeing_self_dictionary_chirho(
                head_id_chirho,
                syntactic_key_chirho,
                local_instance_dicts_chirho,
            ) {
                return Some(self_dict_head_chirho);
            }
            // A ground application of a parametric GND newtype takes the method from the
            // dictionary its instance's factory builds there (row 694, M1).
            // workflow: compiler-pipeline-chirho/deriving-strategies-chirho
            if let Some(dispatched_chirho) = self.method_through_factory_chirho(
                head_id_chirho,
                class_name_chirho,
                local_instance_dicts_chirho,
            ) {
                return Some(dispatched_chirho);
            }
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
            if !literal_wrapper_chirho
                && let Some(prim_chirho) = self.proof_row_chirho(&prim_name_chirho)
            {
                return Some(prim_chirho);
            }
            // A proven list key that names no row never comes back as the
            // canonical id, whose recursion re-keyed the call off the syntax.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            if let Some(dispatched_chirho) = self.proven_list_occurrence_chirho(
                head_id_chirho,
                syntactic_key_chirho,
                local_instance_dicts_chirho,
            ) {
                return Some(dispatched_chirho);
            }
            // Keyed fallback: reuse the existing selector/instance dispatch with
            // the proven key so selector-backed instances (Eq/Num Int, ...) work.
            // Only the binding's own key may reach the context dictionary; any
            // other proof that names nothing comes back as the canonical id, and
            // the caller's recursion takes its own route.
            // workflow: language-features-chirho/own-class-method-occurrences-chirho
            let no_context_chirho = HashSet::new();
            let keyed_classes_chirho = if Self::proof_admits_context_chirho(ty_key_chirho) {
                evidence_classes_chirho
            } else {
                &no_context_chirho
            };
            let keyed_chirho = self.try_rewrite_method_var_chirho(
                *canonical_id_chirho,
                dict_vars_chirho,
                keyed_classes_chirho,
                local_instance_dicts_chirho,
                Some(ty_key_chirho),
            );
            // Every keyed route missed: a ground goal of a conditional instance takes the
            // dictionary the pre-pass built for this occurrence (S1).
            // workflow: language-features-chirho/dictionary-evidence-chirho
            if matches!(&keyed_chirho, CoreExprChirho::VarChirho(kept_chirho) if kept_chirho == canonical_id_chirho)
                && let Some((method_class_chirho, selector_chirho)) =
                    self.class_method_selector_for_name_chirho(method_name_chirho)
                && let Some(projection_chirho) = self.ground_goal_projection_chirho(
                    head_id_chirho,
                    &method_class_chirho,
                    selector_chirho,
                )
            {
                return Some(projection_chirho);
            }
            return Some(keyed_chirho);
        }
        Some(CoreExprChirho::VarChirho(*canonical_id_chirho))
    }

    /// R2 for one method occurrence, the route every entry shares: when the
    /// checker proved an `Eq` or `Ord` occurrence at a whole list key, the
    /// dispatch of `proven_list_dispatch_chirho`, else `None`. Each caller
    /// tries its method rows first. The argument-spine path offers the call's
    /// own key, read lazily; `try_rewrite_method_var_chirho` offers the key its
    /// caller read or guessed, which is how occurrences inside typed
    /// higher-order arguments arrive (R2b, gpt_chirho #27663). Either key
    /// serves only when it agrees with the proof.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn proven_list_occurrence_chirho(
        &self,
        occurrence_id_chirho: CoreIdChirho,
        candidate_key_chirho: &dyn Fn() -> Option<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (_method_name_chirho, canonical_id_chirho) = self
            .method_occurrence_canon_chirho
            .get(&occurrence_id_chirho)?;
        let (class_name_chirho, proven_key_chirho) = self
            .occurrence_evidence_chirho
            .get(&occurrence_id_chirho)?
            .keys_chirho();
        let selector_id_chirho = self.proven_list_selector_chirho(
            *canonical_id_chirho,
            class_name_chirho,
            proven_key_chirho,
        )?;
        Some(self.proven_list_dispatch_chirho(
            occurrence_id_chirho,
            selector_id_chirho,
            class_name_chirho,
            proven_key_chirho,
            candidate_key_chirho,
            local_instance_dicts_chirho,
        ))
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
    /// holds for the proven key; else to the one for the proven key in the
    /// engine's spelling (`engine_spelled_key_chirho`: a defaulted `[Integer]`
    /// is the engine's `[Int]`), derived from the proof alone (R2b); else to the
    /// one for the call's argument-shape key, only when that key agrees with
    /// the proof (`key_agrees_with_proof_chirho`); else to `missing_dictionary_chirho`,
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
                // The proof respelled, before any caller's key: a typed
                // higher-order argument's own key is the element of a collapsed
                // `[Int]`, which never agrees with a defaulted `[Integer]`.
                let engine_key_chirho = Self::engine_spelled_key_chirho(proven_key_chirho);
                if engine_key_chirho == proven_key_chirho {
                    return None;
                }
                self.registry_dict_chirho(
                    class_name_chirho,
                    &engine_key_chirho,
                    local_instance_dicts_chirho,
                )
            })
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
        evidence_chirho: &mut Vec<ReferenceEvidenceChirho>,
        class_name_chirho: &str,
    ) -> Option<ReferenceEvidenceChirho> {
        let position_chirho = evidence_chirho
            .iter()
            .position(|proof_chirho| proof_chirho.class_name_chirho == class_name_chirho)?;
        Some(evidence_chirho.remove(position_chirho))
    }

    /// The dictionary of `class_chirho` at the ground application `ty_chirho` through its
    /// conditional instance's factory (`conditional_dicts_chirho`): the factory applied to its
    /// context's dictionaries, each predicate instantiated by mapping the head's formal binders
    /// to `ty_chirho`'s arguments, its dictionary the registry's at that type or another
    /// factory's. `None` when any of them is missing (row 694, P1, P2).
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn conditional_dictionary_chirho(
        &self,
        class_name_chirho: &str,
        ty_chirho: &TyChirho,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        self.conditional_dictionary_at_depth_chirho(
            class_name_chirho,
            ty_chirho,
            local_instance_dicts_chirho,
            0,
        )
    }

    fn conditional_dictionary_at_depth_chirho(
        &self,
        class_name_chirho: &str,
        ty_chirho: &TyChirho,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        depth_chirho: usize,
    ) -> Option<CoreExprChirho> {
        if depth_chirho > CONDITIONAL_DICTIONARY_DEPTH_CHIRHO {
            return None;
        }
        let (head_chirho, arguments_chirho) = type_application_spine_chirho(ty_chirho);
        let TyChirho::ConChirho(head_name_chirho) = head_chirho else {
            return None;
        };
        // A multi-parameter entry is found by its whole head (`head_dictionary_chirho`), never by
        // a constructor.
        let factory_chirho = self
            .conditional_dicts_chirho
            .get(&(class_name_chirho.to_string(), head_name_chirho.clone()))
            .filter(|factory_chirho| factory_chirho.instance_chirho.is_none())?;
        if factory_chirho.formals_chirho.len() != arguments_chirho.len() {
            return None;
        }
        let binding_chirho: HashMap<TyVarChirho, &TyChirho> = factory_chirho
            .formals_chirho
            .iter()
            .copied()
            .zip(arguments_chirho)
            .collect();
        factory_chirho.context_chirho.iter().try_fold(
            CoreExprChirho::VarChirho(factory_chirho.builder_chirho),
            |dictionary_chirho, predicate_chirho| {
                let argument_chirho = self.dictionary_at_type_chirho(
                    &predicate_chirho.class_name_chirho,
                    &instantiated_ty_chirho(&predicate_chirho.ty_chirho, &binding_chirho),
                    local_instance_dicts_chirho,
                    depth_chirho + 1,
                )?;
                Some(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(dictionary_chirho),
                    arg_chirho: Box::new(argument_chirho),
                })
            },
        )
    }

    /// The dictionary of the multi-parameter class `class_name_chirho` at the complete ground
    /// head `head_chirho` (`ShowSecondChirho Int Bool`): the one conditional-registry entry
    /// whose instance head matches it, its dictionary or its factory applied to its context's
    /// dictionaries at that match. `None` when no entry or several match, or a context
    /// dictionary is missing (row 694, #31909).
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn head_dictionary_chirho(
        &self,
        class_name_chirho: &str,
        head_chirho: &[TyChirho],
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        self.head_dictionary_at_depth_chirho(
            class_name_chirho,
            head_chirho,
            local_instance_dicts_chirho,
            0,
        )
    }

    fn head_dictionary_at_depth_chirho(
        &self,
        class_name_chirho: &str,
        head_chirho: &[TyChirho],
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        depth_chirho: usize,
    ) -> Option<CoreExprChirho> {
        let (first_chirho, extras_chirho) = head_chirho.split_first()?;
        if depth_chirho > CONDITIONAL_DICTIONARY_DEPTH_CHIRHO || extras_chirho.is_empty() {
            return None;
        }
        let wanted_chirho =
            PredChirho::legacy_chirho(haskelujah_typing_chirho::class_chirho::PredPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: class_name_chirho.to_string(),
                ty_chirho: first_chirho.clone(),
                extra_tys_chirho: extras_chirho.to_vec(),
            });
        let mut matching_chirho = self
            .conditional_dicts_chirho
            .iter()
            .filter(|(key_chirho, _)| key_chirho.0 == class_name_chirho)
            .filter_map(|(_, factory_chirho)| {
                let instance_chirho = factory_chirho.instance_chirho.as_ref()?;
                ClassEnvChirho::instance_sub_goals_chirho(instance_chirho, &wanted_chirho)
                    .map(|goals_chirho| (factory_chirho.builder_chirho, goals_chirho))
            });
        let (builder_chirho, goals_chirho) = matching_chirho.next()?;
        if matching_chirho.next().is_some() {
            return None;
        }
        goals_chirho.iter().try_fold(
            CoreExprChirho::VarChirho(builder_chirho),
            |dictionary_chirho, goal_chirho| {
                let argument_chirho = if goal_chirho.extra_tys_chirho.is_empty() {
                    self.dictionary_at_type_chirho(
                        &goal_chirho.class_name_chirho,
                        &goal_chirho.ty_chirho,
                        local_instance_dicts_chirho,
                        depth_chirho + 1,
                    )?
                } else {
                    self.head_dictionary_at_depth_chirho(
                        &goal_chirho.class_name_chirho,
                        &goal_chirho.all_tys_chirho(),
                        local_instance_dicts_chirho,
                        depth_chirho + 1,
                    )?
                };
                Some(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(dictionary_chirho),
                    arg_chirho: Box::new(argument_chirho),
                })
            },
        )
    }

    /// `show` for a `print` occurrence whose `Show` evidence is a ground application with a
    /// conditional instance's factory (a parametric GND newtype, row 694): the `show` selector
    /// applied to the dictionary the factory builds. `None` for any other occurrence.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn print_show_through_factory_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (reference_name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&head_id_chirho)?;
        if reference_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(reference_name_chirho)
            != "print"
        {
            return None;
        }
        let applied_chirho = self
            .reference_evidence_chirho
            .get(&head_id_chirho)?
            .iter()
            .find(|proof_chirho| proof_chirho.class_name_chirho == "Show")?
            .applied_ty_chirho
            .as_ref()?;
        let dictionary_chirho = self.conditional_dictionary_chirho(
            "Show",
            applied_chirho,
            local_instance_dicts_chirho,
        )?;
        let (selector_class_chirho, selector_chirho) = self.method_selectors_chirho.get("show")?;
        (selector_class_chirho == "Show").then(|| CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(*selector_chirho)),
            arg_chirho: Box::new(dictionary_chirho),
        })
    }

    /// A method occurrence the checker proved at a ground application of a parametric GND
    /// newtype (`describeChirho (BoxChirho True)`, row 694, M1): the method's selector applied
    /// to the dictionary that instance's factory builds at the proved arguments, the route
    /// `print` takes too. The instance's own rows take its context first and never serve such
    /// an occurrence, and a factory that cannot build the dictionary refuses the use when the
    /// module compiles (`missing_dictionary_chirho`). `None` for every other occurrence; one
    /// at a type with variables is the separate non-ground limit.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn method_through_factory_chirho(
        &self,
        occurrence_id_chirho: CoreIdChirho,
        class_name_chirho: &str,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let applied_chirho = self
            .occurrence_applied_tys_chirho
            .get(&occurrence_id_chirho)?;
        let (TyChirho::ConChirho(newtype_chirho), _arguments_chirho) =
            type_application_spine_chirho(applied_chirho)
        else {
            return None;
        };
        if !self
            .gnd_parametric_chirho
            .contains(&(class_name_chirho.to_string(), newtype_chirho.clone()))
        {
            return None;
        }
        let (_method_name_chirho, canonical_id_chirho) = self
            .method_occurrence_canon_chirho
            .get(&occurrence_id_chirho)?;
        let (selector_class_chirho, selector_chirho) = self
            .class_method_selector_for_name_chirho(self.names_chirho.get(canonical_id_chirho)?)?;
        if selector_class_chirho != class_name_chirho {
            return None;
        }
        let dictionary_chirho = self
            .conditional_dictionary_chirho(
                class_name_chirho,
                applied_chirho,
                local_instance_dicts_chirho,
            )
            .unwrap_or_else(|| {
                self.missing_dictionary_chirho(
                    class_name_chirho,
                    Some(newtype_chirho.as_str()),
                    *canonical_id_chirho,
                )
            });
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
            arg_chirho: Box::new(dictionary_chirho),
        })
    }

    /// The dictionary of `class_chirho` at the ground type `ty_chirho`: the registry's under
    /// the key the type displays as (the spelling instance heads register under), else a
    /// conditional instance's factory at its arguments.
    fn dictionary_at_type_chirho(
        &self,
        class_name_chirho: &str,
        ty_chirho: &TyChirho,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        depth_chirho: usize,
    ) -> Option<CoreExprChirho> {
        // The engine's body-backed rows are keyed "Int"; the checker defaults an ambiguous
        // numeric to "Integer", as the evidence join already maps it.
        let displayed_chirho = format!("{ty_chirho}");
        let key_chirho = if displayed_chirho == "Integer" {
            "Int".to_string()
        } else {
            displayed_chirho
        };
        if let Some(dictionary_chirho) =
            self.registry_dict_chirho(class_name_chirho, &key_chirho, local_instance_dicts_chirho)
        {
            return Some(CoreExprChirho::VarChirho(dictionary_chirho));
        }
        self.conditional_dictionary_at_depth_chirho(
            class_name_chirho,
            ty_chirho,
            local_instance_dicts_chirho,
            depth_chirho,
        )
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

/// A type application's head and its arguments in order.
fn type_application_spine_chirho(ty_chirho: &TyChirho) -> (&TyChirho, Vec<&TyChirho>) {
    let mut arguments_chirho = Vec::new();
    let mut head_chirho = ty_chirho;
    while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
        arguments_chirho.push(argument_chirho.as_ref());
        head_chirho = function_chirho.as_ref();
    }
    arguments_chirho.reverse();
    (head_chirho, arguments_chirho)
}

/// `ty_chirho` with each formal binder replaced by the type it is bound to.
fn instantiated_ty_chirho(
    ty_chirho: &TyChirho,
    binding_chirho: &HashMap<TyVarChirho, &TyChirho>,
) -> TyChirho {
    match ty_chirho {
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => TyChirho::QualChirho {
            context_chirho: context_chirho
                .iter()
                .map(|constraint_chirho| instantiated_ty_chirho(constraint_chirho, binding_chirho))
                .collect(),
            body_chirho: Box::new(instantiated_ty_chirho(body_chirho, binding_chirho)),
        },
        TyChirho::VarChirho(var_chirho) => binding_chirho
            .get(var_chirho)
            .map(|bound_chirho| (*bound_chirho).clone())
            .unwrap_or_else(|| ty_chirho.clone()),
        TyChirho::AppChirho(function_chirho, argument_chirho) => TyChirho::AppChirho(
            Box::new(instantiated_ty_chirho(function_chirho, binding_chirho)),
            Box::new(instantiated_ty_chirho(argument_chirho, binding_chirho)),
        ),
        TyChirho::FunChirho(argument_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(instantiated_ty_chirho(argument_chirho, binding_chirho)),
            Box::new(instantiated_ty_chirho(result_chirho, binding_chirho)),
            *mult_chirho,
        ),
        TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|element_chirho| instantiated_ty_chirho(element_chirho, binding_chirho))
                .collect(),
        ),
        TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(
            instantiated_ty_chirho(element_chirho, binding_chirho),
        )),
        TyChirho::ConChirho(_)
        | TyChirho::ForallVarChirho(_)
        | TyChirho::ForallChirho { .. }
        | TyChirho::RequiredForallChirho { .. } => ty_chirho.clone(),
    }
}
