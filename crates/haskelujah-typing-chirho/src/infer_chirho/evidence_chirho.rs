// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Literal evidence: the checker records, per source literal, the instance it
//! solved for the literal's overloading class, keyed by the literal's span, so
//! the dictionary pass dispatches `fromInteger` / `fromString` by proof instead
//! of guessing from sibling arguments.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::*;

/// A recursive reference sees the group's monomorphic assumption before its
/// predicates exist. Keep its source identity until generalization supplies
/// those predicates; never infer its evidence from a sibling argument.
#[derive(Default)]
pub(super) struct RecursiveReferencesChirho {
    pub(super) assumptions_chirho: HashMap<String, TyChirho>,
    spans_chirho: HashMap<String, Vec<SpanChirho>>,
}

impl RecursiveReferencesChirho {
    pub(super) fn apply_subst_chirho(&mut self, subst_chirho: &SubstChirho) {
        for ty_chirho in self.assumptions_chirho.values_mut() {
            *ty_chirho = subst_chirho.apply_ty_chirho(ty_chirho);
        }
    }

    pub(super) fn capture_chirho(
        &mut self,
        name_chirho: &str,
        span_chirho: SpanChirho,
        scheme_chirho: &SchemeChirho,
    ) {
        if span_chirho != SpanChirho::DUMMY_CHIRHO
            && scheme_chirho.vars_chirho.is_empty()
            && scheme_chirho.preds_chirho.is_empty()
            && self.assumptions_chirho.get(name_chirho) == Some(&scheme_chirho.ty_chirho)
        {
            self.spans_chirho
                .entry(name_chirho.to_string())
                .or_default()
                .push(span_chirho);
        }
    }
}

/// The evidence key of an occurrence whose type is a rigid variable of the
/// enclosing signature: no instance is the proof, the binding's own dictionary
/// parameter for the class is. The dictionary pass dispatches such an
/// occurrence through that parameter and never through a guessed instance.
pub const OWN_DICTIONARY_KEY_CHIRHO: &str = "$own";

/// The own-parameter key naming the signature predicate the parameter comes
/// from (`$own:2` is the third predicate of the enclosing signature). Without
/// an index the pass falls back to the binding's last parameter of the class.
pub fn own_dictionary_key_chirho(index_chirho: Option<usize>) -> String {
    match index_chirho {
        Some(index_chirho) => format!("{OWN_DICTIONARY_KEY_CHIRHO}:{index_chirho}"),
        None => OWN_DICTIONARY_KEY_CHIRHO.to_string(),
    }
}

/// Whether an evidence key means "the enclosing binding's own parameter".
pub fn is_own_dictionary_key_chirho(key_chirho: &str) -> bool {
    key_chirho == OWN_DICTIONARY_KEY_CHIRHO
        || key_chirho
            .strip_prefix(OWN_DICTIONARY_KEY_CHIRHO)
            .is_some_and(|rest_chirho| rest_chirho.starts_with(':'))
}

/// The evidence for one predicate of a constrained reference (`s True` with
/// `s :: Show a => ...`): the instance head key the checker solved the
/// predicate to, `OWN_DICTIONARY_KEY_CHIRHO` when it is served by the
/// enclosing binding's own dictionary parameter, or `None` when nothing was
/// proved (the dictionary pass then keeps its own guess for that position).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceEvidenceChirho {
    pub class_name_chirho: String,
    pub ty_key_chirho: Option<String>,
}

/// The solved instance of one overloaded literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralEvidenceChirho {
    /// `Num`, `Fractional` or `IsString`.
    pub class_name_chirho: String,
    /// The instance head key the literal's type resolved to (`Int`, `Double`,
    /// `[]`, a user type's name, ...).
    pub ty_key_chirho: String,
}

