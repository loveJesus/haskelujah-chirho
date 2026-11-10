// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lexical evidence owners for expression sigmas, transported with their captures.
//! workflow: language-features-chirho/rank-n-visible-type-application-chirho

use super::super::equality_normalization_chirho::copy_term_with_budget_chirho;
use super::super::*;

const SIGMA_DICTIONARY_PREFIX_CHIRHO: &str = "$sigma:";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SigmaScopeIdChirho(u32);

impl SigmaScopeIdChirho {
    pub fn parameter_key_chirho(self, index_chirho: usize) -> String {
        format!("{SIGMA_DICTIONARY_PREFIX_CHIRHO}{}:{index_chirho}", self.0)
    }
}

pub fn is_sigma_dictionary_key_chirho(key_chirho: &str) -> bool {
    key_chirho
        .strip_prefix(SIGMA_DICTIONARY_PREFIX_CHIRHO)
        .and_then(|rest_chirho| rest_chirho.split_once(':'))
        .is_some_and(|(scope_chirho, index_chirho)| {
            scope_chirho.parse::<u32>().is_ok() && index_chirho.parse::<usize>().is_ok()
        })
}

/// Source transport only: mixed lambdas share an expression span, while each
/// written binder has its own location. Duplicate joins still refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SigmaExpressionKeyChirho {
    pub span_chirho: SpanChirho,
    binder_span_chirho: Option<SpanChirho>,
    function_chirho: bool,
}

impl SigmaExpressionKeyChirho {
    pub fn from_expression_chirho(expression_chirho: &ExprChirho) -> Self {
        let binder_span_chirho = match expression_chirho {
            ExprChirho::TypeLamChirho { binder_chirho, .. } => Some(binder_chirho.span_chirho()),
            ExprChirho::LamChirho { pats_chirho, .. } => pats_chirho
                .first()
                .map(|pattern_chirho| pattern_chirho.span_chirho()),
            _ => None,
        };
        Self {
            span_chirho: expression_chirho.span_chirho(),
            binder_span_chirho,
            function_chirho: false,
        }
    }

