// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Compose result-kind tails and reconcile complete data/newtype signatures.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    DataKindSigChirho, DiagnosticChirho, ErrorCodeChirho, KIND_MISMATCH_CODE_CHIRHO,
    KindBindingChirho, KindChirho, KindInferCtxChirho, KindSchemeChirho, SpanChirho, TyVarChirho,
    TypeChirho,
};
use std::collections::HashSet;

impl KindInferCtxChirho {
    pub(super) fn infer_data_decl_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        kind_sig_chirho: Option<&DataKindSigChirho>,
        span_chirho: SpanChirho,
    ) {
        let standalone_chirho = kind_sig_chirho.and_then(DataKindSigChirho::standalone_chirho);
        let result_chirho = kind_sig_chirho.and_then(DataKindSigChirho::result_chirho);
        // Ordinary unification permits Type/Constraint compatibility, so retain
        // GHC-55233 explicitly on either written contract (one diagnostic).
        if let Some(invalid_chirho) = standalone_chirho
            .into_iter()
            .chain(result_chirho)
            .find(|sig_chirho| self.declared_return_kind_is_constraint_chirho(sig_chirho))
        {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                    format!("data type `{name_chirho}` has non-`*` return kind `Constraint`"),
                    invalid_chirho.span_chirho(),
                ));
        }

        // Standalone binders (including implicitly quantified free kind names)
        // do not scope over the declaration. Swap only the name cache, O(1),
        // retaining fresh ids and substitutions; never clone the growing env.
        let complete_scheme_chirho = standalone_chirho.map(|signature_chirho| {
            let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
            let kind_chirho = self.type_to_kind_chirho(signature_chirho);
            self.kind_var_cache_chirho = outer_names_chirho;
            KindSchemeChirho::generalize_chirho(self.subst_chirho.apply_chirho(&kind_chirho))
        });

        let mut complete_tail_chirho = complete_scheme_chirho
            .as_ref()
            .map(|scheme_chirho| self.open_kind_scheme_chirho(scheme_chirho, true));
        let mut parameters_chirho = Vec::with_capacity(type_vars_chirho.len());
        let mut written_variables_chirho = Vec::new();
        for variable_chirho in type_vars_chirho {
            let kind_chirho = variable_chirho
                .kind_annotation_chirho
                .as_ref()
                .map(|annotation_chirho| self.ast_kind_to_kind_ctx_chirho(annotation_chirho))
                .unwrap_or_else(|| self.fresh_kind_chirho());
            if variable_chirho.kind_annotation_chirho.is_some() {
                written_variables_chirho.extend(kind_chirho.free_vars_chirho());
            }
            // A head binder is a term as well as a classifier. Later annotations
            // may refer to it; its own annotation is outside its lexical scope.
            let identity_chirho = self.fresh_var_chirho();
            self.kind_var_cache_chirho
                .insert(variable_chirho.text_chirho().to_owned(), identity_chirho);
            self.env_chirho.bind_chirho(
                variable_chirho.text_chirho().to_owned(),
                kind_chirho.clone(),
            );
            if variable_chirho.is_visible_chirho() {
                if let Some(tail_chirho) = complete_tail_chirho.take() {
                    self.rigidify_kind_variables_chirho([identity_chirho]);
                    let binder_term_chirho = self
                        .subst_chirho
                        .apply_chirho(&KindChirho::VarChirho(identity_chirho));
                    complete_tail_chirho = Some(self.consume_complete_head_binder_chirho(
                        tail_chirho,
                        &kind_chirho,
                        &binder_term_chirho,
                        span_chirho,
                    ));
                }
                parameters_chirho.push((identity_chirho, kind_chirho));
            }
        }
        // Head annotations and the inline tail share identities and remain in
        // scope while the caller checks constructor fields, as ordinary heads do.
        let head_names_chirho: HashSet<_> = self.kind_var_cache_chirho.keys().cloned().collect();
        let (result_kind_chirho, tail_variables_chirho, quantified_chirho) = result_chirho
            .map(|tail_chirho| self.elaborate_inline_kind_chirho(tail_chirho))
            .unwrap_or_else(|| {
                let result_chirho = if complete_scheme_chirho.is_some() {
                    super::runtime_chirho::runtime_type_chirho(self.fresh_kind_chirho())
                } else {
                    KindChirho::StarChirho
                };
                (result_chirho, Vec::new(), Vec::new())
            });
        written_variables_chirho.extend(tail_variables_chirho);
        // A top-level :: may use head-bound kind names or explicit forall
        // binders. New implicitly introduced names make that legacy CUSK incomplete.
        let tail_bound_chirho = self
            .kind_var_cache_chirho
            .keys()
            .all(|name_chirho| head_names_chirho.contains(name_chirho));
        let cusk_chirho = self.cusks_enabled_chirho
            && tail_bound_chirho
            && type_vars_chirho
                .iter()
                .all(|variable_chirho| variable_chirho.kind_annotation_chirho.is_some());
        let binding_chirho = if let Some(complete_scheme_chirho) = complete_scheme_chirho {
            self.unify_chirho(
                &complete_tail_chirho.expect("a complete signature has a remaining kind"),
                &result_kind_chirho,
                "data declaration signature",
                span_chirho,
            );
            KindBindingChirho::PolyChirho(complete_scheme_chirho)
        } else {
            let mut head_kind_chirho = result_kind_chirho;
            for (identity_chirho, argument_chirho) in parameters_chirho.into_iter().rev() {
                head_kind_chirho = if head_kind_chirho
                    .free_vars_chirho()
                    .contains(&identity_chirho)
                {
                    KindChirho::DependentChirho {
                        argument_chirho: Box::new(argument_chirho),
                        result_chirho: Box::new(
                            head_kind_chirho.abstract_variable_chirho(identity_chirho),
                        ),
                    }
                } else {
                    KindChirho::arrow_chirho(argument_chirho, head_kind_chirho)
                };
            }
            // An incomplete recursive group may equate written variables from
            // different declarations, but may not specialize them to Type or an
            // arrow. Check that contract after solving the whole inference SCC.
            // Complete contracts are checked rigidly from the outset.
            if cusk_chirho && self.poly_kinds_enabled_chirho {
                self.rigidify_kind_variables_chirho(written_variables_chirho);
            } else {
                self.pending_written_kinds_chirho.extend(
                    written_variables_chirho
                        .into_iter()
                        .map(|variable_chirho| (variable_chirho, span_chirho)),
                );
            }
            let head_kind_chirho = self.subst_chirho.apply_chirho(&head_kind_chirho);
            // Legacy CUSKs only break inference cycles under PolyKinds. With
            // NoPolyKinds even a zero-binder declaration must contribute its
            // body constraints before another group member is defaulted.
            if cusk_chirho && self.poly_kinds_enabled_chirho {
                KindBindingChirho::PolyChirho(KindSchemeChirho::generalize_chirho(head_kind_chirho))
            } else if !quantified_chirho.is_empty() {
                KindBindingChirho::PolyChirho(KindSchemeChirho {
                    quantified_chirho,
                    body_chirho: head_kind_chirho,
                })
            } else {
                KindBindingChirho::MonoChirho(head_kind_chirho)
            }
        };
        if let Some(existing_chirho) = self.env_chirho.lookup_binding_chirho(name_chirho).cloned() {
            let existing_chirho = self.instantiate_binding_chirho(&existing_chirho);
            let kind_chirho = self.instantiate_binding_chirho(&binding_chirho);
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "data declaration",
                span_chirho,
            );
        }
        self.env_chirho
            .bind_entry_chirho(name_chirho.to_owned(), binding_chirho);
    }

    /// Apply a complete kind telescope to this declaration's rigid head term.
    /// A dependent binder substitutes its term into the remaining contract;
    /// ordinary arrows merely consume an argument. Neither erases dependency.
    fn consume_complete_head_binder_chirho(
        &mut self,
        tail_chirho: KindChirho,
        classifier_chirho: &KindChirho,
        term_chirho: &KindChirho,
        span_chirho: SpanChirho,
    ) -> KindChirho {
        match self.subst_chirho.apply_chirho(&tail_chirho) {
            KindChirho::DependentChirho {
                argument_chirho,
                result_chirho,
            } => {
                self.unify_chirho(
                    &argument_chirho,
                    classifier_chirho,
                    "data declaration signature",
                    span_chirho,
                );
                result_chirho.substitute_bound_chirho(term_chirho)
            }
            KindChirho::ArrowChirho(argument_chirho, result_chirho) => {
                self.unify_chirho(
                    &argument_chirho,
                    classifier_chirho,
                    "data declaration signature",
                    span_chirho,
                );
                *result_chirho
            }
            other_chirho => {
                let result_chirho = self.fresh_kind_chirho();
                self.unify_chirho(
                    &other_chirho,
                    &KindChirho::arrow_chirho(classifier_chirho.clone(), result_chirho.clone()),
                    "data declaration signature",
                    span_chirho,
                );
                result_chirho
            }
        }
    }

    /// GHC-55233: the return kind of data/newtype cannot be Constraint. This
    /// differs from a binder's kind; the parser now represents that distinction.
    /// A locally declared Constraint retains the previous shadowing boundary.
    fn declared_return_kind_is_constraint_chirho(&self, sig_chirho: &TypeChirho) -> bool {
        if self.local_kind_decl_names_chirho.contains("Constraint") {
            return false;
        }
        let mut tail_chirho = sig_chirho;
        loop {
            match tail_chirho {
                TypeChirho::FunChirho { result_chirho, .. } => tail_chirho = result_chirho,
                TypeChirho::ParenChirho { inner_chirho, .. } => tail_chirho = inner_chirho,
                TypeChirho::ForallChirho { body_chirho, .. }
                | TypeChirho::RequiredForallChirho { body_chirho, .. } => tail_chirho = body_chirho,
                _ => break,
            }
        }
        matches!(tail_chirho, TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Constraint")
    }
}
