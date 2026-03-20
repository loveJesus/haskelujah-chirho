// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Core-to-Core simplifier
//!
//! A simple optimization pass over Core expressions. Implements:
//! - **Beta reduction**: `(\x -> body) arg` → `body[x := arg]`
//! - **Dead binding elimination**: remove `let x = rhs in body` when `x` is
//!   unused in `body`
//! - **Case-of-known-constructor**: `case Con args of { Con xs -> rhs; ... }`
//!   → `rhs[xs := args]`
//! - **Constant folding**: evaluate known integer/bool operations at compile
//!   time
//! - **Inlining**: inline small non-recursive bindings and `{-# INLINE #-}`
//!   annotated bindings at call sites; respect `{-# NOINLINE #-}`
//! - **Common subexpression elimination (CSE)**: deduplicate identical
//!   subexpressions within let-blocks and across top-level bindings
//!
//! Modelled after GHC's simplifier but drastically reduced in scope. Runs a
//! fixed number of iterations (configurable).

use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};

/// Maximum expression size (AST nodes) for auto-inlining.
const AUTO_INLINE_THRESHOLD_CHIRHO: usize = 10;

/// Configuration for the simplifier.
#[derive(Debug, Clone)]
pub struct SimplifyConfigChirho {
    /// Maximum number of simplification passes.
    pub max_iterations_chirho: usize,
    /// Size threshold for automatic inlining of small non-recursive bindings.
    pub inline_threshold_chirho: usize,
    /// Enable strictness analysis and worker/wrapper transform (backend optimization).
    /// Off by default for STG interpretation; backends enable it for native codegen.
    pub enable_worker_wrapper_chirho: bool,
    /// Enable dead argument elimination (removes unused function parameters).
    pub enable_dead_arg_elim_chirho: bool,
    /// Enable constructor specialization (SpecConstr) for recursive functions.
    pub enable_spec_constr_chirho: bool,
}

impl Default for SimplifyConfigChirho {
    fn default() -> Self {
        Self {
            max_iterations_chirho: 4,
            inline_threshold_chirho: AUTO_INLINE_THRESHOLD_CHIRHO,
            enable_worker_wrapper_chirho: false,
            enable_dead_arg_elim_chirho: false,
            enable_spec_constr_chirho: false,
        }
    }
}

/// Count the number of AST nodes in a Core expression (used for inline size heuristics).
pub fn expr_size_chirho(expr_chirho: &CoreExprChirho) -> usize {
    match expr_chirho {
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => 1,
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => 1 + expr_size_chirho(fun_chirho) + expr_size_chirho(arg_chirho),
        CoreExprChirho::LamChirho { body_chirho, .. } => 1 + expr_size_chirho(body_chirho),
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            1 + binds_chirho
                .iter()
                .map(|(_, rhs_chirho)| expr_size_chirho(rhs_chirho))
                .sum::<usize>()
                + expr_size_chirho(body_chirho)
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            1 + expr_size_chirho(scrutinee_chirho)
                + alts_chirho
                    .iter()
                    .map(|a_chirho| expr_size_chirho(&a_chirho.rhs_chirho))
                    .sum::<usize>()
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => 1 + expr_size_chirho(body_chirho),
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } => 1 + expr_size_chirho(inner_chirho),
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
            1 + args_chirho
                .iter()
                .map(|a_chirho| expr_size_chirho(a_chirho))
                .sum::<usize>()
        }
        CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            1 + args_chirho
                .iter()
                .map(|a_chirho| expr_size_chirho(a_chirho))
                .sum::<usize>()
        }
    }
}

/// Check whether an expression is trivial (a variable or literal) and therefore
/// always safe to inline without duplicating work.
fn is_trivial_chirho(expr_chirho: &CoreExprChirho) -> bool {
    matches!(
        expr_chirho,
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_)
    )
}

/// Build the inline environment: a map from CoreId → RHS expression for bindings
/// that should be inlined at call sites.
///
/// Inlining strategy:
/// - `{-# INLINE f #-}` — always inline regardless of size
/// - `{-# NOINLINE f #-}` — never inline
/// - `{-# INLINABLE f #-}` — inline if small and non-recursive
/// - No annotation — auto-inline only trivial expressions (Var/Lit) to avoid
///   changing evaluation semantics or duplicating work
fn build_inline_env_chirho(
    bindings_chirho: &[CoreBindingChirho],
    threshold_chirho: usize,
) -> HashMap<CoreIdChirho, CoreExprChirho> {
    let mut env_chirho = HashMap::new();
    for binding_chirho in bindings_chirho {
        match binding_chirho.inline_chirho {
            // NOINLINE — never inline
            InlineAnnotationChirho::NeverChirho => continue,
            // INLINE — always inline regardless of size
            InlineAnnotationChirho::AlwaysChirho => {
                env_chirho.insert(
                    binding_chirho.binder_chirho.id_chirho,
                    binding_chirho.rhs_chirho.clone(),
                );
            }
            // INLINABLE — inline if small and non-recursive
            InlineAnnotationChirho::InlinableChirho => {
                if !binding_chirho.is_rec_chirho
                    && expr_size_chirho(&binding_chirho.rhs_chirho) <= threshold_chirho
                {
                    env_chirho.insert(
                        binding_chirho.binder_chirho.id_chirho,
                        binding_chirho.rhs_chirho.clone(),
                    );
                }
            }
            // No annotation — auto-inline only trivial expressions (Var/Lit)
            InlineAnnotationChirho::NoneChirho => {
                if !binding_chirho.is_rec_chirho && is_trivial_chirho(&binding_chirho.rhs_chirho) {
                    env_chirho.insert(
                        binding_chirho.binder_chirho.id_chirho,
                        binding_chirho.rhs_chirho.clone(),
                    );
                }
            }
        }
    }
    env_chirho
}

/// Inline variables from the inline environment into an expression.
fn inline_expr_chirho(
    expr_chirho: &CoreExprChirho,
    env_chirho: &HashMap<CoreIdChirho, CoreExprChirho>,
) -> CoreExprChirho {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if let Some(rhs_chirho) = env_chirho.get(id_chirho) {
                rhs_chirho.clone()
            } else {
                expr_chirho.clone()
            }
        }
        CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => CoreExprChirho::AppChirho {
            fun_chirho: Box::new(inline_expr_chirho(fun_chirho, env_chirho)),
            arg_chirho: Box::new(inline_expr_chirho(arg_chirho, env_chirho)),
        },
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            // If the lambda binder shadows an inline candidate, remove it from env
            let mut env2_chirho = env_chirho.clone();
            env2_chirho.remove(&binder_chirho.id_chirho);
            CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho.clone(),
                body_chirho: Box::new(inline_expr_chirho(body_chirho, &env2_chirho)),
            }
        }
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => {
            let mut env2_chirho = env_chirho.clone();
            for (b_chirho, _) in binds_chirho {
                env2_chirho.remove(&b_chirho.id_chirho);
            }
            CoreExprChirho::LetChirho {
                rec_chirho: *rec_chirho,
                binds_chirho: binds_chirho
                    .iter()
                    .map(|(b_chirho, rhs_chirho)| {
                        (
                            b_chirho.clone(),
                            inline_expr_chirho(rhs_chirho, &env2_chirho),
                        )
                    })
                    .collect(),
                body_chirho: Box::new(inline_expr_chirho(body_chirho, &env2_chirho)),
            }
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => {
            let mut env2_chirho = env_chirho.clone();
            env2_chirho.remove(&bind_chirho.id_chirho);
            CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(inline_expr_chirho(scrutinee_chirho, env_chirho)),
                bind_chirho: bind_chirho.clone(),
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let mut alt_env_chirho = env2_chirho.clone();
                        for b_chirho in &alt_chirho.binders_chirho {
                            alt_env_chirho.remove(&b_chirho.id_chirho);
                        }
                        CoreAltChirho {
                            con_chirho: alt_chirho.con_chirho.clone(),
                            binders_chirho: alt_chirho.binders_chirho.clone(),
                            rhs_chirho: inline_expr_chirho(&alt_chirho.rhs_chirho, &alt_env_chirho),
                        }
                    })
                    .collect(),
            }
        }
        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => CoreExprChirho::TyLamChirho {
            ty_var_chirho: ty_var_chirho.clone(),
            body_chirho: Box::new(inline_expr_chirho(body_chirho, env_chirho)),
        },
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(inline_expr_chirho(inner_chirho, env_chirho)),
            ty_chirho: ty_chirho.clone(),
        },
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| inline_expr_chirho(a_chirho, env_chirho))
                .collect(),
        },
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| inline_expr_chirho(a_chirho, env_chirho))
                .collect(),
        },
    }
}

