// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded, demand-directed superclass entailment (row 694, #31972).
//!
//! A given entails each superclass its class's templates instantiate at its own arguments,
//! transitively (`AskChirho Int m` gives `Monad m`). Under UndecidableSuperClasses a cycle can
//! change its arguments (`class C [a] => C a` gives `C [a]`, `C [[a]]`, ...), so no predicate
//! ever repeats and a closure that collects them all never ends. The search here ends at the
//! first match, expands only predicates whose records can still reach the wanted class, and
//! stops at a step bound and a type-size bound. A stopped search proves nothing either way: its
//! answer is `SuperclassSearchExhaustedChirho`, an unknown, never "not entailed".
//!
//! Class identity (row 692, C10 Q, gpt #33150): a compiler class and a program class with equal
//! text never discharge each other's constraints, directly or through a superclass. Each equality
//! needs agreeing proofs: two proofs name one identity, and a single compiler proof meets an
//! unproven side only while its compiler record is the active record of the shared spelling. A
//! compiler proof expands its own compiler record by identity, never the record that holds its
//! spelling. An unproven edge out of a compiler record stands only while its spelling is held by
//! a compiler record. The reachability walk reads the same records, so a displaced compiler class
//! and the program class holding its spelling are never merged by name. Unproven predicates keep
//! the spelling rule they had.
//! workflow: language-features-chirho/rigid-type-variables-chirho

use std::collections::HashSet;
use std::sync::Arc;

use haskelujah_ast_chirho::name_chirho::{
    CompilerClassChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
};

use super::{ClassDeclChirho, ClassEnvChirho, PredChirho};
use crate::ty_chirho::TyChirho;

/// How many predicates one search may expand.
const SUPERCLASS_SEARCH_STEPS_CHIRHO: usize = 256;

/// The largest argument, in type nodes, a derived predicate may carry and still be expanded.
const SUPERCLASS_TYPE_SIZE_CHIRHO: usize = 64;

/// A superclass search a bound stopped before it found the wanted predicate or ran out of
/// predicates to expand: whether the given entails it is unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SuperclassSearchExhaustedChirho;

