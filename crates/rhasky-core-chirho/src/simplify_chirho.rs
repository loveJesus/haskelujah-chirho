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
//!
//! Modelled after GHC's simplifier but drastically reduced in scope. Runs a
//! fixed number of iterations (configurable).

use std::collections::{HashMap, HashSet};

use crate::expr_chirho::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
    CoreModuleChirho, InlineAnnotationChirho,
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
}

impl Default for SimplifyConfigChirho {
    fn default() -> Self {
        Self {
            max_iterations_chirho: 4,
            inline_threshold_chirho: AUTO_INLINE_THRESHOLD_CHIRHO,
        }
    }
}

/// Count the number of AST nodes in a Core expression (used for inline size heuristics).
pub fn expr_size_chirho(expr_chirho: &CoreExprChirho) -> usize {
    match expr_chirho {
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => 1,
        CoreExprChirho::AppChirho { fun_chirho, arg_chirho } => {
            1 + expr_size_chirho(fun_chirho) + expr_size_chirho(arg_chirho)
        }
        CoreExprChirho::LamChirho { body_chirho, .. } => 1 + expr_size_chirho(body_chirho),
        CoreExprChirho::LetChirho { binds_chirho, body_chirho, .. } => {
            1 + binds_chirho.iter().map(|(_, rhs_chirho)| expr_size_chirho(rhs_chirho)).sum::<usize>()
                + expr_size_chirho(body_chirho)
        }
        CoreExprChirho::CaseChirho { scrutinee_chirho, alts_chirho, .. } => {
            1 + expr_size_chirho(scrutinee_chirho)
                + alts_chirho.iter().map(|a_chirho| expr_size_chirho(&a_chirho.rhs_chirho)).sum::<usize>()
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => 1 + expr_size_chirho(body_chirho),
        CoreExprChirho::TyAppChirho { expr_chirho: inner_chirho, .. } => 1 + expr_size_chirho(inner_chirho),
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
            1 + args_chirho.iter().map(|a_chirho| expr_size_chirho(a_chirho)).sum::<usize>()
        }
        CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            1 + args_chirho.iter().map(|a_chirho| expr_size_chirho(a_chirho)).sum::<usize>()
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
                if !binding_chirho.is_rec_chirho
                    && is_trivial_chirho(&binding_chirho.rhs_chirho)
                {
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
        CoreExprChirho::AppChirho { fun_chirho, arg_chirho } => CoreExprChirho::AppChirho {
            fun_chirho: Box::new(inline_expr_chirho(fun_chirho, env_chirho)),
            arg_chirho: Box::new(inline_expr_chirho(arg_chirho, env_chirho)),
        },
        CoreExprChirho::LamChirho { binder_chirho, body_chirho } => {
            // If the lambda binder shadows an inline candidate, remove it from env
            let mut env2_chirho = env_chirho.clone();
            env2_chirho.remove(&binder_chirho.id_chirho);
            CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho.clone(),
                body_chirho: Box::new(inline_expr_chirho(body_chirho, &env2_chirho)),
            }
        }
        CoreExprChirho::LetChirho { rec_chirho, binds_chirho, body_chirho } => {
            let mut env2_chirho = env_chirho.clone();
            for (b_chirho, _) in binds_chirho {
                env2_chirho.remove(&b_chirho.id_chirho);
            }
            CoreExprChirho::LetChirho {
                rec_chirho: *rec_chirho,
                binds_chirho: binds_chirho.iter().map(|(b_chirho, rhs_chirho)| {
                    (b_chirho.clone(), inline_expr_chirho(rhs_chirho, &env2_chirho))
                }).collect(),
                body_chirho: Box::new(inline_expr_chirho(body_chirho, &env2_chirho)),
            }
        }
        CoreExprChirho::CaseChirho { scrutinee_chirho, bind_chirho, result_ty_chirho, alts_chirho } => {
            let mut env2_chirho = env_chirho.clone();
            env2_chirho.remove(&bind_chirho.id_chirho);
            CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(inline_expr_chirho(scrutinee_chirho, env_chirho)),
                bind_chirho: bind_chirho.clone(),
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: alts_chirho.iter().map(|alt_chirho| {
                    let mut alt_env_chirho = env2_chirho.clone();
                    for b_chirho in &alt_chirho.binders_chirho {
                        alt_env_chirho.remove(&b_chirho.id_chirho);
                    }
                    CoreAltChirho {
                        con_chirho: alt_chirho.con_chirho.clone(),
                        binders_chirho: alt_chirho.binders_chirho.clone(),
                        rhs_chirho: inline_expr_chirho(&alt_chirho.rhs_chirho, &alt_env_chirho),
                    }
                }).collect(),
            }
        }
        CoreExprChirho::TyLamChirho { ty_var_chirho, body_chirho } => CoreExprChirho::TyLamChirho {
            ty_var_chirho: ty_var_chirho.clone(),
            body_chirho: Box::new(inline_expr_chirho(body_chirho, env_chirho)),
        },
        CoreExprChirho::TyAppChirho { expr_chirho: inner_chirho, ty_chirho } => CoreExprChirho::TyAppChirho {
            expr_chirho: Box::new(inline_expr_chirho(inner_chirho, env_chirho)),
            ty_chirho: ty_chirho.clone(),
        },
        CoreExprChirho::PrimOpChirho { name_chirho, args_chirho } => CoreExprChirho::PrimOpChirho {
            name_chirho: name_chirho.clone(),
            args_chirho: args_chirho.iter().map(|a_chirho| inline_expr_chirho(a_chirho, env_chirho)).collect(),
        },
        CoreExprChirho::ConAppChirho { con_name_chirho, args_chirho } => CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho.clone(),
            args_chirho: args_chirho.iter().map(|a_chirho| inline_expr_chirho(a_chirho, env_chirho)).collect(),
        },
    }
}

