// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A written dependency is not evidence until its equations validate it.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    FamilyMatchChirho, FamilyTermChirho, match_pattern_chirho, matching_input_fits_chirho,
};
use std::collections::{HashMap, HashSet};

fn variables_chirho<TermChirho: FamilyTermChirho>(
    term_chirho: &TermChirho,
) -> HashSet<TermChirho::VariableChirho> {
    let mut variables_chirho = HashSet::new();
    let mut pending_chirho = vec![term_chirho];
    while let Some(term_chirho) = pending_chirho.pop() {
        if let Some(variable_chirho) = term_chirho.variable_chirho() {
            variables_chirho.insert(variable_chirho);
        }
        if let Some((_, children_chirho)) = term_chirho.parts_chirho() {
            pending_chirho.extend(children_chirho);
        }
    }
    variables_chirho
}

pub(crate) fn contains_family_chirho<TermChirho: FamilyTermChirho>(
    term_chirho: &TermChirho,
    family_chirho: &impl Fn(&str) -> bool,
) -> bool {
    term_chirho.head_name_chirho().is_some_and(family_chirho)
        || term_chirho
            .parts_chirho()
            .is_some_and(|(_, children_chirho)| {
                children_chirho
                    .into_iter()
                    .any(|child_chirho| contains_family_chirho(child_chirho, family_chirho))
            })
}