/// Run the simplifier on a Core module.
///
/// Each iteration performs:
/// 1. Inlining — substitute small/INLINE bindings at call sites
/// 2. Simplification — beta reduction, dead binding elimination, case-of-known,
///    constant folding
/// 3. CSE — deduplicate identical RHS expressions in top-level bindings and
///    let-blocks
pub fn simplify_module_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &SimplifyConfigChirho,
) -> CoreModuleChirho {
    let mut bindings_chirho = module_chirho.bindings_chirho.clone();

    for _ in 0..config_chirho.max_iterations_chirho {
        // Phase 1: Build inline environment and inline variables
        let inline_env_chirho =
            build_inline_env_chirho(&bindings_chirho, config_chirho.inline_threshold_chirho);
        if !inline_env_chirho.is_empty() {
            bindings_chirho = bindings_chirho
                .into_iter()
                .map(|mut b_chirho| {
                    b_chirho.rhs_chirho =
                        inline_expr_chirho(&b_chirho.rhs_chirho, &inline_env_chirho);
                    b_chirho
                })
                .collect();
        }

        // Phase 2: Standard simplification
        let new_bindings_chirho: Vec<CoreBindingChirho> = bindings_chirho
            .into_iter()
            .map(|mut binding_chirho| {
                binding_chirho.rhs_chirho = simplify_expr_chirho(&binding_chirho.rhs_chirho);
                binding_chirho
            })
            .collect();
        bindings_chirho = new_bindings_chirho;

        // Phase 3: CSE — deduplicate identical top-level binding RHSes
        bindings_chirho = cse_top_level_chirho(bindings_chirho);

        // Phase 3b: Intra-expression CSE on each binding's RHS
        bindings_chirho = bindings_chirho
            .into_iter()
            .map(|mut b_chirho| {
                b_chirho.rhs_chirho = cse_expr_chirho(&b_chirho.rhs_chirho);
                b_chirho
            })
            .collect();
    }

    // Phase 4: Specialization — create monomorphized copies for SPECIALIZE pragmas
    if !module_chirho.specialize_pragmas_chirho.is_empty() {
        bindings_chirho = specialize_bindings_chirho(
            bindings_chirho,
            &module_chirho.specialize_pragmas_chirho,
            &module_chirho.names_chirho,
        );
    }

    // Phase 5: Strictness analysis & worker/wrapper transform (backend optimization)
    if config_chirho.enable_worker_wrapper_chirho {
        bindings_chirho = worker_wrapper_chirho(bindings_chirho);
    }

    // Phase 6: Dead argument elimination (removes unused function parameters)
    if config_chirho.enable_dead_arg_elim_chirho {
        bindings_chirho = dead_arg_elimination_chirho(bindings_chirho);
    }

    // Phase 7: Constructor specialization (SpecConstr)
    if config_chirho.enable_spec_constr_chirho {
        bindings_chirho = spec_constr_chirho(bindings_chirho);
    }

    CoreModuleChirho {
        name_chirho: module_chirho.name_chirho.clone(),
        bindings_chirho,
        names_chirho: module_chirho.names_chirho.clone(),
        specialize_pragmas_chirho: module_chirho.specialize_pragmas_chirho.clone(),
        foreign_exports_chirho: module_chirho.foreign_exports_chirho.clone(),
    }
}

/// Simplify a single Core expression (one pass, recursive).
fn simplify_expr_chirho(expr_chirho: &CoreExprChirho) -> CoreExprChirho {
    match expr_chirho {
        // Beta reduction: (\x -> body) arg → body[x := arg]
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            let fun_simplified_chirho = simplify_expr_chirho(fun_chirho);
            let arg_simplified_chirho = simplify_expr_chirho(arg_chirho);

            if let CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } = &fun_simplified_chirho
            {
                // Beta-reduce: substitute the argument for the binder in the body
                let substituted_chirho = subst_var_chirho(
                    &body_chirho,
                    binder_chirho.id_chirho,
                    &arg_simplified_chirho,
                );
                simplify_expr_chirho(&substituted_chirho)
            } else {
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(fun_simplified_chirho),
                    arg_chirho: Box::new(arg_simplified_chirho),
                }
            }
        }

        // Simplify inside lambdas
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => CoreExprChirho::LamChirho {
            binder_chirho: binder_chirho.clone(),
            body_chirho: Box::new(simplify_expr_chirho(body_chirho)),
        },

        // Dead binding elimination + simplify let bodies
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => {
            let body_simplified_chirho = simplify_expr_chirho(body_chirho);
            let body_free_vars_chirho = free_vars_chirho(&body_simplified_chirho);

            // For non-recursive lets, drop bindings that aren't used in the body
            let simplified_binds_chirho: Vec<_> = binds_chirho
                .iter()
                .filter(|(binder_chirho, _)| {
                    *rec_chirho || body_free_vars_chirho.contains(&binder_chirho.id_chirho)
                })
                .map(|(binder_chirho, rhs_chirho)| {
                    (binder_chirho.clone(), simplify_expr_chirho(rhs_chirho))
                })
                .collect();

            if simplified_binds_chirho.is_empty() {
                // All bindings dead — just return the body
                body_simplified_chirho
            } else {
                CoreExprChirho::LetChirho {
                    rec_chirho: *rec_chirho,
                    binds_chirho: simplified_binds_chirho,
                    body_chirho: Box::new(body_simplified_chirho),
                }
            }
        }

        // Case-of-known-constructor
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => {
            let scrut_simplified_chirho = simplify_expr_chirho(scrutinee_chirho);

            // Try to match a literal scrutinee against literal alts
            if let CoreExprChirho::LitChirho(lit_chirho) = &scrut_simplified_chirho {
                for alt_chirho in alts_chirho {
                    if let AltConChirho::LitConChirho(alt_lit_chirho) = &alt_chirho.con_chirho {
                        if alt_lit_chirho == lit_chirho {
                            return simplify_expr_chirho(&alt_chirho.rhs_chirho);
                        }
                    }
                }
                // Fall through to default if no literal match
                for alt_chirho in alts_chirho {
                    if alt_chirho.con_chirho == AltConChirho::DefaultChirho {
                        return simplify_expr_chirho(&alt_chirho.rhs_chirho);
                    }
                }
            }

            // Simplify inside alts
            let simplified_alts_chirho: Vec<CoreAltChirho> = alts_chirho
                .iter()
                .map(|alt_chirho| CoreAltChirho {
                    con_chirho: alt_chirho.con_chirho.clone(),
                    binders_chirho: alt_chirho.binders_chirho.clone(),
                    rhs_chirho: simplify_expr_chirho(&alt_chirho.rhs_chirho),
                })
                .collect();

            CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(scrut_simplified_chirho),
                bind_chirho: bind_chirho.clone(),
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: simplified_alts_chirho,
            }
        }

        // Simplify inside type abstractions
        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => CoreExprChirho::TyLamChirho {
            ty_var_chirho: ty_var_chirho.clone(),
            body_chirho: Box::new(simplify_expr_chirho(body_chirho)),
        },

        // Simplify inside type applications
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(simplify_expr_chirho(inner_chirho)),
            ty_chirho: ty_chirho.clone(),
        },

        // Primitive operations: simplify args, then try constant folding
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => {
            let simplified_args_chirho: Vec<CoreExprChirho> = args_chirho
                .iter()
                .map(|a_chirho| simplify_expr_chirho(a_chirho))
                .collect();

            // Try constant folding for binary integer ops
            if simplified_args_chirho.len() == 2 {
                if let (
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(a_chirho)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(b_chirho)),
                ) = (&simplified_args_chirho[0], &simplified_args_chirho[1])
                {
                    let folded_chirho = match name_chirho.as_str() {
                        "+#" => Some(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            a_chirho.wrapping_add(*b_chirho),
                        ))),
                        "-#" => Some(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            a_chirho.wrapping_sub(*b_chirho),
                        ))),
                        "*#" => Some(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            a_chirho.wrapping_mul(*b_chirho),
                        ))),
                        _ => None,
                    };
                    if let Some(result_chirho) = folded_chirho {
                        return result_chirho;
                    }
                }
            }

            // Unary constant fold: negate
            if simplified_args_chirho.len() == 1 {
                if let CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(v_chirho)) =
                    &simplified_args_chirho[0]
                {
                    if name_chirho == "negate#" {
                        return CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(-v_chirho));
                    }
                }
            }

            CoreExprChirho::PrimOpChirho {
                name_chirho: name_chirho.clone(),
                args_chirho: simplified_args_chirho,
            }
        }

        // Constructor applications: simplify args
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| simplify_expr_chirho(a_chirho))
                .collect(),
        },

        // Vars and Lits are already simple
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
    }
}

