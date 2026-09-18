// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Associated defaults consume finalized family kinds in independent equation scopes.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    DeclChirho, DiagnosticChirho, ErrorCodeChirho, KIND_MISMATCH_CODE_CHIRHO, KindBindingChirho,
    KindChirho, KindInferCtxChirho, ModuleChirho, TypeChirho,
};
use std::collections::HashSet;

impl KindInferCtxChirho {
    pub(super) fn check_associated_default_kinds_chirho(&mut self, module_chirho: &ModuleChirho) {
        for declaration_chirho in &module_chirho.decls_chirho {
            let DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            else {
                continue;
            };
            for family_chirho in associated_tfs_chirho {
                if family_chirho.defaults_chirho.is_empty() {
                    continue;
                }
                let name_chirho = family_chirho.name_chirho.text_chirho();
                let Some(binding_chirho) =
                    self.env_chirho.lookup_binding_chirho(name_chirho).cloned()
                else {
                    self.diagnostics_chirho
                        .push_chirho(DiagnosticChirho::error_with_code_chirho(
                            ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                            format!(
                                "associated family default has no checked kind for `{name_chirho}`"
                            ),
                            family_chirho.span_chirho,
                        ));
                    continue;
                };
                for equation_chirho in &family_chirho.defaults_chirho {
                    self.with_default_kind_scope_chirho(&equation_chirho.lhs_types_chirho, |ctx_chirho| {
                        let diagnostics_start_chirho = ctx_chirho.diagnostics_chirho.len_chirho();
                        let (mut expected_chirho, hidden_chirho) = match &binding_chirho {
                            KindBindingChirho::PolyChirho(scheme_chirho) =>
                                ctx_chirho.open_kind_scheme_parts_chirho(scheme_chirho, false),
                            KindBindingChirho::MonoChirho(kind_chirho) =>
                                (ctx_chirho.subst_chirho.apply_chirho(kind_chirho), Vec::new()),
                        };
                        // Resolve written annotation names before any argument
                        // can specialize them. These are equation-local skolems,
                        // not the class head's same-spelled names.
                        for argument_chirho in &equation_chirho.lhs_types_chirho {
                            ctx_chirho.rigid_default_annotations_chirho(argument_chirho);
                        }
                        let mut equation_terms_chirho = Vec::new();
                        for argument_chirho in &equation_chirho.lhs_types_chirho {
                            let actual_chirho = ctx_chirho.infer_type_kind_chirho(argument_chirho);
                            let term_chirho = ctx_chirho.interpret_kind_term_chirho(argument_chirho);
                            equation_terms_chirho.extend(term_chirho.free_vars_chirho());
                            expected_chirho = ctx_chirho.consume_kind_argument_chirho(
                                expected_chirho, &actual_chirho, Some(&term_chirho),
                                argument_chirho.span_chirho(), "associated family default argument",
                            );
                        }
                        // A default is valid for every LHS variable, including a
                        // visible dependent kind argument. Its RHS may not solve
                        // that universally quantified term to Bool or Type.
                        ctx_chirho.rigidify_kind_variables_chirho(equation_terms_chirho);
                        let actual_chirho = ctx_chirho.infer_type_kind_chirho(&equation_chirho.rhs_chirho);
                        ctx_chirho.unify_chirho(
                            &expected_chirho, &actual_chirho, "associated family default",
                            equation_chirho.span_chirho,
                        );
                        // Recovery may continue checking the module, but this
                        // failed occurrence is not checked elaboration evidence.
                        // Inspect only this equation's new diagnostics, never
                        // rescan the module's growing diagnostic history.
                        if ctx_chirho.diagnostics_chirho.diagnostics_chirho()[diagnostics_start_chirho..]
                            .iter().any(DiagnosticChirho::is_error_chirho)
                        {
                            return;
                        }
                        // The distinct-variable rule also applies to invisible
                        // kind arguments. Do not let an RHS such as Maybe a turn
                        // a quantified kind into Type, or merge two kind slots.
                        let mut identities_chirho = HashSet::new();
                        for (index_chirho, argument_chirho) in hidden_chirho.iter().enumerate() {
                            let distinct_chirho = match ctx_chirho.subst_chirho.apply_chirho(argument_chirho) {
                                KindChirho::VarChirho(identity_chirho)
                                | KindChirho::RigidChirho(identity_chirho) =>
                                    identities_chirho.insert(identity_chirho),
                                _ => false,
                            };
                            if !distinct_chirho {
                                ctx_chirho.diagnostics_chirho.push_chirho(DiagnosticChirho::error_with_code_chirho(
                                    ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                                    format!("associated family default requires distinct variable kind arguments: hidden argument {} is non-variable or repeats an earlier argument", index_chirho + 1),
                                    equation_chirho.span_chirho,
                                ));
                                return;
                            }
                        }
                        ctx_chirho.record_kind_application_chirho(
                            name_chirho, super::elaboration_chirho::KindHeadNamespaceChirho::TypeChirho,
                            &binding_chirho, hidden_chirho, equation_chirho.span_chirho,
                        );
                    });
                }
            }
        }
    }

    /// Unlike a class method, a default owns an independent equation scope.
    /// Swap the local identity cache and hide its outer names plus LHS binders;
    /// do not rely on a previous declaration loop having cleared the cache.
    /// Work visits local names only, not the growing module environment.
    fn with_default_kind_scope_chirho<ResultChirho>(
        &mut self,
        arguments_chirho: &[TypeChirho],
        body_chirho: impl FnOnce(&mut Self) -> ResultChirho,
    ) -> ResultChirho {
        let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
        self.env_chirho.begin_scope_chirho();
        for name_chirho in outer_names_chirho.keys() {
            self.env_chirho.hide_chirho(name_chirho);
        }
        for argument_chirho in arguments_chirho {
            if let TypeChirho::VarChirho(name_chirho) = argument_chirho.unannotated_chirho() {
                self.env_chirho.hide_chirho(name_chirho.text_chirho());
            }
        }
        let result_chirho = body_chirho(self);
        self.env_chirho.end_scope_chirho();
        self.kind_var_cache_chirho = outer_names_chirho;
        result_chirho
    }

    fn rigid_default_annotations_chirho(&mut self, argument_chirho: &TypeChirho) {
        match argument_chirho {
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.rigid_default_annotations_chirho(inner_chirho)
            }
            TypeChirho::KindAnnotChirho {
                type_chirho,
                kind_chirho,
                ..
            } => {
                let (_, written_chirho, _) = self.elaborate_inline_kind_chirho(kind_chirho);
                self.rigidify_kind_variables_chirho(written_chirho);
                self.rigid_default_annotations_chirho(type_chirho);
            }
            // validity_chirho/classes_chirho::check_class_chirho requires
            // unannotated_chirho() to be Var. That helper peels precisely
            // Paren/KindAnnot, so no other admitted pattern hides an annotation.
            _ => {}
        }
    }
}
