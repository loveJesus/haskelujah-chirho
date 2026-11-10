// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked lexical evidence transported to actual Core binder IDs.
//! workflow: language-features-chirho/rank-n-visible-type-application-chirho

use crate::desugar_chirho::*;
use haskelujah_typing_chirho::class_chirho::PredChirho;
use haskelujah_typing_chirho::infer_chirho::{
    SigmaEvidenceChirho, SigmaExpressionKeyChirho, SigmaScopeChirho, sigma_binder_key_chirho,
};

#[derive(Debug, Clone)]
pub struct SigmaDictionaryBinderChirho {
    pub scope_chirho: Arc<SigmaScopeChirho>,
    pub index_chirho: usize,
}

#[derive(Debug)]
pub(in crate::desugar_chirho) struct SigmaExpressionJoinChirho {
    scope_chirho: Option<Arc<SigmaScopeChirho>>,
    consumed_chirho: bool,
}

impl SigmaDictionaryBinderChirho {
    pub fn predicate_chirho(&self) -> &PredChirho {
        &self.scope_chirho.predicates_chirho[self.index_chirho]
    }
}

impl DesugarCtxChirho {
    pub(in crate::desugar_chirho) fn install_sigma_evidence_chirho(
        &mut self,
        evidence_chirho: SigmaEvidenceChirho,
    ) -> Result<(), DesugarInputErrorChirho> {
        if let Some(span_chirho) = evidence_chirho.unsupported_required_spine_chirho() {
            return Err(DesugarInputErrorChirho::required_sigma_spine_chirho(
                span_chirho,
            ));
        }
        for binding_chirho in evidence_chirho.local_bindings_chirho().values().flatten() {
            self.constrained_names_chirho
                .insert(binding_chirho.name_chirho.clone());
        }
        for scope_chirho in evidence_chirho.scopes_chirho() {
            if scope_chirho.expression_chirho.span_chirho == SpanChirho::DUMMY_CHIRHO {
                self.sigma_transport_failure_chirho.get_or_insert(
                    DesugarInputErrorChirho::sigma_account_chirho(
                        scope_chirho.expression_chirho.span_chirho,
                    ),
                );
                continue;
            }
            let entry_chirho = self
                .sigma_expression_scopes_chirho
                .entry(scope_chirho.expression_chirho)
                .or_default()
                .entry(scope_chirho.value_arguments_chirho)
                .or_insert_with(|| SigmaExpressionJoinChirho {
                    scope_chirho: Some(scope_chirho.clone()),
                    consumed_chirho: false,
                });
            if entry_chirho
                .scope_chirho
                .as_ref()
                .is_some_and(|previous_chirho| previous_chirho.id_chirho != scope_chirho.id_chirho)
            {
                entry_chirho.scope_chirho = None;
            }
        }
        self.sigma_evidence_chirho = evidence_chirho;
        Ok(())
    }