/// Substitute all occurrences of `var_id` with `replacement` in `expr`.
fn subst_var_chirho(
    expr_chirho: &CoreExprChirho,
    var_id_chirho: CoreIdChirho,
    replacement_chirho: &CoreExprChirho,
) -> CoreExprChirho {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if *id_chirho == var_id_chirho {
                replacement_chirho.clone()
            } else {
                expr_chirho.clone()
            }
        }

        CoreExprChirho::LitChirho(_) => expr_chirho.clone(),

        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => CoreExprChirho::AppChirho {
            fun_chirho: Box::new(subst_var_chirho(
                fun_chirho,
                var_id_chirho,
                replacement_chirho,
            )),
            arg_chirho: Box::new(subst_var_chirho(
                arg_chirho,
                var_id_chirho,
                replacement_chirho,
            )),
        },

        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            // Don't substitute under a shadowing binder
            if binder_chirho.id_chirho == var_id_chirho {
                expr_chirho.clone()
            } else {
                CoreExprChirho::LamChirho {
                    binder_chirho: binder_chirho.clone(),
                    body_chirho: Box::new(subst_var_chirho(
                        body_chirho,
                        var_id_chirho,
                        replacement_chirho,
                    )),
                }
            }
        }

        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => {
            let shadows_chirho = binds_chirho
                .iter()
                .any(|(b_chirho, _)| b_chirho.id_chirho == var_id_chirho);
            if shadows_chirho {
                // The variable is rebound — only substitute in the RHS of bindings
                let new_binds_chirho: Vec<_> = binds_chirho
                    .iter()
                    .map(|(b_chirho, rhs_chirho)| {
                        (
                            b_chirho.clone(),
                            subst_var_chirho(rhs_chirho, var_id_chirho, replacement_chirho),
                        )
                    })
                    .collect();
                CoreExprChirho::LetChirho {
                    rec_chirho: *rec_chirho,
                    binds_chirho: new_binds_chirho,
                    body_chirho: body_chirho.clone(),
                }
            } else {
                let new_binds_chirho: Vec<_> = binds_chirho
                    .iter()
                    .map(|(b_chirho, rhs_chirho)| {
                        (
                            b_chirho.clone(),
                            subst_var_chirho(rhs_chirho, var_id_chirho, replacement_chirho),
                        )
                    })
                    .collect();
                CoreExprChirho::LetChirho {
                    rec_chirho: *rec_chirho,
                    binds_chirho: new_binds_chirho,
                    body_chirho: Box::new(subst_var_chirho(
                        body_chirho,
                        var_id_chirho,
                        replacement_chirho,
                    )),
                }
            }
        }

        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => {
            let new_scrut_chirho =
                subst_var_chirho(scrutinee_chirho, var_id_chirho, replacement_chirho);
            // The case binder shadows
            if bind_chirho.id_chirho == var_id_chirho {
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(new_scrut_chirho),
                    bind_chirho: bind_chirho.clone(),
                    result_ty_chirho: result_ty_chirho.clone(),
                    alts_chirho: alts_chirho.clone(),
                }
            } else {
                let new_alts_chirho: Vec<CoreAltChirho> = alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let shadows_in_alt_chirho = alt_chirho
                            .binders_chirho
                            .iter()
                            .any(|b_chirho| b_chirho.id_chirho == var_id_chirho);
                        if shadows_in_alt_chirho {
                            alt_chirho.clone()
                        } else {
                            CoreAltChirho {
                                con_chirho: alt_chirho.con_chirho.clone(),
                                binders_chirho: alt_chirho.binders_chirho.clone(),
                                rhs_chirho: subst_var_chirho(
                                    &alt_chirho.rhs_chirho,
                                    var_id_chirho,
                                    replacement_chirho,
                                ),
                            }
                        }
                    })
                    .collect();
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(new_scrut_chirho),
                    bind_chirho: bind_chirho.clone(),
                    result_ty_chirho: result_ty_chirho.clone(),
                    alts_chirho: new_alts_chirho,
                }
            }
        }

        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => CoreExprChirho::TyLamChirho {
            ty_var_chirho: ty_var_chirho.clone(),
            body_chirho: Box::new(subst_var_chirho(
                body_chirho,
                var_id_chirho,
                replacement_chirho,
            )),
        },

        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(subst_var_chirho(
                inner_chirho,
                var_id_chirho,
                replacement_chirho,
            )),
            ty_chirho: ty_chirho.clone(),
        },

        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| subst_var_chirho(a_chirho, var_id_chirho, replacement_chirho))
                .collect(),
        },

        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| subst_var_chirho(a_chirho, var_id_chirho, replacement_chirho))
                .collect(),
        },
    }
}

/// Collect all free variable IDs in an expression.
pub fn free_vars_chirho(expr_chirho: &CoreExprChirho) -> HashSet<CoreIdChirho> {
    let mut vars_chirho = HashSet::new();
    collect_free_vars_chirho(expr_chirho, &mut HashSet::new(), &mut vars_chirho);
    vars_chirho
}

fn collect_free_vars_chirho(
    expr_chirho: &CoreExprChirho,
    bound_chirho: &mut HashSet<CoreIdChirho>,
    free_chirho: &mut HashSet<CoreIdChirho>,
) {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if !bound_chirho.contains(id_chirho) {
                free_chirho.insert(*id_chirho);
            }
        }

        CoreExprChirho::LitChirho(_) => {}

        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            collect_free_vars_chirho(fun_chirho, bound_chirho, free_chirho);
            collect_free_vars_chirho(arg_chirho, bound_chirho, free_chirho);
        }

        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            bound_chirho.insert(binder_chirho.id_chirho);
            collect_free_vars_chirho(body_chirho, bound_chirho, free_chirho);
            bound_chirho.remove(&binder_chirho.id_chirho);
        }

        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (binder_chirho, _) in binds_chirho {
                bound_chirho.insert(binder_chirho.id_chirho);
            }
            for (_, rhs_chirho) in binds_chirho {
                collect_free_vars_chirho(rhs_chirho, bound_chirho, free_chirho);
            }
            collect_free_vars_chirho(body_chirho, bound_chirho, free_chirho);
            for (binder_chirho, _) in binds_chirho {
                bound_chirho.remove(&binder_chirho.id_chirho);
            }
        }

        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            collect_free_vars_chirho(scrutinee_chirho, bound_chirho, free_chirho);
            bound_chirho.insert(bind_chirho.id_chirho);
            for alt_chirho in alts_chirho {
                for b_chirho in &alt_chirho.binders_chirho {
                    bound_chirho.insert(b_chirho.id_chirho);
                }
                collect_free_vars_chirho(&alt_chirho.rhs_chirho, bound_chirho, free_chirho);
                for b_chirho in &alt_chirho.binders_chirho {
                    bound_chirho.remove(&b_chirho.id_chirho);
                }
            }
            bound_chirho.remove(&bind_chirho.id_chirho);
        }

        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            collect_free_vars_chirho(body_chirho, bound_chirho, free_chirho);
        }

        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } => {
            collect_free_vars_chirho(inner_chirho, bound_chirho, free_chirho);
        }

        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                collect_free_vars_chirho(arg_chirho, bound_chirho, free_chirho);
            }
        }
    }
}

// ── Dictionary elision for backend code generation ──
// Replaces selector+dict patterns with direct PrimOps, enabling
// LLVM and WASM backends to compile programs without heap-allocated dictionaries.

/// Resolve the binding name for a CoreId from a bindings list.
fn resolve_name_for_id_chirho(
    id_chirho: &CoreIdChirho,
    bindings_chirho: &[CoreBindingChirho],
) -> Option<String> {
    bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.id_chirho == *id_chirho)
        .map(|b_chirho| b_chirho.binder_chirho.name_chirho.clone())
}

