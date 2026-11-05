// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Free variables in a source type pattern are new lexical binders. Their
//! producer-owned IDs survive the ordinary constructor instantiation handoff.

use super::*;
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;

impl TypeElaboratorChirho<'_> {
    pub(in crate::kind_chirho::elaboration_chirho) fn seed_type_pattern_scope_chirho(
        &mut self,
        pattern_chirho: &mut TypeChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.seed_type_pattern_term_chirho(pattern_chirho, &mut HashMap::new(), &mut Vec::new())
    }

    fn seed_type_pattern_term_chirho(
        &mut self,
        term_chirho: &mut TypeChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
        bound_chirho: &mut Vec<String>,
    ) -> Result<(), ElaborationErrorChirho> {
        let span_chirho = term_chirho.span_chirho();
        self.with_syntax_depth_chirho(span_chirho, |this_chirho| {
            this_chirho.state_chirho.charge_chirho(1, 0, span_chirho)?;
            match term_chirho {
                TypeChirho::VarChirho(name_chirho) => {
                    this_chirho
                        .state_chirho
                        .charge_chirho(bound_chirho.len(), 0, span_chirho)?;
                    if !bound_chirho
                        .iter()
                        .any(|text_chirho| text_chirho == name_chirho.text_chirho())
                    {
                        let variable_chirho = if let Some(variable_chirho) =
                            variables_chirho.get(name_chirho.text_chirho())
                        {
                            *variable_chirho
                        } else if let Some(images_chirho) = this_chirho.lexical_images_chirho {
                            // The ordinary pattern handoff already prepared every
                            // free binder. Reuse its checked lexical image, rather
                            // than allocating a second owner for structured syntax.
                            let variable_chirho = this_chirho
                                .check_variable_owner_chirho(name_chirho)?
                                .ok_or_else(|| {
                                    Self::error_chirho(
                                        "late type pattern has no checked binder",
                                        span_chirho,
                                    )
                                })?;
                            images_chirho
                                .lookup_image_chirho(&variable_chirho)
                                .filter(|image_chirho| {
                                    image_chirho.kind_chirho.is_some()
                                        && image_chirho.source_chirho.as_ref()
                                            == name_chirho.type_variable_chirho()
                                })
                                .ok_or_else(|| {
                                    Self::error_chirho(
                                        "late type pattern lost its checked lexical image",
                                        span_chirho,
                                    )
                                })?;
                            let text_chirho = this_chirho
                                .state_chirho
                                .name_chirho(name_chirho.text_chirho(), span_chirho)?;
                            variables_chirho.insert(text_chirho, variable_chirho);
                            variable_chirho
                        } else {
                            let kind_chirho = this_chirho.fresh_kind_chirho(span_chirho)?;
                            let variable_chirho = this_chirho
                                .state_chirho
                                .fresh_chirho(Some(kind_chirho), span_chirho)?;
                            let text_chirho = this_chirho
                                .state_chirho
                                .name_chirho(name_chirho.text_chirho(), span_chirho)?;
                            variables_chirho.insert(text_chirho.clone(), variable_chirho);
                            let kind_chirho = this_chirho
                                .state_chirho
                                .variable_kind_chirho(variable_chirho, span_chirho)?;
                            this_chirho.bind_variable_chirho(
                                text_chirho,
                                variable_chirho,
                                kind_chirho,
                                span_chirho,
                            )?;
                            variable_chirho
                        };
                        this_chirho.stamp_variable_chirho(name_chirho, variable_chirho)?;
                    }
                }
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                }
                | TypeChirho::KindApplicationChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    this_chirho.seed_type_pattern_term_chirho(
                        fun_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                    this_chirho.seed_type_pattern_term_chirho(
                        arg_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                }
                TypeChirho::KindAnnotationChirho {
                    ty_chirho,
                    kind_chirho,
                    ..
                } => {
                    this_chirho.seed_type_pattern_term_chirho(
                        kind_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                    this_chirho.seed_type_pattern_term_chirho(
                        ty_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                }
                TypeChirho::FunChirho {
                    arg_chirho,
                    result_chirho,
                    ..
                } => {
                    this_chirho.seed_type_pattern_term_chirho(
                        arg_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                    this_chirho.seed_type_pattern_term_chirho(
                        result_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                }
                TypeChirho::TupleChirho {
                    elements_chirho, ..
                }
                | TypeChirho::PromotedListChirho {
                    elements_chirho, ..
                } => {
                    for element_chirho in elements_chirho {
                        this_chirho.seed_type_pattern_term_chirho(
                            element_chirho,
                            variables_chirho,
                            bound_chirho,
                        )?;
                    }
                }
                TypeChirho::ListChirho { element_chirho, .. } => this_chirho
                    .seed_type_pattern_term_chirho(
                        element_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?,
                TypeChirho::ParenChirho { inner_chirho, .. } => this_chirho
                    .seed_type_pattern_term_chirho(inner_chirho, variables_chirho, bound_chirho)?,
                TypeChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                    ..
                }
                | TypeChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                    ..
                } => {
                    let mark_chirho = bound_chirho.len();
                    for binder_chirho in vars_chirho {
                        if let Some(kind_chirho) = &mut binder_chirho.kind_annotation_chirho {
                            this_chirho.seed_type_pattern_term_chirho(
                                kind_chirho,
                                variables_chirho,
                                bound_chirho,
                            )?;
                        }
                        bound_chirho.push(
                            this_chirho
                                .state_chirho
                                .name_chirho(binder_chirho.text_chirho(), span_chirho)?,
                        );
                    }
                    let result_chirho = this_chirho.seed_type_pattern_term_chirho(
                        body_chirho,
                        variables_chirho,
                        bound_chirho,
                    );
                    bound_chirho.truncate(mark_chirho);
                    result_chirho?;
                }
                TypeChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                    ..
                } => {
                    for constraint_chirho in context_chirho {
                        this_chirho.seed_type_pattern_constraint_chirho(
                            constraint_chirho,
                            variables_chirho,
                            bound_chirho,
                        )?;
                    }
                    this_chirho.seed_type_pattern_term_chirho(
                        body_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                }
                TypeChirho::ConChirho(_)
                | TypeChirho::PromotedConChirho { .. }
                | TypeChirho::WildcardChirho { .. }
                | TypeChirho::LitChirho { .. } => {}
            }
            Ok(())
        })
    }

    fn seed_type_pattern_constraint_chirho(
        &mut self,
        constraint_chirho: &mut ConstraintChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
        bound_chirho: &mut Vec<String>,
    ) -> Result<(), ElaborationErrorChirho> {
        let span_chirho = constraint_chirho.span_chirho();
        self.with_syntax_depth_chirho(span_chirho, |this_chirho| {
            this_chirho.state_chirho.charge_chirho(1, 0, span_chirho)?;
            match constraint_chirho {
                ConstraintChirho::ClassChirho {
                    args_chirho,
                    kind_args_chirho,
                    ..
                } => {
                    for term_chirho in kind_args_chirho.iter_mut().chain(args_chirho) {
                        this_chirho.seed_type_pattern_term_chirho(
                            term_chirho,
                            variables_chirho,
                            bound_chirho,
                        )?;
                    }
                }
                ConstraintChirho::KindAnnotationChirho {
                    constraint_chirho,
                    kind_chirho,
                    ..
                } => {
                    this_chirho.seed_type_pattern_constraint_chirho(
                        constraint_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                    this_chirho.seed_type_pattern_term_chirho(
                        kind_chirho,
                        variables_chirho,
                        bound_chirho,
                    )?;
                }
                ConstraintChirho::QuantifiedChirho {
                    vars_chirho,
                    context_chirho,
                    body_chirho,
                    ..
                } => {
                    let mark_chirho = bound_chirho.len();
                    for binder_chirho in vars_chirho {
                        if let Some(kind_chirho) = &mut binder_chirho.kind_annotation_chirho {
                            this_chirho.seed_type_pattern_term_chirho(
                                kind_chirho,
                                variables_chirho,
                                bound_chirho,
                            )?;
                        }
                        bound_chirho.push(
                            this_chirho
                                .state_chirho
                                .name_chirho(binder_chirho.text_chirho(), span_chirho)?,
                        );
                    }
                    for constraint_chirho in context_chirho {
                        this_chirho.seed_type_pattern_constraint_chirho(
                            constraint_chirho,
                            variables_chirho,
                            bound_chirho,
                        )?;
                    }
                    let result_chirho = this_chirho.seed_type_pattern_constraint_chirho(
                        body_chirho,
                        variables_chirho,
                        bound_chirho,
                    );
                    bound_chirho.truncate(mark_chirho);
                    result_chirho?;
                }
            }
            Ok(())
        })
    }
}
