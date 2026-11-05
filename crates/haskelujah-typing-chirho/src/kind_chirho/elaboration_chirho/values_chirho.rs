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
            if let DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } = declaration_chirho
            {
                self.value_signature_chirho(name_chirho, ty_chirho)?;
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
            DeclChirho::TypeSigChirho { .. } => {}
            DeclChirho::ForeignDeclChirho { ty_chirho, .. } => {
                self.check_type_chirho(ty_chirho)?;
            }
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } => {
                let required_chirho = self.signature_scope_chirho(name_chirho)?;
                self.matches_with_required_chirho(matches_chirho, &required_chirho)?;
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
                if let Some(proof_chirho) = self.declaration_chirho(name_chirho)? {
                    let key_chirho =
                        self.declaration_key_chirho(&proof_chirho, name_chirho.span_chirho())?;
                    self.bind_header_scope_chirho(&key_chirho, name_chirho.span_chirho())?;
                }
                for method_chirho in methods_chirho {
                    if let Some(matches_chirho) = &mut method_chirho.default_chirho {
                        self.scoped_chirho(|this_chirho| {
                            let checked_chirho = this_chirho.scoped_chirho(|this_chirho| {
                                this_chirho.check_type_chirho(&mut method_chirho.ty_chirho)
                            })?;
                            this_chirho.store_checked_signature_scope_chirho(
                                &method_chirho.name_chirho,
                                &method_chirho.ty_chirho,
                                &checked_chirho,
                            )?;
                            let required_chirho =
                                this_chirho.signature_scope_chirho(&method_chirho.name_chirho)?;
                            this_chirho
                                .matches_with_required_chirho(matches_chirho, &required_chirho)
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
                self.local_binds_chirho(methods_chirho)?;
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
                pat_chirho,
                dir_chirho,
                ..
            } => {
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

    fn local_binds_chirho(
        &mut self,
        binds_chirho: &mut [LocalBindChirho],
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
            self.scoped_chirho(|this_chirho| match bind_chirho {
                LocalBindChirho::TypeSigChirho { .. } => Ok(()),
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } => {
                    let required_chirho = this_chirho.signature_scope_chirho(name_chirho)?;
                    this_chirho.matches_with_required_chirho(matches_chirho, &required_chirho)
                }
                LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    rhs_chirho,
                    ..
                } => {
                    this_chirho.pattern_chirho(pat_chirho)?;
                    this_chirho.rhs_chirho(rhs_chirho)
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
        self.store_checked_signature_scope_chirho(name_chirho, ty_chirho, &checked_chirho)
    }

    fn store_checked_signature_scope_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        ty_chirho: &TypeChirho,
        checked_chirho: &TyChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let required_chirho =
            self.checked_required_signature_chirho(checked_chirho, name_chirho.span_chirho())?;
        if !self.scoped_type_variables_chirho && required_chirho.is_empty() {
            return Ok(());
        }
        let variables_chirho =
            self.checked_signature_scope_chirho(ty_chirho, name_chirho.span_chirho())?;
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
                required_chirho,
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
    ) -> Result<SourceVariablesChirho, ElaborationErrorChirho> {
        if !self.scoped_type_variables_chirho {
            return Ok(Vec::new());
        }
        let mut variables_chirho = Vec::new();
        if let haskelujah_ast_chirho::ty_chirho::TypeChirho::ForallChirho { vars_chirho, .. } =
            ty_chirho
        {
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
        }
        Ok(variables_chirho)
    }

    /// The common elaborator already expands the exact checked synonym RHS.
    /// Only its required records consume patterns; written invisible names
    /// continue to enter lexical scope through the original source syntax.
    fn checked_required_signature_chirho(
        &mut self,
        mut term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<crate::ty_chirho::QuantifiedVariablesChirho, ElaborationErrorChirho> {
        let mut records_chirho = Vec::new();
        loop {
            self.state_chirho.charge_chirho(1, 0, span_chirho)?;
            match term_chirho {
                TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    self.state_chirho.charge_chirho(
                        vars_chirho.len(),
                        vars_chirho.len(),
                        span_chirho,
                    )?;
                    for record_chirho in vars_chirho.records_chirho() {
                        if record_chirho.kind_chirho.is_none()
                            || record_chirho.source_chirho.is_none()
                        {
                            return Err(Self::error_chirho(
                                "required signature lost its checked kind or binder correspondence",
                                span_chirho,
                            ));
                        }
                        records_chirho.push(record_chirho.clone());
                    }
                    term_chirho = body_chirho;
                }
                TyChirho::ForallChirho { body_chirho, .. }
                | TyChirho::QualChirho { body_chirho, .. } => term_chirho = body_chirho,
                _ => break,
            }
        }
        Ok(crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(records_chirho))
    }

    fn signature_scope_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> Result<crate::ty_chirho::QuantifiedVariablesChirho, ElaborationErrorChirho> {
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
            .map(|scope_chirho| scope_chirho.required_chirho.clone())
            .unwrap_or_default())
    }

    fn matches_with_required_chirho(
        &mut self,
        matches_chirho: &mut [MatchArmChirho],
        required_chirho: &crate::ty_chirho::QuantifiedVariablesChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        for arm_chirho in matches_chirho {
            let rigid_chirho = self
                .state_chirho
                .open_rigid_scope_chirho(required_chirho, arm_chirho.span_chirho)?;
            let result_chirho = self.scoped_chirho(|this_chirho| {
                if arm_chirho.pats_chirho.len() < required_chirho.len() {
                    return Err(Self::error_chirho(
                        "function equation omits a required type binder",
                        arm_chirho.span_chirho,
                    ));
                }
                for (pattern_chirho, record_chirho) in arm_chirho
                    .pats_chirho
                    .iter_mut()
                    .zip(required_chirho.records_chirho())
                {
                    let mut binder_chirho = pattern_chirho;
                    while let PatChirho::ParenChirho { inner_chirho, .. } = binder_chirho {
                        binder_chirho = inner_chirho;
                    }
                    match binder_chirho {
                        PatChirho::VarChirho(name_chirho) => {
                            let span_chirho = name_chirho.span_chirho();
                            if record_chirho.kind_chirho.is_none() {
                                return Err(Self::error_chirho(
                                    "required pattern binder lost its checked kind",
                                    span_chirho,
                                ));
                            }
                            let spelling_chirho = this_chirho
                                .state_chirho
                                .name_chirho(name_chirho.text_chirho(), span_chirho)?;
                            this_chirho.bind_lexical_record_chirho(
                                spelling_chirho,
                                record_chirho.clone(),
                                span_chirho,
                            )?;
                            this_chirho.stamp_variable_chirho(
                                name_chirho,
                                record_chirho.variable_chirho,
                            )?;
                        }
                        PatChirho::WildcardChirho(_) => {}
                        _ => {
                            return Err(Self::error_chirho(
                                "unsupported required type-binder pattern",
                                binder_chirho.span_chirho(),
                            ));
                        }
                    }
                }
                for pat_chirho in &mut arm_chirho.pats_chirho[required_chirho.len()..] {
                    this_chirho.pattern_chirho(pat_chirho)?;
                }
                this_chirho.local_binds_chirho(&mut arm_chirho.where_binds_chirho)?;
                this_chirho.rhs_chirho(&mut arm_chirho.rhs_chirho)
            });
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
                        let variables_chirho = this_chirho
                            .checked_signature_scope_chirho(ty_chirho, ty_chirho.span_chirho())?;
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
