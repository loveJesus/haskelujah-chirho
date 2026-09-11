// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared equation matching. A blocked earlier equation is not a failed match.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use std::collections::HashMap;
use std::hash::Hash;

#[path = "family_injectivity_chirho.rs"]
pub(crate) mod family_injectivity_chirho;
#[path = "family_types_chirho.rs"]
mod family_types_chirho;
pub(crate) use family_types_chirho::FamilyTypeVariableChirho;
#[cfg(test)]
#[path = "family_tests_chirho.rs"]
mod family_tests_chirho;

pub(crate) trait FamilyTermChirho: Clone + Eq {
    type VariableChirho: Clone + Eq + Hash;
    fn variable_chirho(&self) -> Option<Self::VariableChirho>;
    fn unknown_chirho(&self) -> bool;
    fn head_name_chirho(&self) -> Option<&str>;
    fn parts_chirho(&self) -> Option<(&'static str, Vec<&Self>)>;
    fn map_children_chirho(&self, map_chirho: &mut impl FnMut(&Self) -> Self) -> Self;
    fn application_chirho(fun_chirho: Self, argument_chirho: Self) -> Self;

    fn substitute_chirho(&self, bindings_chirho: &HashMap<Self::VariableChirho, Self>) -> Self {
        if let Some(variable_chirho) = self.variable_chirho()
            && let Some(value_chirho) = bindings_chirho.get(&variable_chirho)
        {
            return value_chirho.clone();
        }
        self.map_children_chirho(&mut |child_chirho| {
            child_chirho.substitute_chirho(bindings_chirho)
        })
    }

    /// Account for the expanded tree BEFORE cloning substitutions into it.
    /// Repeated variables spend the budget repeatedly: a small duplicating RHS
    /// cannot hide exponentially growing output behind a reduction-step count.
    fn substitute_bounded_chirho(
        &self,
        bindings_chirho: &HashMap<Self::VariableChirho, Self>,
        budget_chirho: &mut usize,
    ) -> Option<Self> {
        let mut pending_chirho = vec![(self, true)];
        while let Some((term_chirho, replace_chirho)) = pending_chirho.pop() {
            if replace_chirho
                && let Some(variable_chirho) = term_chirho.variable_chirho()
                && let Some(value_chirho) = bindings_chirho.get(&variable_chirho)
            {
                pending_chirho.push((value_chirho, false));
                continue;
            }
            *budget_chirho = budget_chirho.checked_sub(1)?;
            if let Some((_, children_chirho)) = term_chirho.parts_chirho() {
                pending_chirho.extend(
                    children_chirho
                        .into_iter()
                        .map(|child_chirho| (child_chirho, replace_chirho)),
                );
            }
        }
        Some(self.substitute_chirho(bindings_chirho))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FamilyMatchChirho {
    MatchedChirho,
    ApartChirho,
    StuckChirho,
}

pub(crate) enum FamilyReductionChirho<TermChirho> {
    ReducedChirho(TermChirho),
    ApartChirho,
    StuckChirho,
    LimitedChirho,
}

pub(crate) fn match_pattern_chirho<TermChirho: FamilyTermChirho>(
    pattern_chirho: &TermChirho,
    target_chirho: &TermChirho,
    bindings_chirho: &mut HashMap<TermChirho::VariableChirho, TermChirho>,
    family_chirho: &impl Fn(&str) -> bool,
) -> FamilyMatchChirho {
    if let Some(variable_chirho) = pattern_chirho.variable_chirho() {
        if let Some(previous_chirho) = bindings_chirho.get(&variable_chirho) {
            return compare_terms_chirho(previous_chirho, target_chirho, family_chirho);
        }
        bindings_chirho.insert(variable_chirho, target_chirho.clone());
        return FamilyMatchChirho::MatchedChirho;
    }
    if pattern_chirho == target_chirho {
        return FamilyMatchChirho::MatchedChirho;
    }
    if target_chirho.unknown_chirho()
        || target_chirho.head_name_chirho().is_some_and(family_chirho)
        || pattern_chirho.head_name_chirho().is_some_and(family_chirho)
    {
        return FamilyMatchChirho::StuckChirho;
    }
    match (pattern_chirho.parts_chirho(), target_chirho.parts_chirho()) {
        (Some((left_head_chirho, left_chirho)), Some((right_head_chirho, right_chirho)))
            if left_head_chirho == right_head_chirho && left_chirho.len() == right_chirho.len() =>
        {
            combine_matches_chirho(left_chirho.into_iter().zip(right_chirho).map(
                |(left_chirho, right_chirho)| {
                    match_pattern_chirho(left_chirho, right_chirho, bindings_chirho, family_chirho)
                },
            ))
        }
        _ => FamilyMatchChirho::ApartChirho,
    }
}

fn combine_matches_chirho(
    matches_chirho: impl Iterator<Item = FamilyMatchChirho>,
) -> FamilyMatchChirho {
    let mut result_chirho = FamilyMatchChirho::MatchedChirho;
    for match_chirho in matches_chirho {
        match match_chirho {
            FamilyMatchChirho::ApartChirho => return FamilyMatchChirho::ApartChirho,
            FamilyMatchChirho::StuckChirho => result_chirho = FamilyMatchChirho::StuckChirho,
            FamilyMatchChirho::MatchedChirho => {}
        }
    }
    result_chirho
}

/// Compare without binding variables: repeated pattern variables require equality,
/// not a second independent match or nominal decomposition of a stuck family.
fn compare_terms_chirho<TermChirho: FamilyTermChirho>(
    left_chirho: &TermChirho,
    right_chirho: &TermChirho,
    family_chirho: &impl Fn(&str) -> bool,
) -> FamilyMatchChirho {
    if left_chirho == right_chirho {
        return FamilyMatchChirho::MatchedChirho;
    }
    if left_chirho.unknown_chirho()
        || right_chirho.unknown_chirho()
        || left_chirho.head_name_chirho().is_some_and(family_chirho)
        || right_chirho.head_name_chirho().is_some_and(family_chirho)
    {
        return FamilyMatchChirho::StuckChirho;
    }
    match (left_chirho.parts_chirho(), right_chirho.parts_chirho()) {
        (
            Some((left_head_chirho, left_parts_chirho)),
            Some((right_head_chirho, right_parts_chirho)),
        ) if left_head_chirho == right_head_chirho
            && left_parts_chirho.len() == right_parts_chirho.len() =>
        {
            combine_matches_chirho(left_parts_chirho.into_iter().zip(right_parts_chirho).map(
                |(left_chirho, right_chirho)| {
                    compare_terms_chirho(left_chirho, right_chirho, family_chirho)
                },
            ))
        }
        _ => FamilyMatchChirho::ApartChirho,
    }
}

pub(crate) fn reduce_equations_chirho<TermChirho: FamilyTermChirho>(
    equations_chirho: &[(Vec<TermChirho>, TermChirho)],
    arguments_chirho: &[TermChirho],
    family_chirho: &impl Fn(&str) -> bool,
) -> FamilyReductionChirho<TermChirho> {
    let mut budget_chirho = usize::MAX;
    reduce_equations_bounded_chirho(
        equations_chirho,
        arguments_chirho,
        family_chirho,
        &mut budget_chirho,
    )
}

pub(crate) fn reduce_equations_bounded_chirho<TermChirho: FamilyTermChirho>(
    equations_chirho: &[(Vec<TermChirho>, TermChirho)],
    arguments_chirho: &[TermChirho],
    family_chirho: &impl Fn(&str) -> bool,
    budget_chirho: &mut usize,
) -> FamilyReductionChirho<TermChirho> {
    for (patterns_chirho, result_chirho) in equations_chirho {
        if patterns_chirho.len() > arguments_chirho.len() {
            return FamilyReductionChirho::StuckChirho;
        }
        let mut bindings_chirho = HashMap::new();
        match combine_matches_chirho(patterns_chirho.iter().zip(arguments_chirho).map(
            |(pattern_chirho, argument_chirho)| {
                match_pattern_chirho(
                    pattern_chirho,
                    argument_chirho,
                    &mut bindings_chirho,
                    family_chirho,
                )
            },
        )) {
            FamilyMatchChirho::ApartChirho => continue,
            FamilyMatchChirho::StuckChirho => return FamilyReductionChirho::StuckChirho,
            FamilyMatchChirho::MatchedChirho => {
                let Some(mut reduced_chirho) =
                    result_chirho.substitute_bounded_chirho(&bindings_chirho, budget_chirho)
                else {
                    return FamilyReductionChirho::LimitedChirho;
                };
                for argument_chirho in &arguments_chirho[patterns_chirho.len()..] {
                    let Some(argument_chirho) =
                        argument_chirho.substitute_bounded_chirho(&HashMap::new(), budget_chirho)
                    else {
                        return FamilyReductionChirho::LimitedChirho;
                    };
                    reduced_chirho =
                        TermChirho::application_chirho(reduced_chirho, argument_chirho);
                }
                return FamilyReductionChirho::ReducedChirho(reduced_chirho);
            }
        }
    }
    FamilyReductionChirho::ApartChirho
}
