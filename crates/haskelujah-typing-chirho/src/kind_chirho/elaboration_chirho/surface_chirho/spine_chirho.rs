// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declared saturation and result-kind application share checked ownership.

use super::prefix_chirho::ResultKindPrefixChirho;
use super::*;

impl TypeElaboratorChirho<'_> {
    pub(super) fn infer_spine_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        let span_chirho = syntax_chirho.span_chirho();
        let mut head_chirho =
            std::mem::replace(syntax_chirho, TypeChirho::WildcardChirho { span_chirho });
        let mut arguments_chirho = Vec::new();
        loop {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            match head_chirho {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    arguments_chirho.push((false, false, *arg_chirho));
                    head_chirho = *fun_chirho;
                }
                TypeChirho::KindApplicationChirho {
                    fun_chirho,
                    arg_chirho,
                    written_chirho,
                    ..
                } => {
                    arguments_chirho.push((true, written_chirho, *arg_chirho));
                    head_chirho = *fun_chirho;
                }
                _ => break,
            }
        }
        arguments_chirho.reverse();
        let Some(scheme_chirho) = self.scheme_chirho(&mut head_chirho)? else {
            if self.strict_argument_chirho
                && matches!(
                    head_chirho,
                    TypeChirho::ConChirho(_) | TypeChirho::PromotedConChirho { .. }
                )
            {
                *syntax_chirho = head_chirho;
                return Err(Self::error_chirho(
                    "required type argument has no checked declaration telescope",
                    span_chirho,
                ));
            }
            // No kind/nominal authority is manufactured for a generic stub.
            // Such a head is an explicit unsupported boundary of this vertical.
            if matches!(
                &head_chirho,
                TypeChirho::ConChirho(_)
                    | TypeChirho::VarChirho(_)
                    | TypeChirho::ParenChirho { .. }
                    | TypeChirho::KindAnnotationChirho { .. }
                    | TypeChirho::ForallChirho { .. }
                    | TypeChirho::RequiredForallChirho { .. }
            ) {
                let mut typed_chirho = if let TypeChirho::ConChirho(name_chirho) = &head_chirho {
                    // An opaque interface entry supplies no declaration proof. Preserve
                    // ordinary checking conservatively without publishing a telescope.
                    ElaboratedTypeChirho {
                        term_chirho: TyChirho::ConChirho(self.written_name_chirho(name_chirho)?),
                        kind_chirho: self.fresh_kind_chirho(span_chirho)?,
                    }
                } else {
                    self.infer_type_chirho(&mut head_chirho)?
                };
                let resolved_kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&typed_chirho.kind_chirho, span_chirho)?;
                let kind_chirho = self
                    .state_chirho
                    .instantiate_checked_declaration_term_chirho(
                        &resolved_kind_chirho,
                        &SubstChirho::empty_chirho(),
                        true,
                        &[],
                        span_chirho,
                    )?;
                let mut prefix_chirho = ResultKindPrefixChirho::new_chirho(kind_chirho, false);
                for (kind_argument_chirho, written_chirho, mut argument_chirho) in arguments_chirho
                {
                    self.apply_result_kind_argument_chirho(
                        &mut prefix_chirho,
                        &mut head_chirho,
                        &mut typed_chirho.term_chirho,
                        &mut argument_chirho,
                        (kind_argument_chirho, written_chirho),
                    )?;
                }
                typed_chirho.kind_chirho = prefix_chirho.finish_chirho(self, span_chirho)?;
                *syntax_chirho = head_chirho;
                return Ok(typed_chirho);
            }
            *syntax_chirho = head_chirho;
            return Err(Self::error_chirho(
                "type constructor has no proven declaration telescope",
                span_chirho,
            ));
        };
        let (binders_chirho, mut result_kind_chirho, synonym_rhs_chirho) =
            self.instantiate_header_chirho(&scheme_chirho, span_chirho)?;
        let mut term_chirho = TyChirho::ConChirho(
            self.state_chirho
                .name_chirho(&scheme_chirho.identity_chirho, span_chirho)?,
        );
        let mut cursor_chirho = 0;
        let mut extra_arguments_chirho = Vec::new();
        let mut result_prefix_chirho = None;
        for (kind_argument_chirho, written_chirho, mut argument_chirho) in arguments_chirho {
            while let Some(binder_chirho) = binders_chirho.get(cursor_chirho) {
                let automatic_chirho = (binder_chirho.visibility_chirho
                    == TelescopeVisibilityChirho::InferredChirho
                    && (!kind_argument_chirho || written_chirho))
                    || (!kind_argument_chirho
                        && binder_chirho.visibility_chirho
                            != TelescopeVisibilityChirho::RequiredChirho);
                if !automatic_chirho {
                    break;
                }
                let argument_term_chirho = TyChirho::VarChirho(binder_chirho.variable_chirho);
                let argument_syntax_chirho =
                    self.surface_chirho(&argument_term_chirho, span_chirho)?;
                head_chirho = TypeChirho::KindApplicationChirho {
                    fun_chirho: Box::new(head_chirho),
                    arg_chirho: Box::new(argument_syntax_chirho),
                    written_chirho: false,
                    span_chirho,
                };
                term_chirho =
                    TyChirho::AppChirho(Box::new(term_chirho), Box::new(argument_term_chirho));
                cursor_chirho += 1;
            }
            if cursor_chirho == binders_chirho.len() {
                if result_prefix_chirho.is_none() {
                    result_prefix_chirho = Some(ResultKindPrefixChirho::new_chirho(
                        self.state_chirho
                            .copy_chirho(&result_kind_chirho, span_chirho)?,
                        synonym_rhs_chirho.is_some(),
                    ));
                }
                self.apply_result_kind_argument_chirho(
                    result_prefix_chirho.as_mut().unwrap(),
                    &mut head_chirho,
                    &mut term_chirho,
                    &mut argument_chirho,
                    (kind_argument_chirho, written_chirho),
                )?;
                continue;
            }
            let elaborated_argument_chirho = if let Some(binder_chirho) =
                binders_chirho.get(cursor_chirho)
            {
                self.infer_type_at_kind_chirho(&mut argument_chirho, &binder_chirho.kind_chirho)?
            } else {
                let expected_chirho = self
                    .state_chirho
                    .resolve_chirho(&result_kind_chirho, argument_chirho.span_chirho())?;
                if let TyChirho::FunChirho(domain_chirho, _, _) = expected_chirho {
                    // Result arrows are outside the declaration's saturation
                    // boundary, but their argument kinds are still authoritative.
                    self.infer_type_at_kind_chirho(&mut argument_chirho, &domain_chirho)?
                } else {
                    self.infer_type_chirho(&mut argument_chirho)?
                }
            };
            if let Some(binder_chirho) = binders_chirho.get(cursor_chirho) {
                let required_chirho =
                    binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho;
                if kind_argument_chirho == required_chirho {
                    return Err(Self::error_chirho(
                        "type/kind argument visibility does not match the declaration telescope",
                        argument_chirho.span_chirho(),
                    ));
                }
                self.state_chirho.unify_chirho(
                    &elaborated_argument_chirho.kind_chirho,
                    &binder_chirho.kind_chirho,
                    argument_chirho.span_chirho(),
                )?;
                self.state_chirho.unify_chirho(
                    &TyChirho::VarChirho(binder_chirho.variable_chirho),
                    &elaborated_argument_chirho.term_chirho,
                    argument_chirho.span_chirho(),
                )?;
                cursor_chirho += 1;
            } else {
                if kind_argument_chirho {
                    return Err(Self::error_chirho(
                        "no invisible binder remains for written kind application",
                        argument_chirho.span_chirho(),
                    ));
                }
                if synonym_rhs_chirho.is_some() {
                    extra_arguments_chirho.push((
                        false,
                        self.state_chirho
                            .copy_chirho(&elaborated_argument_chirho.term_chirho, span_chirho)?,
                    ));
                }
                let next_kind_chirho = self.fresh_kind_chirho(argument_chirho.span_chirho())?;
                let argument_kind_chirho = self
                    .state_chirho
                    .copy_chirho(&elaborated_argument_chirho.kind_chirho, span_chirho)?;
                let expected_kind_chirho = ElaborationStateChirho::arrow_chirho(
                    argument_kind_chirho,
                    self.state_chirho
                        .copy_chirho(&next_kind_chirho, span_chirho)?,
                );
                self.state_chirho.unify_chirho(
                    &result_kind_chirho,
                    &expected_kind_chirho,
                    argument_chirho.span_chirho(),
                )?;
                result_kind_chirho = next_kind_chirho;
            }
            term_chirho = TyChirho::AppChirho(
                Box::new(term_chirho),
                Box::new(elaborated_argument_chirho.term_chirho),
            );
            head_chirho = if kind_argument_chirho {
                TypeChirho::KindApplicationChirho {
                    fun_chirho: Box::new(head_chirho),
                    arg_chirho: Box::new(argument_chirho),
                    written_chirho,
                    span_chirho,
                }
            } else {
                TypeChirho::AppChirho {
                    fun_chirho: Box::new(head_chirho),
                    arg_chirho: Box::new(argument_chirho),
                    span_chirho,
                }
            };
        }
        if let Some(mut prefix_chirho) = result_prefix_chirho {
            if let Some(arguments_chirho) = prefix_chirho.applications_chirho.take() {
                extra_arguments_chirho.extend(arguments_chirho);
            }
            result_kind_chirho = prefix_chirho.finish_chirho(self, span_chirho)?;
        }
        while let Some(binder_chirho) = binders_chirho.get(cursor_chirho) {
            if binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho {
                break;
            }
            let argument_term_chirho = TyChirho::VarChirho(binder_chirho.variable_chirho);
            let argument_chirho = self.surface_chirho(&argument_term_chirho, span_chirho)?;
            head_chirho = TypeChirho::KindApplicationChirho {
                fun_chirho: Box::new(head_chirho),
                arg_chirho: Box::new(argument_chirho),
                written_chirho: false,
                span_chirho,
            };
            term_chirho =
                TyChirho::AppChirho(Box::new(term_chirho), Box::new(argument_term_chirho));
            cursor_chirho += 1;
        }
        for binder_chirho in binders_chirho[cursor_chirho..].iter().rev() {
            let kind_chirho = self
                .state_chirho
                .resolve_chirho(&binder_chirho.kind_chirho, span_chirho)?;
            result_kind_chirho =
                ElaborationStateChirho::arrow_chirho(kind_chirho, result_kind_chirho);
        }
        if let Some(rhs_chirho) = synonym_rhs_chirho {
            if cursor_chirho < binders_chirho.len() {
                return Err(Self::error_chirho(
                    "type synonym is not saturated at its declaration boundary",
                    span_chirho,
                ));
            }
            term_chirho = self.state_chirho.resolve_chirho(&rhs_chirho, span_chirho)?;
            for (quantified_chirho, argument_chirho) in extra_arguments_chirho {
                term_chirho = self.apply_checked_kind_argument_chirho(
                    term_chirho,
                    argument_chirho,
                    quantified_chirho,
                    span_chirho,
                )?;
            }
        }
        *syntax_chirho = head_chirho;
        Ok(ElaboratedTypeChirho {
            term_chirho: self
                .state_chirho
                .resolve_chirho(&term_chirho, span_chirho)?,
            kind_chirho: self
                .state_chirho
                .resolve_chirho(&result_kind_chirho, span_chirho)?,
        })
    }
}
