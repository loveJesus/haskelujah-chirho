// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Strictness analysis and the worker/wrapper transform, phase 5 of the Core simplifier, moved from
//! `simplify_chirho.rs` (row 700).

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    InlineAnnotationChirho,
};
use haskelujah_span_chirho::SpanChirho;

// ---------------------------------------------------------------------------
// Phase 5: Strictness analysis & worker/wrapper transform
// ---------------------------------------------------------------------------

/// Demand on a function argument: how strictly is it used?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemandChirho {
    /// Argument is never forced (or we can't prove it's forced).
    LazyChirho,
    /// Argument is always forced to WHNF (e.g. via `case` scrutinee).
    StrictChirho,
}

/// Analyze the demand on lambda parameters of a function body.
/// Returns a list of demands, one per leading lambda binder.
fn analyze_demand_chirho(expr_chirho: &CoreExprChirho) -> Vec<(BinderChirho, DemandChirho)> {
    let mut demands_chirho = Vec::new();
    collect_lambda_demands_chirho(expr_chirho, &mut demands_chirho);
    demands_chirho
}

/// Peel off leading lambdas and check if each parameter is used strictly
/// in the body (appears as a case scrutinee or in a strict primop position).
fn collect_lambda_demands_chirho(
    expr_chirho: &CoreExprChirho,
    demands_chirho: &mut Vec<(BinderChirho, DemandChirho)>,
) {
    if let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = expr_chirho
    {
        let demand_chirho = if is_used_strictly_chirho(&binder_chirho.id_chirho, body_chirho) {
            DemandChirho::StrictChirho
        } else {
            DemandChirho::LazyChirho
        };
        demands_chirho.push((binder_chirho.clone(), demand_chirho));
        collect_lambda_demands_chirho(body_chirho, demands_chirho);
    }
}

/// Check if a variable is used strictly in an expression. A variable is
/// "used strictly" if it appears as:
/// - The scrutinee of a `case` expression
/// - An argument to a strict primitive operation (+#, -#, *#, etc.)
/// - The function position of an application (will be entered)
/// - Passed to `seq` (desugared as `case x of _ -> ...`)
fn is_used_strictly_chirho(var_id_chirho: &CoreIdChirho, expr_chirho: &CoreExprChirho) -> bool {
    match expr_chirho {
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            // If the scrutinee IS this variable, it's strict
            if matches!(scrutinee_chirho.as_ref(), CoreExprChirho::VarChirho(id_chirho) if id_chirho == var_id_chirho)
            {
                return true;
            }
            // Also check recursively in scrutinee and alt RHSes
            if is_used_strictly_chirho(var_id_chirho, scrutinee_chirho) {
                return true;
            }
            for alt_chirho in alts_chirho {
                if is_used_strictly_chirho(var_id_chirho, &alt_chirho.rhs_chirho) {
                    return true;
                }
            }
            false
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
            // All primop arguments are strict
            args_chirho.iter().any(|arg_chirho| {
                matches!(arg_chirho, CoreExprChirho::VarChirho(id_chirho) if id_chirho == var_id_chirho)
            })
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            // Check if strict in body or in any binding RHS
            if is_used_strictly_chirho(var_id_chirho, body_chirho) {
                return true;
            }
            for (_b_chirho, rhs_chirho) in binds_chirho {
                if is_used_strictly_chirho(var_id_chirho, rhs_chirho) {
                    return true;
                }
            }
            false
        }
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            // Function position is strict (will be entered)
            if matches!(fun_chirho.as_ref(), CoreExprChirho::VarChirho(id_chirho) if id_chirho == var_id_chirho)
            {
                return true;
            }
            is_used_strictly_chirho(var_id_chirho, fun_chirho)
                || is_used_strictly_chirho(var_id_chirho, arg_chirho)
        }
        CoreExprChirho::LamChirho { body_chirho, .. } => {
            // For multi-arg function analysis, look through remaining
            // lambdas since all args will be applied together
            is_used_strictly_chirho(var_id_chirho, body_chirho)
        }
        _ => false,
    }
}

