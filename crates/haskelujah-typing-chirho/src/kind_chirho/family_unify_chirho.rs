// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family normalization precedes nominal decomposition. Only validated inverse
//! equalities reach the ordinary occurs/rigidity checker.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;
use crate::families_chirho::family_injectivity_chirho::{
    contains_family_chirho, inverse_equations_chirho,
};
use crate::families_chirho::{
    FamilyReductionChirho, FamilyTermChirho, reduce_equations_bounded_chirho,
};

fn family_spine_chirho(term_chirho: &KindChirho) -> Option<(&str, Vec<KindChirho>)> {
    let mut head_chirho = term_chirho;
    let mut arguments_chirho = Vec::new();
    while let KindChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
        arguments_chirho.push(argument_chirho.as_ref().clone());
        head_chirho = fun_chirho;
    }
    arguments_chirho.reverse();
    if let KindChirho::ConChirho(name_chirho) = head_chirho {
        Some((name_chirho, arguments_chirho))
    } else {
        None
    }
}

impl KindInferCtxChirho {
    /// Choose fresh occurrence arguments that make the entire terms identical.
    /// This is forward instantiation, not cancellation of a family application:
    /// it never equates written/rigid arguments because F a ~ F b was observed.
    fn match_kind_instantiation_chirho(
        &self,
        pattern_chirho: &KindChirho,
        target_chirho: &KindChirho,
    ) -> Option<KindSubstChirho> {
        let mut bindings_chirho = KindSubstChirho::empty_chirho();
        let mut pending_chirho = vec![(pattern_chirho, target_chirho)];
        while let Some((pattern_chirho, target_chirho)) = pending_chirho.pop() {
            if pattern_chirho == target_chirho {
                continue;
            }
            if let KindChirho::VarChirho(variable_chirho) = pattern_chirho
                && self
                    .instantiated_kind_variables_chirho
                    .contains(variable_chirho)
            {
                let target_chirho = bindings_chirho.apply_chirho(target_chirho);
                if let Some(previous_chirho) = bindings_chirho.map_chirho.get(variable_chirho) {
                    if bindings_chirho.apply_chirho(previous_chirho) != target_chirho {
                        return None;
                    }
                } else {
                    let next_chirho = unify_kind_chirho(
                        pattern_chirho,
                        &target_chirho,
                        "kind instantiation",
                        SpanChirho::DUMMY_CHIRHO,
                    )
                    .ok()?;
                    bindings_chirho = next_chirho.compose_chirho(&bindings_chirho);
                }
                continue;
            }
            match (pattern_chirho.parts_chirho(), target_chirho.parts_chirho()) {
                (
                    Some((left_head_chirho, left_chirho)),
                    Some((right_head_chirho, right_chirho)),
                ) if left_head_chirho == right_head_chirho
                    && left_chirho.len() == right_chirho.len() =>
                {
                    pending_chirho.extend(left_chirho.into_iter().zip(right_chirho))
                }
                _ => return None,
            }
        }
        (bindings_chirho.apply_chirho(pattern_chirho)
            == bindings_chirho.apply_chirho(target_chirho))
        .then_some(bindings_chirho)
    }

    fn normalize_kind_family_chirho(
        &self,
        term_chirho: &KindChirho,
        fuel_chirho: &mut usize,
    ) -> KindChirho {
        if *fuel_chirho == 0 {
            return term_chirho.clone();
        }
        *fuel_chirho -= 1;
        let mut normalized_chirho = term_chirho.map_children_chirho(&mut |child_chirho| {
            self.normalize_kind_family_chirho(child_chirho, fuel_chirho)
        });
        // Reduction may erase the last use of a dependent binder. Its result
        // is then an ordinary arrow, with surrounding de Bruijn positions
        // shifted out of the removed scope. Never erase a surviving dependency.
        if let KindChirho::DependentChirho {
            argument_chirho,
            result_chirho,
        } = &normalized_chirho
            && !result_chirho.references_bound_chirho(0)
        {
            normalized_chirho = KindChirho::arrow_chirho(
                argument_chirho.as_ref().clone(),
                result_chirho.substitute_bound_chirho(&KindChirho::StarChirho),
            );
        }
        if let Some((name_chirho, arguments_chirho)) = family_spine_chirho(&normalized_chirho)
            && let Some(family_chirho) = self.kind_families_chirho.get(name_chirho)
        {
            match reduce_equations_bounded_chirho(
                &family_chirho.equations_chirho,
                &arguments_chirho,
                &|head_chirho| self.kind_family_names_chirho.contains(head_chirho),
                fuel_chirho,
            ) {
                FamilyReductionChirho::ReducedChirho(reduced_chirho)
                    if reduced_chirho != normalized_chirho =>
                {
                    return self.normalize_kind_family_chirho(&reduced_chirho, fuel_chirho);
                }
                FamilyReductionChirho::LimitedChirho => *fuel_chirho = 0,
                _ => {}
            }
        }
        normalized_chirho
    }