/// Run the simplifier on a Core module.
///
/// Each iteration performs:
/// 1. Inlining — substitute small/INLINE bindings at call sites
/// 2. Simplification — beta reduction, dead binding elimination, case-of-known,
///    constant folding
pub fn simplify_module_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &SimplifyConfigChirho,
) -> CoreModuleChirho {
    let mut bindings_chirho = module_chirho.bindings_chirho.clone();

    for _ in 0..config_chirho.max_iterations_chirho {
        // Phase 1: Build inline environment and inline variables
        let inline_env_chirho = build_inline_env_chirho(&bindings_chirho, config_chirho.inline_threshold_chirho);
        if !inline_env_chirho.is_empty() {
            bindings_chirho = bindings_chirho.into_iter().map(|mut b_chirho| {
                b_chirho.rhs_chirho = inline_expr_chirho(&b_chirho.rhs_chirho, &inline_env_chirho);
                b_chirho
            }).collect();
        }

        // Phase 2: Standard simplification
        let new_bindings_chirho: Vec<CoreBindingChirho> = bindings_chirho
            .into_iter()
            .map(|mut binding_chirho| {
                binding_chirho.rhs_chirho =
                    simplify_expr_chirho(&binding_chirho.rhs_chirho);
                binding_chirho
            })
            .collect();
        bindings_chirho = new_bindings_chirho;
    }

    CoreModuleChirho {
        name_chirho: module_chirho.name_chirho.clone(),
        bindings_chirho,
        names_chirho: module_chirho.names_chirho.clone(),
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
                let substituted_chirho =
                    subst_var_chirho(&body_chirho, binder_chirho.id_chirho, &arg_simplified_chirho);
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
            fun_chirho: Box::new(subst_var_chirho(fun_chirho, var_id_chirho, replacement_chirho)),
            arg_chirho: Box::new(subst_var_chirho(arg_chirho, var_id_chirho, replacement_chirho)),
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
        "$sel_Num_+" => Some("+#"),
        "$sel_Num_-" => Some("-#"),
        "$sel_Num_*" => Some("*#"),
        "$sel_Num_negate" => Some("negate#"),
        "$sel_Eq_==" => Some("==#"),
        "$sel_Ord_compare" => Some("compare#"),
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
            // $sel_Num_fromInteger dict lit → lit
            if name_chirho == "$sel_Num_fromInteger" && all_args_chirho.len() >= 2 {
                let simplified_chirho = elide_dicts_chirho(all_args_chirho[1], all_bindings_chirho);
                if let CoreExprChirho::LitChirho(lit_chirho) = &simplified_chirho {
                    return CoreExprChirho::LitChirho(lit_chirho.clone());
                }
                return simplified_chirho;
            }

            // $sel_Num_+ dict x y → +# x y (and similar binary/unary primops)
            if let Some(primop_chirho) = selector_primop_chirho(&name_chirho) {
                if all_args_chirho.len() >= 3 {
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
    if let CoreExprChirho::LamChirho { binder_chirho, body_chirho } = expr_chirho {
        if binder_chirho.name_chirho.starts_with("$d") {
            return elide_dicts_chirho(body_chirho, all_bindings_chirho);
        }
    }

    // Recurse
    match expr_chirho {
        CoreExprChirho::AppChirho { fun_chirho, arg_chirho } => {
            let sf_chirho = elide_dicts_chirho(fun_chirho, all_bindings_chirho);
            let sa_chirho = elide_dicts_chirho(arg_chirho, all_bindings_chirho);
            // Skip dict arguments ($f-prefixed binding refs)
            if let CoreExprChirho::VarChirho(id_chirho) = &sa_chirho {
                if let Some(n_chirho) = resolve_name_for_id_chirho(id_chirho, all_bindings_chirho) {
                    if n_chirho.starts_with("$f") || n_chirho.starts_with("$d") {
                        return sf_chirho;
                    }
                }
            }
            CoreExprChirho::AppChirho {
                fun_chirho: Box::new(sf_chirho),
                arg_chirho: Box::new(sa_chirho),
            }
        }
        CoreExprChirho::LamChirho { binder_chirho, body_chirho } => CoreExprChirho::LamChirho {
            binder_chirho: binder_chirho.clone(),
            body_chirho: Box::new(elide_dicts_chirho(body_chirho, all_bindings_chirho)),
        },
        CoreExprChirho::LetChirho { rec_chirho, binds_chirho, body_chirho } => CoreExprChirho::LetChirho {
            rec_chirho: *rec_chirho,
            binds_chirho: binds_chirho
                .iter()
                .map(|(b_chirho, r_chirho)| (b_chirho.clone(), elide_dicts_chirho(r_chirho, all_bindings_chirho)))
                .collect(),
            body_chirho: Box::new(elide_dicts_chirho(body_chirho, all_bindings_chirho)),
        },
        CoreExprChirho::CaseChirho {
            scrutinee_chirho, bind_chirho, result_ty_chirho, alts_chirho,
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
        CoreExprChirho::PrimOpChirho { name_chirho, args_chirho } => CoreExprChirho::PrimOpChirho {
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
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::expr_chirho::{BinderChirho, CoreLitChirho};
    use rhasky_span_chirho::SpanChirho;
    use rhasky_typing_chirho::ty_chirho::TyChirho;

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
        assert_eq!(expr_size_chirho(&CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))), 1);
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
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should be the lambda from f, not Var(10)
        assert!(matches!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
    }
}
