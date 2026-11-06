// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Absence analysis and dead argument elimination, phase 6 of the Core simplifier, moved from
//! `simplify_chirho.rs` (row 700).

use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};

// ---------------------------------------------------------------------------
// Phase 6: Demand analysis — absence analysis & dead argument elimination
// ---------------------------------------------------------------------------
// (see also Phase 7: Constructor specialization below)

/// Usage count for a variable in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageChirho {
    /// Variable is never referenced.
    AbsentChirho,
    /// Variable is referenced exactly once.
    UsedOnceChirho,
    /// Variable is referenced multiple times.
    UsedManyChirho,
}

/// Count how many times a variable is used in an expression.
pub fn count_usage_chirho(
    var_id_chirho: &CoreIdChirho,
    expr_chirho: &CoreExprChirho,
) -> UsageChirho {
    let count_chirho = count_var_occurrences_chirho(var_id_chirho, expr_chirho);
    match count_chirho {
        0 => UsageChirho::AbsentChirho,
        1 => UsageChirho::UsedOnceChirho,
        _ => UsageChirho::UsedManyChirho,
    }
}

/// Count raw occurrences of a variable in a Core expression.
fn count_var_occurrences_chirho(
    var_id_chirho: &CoreIdChirho,
    expr_chirho: &CoreExprChirho,
) -> usize {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if id_chirho == var_id_chirho {
                1
            } else {
                0
            }
        }
        CoreExprChirho::LitChirho(_) => 0,
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            count_var_occurrences_chirho(var_id_chirho, fun_chirho)
                + count_var_occurrences_chirho(var_id_chirho, arg_chirho)
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            // If the lambda shadows our variable, stop counting
            if binder_chirho.id_chirho == *var_id_chirho {
                0
            } else {
                count_var_occurrences_chirho(var_id_chirho, body_chirho)
            }
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            rec_chirho,
        } => {
            let mut total_chirho = 0;
            let shadowed_in_body_chirho = binds_chirho
                .iter()
                .any(|(b_chirho, _)| b_chirho.id_chirho == *var_id_chirho);
            // In non-rec let, binders scope over body only (not RHS).
            // In rec let, binders scope over both body and all RHSes.
            if !shadowed_in_body_chirho {
                total_chirho += count_var_occurrences_chirho(var_id_chirho, body_chirho);
            }
            for (b_chirho, rhs_chirho) in binds_chirho {
                // In rec bindings, the binder shadows in its own RHS too
                if *rec_chirho && b_chirho.id_chirho == *var_id_chirho {
                    continue;
                }
                total_chirho += count_var_occurrences_chirho(var_id_chirho, rhs_chirho);
            }
            total_chirho
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            let mut total_chirho = count_var_occurrences_chirho(var_id_chirho, scrutinee_chirho);
            if bind_chirho.id_chirho != *var_id_chirho {
                for alt_chirho in alts_chirho {
                    let shadowed_chirho = alt_chirho
                        .binders_chirho
                        .iter()
                        .any(|b_chirho| b_chirho.id_chirho == *var_id_chirho);
                    if !shadowed_chirho {
                        total_chirho +=
                            count_var_occurrences_chirho(var_id_chirho, &alt_chirho.rhs_chirho);
                    }
                }
            }
            total_chirho
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => args_chirho
            .iter()
            .map(|a_chirho| count_var_occurrences_chirho(var_id_chirho, a_chirho))
            .sum(),
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            count_var_occurrences_chirho(var_id_chirho, body_chirho)
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            count_var_occurrences_chirho(var_id_chirho, expr_chirho)
        }
        CoreExprChirho::ConAppChirho { args_chirho, .. } => args_chirho
            .iter()
            .map(|a_chirho| count_var_occurrences_chirho(var_id_chirho, a_chirho))
            .sum(),
    }
}

/// Analyze usage of each lambda parameter in a function body.
/// Returns (binder, usage) for each leading lambda.
pub fn analyze_usage_chirho(expr_chirho: &CoreExprChirho) -> Vec<(BinderChirho, UsageChirho)> {
    let mut result_chirho = Vec::new();
    collect_lambda_usage_chirho(expr_chirho, &mut result_chirho);
    result_chirho
}