/// Map known typeclass selector names to their primop equivalents.
fn selector_primop_chirho(name_chirho: &str) -> Option<&'static str> {
    match name_chirho {
        // Num
        "$sel_Num_+" => Some("+#"),
        "$sel_Num_-" => Some("-#"),
        "$sel_Num_*" => Some("*#"),
        "$sel_Num_negate" => Some("negate#"),
        "$sel_Num_abs" => Some("absInt#"),
        "$sel_Num_signum" => Some("signumInt#"),
        // Eq
        "$sel_Eq_==" => Some("==#"),
        "$sel_Eq_/=" => Some("/=#"),
        // Ord
        "$sel_Ord_compare" => Some("compare#"),
        "$sel_Ord_<" => Some("<#"),
        "$sel_Ord_>" => Some(">#"),
        "$sel_Ord_<=" => Some("<=#"),
        "$sel_Ord_>=" => Some(">=#"),
        "$sel_Ord_min" => Some("minInt#"),
        "$sel_Ord_max" => Some("maxInt#"),
        // Integral
        "$sel_Integral_div" => Some("divInt#"),
        "$sel_Integral_mod" => Some("modInt#"),
        "$sel_Integral_quot" => Some("quotInt#"),
        "$sel_Integral_rem" => Some("remInt#"),
        _ => None,
    }
}

/// Flatten nested App chains into callee + argument list.
fn flatten_apps_chirho(expr_chirho: &CoreExprChirho) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut cur_chirho = expr_chirho;
    while let CoreExprChirho::AppChirho {
        fun_chirho,
        arg_chirho,
    } = cur_chirho
    {
        args_chirho.push(arg_chirho.as_ref());
        cur_chirho = fun_chirho;
    }
    args_chirho.reverse();
    (cur_chirho, args_chirho)
}

fn expr_is_selector_var_chirho(
    expr_chirho: &CoreExprChirho,
    all_bindings_chirho: &[CoreBindingChirho],
) -> bool {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            resolve_name_for_id_chirho(id_chirho, all_bindings_chirho)
                .is_some_and(|name_chirho| name_chirho.starts_with("$sel_"))
        }
        _ => false,
    }
}

/// Dictionary elision: replace typeclass selector+dict application patterns
/// with direct PrimOp calls. Handles:
/// - `$sel_Num_fromInteger dict lit` → `lit`
/// - `$sel_Num_+ dict x y` → `+# x y` (and -, *, ==, compare, negate)
/// - Strip `\$dXxx -> body` dict lambda parameters
/// - Skip dict arguments (`$f`-prefixed binding references) at call sites
pub fn elide_dicts_chirho(
    expr_chirho: &CoreExprChirho,
    all_bindings_chirho: &[CoreBindingChirho],
) -> CoreExprChirho {
    // Check for selector+dict patterns via flattened App chain
    let (callee_chirho, all_args_chirho) = flatten_apps_chirho(expr_chirho);

    if let CoreExprChirho::VarChirho(sel_id_chirho) = callee_chirho {
        if let Some(name_chirho) = resolve_name_for_id_chirho(sel_id_chirho, all_bindings_chirho) {
            // $sel_Show_show dict x → showInt#/showBool#/showChar#/showFloat# x
            // Determine the show variant from the dict argument name.
            if name_chirho == "$sel_Show_show" && all_args_chirho.len() >= 2 {
                let show_primop_chirho = if let CoreExprChirho::VarChirho(dict_id_chirho) =
                    all_args_chirho[0]
                {
                    resolve_name_for_id_chirho(dict_id_chirho, all_bindings_chirho)
                        .and_then(|dn_chirho| {
                            if dn_chirho.contains("Bool") {
                                Some("showBool#")
                            } else if dn_chirho.contains("Char") {
                                Some("showChar#")
                            } else if dn_chirho.contains("Float")
                                || dn_chirho.contains("Double")
                            {
                                Some("showFloat#")
                            } else {
                                None
                            }
                        })
                        .unwrap_or("showInt#")
                } else {
                    "showInt#"
                };
                let simplified_chirho =
                    elide_dicts_chirho(all_args_chirho[1], all_bindings_chirho);
                return CoreExprChirho::PrimOpChirho {
                    name_chirho: show_primop_chirho.to_string(),
                    args_chirho: vec![simplified_chirho],
                };
            }

            // $sel_Num_fromInteger dict lit → lit
            if name_chirho == "$sel_Num_fromInteger" && all_args_chirho.len() >= 2 {
                let simplified_chirho = elide_dicts_chirho(all_args_chirho[1], all_bindings_chirho);
                if let CoreExprChirho::LitChirho(lit_chirho) = &simplified_chirho {
                    return CoreExprChirho::LitChirho(lit_chirho.clone());
                }
                return simplified_chirho;
            }

            // $sel_Num_+ dict x y → +# x y (binary)
            // $sel_Num_abs dict x → absInt# x (unary)
            if let Some(primop_chirho) = selector_primop_chirho(&name_chirho) {
                let is_unary_chirho = matches!(
                    primop_chirho,
                    "absInt#" | "signumInt#" | "negate#"
                );
                let min_args_chirho = if is_unary_chirho { 2 } else { 3 };
                if all_args_chirho.len() >= min_args_chirho {
                    let real_args_chirho: Vec<CoreExprChirho> = all_args_chirho[1..]
                        .iter()
                        .map(|a_chirho| elide_dicts_chirho(a_chirho, all_bindings_chirho))
                        .collect();
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_chirho.to_string(),
                        args_chirho: real_args_chirho,
                    };
                }
            }
        }
    }

    // Strip dict lambda parameters (\$dXxx -> body)
    if let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = expr_chirho
    {
        if binder_chirho.name_chirho.starts_with("$d") {
            return elide_dicts_chirho(body_chirho, all_bindings_chirho);
        }
    }

    // Recurse
    match expr_chirho {
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            let sf_chirho = elide_dicts_chirho(fun_chirho, all_bindings_chirho);
            let sa_chirho = elide_dicts_chirho(arg_chirho, all_bindings_chirho);
            // Skip dict arguments ($f-prefixed binding refs)
            if let CoreExprChirho::VarChirho(id_chirho) = &sa_chirho {
                if let Some(n_chirho) = resolve_name_for_id_chirho(id_chirho, all_bindings_chirho) {
                    if n_chirho.starts_with("$f") || n_chirho.starts_with("$d") {
                        // Selector value applications like `(-)` must keep the
                        // concrete dictionary so they still project the method
                        // closure after selector bodies are restored.
                        if expr_is_selector_var_chirho(&sf_chirho, all_bindings_chirho) {
                            return CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(sf_chirho),
                                arg_chirho: Box::new(sa_chirho),
                            };
                        }
                        return sf_chirho;
                    }
                }
            }
            CoreExprChirho::AppChirho {
                fun_chirho: Box::new(sf_chirho),
                arg_chirho: Box::new(sa_chirho),
            }
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => CoreExprChirho::LamChirho {
            binder_chirho: binder_chirho.clone(),
            body_chirho: Box::new(elide_dicts_chirho(body_chirho, all_bindings_chirho)),
        },
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => CoreExprChirho::LetChirho {
            rec_chirho: *rec_chirho,
            binds_chirho: binds_chirho
                .iter()
                .map(|(b_chirho, r_chirho)| {
                    (
                        b_chirho.clone(),
                        elide_dicts_chirho(r_chirho, all_bindings_chirho),
                    )
                })
                .collect(),
            body_chirho: Box::new(elide_dicts_chirho(body_chirho, all_bindings_chirho)),
        },
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        } => CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(elide_dicts_chirho(scrutinee_chirho, all_bindings_chirho)),
            bind_chirho: bind_chirho.clone(),
            result_ty_chirho: result_ty_chirho.clone(),
            alts_chirho: alts_chirho
                .iter()
                .map(|a_chirho| CoreAltChirho {
                    con_chirho: a_chirho.con_chirho.clone(),
                    binders_chirho: a_chirho.binders_chirho.clone(),
                    rhs_chirho: elide_dicts_chirho(&a_chirho.rhs_chirho, all_bindings_chirho),
                })
                .collect(),
        },
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho
                .iter()
                .map(|a_chirho| elide_dicts_chirho(a_chirho, all_bindings_chirho))
                .collect(),
        },
        _ => expr_chirho.clone(),
    }
}

