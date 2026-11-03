// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The bounds of the ground-dictionary pre-pass, all in one place.
//!
//! Counting dictionaries does not bound the types they are built at:
//! `instance C (a, a) => C a` doubles its goal at every step. So every type the pass
//! inspects or copies is charged by weight, one per node plus the bytes of each name, against
//! a module-wide allowance. One goal may weigh at most a fixed limit. Before any candidate
//! instance is matched, its match is preflighted. The most a match and its substituted context
//! can copy is the pattern's own weight plus, for each variable occurrence, one copy of the
//! goal. That bound is checked against a per-candidate limit and charged before the checker's
//! matcher runs. Measuring a type never walks past its cap. Each exhaustion is a named refusal,
//! and the attempt that reaches it is rolled back.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::ground_selection_chirho::GroundRefusalChirho;
use haskelujah_typing_chirho::class_chirho::{InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

/// Nested obligations one goal may descend through.
const GROUND_DEPTH_LIMIT_CHIRHO: usize = 64;
/// Conditional dictionaries one module may construct.
const GROUND_DICTIONARY_LIMIT_CHIRHO: usize = 256;
/// Goals one module's pre-pass may examine; a memoized revisit costs nothing.
const GROUND_VISIT_LIMIT_CHIRHO: usize = 4096;
/// Type weight one module's pre-pass may inspect or copy, in nodes plus name bytes.
pub(in crate::dict_chirho) const GROUND_TYPE_WORK_LIMIT_CHIRHO: usize = 1 << 25;
/// The weight one goal may have.
pub(in crate::dict_chirho) const GROUND_GOAL_WEIGHT_LIMIT_CHIRHO: usize = 1 << 12;
/// The most one candidate's match and substituted context may copy.
pub(in crate::dict_chirho) const GROUND_CANDIDATE_WEIGHT_LIMIT_CHIRHO: usize = 1 << 16;

/// The module-wide allowance the pre-pass spends.
#[derive(Debug, Default)]
pub(in crate::dict_chirho) struct GroundBudgetChirho {
    dictionaries_chirho: usize,
    visits_chirho: usize,
    type_work_chirho: usize,
}

impl GroundBudgetChirho {
    /// One more goal examined, `depth_chirho` obligations below its top-level goal.
    pub(in crate::dict_chirho) fn visit_chirho(
        &mut self,
        depth_chirho: usize,
    ) -> Result<(), GroundRefusalChirho> {
        self.visits_chirho += 1;
        if self.visits_chirho > GROUND_VISIT_LIMIT_CHIRHO {
            return Err(GroundRefusalChirho::BudgetChirho(
                "visit",
                GROUND_VISIT_LIMIT_CHIRHO,
            ));
        }
        if depth_chirho > GROUND_DEPTH_LIMIT_CHIRHO {
            return Err(GroundRefusalChirho::BudgetChirho(
                "depth",
                GROUND_DEPTH_LIMIT_CHIRHO,
            ));
        }
        Ok(())
    }

    /// One more dictionary constructed.
    pub(in crate::dict_chirho) fn dictionary_chirho(&mut self) -> Result<(), GroundRefusalChirho> {
        self.dictionaries_chirho += 1;
        if self.dictionaries_chirho > GROUND_DICTIONARY_LIMIT_CHIRHO {
            return Err(GroundRefusalChirho::BudgetChirho(
                "dictionary",
                GROUND_DICTIONARY_LIMIT_CHIRHO,
            ));
        }
        Ok(())
    }

    /// The goal's weight, charged: it is hashed, memoized and copied at most a fixed number of
    /// times. A goal heavier than one goal may be is refused before anything copies it.
    pub(in crate::dict_chirho) fn goal_weight_chirho(
        &mut self,
        goal_chirho: &PredChirho,
    ) -> Result<usize, GroundRefusalChirho> {
        let weight_chirho =
            pred_weight_capped_chirho(goal_chirho, GROUND_GOAL_WEIGHT_LIMIT_CHIRHO).ok_or(
                GroundRefusalChirho::BudgetChirho("goal weight", GROUND_GOAL_WEIGHT_LIMIT_CHIRHO),
            )?;
        self.charge_chirho(weight_chirho)?;
        Ok(weight_chirho)
    }

    /// Preflight one candidate before the checker's matcher copies anything: the most its
    /// match and substituted context can copy is checked against the per-candidate limit and
    /// charged.
    pub(in crate::dict_chirho) fn candidate_chirho(
        &mut self,
        instance_chirho: &InstDeclChirho,
        goal_weight_chirho: usize,
    ) -> Result<(), GroundRefusalChirho> {
        let candidate_limit_chirho = GroundRefusalChirho::BudgetChirho(
            "candidate weight",
            GROUND_CANDIDATE_WEIGHT_LIMIT_CHIRHO,
        );
        let profile_chirho =
            pattern_profile_chirho(instance_chirho, GROUND_CANDIDATE_WEIGHT_LIMIT_CHIRHO)
                .ok_or_else(|| candidate_limit_chirho.clone())?;
        // A head variable's binding may be copied twice (bound, then composed); a context
        // variable once for each occurrence; the goal itself once as it is matched.
        let copies_chirho = profile_chirho
            .head_variables_chirho
            .saturating_mul(2)
            .saturating_add(profile_chirho.context_variables_chirho)
            .saturating_add(1);
        let bound_chirho = profile_chirho
            .fixed_weight_chirho
            .saturating_add(copies_chirho.saturating_mul(goal_weight_chirho));
        if bound_chirho > GROUND_CANDIDATE_WEIGHT_LIMIT_CHIRHO {
            return Err(candidate_limit_chirho);
        }
        self.charge_chirho(bound_chirho)
    }

    fn charge_chirho(&mut self, weight_chirho: usize) -> Result<(), GroundRefusalChirho> {
        self.type_work_chirho = self.type_work_chirho.saturating_add(weight_chirho);
        if self.type_work_chirho > GROUND_TYPE_WORK_LIMIT_CHIRHO {
            return Err(GroundRefusalChirho::BudgetChirho(
                "type work",
                GROUND_TYPE_WORK_LIMIT_CHIRHO,
            ));
        }
        Ok(())
    }

    /// A budget already spent, for the controls of what exhaustion refuses.
    #[cfg(test)]
    pub(in crate::dict_chirho) fn exhausted_chirho() -> Self {
        Self {
            type_work_chirho: GROUND_TYPE_WORK_LIMIT_CHIRHO,
            ..Self::default()
        }
    }
}

/// An instance's pattern: the weight of its own nodes and names, and how many variable
/// occurrences its head and its context hold.
struct PatternProfileChirho {
    fixed_weight_chirho: usize,
    head_variables_chirho: usize,
    context_variables_chirho: usize,
}

/// The pattern profile of an instance, or `None` once its weight passes `cap_chirho`.
fn pattern_profile_chirho(
    instance_chirho: &InstDeclChirho,
    cap_chirho: usize,
) -> Option<PatternProfileChirho> {
    let mut fixed_weight_chirho = 0;
    let mut head_variables_chirho = 0;
    let mut context_variables_chirho = 0;
    let heads_chirho = std::iter::once(&instance_chirho.head_ty_chirho)
        .chain(&instance_chirho.extra_head_tys_chirho)
        .map(|ty_chirho| (ty_chirho, true));
    let context_chirho = instance_chirho
        .context_chirho
        .iter()
        .flat_map(|pred_chirho| {
            std::iter::once(&pred_chirho.ty_chirho).chain(&pred_chirho.extra_tys_chirho)
        })
        .map(|ty_chirho| (ty_chirho, false));
    for (ty_chirho, in_head_chirho) in heads_chirho.chain(context_chirho) {
        let mut pending_chirho = vec![ty_chirho];
        while let Some(next_chirho) = pending_chirho.pop() {
            if let TyChirho::VarChirho(_) = next_chirho {
                if in_head_chirho {
                    head_variables_chirho += 1;
                } else {
                    context_variables_chirho += 1;
                }
            } else {
                fixed_weight_chirho += node_weight_chirho(next_chirho);
            }
            if fixed_weight_chirho + head_variables_chirho + context_variables_chirho > cap_chirho {
                return None;
            }
            push_children_chirho(next_chirho, &mut pending_chirho);
        }
    }
    Some(PatternProfileChirho {
        fixed_weight_chirho,
        head_variables_chirho,
        context_variables_chirho,
    })
}

/// A predicate's weight, extra class arguments included, or `None` once it passes
/// `cap_chirho`.
pub(in crate::dict_chirho) fn pred_weight_capped_chirho(
    pred_chirho: &PredChirho,
    cap_chirho: usize,
) -> Option<usize> {
    let mut weight_chirho = pred_chirho.class_name_chirho.len();
    for ty_chirho in std::iter::once(&pred_chirho.ty_chirho).chain(&pred_chirho.extra_tys_chirho) {
        weight_chirho +=
            ty_weight_capped_chirho(ty_chirho, cap_chirho.checked_sub(weight_chirho)?)?;
    }
    (weight_chirho <= cap_chirho).then_some(weight_chirho)
}

/// A type's weight, one per node plus the bytes of each name, or `None` once it passes
/// `cap_chirho`. The walk stops at the cap, so measuring is bounded too.
pub(in crate::dict_chirho) fn ty_weight_capped_chirho(
    ty_chirho: &TyChirho,
    cap_chirho: usize,
) -> Option<usize> {
    let mut weight_chirho = 0;
    let mut pending_chirho = vec![ty_chirho];
    while let Some(next_chirho) = pending_chirho.pop() {
        weight_chirho += node_weight_chirho(next_chirho);
        if weight_chirho > cap_chirho {
            return None;
        }
        push_children_chirho(next_chirho, &mut pending_chirho);
    }
    Some(weight_chirho)
}

/// One node, and the bytes of the name it carries.
fn node_weight_chirho(ty_chirho: &TyChirho) -> usize {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            1 + name_chirho.len()
        }
        _ => 1,
    }
}

fn push_children_chirho<'ty_chirho>(
    ty_chirho: &'ty_chirho TyChirho,
    pending_chirho: &mut Vec<&'ty_chirho TyChirho>,
) {
    match ty_chirho {
        TyChirho::AppChirho(function_chirho, argument_chirho)
        | TyChirho::FunChirho(function_chirho, argument_chirho, _) => {
            pending_chirho.push(function_chirho);
            pending_chirho.push(argument_chirho);
        }
        TyChirho::TupleChirho(elements_chirho) => pending_chirho.extend(elements_chirho),
        TyChirho::ListChirho(element_chirho) => pending_chirho.push(element_chirho),
        TyChirho::ForallChirho { body_chirho, .. }
        | TyChirho::RequiredForallChirho { body_chirho, .. } => pending_chirho.push(body_chirho),
        TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
    }
}
