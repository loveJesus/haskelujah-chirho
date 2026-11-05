// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded syntax copies and the transient trail of a resumed solver.

use super::*;

impl ElaborationStateChirho {
    pub(crate) fn lend_equality_budget_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<crate::infer_chirho::EqualityBudgetChirho, ElaborationErrorChirho> {
        self.charge_chirho(0, 0, span_chirho)?;
        Ok(crate::infer_chirho::EqualityBudgetChirho::limited_chirho(
            self.work_left_chirho,
            self.output_left_chirho,
        ))
    }

    pub(crate) fn settle_equality_budget_chirho(
        &mut self,
        initial_chirho: (usize, usize),
        budget_chirho: &crate::infer_chirho::EqualityBudgetChirho,
        exhausted_chirho: bool,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let remaining_chirho = budget_chirho.remaining_chirho();
        self.charge_chirho(
            initial_chirho.0 - remaining_chirho.0,
            initial_chirho.1 - remaining_chirho.1,
            span_chirho,
        )?;
        if exhausted_chirho {
            return Err(self.resource_failure_chirho(
                "source signature equality probe bound exhausted",
                span_chirho,
            ));
        }
        Ok(())
    }

    /// Lend only the original remaining allowance. Even a failed normalization
    /// is charged before the state returns to its ordinary inference owner.
    pub(in crate::kind_chirho::elaboration_chirho) fn normalize_equality_pair_chirho(
        &mut self,
        view_chirho: crate::infer_chirho::EqualityNormalizationViewChirho<'_>,
        catalogue_chirho: &crate::infer_chirho::TypeFamilyEnvChirho,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(TyChirho, TyChirho), ElaborationErrorChirho> {
        self.charge_chirho(0, 0, span_chirho)?;
        let mut budget_chirho = crate::infer_chirho::EqualityBudgetChirho::limited_chirho(
            self.work_left_chirho,
            self.output_left_chirho,
        );
        let initial_chirho = budget_chirho.remaining_chirho();
        let result_chirho = (|| {
            let left_chirho = view_chirho.normalize_chirho(
                catalogue_chirho,
                self.owner_chirho.is_some(),
                self.next_variable_chirho,
                left_chirho,
                &mut budget_chirho,
            )?;
            let right_chirho = view_chirho.normalize_chirho(
                catalogue_chirho,
                self.owner_chirho.is_some(),
                self.next_variable_chirho,
                right_chirho,
                &mut budget_chirho,
            )?;
            Ok::<_, crate::infer_chirho::EqualityNormalizationExhaustedChirho>((
                left_chirho,
                right_chirho,
            ))
        })();
        let remaining_chirho = budget_chirho.remaining_chirho();
        self.charge_chirho(
            initial_chirho.0 - remaining_chirho.0,
            initial_chirho.1 - remaining_chirho.1,
            span_chirho,
        )?;
        result_chirho.map_err(|_| {
            self.resource_failure_chirho(
                "scoped kind equality normalization bound exhausted",
                span_chirho,
            )
        })
    }

    pub(in crate::kind_chirho::elaboration_chirho) fn begin_inference_links_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(0, 0, span_chirho)?;
        if self.inference_links_chirho.is_some() {
            return Err(ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: "nested argument solver handoff".into(),
                span_chirho,
            });
        }
        self.inference_links_chirho = Some(Vec::new());
        Ok(())
    }

    pub(super) fn record_link_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        image_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        if self.inference_links_chirho.is_some() {
            self.charge_chirho(1, 1, span_chirho)?;
            if let Some(links_chirho) = &mut self.inference_links_chirho {
                links_chirho.push(variable_chirho);
            }
        }
        self.substitution_chirho
            .insert_chirho(variable_chirho, image_chirho);
        Ok(())
    }

    pub(in crate::kind_chirho::elaboration_chirho) fn finish_inference_links_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<SubstChirho, ElaborationErrorChirho> {
        let variables_chirho =
            self.inference_links_chirho
                .take()
                .ok_or_else(|| ElaborationErrorChirho {
                    mismatch_chirho: None,
                    message_chirho: "argument solver handoff lost its link trail".into(),
                    span_chirho,
                })?;
        let mut links_chirho = SubstChirho::empty_chirho();
        for variable_chirho in variables_chirho {
            let image_chirho =
                self.resolve_chirho(&TyChirho::VarChirho(variable_chirho), span_chirho)?;
            links_chirho.insert_chirho(variable_chirho, image_chirho);
        }
        Ok(links_chirho)
    }

    fn preflight_syntax_chirho(
        &mut self,
        span_chirho: SpanChirho,
        action_chirho: impl FnOnce(
            &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
        ) -> Result<
            (),
            haskelujah_ast_chirho::decl_chirho::GadtProjectionErrorChirho,
        >,
    ) -> Result<(), ElaborationErrorChirho> {
        use haskelujah_ast_chirho::decl_chirho::{
            GadtProjectionBudgetChirho, GadtProjectionLimitsChirho,
        };
        self.charge_chirho(0, 0, span_chirho)?;
        let initial_chirho = GadtProjectionLimitsChirho {
            work_chirho: self.work_left_chirho,
            output_chirho: self.output_left_chirho,
            depth_chirho: 128,
        };
        let mut budget_chirho = GadtProjectionBudgetChirho::with_limits_chirho(initial_chirho);
        let result_chirho = action_chirho(&mut budget_chirho);
        let remaining_chirho = budget_chirho.remaining_limits_chirho();
        self.charge_chirho(
            initial_chirho.work_chirho - remaining_chirho.work_chirho,
            initial_chirho.output_chirho - remaining_chirho.output_chirho,
            span_chirho,
        )?;
        result_chirho.map_err(|failure_chirho| {
            let error_chirho = ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: format!("kind syntax projection bound exhausted: {failure_chirho}"),
                span_chirho,
            };
            self.error_chirho = Some(error_chirho.clone());
            error_chirho
        })
    }

    pub(in crate::kind_chirho::elaboration_chirho) fn copy_syntax_chirho(
        &mut self,
        syntax_chirho: &haskelujah_ast_chirho::ty_chirho::TypeChirho,
    ) -> Result<haskelujah_ast_chirho::ty_chirho::TypeChirho, ElaborationErrorChirho> {
        self.preflight_syntax_chirho(syntax_chirho.span_chirho(), |budget_chirho| {
            haskelujah_ast_chirho::decl_chirho::check_type_projection_chirho(
                syntax_chirho,
                budget_chirho,
            )
        })?;
        Ok(syntax_chirho.clone())
    }

    pub(in crate::kind_chirho::elaboration_chirho) fn copy_name_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> Result<haskelujah_ast_chirho::name_chirho::NameChirho, ElaborationErrorChirho> {
        self.preflight_syntax_chirho(name_chirho.span_chirho(), |budget_chirho| {
            haskelujah_ast_chirho::decl_chirho::check_name_projection_chirho(
                name_chirho,
                budget_chirho,
            )
        })?;
        Ok(name_chirho.clone())
    }
}
