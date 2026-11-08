// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor value applications consume the same checked telescope as
//! promotion, while inferred kind binders stay invisible to written @ arguments.

use super::*;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

pub(crate) struct ArgumentElaborationContextChirho<'scope_chirho> {
    pub state_chirho: &'scope_chirho mut ElaborationStateChirho,
    pub registry_chirho: &'scope_chirho mut TypeFamilyEnvChirho,
    pub type_scope_chirho: &'scope_chirho TypeConstructorScopeChirho,
    pub family_scope_chirho: &'scope_chirho TypeFamilyScopeChirho,
    pub promotion_scope_chirho: &'scope_chirho PromotionScopeChirho,
    pub scoped_variables_chirho: &'scope_chirho HashMap<String, TyVarChirho>,
    pub images_chirho: &'scope_chirho crate::subst_chirho::SubstChirho,
    pub equality_chirho: crate::infer_chirho::EqualityNormalizationViewChirho<'scope_chirho>,
    pub refinements_chirho: &'scope_chirho crate::env_chirho::TyEnvChirho,
    pub refinement_mark_chirho: Option<crate::env_chirho::RefinementMarkChirho>,
}

/// Resume the original checked solver after ordinary inference recognizes a
/// required binder. Ownership, catalogue and resource allowance are not reset.
pub(crate) fn elaborate_required_argument_chirho(
    expression_chirho: &ExprChirho,
    expected_kind_chirho: &TyChirho,
    context_chirho: ArgumentElaborationContextChirho<'_>,
) -> Result<(TyChirho, crate::subst_chirho::SubstChirho), ElaborationErrorChirho> {
    let span_chirho = expression_chirho.span_chirho();
    with_argument_elaborator_chirho(context_chirho, span_chirho, true, |elaborator_chirho| {
        let mut syntax_chirho =
            elaborator_chirho.required_argument_syntax_chirho(expression_chirho, 0)?;
        let expected_kind_chirho =
            elaborator_chirho.normalize_lexical_term_chirho(expected_kind_chirho, span_chirho)?;
        let typed_chirho = elaborator_chirho
            .infer_type_at_kind_chirho(&mut syntax_chirho, &expected_kind_chirho)?;
        elaborator_chirho
            .state_chirho
            .resolve_chirho(&typed_chirho.term_chirho, span_chirho)
    })
}

pub(crate) struct CheckedBodyTypeChirho {
    pub syntax_chirho: TypeChirho,
    pub term_chirho: TyChirho,
    pub kind_chirho: TyChirho,
}

/// Fixedness is checked at value-parameter entry, before that parameter list
/// introduces GADT evidence. A nested lambda resumes under its outer evidence.
pub(crate) fn elaborate_value_parameters_chirho(
    parameters_chirho: &[(TyChirho, SpanChirho)],
    context_chirho: ArgumentElaborationContextChirho<'_>,
) -> Result<((), crate::subst_chirho::SubstChirho), ElaborationErrorChirho> {
    let span_chirho = parameters_chirho
        .first()
        .map_or(SpanChirho::DUMMY_CHIRHO, |(_, span_chirho)| *span_chirho);
    with_argument_elaborator_chirho(context_chirho, span_chirho, false, |elaborator_chirho| {
        for (parameter_chirho, span_chirho) in parameters_chirho {
            let Some(kind_chirho) =
                elaborator_chirho.checked_term_kind_chirho(parameter_chirho, *span_chirho)?
            else {
                continue;
            };
            let kind_chirho =
                elaborator_chirho.normalize_value_kind_term_chirho(&kind_chirho, *span_chirho)?;
            if elaborator_chirho.state_chirho.value_representation_chirho(
                &kind_chirho,
                &elaborator_chirho.registry_chirho,
                *span_chirho,
            )? == ValueRepresentationChirho::NONFIXED_CHIRHO
            {
                return Err(TypeElaboratorChirho::error_chirho(
                    "value parameter does not have a fixed runtime representation",
                    *span_chirho,
                ));
            }
        }
        Ok(())
    })
}