/// Apply dictionary elision to all bindings in a module, then return only
/// bindings transitively reachable from `main`.
pub fn elide_dicts_and_filter_chirho(module_chirho: &CoreModuleChirho) -> CoreModuleChirho {
    let all_chirho = &module_chirho.bindings_chirho;

    // Step 1: Elide dicts in all bindings
    let simplified_chirho: Vec<CoreBindingChirho> = all_chirho
        .iter()
        .map(|b_chirho| {
            let mut b2_chirho = b_chirho.clone();
            b2_chirho.rhs_chirho = elide_dicts_chirho(&b2_chirho.rhs_chirho, all_chirho);
            b2_chirho
        })
        .collect();

    // Step 2: Build id→index map
    let mut id_to_idx_chirho = std::collections::HashMap::new();
    for (i_chirho, b_chirho) in simplified_chirho.iter().enumerate() {
        id_to_idx_chirho.insert(b_chirho.binder_chirho.id_chirho, i_chirho);
    }

    // Step 3: BFS from main
    let mut reachable_chirho: HashSet<usize> = HashSet::new();
    let mut worklist_chirho: Vec<usize> = Vec::new();
    for (i_chirho, b_chirho) in simplified_chirho.iter().enumerate() {
        if b_chirho.binder_chirho.name_chirho == "main" {
            reachable_chirho.insert(i_chirho);
            worklist_chirho.push(i_chirho);
        }
    }
    while let Some(idx_chirho) = worklist_chirho.pop() {
        let refs_chirho = free_vars_chirho(&simplified_chirho[idx_chirho].rhs_chirho);
        for id_chirho in refs_chirho {
            if let Some(&dep_chirho) = id_to_idx_chirho.get(&id_chirho) {
                if reachable_chirho.insert(dep_chirho) {
                    worklist_chirho.push(dep_chirho);
                }
            }
        }
    }

    let filtered_chirho: Vec<CoreBindingChirho> = simplified_chirho
        .into_iter()
        .enumerate()
        .filter(|(i_chirho, _)| reachable_chirho.contains(i_chirho))
        .map(|(_, b_chirho)| b_chirho)
        .collect();

    CoreModuleChirho {
        name_chirho: module_chirho.name_chirho.clone(),
        bindings_chirho: filtered_chirho,
        names_chirho: module_chirho.names_chirho.clone(),
        specialize_pragmas_chirho: module_chirho.specialize_pragmas_chirho.clone(),
        foreign_exports_chirho: module_chirho.foreign_exports_chirho.clone(),
    }
}

// ---------------------------------------------------------------------------
// Common Subexpression Elimination (CSE)
// ---------------------------------------------------------------------------

