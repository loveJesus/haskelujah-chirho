// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written signatures, type applications and pattern annotations share the
//! declaration solver. Every lexical exit restores its exact prior bindings.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{DeclChirho, PatSynDirChirho};
use haskelujah_ast_chirho::expr_chirho::{
    ExprChirho, GuardedExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

impl TypeElaboratorChirho<'_> {
    pub(super) fn elaborate_signature_headers_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for declaration_chirho in &mut module_chirho.decls_chirho {
            match declaration_chirho {
                DeclChirho::TypeSigChirho {
                    name_chirho,
                    ty_chirho,
                    ..
                } => {
                    self.value_signature_chirho(name_chirho, ty_chirho)?;
                }
                DeclChirho::PatSynSigDeclChirho {
                    names_chirho,
                    ty_chirho,
                    ..
                } => {
                    let checked_chirho =
                        self.scoped_chirho(|this_chirho| this_chirho.check_type_chirho(ty_chirho))?;
                    for name_chirho in names_chirho {
                        let checked_chirho = self
                            .state_chirho
                            .copy_chirho(&checked_chirho, name_chirho.span_chirho())?;
                        self.store_checked_signature_scope_chirho(
                            name_chirho,
                            ty_chirho,
                            checked_chirho,
                            true,
                        )?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub(super) fn elaborate_values_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for declaration_chirho in &mut module_chirho.decls_chirho {
            self.scoped_chirho(|this_chirho| {
                this_chirho.value_declaration_chirho(declaration_chirho)
            })?;
        }
        Ok(())
    }

    pub(super) fn elaborate_family_instances_chirho(
        &mut self,
        module_chirho: &mut ModuleChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for declaration_chirho in &mut module_chirho.decls_chirho {
            self.scoped_chirho(|this_chirho| match declaration_chirho {
                DeclChirho::TypeFamilyInstanceDeclChirho {
                    family_name_chirho,
                    kind_args_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    span_chirho,
                } => this_chirho
                    .elaborate_family_application_chirho(
                        family_name_chirho,
                        kind_args_chirho,
                        lhs_types_chirho,
                        rhs_chirho,
                        *span_chirho,
                    )
                    .map(|_| ()),
                // The cold declaration projection retains no method bodies.
                // Reuse the ordinary context/head scope before its equations.
                DeclChirho::InstanceDeclChirho { .. } => {
                    this_chirho.value_declaration_chirho(declaration_chirho)
                }
                _ => Ok(()),
            })?;
        }
        Ok(())
    }

    fn value_declaration_chirho(
        &mut self,
        declaration_chirho: &mut DeclChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 0, declaration_chirho.span_chirho())?;
        match declaration_chirho {
            DeclChirho::TypeSigChirho { .. } | DeclChirho::PatSynSigDeclChirho { .. } => {}
            DeclChirho::ForeignDeclChirho { ty_chirho, .. } => {
                self.check_type_chirho(ty_chirho)?;
            }
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } => {
                let required_chirho = self.signature_scope_chirho(name_chirho)?;
                self.matches_with_signature_chirho(matches_chirho, required_chirho.as_deref())?;
            }
            DeclChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                ..
            } => {
                self.pattern_chirho(pat_chirho)?;
                self.rhs_chirho(rhs_chirho)?;
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                ..
            } => {
                let class_scope_chirho = self.scope_undo_chirho.len();
                if let Some(proof_chirho) = self.declaration_chirho(name_chirho)? {
                    let key_chirho =
                        self.declaration_key_chirho(&proof_chirho, name_chirho.span_chirho())?;
                    self.bind_published_header_scope_chirho(
                        &key_chirho,
                        name_chirho.span_chirho(),
                    )?;
                }
                let hidden_chirho = self
                    .body_hidden_variables_chirho(class_scope_chirho, name_chirho.span_chirho())?;
                for method_chirho in methods_chirho {
                    if let Some(matches_chirho) = &mut method_chirho.default_chirho {
                        self.scoped_chirho(|this_chirho| {
                            let checked_chirho = this_chirho.scoped_chirho(|this_chirho| {
                                this_chirho.check_type_chirho(&mut method_chirho.ty_chirho)
                            })?;
                            this_chirho.store_checked_signature_scope_chirho(
                                &method_chirho.name_chirho,
                                &method_chirho.ty_chirho,
                                checked_chirho,
                                false,
                            )?;
                            for name_chirho in &hidden_chirho {
                                this_chirho
                                    .hide_variable_chirho(name_chirho, method_chirho.span_chirho)?;
                            }
                            let required_chirho =
                                this_chirho.signature_scope_chirho(&method_chirho.name_chirho)?;
                            this_chirho.matches_with_signature_chirho(
                                matches_chirho,
                                required_chirho.as_deref(),
                            )
                        })?;
                    }
                }
            }
            DeclChirho::InstanceDeclChirho {
                context_chirho,
                class_chirho,
                kind_args_chirho,
                types_chirho,
                methods_chirho,
                assoc_tf_instances_chirho,
                span_chirho,
            } => {
                let instance_scope_chirho = self.scope_undo_chirho.len();
                for constraint_chirho in context_chirho {
                    self.infer_constraint_chirho(constraint_chirho)?;
                }
                let mut head_chirho =
                    haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                        class_chirho: class_chirho.clone(),
                        args_chirho: std::mem::take(types_chirho),
                        kind_args_chirho: std::mem::take(kind_args_chirho),
                        span_chirho: *span_chirho,
                    };
                self.infer_constraint_chirho(&mut head_chirho)?;
                if let haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                    class_chirho: checked_class_chirho,
                    args_chirho,
                    kind_args_chirho: hidden_chirho,
                    ..
                } = head_chirho
                {
                    *class_chirho = checked_class_chirho;
                    *types_chirho = args_chirho;
                    *kind_args_chirho = hidden_chirho;
                }
                let hidden_chirho =
                    self.body_hidden_variables_chirho(instance_scope_chirho, *span_chirho)?;
                self.local_binds_in_scope_chirho(methods_chirho, &hidden_chirho)?;
                for equation_chirho in assoc_tf_instances_chirho {
                    self.scoped_chirho(|this_chirho| {
                        this_chirho.elaborate_family_application_chirho(
                            &mut equation_chirho.family_name_chirho,
                            &mut equation_chirho.kind_args_chirho,
                            &mut equation_chirho.lhs_types_chirho,
                            &mut equation_chirho.rhs_chirho,
                            equation_chirho.span_chirho,
                        )
                    })?;
                }
            }
            DeclChirho::TypeFamilyInstanceDeclChirho {
                family_name_chirho,
                kind_args_chirho,
                lhs_types_chirho,
                rhs_chirho,
                span_chirho,
            } => self
                .elaborate_family_application_chirho(
                    family_name_chirho,
                    kind_args_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    *span_chirho,
                )
                .map(|_| ())?,
            DeclChirho::StandaloneDerivingDeclChirho {
                context_chirho,
                class_chirho,
                types_chirho,
                span_chirho,
            } => {
                for constraint_chirho in context_chirho {
                    self.infer_constraint_chirho(constraint_chirho)?;
                }
                let mut constraint_chirho =
                    haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                        class_chirho: class_chirho.clone(),
                        args_chirho: std::mem::take(types_chirho),
                        kind_args_chirho: Vec::new(),
                        span_chirho: *span_chirho,
                    };
                self.infer_constraint_chirho(&mut constraint_chirho)?;
                if let haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                    args_chirho,
                    ..
                } = constraint_chirho
                {
                    *types_chirho = args_chirho;
                }
            }
            DeclChirho::DefaultDeclChirho { types_chirho, .. } => {
                for ty_chirho in types_chirho {
                    self.check_type_chirho(ty_chirho)?;
                }
            }
            DeclChirho::PatSynDeclChirho {
                name_chirho,
                pat_chirho,
                dir_chirho,
                ..
            } => {
                self.signature_scope_chirho(name_chirho)?;
                self.pattern_chirho(pat_chirho)?;
                if let PatSynDirChirho::ExplBidirChirho {
                    builder_binds_chirho,
                } = dir_chirho
                {
                    self.local_binds_chirho(builder_binds_chirho)?;
                }
            }
            DeclChirho::SpliceDeclChirho { expr_chirho, .. } => {
                self.expression_chirho(expr_chirho)?
            }
            DeclChirho::DataDeclChirho { .. }
            | DeclChirho::NewtypeDeclChirho { .. }
            | DeclChirho::TypeAliasDeclChirho { .. }
            | DeclChirho::TypeFamilyDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. } => {}
        }
        Ok(())
    }

    /// A declaration's variables still scope over method signatures and
    /// associated equations. Without ScopedTypeVariables they do not scope
    /// over method bodies; retain the exact original-state undo names only.
    fn body_hidden_variables_chirho(
        &mut self,
        mark_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Result<Vec<String>, ElaborationErrorChirho> {
        if self.scoped_type_variables_chirho {
            return Ok(Vec::new());
        }
        let entries_chirho = &self.scope_undo_chirho[mark_chirho..];
        self.state_chirho
            .charge_chirho(entries_chirho.len(), entries_chirho.len(), span_chirho)?;
        entries_chirho
            .iter()
            .map(|(name_chirho, _)| self.state_chirho.name_chirho(name_chirho, span_chirho))
            .collect()
    }

    fn local_binds_chirho(
        &mut self,
        binds_chirho: &mut [LocalBindChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        self.local_binds_in_scope_chirho(binds_chirho, &[])
    }

    fn local_binds_in_scope_chirho(
        &mut self,
        binds_chirho: &mut [LocalBindChirho],
        hidden_chirho: &[String],
    ) -> Result<(), ElaborationErrorChirho> {
        for bind_chirho in binds_chirho.iter_mut() {
            if let LocalBindChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } = bind_chirho
            {
                self.value_signature_chirho(name_chirho, ty_chirho)?;
            }
        }
        for bind_chirho in binds_chirho {
            self.scoped_chirho(|this_chirho| {
                let span_chirho = match bind_chirho {
                    LocalBindChirho::TypeSigChirho { .. } => return Ok(()),
                    LocalBindChirho::FunBindChirho { span_chirho, .. }
                    | LocalBindChirho::PatBindChirho { span_chirho, .. } => *span_chirho,
                };
                for name_chirho in hidden_chirho {
                    this_chirho.hide_variable_chirho(name_chirho, span_chirho)?;
                }
                match bind_chirho {
                    LocalBindChirho::TypeSigChirho { .. } => Ok(()),
                    LocalBindChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho,
                        ..
                    } => {
                        let required_chirho = this_chirho.signature_scope_chirho(name_chirho)?;
                        this_chirho.matches_with_signature_chirho(
                            matches_chirho,
                            required_chirho.as_deref(),
                        )
                    }
                    LocalBindChirho::PatBindChirho {
                        pat_chirho,
                        rhs_chirho,
                        ..
                    } => {
                        this_chirho.pattern_chirho(pat_chirho)?;
                        this_chirho.rhs_chirho(rhs_chirho)
                    }
                }
            })?;
        }
        Ok(())
    }

    fn value_signature_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        ty_chirho: &mut haskelujah_ast_chirho::ty_chirho::TypeChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let checked_chirho =
            self.scoped_chirho(|this_chirho| this_chirho.check_type_chirho(ty_chirho))?;
        self.store_checked_signature_scope_chirho(name_chirho, ty_chirho, checked_chirho, false)
    }

    fn store_checked_signature_scope_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        ty_chirho: &TypeChirho,
        checked_chirho: TyChirho,
        pattern_chirho: bool,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 1, name_chirho.span_chirho())?;
        let variables_chirho = self.checked_signature_scope_chirho(
            ty_chirho,
            name_chirho.span_chirho(),
            pattern_chirho,
        )?;
        let key_chirho = self
            .state_chirho
            .name_chirho(name_chirho.text_chirho(), name_chirho.span_chirho())?;
        let undo_key_chirho = self
            .state_chirho
            .name_chirho(&key_chirho, name_chirho.span_chirho())?;
        let previous_chirho = self.signature_variables_chirho.insert(
            key_chirho,
            ValueSignatureScopeChirho {
                variables_chirho,
                checked_chirho: Arc::new(checked_chirho),
            },
        );
        self.signature_undo_chirho
            .push((undo_key_chirho, previous_chirho));
        Ok(())
    }

    /// Reuse the identities stamped by the kind checker. Lexical source names
    /// only admit these checked variables; they never mint replacement owners.
    fn checked_signature_scope_chirho(
        &mut self,
        ty_chirho: &haskelujah_ast_chirho::ty_chirho::TypeChirho,
        span_chirho: SpanChirho,
        pattern_chirho: bool,
    ) -> Result<SourceVariablesChirho, ElaborationErrorChirho> {
        if !pattern_chirho && !self.scoped_type_variables_chirho {
            return Ok(Vec::new());
        }
        let mut variables_chirho = Vec::new();
        let mut spine_chirho = ty_chirho;
        loop {
            self.state_chirho.charge_chirho(1, 0, span_chirho)?;
            let TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } = spine_chirho
            else {
                if pattern_chirho && let TypeChirho::QualChirho { body_chirho, .. } = spine_chirho {
                    spine_chirho = body_chirho;
                    continue;
                }
                break;
            };
            for binder_chirho in vars_chirho {
                let variable_chirho = self
                    .check_variable_owner_chirho(&binder_chirho.name_chirho)?
                    .ok_or_else(|| {
                        Self::error_chirho("signature binder has no canonical owner", span_chirho)
                    })?;
                let kind_chirho = self
                    .state_chirho
                    .variable_kind_chirho(variable_chirho, span_chirho)?;
                variables_chirho.push((
                    self.state_chirho
                        .name_chirho(binder_chirho.text_chirho(), span_chirho)?,
                    variable_chirho,
                    kind_chirho,
                ));
            }
            if !pattern_chirho {
                break;
            }
            spine_chirho = body_chirho;
        }
        Ok(variables_chirho)
    }

    fn signature_scope_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> Result<Option<Arc<TyChirho>>, ElaborationErrorChirho> {
        let count_chirho = self
            .signature_variables_chirho
            .get(name_chirho.text_chirho())
            .map_or(0, |scope_chirho| scope_chirho.variables_chirho.len());
        for index_chirho in 0..count_chirho {
            let (text_chirho, variable_chirho, kind_chirho) = &self.signature_variables_chirho
                [name_chirho.text_chirho()]
            .variables_chirho[index_chirho];
            let text_chirho = self
                .state_chirho
                .name_chirho(text_chirho, name_chirho.span_chirho())?;
            let variable_chirho = *variable_chirho;
            let kind_chirho = self
                .state_chirho
                .copy_chirho(kind_chirho, name_chirho.span_chirho())?;
            self.bind_variable_chirho(
                text_chirho,
                variable_chirho,
                kind_chirho,
                name_chirho.span_chirho(),
            )?;
        }
        Ok(self
            .signature_variables_chirho
            .get(name_chirho.text_chirho())
            .map(|scope_chirho| scope_chirho.checked_chirho.clone()))
    }

    /// Consume only the equation's actual patterns. The complete checked
    /// signature decides each slot, and a residual sigma belongs to the RHS.
    fn protect_body_prefix_chirho(
        &mut self,
        mut term_chirho: &TyChirho,
        added_chirho: &mut Vec<TyVarChirho>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for _ in 0..128 {
            self.state_chirho.charge_chirho(1, 0, span_chirho)?;
            match term_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    self.state_chirho.charge_chirho(
                        vars_chirho.len(),
                        vars_chirho.len(),
                        span_chirho,
                    )?;
                    let mut pending_chirho = vars_chirho
                        .records_chirho()
                        .iter()
                        .map(|record_chirho| {
                            (
                                record_chirho.variable_chirho,
                                record_chirho.kind_chirho.clone(),
                            )
                        })
                        .collect::<Vec<_>>();
                    let mut seen_chirho = std::collections::HashSet::new();
                    while let Some((variable_chirho, kind_chirho)) = pending_chirho.pop() {
                        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                        if !seen_chirho.insert(variable_chirho) {
                            continue;
                        }
                        added_chirho.extend(
                            self.state_chirho
                                .open_rigid_scope_chirho(&[variable_chirho], span_chirho)?,
                        );
                        let kind_chirho = kind_chirho.or_else(|| {
                            self.state_chirho
                                .variable_kinds_chirho
                                .get(&variable_chirho)
                                .and_then(|record_chirho| record_chirho.kind_chirho.clone())
                        });
                        if let Some(kind_chirho) = kind_chirho {
                            let kind_chirho = self
                                .state_chirho
                                .resolve_chirho(&kind_chirho, span_chirho)?;
                            let dependencies_chirho = kind_chirho.free_vars_chirho();
                            self.state_chirho.charge_chirho(
                                dependencies_chirho.len(),
                                dependencies_chirho.len(),
                                span_chirho,
                            )?;
                            pending_chirho.extend(dependencies_chirho.into_iter().map(
                                |variable_chirho| {
                                    (
                                        variable_chirho,
                                        self.state_chirho
                                            .variable_kinds_chirho
                                            .get(&variable_chirho)
                                            .and_then(|record_chirho| {
                                                record_chirho.kind_chirho.clone()
                                            }),
                                    )
                                },
                            ));
                        }
                    }
                    term_chirho = body_chirho;
                }
                TyChirho::QualChirho { body_chirho, .. } => term_chirho = body_chirho,
                _ => return Ok(()),
            }
        }
        Err(self
            .state_chirho
            .resource_failure_chirho("body quantifier prefix depth bound exhausted", span_chirho))
    }

    fn matches_with_signature_chirho(
        &mut self,
        matches_chirho: &mut [MatchArmChirho],
        signature_chirho: Option<&TyChirho>,
    ) -> Result<(), ElaborationErrorChirho> {
        for arm_chirho in matches_chirho {
            let mut rigid_chirho = Vec::new();
            let saved_body_scope_chirho = self.state_chirho.body_scope_chirho;
            self.state_chirho.body_scope_chirho = true;
            let result_chirho = self.scoped_chirho(|this_chirho| {
                let mut current_chirho = signature_chirho;
                let mut index_chirho = 0;
                if let Some(signature_chirho) = signature_chirho {
                    this_chirho.protect_body_prefix_chirho(signature_chirho, &mut rigid_chirho, arm_chirho.span_chirho)?;
                }
                for pattern_chirho in &mut arm_chirho.pats_chirho {
                    loop {
                        this_chirho.state_chirho.charge_chirho(1, 0, pattern_chirho.span_chirho())?;
                        match current_chirho {
                            Some(TyChirho::ForallChirho { body_chirho, .. } | TyChirho::QualChirho { body_chirho, .. }) => {
                                current_chirho = Some(body_chirho);
                                index_chirho = 0;
                            }
                            Some(TyChirho::RequiredForallChirho { vars_chirho, body_chirho }) => {
                                let record_chirho = vars_chirho.record_chirho(index_chirho).ok_or_else(|| Self::error_chirho("required signature has an empty binder group", pattern_chirho.span_chirho()))?;
                                rigid_chirho.extend(this_chirho.state_chirho.open_rigid_scope_chirho(&[record_chirho.variable_chirho], pattern_chirho.span_chirho())?);
                                let mut binder_chirho = &mut *pattern_chirho;
                                while let PatChirho::ParenChirho { inner_chirho, .. } = binder_chirho { binder_chirho = inner_chirho; }
                                match binder_chirho {
                                    PatChirho::VarChirho(name_chirho) => {
                                        let span_chirho = name_chirho.span_chirho();
                                        if record_chirho.kind_chirho.is_none() || record_chirho.source_chirho.is_none() {
                                            return Err(Self::error_chirho("required pattern binder lost its checked kind or source correspondence", span_chirho));
                                        }
                                        let spelling_chirho = this_chirho.state_chirho.name_chirho(name_chirho.text_chirho(), span_chirho)?;
                                        this_chirho.bind_lexical_record_chirho(spelling_chirho, record_chirho.clone(), span_chirho)?;
                                        this_chirho.stamp_variable_chirho(name_chirho, record_chirho.variable_chirho)?;
                                    }
                                    PatChirho::WildcardChirho(_) => {}
                                    _ => return Err(Self::error_chirho("unsupported required type-binder pattern", binder_chirho.span_chirho())),
                                }
                                index_chirho += 1;
                                if index_chirho == vars_chirho.len() { current_chirho = Some(body_chirho); index_chirho = 0; }
                                break;
                            }
                            Some(TyChirho::FunChirho(_, body_chirho, _)) => {
                                this_chirho.pattern_chirho(pattern_chirho)?;
                                this_chirho.protect_body_prefix_chirho(body_chirho, &mut rigid_chirho, pattern_chirho.span_chirho())?;
                                current_chirho = Some(body_chirho);
                                index_chirho = 0;
                                break;
                            }
                            _ => { this_chirho.pattern_chirho(pattern_chirho)?; break; }
                        }
                    }
                }
                this_chirho.local_binds_chirho(&mut arm_chirho.where_binds_chirho)?;
                this_chirho.rhs_chirho(&mut arm_chirho.rhs_chirho)
            });
            self.state_chirho.body_scope_chirho = saved_body_scope_chirho;
            self.state_chirho.close_rigid_scope_chirho(rigid_chirho);
            result_chirho?;
        }
        Ok(())
    }

    fn rhs_chirho(&mut self, rhs_chirho: &mut RhsChirho) -> Result<(), ElaborationErrorChirho> {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.expression_chirho(expr_chirho),
            RhsChirho::GuardedChirho(arms_chirho) => self.guards_chirho(arms_chirho),
        }
    }

    fn guards_chirho(
        &mut self,
        arms_chirho: &mut [GuardedExprChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        for arm_chirho in arms_chirho {
            self.scoped_chirho(|this_chirho| {
                this_chirho.statements_chirho(&mut arm_chirho.quals_chirho)?;
                this_chirho.expression_chirho(&mut arm_chirho.body_chirho)
            })?;
        }
        Ok(())
    }

    fn statements_chirho(
        &mut self,
        statements_chirho: &mut [StmtChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        for statement_chirho in statements_chirho {
            match statement_chirho {
                StmtChirho::ExprChirho { expr_chirho, .. } => {
                    self.expression_chirho(expr_chirho)?
                }
                StmtChirho::BindChirho {
                    pat_chirho,
                    expr_chirho,
                    ..
                } => {
                    self.expression_chirho(expr_chirho)?;
                    self.pattern_chirho(pat_chirho)?;
                }
                StmtChirho::LetChirho { binds_chirho, .. } => {
                    self.local_binds_chirho(binds_chirho)?
                }
            }
        }
        Ok(())
    }

    fn pattern_chirho(
        &mut self,
        pattern_chirho: &mut PatChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 0, pattern_chirho.span_chirho())?;
        self.with_syntax_depth_chirho(pattern_chirho.span_chirho(), |this_chirho| {
            match pattern_chirho {
                PatChirho::ConChirho {
                    con_chirho,
                    type_args_chirho,
                    args_chirho,
                    ..
                } => {
                    this_chirho
                        .elaborate_constructor_pattern_types_chirho(con_chirho, type_args_chirho)?;
                    for argument_chirho in args_chirho {
                        this_chirho.pattern_chirho(argument_chirho)?;
                    }
                }
                PatChirho::TupleChirho {
                    elements_chirho, ..
                }
                | PatChirho::ListChirho {
                    elements_chirho, ..
                } => {
                    for element_chirho in elements_chirho {
                        this_chirho.pattern_chirho(element_chirho)?;
                    }
                }
                PatChirho::AsChirho { pattern_chirho, .. } => {
                    this_chirho.pattern_chirho(pattern_chirho)?
                }
                PatChirho::ParenChirho { inner_chirho, .. }
                | PatChirho::LazyChirho { inner_chirho, .. }
                | PatChirho::BangChirho { inner_chirho, .. } => {
                    this_chirho.pattern_chirho(inner_chirho)?
                }
                PatChirho::InfixConChirho {
                    left_chirho,
                    right_chirho,
                    ..
                } => {
                    this_chirho.pattern_chirho(left_chirho)?;
                    this_chirho.pattern_chirho(right_chirho)?;
                }
                PatChirho::RecordChirho { fields_chirho, .. } => {
                    for field_chirho in fields_chirho {
                        this_chirho.pattern_chirho(&mut field_chirho.pattern_chirho)?;
                    }
                }
                PatChirho::ViewChirho {
                    expr_chirho,
                    pat_chirho,
                    ..
                } => {
                    this_chirho.expression_chirho(expr_chirho)?;
                    this_chirho.pattern_chirho(pat_chirho)?;
                }
                PatChirho::TypeAnnotChirho {
                    pat_chirho,
                    ty_chirho,
                    ..
                } => {
                    this_chirho.check_type_chirho(ty_chirho)?;
                    this_chirho.pattern_chirho(pat_chirho)?;
                }
                PatChirho::VarChirho(_)
                | PatChirho::LitChirho(_)
                | PatChirho::WildcardChirho(_)
                | PatChirho::NegChirho { .. } => {}
            }
            Ok(())
        })
    }

    pub(super) fn with_syntax_depth_chirho<ResultChirho>(
        &mut self,
        span_chirho: SpanChirho,
        action_chirho: impl FnOnce(&mut Self) -> Result<ResultChirho, ElaborationErrorChirho>,
    ) -> Result<ResultChirho, ElaborationErrorChirho> {
        if self.type_depth_chirho >= 128 {
            return Err(Self::error_chirho(
                "kind elaboration syntax depth bound exhausted",
                span_chirho,
            ));
        }
        self.type_depth_chirho += 1;
        let result_chirho = action_chirho(self);
        self.type_depth_chirho -= 1;
        result_chirho
    }

    fn expression_chirho(
        &mut self,
        expression_chirho: &mut ExprChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 0, expression_chirho.span_chirho())?;
        self.with_syntax_depth_chirho(expression_chirho.span_chirho(), |this_chirho| {
            if matches!(expression_chirho, ExprChirho::TypeAppChirho { .. })
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
                    this_chirho.scoped_chirho(|this_chirho| {
                        this_chirho.scoped_chirho(|this_chirho| {
                            this_chirho.check_type_chirho(ty_chirho)
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
                ExprChirho::NegChirho { expr_chirho, .. }
                | ExprChirho::SpliceChirho { expr_chirho, .. }
                | ExprChirho::TypedSpliceChirho { expr_chirho, .. }
                | ExprChirho::QuoteExprChirho { expr_chirho, .. } => {
                    this_chirho.expression_chirho(expr_chirho)?
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
                    let kind_chirho = this_chirho.fresh_kind_chirho(*span_chirho)?;
                    let stored_chirho = this_chirho
                        .state_chirho
                        .copy_chirho(&kind_chirho, *span_chirho)?;
                    let variable_chirho = this_chirho
                        .state_chirho
                        .fresh_chirho(Some(stored_chirho), *span_chirho)?;
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
                    this_chirho.statements_chirho(quals_chirho)?;
                    for branch_chirho in parallel_quals_chirho {
                        this_chirho.scoped_chirho(|this_chirho| {
                            this_chirho.statements_chirho(branch_chirho)
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
                ExprChirho::QuoteDeclChirho { decls_chirho, .. } => {
                    for declaration_chirho in decls_chirho {
                        this_chirho.scoped_chirho(|this_chirho| {
                            this_chirho.value_declaration_chirho(declaration_chirho)
                        })?;
                    }
                }
                ExprChirho::QuoteTypeChirho { ty_chirho, .. } => {
                    this_chirho.infer_type_chirho(ty_chirho)?;
                }
                ExprChirho::QuotePatChirho { pat_chirho, .. } => this_chirho
                    .scoped_chirho(|this_chirho| this_chirho.pattern_chirho(pat_chirho))?,
                ExprChirho::VarChirho(_) | ExprChirho::ConChirho(_) | ExprChirho::LitChirho(_) => {}
            }
            Ok(())
        })
    }
}
