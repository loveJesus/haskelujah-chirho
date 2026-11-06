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
        // Copies made here (specializations, workers) take new ids outside every role, so a
        // backend treats them as program code (row 628).
        provenance_chirho: module_chirho.provenance_chirho.clone(),
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
                if *rec_chirho {
                    if let Some(ordered_binds_chirho) =
                        reorder_acyclic_rec_let_binds_chirho(&simplified_binds_chirho)
                    {
                        nest_nonrec_let_binds_chirho(ordered_binds_chirho, body_simplified_chirho)
                    } else {
                        CoreExprChirho::LetChirho {
                            rec_chirho: true,
                            binds_chirho: simplified_binds_chirho,
                            body_chirho: Box::new(body_simplified_chirho),
                        }
                    }
                } else {
                    CoreExprChirho::LetChirho {
                        rec_chirho: false,
                        binds_chirho: simplified_binds_chirho,
                        body_chirho: Box::new(body_simplified_chirho),
                    }
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

fn reorder_acyclic_rec_let_binds_chirho(
    binds_chirho: &[(BinderChirho, CoreExprChirho)],
) -> Option<Vec<(BinderChirho, CoreExprChirho)>> {
    let binder_ids_chirho: HashSet<CoreIdChirho> = binds_chirho
        .iter()
        .map(|(binder_chirho, _)| binder_chirho.id_chirho)
        .collect();
    let mut remaining_ids_chirho: Vec<CoreIdChirho> = binds_chirho
        .iter()
        .map(|(binder_chirho, _)| binder_chirho.id_chirho)
        .collect();
    let bind_map_chirho: HashMap<CoreIdChirho, (BinderChirho, CoreExprChirho)> = binds_chirho
        .iter()
        .cloned()
        .map(|(binder_chirho, rhs_chirho)| (binder_chirho.id_chirho, (binder_chirho, rhs_chirho)))
        .collect();
    let mut deps_by_id_chirho: HashMap<CoreIdChirho, HashSet<CoreIdChirho>> = binds_chirho
        .iter()
        .map(|(binder_chirho, rhs_chirho)| {
            let deps_chirho = free_vars_chirho(rhs_chirho)
                .into_iter()
                .filter(|dep_id_chirho| binder_ids_chirho.contains(dep_id_chirho))
                .collect();
            (binder_chirho.id_chirho, deps_chirho)
        })
        .collect();
    let mut ordered_binds_chirho = Vec::with_capacity(binds_chirho.len());

    while !remaining_ids_chirho.is_empty() {
        let Some(next_idx_chirho) = remaining_ids_chirho.iter().position(|binder_id_chirho| {
            deps_by_id_chirho
                .get(binder_id_chirho)
                .is_some_and(HashSet::is_empty)
        }) else {
            return None;
        };
        let next_id_chirho = remaining_ids_chirho.remove(next_idx_chirho);
        if let Some(bind_pair_chirho) = bind_map_chirho.get(&next_id_chirho).cloned() {
            ordered_binds_chirho.push(bind_pair_chirho);
        }
        for deps_chirho in deps_by_id_chirho.values_mut() {
            deps_chirho.remove(&next_id_chirho);
        }
    }

    Some(ordered_binds_chirho)
}

fn nest_nonrec_let_binds_chirho(
    binds_chirho: Vec<(BinderChirho, CoreExprChirho)>,
    body_chirho: CoreExprChirho,
) -> CoreExprChirho {
    binds_chirho
        .into_iter()
        .rev()
        .fold(body_chirho, |inner_body_chirho, bind_chirho| {
            CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![bind_chirho],
                body_chirho: Box::new(inner_body_chirho),
            }
        })
}