    pub fn from_function_chirho(span_chirho: SpanChirho) -> Self {
        Self {
            span_chirho,
            binder_span_chirho: None,
            function_chirho: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SigmaScopeChirho {
    pub id_chirho: SigmaScopeIdChirho,
    pub expression_chirho: SigmaExpressionKeyChirho,
    pub value_arguments_chirho: usize,
    pub predicates_chirho: Arc<[PredChirho]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SigmaBinderKeyChirho {
    OriginChirho(OriginIdChirho),
    SpanChirho(SpanChirho),
}

pub fn sigma_binder_key_chirho(name_chirho: &NameChirho) -> Option<SigmaBinderKeyChirho> {
    match name_chirho.origin_chirho() {
        Some(origin_chirho) => Some(SigmaBinderKeyChirho::OriginChirho(origin_chirho)),
        None if name_chirho.span_chirho() != SpanChirho::DUMMY_CHIRHO => {
            Some(SigmaBinderKeyChirho::SpanChirho(name_chirho.span_chirho()))
        }
        None => None,
    }
}

#[derive(Debug, Clone)]
pub struct SigmaLocalBindingChirho {
    pub name_chirho: String,
    pub scheme_chirho: SchemeChirho,
}

#[derive(Debug, Clone, Default)]
pub struct SigmaEvidenceChirho {
    scopes_chirho: Vec<Arc<SigmaScopeChirho>>,
    local_bindings_chirho: HashMap<SigmaBinderKeyChirho, Option<Arc<SigmaLocalBindingChirho>>>,
}

impl SigmaEvidenceChirho {
    pub fn scopes_chirho(&self) -> &[Arc<SigmaScopeChirho>] {
        &self.scopes_chirho
    }

    pub fn local_bindings_chirho(
        &self,
    ) -> &HashMap<SigmaBinderKeyChirho, Option<Arc<SigmaLocalBindingChirho>>> {
        &self.local_bindings_chirho
    }
}

#[derive(Debug)]
struct SigmaContextChirho {
    scope_chirho: Arc<SigmaScopeChirho>,
    parent_chirho: Option<Arc<SigmaContextChirho>>,
}

/// The existing instance capture and expression-given capture share one immutable
/// lexical view. Capturing a reference/literal clones one Arc, not its ancestors.
#[derive(Debug, Default)]
pub(in crate::infer_chirho) struct DictionaryContextChirho {
    pub(in crate::infer_chirho) instance_chirho: Option<Arc<[PredChirho]>>,
    sigma_chirho: Option<Arc<SigmaContextChirho>>,
}

impl InferCtxChirho {
    pub(in crate::infer_chirho) fn with_sigma_evidence_chirho<ResultChirho>(
        &mut self,
        expression_chirho: SigmaExpressionKeyChirho,
        given_start_chirho: usize,
        check_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> Option<ResultChirho> {
        self.with_sigma_ranges_chirho(
            expression_chirho,
            std::iter::once((0, given_start_chirho..self.given_preds_chirho.len())),
            check_chirho,
        )
    }

    pub(in crate::infer_chirho) fn with_sigma_spine_evidence_chirho<ResultChirho>(
        &mut self,
        expression_chirho: SigmaExpressionKeyChirho,
        given_start_chirho: usize,
        positions_chirho: &[usize],
        check_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> Option<ResultChirho> {
        if self
            .given_preds_chirho
            .len()
            .checked_sub(given_start_chirho)
            != Some(positions_chirho.len())
        {
            self.type_normalization_exhausted_chirho.set(true);
            return None;
        }
        let ranges_chirho = positions_chirho
            .chunk_by(|left_chirho, right_chirho| left_chirho == right_chirho)
            .scan(given_start_chirho, |start_chirho, group_chirho| {
                let end_chirho = *start_chirho + group_chirho.len();
                let range_chirho = *start_chirho..end_chirho;
                *start_chirho = end_chirho;
                Some((group_chirho[0], range_chirho))
            });
        self.with_sigma_ranges_chirho(expression_chirho, ranges_chirho, check_chirho)
    }

    fn with_sigma_ranges_chirho<ResultChirho>(
        &mut self,
        expression_chirho: SigmaExpressionKeyChirho,
        ranges_chirho: impl Iterator<Item = (usize, std::ops::Range<usize>)>,
        check_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> Option<ResultChirho> {
        let previous_chirho = self.active_dictionary_context_chirho.clone();
        let mut current_chirho = previous_chirho.clone();
        for (position_chirho, range_chirho) in ranges_chirho {
            let Some(scope_chirho) =
                self.create_sigma_scope_chirho(expression_chirho, position_chirho, range_chirho)
            else {
                self.type_normalization_exhausted_chirho.set(true);
                return None;
            };
            let Some(scope_chirho) = scope_chirho else {
                continue;
            };
            current_chirho = Some(Arc::new(DictionaryContextChirho {
                instance_chirho: previous_chirho
                    .as_ref()
                    .and_then(|context_chirho| context_chirho.instance_chirho.clone()),
                sigma_chirho: Some(Arc::new(SigmaContextChirho {
                    scope_chirho,
                    parent_chirho: current_chirho
                        .as_ref()
                        .and_then(|context_chirho| context_chirho.sigma_chirho.clone()),
                })),
            }));
        }
        self.active_dictionary_context_chirho = current_chirho;
        let result_chirho = check_chirho(self);
        self.active_dictionary_context_chirho = previous_chirho;
        Some(result_chirho)
    }

    fn create_sigma_scope_chirho(
        &mut self,
        expression_chirho: SigmaExpressionKeyChirho,
        value_arguments_chirho: usize,
        given_range_chirho: std::ops::Range<usize>,
    ) -> Option<Option<Arc<SigmaScopeChirho>>> {
        let span_chirho = expression_chirho.span_chirho;
        let givens_chirho: Vec<_> = self
            .given_preds_chirho
            .get(given_range_chirho)?
            .iter()
            .filter(|predicate_chirho| predicate_chirho.class_name_chirho != "~")
            .collect();
        if givens_chirho.is_empty() {
            return Some(None);
        }
        let mut budget_chirho = self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
            .ok()?;
        let initial_chirho = budget_chirho.remaining_chirho();
        let copied_chirho = (|| {
            budget_chirho.node_chirho(1)?;
            givens_chirho
                .iter()
                .map(|given_chirho| {
                    budget_chirho.node_chirho(given_chirho.class_name_chirho.len() + 1)?;
                    Some(PredChirho::from_proof_chirho(
                        given_chirho.class_identity_chirho().clone(),
                        crate::class_chirho::PredPartsChirho {
                            class_name_chirho: given_chirho.class_name_chirho.clone(),
                            kind_tys_chirho: given_chirho
                                .kind_tys_chirho
                                .iter()
                                .map(|ty_chirho| {
                                    copy_term_with_budget_chirho(ty_chirho, &mut budget_chirho)
                                })
                                .collect::<Option<_>>()?,
                            ty_chirho: copy_term_with_budget_chirho(
                                &given_chirho.ty_chirho,
                                &mut budget_chirho,
                            )?,
                            extra_tys_chirho: given_chirho
                                .extra_tys_chirho
                                .iter()
                                .map(|ty_chirho| {
                                    copy_term_with_budget_chirho(ty_chirho, &mut budget_chirho)
                                })
                                .collect::<Option<_>>()?,
                        },
                    ))
                })
                .collect::<Option<Vec<_>>>()
        })();
        self.elaboration_state_chirho
            .settle_equality_budget_chirho(
                initial_chirho,
                &budget_chirho,
                copied_chirho.is_none(),
                span_chirho,
            )
            .ok()?;
        let id_chirho =
            SigmaScopeIdChirho(u32::try_from(self.sigma_evidence_chirho.scopes_chirho.len()).ok()?);
        let scope_chirho = Arc::new(SigmaScopeChirho {
            id_chirho,
            expression_chirho,
            value_arguments_chirho,
            predicates_chirho: copied_chirho?.into(),
        });
        self.sigma_evidence_chirho
            .scopes_chirho
            .push(scope_chirho.clone());
        Some(Some(scope_chirho))
    }

    pub(in crate::infer_chirho) fn replace_instance_dictionary_context_chirho(
        &mut self,
        instance_chirho: Arc<[PredChirho]>,
    ) -> Option<Arc<DictionaryContextChirho>> {
        let sigma_chirho = self
            .active_dictionary_context_chirho
            .as_ref()
            .and_then(|context_chirho| context_chirho.sigma_chirho.clone());
        self.active_dictionary_context_chirho
            .replace(Arc::new(DictionaryContextChirho {
                instance_chirho: Some(instance_chirho),
                sigma_chirho,
            }))
    }

    /// Reuse the checker's identity-aware entailment. A field's proof names
    /// its own ordered parameter, including a proof-selected superclass.
    pub(in crate::infer_chirho) fn dictionary_context_key_chirho(
        &self,
        wanted_chirho: &PredChirho,
        context_chirho: Option<&DictionaryContextChirho>,
        subst_chirho: &SubstChirho,
    ) -> Option<String> {
        let context_chirho = context_chirho?;
        let wanted_chirho = wanted_chirho.mapped_chirho(|ty_chirho| {
            self.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(ty_chirho))
        });
        let mut current_chirho = context_chirho.sigma_chirho.as_deref();
        let mut steps_chirho = 65_536_usize;
        while let Some(sigma_chirho) = current_chirho {
            for (index_chirho, given_chirho) in sigma_chirho
                .scope_chirho
                .predicates_chirho
                .iter()
                .enumerate()
            {
                if steps_chirho == 0 {
                    self.type_normalization_exhausted_chirho.set(true);
                    return None;
                }
                steps_chirho -= 1;
                let given_chirho = given_chirho.mapped_chirho(|ty_chirho| {
                    self.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(ty_chirho))
                });
                match self
                    .class_env_chirho
                    .superclass_entails_chirho(&given_chirho, &wanted_chirho)
                {
                    Ok(true) => {
                        return Some(
                            sigma_chirho
                                .scope_chirho
                                .id_chirho
                                .parameter_key_chirho(index_chirho),
                        );
                    }
                    Ok(false) => {}
                    Err(_) => {
                        self.type_normalization_exhausted_chirho.set(true);
                        return None;
                    }
                }
            }
            current_chirho = sigma_chirho.parent_chirho.as_deref();
        }
        self.instance_context_key_chirho(
            &wanted_chirho,
            context_chirho.instance_chirho.as_deref(),
            subst_chirho,
        )
    }

    pub(in crate::infer_chirho) fn record_sigma_local_binding_chirho(
        &mut self,
        name_chirho: &NameChirho,
        scheme_chirho: &SchemeChirho,
    ) {
        if scheme_chirho.preds_chirho.is_empty() {
            return;
        }
        let Some(origin_chirho) = sigma_binder_key_chirho(name_chirho) else {
            return;
        };
        let span_chirho = name_chirho.span_chirho();
        let Ok(mut budget_chirho) = self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        else {
            return;
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let copied_chirho = (|| {
            budget_chirho.node_chirho(name_chirho.text_chirho().len() + 1)?;
            let ty_chirho =
                copy_term_with_budget_chirho(&scheme_chirho.ty_chirho, &mut budget_chirho)?;
            let predicates_chirho = scheme_chirho
                .preds_chirho
                .iter()
                .map(|predicate_chirho| {
                    budget_chirho.node_chirho(predicate_chirho.class_name_chirho.len() + 1)?;
                    Some(crate::ty_chirho::SchemePredChirho::from_proof_chirho(
                        predicate_chirho.class_identity_chirho().clone(),
                        crate::ty_chirho::SchemePredPartsChirho {
                            class_name_chirho: predicate_chirho.class_name_chirho.clone(),
                            kind_tys_chirho: predicate_chirho
                                .kind_tys_chirho
                                .iter()
                                .map(|ty_chirho| {
                                    copy_term_with_budget_chirho(ty_chirho, &mut budget_chirho)
                                })
                                .collect::<Option<_>>()?,
                            ty_chirho: copy_term_with_budget_chirho(
                                &predicate_chirho.ty_chirho,
                                &mut budget_chirho,
                            )?,
                            extra_tys_chirho: predicate_chirho
                                .extra_tys_chirho
                                .iter()
                                .map(|ty_chirho| {
                                    copy_term_with_budget_chirho(ty_chirho, &mut budget_chirho)
                                })
                                .collect::<Option<_>>()?,
                        },
                    ))
                })
                .collect::<Option<Vec<_>>>()?;
            budget_chirho.node_chirho(scheme_chirho.vars_chirho.len())?;
            Some(Arc::new(SigmaLocalBindingChirho {
                name_chirho: name_chirho.text_chirho().to_string(),
                scheme_chirho: SchemeChirho {
                    vars_chirho: scheme_chirho.vars_chirho.clone(),
                    preds_chirho: predicates_chirho,
                    ty_chirho,
                },
            }))
        })();
        if self
            .elaboration_state_chirho
            .settle_equality_budget_chirho(
                initial_chirho,
                &budget_chirho,
                copied_chirho.is_none(),
                span_chirho,
            )
            .is_err()
        {
            return;
        }
        let Some(binding_chirho) = copied_chirho else {
            return;
        };
        use std::collections::hash_map::Entry;
        match self
            .sigma_evidence_chirho
            .local_bindings_chirho
            .entry(origin_chirho)
        {
            Entry::Vacant(entry_chirho) => {
                entry_chirho.insert(Some(binding_chirho));
            }
            Entry::Occupied(mut entry_chirho) => {
                if !entry_chirho.get().as_ref().is_some_and(|previous_chirho| {
                    previous_chirho.name_chirho == binding_chirho.name_chirho
                        && previous_chirho.scheme_chirho == binding_chirho.scheme_chirho
                }) {
                    entry_chirho.insert(None);
                }
            }
        }
    }
}
