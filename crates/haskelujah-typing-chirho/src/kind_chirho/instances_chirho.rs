// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Local class-instance kind consumers; workflow: declaration-kinds-chirho.
use super::{
    DiagnosticChirho, ErrorCodeChirho, KIND_MISMATCH_CODE_CHIRHO, KindChirho, KindInferCtxChirho,
    SpanChirho, TypeChirho,
};
use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, AssocTypeFamilyChirho};

impl KindInferCtxChirho {
    /// Associated equations consume the closed family contract inside the
    /// enclosing instance's kind scope. Its generalized variables are rigid:
    /// an associated RHS must not specialize a polymorphic instance head.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn check_associated_instance_equations_chirho(
        &mut self,
        class_name_chirho: &str,
        context_chirho: &[super::ConstraintChirho],
        head_types_chirho: &[TypeChirho],
        equations_chirho: &[AssocTfInstanceChirho],
        defaults_chirho: Option<(&[super::TyVarChirho], &[AssocTypeFamilyChirho])>,
        span_chirho: SpanChirho,
    ) {
        // A polymorphic class's method schemes need the instance's hidden
        // arguments even when this instance has no associated equations.
        // Only source-owned or imported checked contracts supply that evidence.
        let has_class_binders_chirho = (self
            .local_kind_decl_names_chirho
            .contains(class_name_chirho)
            || matches!(
                self.imported_kind_shapes_chirho.get(class_name_chirho),
                Some(super::imports_chirho::KindHeadShapeChirho::ClassChirho)
            ))
            && matches!(self.env_chirho.lookup_binding_chirho(class_name_chirho),
                Some(super::KindBindingChirho::PolyChirho(scheme_chirho))
                    if !scheme_chirho.quantified_chirho.is_empty());
        if equations_chirho.is_empty()
            && !defaults_chirho.is_some_and(|(_, families_chirho)| {
                families_chirho
                    .iter()
                    .any(|family_chirho| !family_chirho.defaults_chirho.is_empty())
            })
            && !has_class_binders_chirho
        {
            return;
        }
        let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
        let diagnostic_start_chirho = self.diagnostics_chirho.len_chirho();
        self.env_chirho.begin_scope_chirho();
        // Written kind names in an instance signature are skolems even when
        // unnamed classifier variables may still be inferred from its context.
        let outer_ascriptions_chirho =
            std::mem::replace(&mut self.rigid_ascription_names_chirho, true);
        let mut class_kind_chirho = self
            .env_chirho
            .lookup_binding_chirho(class_name_chirho)
            .cloned()
            .map(|binding_chirho| {
                self.instantiate_source_binding_chirho(
                    class_name_chirho,
                    &binding_chirho,
                    span_chirho,
                )
            });
        for argument_chirho in head_types_chirho {
            let actual_chirho = self.infer_type_kind_chirho(argument_chirho);
            if let Some(expected_chirho) = class_kind_chirho.take() {
                let term_chirho = self.interpret_kind_term_chirho(argument_chirho);
                class_kind_chirho = Some(self.consume_kind_argument_chirho(
                    expected_chirho,
                    &actual_chirho,
                    Some(&term_chirho),
                    argument_chirho.span_chirho(),
                    "associated instance head",
                ));
            }
        }
        // The context constrains the same variables as the instance head.
        // Infer those classifiers before quantifying them; only the associated
        // equations are forbidden to specialize a still-polymorphic head.
        for constraint_chirho in context_chirho {
            self.infer_constraint_kind_chirho(constraint_chirho);
        }
        self.rigid_ascription_names_chirho = outer_ascriptions_chirho;
        let bindings_chirho = self.env_chirho.capture_scope_chirho();
        let variables_chirho = bindings_chirho
            .iter()
            .flat_map(|(_, binding_chirho)| {
                self.subst_chirho
                    .apply_chirho(binding_chirho.body_chirho())
                    .free_vars_chirho()
            })
            .collect::<Vec<_>>();
        self.rigidify_kind_variables_chirho(variables_chirho);
        self.env_chirho.begin_scope_chirho();
        for (name_chirho, binding_chirho) in bindings_chirho {
            self.env_chirho
                .bind_entry_chirho(name_chirho, binding_chirho);
        }
        for equation_chirho in equations_chirho {
            let name_chirho = self.canonical_kind_name_chirho(&equation_chirho.family_name_chirho);
            let Some(super::KindBindingChirho::PolyChirho(scheme_chirho)) =
                self.env_chirho.lookup_binding_chirho(&name_chirho).cloned()
            else {
                // An absent imported kind is not a guessed family classifier.
                continue;
            };
            self.with_signature_kind_scope_chirho(|ctx_chirho| {
                ctx_chirho.check_family_equation_body_chirho(
                    &name_chirho,
                    &scheme_chirho,
                    &equation_chirho.lhs_types_chirho,
                    &equation_chirho.rhs_chirho,
                );
            });
        }
        if let Some((class_parameters_chirho, families_chirho)) = defaults_chirho {
            for family_chirho in families_chirho {
                let name_chirho = family_chirho.name_chirho.text_chirho();
                if family_chirho.defaults_chirho.is_empty()
                    || equations_chirho.iter().any(|equation_chirho| {
                        equation_chirho.family_name_chirho.text_chirho() == name_chirho
                    })
                {
                    continue;
                }
                let Some(super::KindBindingChirho::PolyChirho(scheme_chirho)) =
                    self.env_chirho.lookup_binding_chirho(name_chirho).cloned()
                else {
                    // These defaults come only from this module's checked class
                    // declarations, unlike an explicit equation for an imported
                    // family above. Missing local authority is a real error.
                    self.diagnostics_chirho.push_chirho(
                        DiagnosticChirho::error_with_code_chirho(
                            ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                            format!("associated default instance has no checked kind for `{name_chirho}`"),
                            span_chirho,
                        ),
                    );
                    continue;
                };
                self.with_signature_kind_scope_chirho(|ctx_chirho| {
                    let start_chirho = ctx_chirho.diagnostics_chirho.len_chirho();
                    let (mut expected_chirho, hidden_chirho) =
                        ctx_chirho.open_kind_scheme_parts_chirho(&scheme_chirho, false);
                    for parameter_chirho in &family_chirho.type_vars_chirho {
                        let argument_chirho = class_parameters_chirho
                            .iter()
                            .position(|class_parameter_chirho| {
                                class_parameter_chirho.text_chirho()
                                    == parameter_chirho.text_chirho()
                            })
                            .and_then(|index_chirho| head_types_chirho.get(index_chirho));
                        let (actual_chirho, term_chirho) =
                            if let Some(argument_chirho) = argument_chirho {
                                (
                                    ctx_chirho.infer_type_kind_chirho(argument_chirho),
                                    ctx_chirho.interpret_kind_term_chirho(argument_chirho),
                                )
                            } else {
                                // A family-only argument remains an equation binder,
                                // not an extra parameter of the enclosing instance.
                                let classifier_chirho = ctx_chirho.fresh_kind_chirho();
                                let identity_chirho = ctx_chirho.fresh_var_chirho();
                                ctx_chirho
                                    .kind_binder_classifiers_chirho
                                    .insert(identity_chirho, classifier_chirho.clone());
                                (classifier_chirho, KindChirho::VarChirho(identity_chirho))
                            };
                        expected_chirho = ctx_chirho.consume_kind_argument_chirho(
                            expected_chirho,
                            &actual_chirho,
                            Some(&term_chirho),
                            family_chirho.span_chirho,
                            "associated default instance argument",
                        );
                    }
                    if !ctx_chirho.diagnostics_chirho.diagnostics_chirho()[start_chirho..]
                        .iter()
                        .any(DiagnosticChirho::is_error_chirho)
                    {
                        ctx_chirho
                            .kind_associated_defaults_chirho
                            .insert((span_chirho, name_chirho.to_owned()), hidden_chirho);
                    }
                });
            }
        }
        self.env_chirho.end_scope_chirho();
        self.kind_var_cache_chirho = outer_names_chirho;
        if self.diagnostics_chirho.diagnostics_chirho()[diagnostic_start_chirho..]
            .iter()
            .any(DiagnosticChirho::is_error_chirho)
        {
            self.kind_applications_chirho.remove(&span_chirho);
        }
    }

    /// Number of arguments a kind takes before reaching its result, and whether
    /// that count is final. Only a variable in the RESULT position leaves the
    /// arity open (it may still be instantiated to another arrow); a variable
    /// in an argument position is just an un-annotated parameter — `class C a b`
    /// has arity 2 no matter what kinds `a` and `b` turn out to have.
    pub(super) fn kind_arity_chirho(kind_chirho: &KindChirho) -> (usize, bool) {
        let mut arity_chirho = 0;
        let mut cursor_chirho = kind_chirho;
        loop {
            match cursor_chirho {
                KindChirho::ArrowChirho(_, result_chirho) => {
                    arity_chirho += 1;
                    cursor_chirho = result_chirho;
                }
                KindChirho::VarChirho(_) => return (arity_chirho, false),
                _ => return (arity_chirho, true),
            }
        }
    }

    /// Report an instance head that applies its class to the wrong number of
    /// arguments (`class MonadReader a b` + `instance MonadReader Int`), which
    /// GHC rejects with "Expecting one more argument to ...".
    ///
    /// Deliberately narrow: the class must be declared in THIS module (an
    /// imported class may arrive through a placeholder interface whose kind we
    /// do not really know). The existing syntax-representability boundary is
    /// retained; quantified local kinds are instantiated for each instance.
    pub(super) fn check_instance_head_arity_chirho(
        &mut self,
        class_name_chirho: &str,
        head_types_chirho: &[TypeChirho],
        head_forms_representable_chirho: bool,
        span_chirho: SpanChirho,
    ) {
        let head_arg_count_chirho = head_types_chirho.len();
        // An incomplete class head cannot establish class arity, but every
        // represented argument still has its own contract. In particular an
        // explicit ascription must be checked even with FlexibleInstances or
        // an imported class. The local scope keeps unrelated instances apart.
        let argument_kinds_chirho = self.with_signature_kind_scope_chirho(|ctx_chirho| {
            head_types_chirho
                .iter()
                .map(|argument_chirho| ctx_chirho.infer_type_kind_chirho(argument_chirho))
                .collect::<Vec<_>>()
        });
        // Only classes declared in THIS module: an imported class may arrive
        // through a placeholder interface whose kind we do not really know.
        // The existence of a builtin or placeholder entry alone does not prove
        // an imported class's authoritative parameter contract.
        if !self
            .local_kind_decl_names_chirho
            .contains(class_name_chirho)
        {
            return;
        }
        let Some(class_binding_chirho) = self
            .env_chirho
            .lookup_binding_chirho(class_name_chirho)
            .cloned()
        else {
            return;
        };
        let resolved_kind_chirho = self.instantiate_binding_chirho(&class_binding_chirho);
        let (expected_chirho, spine_known_chirho) = Self::kind_arity_chirho(&resolved_kind_chirho);
        if !spine_known_chirho {
            return;
        }
        if expected_chirho == head_arg_count_chirho {
            if head_forms_representable_chirho {
                self.unify_chirho(
                    &resolved_kind_chirho,
                    &KindChirho::arrow_n_chirho(
                        argument_kinds_chirho,
                        KindChirho::ConstraintChirho,
                    ),
                    "instance head",
                    span_chirho,
                );
            }
            return;
        }
        // A head form our lowering drops can only make the head look SHORTER
        // than written, never longer — so over-application stays sound even
        // when some head arguments may be unrepresented.
        if !head_forms_representable_chirho && head_arg_count_chirho < expected_chirho {
            return;
        }
        let message_chirho = if head_arg_count_chirho < expected_chirho {
            format!(
                "expecting {} more argument{} to `{class_name_chirho}` in the instance head",
                expected_chirho - head_arg_count_chirho,
                if expected_chirho - head_arg_count_chirho == 1 {
                    ""
                } else {
                    "s"
                },
            )
        } else {
            format!(
                "`{class_name_chirho}` is applied to {head_arg_count_chirho} arguments, but it takes {expected_chirho}",
            )
        };
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                message_chirho,
                span_chirho,
            ));
    }
}
