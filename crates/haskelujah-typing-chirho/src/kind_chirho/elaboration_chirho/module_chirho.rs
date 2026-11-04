// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordinary declaration SCCs share provisional headers; checked groups publish
//! canonical telescopes before their dependants. Workflow: declaration-kinds-chirho.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{DataKindSigChirho, DeclChirho, TypeFamilyEquationChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

impl TypeElaboratorChirho<'_> {
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
        let typed_chirho = self.infer_type_chirho(syntax_chirho)?;
        self.state_chirho.unify_chirho(
            &typed_chirho.kind_chirho,
            &ElaborationStateChirho::type_universe_chirho(),
            syntax_chirho.span_chirho(),
        )?;
        Ok(typed_chirho.term_chirho)
    }

    fn local_key_chirho(
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

    fn result_kind_chirho(
        &mut self,
        key_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let scheme_chirho = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
            .ok_or_else(|| Self::error_chirho("missing provisional declaration", span_chirho))?;
        self.state_chirho
            .copy_chirho(&scheme_chirho.result_kind_chirho, span_chirho)
    }

    fn check_kind_contract_chirho(
        &mut self,
        key_chirho: &str,
        result_chirho: Option<&mut TypeChirho>,
        standalone_chirho: Option<&mut TypeChirho>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let expected_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
        if let Some(result_chirho) = result_chirho {
            let annotation_chirho = self.check_type_chirho(result_chirho)?;
            self.state_chirho
                .unify_chirho(&expected_chirho, &annotation_chirho, span_chirho)?;
        }
        if let Some(signature_chirho) = standalone_chirho {
            let (signature_chirho, rigid_chirho) = self.scoped_chirho(|this_chirho| {
                let mut signature_chirho = this_chirho.check_type_chirho(signature_chirho)?;
                let mut rigid_chirho = Vec::new();
                while let TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } = signature_chirho
                {
                    rigid_chirho.extend(vars_chirho);
                    signature_chirho = *body_chirho;
                }
                rigid_chirho.extend(signature_chirho.free_vars_chirho());
                rigid_chirho.sort();
                rigid_chirho.dedup();
                Ok((signature_chirho, rigid_chirho))
            })?;
            let scheme_chirho = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(key_chirho)
                .cloned()
                .ok_or_else(|| {
                    Self::error_chirho("missing declaration kind contract", span_chirho)
                })?;
            let mut complete_chirho = expected_chirho;
            for binder_chirho in scheme_chirho.binders_chirho.iter().rev() {
                if binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho {
                    complete_chirho = ElaborationStateChirho::arrow_chirho(
                        self.state_chirho
                            .copy_chirho(&binder_chirho.kind_chirho, span_chirho)?,
                        complete_chirho,
                    );
                }
            }
            self.state_chirho.check_rigid_chirho(
                &complete_chirho,
                &signature_chirho,
                &rigid_chirho,
                span_chirho,
            )?;
            if scheme_chirho.binders_chirho.iter().all(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
            }) {
                let mut checked_chirho = scheme_chirho.as_ref().clone();
                let mut prefix_chirho = Vec::new();
                for variable_chirho in rigid_chirho {
                    prefix_chirho.push(DeclarationBinderChirho {
                        variable_chirho,
                        kind_chirho: self
                            .state_chirho
                            .variable_kind_chirho(variable_chirho, span_chirho)?,
                        visibility_chirho: TelescopeVisibilityChirho::SpecifiedChirho,
                    });
                }
                prefix_chirho.extend(checked_chirho.binders_chirho);
                checked_chirho.binders_chirho = prefix_chirho;
                checked_chirho.reduction_arity_chirho = checked_chirho.binders_chirho.len();
                self.registry_chirho
                    .declaration_schemes_mut_chirho()
                    .stage_chirho(checked_chirho);
            }
        }
        Ok(())
    }

    pub(super) fn elaborate_declarations_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for group_chirho in self.declaration_groups_chirho(module_chirho)? {
            let mut keys_chirho = Vec::new();
            for index_chirho in group_chirho {
                let declaration_chirho = &mut module_chirho.decls_chirho[index_chirho];
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
            keys_chirho.sort_by_key(|key_chirho| {
                self.registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(key_chirho)
                    .is_some_and(|scheme_chirho| scheme_chirho.promoted_chirho)
            });
            self.finish_catalogue_chirho(keys_chirho, module_chirho.span_chirho)?;
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
                self.check_binder_annotations_chirho(type_vars_chirho)?;
                self.data_contract_chirho(key_chirho, kind_sig_chirho, span_chirho)?;
                for constructor_chirho in constructors_chirho {
                    self.elaborate_constructor_chirho(constructor_chirho, key_chirho, keys_chirho)?;
                }
            }
            DeclChirho::NewtypeDeclChirho {
                type_vars_chirho,
                constructor_chirho,
                kind_sig_chirho,
                ..
            } => {
                self.check_binder_annotations_chirho(type_vars_chirho)?;
                self.data_contract_chirho(key_chirho, kind_sig_chirho, span_chirho)?;
                self.elaborate_constructor_chirho(constructor_chirho, key_chirho, keys_chirho)?;
            }
            DeclChirho::TypeAliasDeclChirho {
                type_vars_chirho,
                rhs_chirho,
                ..
            } => {
                self.check_binder_annotations_chirho(type_vars_chirho)?;
                let typed_chirho = self.infer_type_chirho(rhs_chirho)?;
                let expected_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
                self.state_chirho.unify_chirho(
                    &expected_chirho,
                    &typed_chirho.kind_chirho,
                    span_chirho,
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
                ..
            } => {
                self.check_binder_annotations_chirho(type_vars_chirho)?;
                self.check_kind_contract_chirho(
                    key_chirho,
                    result_kind_chirho.as_mut(),
                    standalone_kind_chirho.as_mut(),
                    span_chirho,
                )?;
                for equation_chirho in equations_chirho {
                    self.elaborate_equation_chirho(equation_chirho, key_chirho)?;
                }
            }
            DeclChirho::ClassDeclChirho {
                type_vars_chirho,
                context_chirho,
                methods_chirho,
                associated_tfs_chirho,
                ..
            } => {
                self.check_binder_annotations_chirho(type_vars_chirho)?;
                for constraint_chirho in context_chirho {
                    self.infer_constraint_chirho(constraint_chirho)?;
                }
                for method_chirho in methods_chirho {
                    self.scoped_chirho(|this_chirho| {
                        this_chirho.check_type_chirho(&mut method_chirho.ty_chirho)
                    })?;
                }
                for family_chirho in associated_tfs_chirho {
                    let family_key_chirho = self.local_key_chirho(&family_chirho.name_chirho)?;
                    self.scoped_chirho(|this_chirho| {
                        this_chirho.bind_header_scope_chirho(
                            &family_key_chirho,
                            family_chirho.span_chirho,
                        )?;
                        this_chirho
                            .check_binder_annotations_chirho(&mut family_chirho.type_vars_chirho)?;
                        this_chirho.check_kind_contract_chirho(
                            &family_key_chirho,
                            family_chirho.result_kind_chirho.as_mut(),
                            None,
                            family_chirho.span_chirho,
                        )?;
                        if let Some(rhs_chirho) = &mut family_chirho.default_rhs_chirho {
                            if let Some(binders_chirho) = &mut family_chirho.default_binders_chirho
                            {
                                this_chirho.bind_forall_chirho(binders_chirho)?;
                            }
                            let typed_chirho = this_chirho.infer_type_chirho(rhs_chirho)?;
                            let expected_chirho = this_chirho.result_kind_chirho(
                                &family_key_chirho,
                                family_chirho.span_chirho,
                            )?;
                            this_chirho.state_chirho.unify_chirho(
                                &expected_chirho,
                                &typed_chirho.kind_chirho,
                                family_chirho.span_chirho,
                            )?;
                        }
                        Ok(())
                    })?;
                    keys_chirho.push(family_key_chirho);
                }
            }
            _ => {}
        }
        Ok(())
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
        if signature_chirho.is_none() {
            let result_chirho = self.result_kind_chirho(key_chirho, span_chirho)?;
            self.state_chirho.unify_chirho(
                &result_chirho,
                &ElaborationStateChirho::type_universe_chirho(),
                span_chirho,
            )?;
        }
        Ok(())
    }

    fn elaborate_equation_chirho(
        &mut self,
        equation_chirho: &mut TypeFamilyEquationChirho,
        key_chirho: &str,
    ) -> Result<(), ElaborationErrorChirho> {
        if equation_chirho.is_held_chirho() {
            return Ok(());
        }
        self.scoped_chirho(|this_chirho| {
            // Equation variables have their own lexical owner, independent of the header.
            let header_names_chirho = this_chirho
                .declaration_variables_chirho
                .get(key_chirho)
                .map(|variables_chirho| {
                    variables_chirho
                        .iter()
                        .map(|(name_chirho, _, _)| name_chirho.clone())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            for name_chirho in header_names_chirho {
                this_chirho.hide_variable_chirho(&name_chirho, equation_chirho.span_chirho)?;
            }
            if let Some(binders_chirho) = &mut equation_chirho.explicit_binders_chirho {
                this_chirho.bind_forall_chirho(binders_chirho)?;
            }
            let head_chirho = equation_chirho.head_chirho.as_mut().ok_or_else(|| {
                Self::error_chirho("family equation has no head", equation_chirho.span_chirho)
            })?;
            this_chirho.elaborate_family_application_chirho(
                head_chirho,
                &mut equation_chirho.kind_args_chirho,
                &mut equation_chirho.lhs_types_chirho,
                &mut equation_chirho.rhs_chirho,
                equation_chirho.span_chirho,
            )
        })
    }

    pub(super) fn elaborate_family_application_chirho(
        &mut self,
        head_chirho: &mut NameChirho,
        hidden_chirho: &mut Vec<TypeChirho>,
        visible_chirho: &mut Vec<TypeChirho>,
        rhs_chirho: &mut TypeChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut constraint_chirho = ConstraintChirho::ClassChirho {
            class_chirho: head_chirho.clone(),
            kind_args_chirho: std::mem::take(hidden_chirho),
            args_chirho: std::mem::take(visible_chirho),
            span_chirho,
        };
        // Families return arbitrary kinds, so use the spine without imposing Constraint.
        let ConstraintChirho::ClassChirho {
            class_chirho,
            kind_args_chirho,
            args_chirho,
            ..
        } = &mut constraint_chirho
        else {
            unreachable!()
        };
        let mut syntax_chirho = TypeChirho::ConChirho(class_chirho.clone());
        for argument_chirho in std::mem::take(kind_args_chirho) {
            syntax_chirho = TypeChirho::KindApplicationChirho {
                fun_chirho: Box::new(syntax_chirho),
                arg_chirho: Box::new(argument_chirho),
                written_chirho: true,
                span_chirho,
            };
        }
        for argument_chirho in std::mem::take(args_chirho) {
            syntax_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(syntax_chirho),
                arg_chirho: Box::new(argument_chirho),
                span_chirho,
            };
        }
        let left_chirho = self.infer_type_chirho(&mut syntax_chirho)?;
        let right_chirho = self.infer_type_chirho(rhs_chirho)?;
        self.state_chirho.unify_chirho(
            &left_chirho.kind_chirho,
            &right_chirho.kind_chirho,
            span_chirho,
        )?;
        loop {
            match syntax_chirho {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    visible_chirho.push(*arg_chirho);
                    syntax_chirho = *fun_chirho;
                }
                TypeChirho::KindApplicationChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    hidden_chirho.push(*arg_chirho);
                    syntax_chirho = *fun_chirho;
                }
                TypeChirho::ConChirho(name_chirho) => {
                    *head_chirho = name_chirho;
                    break;
                }
                _ => {
                    return Err(Self::error_chirho(
                        "family application lost its checked head",
                        span_chirho,
                    ));
                }
            }
        }
        visible_chirho.reverse();
        hidden_chirho.reverse();
        Ok(())
    }
}
