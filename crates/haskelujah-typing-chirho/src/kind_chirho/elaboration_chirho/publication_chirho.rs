// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One bounded publication maps checked owner variables into canonical slots in
//! both the typed RHS and its retained syntax. The live module keeps its owner.

use super::*;
use crate::ty_chirho::declaration_syntax_chirho::PublishedSynonymSyntaxChirho;
use crate::ty_chirho::declarations_chirho::DeclarationLocalBinderChirho;
use haskelujah_ast_chirho::decl_chirho::{TyVarChirho as AstTyVarChirho, TyVarSpecificityChirho};
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use std::collections::{HashMap, HashSet};

pub(super) struct CanonicalHeaderPublicationChirho {
    pub(super) binders_chirho: Vec<DeclarationBinderChirho>,
    pub(super) result_kind_chirho: TyChirho,
    pub(super) local_binders_chirho: Arc<[DeclarationLocalBinderChirho]>,
    pub(super) mapping_chirho: crate::subst_chirho::SubstChirho,
}

struct SynonymPublicationChirho {
    identity_chirho: Arc<str>,
    mapping_chirho: HashMap<TyVarChirho, TyChirho>,
    top_chirho: HashSet<TyVarChirho>,
    bound_chirho: Vec<TyVarChirho>,
    syntax_introductions_chirho: HashSet<TyVarChirho>,
    local_binders_chirho: Vec<DeclarationLocalBinderChirho>,
    next_slot_chirho: u32,
}

