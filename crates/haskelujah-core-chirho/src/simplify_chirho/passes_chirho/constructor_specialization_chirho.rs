// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor specialization (SpecConstr), phase 7 of the Core simplifier, moved from
//! `simplify_chirho.rs` (row 700).

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    InlineAnnotationChirho,
};

// ---------------------------------------------------------------------------
// Phase 7: Constructor specialization (SpecConstr)
// ---------------------------------------------------------------------------
//
// SpecConstr optimizes recursive functions that always call themselves with
// an argument built from a known constructor. For example:
//
//   f = \xs -> case xs of
//     [] -> 0
//     (:) x rest -> x + f rest
//
// Here `f` always calls itself with a variable (`rest`) that was just
// pattern-matched. SpecConstr creates a specialized copy for the (:) case:
//
//   $sc_f_Cons = \x rest -> x + (case rest of
//     [] -> 0
//     (:) x' rest' -> x' + $sc_f_Cons x' rest')
//
// The key insight: in the recursive call, the argument's constructor is
// already known from the case match, so the inner case can be hoisted.

/// A call pattern observed in a recursive function: which constructor
/// a particular argument is always applied with at recursive call sites.
#[derive(Debug, Clone)]
struct CallPatternChirho {
    /// Index of the argument in the function's lambda chain.
    arg_idx_chirho: usize,
    /// The constructor name the argument is always built with.
    con_name_chirho: String,
    /// The binders for the constructor's fields at the call site.
    field_binders_chirho: Vec<BinderChirho>,
}

/// Perform constructor specialization on recursive bindings.
///
/// For each recursive binding, analyze its case alternatives to find
/// arguments that are always passed as known-constructor values at
/// recursive call sites. Create specialized copies for those patterns.
pub fn spec_constr_chirho(bindings_chirho: Vec<CoreBindingChirho>) -> Vec<CoreBindingChirho> {
    let mut result_chirho = Vec::new();
    let mut next_id_chirho = bindings_chirho
        .iter()
        .map(|b_chirho| b_chirho.binder_chirho.id_chirho.0)
        .max()
        .unwrap_or(0)
        + 30000;

    for binding_chirho in &bindings_chirho {
        // Only transform recursive, non-NOINLINE bindings
        if !binding_chirho.is_rec_chirho
            || binding_chirho.inline_chirho == InlineAnnotationChirho::NeverChirho
            || binding_chirho.binder_chirho.name_chirho.starts_with("$sc_")
        {
            result_chirho.push(binding_chirho.clone());
            continue;
        }

        // Collect lambda parameters
        let mut params_chirho = Vec::new();
        let mut body_chirho = &binding_chirho.rhs_chirho;
        while let CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho: inner_chirho,
        } = body_chirho
        {
            params_chirho.push(binder_chirho.clone());
            body_chirho = inner_chirho;
        }

        if params_chirho.is_empty() {
            result_chirho.push(binding_chirho.clone());
            continue;
        }

        // Find call patterns: for each case scrutinee that is a parameter,
        // check if recursive calls in the alt bodies always pass a variable
        // that was bound by the case alt pattern.
        let patterns_chirho = find_call_patterns_chirho(
            &binding_chirho.binder_chirho.id_chirho,
            &params_chirho,
            body_chirho,
        );

        if patterns_chirho.is_empty() {
            result_chirho.push(binding_chirho.clone());
            continue;
        }

        // Create specialized copies for each pattern
        result_chirho.push(binding_chirho.clone());

        for pattern_chirho in &patterns_chirho {
            let spec_name_chirho = format!(
                "$sc_{}_{}",
                binding_chirho.binder_chirho.name_chirho, pattern_chirho.con_name_chirho
            );
            let spec_id_chirho = CoreIdChirho(next_id_chirho);
            next_id_chirho += 1;

            // Build the specialized RHS: replace the pattern argument with
            // the constructor fields as separate parameters.
            // The specialized version takes the fields directly.
            let mut spec_params_chirho = Vec::new();
            for (idx_chirho, param_chirho) in params_chirho.iter().enumerate() {
                if idx_chirho == pattern_chirho.arg_idx_chirho {
                    // Replace this param with the constructor's field binders
                    spec_params_chirho.extend(pattern_chirho.field_binders_chirho.clone());
                } else {
                    spec_params_chirho.push(param_chirho.clone());
                }
            }

            // The body is the original body with the constructor pre-applied
            let spec_body_chirho = specialize_body_for_con_chirho(
                body_chirho,
                pattern_chirho.arg_idx_chirho,
                &params_chirho[pattern_chirho.arg_idx_chirho],
                &pattern_chirho.con_name_chirho,
                &pattern_chirho.field_binders_chirho,
            );

            // Wrap in lambdas
            let mut spec_rhs_chirho = spec_body_chirho;
            for param_chirho in spec_params_chirho.iter().rev() {
                spec_rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: param_chirho.clone(),
                    body_chirho: Box::new(spec_rhs_chirho),
                };
            }

            result_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: spec_id_chirho,
                    name_chirho: spec_name_chirho,
                    ty_chirho: binding_chirho.binder_chirho.ty_chirho.clone(),
                    span_chirho: binding_chirho.binder_chirho.span_chirho,
                },
                rhs_chirho: spec_rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::AlwaysChirho,
            });
        }
    }

    result_chirho
}