/// Perform worker/wrapper transformation on bindings with strict arguments.
///
/// For a binding like:
///   `f = \x -> \y -> case x of { _ -> case y of { _ -> body } }`
/// where x and y are strict, generates:
///   `f = \x -> \y -> $wf x y`  (wrapper: evaluates args, calls worker)
///   `$wf = \x -> \y -> body`   (worker: assumes args are in WHNF)
///
/// In practice, the wrapper inserts `case` forcing for strict args before
/// calling the worker, and the worker skips the redundant `case` on those args.
pub fn worker_wrapper_chirho(bindings_chirho: Vec<CoreBindingChirho>) -> Vec<CoreBindingChirho> {
    let mut result_chirho = Vec::new();
    let mut next_id_chirho = bindings_chirho
        .iter()
        .map(|b_chirho| b_chirho.binder_chirho.id_chirho.0)
        .max()
        .unwrap_or(0)
        + 20000; // offset to avoid collisions

    for binding_chirho in &bindings_chirho {
        // Skip recursive bindings, NOINLINE bindings, and already-specialized bindings
        if binding_chirho.is_rec_chirho
            || binding_chirho.inline_chirho == InlineAnnotationChirho::NeverChirho
            || binding_chirho.binder_chirho.name_chirho.starts_with("$w")
            || binding_chirho
                .binder_chirho
                .name_chirho
                .starts_with("$spec_")
        {
            result_chirho.push(binding_chirho.clone());
            continue;
        }

        let demands_chirho = analyze_demand_chirho(&binding_chirho.rhs_chirho);

        // Only transform if at least one argument is strict
        let has_strict_chirho = demands_chirho
            .iter()
            .any(|(_, d_chirho)| *d_chirho == DemandChirho::StrictChirho);

        if !has_strict_chirho || demands_chirho.is_empty() {
            result_chirho.push(binding_chirho.clone());
            continue;
        }

        // Create worker binding: strip the leading lambdas and
        // give it a $w prefix name
        let worker_name_chirho = format!("$w{}", binding_chirho.binder_chirho.name_chirho);
        let worker_id_chirho = CoreIdChirho(next_id_chirho);
        next_id_chirho += 1;

        // The worker body is the original body with leading lambdas intact
        // (the worker still takes the same args but callers will have forced them)
        let worker_binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: worker_id_chirho,
                name_chirho: worker_name_chirho,
                ty_chirho: binding_chirho.binder_chirho.ty_chirho.clone(),
                span_chirho: binding_chirho.binder_chirho.span_chirho,
            },
            rhs_chirho: binding_chirho.rhs_chirho.clone(),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::AlwaysChirho,
        };

        // Create wrapper: re-bind with case forcing for strict args, then call worker
        let mut wrapper_body_chirho = build_worker_call_chirho(worker_id_chirho, &demands_chirho);
        // Wrap in case-forcing for strict args (innermost first)
        for (binder_chirho, demand_chirho) in demands_chirho.iter().rev() {
            if *demand_chirho == DemandChirho::StrictChirho {
                // case x of { _ -> <inner> }
                wrapper_body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(binder_chirho.id_chirho)),
                    bind_chirho: BinderChirho {
                        id_chirho: CoreIdChirho(next_id_chirho),
                        name_chirho: "_ww".to_string(),
                        ty_chirho: binder_chirho.ty_chirho.clone(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    result_ty_chirho: binding_chirho.binder_chirho.ty_chirho.clone(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: wrapper_body_chirho,
                    }],
                };
                next_id_chirho += 1;
            }
        }
        // Wrap in lambdas for the parameters
        for (binder_chirho, _) in demands_chirho.iter().rev() {
            wrapper_body_chirho = CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho.clone(),
                body_chirho: Box::new(wrapper_body_chirho),
            };
        }

        let wrapper_binding_chirho = CoreBindingChirho {
            binder_chirho: binding_chirho.binder_chirho.clone(),
            rhs_chirho: wrapper_body_chirho,
            is_rec_chirho: false,
            inline_chirho: binding_chirho.inline_chirho.clone(),
        };

        result_chirho.push(wrapper_binding_chirho);
        result_chirho.push(worker_binding_chirho);
    }

    result_chirho
}

/// Build a call to the worker function: `$wf x1 x2 ... xn`
fn build_worker_call_chirho(
    worker_id_chirho: CoreIdChirho,
    demands_chirho: &[(BinderChirho, DemandChirho)],
) -> CoreExprChirho {
    let mut call_chirho = CoreExprChirho::VarChirho(worker_id_chirho);
    for (binder_chirho, _) in demands_chirho {
        call_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(call_chirho),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(binder_chirho.id_chirho)),
        };
    }
    call_chirho
}

#[cfg(test)]
#[path = "strictness_tests_chirho.rs"]
mod tests_chirho;
