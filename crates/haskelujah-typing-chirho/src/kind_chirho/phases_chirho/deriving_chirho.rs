// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GND consumes closed kinds inside the context that will check its instances.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;
use crate::deriving_chirho::{StockClassChirho, newtype_chirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;

impl KindInferCtxChirho {
    pub(super) fn elaborate_newtype_deriving_chirho(&mut self, module_chirho: &mut ModuleChirho) {
        let mut generated_chirho = Vec::new();
        for declaration_chirho in &module_chirho.decls_chirho {
            let DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                deriving_chirho,
                ..
            } = declaration_chirho
            else {
                continue;
            };
            for application_chirho in deriving_chirho {
                let Some((class_chirho, _)) = application_chirho.constructor_application_chirho()
                else {
                    self.deriving_error_chirho(
                        "newtype deriving class application is not represented",
                        application_chirho.span_chirho(),
                    );
                    continue;
                };
                if StockClassChirho::from_name_chirho(class_chirho.text_chirho()).is_some() {
                    continue;
                }
                let Some(representation_chirho) =
                    newtype_chirho::representation_chirho(constructor_chirho)
                else {
                    self.deriving_error_chirho(
                        "newtype deriving requires one represented constructor field",
                        application_chirho.span_chirho(),
                    );
                    continue;
                };
                let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
                self.env_chirho.begin_scope_chirho();
                let start_chirho = self.diagnostics_chirho.error_count_chirho();
                let instance_chirho = self.derive_newtype_application_chirho(
                    name_chirho,
                    type_vars_chirho,
                    representation_chirho,
                    application_chirho,
                );
                self.env_chirho.end_scope_chirho();
                self.kind_var_cache_chirho = outer_names_chirho;
                // Recovery never publishes a failed derived instance or its
                // downstream ordinal-keyed hidden-argument contract.
                if self.diagnostics_chirho.error_count_chirho() == start_chirho {
                    generated_chirho.extend(instance_chirho);
                }
            }
        }
        module_chirho.decls_chirho.extend(generated_chirho);
    }

    fn deriving_error_chirho(
        &mut self,
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) {
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                message_chirho,
                span_chirho,
            ));
    }

    fn derive_newtype_application_chirho(
        &mut self,
        name_chirho: &NameChirho,
        parameters_chirho: &[TyVarChirho],
        representation_chirho: &TypeChirho,
        application_chirho: &TypeChirho,
    ) -> Option<DeclChirho> {
        let span_chirho = application_chirho.span_chirho();
        let (class_chirho, written_chirho) = application_chirho.constructor_application_chirho()?;
        // Bound speculative kind matching per declaration; exhaustion is not
        // evidence that any particular target has the required kind.
        if parameters_chirho.len() > 128 {
            self.deriving_error_chirho(
                "newtype deriving exceeded its parameter work limit",
                span_chirho,
            );
            return None;
        }
        let Some(binding_chirho) = self
            .env_chirho
            .lookup_binding_chirho(name_chirho.text_chirho())
            .cloned()
        else {
            self.deriving_error_chirho(
                "newtype deriving has no checked declaration kind",
                span_chirho,
            );
            return None;
        };
        let (mut kind_chirho, hidden_chirho, names_chirho) = match &binding_chirho {
            KindBindingChirho::PolyChirho(scheme_chirho) => {
                // A derived instance USES the newtype's polymorphic kind; it
                // may specialize this occurrence without changing the declaration.
                let (kind_chirho, hidden_chirho) =
                    self.open_kind_scheme_parts_chirho(scheme_chirho, false);
                (
                    kind_chirho,
                    hidden_chirho,
                    scheme_chirho.source_names_chirho.clone(),
                )
            }
            KindBindingChirho::MonoChirho(kind_chirho) => (
                self.subst_chirho.apply_chirho(kind_chirho),
                Vec::new(),
                Vec::new(),
            ),
        };
        // Hidden declaration kind binders keep their identity in written
        // deriving arguments; visible binders below share that lexical map.
        for (source_name_chirho, argument_chirho) in names_chirho.into_iter().zip(hidden_chirho) {
            let Some(source_name_chirho) = source_name_chirho else {
                continue;
            };
            let alias_chirho = self.fresh_var_chirho();
            if let KindChirho::VarChirho(identity_chirho) = &argument_chirho
                && let Some(classifier_chirho) = self
                    .kind_binder_classifiers_chirho
                    .get(identity_chirho)
                    .cloned()
            {
                self.env_chirho
                    .bind_chirho(source_name_chirho.clone(), classifier_chirho);
            }
            self.kind_var_cache_chirho
                .insert(source_name_chirho, alias_chirho);
            self.subst_chirho
                .map_chirho
                .insert(alias_chirho, argument_chirho);
        }
        let visible_chirho = TyVarChirho::visible_binders_chirho(parameters_chirho);
        let mut candidates_chirho = vec![kind_chirho.clone()];
        for parameter_chirho in visible_chirho.iter() {
            let (classifier_chirho, result_chirho, dependent_chirho) =
                match self.subst_chirho.apply_chirho(&kind_chirho) {
                    KindChirho::ArrowChirho(classifier_chirho, result_chirho) => {
                        (*classifier_chirho, *result_chirho, false)
                    }
                    KindChirho::DependentChirho {
                        argument_chirho,
                        result_chirho,
                    } => (*argument_chirho, *result_chirho, true),
                    _ => {
                        self.deriving_error_chirho(
                            "newtype deriving has no checked parameter kind",
                            span_chirho,
                        );
                        return None;
                    }
                };
            let identity_chirho = self.fresh_var_chirho();
            self.kind_var_cache_chirho
                .insert(parameter_chirho.text_chirho().to_owned(), identity_chirho);
            self.kind_binder_classifiers_chirho
                .insert(identity_chirho, classifier_chirho.clone());
            self.env_chirho
                .bind_chirho(parameter_chirho.text_chirho().to_owned(), classifier_chirho);
            kind_chirho = if dependent_chirho {
                result_chirho.substitute_bound_chirho(&KindChirho::VarChirho(identity_chirho))
            } else {
                result_chirho
            };
            candidates_chirho.push(kind_chirho.clone());
        }

        let errors_chirho = self.diagnostics_chirho.error_count_chirho();
        let residual_chirho = self.infer_type_kind_chirho(application_chirho);
        if self.diagnostics_chirho.error_count_chirho() != errors_chirho {
            return None;
        }
        let residual_chirho = self.subst_chirho.apply_chirho(&residual_chirho);
        let expected_chirho = match residual_chirho {
            KindChirho::ArrowChirho(argument_chirho, result_chirho)
                if *result_chirho == KindChirho::ConstraintChirho =>
            {
                *argument_chirho
            }
            _ => {
                self.deriving_error_chirho(
                    format!(
                        "`{}` is not a unary constraint, as expected by a deriving clause",
                        class_chirho.text_chirho()
                    ),
                    span_chirho,
                );
                return None;
            }
        };
        // Drop as few trailing parameters as the checked class kind permits:
        // search the finite declaration telescope, most applied first. A kind
        // match is tried without committing; only the selected target commits.
        let mut selected_chirho = None;
        for (applied_chirho, kind_chirho) in candidates_chirho.iter().enumerate().rev() {
            let actual_chirho = self.subst_chirho.apply_chirho(kind_chirho);
            let expected_chirho = self.subst_chirho.apply_chirho(&expected_chirho);
            match self.unify_family_kinds_chirho(
                &expected_chirho,
                &actual_chirho,
                "newtype deriving target",
                span_chirho,
            ) {
                Ok(substitution_chirho) => {
                    selected_chirho = Some((applied_chirho, substitution_chirho));
                    break;
                }
                Err(error_chirho @ KindErrorChirho::ReductionLimitChirho { .. }) => {
                    self.commit_kind_unification_chirho(
                        Err(error_chirho),
                        "newtype deriving target",
                        span_chirho,
                    );
                    return None;
                }
                Err(_) => {}
            }
        }
        let Some((applied_chirho, substitution_chirho)) = selected_chirho else {
            self.deriving_error_chirho(
                "newtype deriving target does not have the class parameter's checked kind",
                span_chirho,
            );
            return None;
        };
        let reduced_chirho = match newtype_chirho::eta_reduce_chirho(
            representation_chirho,
            &visible_chirho[applied_chirho..],
        ) {
            Ok(reduced_chirho) => reduced_chirho,
            Err(message_chirho) => {
                self.deriving_error_chirho(message_chirho, span_chirho);
                return None;
            }
        };
        self.commit_kind_unification_chirho(
            Ok(substitution_chirho),
            "newtype deriving target",
            span_chirho,
        );
        let actual_representation_chirho = self.infer_type_kind_chirho(&reduced_chirho);
        self.unify_chirho(
            &expected_chirho,
            &actual_representation_chirho,
            "newtype deriving representation",
            span_chirho,
        );
        // Construct only the selected target, not a cloned AST per candidate.
        // It is generated syntax, not a second application at the deriving
        // class's source span. Its hidden evidence is published by declaration
        // ordinal in the ordinary instance pass, not this source-occurrence map.
        let mut target_name_chirho = name_chirho.clone();
        match &mut target_name_chirho {
            NameChirho::RawChirho(raw_chirho) => raw_chirho.span_chirho = SpanChirho::DUMMY_CHIRHO,
            NameChirho::ResolvedChirho(resolved_chirho) => {
                resolved_chirho.raw_chirho.span_chirho = SpanChirho::DUMMY_CHIRHO
            }
        }
        let mut target_chirho = TypeChirho::ConChirho(target_name_chirho);
        for parameter_chirho in &visible_chirho[..applied_chirho] {
            target_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(target_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(parameter_chirho.name_chirho.clone())),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
        }
        Some(newtype_chirho::instance_chirho(
            class_chirho,
            &written_chirho,
            target_chirho,
            reduced_chirho,
            span_chirho,
        ))
    }
}