/// Written contexts extend a retained parameter entry using their actual
/// opened terms. Live pattern rewrites must not normalize these premises.
pub(crate) fn normalize_value_parameter_equalities_chirho(
    equalities_chirho: &[(&TyChirho, &TyChirho)],
    span_chirho: SpanChirho,
    context_chirho: ArgumentElaborationContextChirho<'_>,
) -> Result<(Vec<(TyChirho, TyChirho)>, crate::subst_chirho::SubstChirho), ElaborationErrorChirho> {
    with_argument_elaborator_chirho(context_chirho, span_chirho, false, |elaborator_chirho| {
        elaborator_chirho.state_chirho.charge_chirho(
            equalities_chirho.len(),
            equalities_chirho.len(),
            span_chirho,
        )?;
        equalities_chirho
            .iter()
            .map(|(left_chirho, right_chirho)| {
                Ok((
                    elaborator_chirho.normalize_value_kind_term_chirho(left_chirho, span_chirho)?,
                    elaborator_chirho
                        .normalize_value_kind_term_chirho(right_chirho, span_chirho)?,
                ))
            })
            .collect()
    })
}

/// Recheck actual body syntax after its pattern/signature givens are in scope.
/// The returned syntax retains the same ownership and full hidden applications.
pub(crate) fn elaborate_body_type_chirho(
    syntax_chirho: &TypeChirho,
    value_kind_chirho: bool,
    context_chirho: ArgumentElaborationContextChirho<'_>,
) -> Result<(CheckedBodyTypeChirho, crate::subst_chirho::SubstChirho), ElaborationErrorChirho> {
    let span_chirho = syntax_chirho.span_chirho();
    with_argument_elaborator_chirho(context_chirho, span_chirho, false, |elaborator_chirho| {
        let mut syntax_chirho = elaborator_chirho
            .state_chirho
            .copy_syntax_chirho(syntax_chirho)?;
        let typed_chirho = if value_kind_chirho {
            elaborator_chirho.infer_value_type_chirho(&mut syntax_chirho)?
        } else {
            elaborator_chirho.infer_type_chirho(&mut syntax_chirho)?
        };
        Ok(CheckedBodyTypeChirho {
            syntax_chirho,
            term_chirho: elaborator_chirho
                .state_chirho
                .resolve_chirho(&typed_chirho.term_chirho, span_chirho)?,
            kind_chirho: elaborator_chirho
                .state_chirho
                .resolve_chirho(&typed_chirho.kind_chirho, span_chirho)?,
        })
    })
}

/// Constructor pattern wildcards are checked at the selected producer kind;
/// an arbitrary type wildcard has no authority to acquire a universal kind.
pub(crate) fn elaborate_body_pattern_type_chirho(
    syntax_chirho: &TypeChirho,
    expected_kind_chirho: &TyChirho,
    context_chirho: ArgumentElaborationContextChirho<'_>,
) -> Result<(CheckedBodyTypeChirho, crate::subst_chirho::SubstChirho), ElaborationErrorChirho> {
    let span_chirho = syntax_chirho.span_chirho();
    with_argument_elaborator_chirho(context_chirho, span_chirho, false, |elaborator_chirho| {
        let mut syntax_chirho = elaborator_chirho
            .state_chirho
            .copy_syntax_chirho(syntax_chirho)?;
        let term_chirho = elaborator_chirho
            .bind_pattern_type_argument_chirho(&mut syntax_chirho, expected_kind_chirho)?;
        Ok(CheckedBodyTypeChirho {
            syntax_chirho,
            term_chirho,
            kind_chirho: elaborator_chirho
                .state_chirho
                .resolve_chirho(expected_kind_chirho, span_chirho)?,
        })
    })
}

