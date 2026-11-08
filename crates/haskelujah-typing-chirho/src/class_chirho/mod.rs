// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Typeclass infrastructure
//!
//! Provides the data structures for Haskell-style typeclasses:
//!
//! - `PredChirho` — a class constraint like `Eq a` or `Ord a`
//! - `QualTyChirho` — a qualified type `(Eq a, Show a) => a -> String`
//! - `ClassDeclChirho` — a typeclass declaration with superclasses and methods
//! - `InstDeclChirho` — a typeclass instance with method implementations
//! - `ClassEnvChirho` — the environment of known classes and instances,
//!   supporting instance resolution / context reduction

use std::collections::HashMap;

use haskelujah_span_chirho::SpanChirho;

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use crate::unify_chirho::unify_chirho;

mod arena_chirho;
mod definitions_chirho;
mod instance_matching_chirho;
mod registration_budget_chirho;
mod standard_chirho;
mod superclass_search_chirho;
mod tuple_instances_chirho;
mod views_chirho;
pub use arena_chirho::{
    ClassEnvChirho, ClassRecordIdChirho, ClassRegistryErrorChirho, InstanceRecordIdChirho,
};
pub use definitions_chirho::{
    ClassDeclChirho, ClassDeclPartsChirho, InstDeclChirho, InstDeclPartsChirho, PredChirho,
    PredPartsChirho, QualTyChirho,
};
use instance_matching_chirho::match_consistently_chirho;
pub use superclass_search_chirho::SuperclassSearchExhaustedChirho;
pub use tuple_instances_chirho::MAX_TUPLE_INSTANCE_ARITY_CHIRHO;
pub use views_chirho::{
    ClassNamesViewChirho, InstanceListViewChirho, InstanceNamesViewChirho, InstanceRecordsChirho,
};

impl ClassEnvChirho {
    /// Check if a class exists.
    pub fn has_class_chirho(&self, name_chirho: &str) -> bool {
        self.classes_chirho().contains_key_chirho(name_chirho)
    }

    /// Get the superclasses of a class.
    pub fn superclasses_chirho(&self, class_name_chirho: &str) -> Vec<String> {
        self.classes_chirho()
            .get_chirho(class_name_chirho)
            .map(|c_chirho| c_chirho.supers_chirho.clone())
            .unwrap_or_default()
    }

    /// Attempt to resolve a predicate: find an instance that matches and
    /// return the required context (sub-goals).
    ///
    /// For example, resolving `Eq [Int]` against `instance Eq a => Eq [a]`
    /// would match with `a = Int` and return `[Eq Int]` as a sub-goal.
    ///
    /// For MPTCs, all type arguments must match simultaneously. E.g.,
    /// resolving `Convert Int Double` against `instance Convert Int Double`
    /// matches both the primary and extra type parameters.
    pub fn resolve_chirho(&self, pred_chirho: &PredChirho) -> Option<Vec<PredChirho>> {
        let instances_chirho = self
            .instances_chirho()
            .get_chirho(&pred_chirho.class_name_chirho)?;
        instances_chirho
            .iter_chirho()
            .find_map(|inst_chirho| Self::instance_sub_goals_chirho(inst_chirho, pred_chirho))
    }

