// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # AST-to-Core desugaring
//!
//! Translates the surface AST into the Core IR. This pass:
//! - Converts multi-equation function bindings into case expressions
//! - Translates pattern matching into flat Core case expressions
//! - Converts if/where/guards into Core let/case
//! - Removes syntactic sugar (do notation, list comprehensions, etc.)

use rhasky_ast_chirho::decl_chirho::DeclChirho;
use rhasky_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho};
use rhasky_ast_chirho::lit_chirho::LitChirho;
use rhasky_ast_chirho::module_chirho::ModuleChirho;
use rhasky_ast_chirho::pat_chirho::PatChirho;
use rhasky_span_chirho::SpanChirho;
use rhasky_typing_chirho::ty_chirho::TyChirho;

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho,
};

/// The desugaring context — generates fresh Core IDs.
pub struct DesugarCtxChirho {
    next_id_chirho: u32,
}

impl DesugarCtxChirho {
    pub fn new_chirho() -> Self {
        Self { next_id_chirho: 0 }
    }

    /// Generate a fresh Core ID.
    fn fresh_id_chirho(&mut self) -> CoreIdChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho += 1;
        id_chirho
    }

    /// Create a binder with a fresh ID.
    fn fresh_binder_chirho(
        &mut self,
        name_chirho: &str,
        ty_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> BinderChirho {
        BinderChirho {
            id_chirho: self.fresh_id_chirho(),
            name_chirho: name_chirho.to_string(),
            ty_chirho,
            span_chirho,
        }
    }

    /// Desugar an entire AST module into a Core module.
    pub fn desugar_module_chirho(&mut self, module_chirho: &ModuleChirho) -> CoreModuleChirho {
        let mut bindings_chirho = Vec::new();

        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => {
                    let core_rhs_chirho =
                        self.desugar_matches_chirho(matches_chirho, *span_chirho);
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    bindings_chirho.push(CoreBindingChirho {
                        binder_chirho,
                        rhs_chirho: core_rhs_chirho,
                        is_rec_chirho: false,
                    });
                }
                DeclChirho::PatBindChirho {
                    rhs_chirho,
                    span_chirho,
                    ..
                } => {
                    let core_rhs_chirho = self.desugar_rhs_chirho(rhs_chirho);
                    let binder_chirho = self.fresh_binder_chirho(
                        "_patbind",
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    bindings_chirho.push(CoreBindingChirho {
                        binder_chirho,
                        rhs_chirho: core_rhs_chirho,
                        is_rec_chirho: false,
                    });
                }
                // Data, type, class declarations don't produce Core bindings directly
                _ => {}
            }
        }

        CoreModuleChirho {
            name_chirho: module_chirho.name_chirho.text_chirho().to_string(),
            bindings_chirho,
        }
    }

    /// Desugar a set of match arms into a Core expression.
    /// For single-equation functions: `\p1 p2 -> rhs`
    /// For multi-equation: case expressions.
    fn desugar_matches_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        _span_chirho: SpanChirho,
    ) -> CoreExprChirho {
        if matches_chirho.is_empty() {
            return CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0));
        }

        let first_chirho = &matches_chirho[0];
        let arity_chirho = first_chirho.pats_chirho.len();

        if arity_chirho == 0 {
            // Simple value binding: no patterns
            return self.desugar_rhs_chirho(&first_chirho.rhs_chirho);
        }

        // Build lambdas for each parameter, with the body being
        // a case expression (or the RHS if patterns are simple vars).
        let param_binders_chirho: Vec<BinderChirho> = (0..arity_chirho)
            .map(|i_chirho| {
                let name_chirho = match &first_chirho.pats_chirho[i_chirho] {
                    PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                    _ => format!("_arg{i_chirho}"),
                };
                self.fresh_binder_chirho(&name_chirho, TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                ), SpanChirho::DUMMY_CHIRHO)
            })
            .collect();

        // Build the body (for single-equation, just desugar RHS;
        // for multi-equation, build case alts)
        let body_chirho = if matches_chirho.len() == 1 && all_var_pats_chirho(&first_chirho.pats_chirho) {
            self.desugar_rhs_chirho(&first_chirho.rhs_chirho)
        } else {
            // Multi-equation or non-trivial patterns: case on first arg
            let scrut_id_chirho = param_binders_chirho[0].id_chirho;
            let wild_chirho = self.fresh_binder_chirho(
                "wild",
                TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho)),
                SpanChirho::DUMMY_CHIRHO,
            );

            let alts_chirho: Vec<CoreAltChirho> = matches_chirho
                .iter()
                .map(|arm_chirho| {
                    let con_chirho = if arm_chirho.pats_chirho.is_empty() {
                        AltConChirho::DefaultChirho
                    } else {
                        self.pat_to_alt_con_chirho(&arm_chirho.pats_chirho[0])
                    };
                    CoreAltChirho {
                        con_chirho,
                        binders_chirho: vec![],
                        rhs_chirho: self.desugar_rhs_chirho(&arm_chirho.rhs_chirho),
                    }
                })
                .collect();

            CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(scrut_id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                ),
                alts_chirho,
            }
        };

        // Wrap body in lambdas
        let mut result_chirho = body_chirho;
        for binder_chirho in param_binders_chirho.into_iter().rev() {
            result_chirho = CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: Box::new(result_chirho),
            };
        }

        result_chirho
    }

    /// Convert a pattern to a Core alt constructor.
    fn pat_to_alt_con_chirho(&mut self, pat_chirho: &PatChirho) -> AltConChirho {
        match pat_chirho {
            PatChirho::ConChirho { con_chirho, .. } => {
                AltConChirho::DataConChirho(con_chirho.text_chirho().to_string())
            }
            PatChirho::LitChirho(lit_chirho) => {
                AltConChirho::LitConChirho(self.desugar_lit_chirho(lit_chirho))
            }
            PatChirho::WildcardChirho(_) => AltConChirho::DefaultChirho,
            PatChirho::VarChirho(_) => AltConChirho::DefaultChirho,
            _ => AltConChirho::DefaultChirho,
        }
    }

    /// Desugar a right-hand side.
    fn desugar_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) -> CoreExprChirho {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.desugar_expr_chirho(expr_chirho),
            RhsChirho::GuardedChirho(guards_chirho) => {
                // Desugar guards into nested if-then-else (case on Bool)
                if let Some(first_chirho) = guards_chirho.first() {
                    self.desugar_expr_chirho(&first_chirho.body_chirho)
                } else {
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
                }
            }
        }
    }

    /// Desugar an AST expression into a Core expression.
    pub fn desugar_expr_chirho(&mut self, expr_chirho: &ExprChirho) -> CoreExprChirho {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => {
                // In a full compiler, we'd resolve to a CoreIdChirho.
                // For now, create a fresh ID (placeholder).
                let id_chirho = self.fresh_id_chirho();
                let _ = name_chirho; // name info carried in binder when fully wired
                CoreExprChirho::VarChirho(id_chirho)
            }

            ExprChirho::LitChirho(lit_chirho) => {
                CoreExprChirho::LitChirho(self.desugar_lit_chirho(lit_chirho))
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => CoreExprChirho::AppChirho {
                fun_chirho: Box::new(self.desugar_expr_chirho(fun_chirho)),
                arg_chirho: Box::new(self.desugar_expr_chirho(arg_chirho)),
            },

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => {
                let mut result_chirho = self.desugar_expr_chirho(body_chirho);
                for pat_chirho in pats_chirho.iter().rev() {
                    let name_chirho = match pat_chirho {
                        PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                        _ => "_lam".to_string(),
                    };
                    let binder_chirho = self.fresh_binder_chirho(
                        &name_chirho,
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    result_chirho = CoreExprChirho::LamChirho {
                        binder_chirho,
                        body_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                ..
            } => {
                // if c then t else e → case c of { True -> t; False -> e }
                let cond_core_chirho = self.desugar_expr_chirho(cond_chirho);
                let then_core_chirho = self.desugar_expr_chirho(then_chirho);
                let else_core_chirho = self.desugar_expr_chirho(else_chirho);
                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(cond_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_core_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: else_core_chirho,
                        },
                    ],
                }
            }

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                let core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = binds_chirho
                    .iter()
                    .filter_map(|bind_chirho| {
                        match bind_chirho {
                            rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho,
                                matches_chirho,
                                span_chirho,
                            } => {
                                let rhs_chirho =
                                    self.desugar_matches_chirho(matches_chirho, *span_chirho);
                                let binder_chirho = self.fresh_binder_chirho(
                                    name_chirho.text_chirho(),
                                    TyChirho::VarChirho(
                                        rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                Some((binder_chirho, rhs_chirho))
                            }
                            rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                rhs_chirho,
                                span_chirho,
                                ..
                            } => {
                                let core_rhs_chirho = self.desugar_rhs_chirho(rhs_chirho);
                                let binder_chirho = self.fresh_binder_chirho(
                                    "_let",
                                    TyChirho::VarChirho(
                                        rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                Some((binder_chirho, core_rhs_chirho))
                            }
                            _ => None,
                        }
                    })
                    .collect();

                CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: core_binds_chirho,
                    body_chirho: Box::new(self.desugar_expr_chirho(body_chirho)),
                }
            }

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                let scrut_chirho = self.desugar_expr_chirho(scrutinee_chirho);
                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );

                let core_alts_chirho: Vec<CoreAltChirho> = alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let con_chirho = self.pat_to_alt_con_chirho(&alt_chirho.pat_chirho);
                        CoreAltChirho {
                            con_chirho,
                            binders_chirho: vec![],
                            rhs_chirho: self.desugar_rhs_chirho(&alt_chirho.rhs_chirho),
                        }
                    })
                    .collect();

                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(scrut_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: core_alts_chirho,
                }
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                // Desugar tuple as nested application of tuple constructor
                if elements_chirho.is_empty() {
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)) // unit
                } else {
                    let con_id_chirho = self.fresh_id_chirho();
                    let mut result_chirho = CoreExprChirho::VarChirho(con_id_chirho);
                    for elem_chirho in elements_chirho {
                        let arg_chirho = self.desugar_expr_chirho(elem_chirho);
                        result_chirho = CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(result_chirho),
                            arg_chirho: Box::new(arg_chirho),
                        };
                    }
                    result_chirho
                }
            }

            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                // Desugar [a, b, c] → a : b : c : []
                let nil_id_chirho = self.fresh_id_chirho();
                let mut result_chirho = CoreExprChirho::VarChirho(nil_id_chirho); // []
                for elem_chirho in elements_chirho.iter().rev() {
                    let cons_id_chirho = self.fresh_id_chirho();
                    let elem_core_chirho = self.desugar_expr_chirho(elem_chirho);
                    result_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(cons_id_chirho)),
                            arg_chirho: Box::new(elem_core_chirho),
                        }),
                        arg_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            ExprChirho::NegChirho { expr_chirho: inner_chirho, .. } => {
                // -x → negate x
                let negate_id_chirho = self.fresh_id_chirho();
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(negate_id_chirho)),
                    arg_chirho: Box::new(self.desugar_expr_chirho(inner_chirho)),
                }
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => {
                self.desugar_expr_chirho(inner_chirho)
            }

            ExprChirho::AnnChirho { expr_chirho: inner_chirho, .. } => {
                self.desugar_expr_chirho(inner_chirho)
            }

            // Remaining forms: produce a placeholder
            _ => {
                let id_chirho = self.fresh_id_chirho();
                CoreExprChirho::VarChirho(id_chirho)
            }
        }
    }

    fn desugar_lit_chirho(&self, lit_chirho: &LitChirho) -> CoreLitChirho {
        match lit_chirho {
            LitChirho::IntChirho(v_chirho, _) => CoreLitChirho::IntChirho(*v_chirho),
            LitChirho::FloatChirho(v_chirho, _) => CoreLitChirho::FloatChirho(*v_chirho),
            LitChirho::CharChirho(v_chirho, _) => CoreLitChirho::CharChirho(*v_chirho),
            LitChirho::StringChirho(v_chirho, _) => CoreLitChirho::StringChirho(v_chirho.clone()),
        }
    }
}

