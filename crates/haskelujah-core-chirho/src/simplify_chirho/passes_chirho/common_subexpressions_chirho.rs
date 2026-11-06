// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Common subexpression elimination (CSE), phase 3 of the Core simplifier, moved from
//! `simplify_chirho.rs` (row 700).

use super::is_trivial_chirho;
use crate::expr_chirho::{
    CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Common Subexpression Elimination (CSE)
// ---------------------------------------------------------------------------

/// Top-level CSE: deduplicate bindings with identical RHS expressions.
///
/// If two non-recursive bindings `x = e` and `y = e` have the same RHS,
/// redirect all uses of `y` to `x` (replace `y` with `Var(x)` in its RHS)
/// and let dead-binding elimination clean up `y` in the next iteration.
pub(super) fn cse_top_level_chirho(
    bindings_chirho: Vec<CoreBindingChirho>,
) -> Vec<CoreBindingChirho> {
    // Build redirect map: for each pair of bindings with identical non-trivial RHS,
    // the later one should redirect to the earlier one.
    let mut redirect_chirho: HashMap<CoreIdChirho, CoreIdChirho> = HashMap::new();

    for i_chirho in 0..bindings_chirho.len() {
        if bindings_chirho[i_chirho].is_rec_chirho {
            continue;
        }
        // Skip trivial RHSes (Var, Lit) — not worth deduplicating
        if is_trivial_chirho(&bindings_chirho[i_chirho].rhs_chirho) {
            continue;
        }
        // Skip INLINE-annotated bindings — they are meant to be expanded
        if bindings_chirho[i_chirho].inline_chirho == InlineAnnotationChirho::AlwaysChirho
            || bindings_chirho[i_chirho].inline_chirho == InlineAnnotationChirho::InlinableChirho
        {
            continue;
        }
        // Skip already-redirected bindings
        if redirect_chirho.contains_key(&bindings_chirho[i_chirho].binder_chirho.id_chirho) {
            continue;
        }
        for j_chirho in (i_chirho + 1)..bindings_chirho.len() {
            if bindings_chirho[j_chirho].is_rec_chirho {
                continue;
            }
            if redirect_chirho.contains_key(&bindings_chirho[j_chirho].binder_chirho.id_chirho) {
                continue;
            }
            // Skip if either binding has INLINE/INLINABLE annotation
            if bindings_chirho[j_chirho].inline_chirho == InlineAnnotationChirho::AlwaysChirho
                || bindings_chirho[j_chirho].inline_chirho
                    == InlineAnnotationChirho::InlinableChirho
            {
                continue;
            }
            if bindings_chirho[i_chirho].rhs_chirho == bindings_chirho[j_chirho].rhs_chirho {
                redirect_chirho.insert(
                    bindings_chirho[j_chirho].binder_chirho.id_chirho,
                    bindings_chirho[i_chirho].binder_chirho.id_chirho,
                );
            }
        }
    }

    if redirect_chirho.is_empty() {
        return bindings_chirho;
    }

    // Apply redirections: replace redirected bindings' RHS with a Var reference,
    // and rewrite all uses in all other bindings.
    bindings_chirho
        .into_iter()
        .map(|mut b_chirho| {
            if let Some(canonical_chirho) = redirect_chirho.get(&b_chirho.binder_chirho.id_chirho) {
                // This binding is a duplicate — redirect its RHS to the canonical one
                b_chirho.rhs_chirho = CoreExprChirho::VarChirho(*canonical_chirho);
            } else {
                // Rewrite references in this binding's RHS
                b_chirho.rhs_chirho =
                    apply_cse_redirects_chirho(&b_chirho.rhs_chirho, &redirect_chirho);
            }
            b_chirho
        })
        .collect()
}

/// Rewrite variable references according to CSE redirections.
fn apply_cse_redirects_chirho(
    expr_chirho: &CoreExprChirho,
    redirects_chirho: &HashMap<CoreIdChirho, CoreIdChirho>,
) -> CoreExprChirho {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if let Some(canonical_chirho) = redirects_chirho.get(id_chirho) {
                CoreExprChirho::VarChirho(*canonical_chirho)
            } else {
                expr_chirho.clone()
            }
        }
        CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => CoreExprChirho::AppChirho {
            fun_chirho: Box::new(apply_cse_redirects_chirho(fun_chirho, redirects_chirho)),
            arg_chirho: Box::new(apply_cse_redirects_chirho(arg_chirho, redirects_chirho)),
        },
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => CoreExprChirho::LamChirho {
            binder_chirho: binder_chirho.clone(),
            body_chirho: Box::new(apply_cse_redirects_chirho(body_chirho, redirects_chirho)),
        },
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => CoreExprChirho::LetChirho {
            rec_chirho: *rec_chirho,
            binds_chirho: binds_chirho
                .iter()
                .map(|(b_chirho, e_chirho)| {
                    (
                        b_chirho.clone(),
                        apply_cse_redirects_chirho(e_chirho, redirects_chirho),
                    )
                })
                .collect(),
            body_chirho: Box::new(apply_cse_redirects_chirho(body_chirho, redirects_chirho)),
        },
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(apply_cse_redirects_chirho(
                scrutinee_chirho,
                redirects_chirho,
            )),
            bind_chirho: bind_chirho.clone(),
            result_ty_chirho: result_ty_chirho.clone(),
            alts_chirho: alts_chirho
                .iter()
                .map(|alt_chirho| CoreAltChirho {
                    con_chirho: alt_chirho.con_chirho.clone(),
                    binders_chirho: alt_chirho.binders_chirho.clone(),
                    rhs_chirho: apply_cse_redirects_chirho(
                        &alt_chirho.rhs_chirho,
                        redirects_chirho,
                    ),
                })
                .collect(),
        },
        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => CoreExprChirho::TyLamChirho {
            ty_var_chirho: ty_var_chirho.clone(),
            body_chirho: Box::new(apply_cse_redirects_chirho(body_chirho, redirects_chirho)),
        },
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(apply_cse_redirects_chirho(inner_chirho, redirects_chirho)),
            ty_chirho: ty_chirho.clone(),
        },
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| apply_cse_redirects_chirho(a_chirho, redirects_chirho))
                .collect(),
        },
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| apply_cse_redirects_chirho(a_chirho, redirects_chirho))
                .collect(),
        },
    }
}