impl TypeElaboratorChirho<'_> {
    fn publication_variable_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        variable_chirho: TyVarChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if let Some(image_chirho) = publication_chirho.mapping_chirho.get(&variable_chirho) {
            if !publication_chirho.top_chirho.contains(&variable_chirho)
                && !publication_chirho.bound_chirho.contains(&variable_chirho)
            {
                return Err(Self::error_chirho(
                    "published synonym variable escaped its lexical quantifier",
                    span_chirho,
                ));
            }
            return self.state_chirho.copy_chirho(image_chirho, span_chirho);
        }
        let resolved_chirho = self
            .state_chirho
            .resolve_chirho(&TyChirho::VarChirho(variable_chirho), span_chirho)?;
        if resolved_chirho == TyChirho::VarChirho(variable_chirho) {
            return Err(Self::error_chirho(
                "published synonym contains an unbound owner variable",
                span_chirho,
            ));
        }
        self.publish_typed_term_chirho(publication_chirho, &resolved_chirho, span_chirho, 0)
    }

    fn allocate_publication_binders_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        variables_chirho: &[TyVarChirho],
        visibility_chirho: TelescopeVisibilityChirho,
        specificity_chirho: &[TyVarSpecificityChirho],
        syntax_chirho: bool,
        span_chirho: SpanChirho,
    ) -> Result<crate::ty_chirho::QuantifiedVariablesChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(
            variables_chirho.len(),
            variables_chirho.len(),
            span_chirho,
        )?;
        if variables_chirho.len() != specificity_chirho.len() {
            return Err(Self::error_chirho(
                "published quantifier lost its binder specificity",
                span_chirho,
            ));
        }
        let mark_chirho = publication_chirho.bound_chirho.len();
        let result_chirho = (|| {
            let mut slots_chirho = Vec::with_capacity(variables_chirho.len());
            for (variable_chirho, specificity_chirho) in
                variables_chirho.iter().zip(specificity_chirho)
            {
                if publication_chirho.top_chirho.contains(variable_chirho)
                    || publication_chirho.bound_chirho.contains(variable_chirho)
                    || (syntax_chirho
                        && !publication_chirho
                            .syntax_introductions_chirho
                            .insert(*variable_chirho))
                {
                    return Err(Self::error_chirho(
                        "published synonym introduces one variable more than once",
                        span_chirho,
                    ));
                }
                let slot_chirho = if let Some(image_chirho) =
                    publication_chirho.mapping_chirho.get(variable_chirho)
                {
                    let TyChirho::VarChirho(slot_chirho) = image_chirho else {
                        return Err(Self::error_chirho(
                            "published universal has no canonical variable slot",
                            span_chirho,
                        ));
                    };
                    *slot_chirho
                } else {
                    let slot_chirho = TyVarChirho(publication_chirho.next_slot_chirho);
                    publication_chirho.next_slot_chirho = publication_chirho
                        .next_slot_chirho
                        .checked_add(1)
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "published synonym slot arithmetic exhausted",
                                span_chirho,
                            )
                        })?;
                    publication_chirho
                        .mapping_chirho
                        .insert(*variable_chirho, TyChirho::VarChirho(slot_chirho));
                    slot_chirho
                };
                let visibility_chirho =
                    if visibility_chirho == TelescopeVisibilityChirho::RequiredChirho {
                        visibility_chirho
                    } else if *specificity_chirho == TyVarSpecificityChirho::InferredChirho {
                        TelescopeVisibilityChirho::InferredChirho
                    } else {
                        TelescopeVisibilityChirho::SpecifiedChirho
                    };
                // Each binder kind sees preceding quantifiers only. Allocate
                // its identity first, but never admit self/forward captures.
                let kind_chirho = self
                    .state_chirho
                    .variable_kind_chirho(*variable_chirho, span_chirho)?;
                let kind_chirho = self
                    .state_chirho
                    .resolve_chirho(&kind_chirho, span_chirho)?;
                let kind_chirho = self.publish_typed_term_chirho(
                    publication_chirho,
                    &kind_chirho,
                    span_chirho,
                    0,
                )?;
                let recorded_kind_chirho =
                    self.state_chirho.copy_chirho(&kind_chirho, span_chirho)?;
                if let Some(previous_chirho) =
                    publication_chirho
                        .local_binders_chirho
                        .iter()
                        .find(|record_chirho| {
                            record_chirho.binder_chirho.variable_chirho == slot_chirho
                        })
                {
                    let kinds_agree_chirho = crate::ty_chirho::same_type_contract_chirho(
                        &previous_chirho.binder_chirho.kind_chirho,
                        &kind_chirho,
                        &mut |weight_chirho| {
                            self.state_chirho
                                .charge_chirho(weight_chirho, 0, span_chirho)
                                .ok()
                        },
                    )
                    .ok_or_else(|| {
                        Self::error_chirho(
                            "kind contract comparison resource bound exhausted",
                            span_chirho,
                        )
                    })?;
                    if !kinds_agree_chirho
                        || previous_chirho.binder_chirho.visibility_chirho != visibility_chirho
                        || previous_chirho.specificity_chirho != *specificity_chirho
                    {
                        return Err(Self::error_chirho(
                            "syntax and typed universal binder semantics disagree",
                            span_chirho,
                        ));
                    }
                } else {
                    publication_chirho
                        .local_binders_chirho
                        .push(DeclarationLocalBinderChirho {
                            binder_chirho: DeclarationBinderChirho {
                                variable_chirho: slot_chirho,
                                kind_chirho,
                                visibility_chirho,
                            },
                            specificity_chirho: *specificity_chirho,
                        });
                }
                publication_chirho.bound_chirho.push(*variable_chirho);
                slots_chirho.push(crate::ty_chirho::QuantifiedVariableChirho {
                    variable_chirho: slot_chirho,
                    kind_chirho: Some(Arc::new(recorded_kind_chirho)),
                    source_chirho: Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
                        declaration_chirho: Arc::clone(&publication_chirho.identity_chirho),
                        index_chirho: slot_chirho.0,
                    }),
                    specificity_chirho: Some(*specificity_chirho),
                });
            }
            Ok(crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(slots_chirho))
        })();
        if result_chirho.is_err() {
            publication_chirho.bound_chirho.truncate(mark_chirho);
        }
        result_chirho
    }

    fn publish_typed_term_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(Self::error_chirho(
                "typed synonym publication depth bound exhausted",
                span_chirho,
            ));
        }
        Ok(match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                self.publication_variable_chirho(publication_chirho, *variable_chirho, span_chirho)?
            }
            TyChirho::ConChirho(_) => self.state_chirho.copy_chirho(term_chirho, span_chirho)?,
            TyChirho::ForallVarChirho(_) => {
                return Err(Self::error_chirho(
                    "rigid solver marker cannot become a published synonym binder",
                    span_chirho,
                ));
            }
            TyChirho::AppChirho(function_chirho, argument_chirho) => TyChirho::AppChirho(
                Box::new(self.publish_typed_term_chirho(
                    publication_chirho,
                    function_chirho,
                    span_chirho,
                    depth_chirho + 1,
                )?),
                Box::new(self.publish_typed_term_chirho(
                    publication_chirho,
                    argument_chirho,
                    span_chirho,
                    depth_chirho + 1,
                )?),
            ),
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                TyChirho::FunChirho(
                    Box::new(self.publish_typed_term_chirho(
                        publication_chirho,
                        argument_chirho,
                        span_chirho,
                        depth_chirho + 1,
                    )?),
                    Box::new(self.publish_typed_term_chirho(
                        publication_chirho,
                        result_chirho,
                        span_chirho,
                        depth_chirho + 1,
                    )?),
                    *multiplicity_chirho,
                )
            }
            TyChirho::ListChirho(element_chirho) => {
                TyChirho::ListChirho(Box::new(self.publish_typed_term_chirho(
                    publication_chirho,
                    element_chirho,
                    span_chirho,
                    depth_chirho + 1,
                )?))
            }
            TyChirho::TupleChirho(elements_chirho) => {
                self.state_chirho.charge_chirho(
                    elements_chirho.len(),
                    elements_chirho.len(),
                    span_chirho,
                )?;
                TyChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.publish_typed_term_chirho(
                                publication_chirho,
                                element_chirho,
                                span_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                )
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                self.state_chirho.charge_chirho(
                    context_chirho.len(),
                    context_chirho.len(),
                    span_chirho,
                )?;
                TyChirho::QualChirho {
                    context_chirho: context_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.publish_typed_term_chirho(
                                publication_chirho,
                                constraint_chirho,
                                span_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    body_chirho: Box::new(self.publish_typed_term_chirho(
                        publication_chirho,
                        body_chirho,
                        span_chirho,
                        depth_chirho + 1,
                    )?),
                }
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let mark_chirho = publication_chirho.bound_chirho.len();
                let required_chirho = matches!(term_chirho, TyChirho::RequiredForallChirho { .. });
                let specificity_chirho = vars_chirho
                    .iter()
                    .map(|variable_chirho| {
                        self.state_chirho
                            .source_specificity_chirho(*variable_chirho)
                            .ok_or_else(|| {
                                Self::error_chirho(
                                    "published universal lost its producer-owned specificity",
                                    span_chirho,
                                )
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let variables_chirho = self.allocate_publication_binders_chirho(
                    publication_chirho,
                    vars_chirho,
                    if required_chirho {
                        TelescopeVisibilityChirho::RequiredChirho
                    } else {
                        TelescopeVisibilityChirho::SpecifiedChirho
                    },
                    &specificity_chirho,
                    false,
                    span_chirho,
                )?;
                let body_chirho = self.publish_typed_term_chirho(
                    publication_chirho,
                    body_chirho,
                    span_chirho,
                    depth_chirho + 1,
                );
                publication_chirho.bound_chirho.truncate(mark_chirho);
                let body_chirho = Box::new(body_chirho?);
                if required_chirho {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho,
                    }
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho,
                    }
                }
            }
        })
    }
}

