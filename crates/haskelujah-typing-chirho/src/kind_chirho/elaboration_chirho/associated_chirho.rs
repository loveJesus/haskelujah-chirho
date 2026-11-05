// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Associated declarations and defaults share their class's checked catalogue.
//! The producer records lexical binder correspondence before family shadowing.

use super::*;
use crate::ty_chirho::declarations_chirho::{
    AssociatedDefaultEquationChirho, DeclarationAssociationChirho,
};
use haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

impl TypeElaboratorChirho<'_> {
    pub(super) fn publish_associations_chirho(
        &mut self,
        pending_chirho: Vec<(Arc<str>, DeclarationAssociationChirho)>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for (key_chirho, association_chirho) in pending_chirho {
            let association_chirho = match association_chirho {
                DeclarationAssociationChirho::FamilyChirho {
                    class_chirho,
                    correspondence_chirho,
                    default_chirho: None,
                } => {
                    let family_chirho =
                        &self.declaration_variables_chirho[&key_chirho].publication_chirho;
                    let class_map_chirho = self
                        .declaration_variables_chirho
                        .get(&class_chirho)
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "associated family lost its local class owner",
                                span_chirho,
                            )
                        })?;
                    let class_map_chirho = &class_map_chirho.publication_chirho;
                    let family_top_chirho = self
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(&key_chirho)
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "associated family lost its checked telescope",
                                span_chirho,
                            )
                        })?
                        .binders_chirho
                        .len();
                    let class_top_chirho = self
                        .registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(&class_chirho)
                        .ok_or_else(|| {
                            Self::error_chirho(
                                "associated owner lost its checked telescope",
                                span_chirho,
                            )
                        })?
                        .binders_chirho
                        .len();
                    let mut canonical_chirho = HashMap::new();
                    for (family_source_chirho, class_source_chirho) in correspondence_chirho {
                        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                        let (
                            Some(TyChirho::VarChirho(family_slot_chirho)),
                            Some(TyChirho::VarChirho(class_slot_chirho)),
                        ) = (
                            family_chirho.lookup_chirho(&family_source_chirho),
                            class_map_chirho.lookup_chirho(&class_source_chirho),
                        )
                        else {
                            return Err(Self::error_chirho(
                                "associated binder correspondence is unbound at publication",
                                span_chirho,
                            ));
                        };
                        if family_slot_chirho.0 as usize >= family_top_chirho
                            || class_slot_chirho.0 as usize >= class_top_chirho
                        {
                            return Err(Self::error_chirho(
                                "associated correspondence names a lexical rather than telescope binder",
                                span_chirho,
                            ));
                        }
                        canonical_chirho.insert(*family_slot_chirho, *class_slot_chirho);
                    }
                    // Shared hidden kinds are the same owned variables, rather
                    // than a guessed prefix length or a repeated source spelling.
                    for (source_chirho, family_slot_chirho) in family_chirho.iter_chirho() {
                        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                        if let (
                            TyChirho::VarChirho(family_slot_chirho),
                            Some(TyChirho::VarChirho(class_slot_chirho)),
                        ) = (
                            family_slot_chirho,
                            class_map_chirho.lookup_chirho(source_chirho),
                        ) && (family_slot_chirho.0 as usize) < family_top_chirho
                            && (class_slot_chirho.0 as usize) < class_top_chirho
                            && canonical_chirho
                                .insert(*family_slot_chirho, *class_slot_chirho)
                                .is_some_and(|previous_chirho| {
                                    previous_chirho != *class_slot_chirho
                                })
                        {
                            return Err(Self::error_chirho(
                                "inconsistent associated kind correspondence",
                                span_chirho,
                            ));
                        }
                    }
                    let mut correspondence_chirho =
                        canonical_chirho.into_iter().collect::<Vec<_>>();
                    correspondence_chirho.sort();
                    DeclarationAssociationChirho::FamilyChirho {
                        class_chirho,
                        correspondence_chirho,
                        default_chirho: None,
                    }
                }
                DeclarationAssociationChirho::FamilyChirho {
                    default_chirho: Some(_),
                    ..
                } => {
                    return Err(Self::error_chirho(
                        "associated default was published before its full telescope",
                        span_chirho,
                    ));
                }
                other_chirho => other_chirho,
            };
            self.registry_chirho
                .declaration_schemes_mut_chirho()
                .complete_association_chirho(&key_chirho, association_chirho)
                .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))?;
        }
        Ok(())
    }

    pub(super) fn check_declaration_contracts_chirho(
        &self,
        keys_chirho: &[Arc<str>],
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for key_chirho in keys_chirho {
            if let Some(contract_chirho) = self.declaration_contracts_chirho.get(key_chirho)
                && self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(key_chirho)
                    != Some(contract_chirho)
            {
                return Err(Self::error_chirho(
                    "source declaration does not match its imported or boot telescope",
                    span_chirho,
                ));
            }
        }
        Ok(())
    }

    fn associated_headers_chirho(
        &mut self,
        families_chirho: &[AssocTypeFamilyChirho],
    ) -> Result<Vec<usize>, ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(
            families_chirho.len(),
            families_chirho.len(),
            SpanChirho::DUMMY_CHIRHO,
        )?;
        let mut headers_chirho = HashMap::<Arc<str>, (usize, usize, usize)>::new();
        let mut order_chirho = Vec::new();
        for (index_chirho, family_chirho) in families_chirho.iter().enumerate() {
            let key_chirho = self.local_key_chirho(&family_chirho.name_chirho)?;
            let default_chirho = family_chirho.default_rhs_chirho.is_some();
            if let Some((selected_chirho, declarations_chirho, defaults_chirho)) =
                headers_chirho.get_mut(&key_chirho)
            {
                if default_chirho {
                    *defaults_chirho += 1;
                } else {
                    *declarations_chirho += 1;
                    *selected_chirho = index_chirho;
                }
                if *declarations_chirho > 1 || *defaults_chirho > 1 {
                    return Err(Self::error_chirho(
                        "duplicate associated family header or default",
                        family_chirho.span_chirho,
                    ));
                }
            } else {
                order_chirho.push(key_chirho.clone());
                headers_chirho.insert(
                    key_chirho,
                    (
                        index_chirho,
                        usize::from(!default_chirho),
                        usize::from(default_chirho),
                    ),
                );
            }
        }
        Ok(order_chirho
            .into_iter()
            .map(|key_chirho| headers_chirho[&key_chirho].0)
            .collect())
    }

    pub(super) fn predeclare_associated_chirho(
        &mut self,
        class_name_chirho: &NameChirho,
        families_chirho: &mut [AssocTypeFamilyChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        let class_key_chirho = self.local_key_chirho(class_name_chirho)?;
        let indices_chirho = self.associated_headers_chirho(families_chirho)?;
        let mut family_keys_chirho = Vec::new();
        self.scoped_chirho(|this_chirho| {
            this_chirho
                .bind_header_scope_chirho(&class_key_chirho, class_name_chirho.span_chirho())?;
            for index_chirho in indices_chirho {
                let family_chirho = &mut families_chirho[index_chirho];
                // This lexical lookup is at the defining class producer. The
                // relation is retained by IDs before the new formals shadow it.
                let parent_chirho = family_chirho
                    .type_vars_chirho
                    .iter()
                    .map(|binder_chirho| {
                        this_chirho
                            .variables_chirho
                            .get(binder_chirho.text_chirho())
                            .map(|record_chirho| record_chirho.variable_chirho)
                    })
                    .collect::<Vec<_>>();
                this_chirho.predeclare_header_chirho(
                    &mut family_chirho.name_chirho,
                    &mut family_chirho.type_vars_chirho,
                    false,
                    family_chirho.injectivity_chirho,
                )?;
                let key_chirho = this_chirho.local_key_chirho(&family_chirho.name_chirho)?;
                let header_chirho = this_chirho
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&key_chirho)
                    .expect("just predeclared family");
                let correspondence_chirho = header_chirho
                    .binders_chirho
                    .iter()
                    .zip(parent_chirho)
                    .filter_map(|(binder_chirho, parent_chirho)| {
                        parent_chirho
                            .map(|parent_chirho| (binder_chirho.variable_chirho, parent_chirho))
                    })
                    .collect();
                this_chirho
                    .registry_chirho
                    .declaration_schemes_mut_chirho()
                    .complete_association_chirho(
                        &key_chirho,
                        DeclarationAssociationChirho::FamilyChirho {
                            class_chirho: class_key_chirho.clone(),
                            correspondence_chirho,
                            default_chirho: None,
                        },
                    )
                    .map_err(|message_chirho| {
                        Self::error_chirho(message_chirho, family_chirho.span_chirho)
                    })?;
                family_keys_chirho.push(key_chirho);
            }
            Ok(())
        })?;
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .complete_association_chirho(
                &class_key_chirho,
                DeclarationAssociationChirho::ClassChirho {
                    families_chirho: family_keys_chirho,
                },
            )
            .map_err(|message_chirho| {
                Self::error_chirho(message_chirho, class_name_chirho.span_chirho())
            })
    }

    pub(super) fn elaborate_associated_chirho(
        &mut self,
        families_chirho: &mut [AssocTypeFamilyChirho],
        keys_chirho: &mut Vec<Arc<str>>,
    ) -> Result<(), ElaborationErrorChirho> {
        let indices_chirho = self.associated_headers_chirho(families_chirho)?;
        for index_chirho in indices_chirho {
            let family_chirho = &mut families_chirho[index_chirho];
            let key_chirho = self.local_key_chirho(&family_chirho.name_chirho)?;
            if !self.rechecking_chirho {
                let association_chirho = self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&key_chirho)
                    .expect("associated header")
                    .association_chirho
                    .clone();
                if let DeclarationAssociationChirho::FamilyChirho {
                    correspondence_chirho,
                    ..
                } = association_chirho
                {
                    for (family_variable_chirho, class_variable_chirho) in correspondence_chirho {
                        let family_kind_chirho = self.state_chirho.variable_kind_chirho(
                            family_variable_chirho,
                            family_chirho.span_chirho,
                        )?;
                        let class_kind_chirho = self.state_chirho.variable_kind_chirho(
                            class_variable_chirho,
                            family_chirho.span_chirho,
                        )?;
                        self.unify_kinds_chirho(
                            &family_kind_chirho,
                            &class_kind_chirho,
                            family_chirho.span_chirho,
                        )?;
                    }
                }
                self.scoped_chirho(|this_chirho| {
                    this_chirho.bind_header_scope_chirho(&key_chirho, family_chirho.span_chirho)?;
                    this_chirho
                        .check_binder_annotations_chirho(&mut family_chirho.type_vars_chirho)?;
                    if family_chirho.result_kind_chirho.is_none() {
                        let result_chirho = this_chirho
                            .result_kind_chirho(&key_chirho, family_chirho.span_chirho)?;
                        this_chirho.unify_kinds_chirho(
                            &result_chirho,
                            &ElaborationStateChirho::type_universe_chirho(),
                            family_chirho.span_chirho,
                        )?;
                    }
                    this_chirho.check_kind_contract_chirho(
                        &key_chirho,
                        family_chirho.result_kind_chirho.as_mut(),
                        None,
                        family_chirho.span_chirho,
                    )
                })?;
                keys_chirho.push(key_chirho);
            }
        }
        // A default is an equation checked against the generalized header; it
        // cannot help infer or specialize the class/family declaration's kind.
        if !self.rechecking_chirho {
            return Ok(());
        }
        for family_chirho in families_chirho {
            if family_chirho.default_rhs_chirho.is_none() {
                continue;
            }
            let key_chirho = self.local_key_chirho(&family_chirho.name_chirho)?;
            self.scoped_chirho(|this_chirho| {
                this_chirho.bind_header_scope_chirho(&key_chirho, family_chirho.span_chirho)?;
                if let Some(binders_chirho) = &mut family_chirho.default_binders_chirho {
                    let mut names_chirho = Vec::new();
                    for name_chirho in this_chirho.variables_chirho.keys() {
                        names_chirho.push(
                            this_chirho
                                .state_chirho
                                .name_chirho(name_chirho, family_chirho.span_chirho)?,
                        );
                    }
                    for name_chirho in names_chirho {
                        this_chirho
                            .hide_variable_chirho(&name_chirho, family_chirho.span_chirho)?;
                    }
                    this_chirho.bind_forall_chirho(binders_chirho)?;
                }
                let (mut hidden_chirho, mut visible_chirho) =
                    if let Some(arguments_chirho) = family_chirho.default_arguments_chirho.take() {
                        arguments_chirho
                    } else {
                        let mut arguments_chirho = Vec::new();
                        if family_chirho.default_params_chirho.is_empty() {
                            for binder_chirho in family_chirho
                                .type_vars_chirho
                                .iter()
                                .filter(|binder_chirho| binder_chirho.is_visible_chirho())
                            {
                                arguments_chirho.push(TypeChirho::VarChirho(
                                    this_chirho.copy_name_chirho(&binder_chirho.name_chirho)?,
                                ));
                            }
                        } else {
                            for name_chirho in &family_chirho.default_params_chirho {
                                arguments_chirho.push(TypeChirho::VarChirho(
                                    this_chirho.copy_name_chirho(name_chirho)?,
                                ));
                            }
                        }
                        (Vec::new(), arguments_chirho)
                    };
                let mut head_chirho = this_chirho.copy_name_chirho(&family_chirho.name_chirho)?;
                let equation_chirho = this_chirho.elaborate_family_application_chirho(
                    &mut head_chirho,
                    &mut hidden_chirho,
                    &mut visible_chirho,
                    family_chirho
                        .default_rhs_chirho
                        .as_mut()
                        .expect("default RHS"),
                    family_chirho.span_chirho,
                );
                family_chirho.default_arguments_chirho = Some((hidden_chirho, visible_chirho));
                let (lhs_chirho, rhs_chirho) = equation_chirho?;
                if this_chirho.rechecking_chirho {
                    this_chirho.publish_associated_default_chirho(
                        &key_chirho,
                        lhs_chirho,
                        rhs_chirho,
                        family_chirho.span_chirho,
                    )?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }

    fn publish_associated_default_chirho(
        &mut self,
        key_chirho: &Arc<str>,
        lhs_chirho: Vec<TyChirho>,
        rhs_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let header_chirho = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
            .expect("checked associated header")
            .clone();
        if lhs_chirho.len() != header_chirho.binders_chirho.len() {
            return Err(Self::error_chirho(
                "associated default lost its complete checked telescope",
                span_chirho,
            ));
        }
        let mut rename_chirho = crate::subst_chirho::SubstChirho::empty_chirho();
        let mut lhs_chirho = lhs_chirho
            .into_iter()
            .map(|term_chirho| self.state_chirho.resolve_chirho(&term_chirho, span_chirho))
            .collect::<Result<Vec<_>, _>>()?;
        for (argument_chirho, binder_chirho) in lhs_chirho.iter().zip(&header_chirho.binders_chirho)
        {
            if let TyChirho::VarChirho(variable_chirho) = argument_chirho {
                if rename_chirho.lookup_chirho(variable_chirho).is_some() {
                    return Err(Self::error_chirho(
                        "associated default repeats a variable in distinct telescope slots",
                        span_chirho,
                    ));
                }
                rename_chirho.insert_chirho(
                    *variable_chirho,
                    TyChirho::VarChirho(binder_chirho.variable_chirho),
                );
            } else {
                return Err(Self::error_chirho(
                    "associated default arguments, including hidden kinds, must be distinct variables",
                    span_chirho,
                ));
            }
        }
        for argument_chirho in &mut lhs_chirho {
            *argument_chirho = self.state_chirho.instantiate_term_chirho(
                argument_chirho,
                &rename_chirho,
                false,
                span_chirho,
            )?;
        }
        let rhs_chirho = self.state_chirho.resolve_chirho(&rhs_chirho, span_chirho)?;
        let rhs_chirho = self.state_chirho.instantiate_term_chirho(
            &rhs_chirho,
            &rename_chirho,
            false,
            span_chirho,
        )?;
        let DeclarationAssociationChirho::FamilyChirho {
            class_chirho,
            correspondence_chirho,
            ..
        } = &header_chirho.association_chirho
        else {
            return Err(Self::error_chirho(
                "associated default has no checked class owner",
                span_chirho,
            ));
        };
        self.registry_chirho
            .declaration_schemes_mut_chirho()
            .complete_association_chirho(
                key_chirho,
                DeclarationAssociationChirho::FamilyChirho {
                    class_chirho: class_chirho.clone(),
                    correspondence_chirho: correspondence_chirho.clone(),
                    default_chirho: Some(AssociatedDefaultEquationChirho {
                        lhs_chirho,
                        rhs_chirho,
                    }),
                },
            )
            .map_err(|message_chirho| Self::error_chirho(message_chirho, span_chirho))
    }
}