fn collect_lambda_usage_chirho(
    expr_chirho: &CoreExprChirho,
    result_chirho: &mut Vec<(BinderChirho, UsageChirho)>,
) {
    if let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = expr_chirho
    {
        let usage_chirho = count_usage_in_full_body_chirho(&binder_chirho.id_chirho, body_chirho);
        result_chirho.push((binder_chirho.clone(), usage_chirho));
        collect_lambda_usage_chirho(body_chirho, result_chirho);
    }
}

/// Count usage looking through nested lambdas (for multi-arg function analysis).
fn count_usage_in_full_body_chirho(
    var_id_chirho: &CoreIdChirho,
    expr_chirho: &CoreExprChirho,
) -> UsageChirho {
    // Peel off remaining lambdas and count in the innermost body
    let mut body_chirho = expr_chirho;
    while let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho: inner_chirho,
    } = body_chirho
    {
        if binder_chirho.id_chirho == *var_id_chirho {
            return UsageChirho::AbsentChirho; // shadowed
        }
        body_chirho = inner_chirho;
    }
    count_usage_chirho(var_id_chirho, body_chirho)
}

/// Dead argument elimination: remove unused (absent) arguments from function bindings.
///
/// For a binding like `f = \x -> \y -> \z -> x + z` where y is absent:
///   - Rewrites to `f = \x -> \z -> x + z` (drops the unused y parameter)
///   - This enables further optimization: callers passing arguments to dropped
///     positions can be simplified by later inlining/simplification passes.
///
/// Only eliminates trailing absent args or interior absent args when safe.
/// Skips recursive, NOINLINE, and already-transformed bindings.
pub fn dead_arg_elimination_chirho(
    bindings_chirho: Vec<CoreBindingChirho>,
) -> Vec<CoreBindingChirho> {
    bindings_chirho
        .into_iter()
        .map(|binding_chirho| {
            // Skip recursive, NOINLINE, and internal bindings
            if binding_chirho.is_rec_chirho
                || binding_chirho.inline_chirho == InlineAnnotationChirho::NeverChirho
                || binding_chirho.binder_chirho.name_chirho.starts_with("$w")
                || binding_chirho
                    .binder_chirho
                    .name_chirho
                    .starts_with("$spec_")
                || binding_chirho
                    .binder_chirho
                    .name_chirho
                    .starts_with("$dae_")
            {
                return binding_chirho;
            }

            let usage_chirho = analyze_usage_chirho(&binding_chirho.rhs_chirho);

            // Check if any trailing arguments are absent
            let has_absent_chirho = usage_chirho
                .iter()
                .any(|(_, u_chirho)| *u_chirho == UsageChirho::AbsentChirho);

            if !has_absent_chirho || usage_chirho.is_empty() {
                return binding_chirho;
            }

            // Rebuild the lambda chain, dropping absent parameters
            let inner_body_chirho =
                peel_lambdas_chirho(&binding_chirho.rhs_chirho, usage_chirho.len());
            let mut new_body_chirho = inner_body_chirho.clone();

            // Wrap back in lambdas for non-absent params (in reverse order)
            for (binder_chirho, u_chirho) in usage_chirho.iter().rev() {
                if *u_chirho != UsageChirho::AbsentChirho {
                    new_body_chirho = CoreExprChirho::LamChirho {
                        binder_chirho: binder_chirho.clone(),
                        body_chirho: Box::new(new_body_chirho),
                    };
                }
            }

            CoreBindingChirho {
                rhs_chirho: new_body_chirho,
                ..binding_chirho
            }
        })
        .collect()
}

/// Peel off `n` leading lambda abstractions and return the inner body.
fn peel_lambdas_chirho(expr_chirho: &CoreExprChirho, n_chirho: usize) -> &CoreExprChirho {
    if n_chirho == 0 {
        return expr_chirho;
    }
    if let CoreExprChirho::LamChirho { body_chirho, .. } = expr_chirho {
        peel_lambdas_chirho(body_chirho, n_chirho - 1)
    } else {
        expr_chirho
    }
}

#[cfg(test)]
#[path = "dead_arguments_tests_chirho.rs"]
mod tests_chirho;
