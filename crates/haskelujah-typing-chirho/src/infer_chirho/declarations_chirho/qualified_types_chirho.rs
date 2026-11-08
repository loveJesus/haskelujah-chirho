// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked qualified types expose constraints to the existing wanted/given
//! machinery. The declaration catalogue owns every complete class telescope.

use super::InferCtxChirho;
use super::equality_normalization_chirho::EqualityBudgetChirho;
use crate::ty_chirho::declarations_chirho::TelescopeVisibilityChirho;
use crate::ty_chirho::{SchemePredChirho, SchemePredPartsChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashMap;

impl InferCtxChirho {
    pub(super) fn qualified_type_failure_chirho(
        &mut self,
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) {
        self.non_deferrable_diagnostics_chirho.push_chirho(
            haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                message_chirho,
                span_chirho,
            ),
        );
    }

    pub(super) fn ast_constraint_to_term_chirho(
        &mut self,
        syntax_chirho: &ConstraintChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyChirho {
        match syntax_chirho {
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho, ..
            } => self.ast_constraint_to_term_chirho(constraint_chirho, variables_chirho),
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                let (vars_chirho, body_chirho) = self.with_ast_forall_bindings_chirho(
                    vars_chirho,
                    variables_chirho,
                    |context_self_chirho, variables_chirho| TyChirho::QualChirho {
                        context_chirho: context_chirho
                            .iter()
                            .map(|constraint_chirho| {
                                context_self_chirho.ast_constraint_to_term_chirho(
                                    constraint_chirho,
                                    variables_chirho,
                                )
                            })
                            .collect(),
                        body_chirho: Box::new(
                            context_self_chirho
                                .ast_constraint_to_term_chirho(body_chirho, variables_chirho),
                        ),
                    },
                );
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }
            ConstraintChirho::ClassChirho {
                class_chirho,
                kind_args_chirho,
                args_chirho,
                span_chirho,
            } => {
                let key_chirho = class_chirho
                    .family_key_chirho()
                    .map(str::to_owned)
                    .or_else(|| {
                        class_chirho
                            .type_constructor_chirho()
                            .or_else(|| {
                                self.type_constructor_scope_chirho.as_ref().and_then(
                                    |scope_chirho| {
                                        scope_chirho
                                            .declaration_chirho(&class_chirho.full_name_chirho())
                                    },
                                )
                            })
                            .map(|declaration_chirho| declaration_chirho.type_key_chirho())
                    })
                    .or_else(|| self.naming_selected_builtin_class_key_chirho(class_chirho))
                    .unwrap_or_else(|| class_chirho.full_name_chirho());
                let hidden_chirho = kind_args_chirho
                    .iter()
                    .map(|argument_chirho| {
                        self.ast_type_to_ty_chirho(argument_chirho, variables_chirho)
                    })
                    .collect::<Vec<_>>();
                let visible_chirho = args_chirho
                    .iter()
                    .map(|argument_chirho| {
                        self.ast_type_to_ty_chirho(argument_chirho, variables_chirho)
                    })
                    .collect::<Vec<_>>();
                let schema_chirho = self
                    .type_families_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&key_chirho)
                    .cloned();
                let mut term_chirho = TyChirho::ConChirho(key_chirho);
                if let Some(schema_chirho) = schema_chirho {
                    if schema_chirho.owner_chirho.is_some()
                        || schema_chirho.held_chirho
                        || hidden_chirho.len() != schema_chirho.hidden_arity_chirho()
                        || visible_chirho.len() != schema_chirho.visible_arity_chirho
                    {
                        self.qualified_type_failure_chirho(
                            "constraint conversion lost its checked telescope",
                            *span_chirho,
                        );
                        return term_chirho;
                    }
                    let mut hidden_chirho = hidden_chirho.into_iter();
                    let mut visible_chirho = visible_chirho.into_iter();
                    for binder_chirho in &schema_chirho.binders_chirho {
                        let argument_chirho = if binder_chirho.visibility_chirho
                            == TelescopeVisibilityChirho::RequiredChirho
                        {
                            visible_chirho.next()
                        } else {
                            hidden_chirho.next()
                        }
                        .expect("checked complete constraint arity");
                        term_chirho =
                            TyChirho::AppChirho(Box::new(term_chirho), Box::new(argument_chirho));
                    }
                } else {
                    for argument_chirho in hidden_chirho.into_iter().chain(visible_chirho) {
                        term_chirho =
                            TyChirho::AppChirho(Box::new(term_chirho), Box::new(argument_chirho));
                    }
                }
                term_chirho
            }
        }
    }

    /// Row 692, C10 Q (gpt #33150): a QUALIFIED written class (`P.Num`, `Prelude.Ord`) names a
    /// builtin class only through the naming-selected scope this module's elaboration adopted, the
    /// scope kind elaboration resolved the same spelling through. The key is the selected compiler
    /// identity's own declaration key, whose header kind elaboration seeded for every selected class,
    /// so the predicate decodes to that compiler class with its proof. An unqualified spelling, a
    /// spelling naming did not select and a context without that scope keep the spelling fallback;
    /// no prefix, written name or scheme equality grants the identity.
    /// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
    fn naming_selected_builtin_class_key_chirho(
        &self,
        _class_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> Option<String> {
        // MUTANT M1 (gpt #33157), never landed: R1 is disabled, so every class key falls back to
        // its spelling as at 81df0c74.
        None
    }

    pub(super) fn constraint_head_proof_chirho(
        &self,
        key_chirho: &str,
    ) -> haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho {
        use haskelujah_ast_chirho::name_chirho::{
            ClassIdentityUnknownChirho, CompilerTypeConstructorChirho,
            PredicateClassIdentityChirho, PredicateClassProofChirho, TypeConstructorIdentityChirho,
            TypeConstructorRoleChirho,
        };
        let Some(scheme_chirho) = self
            .type_families_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
            .filter(|scheme_chirho| {
                scheme_chirho.owner_chirho.is_none()
                    && !scheme_chirho.held_chirho
                    && scheme_chirho.role_chirho == TypeConstructorRoleChirho::ClassChirho
            })
        else {
            return PredicateClassProofChirho::UnknownChirho(
                ClassIdentityUnknownChirho::LegacyChirho,
            );
        };
        match scheme_chirho
            .declaration_chirho
            .as_ref()
            .map(|declaration_chirho| &declaration_chirho.identity_chirho)
        {
            Some(TypeConstructorIdentityChirho::CompilerChirho(
                CompilerTypeConstructorChirho::ClassChirho(identity_chirho),
            )) => PredicateClassProofChirho::ProvenChirho(
                PredicateClassIdentityChirho::CompilerChirho(*identity_chirho),
            ),
            Some(TypeConstructorIdentityChirho::SourceChirho { .. }) => {
                PredicateClassProofChirho::UnknownChirho(
                    ClassIdentityUnknownChirho::MissingUnitChirho,
                )
            }
            _ => PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
        }
    }

    pub(super) fn scheme_preds_from_constraint_ty_chirho(
        &mut self,
        term_chirho: &TyChirho,
        predicates_chirho: &mut Vec<SchemePredChirho>,
        equalities_chirho: &mut Vec<(TyChirho, TyChirho)>,
    ) {
        let mut budget_chirho = EqualityBudgetChirho::new_chirho();
        if self
            .decode_constraint_terms_chirho(
                term_chirho,
                predicates_chirho,
                equalities_chirho,
                &mut budget_chirho,
            )
            .is_none()
        {
            self.qualified_type_failure_chirho(
                "constraint transport is unbound, unsupported or exceeds its resource bound",
                SpanChirho::DUMMY_CHIRHO,
            );
        }
    }

    fn decode_constraint_terms_chirho(
        &mut self,
        term_chirho: &TyChirho,
        predicates_chirho: &mut Vec<SchemePredChirho>,
        equalities_chirho: &mut Vec<(TyChirho, TyChirho)>,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<()> {
        let mut pending_chirho = vec![(term_chirho, 0usize)];
        while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
            budget_chirho.work_chirho(1)?;
            if depth_chirho > 128 {
                return None;
            }
            if let TyChirho::TupleChirho(elements_chirho) = term_chirho {
                budget_chirho.work_chirho(elements_chirho.len())?;
                pending_chirho.extend(
                    elements_chirho
                        .iter()
                        .rev()
                        .map(|element_chirho| (element_chirho, depth_chirho + 1)),
                );
                continue;
            }
            let mut head_chirho = term_chirho;
            let mut arguments_chirho = Vec::new();
            while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                budget_chirho.node_chirho(1)?;
                if depth_chirho + arguments_chirho.len() >= 128 {
                    return None;
                }
                arguments_chirho.push(argument_chirho.as_ref());
                head_chirho = function_chirho;
            }
            arguments_chirho.reverse();
            let TyChirho::ConChirho(key_chirho) = head_chirho else {
                return None;
            };
            let schema_chirho = self
                .type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(key_chirho)
                .cloned();
            let equality_chirho = schema_chirho.as_ref().and_then(|schema_chirho| schema_chirho.declaration_chirho.as_ref()).is_some_and(|declaration_chirho| matches!(declaration_chirho.identity_chirho, haskelujah_ast_chirho::name_chirho::TypeConstructorIdentityChirho::CompilerChirho(haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho::EqualityChirho(_))));
            if let Some(schema_chirho) = &schema_chirho {
                if schema_chirho.owner_chirho.is_some() || schema_chirho.held_chirho || (!equality_chirho && schema_chirho.role_chirho != haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::ClassChirho) || schema_chirho.reduction_arity_chirho != arguments_chirho.len() { return None; }
            } else if key_chirho.starts_with('%') {
                return None;
            }
            let mut hidden_chirho = Vec::new();
            let mut visible_chirho = Vec::new();
            for (index_chirho, argument_chirho) in arguments_chirho.into_iter().enumerate() {
                let copied_chirho =
                    super::equality_normalization_chirho::copy_term_with_budget_chirho(
                        argument_chirho,
                        budget_chirho,
                    )?;
                if schema_chirho.as_ref().is_some_and(|schema_chirho| {
                    schema_chirho.binders_chirho[index_chirho].visibility_chirho
                        != TelescopeVisibilityChirho::RequiredChirho
                }) {
                    hidden_chirho.push(copied_chirho);
                } else {
                    visible_chirho.push(copied_chirho);
                }
            }
            if (equality_chirho || matches!(key_chirho.as_str(), "~" | "~~" | "∼"))
                && visible_chirho.len() == 2
            {
                let mut arguments_chirho = visible_chirho.into_iter();
                equalities_chirho.push((arguments_chirho.next()?, arguments_chirho.next()?));
                continue;
            }
            use haskelujah_ast_chirho::name_chirho::{
                CompilerTypeConstructorChirho, TypeConstructorIdentityChirho,
            };
            let class_name_chirho = match schema_chirho
                .as_ref()
                .and_then(|schema_chirho| schema_chirho.declaration_chirho.as_ref())
                .map(|declaration_chirho| &declaration_chirho.identity_chirho)
            {
                Some(TypeConstructorIdentityChirho::CompilerChirho(
                    CompilerTypeConstructorChirho::ClassChirho(identity_chirho),
                )) => identity_chirho.legacy_name_chirho().to_owned(),
                Some(TypeConstructorIdentityChirho::SourceChirho { name_chirho, .. }) => {
                    name_chirho.clone()
                }
                _ => key_chirho.clone(),
            };
            let proof_chirho = self.constraint_head_proof_chirho(key_chirho);
            let mut visible_chirho = visible_chirho.into_iter();
            predicates_chirho.push(SchemePredChirho::from_proof_chirho(
                proof_chirho,
                SchemePredPartsChirho {
                    kind_tys_chirho: hidden_chirho,
                    class_name_chirho,
                    ty_chirho: visible_chirho.next().unwrap_or_else(TyChirho::unit_chirho),
                    extra_tys_chirho: visible_chirho.collect(),
                },
            ));
        }
        Some(())
    }

    pub(super) fn split_typed_scheme_prefix_chirho(
        &mut self,
        mut term_chirho: TyChirho,
        predicates_chirho: &mut Vec<SchemePredChirho>,
        equalities_chirho: &mut Vec<(TyChirho, TyChirho)>,
    ) -> (crate::ty_chirho::QuantifiedVariablesChirho, TyChirho) {
        let mut variables_chirho = crate::ty_chirho::QuantifiedVariablesChirho::default();
        let mut budget_chirho = EqualityBudgetChirho::new_chirho();
        loop {
            match term_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    variables_chirho = variables_chirho.concat_chirho(&vars_chirho);
                    term_chirho = *body_chirho;
                }
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    for constraint_chirho in &context_chirho {
                        if self
                            .decode_constraint_terms_chirho(
                                constraint_chirho,
                                predicates_chirho,
                                equalities_chirho,
                                &mut budget_chirho,
                            )
                            .is_none()
                        {
                            self.qualified_type_failure_chirho(
                                "qualified type exceeded its checked constraint transport boundary",
                                SpanChirho::DUMMY_CHIRHO,
                            );
                        }
                    }
                    term_chirho = *body_chirho;
                }
                _ => return (variables_chirho, term_chirho),
            }
        }
    }
    pub(super) fn expose_qualified_parts_chirho(
        &mut self,
        mut term_chirho: TyChirho,
        mut predicates_chirho: Vec<crate::class_chirho::PredChirho>,
    ) -> (TyChirho, Vec<crate::class_chirho::PredChirho>) {
        let mut budget_chirho = EqualityBudgetChirho::new_chirho();
        while let TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } = term_chirho
        {
            let mut schemes_chirho = Vec::new();
            let mut equalities_chirho = Vec::new();
            for constraint_chirho in &context_chirho {
                if self
                    .decode_constraint_terms_chirho(
                        constraint_chirho,
                        &mut schemes_chirho,
                        &mut equalities_chirho,
                        &mut budget_chirho,
                    )
                    .is_none()
                {
                    self.qualified_type_failure_chirho("qualified predicate transport exceeded its checked scope or resource bound", SpanChirho::DUMMY_CHIRHO);
                }
            }
            predicates_chirho.extend(schemes_chirho.into_iter().map(|predicate_chirho| {
                crate::class_chirho::PredChirho::from_proof_chirho(
                    predicate_chirho.class_identity_chirho().clone(),
                    crate::class_chirho::PredPartsChirho {
                        class_name_chirho: predicate_chirho.class_name_chirho,
                        kind_tys_chirho: predicate_chirho.kind_tys_chirho,
                        ty_chirho: predicate_chirho.ty_chirho,
                        extra_tys_chirho: predicate_chirho.extra_tys_chirho,
                    },
                )
            }));
            predicates_chirho.extend(equalities_chirho.into_iter().map(
                |(left_chirho, right_chirho)| {
                    crate::class_chirho::PredChirho::legacy_chirho(
                        crate::class_chirho::PredPartsChirho {
                            class_name_chirho: "~".to_owned(),
                            kind_tys_chirho: Vec::new(),
                            ty_chirho: left_chirho,
                            extra_tys_chirho: vec![right_chirho],
                        },
                    )
                },
            ));
            term_chirho = *body_chirho;
        }
        (term_chirho, predicates_chirho)
    }

    pub(super) fn expose_qualified_wanteds_chirho(
        &mut self,
        term_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> TyChirho {
        let (term_chirho, predicates_chirho) =
            self.expose_qualified_parts_chirho(term_chirho, Vec::new());
        for predicate_chirho in predicates_chirho {
            self.defer_pred_chirho(predicate_chirho, span_chirho);
        }
        term_chirho
    }
    /// Open quantifiers only along the expected FUNCTION spine. A forall in a
    /// parameter stays a polymorphic parameter and freshens at each occurrence.
    pub(super) fn open_expected_function_spine_chirho(
        &mut self,
        expected_chirho: &TyChirho,
        arity_chirho: usize,
    ) -> Option<(
        Vec<TyChirho>,
        TyChirho,
        Vec<crate::class_chirho::PredChirho>,
    )> {
        let mut budget_chirho = EqualityBudgetChirho::new_chirho();
        let mut current_chirho =
            super::equality_normalization_chirho::copy_term_with_budget_chirho(
                expected_chirho,
                &mut budget_chirho,
            )?;
        let mut parameters_chirho = Vec::new();
        let mut givens_chirho = Vec::new();
        while parameters_chirho.len() < arity_chirho {
            budget_chirho.work_chirho(1)?;
            match current_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    let (opened_chirho, predicates_chirho) =
                        self.skolemize_scheme_parts_chirho(&crate::ty_chirho::SchemeChirho {
                            vars_chirho,
                            preds_chirho: Vec::new(),
                            ty_chirho: *body_chirho,
                        });
                    current_chirho = opened_chirho;
                    givens_chirho.extend(predicates_chirho);
                }
                TyChirho::QualChirho { .. } => {
                    let (opened_chirho, predicates_chirho) =
                        self.expose_qualified_parts_chirho(current_chirho, Vec::new());
                    current_chirho = opened_chirho;
                    givens_chirho.extend(predicates_chirho);
                }
                TyChirho::FunChirho(parameter_chirho, result_chirho, _) => {
                    parameters_chirho.push(*parameter_chirho);
                    current_chirho = *result_chirho;
                }
                _ => return None,
            }
        }
        Some((parameters_chirho, current_chirho, givens_chirho))
    }
}