impl InferCtxChirho {
    /// Close the recursive group's deferred evidence after its type is known.
    /// The monomorphic recursive references share these very type variables;
    /// ordinary polymorphic calls still capture their freshly instantiated ones.
    pub(super) fn finish_recursive_references_chirho(
        &mut self,
        name_chirho: &str,
        scheme_chirho: &SchemeChirho,
    ) {
        self.recursive_references_chirho
            .assumptions_chirho
            .remove(name_chirho);
        for span_chirho in self
            .recursive_references_chirho
            .spans_chirho
            .remove(name_chirho)
            .unwrap_or_default()
        {
            self.reference_captures_chirho.push((
                span_chirho,
                scheme_chirho
                    .preds_chirho
                    .iter()
                    .map(|pred_chirho| {
                        (
                            pred_chirho.class_name_chirho.clone(),
                            pred_chirho.ty_chirho.clone(),
                        )
                    })
                    .collect(),
            ));
        }
    }

    /// Record a literal's overloading predicate for evidence. Generated code
    /// carries the dummy span and is skipped: a key must name one source token.
    pub(super) fn capture_literal_evidence_chirho(
        &mut self,
        span_chirho: SpanChirho,
        class_name_chirho: &str,
        ty_chirho: &TyChirho,
    ) {
        if span_chirho == SpanChirho::DUMMY_CHIRHO {
            return;
        }
        self.literal_captures_chirho.push((
            span_chirho,
            class_name_chirho.to_string(),
            ty_chirho.clone(),
        ));
    }

    /// Record a constrained reference's instantiated predicates, in the
    /// scheme's order, for evidence. Dummy spans (generated code) are skipped.
    pub(super) fn capture_reference_evidence_chirho(
        &mut self,
        span_chirho: SpanChirho,
        preds_chirho: &[PredChirho],
    ) {
        if span_chirho == SpanChirho::DUMMY_CHIRHO || preds_chirho.is_empty() {
            return;
        }
        self.reference_captures_chirho.push((
            span_chirho,
            preds_chirho
                .iter()
                .map(|pred_chirho| {
                    (
                        pred_chirho.class_name_chirho.clone(),
                        pred_chirho.ty_chirho.clone(),
                    )
                })
                .collect(),
        ));
    }

    /// Remember, for each predicate of a signature being checked with rigid
    /// variables, which predicate index its skolem stands for, so an occurrence
    /// at that skolem can name the binding's parameter exactly (a signature may
    /// carry two predicates of one class: `(Describe a, Describe b) =>`).
    pub(super) fn record_own_dictionary_indices_chirho(&mut self, preds_chirho: &[PredChirho]) {
        for (index_chirho, pred_chirho) in preds_chirho.iter().enumerate() {
            if let TyChirho::ForallVarChirho(skolem_chirho) = &pred_chirho.ty_chirho {
                self.skolem_pred_index_chirho.insert(
                    (pred_chirho.class_name_chirho.clone(), skolem_chirho.clone()),
                    index_chirho,
                );
            }
        }
    }

    /// The own-parameter key for a method or predicate of `class` at a rigid
    /// variable: indexed when the enclosing signature declares that predicate.
    pub(super) fn own_key_for_skolem_chirho(
        &self,
        class_name_chirho: &str,
        skolem_chirho: &str,
    ) -> String {
        own_dictionary_key_chirho(
            self.skolem_pred_index_chirho
                .get(&(class_name_chirho.to_string(), skolem_chirho.to_string()))
                .copied(),
        )
    }

    /// The key a local binding's quantified variable is specialized at: the
    /// dictionary pass gives a `let`/`where` binding no dictionary parameter,
    /// so an occurrence at that variable is served by the one concrete type
    /// every instantiation of the binding resolved to. Two different
    /// instantiations, or an open one, prove nothing.
    pub(super) fn local_specialization_key_chirho(
        &self,
        var_chirho: TyVarChirho,
        final_subst_chirho: &SubstChirho,
    ) -> Option<String> {
        self.local_specialization_key_at_depth_chirho(var_chirho, final_subst_chirho, 0)
    }