/// `Ok(true)` is a proof within the represented first-order fragment.
/// `Ok(false)` is unproved (opaque terms or exhausted local budget),
/// never permission to infer an argument. Equation variables must be fresh per row.
/// Compositions use only earlier validated, arity-matched dependencies: the
/// callback must not provisionally trust an annotation in a recursive group.
pub(crate) fn validate_injectivity_chirho<'proof_chirho, TermChirho: FamilyTermChirho>(
    equations_chirho: &[(Vec<TermChirho>, TermChirho)],
    injective_chirho: &[usize],
    family_chirho: &impl Fn(&str) -> bool,
    injective_arguments_chirho: &impl Fn(&str, usize) -> Option<&'proof_chirho [usize]>,
) -> Result<bool, &'static str> {
    let mut budget_chirho = 16_384;
    for (patterns_chirho, result_chirho) in equations_chirho {
        for term_chirho in patterns_chirho.iter().chain([result_chirho]) {
            if !matching_input_fits_chirho(term_chirho, &mut budget_chirho) {
                return Ok(false);
            }
        }
        let mut pending_chirho: Vec<_> = patterns_chirho.iter().chain([result_chirho]).collect();
        while let Some(term_chirho) = pending_chirho.pop() {
            if term_chirho.unknown_chirho() && term_chirho.variable_chirho().is_none() {
                return Ok(false);
            }
            if let Some((_, children_chirho)) = term_chirho.parts_chirho() {
                pending_chirho.extend(children_chirho);
            }
        }
        if contains_family_chirho(result_chirho, family_chirho) {
            // With one covering equation there is no cross-row overlap to
            // justify. Equality of results determines each variable only along
            // nominal constructors and independently proved injective positions.
            // Partial patterns and multi-row compositions need a stronger proof.
            let variables_chirho: Option<HashSet<_>> = patterns_chirho
                .iter()
                .map(FamilyTermChirho::variable_chirho)
                .collect();
            if equations_chirho.len() == 1
                && variables_chirho
                    .is_some_and(|variables_chirho| variables_chirho.len() == patterns_chirho.len())
            {
                if let Some(determined_chirho) = determining_variables_chirho(
                    result_chirho,
                    family_chirho,
                    injective_arguments_chirho,
                    &mut budget_chirho,
                ) {
                    if injective_chirho.iter().all(|index_chirho| {
                        patterns_chirho
                            .get(*index_chirho)
                            .and_then(FamilyTermChirho::variable_chirho)
                            .is_some_and(|variable_chirho| {
                                determined_chirho.contains(&variable_chirho)
                            })
                    }) {
                        continue;
                    }
                    return Err("injectivity loses a determining argument variable in the result");
                }
                if budget_chirho == 0 {
                    return Ok(false);
                }
            }
            if result_chirho.head_name_chirho().is_some_and(family_chirho) {
                return Err("injectivity cannot be established by a family-headed result");
            }
            return Ok(false);
        }
        if result_chirho.variable_chirho().is_some() {
            let bare_chirho: Option<HashSet<_>> = patterns_chirho
                .iter()
                .map(FamilyTermChirho::variable_chirho)
                .collect();
            if bare_chirho
                .is_none_or(|variables_chirho| variables_chirho.len() != patterns_chirho.len())
            {
                return Err(
                    "injectivity with a bare result variable requires covering variable patterns",
                );
            }
        }
        let result_variables_chirho = variables_chirho(result_chirho);
        for &index_chirho in injective_chirho {
            let Some(pattern_chirho) = patterns_chirho.get(index_chirho) else {
                return Ok(false);
            };
            if !variables_chirho(pattern_chirho).is_subset(&result_variables_chirho) {
                return Err("injectivity loses a determining argument variable in the result");
            }
        }
    }
    for (later_index_chirho, (later_patterns_chirho, later_result_chirho)) in
        equations_chirho.iter().enumerate()
    {
        for (earlier_patterns_chirho, earlier_result_chirho) in
            &equations_chirho[..later_index_chirho]
        {
            let Ok(bindings_chirho) = unify_results_chirho(
                earlier_result_chirho,
                later_result_chirho,
                &mut budget_chirho,
            ) else {
                return Ok(false);
            };
            let Some(bindings_chirho) = bindings_chirho else {
                continue;
            };
            let instantiate_chirho = |patterns_chirho: &[TermChirho], budget_chirho: &mut usize| {
                patterns_chirho
                    .iter()
                    .map(|pattern_chirho| {
                        pattern_chirho.substitute_bounded_chirho(&bindings_chirho, budget_chirho)
                    })
                    .collect::<Option<Vec<_>>>()
            };
            let Some(earlier_chirho) =
                instantiate_chirho(earlier_patterns_chirho, &mut budget_chirho)
            else {
                return Ok(false);
            };
            let Some(later_chirho) = instantiate_chirho(later_patterns_chirho, &mut budget_chirho)
            else {
                return Ok(false);
            };
            let compatible_chirho = injective_chirho.iter().all(|&index_chirho| {
                earlier_chirho
                    .get(index_chirho)
                    .zip(later_chirho.get(index_chirho))
                    .is_some_and(|(earlier_chirho, later_chirho)| earlier_chirho == later_chirho)
            });
            if !compatible_chirho {
                // Check the conflicting INSTANCE of the later LHS against the
                // whole earlier prefix. The covering row need not be this pair's
                // partner, and a row after the later one cannot justify it.
                let Some(covered_chirho) = covered_by_prefix_chirho(
                    &equations_chirho[..later_index_chirho],
                    &later_chirho,
                    family_chirho,
                    &mut budget_chirho,
                ) else {
                    return Ok(false);
                };
                if !covered_chirho {
                    return Err(
                        "injectivity conflict: equal results do not determine equal arguments",
                    );
                }
            }
        }
    }
    Ok(true)
}

/// A source-ordered prefix excludes a later row only by a definite match.
/// The caller supplies RHS-instantiated inputs; prefix patterns keep their own
/// row-local variables. The shared budget bounds prefix scans and binding clones.
fn covered_by_prefix_chirho<TermChirho: FamilyTermChirho>(
    prefix_chirho: &[(Vec<TermChirho>, TermChirho)],
    arguments_chirho: &[TermChirho],
    family_chirho: &impl Fn(&str) -> bool,
    budget_chirho: &mut usize,
) -> Option<bool> {
    for (patterns_chirho, _) in prefix_chirho {
        *budget_chirho = budget_chirho.checked_sub(1)?;
        if patterns_chirho.len() != arguments_chirho.len() {
            continue;
        }
        let mut bindings_chirho = HashMap::new();
        let mut covered_chirho = true;
        for (pattern_chirho, argument_chirho) in patterns_chirho.iter().zip(arguments_chirho) {
            if !matching_input_fits_chirho(pattern_chirho, budget_chirho)
                || !matching_input_fits_chirho(argument_chirho, budget_chirho)
            {
                return None;
            }
            if match_pattern_chirho(
                pattern_chirho,
                argument_chirho,
                &mut bindings_chirho,
                family_chirho,
            ) != FamilyMatchChirho::MatchedChirho
            {
                covered_chirho = false;
                break;
            }
        }
        if covered_chirho {
            return Some(true);
        }
    }
    Some(false)
}

