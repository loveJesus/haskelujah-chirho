// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Check an expression's annotation universally, then instantiate its outer
//! scheme at ordinary uses. Nested quantifiers remain in the checked body.

use super::InferCtxChirho;
use super::equality_normalization_chirho::{EqualityBudgetChirho, copy_term_with_budget_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, SchemePredPartsChirho, TyChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

mod ambiguity_chirho;
mod body_types_chirho;
pub(super) use body_types_chirho::ValueParameterScopeChirho;
mod escape_chirho;
#[cfg(test)]
mod tests_chirho;

impl InferCtxChirho {
    /// Check a published expected sigma with its own rigid variables and all
    /// written givens. Return the external sigma only after the escape check;
    /// an ordinary unification variable never receives a polymorphic image.
    /// The caller lends its existing allowance, including to its body check.
    /// workflow: language-features-chirho/rank-n-visible-type-application-chirho
    pub(super) fn check_expected_sigma_chirho(
        &mut self,
        expected_chirho: &TyChirho,
        span_chirho: SpanChirho,
        budget_chirho: &mut EqualityBudgetChirho,
        check_chirho: impl FnOnce(
            &mut Self,
            &TyChirho,
            &mut EqualityBudgetChirho,
        ) -> Option<(SubstChirho, TyChirho)>,
    ) -> Option<(SubstChirho, TyChirho)> {
        let mut predicates_chirho = Vec::new();
        let mut equalities_chirho = Vec::new();
        let (vars_chirho, body_chirho) = self.split_typed_scheme_prefix_chirho(
            copy_term_with_budget_chirho(expected_chirho, budget_chirho)?,
            &mut predicates_chirho,
            &mut equalities_chirho,
        );
        predicates_chirho.extend(equalities_chirho.into_iter().map(
            |(left_chirho, right_chirho)| {
                SchemePredChirho::legacy_chirho(SchemePredPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "~".to_owned(),
                    ty_chirho: left_chirho,
                    extra_tys_chirho: vec![right_chirho],
                })
            },
        ));
        let (checked_chirho, public_chirho) = self.with_rigid_signature_chirho(
            &SchemeChirho {
                vars_chirho,
                preds_chirho: predicates_chirho,
                ty_chirho: body_chirho,
            },
            span_chirho,
            |context_chirho, opened_chirho| {
                check_chirho(context_chirho, opened_chirho, budget_chirho)
            },
        );
        let (substitution_chirho, _) = checked_chirho?;
        let substitution_chirho =
            self.publish_signature_solution_chirho(substitution_chirho, public_chirho);
        let checked_chirho = substitution_chirho.apply_ty_chirho(expected_chirho);
        Some((substitution_chirho, checked_chirho))
    }

    /// The producer returns the checked sigma, so visible type application can
    /// consume its quantifiers before ordinary instantiation erases the prefix.
    pub(super) fn check_expression_annotation_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        annotation_chirho: &TypeChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, SchemeChirho) {
        let checked_annotation_chirho = self.check_body_type_chirho(annotation_chirho, true);
        if self.has_non_deferrable_failure_chirho() {
            return (
                SubstChirho::empty_chirho(),
                SchemeChirho::mono_chirho(Self::terminal_commit_type_chirho()),
            );
        }
        let annotation_chirho = checked_annotation_chirho
            .as_ref()
            .map_or(annotation_chirho, |checked_chirho| {
                &checked_chirho.syntax_chirho
            });
        self.check_source_signature_ambiguity_chirho(annotation_chirho);
        let (scheme_chirho, variables_chirho) =
            self.ast_type_to_scheme_with_var_map_chirho(annotation_chirho);
        let saved_scope_chirho = self.scoped_tyvars_chirho.clone();
        self.scope_signature_for_body_chirho(annotation_chirho, &variables_chirho);
        let ((substitution_chirho, _), public_chirho) = self.with_rigid_signature_chirho(
            &scheme_chirho,
            span_chirho,
            |context_chirho, expected_chirho| {
                context_chirho.infer_expr_against_expected_chirho(
                    expression_chirho,
                    expected_chirho,
                    span_chirho,
                )
            },
        );
        self.scoped_tyvars_chirho = saved_scope_chirho;
        let substitution_chirho =
            self.publish_signature_solution_chirho(substitution_chirho, public_chirho);
        let checked_chirho = substitution_chirho.apply_scheme_chirho(&scheme_chirho);
        (substitution_chirho, checked_chirho)
    }

    /// Each expected sigma checks escape before restoring its own rigid names.
    /// Nested checking cannot hide a committed outer-meta solution by dropping
    /// its temporary skolem map before its parent's scope check.
    pub(super) fn with_rigid_signature_chirho<ResultChirho>(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
        check_chirho: impl FnOnce(&mut Self, &TyChirho) -> ResultChirho,
    ) -> (ResultChirho, SubstChirho) {
        self.with_rigid_signature_at_parameter_entry_chirho(
            scheme_chirho,
            span_chirho,
            None,
            |context_chirho, expected_chirho, _entry_chirho| {
                check_chirho(context_chirho, expected_chirho)
            },
        )
    }

    /// A mixed function spine uses the same actual rigid images and written
    /// predicates, while its fixedness view excludes earlier own patterns.
    pub(super) fn with_rigid_signature_at_parameter_entry_chirho<ResultChirho>(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
        entry_chirho: Option<&ValueParameterScopeChirho>,
        check_chirho: impl FnOnce(
            &mut Self,
            &TyChirho,
            Option<&ValueParameterScopeChirho>,
        ) -> ResultChirho,
    ) -> (ResultChirho, SubstChirho) {
        let saved_skolems_chirho = self.skolem_bindings_chirho.clone();
        let captured_chirho = if scheme_chirho.vars_chirho.iter().any(|variable_chirho| {
            saved_skolems_chirho
                .lookup_chirho(variable_chirho)
                .is_none()
        }) {
            Some(self.capture_scope_metas_chirho(Some(scheme_chirho), &[], span_chirho))
        } else {
            None
        };
        if captured_chirho.as_ref().is_some_and(Option::is_none)
            || self.has_non_deferrable_failure_chirho()
        {
            self.type_normalization_exhausted_chirho.set(true);
            let result_chirho = check_chirho(self, &Self::terminal_commit_type_chirho(), None);
            return (result_chirho, SubstChirho::empty_chirho());
        }
        let (expected_chirho, givens_chirho) = self.skolemize_scheme_parts_chirho(scheme_chirho);
        let parameter_entry_chirho = entry_chirho.and_then(|entry_chirho| {
            self.extend_value_parameter_scope_chirho(entry_chirho, &givens_chirho, span_chirho)
        });
        let local_skolems_chirho: std::collections::HashSet<String> = scheme_chirho
            .vars_chirho
            .iter()
            .filter_map(|variable_chirho| {
                if saved_skolems_chirho
                    .lookup_chirho(variable_chirho)
                    .is_some()
                {
                    return None;
                }
                match self.skolem_bindings_chirho.lookup_chirho(variable_chirho) {
                    Some(TyChirho::ForallVarChirho(name_chirho)) => Some(name_chirho.clone()),
                    _ => None,
                }
            })
            .collect();

        let mut public_chirho = SubstChirho::empty_chirho();
        let result_chirho = self.with_signature_givens_and_finish_chirho(
            givens_chirho,
            &expected_chirho,
            |context_chirho| {
                check_chirho(
                    context_chirho,
                    &expected_chirho,
                    parameter_entry_chirho.as_ref(),
                )
            },
            |context_chirho| {
                if let Some(captured_chirho) = captured_chirho {
                    public_chirho = context_chirho.check_annotation_escape_chirho(
                        captured_chirho,
                        &local_skolems_chirho,
                        span_chirho,
                    );
                }
            },
        );
        self.skolem_bindings_chirho = saved_skolems_chirho;
        (result_chirho, public_chirho)
    }

    /// A safe public image replaces a scope-local image; it is a domain update
    /// as well as substitution composition, just like pattern implication exit.
    pub(super) fn publish_signature_solution_chirho(
        &mut self,
        checked_chirho: SubstChirho,
        public_chirho: SubstChirho,
    ) -> SubstChirho {
        if self.has_non_deferrable_failure_chirho() {
            return SubstChirho::empty_chirho();
        }
        let mut combined_chirho = public_chirho.compose_chirho(&checked_chirho);
        for (variable_chirho, image_chirho) in public_chirho.iter_chirho() {
            combined_chirho.insert_chirho(*variable_chirho, image_chirho.clone());
        }
        combined_chirho
    }
}