/// Intra-expression CSE: within `let` blocks, deduplicate bindings with
/// identical non-trivial RHS expressions.
///
/// ```text
/// let x = expensive_expr     let x = expensive_expr
///     y = expensive_expr  →      y = x          -- y redirects to x
/// in f x y                   in f x y
/// ```
pub(super) fn cse_expr_chirho(expr_chirho: &CoreExprChirho) -> CoreExprChirho {
    match expr_chirho {
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => {
            // First, recurse into each binding's RHS and the body
            let simplified_binds_chirho: Vec<(crate::expr_chirho::BinderChirho, CoreExprChirho)> =
                binds_chirho
                    .iter()
                    .map(|(b_chirho, e_chirho)| (b_chirho.clone(), cse_expr_chirho(e_chirho)))
                    .collect();
            let simplified_body_chirho = cse_expr_chirho(body_chirho);

            // For non-recursive lets, find bindings with identical non-trivial RHS
            if !rec_chirho {
                let mut redirect_chirho: HashMap<CoreIdChirho, CoreIdChirho> = HashMap::new();
                for i_chirho in 0..simplified_binds_chirho.len() {
                    if is_trivial_chirho(&simplified_binds_chirho[i_chirho].1) {
                        continue;
                    }
                    if redirect_chirho.contains_key(&simplified_binds_chirho[i_chirho].0.id_chirho)
                    {
                        continue;
                    }
                    for j_chirho in (i_chirho + 1)..simplified_binds_chirho.len() {
                        if redirect_chirho
                            .contains_key(&simplified_binds_chirho[j_chirho].0.id_chirho)
                        {
                            continue;
                        }
                        if simplified_binds_chirho[i_chirho].1
                            == simplified_binds_chirho[j_chirho].1
                        {
                            redirect_chirho.insert(
                                simplified_binds_chirho[j_chirho].0.id_chirho,
                                simplified_binds_chirho[i_chirho].0.id_chirho,
                            );
                        }
                    }
                }

                if !redirect_chirho.is_empty() {
                    let new_binds_chirho: Vec<_> = simplified_binds_chirho
                        .into_iter()
                        .map(|(b_chirho, e_chirho)| {
                            if let Some(canonical_chirho) = redirect_chirho.get(&b_chirho.id_chirho)
                            {
                                (b_chirho, CoreExprChirho::VarChirho(*canonical_chirho))
                            } else {
                                (
                                    b_chirho,
                                    apply_cse_redirects_chirho(&e_chirho, &redirect_chirho),
                                )
                            }
                        })
                        .collect();
                    let new_body_chirho =
                        apply_cse_redirects_chirho(&simplified_body_chirho, &redirect_chirho);
                    return CoreExprChirho::LetChirho {
                        rec_chirho: false,
                        binds_chirho: new_binds_chirho,
                        body_chirho: Box::new(new_body_chirho),
                    };
                }
            }

            CoreExprChirho::LetChirho {
                rec_chirho: *rec_chirho,
                binds_chirho: simplified_binds_chirho,
                body_chirho: Box::new(simplified_body_chirho),
            }
        }

        // Recurse into all subexpressions
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => CoreExprChirho::AppChirho {
            fun_chirho: Box::new(cse_expr_chirho(fun_chirho)),
            arg_chirho: Box::new(cse_expr_chirho(arg_chirho)),
        },
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => CoreExprChirho::LamChirho {
            binder_chirho: binder_chirho.clone(),
            body_chirho: Box::new(cse_expr_chirho(body_chirho)),
        },
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(cse_expr_chirho(scrutinee_chirho)),
            bind_chirho: bind_chirho.clone(),
            result_ty_chirho: result_ty_chirho.clone(),
            alts_chirho: alts_chirho
                .iter()
                .map(|alt_chirho| CoreAltChirho {
                    con_chirho: alt_chirho.con_chirho.clone(),
                    binders_chirho: alt_chirho.binders_chirho.clone(),
                    rhs_chirho: cse_expr_chirho(&alt_chirho.rhs_chirho),
                })
                .collect(),
        },
        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => CoreExprChirho::TyLamChirho {
            ty_var_chirho: ty_var_chirho.clone(),
            body_chirho: Box::new(cse_expr_chirho(body_chirho)),
        },
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(cse_expr_chirho(inner_chirho)),
            ty_chirho: ty_chirho.clone(),
        },
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| cse_expr_chirho(a_chirho))
                .collect(),
        },
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| cse_expr_chirho(a_chirho))
                .collect(),
        },
        // Leaves — no transformation needed
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
    }
}

#[cfg(test)]
#[path = "common_subexpressions_tests_chirho.rs"]
mod tests_chirho;