/// Top-level CSE: deduplicate bindings with identical RHS expressions.
///
/// If two non-recursive bindings `x = e` and `y = e` have the same RHS,
/// redirect all uses of `y` to `x` (replace `y` with `Var(x)` in its RHS)
/// and let dead-binding elimination clean up `y` in the next iteration.
fn cse_top_level_chirho(bindings_chirho: Vec<CoreBindingChirho>) -> Vec<CoreBindingChirho> {
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
fn cse_expr_chirho(expr_chirho: &CoreExprChirho) -> CoreExprChirho {
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

// ---------------------------------------------------------------------------
// Phase 4: Specialization — create monomorphized binding copies
// ---------------------------------------------------------------------------

/// For each `{-# SPECIALIZE f :: Type #-}` pragma, clone the original binding
/// of `f`, rename it to `$spec_f_<hash>`, and mark it for aggressive inlining.
/// This allows the simplifier's subsequent passes to inline and optimize the
/// specialized copy without dictionary indirection.
fn specialize_bindings_chirho(
    mut bindings_chirho: Vec<CoreBindingChirho>,
    specialize_pragmas_chirho: &HashMap<String, Vec<String>>,
    names_chirho: &HashMap<CoreIdChirho, String>,
) -> Vec<CoreBindingChirho> {
    // Build name→index map for quick lookup
    let name_to_idx_chirho: HashMap<&str, usize> = bindings_chirho
        .iter()
        .enumerate()
        .map(|(idx_chirho, b_chirho)| (b_chirho.binder_chirho.name_chirho.as_str(), idx_chirho))
        .collect();

    // Also build a reverse map from CoreId → name for any binding (reserved
    // for future type-aware specialization that needs to look up binder names)
    let _id_to_name_chirho: HashMap<CoreIdChirho, &str> = bindings_chirho
        .iter()
        .map(|b_chirho| {
            (
                b_chirho.binder_chirho.id_chirho,
                b_chirho.binder_chirho.name_chirho.as_str(),
            )
        })
        .chain(
            names_chirho
                .iter()
                .map(|(id_chirho, name_chirho)| (*id_chirho, name_chirho.as_str())),
        )
        .collect();

    let mut new_bindings_chirho = Vec::new();

    for (func_name_chirho, spec_types_chirho) in specialize_pragmas_chirho {
        // Find the original binding
        let idx_chirho =
            if let Some(&idx_chirho) = name_to_idx_chirho.get(func_name_chirho.as_str()) {
                idx_chirho
            } else {
                continue; // Binding not found, skip
            };

        let original_chirho = &bindings_chirho[idx_chirho];

        for (spec_idx_chirho, spec_type_chirho) in spec_types_chirho.iter().enumerate() {
            // Create a unique name for the specialized binding
            let spec_name_chirho = format!("$spec_{}_{}", func_name_chirho, spec_idx_chirho);

            // Generate a new CoreId for the specialized binding
            // Use a high offset to avoid collisions
            let spec_id_chirho = CoreIdChirho(
                original_chirho
                    .binder_chirho
                    .id_chirho
                    .0
                    .wrapping_add(10000 + spec_idx_chirho as u32),
            );

            // Clone the original binding with a new name and INLINE annotation
            let spec_binding_chirho = CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: spec_id_chirho,
                    name_chirho: spec_name_chirho.clone(),
                    ty_chirho: original_chirho.binder_chirho.ty_chirho.clone(),
                    span_chirho: original_chirho.binder_chirho.span_chirho,
                },
                rhs_chirho: original_chirho.rhs_chirho.clone(),
                is_rec_chirho: original_chirho.is_rec_chirho,
                // Mark specialized copies for aggressive inlining
                inline_chirho: InlineAnnotationChirho::AlwaysChirho,
            };

            new_bindings_chirho.push((
                spec_name_chirho,
                spec_type_chirho.clone(),
                spec_binding_chirho,
            ));
        }
    }

    // Append all specialized bindings to the module
    for (_name_chirho, _ty_chirho, binding_chirho) in new_bindings_chirho {
        bindings_chirho.push(binding_chirho);
    }

    bindings_chirho
}

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
mod tests_chirho {
    use super::*;
    use crate::expr_chirho::{BinderChirho, CoreLitChirho};
    use haskelujah_span_chirho::SpanChirho;
    use haskelujah_typing_chirho::ty_chirho::TyChirho;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn beta_reduction_chirho() {
        // (\x -> x) 42 → 42
        let expr_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            }),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
        };

        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
        );
    }

    #[test]
    fn dead_binding_elimination_chirho() {
        // let x = 42 in 99 → 99  (x is unused)
        let expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                dummy_binder_chirho("x", 0),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            )],
            body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99))),
        };

        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99))
        );
    }

    #[test]
    fn live_binding_preserved_chirho() {
        // let x = 42 in x → let x = 42 in x  (x IS used)
        let expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                dummy_binder_chirho("x", 0),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            )],
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        };

        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert!(matches!(result_chirho, CoreExprChirho::LetChirho { .. }));
    }

    #[test]
    fn case_of_known_literal_chirho() {
        // case 1 of { 1 -> 10; _ -> 20 } → 10
        let expr_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
            bind_chirho: dummy_binder_chirho("wild", 99),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
                },
            ],
        };

        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10))
        );
    }

    #[test]
    fn case_of_known_literal_default_chirho() {
        // case 99 of { 1 -> 10; _ -> 20 } → 20
        let expr_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99))),
            bind_chirho: dummy_binder_chirho("wild", 99),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20)),
                },
            ],
        };

        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(20))
        );
    }

    #[test]
    fn nested_beta_reduction_chirho() {
        // (\x -> \y -> x) 1 2 → 1
        let inner_lam_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("y", 1),
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        };
        let outer_lam_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(inner_lam_chirho),
        };
        let app1_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(outer_lam_chirho),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
        };
        let app2_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(app1_chirho),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
        };

        let result_chirho = simplify_expr_chirho(&app2_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))
        );
    }

    #[test]
    fn simplify_module_chirho() {
        // Module with one binding: f = (\x -> x) 42
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: dummy_binder_chirho("x", 0),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        assert_eq!(result_chirho.bindings_chirho.len(), 1);
        assert_eq!(
            result_chirho.bindings_chirho[0].rhs_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
        );
    }

    #[test]
    fn free_vars_basic_chirho() {
        // \x -> y  — y is free, x is bound
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
        };
        let fv_chirho = free_vars_chirho(&expr_chirho);
        assert!(fv_chirho.contains(&CoreIdChirho(1)));
        assert!(!fv_chirho.contains(&CoreIdChirho(0)));
    }

    #[test]
    fn subst_respects_shadowing_chirho() {
        // subst [x := 42] in (\x -> x) should NOT substitute
        let lam_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        };
        let result_chirho = subst_var_chirho(
            &lam_chirho,
            CoreIdChirho(0),
            &CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
        );
        // Should be unchanged — the lambda shadows x
        assert_eq!(result_chirho, lam_chirho);
    }

    #[test]
    fn constant_fold_primop_add_chirho() {
        // PrimOp("+#", [3, 4]) → 7
        let expr_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "+#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(4)),
            ],
        };
        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7))
        );
    }

    #[test]
    fn constant_fold_primop_mul_chirho() {
        // PrimOp("*#", [6, 7]) → 42
        let expr_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "*#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(6)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
            ],
        };
        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
        );
    }

    #[test]
    fn constant_fold_primop_negate_chirho() {
        // PrimOp("negate#", [42]) → -42
        let expr_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "negate#".to_string(),
            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))],
        };
        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert_eq!(
            result_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(-42))
        );
    }

    #[test]
    fn primop_non_literal_args_preserved_chirho() {
        // PrimOp("+#", [Var(0), Lit(1)]) — not foldable, should be preserved
        let expr_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "+#".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(CoreIdChirho(0)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
            ],
        };
        let result_chirho = simplify_expr_chirho(&expr_chirho);
        assert!(matches!(result_chirho, CoreExprChirho::PrimOpChirho { .. }));
    }

    // ── Inlining tests ──

    #[test]
    fn expr_size_lit_chirho() {
        assert_eq!(
            expr_size_chirho(&CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
            1
        );
    }

    #[test]
    fn expr_size_app_chirho() {
        // f x → 1 (App) + 1 (Var f) + 1 (Var x) = 3
        let expr_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
        };
        assert_eq!(expr_size_chirho(&expr_chirho), 3);
    }

    #[test]
    fn expr_size_lam_chirho() {
        // \x -> 42 → 1 (Lam) + 1 (Lit) = 2
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
        };
        assert_eq!(expr_size_chirho(&expr_chirho), 2);
    }

    #[test]
    fn inline_always_annotation_chirho() {
        // f = 42 (INLINE), g = f
        // After inlining: g = 42
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 10),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::AlwaysChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("g", 11),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should be inlined to 42
        assert_eq!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
        );
    }

    #[test]
    fn noinline_annotation_chirho() {
        // f = 42 (NOINLINE), g = f
        // After inlining: g should still be Var(f), not 42
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 10),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NeverChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("g", 11),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should NOT be inlined — f is NOINLINE
        assert_eq!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::VarChirho(CoreIdChirho(10))
        );
    }

    #[test]
    fn auto_inline_trivial_chirho() {
        // f = x (trivial var-to-var), g = f
        // After inlining: g = x
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 10),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(5)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("g", 11),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should be inlined to Var(5) since f = Var(5) is trivial
        assert_eq!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::VarChirho(CoreIdChirho(5))
        );
    }

    #[test]
    fn no_auto_inline_non_trivial_chirho() {
        // f = \x -> x (non-trivial, size=2), g = f (no annotation)
        // Auto-inlining should NOT inline because f is not trivial and has no INLINE
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 10),
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: dummy_binder_chirho("x", 0),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("g", 11),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should NOT be inlined since f is a lambda (not trivial)
        assert_eq!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::VarChirho(CoreIdChirho(10))
        );
    }

    #[test]
    fn inlinable_small_function_chirho() {
        // f = \x -> x (INLINABLE, size=2 <= threshold), g = f
        // Should inline because INLINABLE + small
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 10),
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: dummy_binder_chirho("x", 0),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::InlinableChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("g", 11),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should be the identity lambda after inlining
        assert!(matches!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
    }

    #[test]
    fn recursive_not_inlined_chirho() {
        // f = f (recursive), g = f
        // Recursive bindings should never be auto-inlined
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 10),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: true,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("g", 11),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should not be inlined — f is recursive
        assert_eq!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::VarChirho(CoreIdChirho(10))
        );
    }

    #[test]
    fn inline_always_large_function_chirho() {
        // f = \x -> \y -> PrimOp("+#", [x, y]) (INLINE — should inline regardless of size)
        let large_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("y", 1),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(0)),
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                    ],
                }),
            }),
        };
        assert!(expr_size_chirho(&large_rhs_chirho) > 1); // size = 5
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 10),
                    rhs_chirho: large_rhs_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::AlwaysChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("g", 11),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(10)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should be the lambda from f, not Var(10)
        assert!(matches!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
    }

    // ── CSE tests ───────────────────────────────────────────────────────

    fn mk_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        dummy_binder_chirho(name_chirho, id_chirho)
    }

    #[test]
    fn cse_top_level_duplicate_chirho() {
        // Two top-level bindings with identical RHS should be deduplicated:
        // x = +# 1 2;  y = +# 1 2  →  x = +# 1 2;  y = x
        let bindings_chirho = vec![
            CoreBindingChirho {
                binder_chirho: mk_binder_chirho("x", 100),
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                    ],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: mk_binder_chirho("y", 101),
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                    ],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ];

        let result_chirho = cse_top_level_chirho(bindings_chirho);
        // y should be redirected to x (Var(100))
        assert_eq!(
            result_chirho[1].rhs_chirho,
            CoreExprChirho::VarChirho(CoreIdChirho(100))
        );
        // x should keep its original RHS
        assert!(matches!(
            result_chirho[0].rhs_chirho,
            CoreExprChirho::PrimOpChirho { .. }
        ));
    }

    #[test]
    fn cse_top_level_no_duplicate_chirho() {
        // Different RHSes should not be deduplicated
        let bindings_chirho = vec![
            CoreBindingChirho {
                binder_chirho: mk_binder_chirho("x", 100),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: mk_binder_chirho("y", 101),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ];

        let result_chirho = cse_top_level_chirho(bindings_chirho);
        // Both should keep their original RHS (trivial expressions skip CSE anyway)
        assert_eq!(
            result_chirho[0].rhs_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))
        );
        assert_eq!(
            result_chirho[1].rhs_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))
        );
    }

    #[test]
    fn cse_top_level_recursive_skipped_chirho() {
        // Recursive bindings should not be CSE'd
        let rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "+#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
            ],
        };
        let bindings_chirho = vec![
            CoreBindingChirho {
                binder_chirho: mk_binder_chirho("x", 100),
                rhs_chirho: rhs_chirho.clone(),
                is_rec_chirho: true, // recursive — skip
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: mk_binder_chirho("y", 101),
                rhs_chirho: rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ];

        let result_chirho = cse_top_level_chirho(bindings_chirho);
        // Neither should be redirected (x is recursive)
        assert!(matches!(
            result_chirho[0].rhs_chirho,
            CoreExprChirho::PrimOpChirho { .. }
        ));
        assert!(matches!(
            result_chirho[1].rhs_chirho,
            CoreExprChirho::PrimOpChirho { .. }
        ));
    }

    #[test]
    fn cse_let_binding_duplicate_chirho() {
        // let x = +# 1 2; y = +# 1 2 in +# x y
        // → let x = +# 1 2; y = x in +# x y
        let expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![
                (
                    mk_binder_chirho("x", 200),
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                        ],
                    },
                ),
                (
                    mk_binder_chirho("y", 201),
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                        ],
                    },
                ),
            ],
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(CoreIdChirho(200)),
                    CoreExprChirho::VarChirho(CoreIdChirho(201)),
                ],
            }),
        };

        let result_chirho = cse_expr_chirho(&expr_chirho);
        if let CoreExprChirho::LetChirho { binds_chirho, .. } = &result_chirho {
            // y should be redirected to x
            assert_eq!(
                binds_chirho[1].1,
                CoreExprChirho::VarChirho(CoreIdChirho(200))
            );
        } else {
            panic!("expected LetChirho");
        }
    }

    #[test]
    fn cse_let_binding_different_rhs_chirho() {
        // let x = +# 1 2; y = +# 3 4 in ... — different RHSes, no CSE
        let expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![
                (
                    mk_binder_chirho("x", 200),
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                        ],
                    },
                ),
                (
                    mk_binder_chirho("y", 201),
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(3)),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(4)),
                        ],
                    },
                ),
            ],
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(200))),
        };

        let result_chirho = cse_expr_chirho(&expr_chirho);
        if let CoreExprChirho::LetChirho { binds_chirho, .. } = &result_chirho {
            // Both should keep their original RHS
            assert!(matches!(
                binds_chirho[0].1,
                CoreExprChirho::PrimOpChirho { .. }
            ));
            assert!(matches!(
                binds_chirho[1].1,
                CoreExprChirho::PrimOpChirho { .. }
            ));
        } else {
            panic!("expected LetChirho");
        }
    }

    #[test]
    fn cse_redirect_in_body_chirho() {
        // let x = ConApp("True", []); y = ConApp("True", []) in y
        // → let x = ConApp("True", []); y = x in y  — and body references y→x
        let expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![
                (
                    mk_binder_chirho("x", 300),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "True".to_string(),
                        args_chirho: vec![],
                    },
                ),
                (
                    mk_binder_chirho("y", 301),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "True".to_string(),
                        args_chirho: vec![],
                    },
                ),
            ],
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(301))),
        };

        let result_chirho = cse_expr_chirho(&expr_chirho);
        if let CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } = &result_chirho
        {
            // y's RHS redirected to x
            assert_eq!(
                binds_chirho[1].1,
                CoreExprChirho::VarChirho(CoreIdChirho(300))
            );
            // body's reference to y redirected to x
            assert_eq!(**body_chirho, CoreExprChirho::VarChirho(CoreIdChirho(300)));
        } else {
            panic!("expected LetChirho");
        }
    }

    // -----------------------------------------------------------------------
    // Specialization pass tests
    // -----------------------------------------------------------------------

    #[test]
    fn specialize_creates_copy_chirho() {
        // f = \x -> x, with {-# SPECIALIZE f :: Int -> Int #-}
        let mut spec_map_chirho = std::collections::HashMap::new();
        spec_map_chirho.insert("f".to_string(), vec!["Int -> Int".to_string()]);

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 1),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 2),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: spec_map_chirho,
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // Should have 2 bindings: original f + $spec_f_0
        assert_eq!(result_chirho.bindings_chirho.len(), 2);
        assert_eq!(
            result_chirho.bindings_chirho[1].binder_chirho.name_chirho,
            "$spec_f_0"
        );
        assert_eq!(
            result_chirho.bindings_chirho[1].inline_chirho,
            InlineAnnotationChirho::AlwaysChirho
        );
    }

    #[test]
    fn specialize_multiple_types_chirho() {
        // f = \x -> x, with two specializations
        let mut spec_map_chirho = std::collections::HashMap::new();
        spec_map_chirho.insert(
            "f".to_string(),
            vec!["Int -> Int".to_string(), "Double -> Double".to_string()],
        );

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 1),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 2),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: spec_map_chirho,
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // Should have 3 bindings: original f + $spec_f_0 + $spec_f_1
        assert_eq!(result_chirho.bindings_chirho.len(), 3);
        assert_eq!(
            result_chirho.bindings_chirho[1].binder_chirho.name_chirho,
            "$spec_f_0"
        );
        assert_eq!(
            result_chirho.bindings_chirho[2].binder_chirho.name_chirho,
            "$spec_f_1"
        );
    }

    #[test]
    fn specialize_nonexistent_binding_chirho() {
        // SPECIALIZE for a binding that doesn't exist — should be a no-op
        let mut spec_map_chirho = std::collections::HashMap::new();
        spec_map_chirho.insert("nonexistent".to_string(), vec!["Int -> Int".to_string()]);

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 1),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: spec_map_chirho,
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // Should still have just 1 binding — nonexistent target is silently skipped
        assert_eq!(result_chirho.bindings_chirho.len(), 1);
    }

    #[test]
    fn specialize_preserves_rhs_chirho() {
        // Specialized copy should have the same RHS as the original
        let mut spec_map_chirho = std::collections::HashMap::new();
        spec_map_chirho.insert("f".to_string(), vec!["Int -> Int".to_string()]);

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 2),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(CoreIdChirho(2)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            }),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 1),
                rhs_chirho: rhs_chirho.clone(),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: spec_map_chirho,
            foreign_exports_chirho: vec![],
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        assert_eq!(result_chirho.bindings_chirho.len(), 2);
        // The specialized copy's RHS should be structurally identical
        // (after simplification, it may get optimized, but the structure should match)
        assert!(matches!(
            &result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
    }

    // -- Strictness analysis tests --

    #[test]
    fn strictness_case_scrutinee_is_strict_chirho() {
        // \x -> case x of { _ -> 42 }
        // x is strict (used as case scrutinee)
        let x_chirho = dummy_binder_chirho("x", 0);
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                bind_chirho: dummy_binder_chirho("_", 99),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                }],
            }),
        };
        let demands_chirho = analyze_demand_chirho(&expr_chirho);
        assert_eq!(demands_chirho.len(), 1);
        assert_eq!(demands_chirho[0].1, DemandChirho::StrictChirho);
    }

    #[test]
    fn strictness_primop_arg_is_strict_chirho() {
        // \x -> \y -> x +# y
        // Both x and y are strict (primop arguments)
        let x_chirho = dummy_binder_chirho("x", 0);
        let y_chirho = dummy_binder_chirho("y", 1);
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: y_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(0)),
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                    ],
                }),
            }),
        };
        let demands_chirho = analyze_demand_chirho(&expr_chirho);
        assert_eq!(demands_chirho.len(), 2);
        assert_eq!(demands_chirho[0].1, DemandChirho::StrictChirho);
        assert_eq!(demands_chirho[1].1, DemandChirho::StrictChirho);
    }

    #[test]
    fn strictness_unused_var_is_lazy_chirho() {
        // \x -> 42
        // x is lazy (never used)
        let x_chirho = dummy_binder_chirho("x", 0);
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
        };
        let demands_chirho = analyze_demand_chirho(&expr_chirho);
        assert_eq!(demands_chirho.len(), 1);
        assert_eq!(demands_chirho[0].1, DemandChirho::LazyChirho);
    }

    #[test]
    fn strictness_mixed_demands_chirho() {
        // \x -> \y -> case x of { _ -> y }
        // x is strict (case scrutinee), y is lazy (just returned)
        let x_chirho = dummy_binder_chirho("x", 0);
        let y_chirho = dummy_binder_chirho("y", 1);
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: y_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    bind_chirho: dummy_binder_chirho("_", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(1)),
                    }],
                }),
            }),
        };
        let demands_chirho = analyze_demand_chirho(&expr_chirho);
        assert_eq!(demands_chirho.len(), 2);
        assert_eq!(
            demands_chirho[0].1,
            DemandChirho::StrictChirho,
            "x should be strict"
        );
        assert_eq!(
            demands_chirho[1].1,
            DemandChirho::LazyChirho,
            "y should be lazy"
        );
    }

    #[test]
    fn worker_wrapper_creates_worker_chirho() {
        // f = \x -> case x of { _ -> 42 }
        // Should generate wrapper f and worker $wf
        let x_chirho = dummy_binder_chirho("x", 0);
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    bind_chirho: dummy_binder_chirho("_", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                    }],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = worker_wrapper_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 2, "should have wrapper + worker");
        assert_eq!(
            result_chirho[0].binder_chirho.name_chirho, "f",
            "first should be wrapper"
        );
        assert_eq!(
            result_chirho[1].binder_chirho.name_chirho, "$wf",
            "second should be worker"
        );
        assert_eq!(
            result_chirho[1].inline_chirho,
            InlineAnnotationChirho::AlwaysChirho,
            "worker should be marked INLINE"
        );
    }

    #[test]
    fn worker_wrapper_skips_lazy_only_chirho() {
        // f = \x -> 42  (x is lazy — no worker/wrapper needed)
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = worker_wrapper_chirho(vec![binding_chirho]);
        assert_eq!(
            result_chirho.len(),
            1,
            "lazy-only function should not be split"
        );
    }

    #[test]
    fn worker_wrapper_skips_noinline_chirho() {
        // {-# NOINLINE f #-}
        // f = \x -> case x of { _ -> 42 }
        // Should NOT transform despite strict arg
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    bind_chirho: dummy_binder_chirho("_", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                    }],
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NeverChirho,
        };

        let result_chirho = worker_wrapper_chirho(vec![binding_chirho]);
        assert_eq!(
            result_chirho.len(),
            1,
            "NOINLINE function should not be split"
        );
    }

    // -----------------------------------------------------------------------
    // Demand analysis tests
    // -----------------------------------------------------------------------

    #[test]
    fn usage_absent_variable_chirho() {
        // \x -> 42 — x is never used
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
        };
        let usage_chirho = analyze_usage_chirho(&expr_chirho);
        assert_eq!(usage_chirho.len(), 1);
        assert_eq!(usage_chirho[0].1, UsageChirho::AbsentChirho);
    }

    #[test]
    fn usage_used_once_chirho() {
        // \x -> x — x is used exactly once
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        };
        let usage_chirho = analyze_usage_chirho(&expr_chirho);
        assert_eq!(usage_chirho.len(), 1);
        assert_eq!(usage_chirho[0].1, UsageChirho::UsedOnceChirho);
    }

    #[test]
    fn usage_used_many_chirho() {
        // \x -> x +# x — x is used twice
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(CoreIdChirho(0)),
                    CoreExprChirho::VarChirho(CoreIdChirho(0)),
                ],
            }),
        };
        let usage_chirho = analyze_usage_chirho(&expr_chirho);
        assert_eq!(usage_chirho.len(), 1);
        assert_eq!(usage_chirho[0].1, UsageChirho::UsedManyChirho);
    }

    #[test]
    fn usage_multi_arg_mixed_chirho() {
        // \x -> \y -> \z -> x +# z — x used once, y absent, z used once
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("y", 1),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("z", 2),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(CoreIdChirho(0)),
                            CoreExprChirho::VarChirho(CoreIdChirho(2)),
                        ],
                    }),
                }),
            }),
        };
        let usage_chirho = analyze_usage_chirho(&expr_chirho);
        assert_eq!(usage_chirho.len(), 3);
        assert_eq!(
            usage_chirho[0].1,
            UsageChirho::UsedOnceChirho,
            "x used once"
        );
        assert_eq!(usage_chirho[1].1, UsageChirho::AbsentChirho, "y absent");
        assert_eq!(
            usage_chirho[2].1,
            UsageChirho::UsedOnceChirho,
            "z used once"
        );
    }

    #[test]
    fn dead_arg_elim_removes_absent_chirho() {
        // f = \x -> \y -> x — y is dead, should be eliminated
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("y", 1),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 1);
        // Should be \x -> x (y removed)
        if let CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } = &result_chirho[0].rhs_chirho
        {
            assert_eq!(binder_chirho.name_chirho, "x");
            assert!(matches!(
                body_chirho.as_ref(),
                CoreExprChirho::VarChirho(CoreIdChirho(0))
            ));
        } else {
            panic!("expected lambda after dead arg elim");
        }
    }

    #[test]
    fn dead_arg_elim_keeps_used_chirho() {
        // f = \x -> x — x is used, should NOT be eliminated
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 1);
        // Should still be \x -> x (nothing to eliminate)
        if let CoreExprChirho::LamChirho { binder_chirho, .. } = &result_chirho[0].rhs_chirho {
            assert_eq!(binder_chirho.name_chirho, "x");
        } else {
            panic!("expected lambda preserved");
        }
    }

    #[test]
    fn dead_arg_elim_skips_noinline_chirho() {
        // {-# NOINLINE f #-}
        // f = \x -> 42 — x is dead but NOINLINE prevents elimination
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NeverChirho,
        };

        let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 1);
        // Should still be \x -> 42 (NOINLINE preserves everything)
        if let CoreExprChirho::LamChirho { binder_chirho, .. } = &result_chirho[0].rhs_chirho {
            assert_eq!(binder_chirho.name_chirho, "x");
        } else {
            panic!("expected lambda preserved for NOINLINE");
        }
    }

    #[test]
    fn dead_arg_elim_middle_arg_chirho() {
        // f = \x -> \y -> \z -> x +# z — y is dead (middle arg)
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("y", 1),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: dummy_binder_chirho("z", 2),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(CoreIdChirho(0)),
                                CoreExprChirho::VarChirho(CoreIdChirho(2)),
                            ],
                        }),
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = dead_arg_elimination_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 1);
        // Should be \x -> \z -> x +# z (y removed)
        if let CoreExprChirho::LamChirho {
            binder_chirho: b1_chirho,
            body_chirho,
        } = &result_chirho[0].rhs_chirho
        {
            assert_eq!(b1_chirho.name_chirho, "x");
            if let CoreExprChirho::LamChirho {
                binder_chirho: b2_chirho,
                ..
            } = body_chirho.as_ref()
            {
                assert_eq!(
                    b2_chirho.name_chirho, "z",
                    "y should be eliminated, z remains"
                );
            } else {
                panic!("expected second lambda for z");
            }
        } else {
            panic!("expected lambda after dead arg elim");
        }
    }

    #[test]
    fn count_usage_shadowed_chirho() {
        // \x -> let x = 99 in x — the outer x is absent (shadowed by let binding)
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: dummy_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(
                    dummy_binder_chirho("x", 0),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99)),
                )],
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            }),
        };
        let usage_chirho = analyze_usage_chirho(&expr_chirho);
        assert_eq!(usage_chirho.len(), 1);
        // The outer x is shadowed by the let binding, so it's absent
        assert_eq!(usage_chirho[0].1, UsageChirho::AbsentChirho);
    }

    // -----------------------------------------------------------------------
    // SpecConstr tests
    // -----------------------------------------------------------------------

    #[test]
    fn spec_constr_finds_pattern_chirho() {
        // f = \xs -> case xs of
        //   Nil -> 0
        //   Cons x rest -> x +# f rest
        // `f` is recursive, case scrutinee is the param `xs`,
        // recursive call `f rest` passes `rest` (an alt binder).
        let xs_chirho = dummy_binder_chirho("xs", 0);
        let x_chirho = dummy_binder_chirho("x", 1);
        let rest_chirho = dummy_binder_chirho("rest", 2);

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            bind_chirho: dummy_binder_chirho("_", 99),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Nil".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Cons".to_string()),
                    binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                    rhs_chirho: CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(CoreIdChirho(1)),
                            CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(100))),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                            },
                        ],
                    },
                },
            ],
        };

        let patterns_chirho =
            find_call_patterns_chirho(&CoreIdChirho(100), &[xs_chirho.clone()], &body_chirho);
        assert_eq!(patterns_chirho.len(), 1);
        assert_eq!(patterns_chirho[0].arg_idx_chirho, 0);
        assert_eq!(patterns_chirho[0].con_name_chirho, "Cons");
        assert_eq!(patterns_chirho[0].field_binders_chirho.len(), 2);
    }

    #[test]
    fn spec_constr_creates_copy_chirho() {
        // Same as above but wrapped in a CoreBindingChirho
        let xs_chirho = dummy_binder_chirho("xs", 0);
        let x_chirho = dummy_binder_chirho("x", 1);
        let rest_chirho = dummy_binder_chirho("rest", 2);

        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    bind_chirho: dummy_binder_chirho("_", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Nil".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Cons".to_string()),
                            binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                            rhs_chirho: CoreExprChirho::PrimOpChirho {
                                name_chirho: "+#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(CoreIdChirho(1)),
                                    CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                            CoreIdChirho(100),
                                        )),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                            CoreIdChirho(2),
                                        )),
                                    },
                                ],
                            },
                        },
                    ],
                }),
            },
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = spec_constr_chirho(vec![binding_chirho]);
        // Should have original f + $sc_f_Cons
        assert_eq!(result_chirho.len(), 2, "should have original + specialized");
        assert_eq!(result_chirho[0].binder_chirho.name_chirho, "f");
        assert_eq!(result_chirho[1].binder_chirho.name_chirho, "$sc_f_Cons");
        assert!(
            result_chirho[1].is_rec_chirho,
            "specialized copy should be recursive"
        );
        assert_eq!(
            result_chirho[1].inline_chirho,
            InlineAnnotationChirho::AlwaysChirho
        );
    }

    #[test]
    fn spec_constr_skips_non_recursive_chirho() {
        // Non-recursive function should not be spec-constr'd
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = spec_constr_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 1, "non-recursive should pass through");
    }

    #[test]
    fn spec_constr_skips_noinline_chirho() {
        // NOINLINE recursive function should not be spec-constr'd
        let xs_chirho = dummy_binder_chirho("xs", 0);
        let x_chirho = dummy_binder_chirho("x", 1);
        let rest_chirho = dummy_binder_chirho("rest", 2);

        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                    bind_chirho: dummy_binder_chirho("_", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Nil".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Cons".to_string()),
                            binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                            rhs_chirho: CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(100))),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                            },
                        },
                    ],
                }),
            },
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NeverChirho,
        };

        let result_chirho = spec_constr_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 1, "NOINLINE should not be specialized");
    }

    #[test]
    fn spec_constr_no_pattern_found_chirho() {
        // Recursive function that doesn't case-match its arg
        // f = \x -> f (x +# 1) — no constructor pattern
        let binding_chirho = CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(100),
                name_chirho: "f".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(100))),
                    arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(CoreIdChirho(0)),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                }),
            },
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };

        let result_chirho = spec_constr_chirho(vec![binding_chirho]);
        assert_eq!(result_chirho.len(), 1, "no constructor pattern found");
    }
}