impl TypeElaboratorChirho<'_> {
    pub(super) fn publish_synonym_body_chirho(
        &mut self,
        identity_chirho: &Arc<str>,
        parameters_chirho: &[AstTyVarChirho],
        syntax_chirho: &TypeChirho,
        typed_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(TyChirho, Arc<PublishedSynonymSyntaxChirho>), ElaborationErrorChirho> {
        let schema_chirho = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(identity_chirho)
            .cloned()
            .ok_or_else(|| {
                Self::error_chirho(
                    "synonym publication has no checked declaration",
                    span_chirho,
                )
            })?;
        let count_chirho = schema_chirho.binders_chirho.len();
        if schema_chirho.owner_chirho.is_some()
            || schema_chirho.binders_chirho.iter().enumerate().any(
                |(index_chirho, binder_chirho)| {
                    binder_chirho.variable_chirho.0 as usize != index_chirho
                },
            )
        {
            return Err(Self::error_chirho(
                "synonym publication requires its canonical checked telescope",
                span_chirho,
            ));
        }
        let mut publication_chirho = SynonymPublicationChirho {
            identity_chirho: identity_chirho.clone(),
            mapping_chirho: HashMap::new(),
            top_chirho: HashSet::new(),
            bound_chirho: Vec::new(),
            syntax_introductions_chirho: HashSet::new(),
            local_binders_chirho: Vec::new(),
            next_slot_chirho: u32::try_from(count_chirho).map_err(|_| {
                Self::error_chirho("synonym publication slot arithmetic exhausted", span_chirho)
            })?,
        };
        for (source_chirho, image_chirho) in self.declaration_variables_chirho[identity_chirho]
            .publication_chirho
            .iter_chirho()
        {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            publication_chirho.mapping_chirho.insert(
                *source_chirho,
                self.state_chirho.copy_chirho(image_chirho, span_chirho)?,
            );
            if matches!(image_chirho, TyChirho::VarChirho(slot_chirho) if (slot_chirho.0 as usize) < count_chirho)
            {
                publication_chirho.top_chirho.insert(*source_chirho);
            }
        }
        // A second body check starts from the header's local binders only;
        // previous RHS quantifiers are fresh lexical producers on replay.
        let header_slots_chirho = publication_chirho
            .mapping_chirho
            .values()
            .filter_map(|image_chirho| {
                if let TyChirho::VarChirho(variable_chirho) = image_chirho {
                    Some(*variable_chirho)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        for record_chirho in schema_chirho
            .local_binders_chirho
            .iter()
            .filter(|record_chirho| {
                header_slots_chirho.contains(&record_chirho.binder_chirho.variable_chirho)
            })
        {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            publication_chirho
                .local_binders_chirho
                .push(DeclarationLocalBinderChirho {
                    binder_chirho: DeclarationBinderChirho {
                        variable_chirho: record_chirho.binder_chirho.variable_chirho,
                        kind_chirho: self
                            .state_chirho
                            .copy_chirho(&record_chirho.binder_chirho.kind_chirho, span_chirho)?,
                        visibility_chirho: record_chirho.binder_chirho.visibility_chirho,
                    },
                    specificity_chirho: record_chirho.specificity_chirho,
                });
        }
        for binder_chirho in &publication_chirho.local_binders_chirho {
            publication_chirho.next_slot_chirho = publication_chirho.next_slot_chirho.max(
                binder_chirho
                    .binder_chirho
                    .variable_chirho
                    .0
                    .checked_add(1)
                    .ok_or_else(|| {
                        Self::error_chirho(
                            "published local binder slot arithmetic exhausted",
                            span_chirho,
                        )
                    })?,
            );
        }
        self.state_chirho.charge_chirho(
            parameters_chirho.len(),
            parameters_chirho.len(),
            span_chirho,
        )?;
        let mut published_parameters_chirho = Vec::with_capacity(parameters_chirho.len());
        for parameter_chirho in parameters_chirho {
            published_parameters_chirho.push(self.publish_syntax_binder_chirho(
                &mut publication_chirho,
                parameter_chirho,
                0,
            )?);
        }
        let rhs_chirho =
            self.publish_syntax_type_chirho(&mut publication_chirho, syntax_chirho, 0)?;
        let typed_chirho =
            self.publish_typed_term_chirho(&mut publication_chirho, typed_chirho, span_chirho, 0)?;
        publication_chirho
            .local_binders_chirho
            .sort_by_key(|binder_chirho| binder_chirho.binder_chirho.variable_chirho);
        let syntax_chirho = Arc::new(PublishedSynonymSyntaxChirho::new_chirho(
            published_parameters_chirho,
            rhs_chirho,
            Arc::from(publication_chirho.local_binders_chirho),
        ));
        Ok((typed_chirho, syntax_chirho))
    }

    fn publication_source_variable_chirho(
        &self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> Result<TyVarChirho, ElaborationErrorChirho> {
        self.check_variable_owner_chirho(name_chirho)?
            .ok_or_else(|| {
                Self::error_chirho(
                    "checked synonym syntax lost its variable producer",
                    name_chirho.span_chirho(),
                )
            })
    }
    fn publication_canonical_slots_chirho(
        &mut self,
        publication_chirho: &SynonymPublicationChirho,
        span_chirho: SpanChirho,
    ) -> Result<HashSet<TyVarChirho>, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(
            publication_chirho.mapping_chirho.len(),
            publication_chirho.mapping_chirho.len(),
            span_chirho,
        )?;
        Ok(publication_chirho
            .mapping_chirho
            .iter()
            .filter_map(|(source_chirho, image_chirho)| {
                if (publication_chirho.top_chirho.contains(source_chirho)
                    || publication_chirho.bound_chirho.contains(source_chirho))
                    && let TyChirho::VarChirho(slot_chirho) = image_chirho
                {
                    Some(*slot_chirho)
                } else {
                    None
                }
            })
            .collect())
    }
    fn publish_syntax_variable_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> Result<TypeChirho, ElaborationErrorChirho> {
        let span_chirho = name_chirho.span_chirho();
        let variable_chirho = self.publication_source_variable_chirho(name_chirho)?;
        let image_chirho =
            self.publication_variable_chirho(publication_chirho, variable_chirho, span_chirho)?;
        if let TyChirho::VarChirho(variable_chirho) = image_chirho {
            let mut name_chirho = self.copy_name_chirho(name_chirho)?;
            name_chirho.set_type_variable_chirho(Some(
                ScopedTypeVariableIdChirho::QuantifiedChirho {
                    declaration_chirho: publication_chirho.identity_chirho.clone(),
                    index_chirho: variable_chirho.0,
                },
            ));
            Ok(TypeChirho::VarChirho(name_chirho))
        } else {
            let allowed_chirho =
                self.publication_canonical_slots_chirho(publication_chirho, span_chirho)?;
            self.canonical_surface_chirho(
                &image_chirho,
                &publication_chirho.identity_chirho,
                &allowed_chirho,
                span_chirho,
                0,
            )
        }
    }
    fn publish_syntax_binder_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        binder_chirho: &AstTyVarChirho,
        depth_chirho: usize,
    ) -> Result<AstTyVarChirho, ElaborationErrorChirho> {
        let TypeChirho::VarChirho(name_chirho) =
            self.publish_syntax_variable_chirho(publication_chirho, &binder_chirho.name_chirho)?
        else {
            return Err(Self::error_chirho(
                "published syntax binder was specialized instead of quantified",
                binder_chirho.name_chirho.span_chirho(),
            ));
        };
        Ok(AstTyVarChirho {
            name_chirho,
            kind_annotation_chirho: binder_chirho
                .kind_annotation_chirho
                .as_ref()
                .map(|kind_chirho| {
                    self.publish_syntax_type_chirho(
                        publication_chirho,
                        kind_chirho,
                        depth_chirho + 1,
                    )
                    .map(Box::new)
                })
                .transpose()?,
            visibility_chirho: binder_chirho.visibility_chirho,
            specificity_chirho: binder_chirho.specificity_chirho,
        })
    }
    fn publish_syntax_quantifiers_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        binders_chirho: &[AstTyVarChirho],
        required_chirho: bool,
        depth_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Result<Vec<AstTyVarChirho>, ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(binders_chirho.len(), binders_chirho.len(), span_chirho)?;
        let variables_chirho = binders_chirho
            .iter()
            .map(|binder_chirho| {
                self.publication_source_variable_chirho(&binder_chirho.name_chirho)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let specificity_chirho = binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.specificity_chirho)
            .collect::<Vec<_>>();
        self.allocate_publication_binders_chirho(
            publication_chirho,
            &variables_chirho,
            if required_chirho {
                TelescopeVisibilityChirho::RequiredChirho
            } else {
                TelescopeVisibilityChirho::SpecifiedChirho
            },
            &specificity_chirho,
            true,
            span_chirho,
        )?;
        binders_chirho
            .iter()
            .map(|binder_chirho| {
                self.publish_syntax_binder_chirho(
                    publication_chirho,
                    binder_chirho,
                    depth_chirho + 1,
                )
            })
            .collect()
    }
    fn publish_syntax_constraint_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        constraint_chirho: &ConstraintChirho,
        depth_chirho: usize,
    ) -> Result<ConstraintChirho, ElaborationErrorChirho> {
        let span_chirho = constraint_chirho.span_chirho();
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(Self::error_chirho(
                "constraint syntax publication depth bound exhausted",
                span_chirho,
            ));
        }
        Ok(match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                kind_args_chirho,
                ..
            } => {
                self.state_chirho.charge_chirho(
                    args_chirho.len() + kind_args_chirho.len(),
                    args_chirho.len() + kind_args_chirho.len(),
                    span_chirho,
                )?;
                ConstraintChirho::ClassChirho {
                    class_chirho: if class_chirho.type_variable_chirho().is_some() {
                        match self
                            .publish_syntax_variable_chirho(publication_chirho, class_chirho)?
                        {
                            TypeChirho::VarChirho(name_chirho)
                            | TypeChirho::ConChirho(name_chirho) => name_chirho,
                            _ => {
                                return Err(Self::error_chirho(
                                    "constraint head solution cannot be represented by its checked syntax",
                                    span_chirho,
                                ));
                            }
                        }
                    } else {
                        self.copy_name_chirho(class_chirho)?
                    },
                    args_chirho: args_chirho
                        .iter()
                        .map(|argument_chirho| {
                            self.publish_syntax_type_chirho(
                                publication_chirho,
                                argument_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    kind_args_chirho: kind_args_chirho
                        .iter()
                        .map(|argument_chirho| {
                            self.publish_syntax_type_chirho(
                                publication_chirho,
                                argument_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    span_chirho,
                }
            }
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => ConstraintChirho::KindAnnotationChirho {
                constraint_chirho: Box::new(self.publish_syntax_constraint_chirho(
                    publication_chirho,
                    constraint_chirho,
                    depth_chirho + 1,
                )?),
                kind_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    kind_chirho,
                    depth_chirho + 1,
                )?),
                span_chirho,
            },
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                let mark_chirho = publication_chirho.bound_chirho.len();
                let result_chirho = (|| {
                    let vars_chirho = self.publish_syntax_quantifiers_chirho(
                        publication_chirho,
                        vars_chirho,
                        false,
                        depth_chirho,
                        span_chirho,
                    )?;
                    self.state_chirho.charge_chirho(
                        context_chirho.len(),
                        context_chirho.len(),
                        span_chirho,
                    )?;
                    let context_chirho = context_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.publish_syntax_constraint_chirho(
                                publication_chirho,
                                constraint_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?;
                    let body_chirho = Box::new(self.publish_syntax_constraint_chirho(
                        publication_chirho,
                        body_chirho,
                        depth_chirho + 1,
                    )?);
                    Ok(ConstraintChirho::QuantifiedChirho {
                        vars_chirho,
                        context_chirho,
                        body_chirho,
                        span_chirho,
                    })
                })();
                publication_chirho.bound_chirho.truncate(mark_chirho);
                result_chirho?
            }
        })
    }
    fn publish_syntax_type_chirho(
        &mut self,
        publication_chirho: &mut SynonymPublicationChirho,
        source_chirho: &TypeChirho,
        depth_chirho: usize,
    ) -> Result<TypeChirho, ElaborationErrorChirho> {
        let span_chirho = source_chirho.span_chirho();
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(Self::error_chirho(
                "synonym syntax publication depth bound exhausted",
                span_chirho,
            ));
        }
        Ok(match source_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                self.publish_syntax_variable_chirho(publication_chirho, name_chirho)?
            }
            TypeChirho::ConChirho(name_chirho) => {
                TypeChirho::ConChirho(self.copy_name_chirho(name_chirho)?)
            }
            TypeChirho::PromotedConChirho { name_chirho, .. } => TypeChirho::PromotedConChirho {
                name_chirho: self.copy_name_chirho(name_chirho)?,
                span_chirho,
            },
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => TypeChirho::AppChirho {
                fun_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    fun_chirho,
                    depth_chirho + 1,
                )?),
                arg_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    arg_chirho,
                    depth_chirho + 1,
                )?),
                span_chirho,
            },
            TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                written_chirho,
                ..
            } => TypeChirho::KindApplicationChirho {
                fun_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    fun_chirho,
                    depth_chirho + 1,
                )?),
                arg_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    arg_chirho,
                    depth_chirho + 1,
                )?),
                written_chirho: *written_chirho,
                span_chirho,
            },
            TypeChirho::KindAnnotationChirho {
                ty_chirho,
                kind_chirho,
                ..
            } => TypeChirho::KindAnnotationChirho {
                ty_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    ty_chirho,
                    depth_chirho + 1,
                )?),
                kind_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    kind_chirho,
                    depth_chirho + 1,
                )?),
                span_chirho,
            },
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                let mult_chirho = match mult_chirho {
                    Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::MultVarChirho(
                        name_chirho,
                    )) => {
                        let TypeChirho::VarChirho(name_chirho) =
                            self.publish_syntax_variable_chirho(publication_chirho, name_chirho)?
                        else {
                            return Err(Self::error_chirho(
                                "published multiplicity lost its checked binder",
                                span_chirho,
                            ));
                        };
                        Some(
                            haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::MultVarChirho(
                                name_chirho,
                            ),
                        )
                    }
                    other_chirho => other_chirho.clone(),
                };
                TypeChirho::FunChirho {
                    arg_chirho: Box::new(self.publish_syntax_type_chirho(
                        publication_chirho,
                        arg_chirho,
                        depth_chirho + 1,
                    )?),
                    mult_chirho,
                    result_chirho: Box::new(self.publish_syntax_type_chirho(
                        publication_chirho,
                        result_chirho,
                        depth_chirho + 1,
                    )?),
                    span_chirho,
                }
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                self.state_chirho.charge_chirho(
                    elements_chirho.len(),
                    elements_chirho.len(),
                    span_chirho,
                )?;
                let elements_chirho = elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        self.publish_syntax_type_chirho(
                            publication_chirho,
                            element_chirho,
                            depth_chirho + 1,
                        )
                    })
                    .collect::<Result<_, _>>()?;
                if matches!(source_chirho, TypeChirho::TupleChirho { .. }) {
                    TypeChirho::TupleChirho {
                        elements_chirho,
                        span_chirho,
                    }
                } else {
                    TypeChirho::PromotedListChirho {
                        elements_chirho,
                        span_chirho,
                    }
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => TypeChirho::ListChirho {
                element_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    element_chirho,
                    depth_chirho + 1,
                )?),
                span_chirho,
            },
            TypeChirho::ParenChirho { inner_chirho, .. } => TypeChirho::ParenChirho {
                inner_chirho: Box::new(self.publish_syntax_type_chirho(
                    publication_chirho,
                    inner_chirho,
                    depth_chirho + 1,
                )?),
                span_chirho,
            },
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                self.state_chirho.charge_chirho(
                    context_chirho.len(),
                    context_chirho.len(),
                    span_chirho,
                )?;
                TypeChirho::QualChirho {
                    context_chirho: context_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.publish_syntax_constraint_chirho(
                                publication_chirho,
                                constraint_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    body_chirho: Box::new(self.publish_syntax_type_chirho(
                        publication_chirho,
                        body_chirho,
                        depth_chirho + 1,
                    )?),
                    span_chirho,
                }
            }
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
                let mark_chirho = publication_chirho.bound_chirho.len();
                let required_chirho =
                    matches!(source_chirho, TypeChirho::RequiredForallChirho { .. });
                let result_chirho = (|| {
                    let vars_chirho = self.publish_syntax_quantifiers_chirho(
                        publication_chirho,
                        vars_chirho,
                        required_chirho,
                        depth_chirho,
                        span_chirho,
                    )?;
                    let body_chirho = Box::new(self.publish_syntax_type_chirho(
                        publication_chirho,
                        body_chirho,
                        depth_chirho + 1,
                    )?);
                    Ok(if required_chirho {
                        TypeChirho::RequiredForallChirho {
                            vars_chirho,
                            body_chirho,
                            span_chirho,
                        }
                    } else {
                        TypeChirho::ForallChirho {
                            vars_chirho,
                            body_chirho,
                            span_chirho,
                        }
                    })
                })();
                publication_chirho.bound_chirho.truncate(mark_chirho);
                result_chirho?
            }
            TypeChirho::WildcardChirho { .. } => {
                return Err(Self::error_chirho(
                    "unbound wildcard cannot become checked exported synonym syntax",
                    span_chirho,
                ));
            }
            TypeChirho::LitChirho { value_chirho, .. } => TypeChirho::LitChirho {
                value_chirho: self.state_chirho.name_chirho(value_chirho, span_chirho)?,
                span_chirho,
            },
        })
    }
}