    fn inverse_kind_family_chirho(
        &self,
        term_chirho: &KindChirho,
        result_chirho: &KindChirho,
    ) -> Option<Vec<(KindChirho, KindChirho)>> {
        let (name_chirho, arguments_chirho) = family_spine_chirho(term_chirho)?;
        let family_chirho = self.kind_families_chirho.get(name_chirho)?;
        if family_chirho.injective_chirho.is_empty() {
            return None;
        }
        inverse_equations_chirho(
            &family_chirho.equations_chirho,
            &arguments_chirho,
            result_chirho,
            &family_chirho.injective_chirho,
            &|head_chirho| self.kind_family_names_chirho.contains(head_chirho),
        )
    }

    pub(super) fn unify_family_kinds_chirho(
        &self,
        left_chirho: &KindChirho,
        right_chirho: &KindChirho,
        context_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<KindSubstChirho, KindErrorChirho> {
        let family_chirho = |name_chirho: &str| self.kind_family_names_chirho.contains(name_chirho);
        if !contains_family_chirho(left_chirho, &family_chirho)
            && !contains_family_chirho(right_chirho, &family_chirho)
        {
            return unify_kind_chirho(left_chirho, right_chirho, context_chirho, span_chirho);
        }
        let mut pending_chirho = vec![(left_chirho.clone(), right_chirho.clone())];
        let mut substitution_chirho = KindSubstChirho::empty_chirho();
        let mut fuel_chirho = 4096;
        let initial_error_chirho = KindErrorChirho::MismatchChirho {
            expected_chirho: left_chirho.clone(),
            actual_chirho: right_chirho.clone(),
            context_chirho: context_chirho.to_string(),
            span_chirho,
        };
        while !pending_chirho.is_empty() {
            let mut deferred_chirho = Vec::new();
            let mut progress_chirho = false;
            while let Some((left_chirho, right_chirho)) = pending_chirho.pop() {
                if fuel_chirho == 0 {
                    return Err(KindErrorChirho::ReductionLimitChirho { span_chirho });
                }
                fuel_chirho -= 1;
                let left_chirho = self.normalize_kind_family_chirho(
                    &substitution_chirho.apply_chirho(&left_chirho),
                    &mut fuel_chirho,
                );
                let right_chirho = self.normalize_kind_family_chirho(
                    &substitution_chirho.apply_chirho(&right_chirho),
                    &mut fuel_chirho,
                );
                if fuel_chirho == 0 {
                    return Err(KindErrorChirho::ReductionLimitChirho { span_chirho });
                }
                if left_chirho == right_chirho {
                    continue;
                }
                if matches!(left_chirho, KindChirho::VarChirho(_))
                    || matches!(right_chirho, KindChirho::VarChirho(_))
                {
                    let next_chirho = unify_kind_chirho(
                        &left_chirho,
                        &right_chirho,
                        context_chirho,
                        span_chirho,
                    )?;
                    substitution_chirho = next_chirho.compose_chirho(&substitution_chirho);
                    progress_chirho = true;
                    continue;
                }
                let left_family_chirho = left_chirho
                    .head_name_chirho()
                    .is_some_and(|name_chirho| self.kind_family_names_chirho.contains(name_chirho));
                let right_family_chirho = right_chirho
                    .head_name_chirho()
                    .is_some_and(|name_chirho| self.kind_family_names_chirho.contains(name_chirho));
                if left_family_chirho || right_family_chirho {
                    if let Some(next_chirho) = self
                        .match_kind_instantiation_chirho(&left_chirho, &right_chirho)
                        .or_else(|| {
                            self.match_kind_instantiation_chirho(&right_chirho, &left_chirho)
                        })
                    {
                        progress_chirho |= !next_chirho.map_chirho.is_empty();
                        substitution_chirho = next_chirho.compose_chirho(&substitution_chirho);
                        continue;
                    }
                    if let Some(equalities_chirho) = self
                        .inverse_kind_family_chirho(&left_chirho, &right_chirho)
                        .or_else(|| self.inverse_kind_family_chirho(&right_chirho, &left_chirho))
                    {
                        pending_chirho.extend(
                            equalities_chirho
                                .into_iter()
                                .filter(|(left_chirho, right_chirho)| left_chirho != right_chirho),
                        );
                    }
                    deferred_chirho.push((left_chirho, right_chirho));
                    continue;
                }
                match (&left_chirho, &right_chirho) {
                    (
                        KindChirho::ArrowChirho(left_argument_chirho, left_result_chirho),
                        KindChirho::ArrowChirho(right_argument_chirho, right_result_chirho),
                    )
                    | (
                        KindChirho::AppChirho(left_argument_chirho, left_result_chirho),
                        KindChirho::AppChirho(right_argument_chirho, right_result_chirho),
                    )
                    | (
                        KindChirho::DependentChirho {
                            argument_chirho: left_argument_chirho,
                            result_chirho: left_result_chirho,
                        },
                        KindChirho::DependentChirho {
                            argument_chirho: right_argument_chirho,
                            result_chirho: right_result_chirho,
                        },
                    ) => {
                        pending_chirho.push((
                            left_argument_chirho.as_ref().clone(),
                            right_argument_chirho.as_ref().clone(),
                        ));
                        pending_chirho.push((
                            left_result_chirho.as_ref().clone(),
                            right_result_chirho.as_ref().clone(),
                        ));
                    }
                    (
                        KindChirho::ArrowChirho(argument_chirho, result_chirho),
                        application_chirho @ KindChirho::AppChirho(_, _),
                    )
                    | (
                        application_chirho @ KindChirho::AppChirho(_, _),
                        KindChirho::ArrowChirho(argument_chirho, result_chirho),
                    ) => {
                        let arrow_chirho = KindChirho::app_chirho(
                            KindChirho::app_chirho(
                                KindChirho::ConChirho("->".into()),
                                argument_chirho.as_ref().clone(),
                            ),
                            result_chirho.as_ref().clone(),
                        );
                        pending_chirho.push((arrow_chirho, application_chirho.clone()));
                    }
                    (
                        KindChirho::StarChirho,
                        KindChirho::AppChirho(fun_chirho, representation_chirho),
                    )
                    | (
                        KindChirho::AppChirho(fun_chirho, representation_chirho),
                        KindChirho::StarChirho,
                    ) if matches!(fun_chirho.as_ref(), KindChirho::ConChirho(name_chirho) if name_chirho == super::runtime_chirho::TYPE_CHIRHO) =>
                    {
                        pending_chirho.push((
                            representation_chirho.as_ref().clone(),
                            super::runtime_chirho::boxed_rep_chirho("Lifted"),
                        ));
                    }
                    _ => {
                        let next_chirho = unify_kind_chirho(
                            &left_chirho,
                            &right_chirho,
                            context_chirho,
                            span_chirho,
                        )?;
                        progress_chirho |= !next_chirho.map_chirho.is_empty();
                        substitution_chirho = next_chirho.compose_chirho(&substitution_chirho);
                    }
                }
            }
            if !deferred_chirho.is_empty() && !progress_chirho {
                return Err(initial_error_chirho);
            }
            pending_chirho = deferred_chirho;
        }
        Ok(substitution_chirho)
    }
}
