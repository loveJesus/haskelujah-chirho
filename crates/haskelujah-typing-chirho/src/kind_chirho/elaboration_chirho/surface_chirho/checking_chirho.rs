// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The final kind comparison belongs to the same lexical checking operation
//! as inference. A closed temporary forall scope does not turn its binder into
//! an instantiation meta, while unresolved kind metas must remain solvable.

use super::*;
use std::collections::HashSet;

#[cfg(test)]
#[path = "checking_tests_chirho.rs"]
mod tests_chirho;

impl TypeElaboratorChirho<'_> {
    pub(in crate::kind_chirho::elaboration_chirho) fn at_kind_level_chirho<ResultChirho>(
        &mut self,
        action_chirho: impl FnOnce(&mut Self) -> Result<ResultChirho, ElaborationErrorChirho>,
    ) -> Result<ResultChirho, ElaborationErrorChirho> {
        let previous_chirho = std::mem::replace(&mut self.checking_kind_chirho, true);
        let result_chirho = action_chirho(self);
        self.checking_kind_chirho = previous_chirho;
        result_chirho
    }

    pub(in crate::kind_chirho::elaboration_chirho) fn infer_kind_syntax_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        self.at_kind_level_chirho(|this_chirho| {
            this_chirho.infer_type_at_kind_chirho(
                syntax_chirho,
                &ElaborationStateChirho::type_universe_chirho(),
            )
        })
    }

    /// Actual leading quantifiers authorize rigidity even when a legacy record
    /// lacks the binding tag. Dependencies require a positive Universal record;
    /// a kind annotation or source spelling alone never freezes a fresh meta.
    fn checking_universals_chirho(
        &mut self,
        result_chirho: &ElaboratedTypeChirho,
        span_chirho: SpanChirho,
    ) -> Result<Vec<TyVarChirho>, ElaborationErrorChirho> {
        let mut variables_chirho = Vec::new();
        let mut pending_chirho = Vec::new();
        let mut seen_chirho = HashSet::new();
        let mut prefix_chirho = &result_chirho.term_chirho;
        let mut remaining_chirho = 128;
        loop {
            self.state_chirho.charge_chirho(1, 0, span_chirho)?;
            match prefix_chirho {
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
                        vars_chirho.len().saturating_mul(2),
                        span_chirho,
                    )?;
                    for record_chirho in vars_chirho.records_chirho() {
                        variables_chirho.push(record_chirho.variable_chirho);
                        pending_chirho.push(record_chirho.variable_chirho);
                        if let Some(kind_chirho) = &record_chirho.kind_chirho {
                            self.push_checking_dependencies_chirho(
                                kind_chirho,
                                &mut pending_chirho,
                                span_chirho,
                            )?;
                        }
                    }
                    prefix_chirho = body_chirho;
                }
                TyChirho::QualChirho { body_chirho, .. } => prefix_chirho = body_chirho,
                _ => break,
            }
            remaining_chirho -= 1;
            if remaining_chirho == 0 {
                return Err(self.state_chirho.resource_failure_chirho(
                    "kind checking prefix depth bound exhausted",
                    span_chirho,
                ));
            }
        }
        self.push_checking_dependencies_chirho(
            &result_chirho.kind_chirho,
            &mut pending_chirho,
            span_chirho,
        )?;
        while let Some(variable_chirho) = pending_chirho.pop() {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            if !seen_chirho.insert(variable_chirho) {
                continue;
            }
            if self.state_chirho.variable_binding_chirho(variable_chirho)
                == ElaborationVariableBindingChirho::UniversalChirho
            {
                self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                variables_chirho.push(variable_chirho);
            }
            let kind_chirho = self
                .state_chirho
                .variable_kinds_chirho
                .get(&variable_chirho)
                .and_then(|record_chirho| record_chirho.kind_chirho.clone());
            if let Some(kind_chirho) = kind_chirho {
                self.push_checking_dependencies_chirho(
                    &kind_chirho,
                    &mut pending_chirho,
                    span_chirho,
                )?;
            }
        }
        Ok(variables_chirho)
    }

    fn push_checking_dependencies_chirho(
        &mut self,
        kind_chirho: &TyChirho,
        pending_chirho: &mut Vec<TyVarChirho>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let kind_chirho = self.state_chirho.resolve_chirho(kind_chirho, span_chirho)?;
        let dependencies_chirho = kind_chirho.free_vars_chirho();
        self.state_chirho.charge_chirho(
            dependencies_chirho.len(),
            dependencies_chirho.len(),
            span_chirho,
        )?;
        pending_chirho.extend(dependencies_chirho);
        Ok(())
    }

    pub(super) fn check_inferred_kind_chirho(
        &mut self,
        syntax_chirho: &mut TypeChirho,
        mut result_chirho: ElaboratedTypeChirho,
        expected_chirho: &TyChirho,
    ) -> Result<ElaboratedTypeChirho, ElaborationErrorChirho> {
        let span_chirho = syntax_chirho.span_chirho();
        let variables_chirho = self.checking_universals_chirho(&result_chirho, span_chirho)?;
        let rigid_chirho = self
            .state_chirho
            .open_rigid_scope_chirho(&variables_chirho, span_chirho)?;
        let checked_chirho = (|| {
            self.instantiate_invisible_kind_chirho(syntax_chirho, &mut result_chirho, span_chirho)?;
            // Lexical GADT images and equality evidence normalize here before
            // the State rigid check; binding status itself is not global rigidity.
            self.unify_kinds_chirho(&result_chirho.kind_chirho, expected_chirho, span_chirho)?;
            Ok(result_chirho)
        })();
        self.state_chirho.close_rigid_scope_chirho(rigid_chirho);
        checked_chirho
    }
}