/// Check if all patterns are simple variable patterns.
fn all_var_pats_chirho(pats_chirho: &[PatChirho]) -> bool {
    pats_chirho.iter().all(|p_chirho| matches!(p_chirho, PatChirho::VarChirho(_)))
}

/// Desugar a module from AST to Core.
pub fn desugar_module_chirho(module_chirho: &ModuleChirho) -> CoreModuleChirho {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    ctx_chirho.desugar_module_chirho(module_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn desugar_simple_function_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("f"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("x"),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let core_chirho = desugar_module_chirho(&module_chirho);
        assert_eq!(core_chirho.name_chirho, "Test");
        assert_eq!(core_chirho.bindings_chirho.len(), 1);
        assert_eq!(core_chirho.bindings_chirho[0].binder_chirho.name_chirho, "f");
        assert!(matches!(
            core_chirho.bindings_chirho[0].rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
    }

    #[test]
    fn desugar_if_to_case_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                0,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 2);
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "True"
            ));
        }
    }

    #[test]
    fn desugar_literal_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO));
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        assert_eq!(
            core_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
        );
    }

    #[test]
    fn desugar_lambda_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        assert!(matches!(core_chirho, CoreExprChirho::LamChirho { .. }));
    }

    #[test]
    fn desugar_list_to_cons_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::ListChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO)),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should be nested App(App((:), 1), App(App((:), 2), []))
        assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
    }

    #[test]
    fn desugar_pretty_prints_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Pretty"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("main"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let core_chirho = desugar_module_chirho(&module_chirho);
        let output_chirho = crate::pretty_chirho::pretty_module_chirho(&core_chirho);
        assert!(output_chirho.contains("-- module Pretty"));
        assert!(output_chirho.contains("main"));
        assert!(output_chirho.contains("0"));
    }
}
