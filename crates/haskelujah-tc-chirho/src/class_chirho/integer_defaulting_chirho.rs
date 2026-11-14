// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Module-end subset of GHC.Tc.Solver.Default.findDefaultableGroups/disambigGroup.
//! Only the first standard candidate, Integer, with a checked context-free Num instance.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use crate::TyConChirho;

#[cfg(test)]
#[path = "integer_defaulting_tests_chirho.rs"]
mod integer_defaulting_tests_chirho;

impl SolverChirho {
    /// The caller certifies Num's original name. Generalization has already finished;
    /// generalized metas and skolems are never candidates. No instance-count heuristic.
    pub(crate) fn default_integer_chirho(
        &mut self,
        num_chirho: NameIdChirho,
    ) -> Result<(), TcErrorChirho> {
        let mut candidates_chirho = Vec::new();
        let mut seen_chirho = HashSet::new();
        let mut blocked_chirho = HashSet::new();
        // One zonked view of all remaining constraints: a non-unary occurrence or
        // another class blocks its variable, even if Num Integer exists.
        for index_chirho in 0..self.dictionary_wanteds_chirho.len() {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            let wanted_chirho = self.dictionary_wanteds_chirho[index_chirho].clone();
            if self
                .evidence_term_chirho(wanted_chirho.evidence_chirho)?
                .is_some()
            {
                continue;
            }
            let predicate_chirho = self
                .arena_chirho
                .replace_chirho(wanted_chirho.predicate_chirho, HashMap::new())?;
            let (class_chirho, argument_chirho) = self.predicate_view_chirho(predicate_chirho)?;
            if class_chirho == num_chirho
                && self.arena_chirho.is_flexible_chirho(argument_chirho)?
            {
                if seen_chirho.insert(argument_chirho) {
                    self.arena_chirho.reserve_auxiliary_chirho()?;
                    candidates_chirho.push((
                        argument_chirho,
                        wanted_chirho.scope_chirho,
                        wanted_chirho.span_chirho,
                    ));
                }
            } else {
                for variable_chirho in self
                    .arena_chirho
                    .free_variables_chirho(&[argument_chirho])?
                {
                    self.arena_chirho.reserve_auxiliary_chirho()?;
                    blocked_chirho.insert(variable_chirho);
                }
            }
        }
        if candidates_chirho.is_empty() {
            return Ok(());
        }
        let integer_chirho = self.arena_chirho.con_chirho(TyConChirho::IntegerChirho)?;
        // Prove this candidate without filling a meta or manufacturing evidence.
        // A slice cannot prove failure; leave an unavailable candidate to the existing
        // partiality gate. Future default types require isolated solver trials.
        let Some((instance_chirho, arguments_chirho)) =
            self.match_instance_chirho(num_chirho, integer_chirho)?
        else {
            return Ok(());
        };
        let instance_chirho = self.instance_info_chirho(instance_chirho)?;
        if !arguments_chirho.is_empty()
            || !instance_chirho.binders_chirho.is_empty()
            || !instance_chirho.context_chirho.is_empty()
        {
            return Ok(());
        }
        let type_chirho = self.arena_chirho.con_chirho(TyConChirho::TypeChirho)?;
        for (variable_chirho, scope_chirho, span_chirho) in candidates_chirho {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            if blocked_chirho.contains(&variable_chirho)
                || self.arena_chirho.kind_chirho(variable_chirho)? != type_chirho
            {
                continue;
            }
            self.want_chirho(EqualityChirho {
                left_chirho: variable_chirho,
                right_chirho: integer_chirho,
                scope_chirho,
                span_chirho,
            });
        }
        self.solve_chirho()?;
        // Normal solving binds each dictionary at its emitting scope, including
        // captured constraints below signed bindings. No separate evidence path.
        self.solve_dictionaries_chirho()
    }
}