/// Substitute all occurrences of `var_id` with `replacement` in `expr`.
pub(crate) fn subst_var_chirho(
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
mod dictionaries_chirho;
pub use dictionaries_chirho::{elide_dicts_and_filter_chirho, elide_dicts_chirho};
#[cfg(test)]
mod dictionary_tests_chirho;

// ── Newtype constructor erasure for GND-represented newtypes (row 694) ──
mod newtype_erasure_chirho;
pub use newtype_erasure_chirho::erase_newtype_constructors_chirho;
#[cfg(test)]
mod newtype_erasure_tests_chirho;

// ── Shared builders for the simplifier's unit tests ──
#[cfg(test)]
mod test_support_chirho;

// ── Common subexpression elimination (CSE), phase 3 ──
#[path = "simplify_chirho/passes_chirho/common_subexpressions_chirho.rs"]
mod common_subexpressions_chirho;
use common_subexpressions_chirho::{cse_expr_chirho, cse_top_level_chirho};

// ── SPECIALIZE binding specialization, phase 4 ──
#[path = "simplify_chirho/passes_chirho/binding_specialization_chirho.rs"]
mod binding_specialization_chirho;
use binding_specialization_chirho::specialize_bindings_chirho;

// ── Strictness analysis and the worker/wrapper transform, phase 5 ──
#[path = "simplify_chirho/passes_chirho/strictness_chirho.rs"]
mod strictness_chirho;
pub use strictness_chirho::{DemandChirho, worker_wrapper_chirho};

// ── Absence analysis and dead argument elimination, phase 6 ──
#[path = "simplify_chirho/passes_chirho/dead_arguments_chirho.rs"]
mod dead_arguments_chirho;
pub use dead_arguments_chirho::{
    UsageChirho, analyze_usage_chirho, count_usage_chirho, dead_arg_elimination_chirho,
};

// ── Constructor specialization (SpecConstr), phase 7 ──
#[path = "simplify_chirho/passes_chirho/constructor_specialization_chirho.rs"]
mod constructor_specialization_chirho;
pub use constructor_specialization_chirho::spec_constr_chirho;

#[cfg(test)]
mod tests_chirho {
    use super::test_support_chirho::dummy_binder_chirho;
    use super::*;
    use crate::expr_chirho::CoreLitChirho;
    use haskelujah_typing_chirho::ty_chirho::TyChirho;

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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
        };
        let config_chirho = SimplifyConfigChirho::default();
        let result_chirho = super::simplify_module_chirho(&module_chirho, &config_chirho);
        // g should be the lambda from f, not Var(10)
        assert!(matches!(
            result_chirho.bindings_chirho[1].rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
    }

    #[test]
    fn simplify_reorders_acyclic_rec_let_binds_chirho() {
        let result_binder_chirho = dummy_binder_chirho("result", 1);
        let a_binder_chirho = dummy_binder_chirho("a", 2);
        let b_binder_chirho = dummy_binder_chirho("b", 3);
        let expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![
                (
                    result_binder_chirho.clone(),
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_binder_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_binder_chirho.id_chirho),
                        ],
                    },
                ),
                (
                    a_binder_chirho.clone(),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                ),
                (
                    b_binder_chirho.clone(),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(15)),
                ),
            ],
            body_chirho: Box::new(CoreExprChirho::VarChirho(result_binder_chirho.id_chirho)),
        };

        let simplified_chirho = simplify_expr_chirho(&expr_chirho);
        let mut bind_order_chirho = Vec::new();
        let mut cursor_chirho = &simplified_chirho;
        loop {
            match cursor_chirho {
                CoreExprChirho::LetChirho {
                    rec_chirho,
                    binds_chirho,
                    body_chirho,
                } => {
                    assert!(
                        !rec_chirho,
                        "acyclic letrec should become nested non-rec lets"
                    );
                    assert_eq!(
                        binds_chirho.len(),
                        1,
                        "acyclic letrec should lower to one binding per let"
                    );
                    bind_order_chirho.push(binds_chirho[0].0.id_chirho);
                    cursor_chirho = body_chirho;
                }
                CoreExprChirho::VarChirho(id_chirho) => {
                    assert_eq!(*id_chirho, result_binder_chirho.id_chirho);
                    break;
                }
                _ => panic!("expected nested let chain ending in result var"),
            }
        }
        assert_eq!(
            bind_order_chirho,
            vec![
                a_binder_chirho.id_chirho,
                b_binder_chirho.id_chirho,
                result_binder_chirho.id_chirho
            ]
        );
    }

    #[test]
    fn simplify_preserves_recursive_let_cycle_chirho() {
        let f_binder_chirho = dummy_binder_chirho("f", 10);
        let expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![(
                f_binder_chirho.clone(),
                CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 11),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_binder_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            0,
                        ))),
                    }),
                },
            )],
            body_chirho: Box::new(CoreExprChirho::VarChirho(f_binder_chirho.id_chirho)),
        };

        let simplified_chirho = simplify_expr_chirho(&expr_chirho);
        let CoreExprChirho::LetChirho { rec_chirho, .. } = simplified_chirho else {
            panic!("expected LetChirho");
        };
        assert!(rec_chirho, "true recursive lets should stay recursive");
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
            provenance_chirho: Default::default(),
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
}
