// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Literal evidence: the checker records, per source literal, the instance it
//! solved for the literal's overloading class, keyed by the literal's span, so
//! the dictionary pass dispatches `fromInteger` / `fromString` by proof instead
//! of guessing from sibling arguments.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::*;

/// One constrained reference's predicate, captured during inference and
/// finalized into a `MethodOccurrenceRecordChirho` once the module is solved.
/// The origin is the reference's identity; the span stays for diagnostics and
/// for references that have no origin yet.
/// workflow: language-features-chirho/dictionary-evidence-chirho
#[derive(Debug, Clone)]
pub(super) struct OccurrenceCaptureChirho {
    pub(super) name_chirho: String,
    pub(super) ordinal_chirho: u32,
    pub(super) class_name_chirho: String,
    pub(super) ty_chirho: TyChirho,
    pub(super) span_chirho: SpanChirho,
    pub(super) origin_chirho: Option<OriginIdChirho>,
}

/// One constrained reference's instantiated predicates, in scheme order.
#[derive(Debug, Clone)]
pub(super) struct ReferenceCaptureChirho {
    pub(super) span_chirho: SpanChirho,
    pub(super) origin_chirho: Option<OriginIdChirho>,
    pub(super) preds_chirho: Vec<(String, TyChirho)>,
}