impl ClassEnvChirho {
    /// Whether `given_chirho` entails `wanted_chirho` through superclasses: `Ok(true)` when a
    /// chain of templates reaches it, `Ok(false)` when nothing is left that could, and
    /// `Err(SuperclassSearchExhaustedChirho)` when a bound stopped the search first.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub fn superclass_entails_chirho(
        &self,
        given_chirho: &PredChirho,
        wanted_chirho: &PredChirho,
    ) -> Result<bool, SuperclassSearchExhaustedChirho> {
        if self.same_predicate_chirho(given_chirho, wanted_chirho) {
            return Ok(true);
        }
        let target_chirho = wanted_chirho.class_name_chirho.as_str();
        if !self.predicate_reaches_chirho(given_chirho, target_chirho) {
            return Ok(false);
        }
        // Keyed with the proof: equal text under two identities is two predicates here.
        let mut seen_chirho = HashSet::from([(
            given_chirho.clone(),
            given_chirho.class_identity_chirho().clone(),
        )]);
        let mut pending_chirho = vec![given_chirho.clone()];
        let mut steps_chirho = 0usize;
        let mut stopped_chirho = false;
        while let Some(current_chirho) = pending_chirho.pop() {
            if steps_chirho == SUPERCLASS_SEARCH_STEPS_CHIRHO {
                return Err(SuperclassSearchExhaustedChirho);
            }
            steps_chirho += 1;
            let Some(class_chirho) = self.record_for_chirho(
                &current_chirho.class_name_chirho,
                current_chirho.class_identity_chirho(),
            ) else {
                continue;
            };
            let Some(superclasses_chirho) = class_chirho.superclass_preds_at_chirho(
                &current_chirho.kind_tys_chirho,
                &current_chirho.all_tys_chirho(),
            ) else {
                stopped_chirho = true;
                continue;
            };
            let compiler_record_chirho =
                compiler_identity_chirho(class_chirho.class_identity_chirho()).is_some();
            for derived_chirho in superclasses_chirho {
                if compiler_record_chirho
                    && compiler_identity_chirho(derived_chirho.class_identity_chirho()).is_none()
                    && !self
                        .spelling_holds_compiler_record_chirho(&derived_chirho.class_name_chirho)
                {
                    continue;
                }
                if self.same_predicate_chirho(&derived_chirho, wanted_chirho) {
                    return Ok(true);
                }
                if !self.predicate_reaches_chirho(&derived_chirho, target_chirho) {
                    continue;
                }
                if derived_chirho
                    .semantic_tys_chirho()
                    .any(|ty_chirho| type_size_chirho(ty_chirho) > SUPERCLASS_TYPE_SIZE_CHIRHO)
                {
                    stopped_chirho = true;
                    continue;
                }
                let proof_chirho = derived_chirho.class_identity_chirho().clone();
                if seen_chirho.insert((derived_chirho.clone(), proof_chirho)) {
                    pending_chirho.push(derived_chirho);
                }
            }
        }
        if stopped_chirho {
            Err(SuperclassSearchExhaustedChirho)
        } else {
            Ok(false)
        }
    }

    /// The compiler record a predicate's proof names, when it is also the record active under
    /// the predicate's spelling. An unknown or source proof, or a displaced spelling, has none.
    /// Context reduction (`schemes_chirho`) and the search here share this one authority.
    /// workflow: language-features-chirho/fundep-generalization-chirho
    pub(crate) fn active_compiler_record_chirho(
        &self,
        predicate_chirho: &PredChirho,
    ) -> Option<&Arc<ClassDeclChirho>> {
        let identity_chirho = compiler_identity_chirho(predicate_chirho.class_identity_chirho())?;
        let producer_chirho = self.compiler_class_record_chirho(identity_chirho)?;
        let active_chirho = self
            .classes_chirho()
            .get_chirho(&predicate_chirho.class_name_chirho)?;
        Arc::ptr_eq(producer_chirho, active_chirho).then_some(producer_chirho)
    }

    /// Equal text and arguments, with agreeing class proofs: two proofs name one identity, a
    /// single compiler proof meets an unproven side only while its compiler record is the active
    /// record of the shared spelling, and two unproven sides keep the spelling rule.
    fn same_predicate_chirho(&self, left_chirho: &PredChirho, right_chirho: &PredChirho) -> bool {
        if left_chirho != right_chirho {
            return false;
        }
        match (
            left_chirho.class_identity_chirho(),
            right_chirho.class_identity_chirho(),
        ) {
            (
                PredicateClassProofChirho::ProvenChirho(left_identity_chirho),
                PredicateClassProofChirho::ProvenChirho(right_identity_chirho),
            ) => left_identity_chirho == right_identity_chirho,
            (
                PredicateClassProofChirho::ProvenChirho(
                    PredicateClassIdentityChirho::CompilerChirho(_),
                ),
                _,
            ) => self.active_compiler_record_chirho(left_chirho).is_some(),
            (
                _,
                PredicateClassProofChirho::ProvenChirho(
                    PredicateClassIdentityChirho::CompilerChirho(_),
                ),
            ) => self.active_compiler_record_chirho(right_chirho).is_some(),
            _ => true,
        }
    }

    /// The record a predicate of this spelling and proof expands: a compiler proof selects its own
    /// compiler record by identity, and with no unique compiler record nothing is expanded (a
    /// named conservative refusal). Any other proof reads the active record of its spelling.
    fn record_for_chirho(
        &self,
        spelling_chirho: &str,
        proof_chirho: &PredicateClassProofChirho,
    ) -> Option<&Arc<ClassDeclChirho>> {
        match compiler_identity_chirho(proof_chirho) {
            Some(identity_chirho) => self.compiler_class_record_chirho(identity_chirho),
            None => self.classes_chirho().get_chirho(spelling_chirho),
        }
    }

    /// Whether the record active under `spelling_chirho` is the compiler record its own proof
    /// selects: that spelling is held by the compiler, not displaced by a program class.
    fn spelling_holds_compiler_record_chirho(&self, spelling_chirho: &str) -> bool {
        let Some(active_chirho) = self.classes_chirho().get_chirho(spelling_chirho) else {
            return false;
        };
        let Some(identity_chirho) = compiler_identity_chirho(active_chirho.class_identity_chirho())
        else {
            return false;
        };
        self.compiler_class_record_chirho(identity_chirho)
            .is_some_and(|producer_chirho| Arc::ptr_eq(producer_chirho, active_chirho))
    }

    /// Whether `predicate_chirho`'s class is `target_chirho` or reaches it through superclass
    /// templates, walking the records the expansion itself reads. Each record is visited once,
    /// so a cycle ends the walk.
    fn predicate_reaches_chirho(&self, predicate_chirho: &PredChirho, target_chirho: &str) -> bool {
        let mut visited_chirho: HashSet<*const ClassDeclChirho> = HashSet::new();
        let mut pending_chirho = vec![(
            predicate_chirho.class_name_chirho.clone(),
            predicate_chirho.class_identity_chirho().clone(),
        )];
        while let Some((spelling_chirho, proof_chirho)) = pending_chirho.pop() {
            if spelling_chirho == target_chirho {
                return true;
            }
            let Some(record_chirho) = self.record_for_chirho(&spelling_chirho, &proof_chirho)
            else {
                continue;
            };
            if !visited_chirho.insert(Arc::as_ptr(record_chirho)) {
                continue;
            }
            for template_chirho in record_chirho.superclass_templates_chirho() {
                let proof_chirho = template_chirho.class_identity_chirho().clone();
                pending_chirho.push((template_chirho.class_name_chirho, proof_chirho));
            }
        }
        false
    }
}

/// The compiler class a proof names, if it names one.
fn compiler_identity_chirho(
    proof_chirho: &PredicateClassProofChirho,
) -> Option<CompilerClassChirho> {
    match proof_chirho {
        PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
            identity_chirho,
        )) => Some(*identity_chirho),
        _ => None,
    }
}

/// The number of nodes in `ty_chirho`.
fn type_size_chirho(ty_chirho: &TyChirho) -> usize {
    let mut pending_chirho = vec![ty_chirho];
    let mut count_chirho = 0usize;
    while let Some(ty_chirho) = pending_chirho.pop() {
        count_chirho += 1;
        if count_chirho > SUPERCLASS_TYPE_SIZE_CHIRHO {
            return count_chirho;
        }
        match ty_chirho {
            TyChirho::AppChirho(left_chirho, right_chirho)
            | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                pending_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()]);
            }
            TyChirho::ListChirho(element_chirho) => pending_chirho.push(element_chirho),
            TyChirho::TupleChirho(elements_chirho) => pending_chirho.extend(elements_chirho),
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                pending_chirho.extend(context_chirho);
                pending_chirho.push(body_chirho);
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                for record_chirho in vars_chirho.records_chirho() {
                    if let Some(kind_chirho) = &record_chirho.kind_chirho {
                        pending_chirho.push(kind_chirho.as_ref());
                    }
                }
                pending_chirho.push(body_chirho);
            }
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
        }
        if pending_chirho.len() > SUPERCLASS_TYPE_SIZE_CHIRHO {
            return SUPERCLASS_TYPE_SIZE_CHIRHO + 1;
        }
    }
    count_chirho
}

#[cfg(test)]
#[path = "superclass_search_tests_chirho.rs"]
mod tests_chirho;