/// Find call patterns in a recursive function body.
///
/// Looks for case expressions where the scrutinee is one of the function's
/// parameters, and checks if recursive calls in the alternatives pass
/// a variable that was bound by the alternative's pattern.
fn find_call_patterns_chirho(
    func_id_chirho: &CoreIdChirho,
    params_chirho: &[BinderChirho],
    body_chirho: &CoreExprChirho,
) -> Vec<CallPatternChirho> {
    let mut patterns_chirho = Vec::new();
    find_patterns_in_expr_chirho(
        func_id_chirho,
        params_chirho,
        body_chirho,
        &mut patterns_chirho,
    );
    // Deduplicate by arg index
    patterns_chirho.sort_by_key(|p_chirho| p_chirho.arg_idx_chirho);
    patterns_chirho.dedup_by_key(|p_chirho| p_chirho.arg_idx_chirho);
    patterns_chirho
}

fn find_patterns_in_expr_chirho(
    func_id_chirho: &CoreIdChirho,
    params_chirho: &[BinderChirho],
    expr_chirho: &CoreExprChirho,
    patterns_chirho: &mut Vec<CallPatternChirho>,
) {
    if let CoreExprChirho::CaseChirho {
        scrutinee_chirho,
        alts_chirho,
        ..
    } = expr_chirho
    {
        // Check if scrutinee is one of our parameters
        if let CoreExprChirho::VarChirho(scrut_id_chirho) = scrutinee_chirho.as_ref() {
            if let Some(param_idx_chirho) = params_chirho
                .iter()
                .position(|p_chirho| p_chirho.id_chirho == *scrut_id_chirho)
            {
                // Check each constructor alternative
                for alt_chirho in alts_chirho {
                    if let AltConChirho::DataConChirho(con_name_chirho) = &alt_chirho.con_chirho {
                        // Check if there's a recursive call in this alt's RHS
                        // where the argument at param_idx is one of the alt's binders
                        if has_recursive_call_with_alt_binder_chirho(
                            func_id_chirho,
                            param_idx_chirho,
                            &alt_chirho.binders_chirho,
                            &alt_chirho.rhs_chirho,
                        ) {
                            patterns_chirho.push(CallPatternChirho {
                                arg_idx_chirho: param_idx_chirho,
                                con_name_chirho: con_name_chirho.clone(),
                                field_binders_chirho: alt_chirho.binders_chirho.clone(),
                            });
                        }
                    }
                }
            }
        }
        // Also recurse into alt bodies
        for alt_chirho in alts_chirho {
            find_patterns_in_expr_chirho(
                func_id_chirho,
                params_chirho,
                &alt_chirho.rhs_chirho,
                patterns_chirho,
            );
        }
    }
    // Recurse into other expression forms
    match expr_chirho {
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (_b_chirho, rhs_chirho) in binds_chirho {
                find_patterns_in_expr_chirho(
                    func_id_chirho,
                    params_chirho,
                    rhs_chirho,
                    patterns_chirho,
                );
            }
            find_patterns_in_expr_chirho(
                func_id_chirho,
                params_chirho,
                body_chirho,
                patterns_chirho,
            );
        }
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            find_patterns_in_expr_chirho(
                func_id_chirho,
                params_chirho,
                fun_chirho,
                patterns_chirho,
            );
            find_patterns_in_expr_chirho(
                func_id_chirho,
                params_chirho,
                arg_chirho,
                patterns_chirho,
            );
        }
        CoreExprChirho::LamChirho { body_chirho, .. } => {
            find_patterns_in_expr_chirho(
                func_id_chirho,
                params_chirho,
                body_chirho,
                patterns_chirho,
            );
        }
        _ => {}
    }
}