    /// Apply functional dependency improvement to a predicate.
    ///
    /// Given a predicate like `Convert Int b` and a class `Convert a b | a -> b`
    /// with an instance `instance Convert Int Double`, the functional dependency
    /// `a -> b` lets us determine that `b = Double`. Returns a substitution
    /// that improves the unknown type variables.
    pub fn fundep_improve_chirho(&self, pred_chirho: &PredChirho) -> SubstChirho {
        let class_chirho = match self
            .classes_chirho()
            .get_chirho(&pred_chirho.class_name_chirho)
        {
            Some(c_chirho) => c_chirho,
            None => return SubstChirho::empty_chirho(),
        };

        if class_chirho.fundeps_chirho.is_empty() {
            return SubstChirho::empty_chirho();
        }

        let instances_chirho = match self
            .instances_chirho()
            .get_chirho(&pred_chirho.class_name_chirho)
        {
            Some(i_chirho) => i_chirho,
            None => return SubstChirho::empty_chirho(),
        };

        let pred_tys_chirho = pred_chirho.all_tys_chirho();
        let mut improvement_chirho = SubstChirho::empty_chirho();

        for inst_chirho in instances_chirho {
            let mut inst_tys_chirho = vec![inst_chirho.head_ty_chirho.clone()];
            inst_tys_chirho.extend(inst_chirho.extra_head_tys_chirho.iter().cloned());

            for (from_chirho, to_chirho) in &class_chirho.fundeps_chirho {
                // Check if "from" positions in the predicate match the instance
                let mut from_subst_chirho = SubstChirho::empty_chirho();
                let mut from_matches_chirho = true;

                for &idx_chirho in from_chirho {
                    if idx_chirho >= pred_tys_chirho.len() || idx_chirho >= inst_tys_chirho.len() {
                        from_matches_chirho = false;
                        break;
                    }
                    match match_ty_chirho(
                        &inst_tys_chirho[idx_chirho],
                        &pred_tys_chirho[idx_chirho],
                    ) {
                        Some(s_chirho) => {
                            if !from_subst_chirho.merge_chirho(&s_chirho) {
                                from_matches_chirho = false;
                                break;
                            }
                        }
                        None => {
                            from_matches_chirho = false;
                            break;
                        }
                    }
                }

                if !from_matches_chirho {
                    continue;
                }

                // "from" positions match — the "to" positions are determined
                for &idx_chirho in to_chirho {
                    if idx_chirho >= pred_tys_chirho.len() || idx_chirho >= inst_tys_chirho.len() {
                        continue;
                    }
                    let determined_ty_chirho =
                        from_subst_chirho.apply_ty_chirho(&inst_tys_chirho[idx_chirho]);
                    let pred_ty_chirho = &pred_tys_chirho[idx_chirho];

                    match pred_ty_chirho {
                        // A bare variable in the determined position is improved
                        // to what the instance determines.
                        TyChirho::VarChirho(v_chirho) => {
                            improvement_chirho.insert_chirho(*v_chirho, determined_ty_chirho);
                        }
                        // Otherwise the instance still determines it: unify, which
                        // binds the unification variables the matched "from"
                        // positions carried into the determined type (`Y [[t]] a`
                        // against `instance Y [[b]] b` yields `t := a`). Only the
                        // predicate's own variables may be bound.
                        _ => {
                            if let Ok(unifier_chirho) = unify_chirho(
                                &determined_ty_chirho,
                                pred_ty_chirho,
                                SpanChirho::DUMMY_CHIRHO,
                            ) {
                                let pred_vars_chirho: Vec<TyVarChirho> = pred_tys_chirho
                                    .iter()
                                    .flat_map(|t_chirho| t_chirho.free_vars_chirho())
                                    .collect();
                                for (var_chirho, ty_chirho) in unifier_chirho.iter_chirho() {
                                    if pred_vars_chirho.contains(var_chirho) {
                                        improvement_chirho
                                            .insert_chirho(*var_chirho, ty_chirho.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        improvement_chirho
    }

    /// Check if a predicate is entailed by the class environment
    /// (i.e., there exists an instance that satisfies it with no remaining goals).
    pub fn entails_chirho(&self, pred_chirho: &PredChirho) -> bool {
        self.entails_depth_chirho(pred_chirho, 0)
    }

    fn entails_depth_chirho(&self, pred_chirho: &PredChirho, depth_chirho: usize) -> bool {
        // Prevent stack overflow from deep or cyclic instance resolution
        if depth_chirho > 64 {
            return true; // assume satisfiable at excessive depth
        }
        // If the predicate's type (or any extra type argument) is still a
        // variable, we cannot resolve it concretely — defer it (assume
        // satisfiable, like GHC does for ambiguous/deferred constraints).
        if pred_chirho.ty_chirho.contains_var_chirho()
            || pred_chirho
                .extra_tys_chirho
                .iter()
                .any(|t_chirho| t_chirho.contains_var_chirho())
        {
            return true;
        }
        if let Some(sub_goals_chirho) = self.resolve_chirho(pred_chirho) {
            sub_goals_chirho
                .iter()
                .all(|sg_chirho| self.entails_depth_chirho(sg_chirho, depth_chirho + 1))
        } else {
            false
        }
    }

    /// Two instances are duplicates when their heads are equal up to a
    /// renaming of their own type variables (GHC-59692); contexts do not
    /// distinguish them. Consistent matching in both directions: the second
    /// direction makes the renaming injective.
    /// workflow: language-features-chirho/instance-obligations-chirho
    pub fn instance_heads_alpha_equal_chirho(
        left_chirho: &InstDeclChirho,
        right_chirho: &InstDeclChirho,
    ) -> bool {
        if left_chirho.class_name_chirho != right_chirho.class_name_chirho
            || left_chirho.extra_head_tys_chirho.len() != right_chirho.extra_head_tys_chirho.len()
        {
            return false;
        }
        let heads_chirho = |inst_chirho: &InstDeclChirho| -> Vec<TyChirho> {
            std::iter::once(inst_chirho.head_ty_chirho.clone())
                .chain(inst_chirho.extra_head_tys_chirho.iter().cloned())
                .collect()
        };
        let (left_heads_chirho, right_heads_chirho) =
            (heads_chirho(left_chirho), heads_chirho(right_chirho));
        let one_way_chirho = |patterns_chirho: &[TyChirho], targets_chirho: &[TyChirho]| {
            let mut bindings_chirho = HashMap::new();
            patterns_chirho
                .iter()
                .zip(targets_chirho)
                .all(|(pattern_chirho, target_chirho)| {
                    match_consistently_chirho(pattern_chirho, target_chirho, &mut bindings_chirho)
                })
        };
        one_way_chirho(&left_heads_chirho, &right_heads_chirho)
            && one_way_chirho(&right_heads_chirho, &left_heads_chirho)
    }

    /// Seed the environment with standard Haskell typeclasses.
    pub fn seed_standard_chirho(&mut self) {
        standard_chirho::seed_standard_chirho(self);
    }
}

/// Legacy one-way matching used by functional-dependency improvement. Its
/// sub-match composition is not a consistent instance-head substitution;
/// instance selection uses `instance_matching_chirho` instead.
pub(crate) fn match_ty_chirho(
    pattern_chirho: &TyChirho,
    target_chirho: &TyChirho,
) -> Option<SubstChirho> {
    match (pattern_chirho, target_chirho) {
        (TyChirho::VarChirho(v_chirho), _) => Some(SubstChirho::singleton_chirho(
            *v_chirho,
            target_chirho.clone(),
        )),

        (TyChirho::ConChirho(a_chirho), TyChirho::ConChirho(b_chirho)) if a_chirho == b_chirho => {
            Some(SubstChirho::empty_chirho())
        }

        (
            TyChirho::FunChirho(pa_chirho, pb_chirho, _),
            TyChirho::FunChirho(ta_chirho, tb_chirho, _),
        ) => {
            let s1_chirho = match_ty_chirho(pa_chirho, ta_chirho)?;
            let s2_chirho = match_ty_chirho(pb_chirho, tb_chirho)?;
            Some(s1_chirho.compose_chirho(&s2_chirho))
        }

        (TyChirho::AppChirho(pf_chirho, pa_chirho), TyChirho::AppChirho(tf_chirho, ta_chirho)) => {
            let s1_chirho = match_ty_chirho(pf_chirho, tf_chirho)?;
            let s2_chirho = match_ty_chirho(pa_chirho, ta_chirho)?;
            Some(s1_chirho.compose_chirho(&s2_chirho))
        }

        (TyChirho::ListChirho(p_chirho), TyChirho::ListChirho(t_chirho)) => {
            match_ty_chirho(p_chirho, t_chirho)
        }

        // Cross-representation: ListChirho ↔ AppChirho(ConChirho("[]"), elem)
        (TyChirho::ListChirho(p_chirho), TyChirho::AppChirho(f_chirho, t_chirho)) if matches!(f_chirho.as_ref(), TyChirho::ConChirho(n) if n == "[]") => {
            match_ty_chirho(p_chirho, t_chirho)
        }
        (TyChirho::AppChirho(f_chirho, p_chirho), TyChirho::ListChirho(t_chirho)) if matches!(f_chirho.as_ref(), TyChirho::ConChirho(n) if n == "[]") => {
            match_ty_chirho(p_chirho, t_chirho)
        }

        (TyChirho::TupleChirho(ps_chirho), TyChirho::TupleChirho(ts_chirho))
            if ps_chirho.len() == ts_chirho.len() =>
        {
            let mut subst_chirho = SubstChirho::empty_chirho();
            for (p_chirho, t_chirho) in ps_chirho.iter().zip(ts_chirho.iter()) {
                let s_chirho = match_ty_chirho(p_chirho, t_chirho)?;
                subst_chirho = subst_chirho.compose_chirho(&s_chirho);
            }
            Some(subst_chirho)
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests_chirho;