/// A family result does not expose every variable written inside it. Follow
/// only its proved determining positions, without normalizing or validating
/// another family recursively. This traverses local terms under a fixed budget.
fn determining_variables_chirho<'proof_chirho, TermChirho: FamilyTermChirho>(
    result_chirho: &TermChirho,
    family_chirho: &impl Fn(&str) -> bool,
    injective_arguments_chirho: &impl Fn(&str, usize) -> Option<&'proof_chirho [usize]>,
    budget_chirho: &mut usize,
) -> Option<HashSet<TermChirho::VariableChirho>> {
    let mut determined_chirho = HashSet::new();
    let mut pending_chirho = vec![result_chirho];
    while let Some(term_chirho) = pending_chirho.pop() {
        *budget_chirho = budget_chirho.checked_sub(1)?;
        if let Some(variable_chirho) = term_chirho.variable_chirho() {
            determined_chirho.insert(variable_chirho);
        } else if term_chirho.unknown_chirho() {
            return None;
        } else if let Some(name_chirho) = term_chirho.head_name_chirho()
            && family_chirho(name_chirho)
        {
            let mut head_chirho = term_chirho;
            let mut arguments_chirho = Vec::new();
            while let Some((fun_chirho, argument_chirho)) = head_chirho.application_parts_chirho() {
                *budget_chirho = budget_chirho.checked_sub(1)?;
                arguments_chirho.push(argument_chirho);
                head_chirho = fun_chirho;
            }
            arguments_chirho.reverse();
            for &index_chirho in injective_arguments_chirho(name_chirho, arguments_chirho.len())? {
                pending_chirho.push(*arguments_chirho.get(index_chirho)?);
            }
        } else if let Some((_, children_chirho)) = term_chirho.parts_chirho() {
            if children_chirho.len_chirho() > *budget_chirho {
                *budget_chirho = 0;
                return None;
            }
            pending_chirho.extend(children_chirho);
        }
    }
    Some(determined_chirho)
}

fn unify_results_chirho<TermChirho: FamilyTermChirho>(
    left_chirho: &TermChirho,
    right_chirho: &TermChirho,
    budget_chirho: &mut usize,
) -> Result<Option<HashMap<TermChirho::VariableChirho, TermChirho>>, &'static str> {
    let mut bindings_chirho = HashMap::new();
    let mut pending_chirho = vec![(left_chirho.clone(), right_chirho.clone())];
    while let Some((left_chirho, right_chirho)) = pending_chirho.pop() {
        if *budget_chirho == 0 {
            return Err("injectivity verification exceeded its equation-work limit");
        }
        *budget_chirho -= 1;
        let left_chirho = left_chirho
            .substitute_bounded_chirho(&bindings_chirho, budget_chirho)
            .ok_or("injectivity substitution exceeded its work limit")?;
        let right_chirho = right_chirho
            .substitute_bounded_chirho(&bindings_chirho, budget_chirho)
            .ok_or("injectivity substitution exceeded its work limit")?;
        if left_chirho == right_chirho {
            continue;
        }
        if let Some((variable_chirho, value_chirho)) = left_chirho
            .variable_chirho()
            .map(|variable_chirho| (variable_chirho, &right_chirho))
            .or_else(|| {
                right_chirho
                    .variable_chirho()
                    .map(|variable_chirho| (variable_chirho, &left_chirho))
            })
        {
            if variables_chirho(value_chirho).contains(&variable_chirho) {
                return Ok(None);
            }
            let single_chirho = HashMap::from([(variable_chirho.clone(), value_chirho.clone())]);
            for previous_chirho in bindings_chirho.values_mut() {
                *previous_chirho = previous_chirho
                    .substitute_bounded_chirho(&single_chirho, budget_chirho)
                    .ok_or("injectivity substitution exceeded its work limit")?;
            }
            bindings_chirho.insert(variable_chirho, value_chirho.clone());
            continue;
        }
        match (left_chirho.parts_chirho(), right_chirho.parts_chirho()) {
            (
                Some((left_head_chirho, left_parts_chirho)),
                Some((right_head_chirho, right_parts_chirho)),
            ) if left_head_chirho == right_head_chirho
                && left_parts_chirho.len_chirho() == right_parts_chirho.len_chirho() =>
            {
                pending_chirho.extend(left_parts_chirho.into_iter().zip(right_parts_chirho).map(
                    |(left_chirho, right_chirho)| (left_chirho.clone(), right_chirho.clone()),
                ));
            }
            _ => return Ok(None),
        }
    }
    Ok(Some(bindings_chirho))
}