fn with_argument_elaborator_chirho<ResultChirho>(
    context_chirho: ArgumentElaborationContextChirho<'_>,
    span_chirho: SpanChirho,
    strict_argument_chirho: bool,
    check_chirho: impl FnOnce(
        &mut TypeElaboratorChirho<'_>,
    ) -> Result<ResultChirho, ElaborationErrorChirho>,
) -> Result<(ResultChirho, crate::subst_chirho::SubstChirho), ElaborationErrorChirho> {
    context_chirho
        .state_chirho
        .charge_chirho(0, 0, span_chirho)?;
    if context_chirho
        .refinement_mark_chirho
        .is_some_and(|mark_chirho| {
            !context_chirho
                .refinements_chirho
                .refinement_mark_is_live_chirho(mark_chirho)
        })
    {
        return Err(TypeElaboratorChirho::error_chirho(
            "value parameter evidence outlived its lexical scope",
            span_chirho,
        ));
    }
    context_chirho.state_chirho.owner_id_chirho(span_chirho)?;
    let policy_chirho = context_chirho
        .state_chirho
        .source_policy_chirho
        .clone()
        .ok_or_else(|| {
            TypeElaboratorChirho::error_chirho(
                "required argument lost its producer options",
                span_chirho,
            )
        })?;
    context_chirho
        .state_chirho
        .begin_inference_links_chirho(span_chirho)?;
    let state_chirho = std::mem::replace(
        context_chirho.state_chirho,
        ElaborationStateChirho::unowned_chirho(),
    );
    let mut elaborator_chirho = TypeElaboratorChirho {
        normalization_module_chirho: None,
        standalone_kind_scope_chirho: false,
        checking_kind_chirho: false,
        state_chirho,
        registry_chirho: ElaborationCatalogueChirho::BorrowedChirho(context_chirho.registry_chirho),
        type_scope_chirho: context_chirho.type_scope_chirho,
        family_scope_chirho: context_chirho.family_scope_chirho,
        promotion_scope_chirho: context_chirho.promotion_scope_chirho,
        builtin_classes_chirho: &policy_chirho.builtin_classes_chirho,
        star_is_type_chirho: policy_chirho.star_is_type_chirho,
        poly_kinds_chirho: policy_chirho.poly_kinds_chirho,
        cusks_chirho: policy_chirho.cusks_chirho,
        unlifted_newtypes_chirho: policy_chirho.unlifted_newtypes_chirho,
        unlifted_datatypes_chirho: policy_chirho.unlifted_datatypes_chirho,
        scoped_type_variables_chirho: policy_chirho.scoped_type_variables_chirho,
        template_haskell_quotes_chirho: policy_chirho.template_haskell_quotes_chirho,
        transform_list_comp_chirho: policy_chirho.transform_list_comp_chirho,
        quote_stage_chirho: 0,
        signature_variables_chirho: HashMap::new(),
        signature_undo_chirho: Vec::new(),
        type_depth_chirho: 0,
        rechecking_chirho: false,
        variables_chirho: HashMap::new(),
        scope_undo_chirho: Vec::new(),
        declaration_variables_chirho: HashMap::new(),
        current_declaration_chirho: None,
        declaration_contracts_chirho: HashMap::new(),
        lexical_images_chirho: Some(context_chirho.images_chirho),
        lexical_refinements_chirho: Some(context_chirho.refinements_chirho),
        lexical_refinement_mark_chirho: context_chirho.refinement_mark_chirho,
        lexical_equality_chirho: Some(context_chirho.equality_chirho),
        strict_argument_chirho,
    };
    let result_chirho = (|| {
        for (name_chirho, variable_chirho) in context_chirho.scoped_variables_chirho {
            let record_chirho = elaborator_chirho
                .state_chirho
                .variable_kinds_chirho
                .get(variable_chirho);
            let kind_chirho = context_chirho
                .images_chirho
                .lookup_image_chirho(variable_chirho)
                .and_then(|image_chirho| image_chirho.kind_chirho.as_ref())
                .or_else(|| {
                    record_chirho.and_then(|record_chirho| record_chirho.kind_chirho.as_ref())
                })
                .cloned()
                .ok_or_else(|| {
                    TypeElaboratorChirho::error_chirho(
                        "lexical required argument has no checked kind",
                        span_chirho,
                    )
                })?;
            let name_chirho = elaborator_chirho
                .state_chirho
                .name_chirho(name_chirho, span_chirho)?;
            let kind_chirho = elaborator_chirho
                .state_chirho
                .copy_chirho(&kind_chirho, span_chirho)?;
            elaborator_chirho.bind_variable_chirho(
                name_chirho,
                *variable_chirho,
                kind_chirho,
                span_chirho,
            )?;
        }
        check_chirho(&mut elaborator_chirho)
    })();
    let links_chirho = elaborator_chirho
        .state_chirho
        .finish_inference_links_chirho(span_chirho);
    *context_chirho.state_chirho = elaborator_chirho.state_chirho;
    result_chirho
        .and_then(|term_chirho| links_chirho.map(|links_chirho| (term_chirho, links_chirho)))
}

