// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded syntax copies and the transient trail of a resumed solver.

use super::*;

impl ElaborationStateChirho {
    pub(in crate::kind_chirho::elaboration_chirho) fn begin_inference_links_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(0, 0, span_chirho)?;
        if self.inference_links_chirho.is_some() {
            return Err(ElaborationErrorChirho {
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