impl TypeElaboratorChirho<'_> {
    pub(super) fn publish_header_terms_chirho(
        &mut self,
        identity_chirho: &Arc<str>,
        binders_chirho: &[DeclarationBinderChirho],
        result_kind_chirho: &TyChirho,
        mapping_chirho: &crate::subst_chirho::SubstChirho,
        span_chirho: SpanChirho,
    ) -> Result<CanonicalHeaderPublicationChirho, ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(binders_chirho.len(), binders_chirho.len(), span_chirho)?;
        let mut publication_chirho = SynonymPublicationChirho {
            identity_chirho: identity_chirho.clone(),
            mapping_chirho: HashMap::new(),
            top_chirho: HashSet::new(),
            bound_chirho: Vec::new(),
            syntax_introductions_chirho: HashSet::new(),
            local_binders_chirho: Vec::new(),
            next_slot_chirho: u32::try_from(binders_chirho.len()).map_err(|_| {
                Self::error_chirho("header publication slot arithmetic exhausted", span_chirho)
            })?,
        };
        for (variable_chirho, image_chirho) in mapping_chirho.iter_chirho() {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            publication_chirho.mapping_chirho.insert(
                *variable_chirho,
                self.state_chirho.copy_chirho(image_chirho, span_chirho)?,
            );
        }
        let mut canonical_binders_chirho = Vec::with_capacity(binders_chirho.len());
        // A parameter kind sees only preceding telescope parameters. The
        // dependency closure was completed before these slots were fixed.
        for (index_chirho, binder_chirho) in binders_chirho.iter().enumerate() {
            let kind_chirho = self.publish_typed_term_chirho(
                &mut publication_chirho,
                &binder_chirho.kind_chirho,
                span_chirho,
                0,
            )?;
            canonical_binders_chirho.push(DeclarationBinderChirho {
                variable_chirho: TyVarChirho(index_chirho as u32),
                kind_chirho,
                visibility_chirho: binder_chirho.visibility_chirho,
            });
            publication_chirho
                .top_chirho
                .insert(binder_chirho.variable_chirho);
        }
        let result_kind_chirho = self.publish_typed_term_chirho(
            &mut publication_chirho,
            result_kind_chirho,
            span_chirho,
            0,
        )?;
        let mut mapping_chirho = crate::subst_chirho::SubstChirho::empty_chirho();
        for (variable_chirho, image_chirho) in publication_chirho.mapping_chirho {
            mapping_chirho.insert_chirho(variable_chirho, image_chirho);
        }
        publication_chirho
            .local_binders_chirho
            .sort_by_key(|binder_chirho| binder_chirho.binder_chirho.variable_chirho);
        Ok(CanonicalHeaderPublicationChirho {
            binders_chirho: canonical_binders_chirho,
            result_kind_chirho,
            local_binders_chirho: Arc::from(publication_chirho.local_binders_chirho),
            mapping_chirho,
        })
    }
}