    /// A span is only the transport join for this checked AST, never the scope
    /// identity. Conflicting records produce no wrapper or dictionary privilege.
    pub(in crate::desugar_chirho) fn abstract_sigma_expression_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        self.abstract_sigma_key_chirho(
            SigmaExpressionKeyChirho::from_expression_chirho(expression_chirho),
            body_chirho,
        )
    }

    pub(in crate::desugar_chirho) fn abstract_sigma_function_chirho(
        &mut self,
        span_chirho: SpanChirho,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        self.abstract_sigma_key_chirho(
            SigmaExpressionKeyChirho::from_function_chirho(span_chirho),
            body_chirho,
        )
    }

    fn abstract_sigma_key_chirho(
        &mut self,
        expression_chirho: SigmaExpressionKeyChirho,
        mut body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let span_chirho = expression_chirho.span_chirho;
        let count_chirho = match self.sigma_expression_scopes_chirho.get(&expression_chirho) {
            Some(joins_chirho) => joins_chirho.len(),
            None => return body_chirho,
        };
        if !self.reserve_sigma_transport_chirho(count_chirho.saturating_add(1), span_chirho) {
            return body_chirho;
        }
        let positions_chirho = self.sigma_expression_scopes_chirho[&expression_chirho]
            .keys()
            .copied()
            .collect::<Vec<_>>();
        // Insert later dictionaries first, so inserted lambdas never count as
        // source value parameters when an earlier position is reached.
        for position_chirho in positions_chirho.into_iter().rev() {
            let join_chirho = self
                .sigma_expression_scopes_chirho
                .get_mut(&expression_chirho)
                .and_then(|joins_chirho| joins_chirho.get_mut(&position_chirho))
                .expect("recorded sigma position Chirho");
            if join_chirho.consumed_chirho || join_chirho.scope_chirho.is_none() {
                self.sigma_transport_failure_chirho
                    .get_or_insert(DesugarInputErrorChirho::sigma_account_chirho(span_chirho));
                return body_chirho;
            }
            let scope_chirho = join_chirho
                .scope_chirho
                .clone()
                .expect("checked sigma scope Chirho");
            join_chirho.consumed_chirho = true;
            let mut target_chirho = &mut body_chirho;
            let mut remaining_chirho = position_chirho;
            while remaining_chirho > 0 {
                if !self.reserve_sigma_transport_chirho(1, span_chirho) {
                    return body_chirho;
                }
                target_chirho = match target_chirho {
                    CoreExprChirho::TyLamChirho { body_chirho, .. } => body_chirho,
                    CoreExprChirho::LamChirho { body_chirho, .. } => {
                        remaining_chirho -= 1;
                        body_chirho
                    }
                    _ => {
                        self.sigma_transport_failure_chirho.get_or_insert(
                            DesugarInputErrorChirho::sigma_account_chirho(span_chirho),
                        );
                        return body_chirho;
                    }
                };
            }
            for index_chirho in (0..scope_chirho.predicates_chirho.len()).rev() {
                let key_chirho = scope_chirho.id_chirho.parameter_key_chirho(index_chirho);
                let Some(budget_chirho) = self.sigma_projection_budget_chirho.as_mut() else {
                    self.sigma_transport_failure_chirho
                        .get_or_insert(DesugarInputErrorChirho::sigma_account_chirho(span_chirho));
                    return body_chirho;
                };
                if let Err(error_chirho) = budget_chirho.reserve_transport_chirho(
                    (key_chirho.len() + 32).saturating_mul(4),
                    1,
                    span_chirho,
                ) {
                    self.sigma_transport_failure_chirho
                        .get_or_insert(error_chirho.into());
                    return body_chirho;
                }
                let binder_chirho = self.fresh_binder_chirho(
                    &format!("$d_sigma_{}_chirho", key_chirho),
                    TyChirho::ConChirho("$DictionaryChirho".to_owned()),
                    span_chirho,
                );
                self.sigma_dictionary_binders_chirho.insert(
                    binder_chirho.id_chirho,
                    SigmaDictionaryBinderChirho {
                        scope_chirho: scope_chirho.clone(),
                        index_chirho,
                    },
                );
                let previous_chirho = std::mem::replace(
                    target_chirho,
                    CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::IntChirho(0)),
                );
                *target_chirho = CoreExprChirho::LamChirho {
                    binder_chirho,
                    body_chirho: Box::new(previous_chirho),
                };
            }
        }
        body_chirho
    }

    fn reserve_sigma_transport_chirho(
        &mut self,
        weight_chirho: usize,
        span_chirho: SpanChirho,
    ) -> bool {
        let Some(budget_chirho) = self.sigma_projection_budget_chirho.as_mut() else {
            self.sigma_transport_failure_chirho
                .get_or_insert(DesugarInputErrorChirho::sigma_account_chirho(span_chirho));
            return false;
        };
        if let Err(error_chirho) =
            budget_chirho.reserve_transport_chirho(weight_chirho, 0, span_chirho)
        {
            self.sigma_transport_failure_chirho
                .get_or_insert(error_chirho.into());
            return false;
        }
        true
    }

    pub(in crate::desugar_chirho) fn finish_sigma_transport_chirho(
        &self,
    ) -> Result<(), DesugarInputErrorChirho> {
        if let Some(error_chirho) = self.sigma_transport_failure_chirho {
            return Err(error_chirho);
        }
        if let Some((span_chirho, _)) =
            self.sigma_expression_scopes_chirho
                .iter()
                .find(|(_, joins_chirho)| {
                    joins_chirho.values().any(|join_chirho| {
                        !join_chirho.consumed_chirho || join_chirho.scope_chirho.is_none()
                    })
                })
        {
            return Err(DesugarInputErrorChirho::sigma_account_chirho(
                span_chirho.span_chirho,
            ));
        }
        Ok(())
    }

    pub(in crate::desugar_chirho) fn record_sigma_pattern_binder_chirho(
        &mut self,
        name_chirho: &NameChirho,
        id_chirho: CoreIdChirho,
    ) {
        let Some(origin_chirho) = sigma_binder_key_chirho(name_chirho) else {
            return;
        };
        let Some(binding_chirho) = self
            .sigma_evidence_chirho
            .local_bindings_chirho()
            .get(&origin_chirho)
        else {
            return;
        };
        let Some(binding_chirho) = binding_chirho else {
            self.sigma_transport_failure_chirho.get_or_insert(
                DesugarInputErrorChirho::sigma_account_chirho(name_chirho.span_chirho()),
            );
            return;
        };
        if binding_chirho.name_chirho != name_chirho.text_chirho() {
            self.sigma_transport_failure_chirho.get_or_insert(
                DesugarInputErrorChirho::sigma_account_chirho(name_chirho.span_chirho()),
            );
            return;
        }
        self.sigma_local_schemes_chirho
            .insert(id_chirho, binding_chirho.clone());
    }
}