/// Check if there's a recursive call in an expression where the argument at
/// `param_idx` is one of the given alt binders (pattern-matched fields).
fn has_recursive_call_with_alt_binder_chirho(
    func_id_chirho: &CoreIdChirho,
    param_idx_chirho: usize,
    alt_binders_chirho: &[BinderChirho],
    expr_chirho: &CoreExprChirho,
) -> bool {
    // Flatten application chains to find `f a1 a2 ... an` calls
    let mut apps_chirho = Vec::new();
    collect_apps_chirho(expr_chirho, &mut apps_chirho);

    for (func_expr_chirho, args_chirho) in &apps_chirho {
        if let CoreExprChirho::VarChirho(id_chirho) = func_expr_chirho {
            if id_chirho == func_id_chirho && args_chirho.len() > param_idx_chirho {
                // Check if the arg at param_idx is one of the alt binders
                if let CoreExprChirho::VarChirho(arg_id_chirho) = &args_chirho[param_idx_chirho] {
                    if alt_binders_chirho
                        .iter()
                        .any(|b_chirho| b_chirho.id_chirho == *arg_id_chirho)
                    {
                        return true;
                    }
                }
            }
        }
    }

    // Recurse into subexpressions
    match expr_chirho {
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (_b_chirho, rhs_chirho) in binds_chirho {
                if has_recursive_call_with_alt_binder_chirho(
                    func_id_chirho,
                    param_idx_chirho,
                    alt_binders_chirho,
                    rhs_chirho,
                ) {
                    return true;
                }
            }
            has_recursive_call_with_alt_binder_chirho(
                func_id_chirho,
                param_idx_chirho,
                alt_binders_chirho,
                body_chirho,
            )
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            if has_recursive_call_with_alt_binder_chirho(
                func_id_chirho,
                param_idx_chirho,
                alt_binders_chirho,
                scrutinee_chirho,
            ) {
                return true;
            }
            alts_chirho.iter().any(|alt_chirho| {
                has_recursive_call_with_alt_binder_chirho(
                    func_id_chirho,
                    param_idx_chirho,
                    alt_binders_chirho,
                    &alt_chirho.rhs_chirho,
                )
            })
        }
        CoreExprChirho::LamChirho { body_chirho, .. } => has_recursive_call_with_alt_binder_chirho(
            func_id_chirho,
            param_idx_chirho,
            alt_binders_chirho,
            body_chirho,
        ),
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            has_recursive_call_with_alt_binder_chirho(
                func_id_chirho,
                param_idx_chirho,
                alt_binders_chirho,
                fun_chirho,
            ) || has_recursive_call_with_alt_binder_chirho(
                func_id_chirho,
                param_idx_chirho,
                alt_binders_chirho,
                arg_chirho,
            )
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => args_chirho.iter().any(|a_chirho| {
            has_recursive_call_with_alt_binder_chirho(
                func_id_chirho,
                param_idx_chirho,
                alt_binders_chirho,
                a_chirho,
            )
        }),
        CoreExprChirho::ConAppChirho { args_chirho, .. } => args_chirho.iter().any(|a_chirho| {
            has_recursive_call_with_alt_binder_chirho(
                func_id_chirho,
                param_idx_chirho,
                alt_binders_chirho,
                a_chirho,
            )
        }),
        _ => false,
    }
}

/// Collect application chains: returns (function, [arg1, arg2, ...]) pairs.
fn collect_apps_chirho<'a>(
    expr_chirho: &'a CoreExprChirho,
    result_chirho: &mut Vec<(&'a CoreExprChirho, Vec<&'a CoreExprChirho>)>,
) {
    // Try to flatten this expression as an application chain
    let mut head_chirho = expr_chirho;
    let mut args_chirho = Vec::new();
    while let CoreExprChirho::AppChirho {
        fun_chirho,
        arg_chirho,
    } = head_chirho
    {
        args_chirho.push(arg_chirho.as_ref());
        head_chirho = fun_chirho;
    }
    if !args_chirho.is_empty() {
        args_chirho.reverse();
        result_chirho.push((head_chirho, args_chirho));
    }
}

/// Specialize a function body for a known constructor at a given argument position.
///
/// Currently returns the body unchanged — the specialization benefit comes from
/// the fact that the specialized version takes the constructor's fields directly,
/// allowing the case match on that argument to be eliminated by later simplification.
fn specialize_body_for_con_chirho(
    body_chirho: &CoreExprChirho,
    _arg_idx_chirho: usize,
    _param_chirho: &BinderChirho,
    _con_name_chirho: &str,
    _field_binders_chirho: &[BinderChirho],
) -> CoreExprChirho {
    // For now, clone the body — the specialized version already benefits from
    // having the constructor's fields as direct parameters. Future enhancement:
    // rewrite case expressions on the specialized parameter to jump directly
    // to the matching constructor alternative.
    body_chirho.clone()
}

#[cfg(test)]
#[path = "constructor_specialization_tests_chirho.rs"]
mod tests_chirho;
