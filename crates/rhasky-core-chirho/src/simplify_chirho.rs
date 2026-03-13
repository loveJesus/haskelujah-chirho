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
//!
//! Modelled after GHC's simplifier but drastically reduced in scope. Runs a
//! fixed number of iterations (configurable).

use std::collections::HashSet;

use crate::expr_chirho::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreModuleChirho,
};

/// Configuration for the simplifier.
#[derive(Debug, Clone)]
pub struct SimplifyConfigChirho {
    /// Maximum number of simplification passes.
    pub max_iterations_chirho: usize,
}

impl Default for SimplifyConfigChirho {
    fn default() -> Self {
        Self {
            max_iterations_chirho: 4,
        }
    }
}

/// Run the simplifier on a Core module.
pub fn simplify_module_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &SimplifyConfigChirho,
) -> CoreModuleChirho {
    let mut bindings_chirho = module_chirho.bindings_chirho.clone();

    for _ in 0..config_chirho.max_iterations_chirho {
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
    }
}

/// Collect all free variable IDs in an expression.
fn free_vars_chirho(expr_chirho: &CoreExprChirho) -> HashSet<CoreIdChirho> {
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
            }],
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
}
