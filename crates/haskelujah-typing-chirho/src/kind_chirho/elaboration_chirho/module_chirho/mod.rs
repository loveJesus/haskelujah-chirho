// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordinary declaration SCCs share provisional headers; checked groups publish
//! canonical telescopes before their dependants. Workflow: declaration-kinds-chirho.

use super::*;
mod families_chirho;
use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, DataKindSigChirho, DeclChirho, TypeFamilyEquationChirho,
};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

impl TypeElaboratorChirho<'_> {
    fn recheck_declaration_bodies_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
        group_chirho: &[usize],
        aliases_chirho: &[usize],
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho.charge_chirho(
            group_chirho.len().saturating_mul(aliases_chirho.len() + 1),
            group_chirho.len(),
            module_chirho.span_chirho,
        )?;
        let ordered_chirho = aliases_chirho
            .iter()
            .copied()
            .filter(|index_chirho| group_chirho.contains(index_chirho))
            .chain(group_chirho.iter().copied().filter(|index_chirho| {
                !matches!(
                    module_chirho.decls_chirho[*index_chirho],
                    DeclChirho::TypeAliasDeclChirho { .. }
                )
            }))
            .collect::<Vec<_>>();
        let previous_chirho = self.rechecking_chirho;
        self.rechecking_chirho = true;
        let result_chirho = (|| {
            for index_chirho in &ordered_chirho {
                match &mut module_chirho.decls_chirho[*index_chirho] {
                    DeclChirho::TypeAliasDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        rhs_chirho,
                        ..
                    } => {
                        let key_chirho = self.local_key_chirho(name_chirho)?;
                        self.scoped_chirho(|this_chirho| {
                            this_chirho
                                .bind_header_scope_chirho(&key_chirho, name_chirho.span_chirho())?;
                            let typed_chirho = this_chirho.infer_type_chirho(rhs_chirho)?;
                            let body_chirho = this_chirho.state_chirho.resolve_chirho(
                                &typed_chirho.term_chirho,
                                name_chirho.span_chirho(),
                            )?;
                            let (body_chirho, syntax_chirho) = this_chirho
                                .publish_synonym_body_chirho(
                                    &key_chirho,
                                    type_vars_chirho,
                                    rhs_chirho,
                                    &body_chirho,
                                    name_chirho.span_chirho(),
                                )?;
                            this_chirho
                                .registry_chirho
                                .declaration_schemes_mut_chirho()
                                .complete_synonym_body_chirho(
                                    &key_chirho,
                                    body_chirho,
                                    syntax_chirho,
                                )
                                .map_err(|message_chirho| {
                                    Self::error_chirho(message_chirho, name_chirho.span_chirho())
                                })
                        })?;
                    }
                    DeclChirho::TypeFamilyDeclChirho {
                        name_chirho,
                        equations_chirho,
                        closed_chirho,
                        ..
                    } => {
                        let key_chirho = self.local_key_chirho(name_chirho)?;
                        self.elaborate_family_equations_chirho(
                            equations_chirho,
                            &key_chirho,
                            *closed_chirho,
                            name_chirho.span_chirho(),
                        )?;
                    }
                    DeclChirho::ClassDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        associated_tfs_chirho,
                        ..
                    } => {
                        let key_chirho = self.local_key_chirho(name_chirho)?;
                        self.scoped_chirho(|this_chirho| {
                            this_chirho
                                .bind_header_scope_chirho(&key_chirho, name_chirho.span_chirho())?;
                            this_chirho.check_binder_annotations_chirho(type_vars_chirho)?;
                            this_chirho
                                .elaborate_associated_chirho(associated_tfs_chirho, &mut Vec::new())
                        })?;
                    }
                    _ => {}
                }
            }
            Ok(())
        })();
        self.rechecking_chirho = previous_chirho;
        result_chirho
    }

    pub(super) fn scoped_chirho<ResultChirho>(
        &mut self,
        action_chirho: impl FnOnce(&mut Self) -> Result<ResultChirho, ElaborationErrorChirho>,
    ) -> Result<ResultChirho, ElaborationErrorChirho> {
        let mark_chirho = self.scope_undo_chirho.len();
        let signature_mark_chirho = self.signature_undo_chirho.len();
        let result_chirho = action_chirho(self);
        self.close_scope_chirho(mark_chirho);
        while self.signature_undo_chirho.len() > signature_mark_chirho {
            let (name_chirho, previous_chirho) = self.signature_undo_chirho.pop().unwrap();
            if let Some(previous_chirho) = previous_chirho {
                self.signature_variables_chirho
                    .insert(name_chirho, previous_chirho);
            } else {
                self.signature_variables_chirho.remove(&name_chirho);
            }
        }
        result_chirho
    }

    pub(super) fn check_type_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let typed_chirho = self.infer_type_at_kind_chirho(
            syntax_chirho,
            &ElaborationStateChirho::type_universe_chirho(),
        )?;
        Ok(typed_chirho.term_chirho)
    }

    pub(super) fn local_key_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> Result<Arc<str>, ElaborationErrorChirho> {
        let declaration_chirho = self.declaration_chirho(name_chirho)?.ok_or_else(|| {
            Self::error_chirho(
                "local declaration has no positive producer",
                name_chirho.span_chirho(),
            )
        })?;
        self.declaration_key_chirho(&declaration_chirho, name_chirho.span_chirho())
    }

    pub(super) fn result_kind_chirho(
        &mut self,
        key_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let scheme_chirho = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
            .ok_or_else(|| Self::error_chirho("missing provisional declaration", span_chirho))?;
        if scheme_chirho.owner_chirho.is_some() {
            return self
                .state_chirho
                .copy_chirho(&scheme_chirho.result_kind_chirho, span_chirho);
        }
        let result_chirho = self
            .declaration_variables_chirho
            .get(key_chirho)
            .and_then(|variables_chirho| variables_chirho.owned_result_kind_chirho.as_ref())
            .ok_or_else(|| {
                Self::error_chirho(
                    "local kind recheck lost its owner-local result contract",
                    span_chirho,
                )
            })?;
        self.state_chirho.copy_chirho(result_chirho, span_chirho)
    }

    pub(super) fn check_kind_contract_chirho(
        &mut self,
        key_chirho: &str,
        result_chirho: Option<&mut TypeChirho>,
        standalone_chirho: Option<&mut TypeChirho>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let expected_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
        if let Some(result_chirho) = result_chirho {
            let annotation_chirho = self.infer_kind_syntax_chirho(result_chirho)?.term_chirho;
            self.state_chirho
                .unify_chirho(&expected_chirho, &annotation_chirho, span_chirho)?;
        }
        if let Some(signature_chirho) = standalone_chirho {
            let scheme_chirho = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(key_chirho)
                .cloned()
                .ok_or_else(|| {
                    Self::error_chirho("missing declaration kind contract", span_chirho)
                })?;
            if scheme_chirho.owner_chirho != self.state_chirho.owner_chirho {
                return Err(Self::error_chirho(
                    "standalone kind contract cannot restage a published telescope",
                    span_chirho,
                ));
            }
            let mut signature_chirho = self.scoped_chirho(|this_chirho| {
                let previous_chirho =
                    std::mem::replace(&mut this_chirho.standalone_kind_scope_chirho, true);
                let result_chirho = this_chirho
                    .infer_kind_syntax_chirho(signature_chirho)
                    .map(|typed_chirho| typed_chirho.term_chirho);
                this_chirho.standalone_kind_scope_chirho = previous_chirho;
                result_chirho
            })?;
            let mut rigid_chirho = signature_chirho.free_vars_chirho();
            let mut mapping_chirho = crate::subst_chirho::SubstChirho::empty_chirho();
            let mut binders_chirho = Vec::new();
            let mut checks_chirho = Vec::new();
            let mut index_chirho = 0;
            loop {
                self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                let required_chirho =
                    matches!(&signature_chirho, TyChirho::RequiredForallChirho { .. });
                match signature_chirho {
                    TyChirho::ForallChirho {
                        vars_chirho,
                        body_chirho,
                    }
                    | TyChirho::RequiredForallChirho {
                        vars_chirho,
                        body_chirho,
                    } if index_chirho < scheme_chirho.binders_chirho.len()
                        || (!required_chirho && index_chirho == 0) =>
                    {
                        // Required quantifiers after the written header belong
                        // to the result, including a suffix of this same group.
                        let mut consumed_chirho = 0;
                        for variable_chirho in vars_chirho.iter().copied() {
                            if required_chirho && index_chirho == scheme_chirho.binders_chirho.len()
                            {
                                break;
                            }
                            let kind_chirho = self
                                .state_chirho
                                .variable_kind_chirho(variable_chirho, span_chirho)?;
                            let header_chirho = scheme_chirho.binders_chirho.get(index_chirho);
                            if required_chirho
                                || header_chirho.is_some_and(|binder_chirho| {
                                    binder_chirho.visibility_chirho
                                        != TelescopeVisibilityChirho::RequiredChirho
                                })
                            {
                                let header_chirho = header_chirho.ok_or_else(|| {
                                    Self::error_chirho(
                                        "required kind quantifier has no declaration binder",
                                        span_chirho,
                                    )
                                })?;
                                if required_chirho
                                    != (header_chirho.visibility_chirho
                                        == TelescopeVisibilityChirho::RequiredChirho)
                                {
                                    return Err(Self::error_chirho(
                                        "declaration binder visibility disagrees with its required kind quantifier",
                                        span_chirho,
                                    ));
                                }
                                mapping_chirho.insert_chirho(
                                    variable_chirho,
                                    TyChirho::VarChirho(header_chirho.variable_chirho),
                                );
                                checks_chirho.push((
                                    self.state_chirho
                                        .copy_chirho(&header_chirho.kind_chirho, span_chirho)?,
                                    kind_chirho,
                                ));
                                binders_chirho.push(header_chirho.clone());
                                index_chirho += 1;
                            } else {
                                rigid_chirho.push(variable_chirho);
                                binders_chirho.push(DeclarationBinderChirho { term_specificity_chirho: None, variable_chirho, kind_chirho, visibility_chirho: if self.state_chirho.source_specificity_chirho(variable_chirho) == Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho) { TelescopeVisibilityChirho::InferredChirho } else { TelescopeVisibilityChirho::SpecifiedChirho } });
                            }
                            consumed_chirho += 1;
                        }
                        if consumed_chirho < vars_chirho.len() {
                            let remaining_chirho = vars_chirho.len() - consumed_chirho;
                            self.state_chirho.charge_chirho(
                                remaining_chirho,
                                remaining_chirho,
                                span_chirho,
                            )?;
                            signature_chirho = TyChirho::RequiredForallChirho {
                                vars_chirho: vars_chirho
                                    .slice_chirho(consumed_chirho..vars_chirho.len()),
                                body_chirho,
                            };
                            break;
                        }
                        signature_chirho = *body_chirho;
                    }
                    TyChirho::FunChirho(argument_chirho, result_chirho, _)
                        if index_chirho < scheme_chirho.binders_chirho.len() =>
                    {
                        let header_chirho = &scheme_chirho.binders_chirho[index_chirho];
                        if header_chirho.visibility_chirho
                            != TelescopeVisibilityChirho::RequiredChirho
                        {
                            return Err(Self::error_chirho(
                                "invisible declaration binder has no kind quantifier",
                                span_chirho,
                            ));
                        }
                        checks_chirho.push((
                            self.state_chirho
                                .copy_chirho(&header_chirho.kind_chirho, span_chirho)?,
                            *argument_chirho,
                        ));
                        binders_chirho.push(header_chirho.clone());
                        index_chirho += 1;
                        signature_chirho = *result_chirho;
                    }
                    _ => break,
                }
            }
            if index_chirho != scheme_chirho.binders_chirho.len() {
                return Err(Self::error_chirho(
                    "standalone kind signature has fewer required arrows than the declaration header",
                    span_chirho,
                ));
            }
            rigid_chirho.sort();
            rigid_chirho.dedup();
            for (actual_chirho, contract_chirho) in checks_chirho {
                let contract_chirho = self.state_chirho.instantiate_term_chirho(
                    &contract_chirho,
                    &mapping_chirho,
                    true,
                    span_chirho,
                )?;
                self.state_chirho.check_rigid_chirho(
                    &actual_chirho,
                    &contract_chirho,
                    &rigid_chirho,
                    span_chirho,
                )?;
            }
            let result_chirho = self.state_chirho.instantiate_term_chirho(
                &signature_chirho,
                &mapping_chirho,
                true,
                span_chirho,
            )?;
            self.state_chirho.check_rigid_chirho(
                &expected_chirho,
                &result_chirho,
                &rigid_chirho,
                span_chirho,
            )?;
            let mut checked_chirho = scheme_chirho.as_ref().clone();
            checked_chirho.binders_chirho = binders_chirho;
            checked_chirho.reduction_arity_chirho = checked_chirho.binders_chirho.len();
            self.registry_chirho
                .declaration_schemes_mut_chirho()
                .stage_chirho(checked_chirho);
        }
        Ok(())
    }

    /// Complete written kind headers support polymorphic recursion. Inferred
    /// headers remain shared until their declaration SCC has been checked.
    /// A family's header is complete when it has a standalone kind signature:
    /// its telescope, with the signature's implicit kind variables quantified,
    /// is published before its equations are checked, so each equation
    /// instantiates it (row 711 c18b).
    fn elaborate_complete_headers_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
        group_chirho: &[usize],
    ) -> Result<(), ElaborationErrorChirho> {
        let module_span_chirho = module_chirho.span_chirho;
        for index_chirho in group_chirho {
            let declaration_chirho = &mut module_chirho.decls_chirho[*index_chirho];
            if let DeclChirho::ClassDeclChirho {
                name_chirho,
                type_vars_chirho,
                standalone_kind_chirho,
                ..
            } = declaration_chirho
            {
                let span_chirho = name_chirho.span_chirho();
                if !self.class_header_complete_chirho(
                    type_vars_chirho,
                    standalone_kind_chirho.is_some(),
                    span_chirho,
                )? {
                    continue;
                }
                let key_chirho = self.local_key_chirho(name_chirho)?;
                let previous_chirho = self.current_declaration_chirho.replace(key_chirho.clone());
                let result_chirho = self.scoped_chirho(|this_chirho| {
                    this_chirho.bind_header_scope_chirho(&key_chirho, span_chirho)?;
                    this_chirho.check_binder_annotations_chirho(type_vars_chirho)?;
                    this_chirho.check_kind_contract_chirho(
                        &key_chirho,
                        None,
                        standalone_kind_chirho.as_mut(),
                        span_chirho,
                    )?;
                    this_chirho
                        .finish_catalogue_chirho(std::slice::from_ref(&key_chirho), span_chirho)
                });
                self.current_declaration_chirho = previous_chirho;
                result_chirho?;
                continue;
            }
            // A family with a standalone kind signature: the body arm's header steps, at the
            // spans it used, then the publication its group would reach after the bodies.
            let declaration_span_chirho = declaration_chirho.span_chirho();
            if let DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                result_kind_chirho,
                standalone_kind_chirho,
                ..
            } = declaration_chirho
                && standalone_kind_chirho.is_some()
            {
                let key_chirho = self.local_key_chirho(name_chirho)?;
                let previous_chirho = self.current_declaration_chirho.replace(key_chirho.clone());
                let result_chirho = self.scoped_chirho(|this_chirho| {
                    this_chirho.bind_header_scope_chirho(&key_chirho, declaration_span_chirho)?;
                    this_chirho.check_binder_annotations_chirho(type_vars_chirho)?;
                    this_chirho.check_kind_contract_chirho(
                        &key_chirho,
                        result_kind_chirho.as_mut(),
                        standalone_kind_chirho.as_mut(),
                        declaration_span_chirho,
                    )?;
                    this_chirho.finish_catalogue_chirho(
                        std::slice::from_ref(&key_chirho),
                        module_span_chirho,
                    )
                });
                self.current_declaration_chirho = previous_chirho;
                result_chirho?;
                continue;
            }
            let (name_chirho, variables_chirho, signature_chirho, newtype_chirho) =
                match declaration_chirho {
                    DeclChirho::DataDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        kind_sig_chirho,
                        ..
                    } => (name_chirho, type_vars_chirho, kind_sig_chirho, false),
                    DeclChirho::NewtypeDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        kind_sig_chirho,
                        ..
                    } => (name_chirho, type_vars_chirho, kind_sig_chirho, true),
                    _ => continue,
                };
            let standalone_chirho = matches!(
                signature_chirho,
                Some(DataKindSigChirho::StandaloneChirho { .. })
            );
            if !standalone_chirho
                && (!self.cusks_chirho
                    || variables_chirho
                        .iter()
                        .any(|variable_chirho| variable_chirho.kind_annotation_chirho.is_none())
                    || (newtype_chirho
                        && self.unlifted_newtypes_chirho
                        && signature_chirho.is_none()))
            {
                continue;
            }
            let span_chirho = name_chirho.span_chirho();
            let key_chirho = self.local_key_chirho(name_chirho)?;
            let previous_chirho = self.current_declaration_chirho.replace(key_chirho.clone());
            let result_chirho = self.scoped_chirho(|this_chirho| {
                this_chirho.bind_header_scope_chirho(&key_chirho, span_chirho)?;
                this_chirho.check_binder_annotations_chirho(variables_chirho)?;
                let header_chirho = this_chirho
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&key_chirho)
                    .cloned()
                    .ok_or_else(|| {
                        Self::error_chirho(
                            "complete kind header has no declaration producer",
                            span_chirho,
                        )
                    })?;
                let mut known_chirho = std::collections::HashSet::new();
                for binder_chirho in &header_chirho.binders_chirho {
                    known_chirho.insert(binder_chirho.variable_chirho);
                    known_chirho.extend(
                        this_chirho
                            .state_chirho
                            .resolve_chirho(&binder_chirho.kind_chirho, span_chirho)?
                            .free_vars_chirho(),
                    );
                }
                this_chirho.data_contract_chirho(&key_chirho, signature_chirho, span_chirho)?;
                let result_chirho = this_chirho.result_kind_chirho(&key_chirho, span_chirho)?;
                let result_chirho = this_chirho
                    .state_chirho
                    .resolve_chirho(&result_chirho, span_chirho)?;
                // An inline result cannot introduce a free kind variable after
                // the written header. Explicit foralls bind their own locals.
                if !standalone_chirho
                    && result_chirho
                        .free_vars_chirho()
                        .iter()
                        .any(|variable_chirho| !known_chirho.contains(variable_chirho))
                {
                    return Ok(());
                }
                this_chirho.finish_catalogue_chirho(std::slice::from_ref(&key_chirho), span_chirho)
            });
            self.current_declaration_chirho = previous_chirho;
            result_chirho?;
        }
        Ok(())
    }

    pub(super) fn elaborate_declarations_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let dependencies_chirho::DeclarationOrderChirho {
            groups_chirho,
            aliases_chirho,
        } = self.declaration_groups_chirho(module_chirho)?;
        for group_chirho in groups_chirho {
            self.elaborate_complete_headers_chirho(module_chirho, &group_chirho)?;
            let mut keys_chirho = Vec::new();
            for index_chirho in &group_chirho {
                let declaration_chirho = &mut module_chirho.decls_chirho[*index_chirho];
                let name_chirho = match declaration_chirho {
                    DeclChirho::DataDeclChirho { name_chirho, .. }
                    | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
                    | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                    | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
                    | DeclChirho::ClassDeclChirho { name_chirho, .. } => name_chirho,
                    _ => continue,
                };
                let key_chirho = self.local_key_chirho(name_chirho)?;
                let previous_chirho = self.current_declaration_chirho.replace(key_chirho.clone());
                let result_chirho = self.scoped_chirho(|this_chirho| {
                    this_chirho
                        .bind_header_scope_chirho(&key_chirho, declaration_chirho.span_chirho())?;
                    this_chirho.elaborate_declaration_body_chirho(
                        declaration_chirho,
                        &key_chirho,
                        &mut keys_chirho,
                    )
                });
                self.current_declaration_chirho = previous_chirho;
                result_chirho?;
                keys_chirho.push(key_chirho);
            }
            let (constructors_chirho, headers_chirho): (Vec<_>, Vec<_>) =
                keys_chirho.iter().cloned().partition(|key_chirho| {
                    self.registry_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(key_chirho)
                        .is_some_and(|scheme_chirho| scheme_chirho.promoted_chirho.is_some())
                });
            self.finish_catalogue_chirho(&headers_chirho, module_chirho.span_chirho)?;
            // Transparent dependencies must carry their nested binder kinds
            // before a constructor signature can use the published telescope.
            let alias_group_chirho = group_chirho
                .iter()
                .copied()
                .filter(|index_chirho| {
                    matches!(
                        module_chirho.decls_chirho[*index_chirho],
                        DeclChirho::TypeAliasDeclChirho { .. }
                    )
                })
                .collect::<Vec<_>>();
            if constructors_chirho.iter().any(|key_chirho| {
                self.declaration_variables_chirho
                    .get(key_chirho)
                    .is_some_and(|variables_chirho| {
                        variables_chirho.written_signature_chirho.is_some()
                    })
            }) {
                self.recheck_declaration_bodies_chirho(
                    module_chirho,
                    &alias_group_chirho,
                    &aliases_chirho,
                )?;
            }
            for index_chirho in &group_chirho {
                let declaration_chirho = &mut module_chirho.decls_chirho[*index_chirho];
                let name_chirho = match declaration_chirho {
                    DeclChirho::DataDeclChirho { name_chirho, .. }
                    | DeclChirho::NewtypeDeclChirho { name_chirho, .. } => name_chirho,
                    _ => continue,
                };
                let key_chirho = self.local_key_chirho(name_chirho)?;
                let previous_chirho = self.current_declaration_chirho.replace(key_chirho.clone());
                let result_chirho = self.scoped_chirho(|this_chirho| {
                    this_chirho
                        .bind_header_scope_chirho(&key_chirho, declaration_chirho.span_chirho())?;
                    match declaration_chirho {
                        DeclChirho::DataDeclChirho {
                            constructors_chirho,
                            ..
                        } => {
                            for (constructor_index_chirho, constructor_chirho) in
                                constructors_chirho.iter_mut().enumerate()
                            {
                                this_chirho.replay_gadt_constructor_chirho(
                                    constructor_chirho,
                                    &key_chirho,
                                    constructor_index_chirho,
                                )?;
                            }
                        }
                        DeclChirho::NewtypeDeclChirho {
                            constructor_chirho, ..
                        } => this_chirho.replay_gadt_constructor_chirho(
                            constructor_chirho,
                            &key_chirho,
                            0,
                        )?,
                        _ => unreachable!(),
                    }
                    Ok(())
                });
                self.current_declaration_chirho = previous_chirho;
                result_chirho?;
            }
            self.finish_catalogue_chirho(&constructors_chirho, module_chirho.span_chirho)?;
            self.recheck_declaration_bodies_chirho(module_chirho, &group_chirho, &aliases_chirho)?;
            self.publish_closed_families_chirho(module_chirho, &group_chirho)?;
            self.check_declaration_contracts_chirho(&keys_chirho, module_chirho.span_chirho)?;
            self.check_datatype_result_roles_chirho(&headers_chirho, module_chirho.span_chirho)?;
            self.queue_declaration_field_kinds_chirho(&headers_chirho, module_chirho.span_chirho)?;
        }
        Ok(())
    }

    fn elaborate_declaration_body_chirho(
        &mut self,
        declaration_chirho: &mut DeclChirho,
        key_chirho: &Arc<str>,
        keys_chirho: &mut Vec<Arc<str>>,
    ) -> Result<(), ElaborationErrorChirho> {
        let span_chirho = declaration_chirho.span_chirho();
        match declaration_chirho {
            DeclChirho::DataDeclChirho {
                type_vars_chirho,
                constructors_chirho,
                kind_sig_chirho,
                ..
            } => {
                if self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(key_chirho)
                    .is_none_or(|scheme_chirho| scheme_chirho.owner_chirho.is_some())
                {
                    self.check_binder_annotations_chirho(type_vars_chirho)?;
                    self.data_contract_chirho(key_chirho, kind_sig_chirho, span_chirho)?;
                }
                for (constructor_index_chirho, constructor_chirho) in
                    constructors_chirho.iter_mut().enumerate()
                {
                    self.elaborate_constructor_chirho(
                        constructor_chirho,
                        key_chirho,
                        constructor_index_chirho,
                        keys_chirho,
                    )?;
                }
            }
            DeclChirho::NewtypeDeclChirho {
                type_vars_chirho,
                constructor_chirho,
                kind_sig_chirho,
                ..
            } => {
                if self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(key_chirho)
                    .is_none_or(|scheme_chirho| scheme_chirho.owner_chirho.is_some())
                {
                    self.check_binder_annotations_chirho(type_vars_chirho)?;
                    self.data_contract_chirho(key_chirho, kind_sig_chirho, span_chirho)?;
                }
                self.elaborate_constructor_chirho(constructor_chirho, key_chirho, 0, keys_chirho)?;
            }
            DeclChirho::TypeAliasDeclChirho {
                type_vars_chirho,
                rhs_chirho,
                ..
            } => {
                self.check_binder_annotations_chirho(type_vars_chirho)?;
                let expected_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
                let typed_chirho = self.infer_type_chirho(rhs_chirho)?;
                let typed_chirho = self.check_declaration_result_kind_chirho(
                    rhs_chirho,
                    typed_chirho,
                    &expected_chirho,
                )?;
                let mut scheme_chirho = self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(key_chirho)
                    .unwrap()
                    .as_ref()
                    .clone();
                scheme_chirho.synonym_rhs_chirho = Some(typed_chirho.term_chirho);
                self.registry_chirho
                    .declaration_schemes_mut_chirho()
                    .stage_chirho(scheme_chirho);
            }
            DeclChirho::TypeFamilyDeclChirho {
                type_vars_chirho,
                result_kind_chirho,
                standalone_kind_chirho,
                equations_chirho,
                closed_chirho,
                ..
            } => {
                // A complete header published early keeps its telescope; only a
                // provisional one is checked here.
                if self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(key_chirho)
                    .is_none_or(|scheme_chirho| scheme_chirho.owner_chirho.is_some())
                {
                    self.check_binder_annotations_chirho(type_vars_chirho)?;
                    if !*closed_chirho
                        && result_kind_chirho.is_none()
                        && standalone_kind_chirho.is_none()
                    {
                        let expected_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
                        self.unify_kinds_chirho(
                            &expected_chirho,
                            &ElaborationStateChirho::type_universe_chirho(),
                            span_chirho,
                        )?;
                    }
                    self.check_kind_contract_chirho(
                        key_chirho,
                        result_kind_chirho.as_mut(),
                        standalone_kind_chirho.as_mut(),
                        span_chirho,
                    )?;
                }
                self.elaborate_family_equations_chirho(
                    equations_chirho,
                    key_chirho,
                    *closed_chirho,
                    span_chirho,
                )?;
            }
            DeclChirho::ClassDeclChirho {
                standalone_kind_chirho,
                type_vars_chirho,
                context_chirho,
                methods_chirho,
                associated_tfs_chirho,
                ..
            } => {
                if standalone_kind_chirho.is_none() {
                    self.check_binder_annotations_chirho(type_vars_chirho)?;
                }
                self.with_class_kind_scope_chirho(
                    key_chirho,
                    span_chirho,
                    associated_tfs_chirho,
                    keys_chirho,
                    |this_chirho| {
                        for constraint_chirho in context_chirho {
                            this_chirho.infer_constraint_chirho(constraint_chirho)?;
                        }
                        for method_chirho in methods_chirho {
                            this_chirho.scoped_chirho(|this_chirho| {
                                this_chirho
                                    .check_class_method_type_chirho(&mut method_chirho.ty_chirho)
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Class methods cannot specialize the producer's written universals.
    /// Use live owned header variables, including kind-only dependencies; a
    /// published telescope's canonical slot numbers are not State variables.
    fn with_class_kind_scope_chirho<ResultChirho>(
        &mut self,
        key_chirho: &Arc<str>,
        span_chirho: SpanChirho,
        families_chirho: &mut [AssocTypeFamilyChirho],
        keys_chirho: &mut Vec<Arc<str>>,
        action_chirho: impl FnOnce(&mut Self) -> Result<ResultChirho, ElaborationErrorChirho>,
    ) -> Result<ResultChirho, ElaborationErrorChirho> {
        let length_chirho = self
            .declaration_variables_chirho
            .get(key_chirho)
            .ok_or_else(|| Self::error_chirho("class body lost its owned header", span_chirho))?
            .written_chirho
            .len();
        self.state_chirho
            .charge_chirho(length_chirho, length_chirho, span_chirho)?;
        let mut universals_chirho = std::collections::BTreeSet::new();
        for index_chirho in 0..length_chirho {
            let (_, variable_chirho, kind_chirho) =
                &self.declaration_variables_chirho[key_chirho].written_chirho[index_chirho];
            universals_chirho.insert(*variable_chirho);
            let kind_chirho = self.state_chirho.resolve_chirho(kind_chirho, span_chirho)?;
            for variable_chirho in kind_chirho.free_vars_chirho() {
                self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                if self
                    .state_chirho
                    .source_specificity_chirho(variable_chirho)
                    .is_some()
                {
                    universals_chirho.insert(variable_chirho);
                }
            }
        }
        self.state_chirho.charge_chirho(
            universals_chirho.len(),
            universals_chirho.len(),
            span_chirho,
        )?;
        let universals_chirho = universals_chirho.into_iter().collect::<Vec<_>>();
        let rigid_chirho = self
            .state_chirho
            .open_rigid_scope_chirho(&universals_chirho, span_chirho)?;
        let result_chirho = (|| {
            // Both binder and result-kind metas belong to these associated
            // declarations. Check their actual headers while the class's
            // universals are rigid, before guarding enclosing inference metas.
            self.elaborate_associated_chirho(families_chirho, keys_chirho)?;
            let guard_chirho = self.state_chirho.open_kind_guard_chirho(
                &universals_chirho,
                self.state_chirho.next_variable_chirho,
                span_chirho,
            )?;
            let result_chirho = action_chirho(self);
            self.state_chirho.close_kind_guard_chirho(guard_chirho);
            result_chirho
        })();
        self.state_chirho.close_rigid_scope_chirho(rigid_chirho);
        result_chirho
    }

    fn data_contract_chirho(
        &mut self,
        key_chirho: &str,
        signature_chirho: &mut Option<DataKindSigChirho>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let (result_chirho, standalone_chirho) = match signature_chirho {
            Some(DataKindSigChirho::ResultChirho(result_chirho)) => (Some(result_chirho), None),
            Some(DataKindSigChirho::StandaloneChirho {
                signature_chirho,
                result_chirho,
            }) => (result_chirho.as_mut(), Some(signature_chirho)),
            None => (None, None),
        };
        self.check_kind_contract_chirho(key_chirho, result_chirho, standalone_chirho, span_chirho)?;
        let kind_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
        let kind_chirho = self
            .state_chirho
            .resolve_chirho(&kind_chirho, span_chirho)?;
        let mut tail_chirho = &kind_chirho;
        while let TyChirho::FunChirho(_, result_chirho, _)
        | TyChirho::ForallChirho {
            body_chirho: result_chirho,
            ..
        }
        | TyChirho::RequiredForallChirho {
            body_chirho: result_chirho,
            ..
        } = tail_chirho
        {
            tail_chirho = result_chirho;
        }
        if *tail_chirho == ElaborationStateChirho::constraint_universe_chirho() {
            return Err(Self::error_chirho(
                "an ordinary datatype cannot return Constraint",
                span_chirho,
            ));
        }
        let open_newtype_chirho = self.unlifted_newtypes_chirho
            && self.registry_chirho.declaration_schemes_chirho().get_chirho(key_chirho)
                .is_some_and(|scheme_chirho| scheme_chirho.role_chirho == haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::NewtypeChirho);
        if signature_chirho.is_none() && !open_newtype_chirho {
            let result_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
            self.unify_kinds_chirho(
                &result_chirho,
                &ElaborationStateChirho::type_universe_chirho(),
                span_chirho,
            )?;
        }
        Ok(())
    }
}