/// A recursive reference sees the group's monomorphic assumption before its
/// predicates exist. Keep its source identity until generalization supplies
/// those predicates; never infer its evidence from a sibling argument.
#[derive(Default)]
pub(super) struct RecursiveReferencesChirho {
    pub(super) assumptions_chirho: HashMap<String, TyChirho>,
    references_chirho: HashMap<String, Vec<(SpanChirho, Option<OriginIdChirho>)>>,
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
        origin_chirho: Option<OriginIdChirho>,
        scheme_chirho: &SchemeChirho,
    ) {
        if (span_chirho != SpanChirho::DUMMY_CHIRHO || origin_chirho.is_some())
            && scheme_chirho.vars_chirho.is_empty()
            && scheme_chirho.preds_chirho.is_empty()
            && self.assumptions_chirho.get(name_chirho) == Some(&scheme_chirho.ty_chirho)
        {
            self.references_chirho
                .entry(name_chirho.to_string())
                .or_default()
                .push((span_chirho, origin_chirho));
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
        for (span_chirho, origin_chirho) in self
            .recursive_references_chirho
            .references_chirho
            .remove(name_chirho)
            .unwrap_or_default()
        {
            self.reference_captures_chirho.push(ReferenceCaptureChirho {
                span_chirho,
                origin_chirho,
                preds_chirho: scheme_chirho
                    .preds_chirho
                    .iter()
                    .map(|pred_chirho| {
                        (
                            pred_chirho.class_name_chirho.clone(),
                            pred_chirho.ty_chirho.clone(),
                        )
                    })
                    .collect(),
            });
        }
    }

    /// Record a literal's overloading predicate for evidence, under the
    /// literal's span and its origin. A literal with neither a real span nor an
    /// origin identifies nothing and is skipped.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn capture_literal_evidence_chirho(
        &mut self,
        lit_chirho: &LitChirho,
        class_name_chirho: &str,
        ty_chirho: &TyChirho,
    ) {
        let span_chirho = lit_chirho.span_chirho();
        let origin_chirho = lit_chirho.origin_chirho();
        if span_chirho == SpanChirho::DUMMY_CHIRHO && origin_chirho.is_none() {
            return;
        }
        self.literal_captures_chirho.push((
            span_chirho,
            origin_chirho,
            class_name_chirho.to_string(),
            ty_chirho.clone(),
        ));
    }

    /// Record every predicate of one constrained reference: the reference's
    /// instantiated predicates in scheme order, and one occurrence capture per
    /// predicate under the per-name ordinal and the reference's origin. An
    /// occurrence with several predicates keeps all of them.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn capture_occurrence_predicates_chirho(
        &mut self,
        name_chirho: &NameChirho,
        preds_chirho: &[PredChirho],
    ) {
        if preds_chirho.is_empty() {
            return;
        }
        let span_chirho = name_chirho.span_chirho();
        let origin_chirho = name_chirho.origin_chirho();
        self.capture_reference_evidence_chirho(span_chirho, origin_chirho, preds_chirho);
        let counter_chirho = self
            .occurrence_counters_chirho
            .entry(name_chirho.text_chirho().to_string())
            .or_insert(0);
        let ordinal_chirho = *counter_chirho;
        *counter_chirho += 1;
        for pred_chirho in preds_chirho {
            self.occurrence_captures_chirho
                .push(OccurrenceCaptureChirho {
                    name_chirho: name_chirho.text_chirho().to_string(),
                    ordinal_chirho,
                    class_name_chirho: pred_chirho.class_name_chirho.clone(),
                    ty_chirho: pred_chirho.ty_chirho.clone(),
                    span_chirho,
                    origin_chirho,
                });
        }
    }

    /// Record a constrained reference's instantiated predicates, in the
    /// scheme's order, for evidence. A reference with neither a real span nor
    /// an origin (generated code before provenance) identifies nothing and is
    /// skipped.
    fn capture_reference_evidence_chirho(
        &mut self,
        span_chirho: SpanChirho,
        origin_chirho: Option<OriginIdChirho>,
        preds_chirho: &[PredChirho],
    ) {
        if span_chirho == SpanChirho::DUMMY_CHIRHO && origin_chirho.is_none() {
            return;
        }
        self.reference_captures_chirho.push(ReferenceCaptureChirho {
            span_chirho,
            origin_chirho,
            preds_chirho: preds_chirho
                .iter()
                .map(|pred_chirho| {
                    (
                        pred_chirho.class_name_chirho.clone(),
                        pred_chirho.ty_chirho.clone(),
                    )
                })
                .collect(),
        });
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
    /// record per predicate in scheme order. A capture with an origin is
    /// published under that origin ONLY; the span map is the legacy projection
    /// and holds only captures without an origin, never a placeholder span, so
    /// an origin-owned proof cannot reach another reference through its span
    /// (gpt_chirho review F2, #24598). A key captured more than once yields
    /// evidence only when every capture agrees.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn finalize_reference_evidence_chirho(
        &self,
        final_subst_chirho: &SubstChirho,
    ) -> (
        HashMap<SpanChirho, Vec<ReferenceEvidenceChirho>>,
        HashMap<OriginIdChirho, Vec<ReferenceEvidenceChirho>>,
    ) {
        let mut by_span_chirho: HashMap<SpanChirho, Vec<Vec<ReferenceEvidenceChirho>>> =
            HashMap::new();
        let mut by_origin_chirho: HashMap<OriginIdChirho, Vec<Vec<ReferenceEvidenceChirho>>> =
            HashMap::new();
        for capture_chirho in &self.reference_captures_chirho {
            let records_chirho: Vec<ReferenceEvidenceChirho> = capture_chirho
                .preds_chirho
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
            match capture_chirho.origin_chirho {
                Some(origin_chirho) => by_origin_chirho
                    .entry(origin_chirho)
                    .or_default()
                    .push(records_chirho),
                None if capture_chirho.span_chirho != SpanChirho::DUMMY_CHIRHO => by_span_chirho
                    .entry(capture_chirho.span_chirho)
                    .or_default()
                    .push(records_chirho),
                None => {}
            }
        }
        (
            agreeing_chirho(by_span_chirho),
            agreeing_chirho(by_origin_chirho),
        )
    }

    /// Finalize the captures through the module's composed substitution.
    /// Only a literal whose type resolved to a concrete head yields evidence;
    /// one still on a variable (generalized, or ambiguous) or on a rigid
    /// variable keeps today's dispatch path. A literal with an origin is
    /// published under that origin only; the span map holds only literals
    /// without one. A key inferred more than once (a re-check, a speculative
    /// branch) yields evidence only when every inference agrees.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn finalize_literal_evidence_chirho(
        &self,
        final_subst_chirho: &SubstChirho,
    ) -> (
        HashMap<SpanChirho, LiteralEvidenceChirho>,
        HashMap<OriginIdChirho, LiteralEvidenceChirho>,
    ) {
        let mut keys_by_span_chirho: HashMap<SpanChirho, Vec<LiteralEvidenceChirho>> =
            HashMap::new();
        let mut keys_by_origin_chirho: HashMap<OriginIdChirho, Vec<LiteralEvidenceChirho>> =
            HashMap::new();
        for (span_chirho, origin_chirho, class_name_chirho, ty_chirho) in
            &self.literal_captures_chirho
        {
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
            let record_chirho = LiteralEvidenceChirho {
                class_name_chirho: class_name_chirho.clone(),
                ty_key_chirho: key_chirho,
            };
            // Exclusive publication, as for references (gpt_chirho review F2): a
            // literal with an origin is published under that origin only, and the
            // span map is the origin-free legacy projection.
            match origin_chirho {
                Some(origin_chirho) => keys_by_origin_chirho
                    .entry(*origin_chirho)
                    .or_default()
                    .push(record_chirho),
                None if *span_chirho != SpanChirho::DUMMY_CHIRHO => keys_by_span_chirho
                    .entry(*span_chirho)
                    .or_default()
                    .push(record_chirho),
                None => {}
            }
        }
        (
            agreeing_chirho(keys_by_span_chirho),
            agreeing_chirho(keys_by_origin_chirho),
        )
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

/// Keep a key's evidence only when every capture of that key agrees: a key
/// inferred more than once (a re-check, a speculative branch) proves nothing
/// when the inferences differ.
fn agreeing_chirho<KeyChirho: std::hash::Hash + Eq, RecordChirho: PartialEq>(
    captures_chirho: HashMap<KeyChirho, Vec<RecordChirho>>,
) -> HashMap<KeyChirho, RecordChirho> {
    captures_chirho
        .into_iter()
        .filter_map(|(key_chirho, mut records_chirho)| {
            let first_chirho = records_chirho.pop()?;
            records_chirho
                .iter()
                .all(|record_chirho| *record_chirho == first_chirho)
                .then_some((key_chirho, first_chirho))
        })
        .collect()
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    /// An integer literal at `span_chirho`, identified by `origin_chirho`.
    fn test_int_literal_chirho(
        span_chirho: SpanChirho,
        origin_chirho: Option<OriginIdChirho>,
    ) -> LitChirho {
        LitChirho::IntChirho(0, span_chirho, origin_chirho)
    }

    #[test]
    fn an_origin_owned_literal_is_never_published_as_span_evidence_chirho() {
        // The literal analogue of review F2 (#24598): a literal owned by origin A
        // at a genuine span S must not also become legacy evidence at S.
        let mut supply_chirho =
            haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
        let owner_chirho = supply_chirho.fresh_chirho();
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(70), Some(owner_chirho)),
            "Num",
            &TyChirho::int_chirho(),
        );
        let (by_span_chirho, by_origin_chirho) =
            ctx_chirho.finalize_literal_evidence_chirho(&SubstChirho::empty_chirho());
        assert!(by_span_chirho.is_empty(), "{by_span_chirho:?}");
        assert_eq!(by_origin_chirho[&owner_chirho].ty_key_chirho, "Int");
    }

    #[test]
    fn two_generated_literals_under_one_placeholder_keep_their_own_evidence_chirho() {
        // gpt_chirho's control (#24575): two generated literals share the
        // placeholder span and need different concrete evidence. The span map
        // holds neither; the origin map holds each literal's own proof.
        let mut supply_chirho =
            haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
        let int_origin_chirho = supply_chirho.fresh_chirho();
        let double_origin_chirho = supply_chirho.fresh_chirho();
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let int_ty_chirho = ctx_chirho.fresh_var_chirho();
        let double_ty_chirho = ctx_chirho.fresh_var_chirho();
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(SpanChirho::DUMMY_CHIRHO, Some(int_origin_chirho)),
            "Num",
            &int_ty_chirho,
        );
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(SpanChirho::DUMMY_CHIRHO, Some(double_origin_chirho)),
            "Num",
            &double_ty_chirho,
        );
        let mut subst_chirho = SubstChirho::empty_chirho();
        if let TyChirho::VarChirho(var_chirho) = int_ty_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::int_chirho());
        }
        if let TyChirho::VarChirho(var_chirho) = double_ty_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::ConChirho("Double".to_string()));
        }
        let (by_span_chirho, by_origin_chirho) =
            ctx_chirho.finalize_literal_evidence_chirho(&subst_chirho);
        assert!(by_span_chirho.is_empty(), "{by_span_chirho:?}");
        let key_chirho = |origin_chirho| by_origin_chirho[&origin_chirho].ty_key_chirho.clone();
        assert_eq!(key_chirho(int_origin_chirho), "Int");
        assert_eq!(key_chirho(double_origin_chirho), "Double");
    }

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
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(10), None),
            "Num",
            &int_lit_chirho,
        );
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(20), None),
            "Num",
            &open_lit_chirho,
        );
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(30), None),
            "Num",
            &TyChirho::ForallVarChirho("a%1".to_string()),
        );
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(SpanChirho::DUMMY_CHIRHO, None),
            "Num",
            &TyChirho::int_chirho(),
        );
        let mut subst_chirho = SubstChirho::empty_chirho();
        if let TyChirho::VarChirho(var_chirho) = int_lit_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::ConChirho("Double".to_string()));
        }
        let (evidence_chirho, _by_origin_chirho) =
            ctx_chirho.finalize_literal_evidence_chirho(&subst_chirho);
        assert_eq!(
            evidence_chirho.get(&span_chirho(10)),
            Some(&LiteralEvidenceChirho {
                class_name_chirho: "Num".to_string(),
                ty_key_chirho: "Double".to_string(),
            })
        );
        assert_eq!(evidence_chirho.len(), 1, "{evidence_chirho:?}");
    }

    /// Finalize one `Show` reference capture at a genuine span, resolved to Int.
    fn finalize_one_reference_chirho(
        origin_chirho: Option<OriginIdChirho>,
    ) -> (
        HashMap<SpanChirho, Vec<ReferenceEvidenceChirho>>,
        HashMap<OriginIdChirho, Vec<ReferenceEvidenceChirho>>,
    ) {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let ty_chirho = ctx_chirho.fresh_var_chirho();
        ctx_chirho.capture_reference_evidence_chirho(
            span_chirho(50),
            origin_chirho,
            &[PredChirho::new_chirho("Show", ty_chirho.clone())],
        );
        let mut subst_chirho = SubstChirho::empty_chirho();
        if let TyChirho::VarChirho(var_chirho) = ty_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::int_chirho());
        }
        ctx_chirho.finalize_reference_evidence_chirho(&subst_chirho)
    }

    #[test]
    fn an_origin_owned_capture_is_never_published_as_span_evidence_chirho() {
        // gpt_chirho review F2 (#24598): a capture owned by origin A at a
        // genuine span S must not also become legacy evidence at S.
        let mut supply_chirho =
            haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
        let owner_chirho = supply_chirho.fresh_chirho();
        let (by_span_chirho, by_origin_chirho) = finalize_one_reference_chirho(Some(owner_chirho));
        assert!(by_span_chirho.is_empty(), "{by_span_chirho:?}");
        assert_eq!(by_origin_chirho[&owner_chirho].len(), 1);
    }

    #[test]
    fn a_legacy_capture_keeps_its_span_evidence_chirho() {
        let (by_span_chirho, by_origin_chirho) = finalize_one_reference_chirho(None);
        assert!(by_origin_chirho.is_empty());
        assert_eq!(
            by_span_chirho[&span_chirho(50)][0].ty_key_chirho.as_deref(),
            Some("Int")
        );
    }

    #[test]
    fn a_generated_reference_is_keyed_by_its_origin_and_never_by_its_placeholder_chirho() {
        // Two generated references under one placeholder span need different
        // evidence (Bool, Int). The span map must not hold the placeholder at
        // all; the origin map keeps each reference's own proof, and an
        // occurrence with two predicates keeps both.
        let mut supply_chirho =
            haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
        let first_origin_chirho = supply_chirho.fresh_chirho();
        let second_origin_chirho = supply_chirho.fresh_chirho();
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let first_ty_chirho = ctx_chirho.fresh_var_chirho();
        let second_ty_chirho = ctx_chirho.fresh_var_chirho();
        ctx_chirho.capture_reference_evidence_chirho(
            SpanChirho::DUMMY_CHIRHO,
            Some(first_origin_chirho),
            &[PredChirho::new_chirho("Show", first_ty_chirho.clone())],
        );
        ctx_chirho.capture_reference_evidence_chirho(
            SpanChirho::DUMMY_CHIRHO,
            Some(second_origin_chirho),
            &[
                PredChirho::new_chirho("Show", second_ty_chirho.clone()),
                PredChirho::new_chirho("Num", second_ty_chirho.clone()),
            ],
        );
        let mut subst_chirho = SubstChirho::empty_chirho();
        if let TyChirho::VarChirho(var_chirho) = first_ty_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::ConChirho("Bool".to_string()));
        }
        if let TyChirho::VarChirho(var_chirho) = second_ty_chirho {
            subst_chirho.insert_chirho(var_chirho, TyChirho::int_chirho());
        }
        let (by_span_chirho, by_origin_chirho) =
            ctx_chirho.finalize_reference_evidence_chirho(&subst_chirho);
        assert!(by_span_chirho.is_empty(), "{by_span_chirho:?}");
        let keys_chirho = |origin_chirho| -> Vec<(String, Option<String>)> {
            by_origin_chirho[&origin_chirho]
                .iter()
                .map(|record_chirho| {
                    (
                        record_chirho.class_name_chirho.clone(),
                        record_chirho.ty_key_chirho.clone(),
                    )
                })
                .collect()
        };
        assert_eq!(
            keys_chirho(first_origin_chirho),
            vec![("Show".to_string(), Some("Bool".to_string()))]
        );
        assert_eq!(
            keys_chirho(second_origin_chirho),
            vec![
                ("Show".to_string(), Some("Int".to_string())),
                ("Num".to_string(), Some("Int".to_string())),
            ]
        );
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
            None,
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
        let (evidence_chirho, _by_origin_chirho) =
            ctx_chirho.finalize_reference_evidence_chirho(&subst_chirho);
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
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(5), None),
            "Num",
            &TyChirho::int_chirho(),
        );
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(5), None),
            "Num",
            &TyChirho::ConChirho("Double".to_string()),
        );
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(6), None),
            "Num",
            &TyChirho::int_chirho(),
        );
        ctx_chirho.capture_literal_evidence_chirho(
            &test_int_literal_chirho(span_chirho(6), None),
            "Num",
            &TyChirho::int_chirho(),
        );
        let (evidence_chirho, _by_origin_chirho) =
            ctx_chirho.finalize_literal_evidence_chirho(&SubstChirho::empty_chirho());
        assert!(!evidence_chirho.contains_key(&span_chirho(5)));
        assert_eq!(evidence_chirho[&span_chirho(6)].ty_key_chirho, "Int");
    }
}