    /// A local binding instantiated only by another local binding (`isPrime n =
    /// checkDiv n 2` inside a `where`) resolves to that binding's own quantified
    /// variable; the key is then that variable's, transitively, with a depth
    /// guard against a cycle of mutually recursive locals.
    fn local_specialization_key_at_depth_chirho(
        &self,
        var_chirho: TyVarChirho,
        final_subst_chirho: &SubstChirho,
        depth_chirho: usize,
    ) -> Option<String> {
        if depth_chirho > 8 || !self.local_generalized_vars_chirho.contains(&var_chirho) {
            return None;
        }
        let instantiations_chirho = self.local_instantiations_chirho.get(&var_chirho)?;
        let mut key_chirho: Option<String> = None;
        for fresh_chirho in instantiations_chirho {
            let resolved_chirho =
                final_subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(*fresh_chirho));
            let this_key_chirho = match &resolved_chirho {
                TyChirho::VarChirho(other_chirho) if *other_chirho != var_chirho => self
                    .local_specialization_key_at_depth_chirho(
                        *other_chirho,
                        final_subst_chirho,
                        depth_chirho + 1,
                    )?,
                _ => literal_head_key_chirho(&resolved_chirho)?,
            };
            match &key_chirho {
                None => key_chirho = Some(this_key_chirho),
                Some(seen_chirho) if *seen_chirho == this_key_chirho => {}
                Some(_) => return None,
            }
        }
        key_chirho
    }

    /// The evidence key of one predicate's resolved type: a concrete head, the
    /// enclosing binding's own parameter for a rigid or generalized variable
    /// (or a variable with a numeric default hint, which the pass keys by), and
    /// nothing for anything still open.
    fn reference_key_chirho(
        &self,
        class_name_chirho: &str,
        ty_chirho: &TyChirho,
        final_subst_chirho: &SubstChirho,
    ) -> Option<String> {
        match ty_chirho {
            TyChirho::ForallVarChirho(skolem_chirho) => {
                Some(self.own_key_for_skolem_chirho(class_name_chirho, skolem_chirho))
            }
            TyChirho::VarChirho(var_chirho) => {
                if let Some(hint_chirho) = self.occurrence_default_hints_chirho.get(var_chirho) {
                    Some(hint_chirho.clone())
                } else if self.local_generalized_vars_chirho.contains(var_chirho) {
                    self.local_specialization_key_chirho(*var_chirho, final_subst_chirho)
                } else if self.occurrence_generalized_vars_chirho.contains(var_chirho) {
                    Some(OWN_DICTIONARY_KEY_CHIRHO.to_string())
                } else {
                    None
                }
            }
            _ => literal_head_key_chirho(ty_chirho),
        }
    }

    /// Finalize the reference captures through the composed substitution, one
    /// record per predicate in scheme order. A span captured more than once
    /// yields evidence only when every capture agrees.
    pub(super) fn finalize_reference_evidence_chirho(
        &self,
        final_subst_chirho: &SubstChirho,
    ) -> HashMap<SpanChirho, Vec<ReferenceEvidenceChirho>> {
        let mut by_span_chirho: HashMap<SpanChirho, Vec<Vec<ReferenceEvidenceChirho>>> =
            HashMap::new();
        for (span_chirho, preds_chirho) in &self.reference_captures_chirho {
            let records_chirho: Vec<ReferenceEvidenceChirho> = preds_chirho
                .iter()
                .map(|(class_name_chirho, ty_chirho)| ReferenceEvidenceChirho {
                    class_name_chirho: class_name_chirho.clone(),
                    ty_key_chirho: self.reference_key_chirho(
                        class_name_chirho,
                        &final_subst_chirho.apply_ty_chirho(ty_chirho),
                        final_subst_chirho,
                    ),
                })
                .collect();
            by_span_chirho
                .entry(*span_chirho)
                .or_default()
                .push(records_chirho);
        }
        by_span_chirho
            .into_iter()
            .filter_map(|(span_chirho, mut captures_chirho)| {
                let first_chirho = captures_chirho.pop()?;
                captures_chirho
                    .iter()
                    .all(|capture_chirho| *capture_chirho == first_chirho)
                    .then_some((span_chirho, first_chirho))
            })
            .collect()
    }

    /// Finalize the captures through the module's composed substitution.
    /// Only a literal whose type resolved to a concrete head yields evidence;
    /// one still on a variable (generalized, or ambiguous) or on a rigid
    /// variable keeps today's dispatch path. A span inferred more than once
    /// (a re-check, a speculative branch) yields evidence only when every
    /// inference agrees.
    pub(super) fn finalize_literal_evidence_chirho(
        &self,
        final_subst_chirho: &SubstChirho,
    ) -> HashMap<SpanChirho, LiteralEvidenceChirho> {
        let mut keys_by_span_chirho: HashMap<SpanChirho, Vec<LiteralEvidenceChirho>> =
            HashMap::new();
        for (span_chirho, class_name_chirho, ty_chirho) in &self.literal_captures_chirho {
            let resolved_chirho = final_subst_chirho.apply_ty_chirho(ty_chirho);
            let key_chirho = match &resolved_chirho {
                TyChirho::VarChirho(var_chirho) => {
                    self.local_specialization_key_chirho(*var_chirho, final_subst_chirho)
                }
                _ => literal_head_key_chirho(&resolved_chirho),
            };
            let Some(key_chirho) = key_chirho else {
                continue;
            };
            keys_by_span_chirho
                .entry(*span_chirho)
                .or_default()
                .push(LiteralEvidenceChirho {
                    class_name_chirho: class_name_chirho.clone(),
                    ty_key_chirho: key_chirho,
                });
        }
        keys_by_span_chirho
            .into_iter()
            .filter_map(|(span_chirho, mut records_chirho)| {
                let first_chirho = records_chirho.pop()?;
                records_chirho
                    .iter()
                    .all(|record_chirho| *record_chirho == first_chirho)
                    .then_some((span_chirho, first_chirho))
            })
            .collect()
    }
}

