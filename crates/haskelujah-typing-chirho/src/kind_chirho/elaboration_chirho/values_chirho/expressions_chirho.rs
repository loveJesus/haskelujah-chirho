// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expression quotation stages share the original elaborator and allowance.
use super::*;

impl TypeElaboratorChirho<'_> {
    pub(in crate::kind_chirho::elaboration_chirho) fn with_syntax_depth_chirho<ResultChirho>(
        &mut self,
        span_chirho: SpanChirho,
        action_chirho: impl FnOnce(&mut Self) -> Result<ResultChirho, ElaborationErrorChirho>,
    ) -> Result<ResultChirho, ElaborationErrorChirho> {
        if self.type_depth_chirho >= 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "kind elaboration syntax depth bound exhausted",
                span_chirho,
            ));
        }
        self.type_depth_chirho += 1;
        let result_chirho = action_chirho(self);
        self.type_depth_chirho -= 1;
        result_chirho
    }

    pub(super) fn expression_chirho(
        &mut self,
        expression_chirho: &mut ExprChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 0, expression_chirho.span_chirho())?;
        self.with_syntax_depth_chirho(expression_chirho.span_chirho(), |this_chirho| {
            if this_chirho.quote_stage_chirho == 0
                && matches!(expression_chirho, ExprChirho::TypeAppChirho { .. })
                && this_chirho.elaborate_constructor_type_application_chirho(expression_chirho)?
            {
                return Ok(());
            }
            match expression_chirho {
                ExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    this_chirho.expression_chirho(fun_chirho)?;
                    this_chirho.expression_chirho(arg_chirho)?;
                }
                ExprChirho::InfixChirho {
                    left_chirho,
                    right_chirho,
                    ..
                } => {
                    this_chirho.expression_chirho(left_chirho)?;
                    this_chirho.expression_chirho(right_chirho)?;
                }
                ExprChirho::TypeAppChirho {
                    expr_chirho,
                    ty_chirho,
                    ..
                } => {
                    if this_chirho.quote_stage_chirho > 0 {
                        this_chirho.state_chirho.check_retained_syntax_chirho(
                            ty_chirho,
                            128usize.saturating_sub(this_chirho.type_depth_chirho),
                        )?;
                        return this_chirho.expression_chirho(expr_chirho);
                    }
                    let typed_chirho = this_chirho.infer_type_chirho(ty_chirho)?;
                    let span_chirho = ty_chirho.span_chirho();
                    let kind_chirho =
                        this_chirho.surface_chirho(&typed_chirho.kind_chirho, span_chirho)?;
                    let original_chirho =
                        std::mem::replace(ty_chirho, TypeChirho::WildcardChirho { span_chirho });
                    *ty_chirho = TypeChirho::KindAnnotationChirho {
                        ty_chirho: Box::new(original_chirho),
                        kind_chirho: Box::new(kind_chirho),
                        span_chirho,
                    };
                    this_chirho.expression_chirho(expr_chirho)?;
                }
                ExprChirho::AnnChirho {
                    expr_chirho,
                    ty_chirho,
                    ..
                } => {
                    if this_chirho.quote_stage_chirho > 0 {
                        this_chirho.state_chirho.check_retained_syntax_chirho(
                            ty_chirho,
                            128usize.saturating_sub(this_chirho.type_depth_chirho),
                        )?;
                        return this_chirho.expression_chirho(expr_chirho);
                    }
                    this_chirho.scoped_chirho(|this_chirho| {
                        this_chirho.scoped_chirho(|this_chirho| {
                            this_chirho.check_value_type_chirho(ty_chirho)
                        })?;
                        let variables_chirho = this_chirho.checked_signature_scope_chirho(
                            ty_chirho,
                            ty_chirho.span_chirho(),
                            false,
                        )?;
                        for (name_chirho, variable_chirho, kind_chirho) in variables_chirho {
                            this_chirho.bind_variable_chirho(
                                name_chirho,
                                variable_chirho,
                                kind_chirho,
                                ty_chirho.span_chirho(),
                            )?;
                        }
                        this_chirho.expression_chirho(expr_chirho)
                    })?;
                }
                ExprChirho::NegChirho { expr_chirho, .. } => {
                    this_chirho.expression_chirho(expr_chirho)?
                }
                ExprChirho::SpliceChirho { expr_chirho, .. }
                | ExprChirho::TypedSpliceChirho { expr_chirho, .. } => {
                    this_chirho.at_quote_stage_chirho(
                        this_chirho.quote_stage_chirho.saturating_sub(1),
                        |this_chirho| this_chirho.expression_chirho(expr_chirho),
                    )?;
                }
                ExprChirho::QuoteExprChirho {
                    expr_chirho,
                    typed_chirho,
                    span_chirho,
                } => {
                    this_chirho.require_quotation_syntax_chirho(*span_chirho)?;
                    let stage_chirho = if *typed_chirho && this_chirho.quote_stage_chirho == 0 {
                        0
                    } else {
                        this_chirho.quote_stage_chirho + 1
                    };
                    this_chirho.at_quote_stage_chirho(stage_chirho, |this_chirho| {
                        this_chirho.expression_chirho(expr_chirho)
                    })?;
                }
                ExprChirho::ParenChirho { inner_chirho, .. } => {
                    this_chirho.expression_chirho(inner_chirho)?
                }
                ExprChirho::LeftSectionChirho { arg_chirho, .. }
                | ExprChirho::RightSectionChirho { arg_chirho, .. } => {
                    this_chirho.expression_chirho(arg_chirho)?
                }
                ExprChirho::LamChirho {
                    pats_chirho,
                    body_chirho,
                    ..
                } => this_chirho.scoped_chirho(|this_chirho| {
                    for pattern_chirho in pats_chirho {
                        this_chirho.pattern_chirho(pattern_chirho)?;
                    }
                    this_chirho.expression_chirho(body_chirho)
                })?,
                ExprChirho::TypeLamChirho {
                    binder_chirho,
                    body_chirho,
                    span_chirho,
                } => this_chirho.scoped_chirho(|this_chirho| {
                    if this_chirho.quote_stage_chirho > 0 {
                        return this_chirho.expression_chirho(body_chirho);
                    }
                    let kind_chirho = this_chirho.fresh_kind_chirho(*span_chirho)?;
                    let stored_chirho = this_chirho
                        .state_chirho
                        .copy_chirho(&kind_chirho, *span_chirho)?;
                    let variable_chirho = this_chirho
                        .state_chirho
                        .fresh_chirho(Some(stored_chirho), *span_chirho)?;
                    this_chirho
                        .state_chirho
                        .mark_universal_chirho(variable_chirho, *span_chirho)?;
                    this_chirho.stamp_variable_chirho(binder_chirho, variable_chirho)?;
                    let name_chirho = this_chirho
                        .state_chirho
                        .name_chirho(binder_chirho.text_chirho(), *span_chirho)?;
                    this_chirho.bind_variable_chirho(
                        name_chirho,
                        variable_chirho,
                        kind_chirho,
                        *span_chirho,
                    )?;
                    this_chirho.expression_chirho(body_chirho)
                })?,
                ExprChirho::LetChirho {
                    binds_chirho,
                    body_chirho,
                    ..
                } => this_chirho.scoped_chirho(|this_chirho| {
                    this_chirho.local_binds_chirho(binds_chirho)?;
                    this_chirho.expression_chirho(body_chirho)
                })?,
                ExprChirho::IfChirho {
                    cond_chirho,
                    then_chirho,
                    else_chirho,
                    ..
                } => {
                    this_chirho.expression_chirho(cond_chirho)?;
                    this_chirho
                        .scoped_chirho(|this_chirho| this_chirho.expression_chirho(then_chirho))?;
                    this_chirho
                        .scoped_chirho(|this_chirho| this_chirho.expression_chirho(else_chirho))?;
                }
                ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
                    this_chirho.guards_chirho(arms_chirho)?
                }
                ExprChirho::CaseChirho {
                    scrutinee_chirho,
                    alts_chirho,
                    ..
                } => {
                    this_chirho.expression_chirho(scrutinee_chirho)?;
                    for alternative_chirho in alts_chirho {
                        this_chirho.scoped_chirho(|this_chirho| {
                            this_chirho.pattern_chirho(&mut alternative_chirho.pat_chirho)?;
                            this_chirho
                                .local_binds_chirho(&mut alternative_chirho.where_binds_chirho)?;
                            this_chirho.rhs_chirho(&mut alternative_chirho.rhs_chirho)
                        })?;
                    }
                }
                ExprChirho::DoChirho { stmts_chirho, .. } => this_chirho
                    .scoped_chirho(|this_chirho| this_chirho.statements_chirho(stmts_chirho))?,
                ExprChirho::TupleChirho {
                    elements_chirho, ..
                }
                | ExprChirho::ListChirho {
                    elements_chirho, ..
                } => {
                    for element_chirho in elements_chirho {
                        this_chirho.expression_chirho(element_chirho)?;
                    }
                }
                ExprChirho::ArithSeqChirho {
                    from_chirho,
                    then_chirho,
                    to_chirho,
                    ..
                } => {
                    this_chirho.expression_chirho(from_chirho)?;
                    for expression_chirho in then_chirho.iter_mut().chain(to_chirho.iter_mut()) {
                        this_chirho.expression_chirho(expression_chirho)?;
                    }
                }
                ExprChirho::ListCompChirho {
                    body_chirho,
                    quals_chirho,
                    parallel_quals_chirho,
                    ..
                } => this_chirho.scoped_chirho(|this_chirho| {
                    this_chirho.comprehension_using_chirho(quals_chirho)?;
                    for branch_chirho in parallel_quals_chirho.iter_mut() {
                        this_chirho.comprehension_using_chirho(branch_chirho)?;
                    }
                    this_chirho.comprehension_qualifiers_chirho(quals_chirho)?;
                    for branch_chirho in parallel_quals_chirho {
                        this_chirho.scoped_chirho(|this_chirho| {
                            this_chirho.comprehension_qualifiers_chirho(branch_chirho)
                        })?;
                    }
                    this_chirho.expression_chirho(body_chirho)
                })?,
                ExprChirho::RecordConChirho { fields_chirho, .. } => {
                    for field_chirho in fields_chirho {
                        this_chirho.expression_chirho(&mut field_chirho.value_chirho)?;
                    }
                }
                ExprChirho::RecordUpdateChirho {
                    expr_chirho,
                    fields_chirho,
                    ..
                } => {
                    this_chirho.expression_chirho(expr_chirho)?;
                    for field_chirho in fields_chirho {
                        this_chirho.expression_chirho(&mut field_chirho.value_chirho)?;
                    }
                }
                ExprChirho::QuoteDeclChirho {
                    decls_chirho,
                    span_chirho,
                } => {
                    this_chirho.require_quotation_syntax_chirho(*span_chirho)?;
                    this_chirho.at_quote_stage_chirho(
                        this_chirho.quote_stage_chirho + 1,
                        |this_chirho| {
                            for declaration_chirho in decls_chirho {
                                this_chirho.scoped_chirho(|this_chirho| {
                                    this_chirho.value_declaration_chirho(declaration_chirho)
                                })?;
                            }
                            Ok(())
                        },
                    )?;
                }
                ExprChirho::QuoteTypeChirho {
                    ty_chirho,
                    span_chirho,
                } => {
                    this_chirho.require_quotation_syntax_chirho(*span_chirho)?;
                    this_chirho.state_chirho.check_retained_syntax_chirho(
                        ty_chirho,
                        128usize.saturating_sub(this_chirho.type_depth_chirho),
                    )?;
                }
                ExprChirho::QuotePatChirho {
                    pat_chirho,
                    span_chirho,
                } => {
                    this_chirho.require_quotation_syntax_chirho(*span_chirho)?;
                    this_chirho.at_quote_stage_chirho(
                        this_chirho.quote_stage_chirho + 1,
                        |this_chirho| {
                            this_chirho
                                .scoped_chirho(|this_chirho| this_chirho.pattern_chirho(pat_chirho))
                        },
                    )?;
                }
                ExprChirho::VarChirho(_) | ExprChirho::ConChirho(_) | ExprChirho::LitChirho(_) => {}
            }
            Ok(())
        })
    }
}