impl TypeElaboratorChirho<'_> {
    pub(super) fn unify_kinds_chirho(
        &mut self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let left_chirho = self.normalize_lexical_term_chirho(left_chirho, span_chirho)?;
        let right_chirho = self.normalize_lexical_term_chirho(right_chirho, span_chirho)?;
        let identity_chirho = crate::subst_chirho::SubstChirho::empty_chirho();
        let legacy_synonyms_chirho = HashMap::new();
        let view_chirho = match self.lexical_equality_chirho {
            Some(view_chirho) => view_chirho,
            None => crate::infer_chirho::EqualityNormalizationViewChirho {
                substitutions_chirho: &identity_chirho,
                legacy_synonyms_chirho: &legacy_synonyms_chirho,
                type_scope_chirho: Some(self.type_scope_chirho),
                family_scope_chirho: Some(self.family_scope_chirho),
                family_module_chirho: self.normalization_module_chirho.as_deref().ok_or_else(
                    || {
                        Self::error_chirho(
                            "kind equality lost its source module context",
                            span_chirho,
                        )
                    },
                )?,
                promotion_present_chirho: true,
                evidence_chirho: Default::default(),
            },
        };
        let (left_chirho, right_chirho) = self.state_chirho.normalize_equality_pair_chirho(
            view_chirho,
            &self.registry_chirho,
            &left_chirho,
            &right_chirho,
            span_chirho,
        )?;
        self.state_chirho
            .unify_chirho(&left_chirho, &right_chirho, span_chirho)
    }

    pub(super) fn normalize_lexical_term_chirho(
        &mut self,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let Some(images_chirho) = self.lexical_images_chirho else {
            return self.state_chirho.copy_chirho(term_chirho, span_chirho);
        };
        let resolved_chirho = self.state_chirho.resolve_chirho(term_chirho, span_chirho)?;
        let mut term_chirho = images_chirho
            .apply_bounded_chirho(&resolved_chirho, &mut |work_chirho, output_chirho| {
                self.state_chirho
                    .charge_chirho(work_chirho, output_chirho, span_chirho)
                    .ok()
            })
            .ok_or_else(|| {
                self.state_chirho.resource_failure_chirho(
                    "required argument kind substitution bound exhausted",
                    span_chirho,
                )
            })?;
        let Some(refinements_chirho) = self
            .lexical_refinements_chirho
            .filter(|env_chirho| env_chirho.has_refinements_chirho())
        else {
            return Ok(term_chirho);
        };
        for _ in 0..128 {
            let next_chirho = crate::skolem_chirho::rewrite_borrowed_skolems_bounded_chirho(
                &term_chirho,
                &mut |name_chirho| {
                    refinements_chirho.lookup_refinement_at_mark_chirho(
                        name_chirho,
                        self.lexical_refinement_mark_chirho,
                    )
                },
                &mut |work_chirho, output_chirho| {
                    self.state_chirho
                        .charge_chirho(work_chirho, output_chirho, span_chirho)
                        .ok()
                },
            )
            .map_err(|_| {
                self.state_chirho.resource_failure_chirho(
                    "required argument refinement bound exhausted",
                    span_chirho,
                )
            })?;
            if next_chirho == term_chirho {
                return Ok(next_chirho);
            }
            term_chirho = next_chirho;
        }
        Err(self.state_chirho.resource_failure_chirho(
            "required argument refinement iterations exhausted",
            span_chirho,
        ))
    }

    /// Keep term-namespace resolution while projecting the syntactically
    /// ambiguous required argument into the common type/kind vocabulary.
    fn required_argument_syntax_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        depth_chirho: usize,
    ) -> Result<TypeChirho, ElaborationErrorChirho> {
        use haskelujah_ast_chirho::lit_chirho::LitChirho;
        use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
        let span_chirho = expression_chirho.span_chirho();
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho >= 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "required argument syntax depth bound exhausted",
                span_chirho,
            ));
        }
        Ok(match expression_chirho {
            ExprChirho::VarChirho(name_chirho)
                if name_chirho.text_chirho() == "_" && name_chirho.qualifier_chirho().is_none() =>
            {
                TypeChirho::WildcardChirho { span_chirho }
            }
            ExprChirho::VarChirho(name_chirho) => {
                let variable_chirho = if name_chirho.type_variable_chirho().is_some() {
                    self.check_variable_owner_chirho(name_chirho)?
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "required argument has no owned type variable",
                                span_chirho,
                            )
                        })?
                } else {
                    if name_chirho.qualifier_chirho().is_some() {
                        return Err(Self::error_chirho(
                            "qualified term variable is not a lexical type binder",
                            span_chirho,
                        ));
                    }
                    self.variables_chirho
                        .get(name_chirho.text_chirho())
                        .map(|record_chirho| record_chirho.variable_chirho)
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "term variable cannot be used as a required type argument",
                                span_chirho,
                            )
                        })?
                };
                if self
                    .variables_chirho
                    .get(name_chirho.text_chirho())
                    .map(|record_chirho| record_chirho.variable_chirho)
                    != Some(variable_chirho)
                {
                    return Err(Self::error_chirho(
                        "required argument type variable is outside its lexical scope",
                        span_chirho,
                    ));
                }
                let mut name_chirho = self.state_chirho.copy_name_chirho(name_chirho)?;
                self.stamp_variable_chirho(&mut name_chirho, variable_chirho)?;
                TypeChirho::VarChirho(name_chirho)
            }
            ExprChirho::ConChirho(name_chirho) => {
                let mut name_chirho = self.state_chirho.copy_name_chirho(name_chirho)?;
                let resolution_chirho = name_chirho
                    .promotion_chirho()
                    .cloned()
                    .unwrap_or_else(|| self.promotion_scope_chirho.quoted_chirho(&name_chirho));
                if !haskelujah_ast_chirho::promotion_chirho::is_unknown_promoted_key_chirho(
                    resolution_chirho.key_chirho().as_str_chirho(),
                ) {
                    name_chirho.set_promotion_chirho(resolution_chirho);
                    TypeChirho::PromotedConChirho {
                        name_chirho,
                        span_chirho,
                    }
                } else {
                    TypeChirho::ConChirho(name_chirho)
                }
            }
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => TypeChirho::AppChirho {
                fun_chirho: Box::new(
                    self.required_argument_syntax_chirho(fun_chirho, depth_chirho + 1)?,
                ),
                arg_chirho: Box::new(
                    self.required_argument_syntax_chirho(arg_chirho, depth_chirho + 1)?,
                ),
                span_chirho,
            },
            ExprChirho::ParenChirho { inner_chirho, .. } => TypeChirho::ParenChirho {
                inner_chirho: Box::new(
                    self.required_argument_syntax_chirho(inner_chirho, depth_chirho + 1)?,
                ),
                span_chirho,
            },
            ExprChirho::AnnChirho {
                expr_chirho,
                ty_chirho,
                ..
            } => TypeChirho::KindAnnotationChirho {
                ty_chirho: Box::new(
                    self.required_argument_syntax_chirho(expr_chirho, depth_chirho + 1)?,
                ),
                kind_chirho: Box::new(self.state_chirho.copy_syntax_chirho(ty_chirho)?),
                span_chirho,
            },
            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                self.state_chirho.charge_chirho(
                    elements_chirho.len(),
                    elements_chirho.len(),
                    span_chirho,
                )?;
                TypeChirho::PromotedListChirho {
                    elements_chirho: elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.required_argument_syntax_chirho(element_chirho, depth_chirho + 1)
                        })
                        .collect::<Result<_, _>>()?,
                    span_chirho,
                }
            }
            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                if elements_chirho.len() == 1 || elements_chirho.len() > 64 {
                    return Err(Self::error_chirho(
                        "unsupported required tuple arity",
                        span_chirho,
                    ));
                }
                self.state_chirho.charge_chirho(
                    elements_chirho.len().saturating_mul(4).saturating_add(2),
                    elements_chirho.len().saturating_mul(4).saturating_add(2),
                    span_chirho,
                )?;
                let text_chirho =
                    format!("({})", ",".repeat(elements_chirho.len().saturating_sub(1)));
                let mut head_chirho = TypeChirho::PromotedConChirho {
                    name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                        text_chirho,
                        span_chirho,
                    )),
                    span_chirho,
                };
                for element_chirho in elements_chirho {
                    head_chirho = TypeChirho::AppChirho {
                        fun_chirho: Box::new(head_chirho),
                        arg_chirho: Box::new(
                            self.required_argument_syntax_chirho(element_chirho, depth_chirho + 1)?,
                        ),
                        span_chirho,
                    };
                }
                head_chirho
            }
            ExprChirho::LitChirho(literal_chirho) => {
                let value_chirho = match literal_chirho {
                    LitChirho::IntChirho(value_chirho, ..) => {
                        self.state_chirho.charge_chirho(32, 32, span_chirho)?;
                        value_chirho.to_string()
                    }
                    LitChirho::StringChirho(value_chirho, ..) => {
                        self.state_chirho.charge_chirho(
                            value_chirho.len().saturating_add(2),
                            value_chirho.len().saturating_add(2),
                            span_chirho,
                        )?;
                        format!("\"{value_chirho}\"")
                    }
                    LitChirho::CharChirho(value_chirho, ..) => {
                        self.state_chirho.charge_chirho(6, 6, span_chirho)?;
                        format!("'{value_chirho}'")
                    }
                    _ => {
                        return Err(Self::error_chirho(
                            "unsupported required type literal",
                            span_chirho,
                        ));
                    }
                };
                TypeChirho::LitChirho {
                    value_chirho,
                    span_chirho,
                }
            }
            _ => {
                return Err(Self::error_chirho(
                    "expression is not a required type argument",
                    span_chirho,
                ));
            }
        })
    }

    pub(super) fn elaborate_constructor_type_application_chirho(
        &mut self,
        expression_chirho: &mut ExprChirho,
    ) -> Result<bool, ElaborationErrorChirho> {
        let span_chirho = expression_chirho.span_chirho();
        let mut base_chirho = expression_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            match base_chirho {
                ExprChirho::TypeAppChirho {
                    expr_chirho,
                    ty_chirho,
                    ..
                } => {
                    arguments_chirho.push(ty_chirho);
                    base_chirho = expr_chirho;
                }
                ExprChirho::ParenChirho { inner_chirho, .. } => base_chirho = inner_chirho,
                _ => break,
            }
        }
        let ExprChirho::ConChirho(name_chirho) = base_chirho else {
            return Ok(false);
        };
        let Some(key_chirho) = self.constructor_key_chirho(name_chirho)? else {
            return Ok(false);
        };
        let Some(scheme_chirho) = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&key_chirho)
            .filter(|scheme_chirho| {
                scheme_chirho.promoted_chirho.is_some() && !scheme_chirho.held_chirho
            })
            .cloned()
        else {
            return Ok(false);
        };
        if scheme_chirho.binders_chirho.iter().any(|binder_chirho| {
            binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
                && binder_chirho.term_specificity_chirho.is_none()
        }) {
            return Ok(false);
        }
        let (binders_chirho, _, _) = self.instantiate_header_chirho(&scheme_chirho, span_chirho)?;
        let mut cursor_chirho = 0;
        for argument_chirho in arguments_chirho.into_iter().rev() {
            while binders_chirho
                .get(cursor_chirho)
                .is_some_and(|binder_chirho| {
                    binder_chirho.term_specificity_chirho == Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho)
                })
            {
                cursor_chirho += 1;
            }
            let Some(binder_chirho) = binders_chirho.get(cursor_chirho).filter(|binder_chirho| {
                binder_chirho.term_specificity_chirho
                    == Some(
                        haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho,
                    )
            }) else {
                return Err(Self::error_chirho(
                    "no specified constructor type binder remains for written type application",
                    argument_chirho.span_chirho(),
                ));
            };
            let typed_chirho = self.infer_type_chirho(argument_chirho)?;
            self.unify_kinds_chirho(
                &typed_chirho.kind_chirho,
                &binder_chirho.kind_chirho,
                argument_chirho.span_chirho(),
            )?;
            self.unify_kinds_chirho(
                &TyChirho::VarChirho(binder_chirho.variable_chirho),
                &typed_chirho.term_chirho,
                argument_chirho.span_chirho(),
            )?;
            // This is the already checked kind, not another inference pass in
            // the term converter. It shares this module's owner and supply.
            let argument_span_chirho = argument_chirho.span_chirho();
            let kind_chirho =
                self.surface_chirho(&typed_chirho.kind_chirho, argument_span_chirho)?;
            let original_chirho = std::mem::replace(
                argument_chirho,
                TypeChirho::WildcardChirho {
                    span_chirho: argument_span_chirho,
                },
            );
            *argument_chirho = TypeChirho::KindAnnotationChirho {
                ty_chirho: Box::new(original_chirho),
                kind_chirho: Box::new(kind_chirho),
                span_chirho: argument_span_chirho,
            };
            cursor_chirho += 1;
        }
        Ok(true)
    }

    /// Pattern type binders use the actual constructor's checked telescope.
    /// No term field is consumed and no runtime dictionary route is activated.
    pub(super) fn elaborate_constructor_pattern_types_chirho(
        &mut self,
        constructor_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        arguments_chirho: &mut [haskelujah_ast_chirho::pat_chirho::ConstructorTypePatternChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        if arguments_chirho.is_empty() {
            return Ok(());
        }
        let span_chirho = constructor_chirho.span_chirho();
        let scheme_chirho =
            if let Some(key_chirho) = self.constructor_key_chirho(constructor_chirho)? {
                self.registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&key_chirho)
                    .filter(|scheme_chirho| {
                        scheme_chirho.promoted_chirho.is_some() && !scheme_chirho.held_chirho
                    })
                    .cloned()
            } else {
                None
            };
        let binders_chirho = if let Some(scheme_chirho) = &scheme_chirho {
            Some(
                self.instantiate_header_chirho(scheme_chirho, span_chirho)?
                    .0,
            )
        } else {
            None
        };
        let mut cursor_chirho = 0;
        for pattern_chirho in arguments_chirho {
            let argument_chirho = &mut pattern_chirho.ty_chirho;
            let expected_chirho = if let (Some(scheme_chirho), Some(binders_chirho)) =
                (&scheme_chirho, &binders_chirho)
            {
                let mut sources_chirho = scheme_chirho
                    .source_variables_chirho
                    .iter()
                    .collect::<Vec<_>>();
                self.state_chirho.charge_chirho(
                    sources_chirho.len(),
                    sources_chirho.len(),
                    span_chirho,
                )?;
                sources_chirho.sort_by_key(|source_chirho| source_chirho.header_index_chirho);
                while sources_chirho.get(cursor_chirho).is_some_and(|source_chirho| source_chirho.specificity_chirho == haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho) { cursor_chirho += 1; }
                if sources_chirho.is_empty() {
                    None
                } else {
                    let source_chirho = sources_chirho.get(cursor_chirho).ok_or_else(|| {
                        Self::error_chirho(
                            "no source constructor type binder remains",
                            argument_chirho.span_chirho(),
                        )
                    })?;
                    let position_chirho = scheme_chirho
                        .binders_chirho
                        .iter()
                        .position(|binder_chirho| {
                            binder_chirho.variable_chirho == source_chirho.variable_chirho
                        })
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "checked constructor source binder has no telescope slot",
                                span_chirho,
                            )
                        })?;
                    // instantiate_header preserves this exact typed telescope's
                    // order. Source-written order comes solely from its mapping.
                    pattern_chirho.checked_binder_chirho =
                        Some(source_chirho.source_chirho.clone());
                    Some(&binders_chirho[position_chirho])
                }
            } else {
                None
            };
            let kind_chirho = if let Some(binder_chirho) = expected_chirho {
                self.state_chirho
                    .copy_chirho(&binder_chirho.kind_chirho, span_chirho)?
            } else {
                self.fresh_kind_chirho(argument_chirho.span_chirho())?
            };
            let argument_term_chirho =
                self.bind_pattern_type_argument_chirho(argument_chirho, &kind_chirho)?;
            if let Some(binder_chirho) = expected_chirho {
                self.unify_kinds_chirho(
                    &TyChirho::VarChirho(binder_chirho.variable_chirho),
                    &argument_term_chirho,
                    argument_chirho.span_chirho(),
                )?;
            }
            cursor_chirho += 1;
        }
        Ok(())
    }

    fn bind_pattern_type_argument_chirho(
        &mut self,
        argument_chirho: &mut TypeChirho,
        expected_chirho: &TyChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let span_chirho = argument_chirho.span_chirho();
        self.with_syntax_depth_chirho(span_chirho, |this_chirho| match argument_chirho {
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                this_chirho.bind_pattern_type_argument_chirho(inner_chirho, expected_chirho)
            }
            TypeChirho::KindAnnotationChirho {
                ty_chirho,
                kind_chirho,
                ..
            } => {
                let annotation_chirho = this_chirho
                    .scoped_chirho(|this_chirho| this_chirho.infer_type_chirho(kind_chirho))?;
                this_chirho.unify_kinds_chirho(
                    &annotation_chirho.kind_chirho,
                    &ElaborationStateChirho::type_universe_chirho(),
                    span_chirho,
                )?;
                this_chirho.unify_kinds_chirho(
                    &annotation_chirho.term_chirho,
                    expected_chirho,
                    span_chirho,
                )?;
                this_chirho.bind_pattern_type_argument_chirho(ty_chirho, expected_chirho)
            }
            TypeChirho::VarChirho(name_chirho) => {
                let variable_chirho = if let Some(variable_chirho) =
                    this_chirho.check_variable_owner_chirho(name_chirho)?
                {
                    let kind_chirho = this_chirho
                        .state_chirho
                        .variable_kind_chirho(variable_chirho, span_chirho)?;
                    this_chirho.unify_kinds_chirho(&kind_chirho, expected_chirho, span_chirho)?;
                    variable_chirho
                } else {
                    let kind_chirho = this_chirho
                        .state_chirho
                        .copy_chirho(expected_chirho, span_chirho)?;
                    this_chirho
                        .state_chirho
                        .fresh_chirho(Some(kind_chirho), span_chirho)?
                };
                this_chirho
                    .state_chirho
                    .mark_universal_chirho(variable_chirho, span_chirho)?;
                this_chirho.stamp_variable_chirho(name_chirho, variable_chirho)?;
                let name_chirho = this_chirho
                    .state_chirho
                    .name_chirho(name_chirho.text_chirho(), span_chirho)?;
                let kind_chirho = this_chirho
                    .state_chirho
                    .copy_chirho(expected_chirho, span_chirho)?;
                this_chirho.bind_variable_chirho(
                    name_chirho,
                    variable_chirho,
                    kind_chirho,
                    span_chirho,
                )?;
                Ok(TyChirho::VarChirho(variable_chirho))
            }
            TypeChirho::WildcardChirho { .. } => {
                let kind_chirho = this_chirho
                    .state_chirho
                    .copy_chirho(expected_chirho, span_chirho)?;
                Ok(TyChirho::VarChirho(
                    this_chirho
                        .state_chirho
                        .fresh_chirho(Some(kind_chirho), span_chirho)?,
                ))
            }
            _ => {
                this_chirho.seed_type_pattern_scope_chirho(argument_chirho)?;
                this_chirho
                    .infer_type_at_kind_chirho(argument_chirho, expected_chirho)
                    .map(|typed_chirho| typed_chirho.term_chirho)
            }
        })
    }
}