/// The instance-head key of a concrete literal type; `None` for anything still
/// carrying a unification variable, and for a rigid variable (that literal is
/// dispatched through the enclosing binding's dictionary parameter).
fn literal_head_key_chirho(ty_chirho: &TyChirho) -> Option<String> {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho) => Some(name_chirho.clone()),
        TyChirho::ListChirho(_) => Some("[]".to_string()),
        TyChirho::AppChirho(fun_chirho, _) => literal_head_key_chirho(fun_chirho),
        _ => None,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn span_chirho(start_chirho: u32) -> SpanChirho {
        SpanChirho::new_chirho(
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
            haskelujah_span_chirho::ByteOffsetChirho::new_chirho(start_chirho),
            haskelujah_span_chirho::ByteOffsetChirho::new_chirho(start_chirho + 1),
        )
    }

    #[test]
    fn literal_evidence_keeps_concrete_keys_and_drops_variables_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let int_lit_chirho = ctx_chirho.fresh_var_chirho();
        let open_lit_chirho = ctx_chirho.fresh_var_chirho();
        ctx_chirho.capture_literal_evidence_chirho(span_chirho(10), "Num", &int_lit_chirho);
        ctx_chirho.capture_literal_evidence_chirho(span_chirho(20), "Num", &open_lit_chirho);
        ctx_chirho.capture_literal_evidence_chirho(
            span_chirho(30),
            "Num",
            &TyChirho::ForallVarChirho("a%1".to_string()),
        );
        ctx_chirho.capture_literal_evidence_chirho(
            SpanChirho::DUMMY_CHIRHO,
            "Num",
            &TyChirho::int_chirho(),
        );
        let mut subst_chirho = SubstChirho::empty_chirho();
        if let TyChirho::VarChirho(var_chirho) = int_lit_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::ConChirho("Double".to_string()));
        }
        let evidence_chirho = ctx_chirho.finalize_literal_evidence_chirho(&subst_chirho);
        assert_eq!(
            evidence_chirho.get(&span_chirho(10)),
            Some(&LiteralEvidenceChirho {
                class_name_chirho: "Num".to_string(),
                ty_key_chirho: "Double".to_string(),
            })
        );
        assert_eq!(evidence_chirho.len(), 1, "{evidence_chirho:?}");
    }

    #[test]
    fn reference_evidence_names_the_proof_per_predicate_chirho() {
        // `both :: (Describe a, Describe b) => ...` used at (Bool, Int): one
        // record per predicate, in scheme order; a rigid variable is the
        // enclosing binding's own parameter; an open variable proves nothing.
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let a_chirho = ctx_chirho.fresh_var_chirho();
        let b_chirho = ctx_chirho.fresh_var_chirho();
        let open_chirho = ctx_chirho.fresh_var_chirho();
        let generalized_chirho = ctx_chirho.fresh_var_chirho();
        if let TyChirho::VarChirho(var_chirho) = generalized_chirho {
            ctx_chirho
                .occurrence_generalized_vars_chirho
                .insert(var_chirho);
        }
        ctx_chirho.capture_reference_evidence_chirho(
            span_chirho(40),
            &[
                PredChirho::new_chirho("Describe", a_chirho.clone()),
                PredChirho::new_chirho("Describe", b_chirho.clone()),
                PredChirho::new_chirho("Num", TyChirho::ForallVarChirho("n%1".to_string())),
                PredChirho::new_chirho("Show", open_chirho),
                PredChirho::new_chirho("Eq", generalized_chirho),
            ],
        );
        let mut subst_chirho = SubstChirho::empty_chirho();
        if let TyChirho::VarChirho(var_chirho) = a_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::ConChirho("Bool".to_string()));
        }
        if let TyChirho::VarChirho(var_chirho) = b_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::int_chirho());
        }
        let evidence_chirho = ctx_chirho.finalize_reference_evidence_chirho(&subst_chirho);
        let keys_chirho: Vec<(String, Option<String>)> = evidence_chirho[&span_chirho(40)]
            .iter()
            .map(|record_chirho| {
                (
                    record_chirho.class_name_chirho.clone(),
                    record_chirho.ty_key_chirho.clone(),
                )
            })
            .collect();
        assert_eq!(
            keys_chirho,
            vec![
                ("Describe".to_string(), Some("Bool".to_string())),
                ("Describe".to_string(), Some("Int".to_string())),
                (
                    "Num".to_string(),
                    Some(OWN_DICTIONARY_KEY_CHIRHO.to_string())
                ),
                ("Show".to_string(), None),
                (
                    "Eq".to_string(),
                    Some(OWN_DICTIONARY_KEY_CHIRHO.to_string())
                ),
            ]
        );
    }

    #[test]
    fn own_parameter_evidence_names_the_signature_predicate_by_index_chirho() {
        // `both :: (Describe a, Describe b) => ...`: an occurrence at `b` is
        // the second parameter, not "the binding's Describe parameter".
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let a_chirho = TyChirho::ForallVarChirho("a%1".to_string());
        let b_chirho = TyChirho::ForallVarChirho("b%2".to_string());
        ctx_chirho.record_own_dictionary_indices_chirho(&[
            PredChirho::new_chirho("Describe", a_chirho.clone()),
            PredChirho::new_chirho("Describe", b_chirho.clone()),
        ]);
        assert_eq!(
            ctx_chirho.own_key_for_skolem_chirho("Describe", "b%2"),
            "$own:1"
        );
        assert_eq!(
            ctx_chirho.own_key_for_skolem_chirho("Describe", "a%1"),
            "$own:0"
        );
        // A superclass method (`==` under `Ord a`) names no declared predicate:
        // the class-only key lets the pass extract the superclass dictionary.
        assert_eq!(ctx_chirho.own_key_for_skolem_chirho("Eq", "a%1"), "$own");
        assert!(is_own_dictionary_key_chirho("$own"));
        assert!(is_own_dictionary_key_chirho("$own:7"));
        assert!(!is_own_dictionary_key_chirho("$owner"));
        assert!(!is_own_dictionary_key_chirho("Int"));
    }

    #[test]
    fn local_specialization_follows_instantiations_transitively_chirho() {
        // `checkDiv`'s variable is instantiated only by `isPrime`, whose own
        // variable is instantiated at Int: the key is Int, transitively. Two
        // disagreeing instantiations prove nothing.
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let fresh_chirho = |ctx_chirho: &mut InferCtxChirho| match ctx_chirho.fresh_var_chirho() {
            TyChirho::VarChirho(var_chirho) => var_chirho,
            _ => unreachable!(),
        };
        let check_div_chirho = fresh_chirho(&mut ctx_chirho);
        let is_prime_chirho = fresh_chirho(&mut ctx_chirho);
        let inst_a_chirho = fresh_chirho(&mut ctx_chirho);
        let inst_b_chirho = fresh_chirho(&mut ctx_chirho);
        let split_chirho = fresh_chirho(&mut ctx_chirho);
        let inst_c_chirho = fresh_chirho(&mut ctx_chirho);
        let inst_d_chirho = fresh_chirho(&mut ctx_chirho);
        ctx_chirho.local_generalized_vars_chirho.extend([
            check_div_chirho,
            is_prime_chirho,
            split_chirho,
        ]);
        ctx_chirho
            .local_instantiations_chirho
            .insert(check_div_chirho, vec![inst_a_chirho]);
        ctx_chirho
            .local_instantiations_chirho
            .insert(is_prime_chirho, vec![inst_b_chirho]);
        ctx_chirho
            .local_instantiations_chirho
            .insert(split_chirho, vec![inst_c_chirho, inst_d_chirho]);
        let mut subst_chirho = SubstChirho::empty_chirho();
        subst_chirho.insert_chirho(inst_a_chirho, TyChirho::VarChirho(is_prime_chirho));
        subst_chirho.insert_chirho(inst_b_chirho, TyChirho::int_chirho());
        subst_chirho.insert_chirho(inst_c_chirho, TyChirho::int_chirho());
        subst_chirho.insert_chirho(inst_d_chirho, TyChirho::ConChirho("Double".to_string()));
        assert_eq!(
            ctx_chirho.local_specialization_key_chirho(check_div_chirho, &subst_chirho),
            Some("Int".to_string())
        );
        assert_eq!(
            ctx_chirho.local_specialization_key_chirho(split_chirho, &subst_chirho),
            None
        );
        assert_eq!(
            ctx_chirho.local_specialization_key_chirho(inst_a_chirho, &subst_chirho),
            None
        );
    }

    #[test]
    fn literal_evidence_needs_agreement_for_a_span_seen_twice_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.capture_literal_evidence_chirho(span_chirho(5), "Num", &TyChirho::int_chirho());
        ctx_chirho.capture_literal_evidence_chirho(
            span_chirho(5),
            "Num",
            &TyChirho::ConChirho("Double".to_string()),
        );
        ctx_chirho.capture_literal_evidence_chirho(span_chirho(6), "Num", &TyChirho::int_chirho());
        ctx_chirho.capture_literal_evidence_chirho(span_chirho(6), "Num", &TyChirho::int_chirho());
        let evidence_chirho =
            ctx_chirho.finalize_literal_evidence_chirho(&SubstChirho::empty_chirho());
        assert!(!evidence_chirho.contains_key(&span_chirho(5)));
        assert_eq!(evidence_chirho[&span_chirho(6)].ty_key_chirho, "Int");
    }
}