/// Recover only arguments whose variables the result actually determined.
/// No solver metavariable is changed here; the caller checks these equalities.
pub(crate) fn inverse_equations_chirho<TermChirho: FamilyTermChirho>(
    equations_chirho: &[(Vec<TermChirho>, TermChirho)],
    arguments_chirho: &[TermChirho],
    result_chirho: &TermChirho,
    injective_chirho: &[usize],
    family_chirho: &impl Fn(&str) -> bool,
) -> Option<Vec<(TermChirho, TermChirho)>> {
    let mut selected_chirho: Option<Vec<(TermChirho, TermChirho)>> = None;
    let mut budget_chirho = 16_384;
    for (index_chirho, (patterns_chirho, rhs_chirho)) in equations_chirho.iter().enumerate() {
        if patterns_chirho.len() != arguments_chirho.len() {
            return None;
        }
        if !matching_input_fits_chirho(rhs_chirho, &mut budget_chirho)
            || !matching_input_fits_chirho(result_chirho, &mut budget_chirho)
        {
            return None;
        }
        let mut bindings_chirho = HashMap::new();
        match match_pattern_chirho(
            rhs_chirho,
            result_chirho,
            &mut bindings_chirho,
            family_chirho,
        ) {
            FamilyMatchChirho::ApartChirho => continue,
            FamilyMatchChirho::StuckChirho => return None,
            FamilyMatchChirho::MatchedChirho => {}
        }
        let instantiated_chirho = patterns_chirho
            .iter()
            .map(|pattern_chirho| {
                pattern_chirho.substitute_bounded_chirho(&bindings_chirho, &mut budget_chirho)
            })
            .collect::<Option<Vec<_>>>()?;
        if covered_by_prefix_chirho(
            &equations_chirho[..index_chirho],
            &instantiated_chirho,
            family_chirho,
            &mut budget_chirho,
        )? {
            continue;
        }
        let mut argument_bindings_chirho = HashMap::new();
        for term_chirho in patterns_chirho.iter().chain(arguments_chirho) {
            if !matching_input_fits_chirho(term_chirho, &mut budget_chirho) {
                return None;
            }
        }
        if patterns_chirho
            .iter()
            .zip(arguments_chirho)
            .any(|(pattern_chirho, argument_chirho)| {
                match_pattern_chirho(
                    pattern_chirho,
                    argument_chirho,
                    &mut argument_bindings_chirho,
                    family_chirho,
                ) == FamilyMatchChirho::ApartChirho
            })
        {
            continue;
        }
        let equalities_chirho: Vec<_> = injective_chirho
            .iter()
            .map(|&index_chirho| {
                Some((
                    arguments_chirho.get(index_chirho)?.clone(),
                    instantiated_chirho.get(index_chirho)?.clone(),
                ))
            })
            .collect::<Option<_>>()?;
        if selected_chirho
            .as_ref()
            .is_some_and(|previous_chirho| previous_chirho != &equalities_chirho)
        {
            return None;
        }
        selected_chirho = Some(equalities_chirho);
    }
    selected_chirho
}
