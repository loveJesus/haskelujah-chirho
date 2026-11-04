// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Do-notation, moved whole from the desugar root (row 691, D5): each statement becomes its
//! bind, its sequence or its `let`, through the selected monad's operations, and QualifiedDo
//! takes those operations from the named module.

use super::*;

impl DesugarCtxChirho {
    /// Desugar do-notation statements into Core.
    pub(super) fn desugar_do_chirho(
        &mut self,
        stmts_chirho: &[haskelujah_ast_chirho::expr_chirho::StmtChirho],
        qualifier_chirho: Option<&str>,
    ) -> CoreExprChirho {
        use haskelujah_ast_chirho::expr_chirho::StmtChirho;

        if stmts_chirho.is_empty() {
            return CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0));
        }

        if stmts_chirho.len() == 1 {
            return match &stmts_chirho[0] {
                StmtChirho::ExprChirho { expr_chirho, .. } => self.desugar_expr_chirho(expr_chirho),
                StmtChirho::BindChirho { expr_chirho, .. } => self.desugar_expr_chirho(expr_chirho),
                StmtChirho::LetChirho { .. } => {
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
                }
            };
        }

        match &stmts_chirho[0] {
            StmtChirho::ExprChirho {
                expr_chirho,
                then_chirho,
            } => {
                // e; stmts → (>>) e (do stmts) — rides the per-type dispatch;
                // IO falls back to ThenIOChirho by name at STG (INV-001).
                // workflow: monadic-dispatch-chirho
                let e_chirho = self.desugar_expr_chirho(expr_chirho);
                let rest_chirho = self.desugar_do_chirho(&stmts_chirho[1..], qualifier_chirho);
                let then_id_chirho = self.resolve_selected_operation_chirho(
                    then_chirho.as_ref(),
                    qualifier_chirho,
                    ">>",
                );
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(then_id_chirho)),
                        arg_chirho: Box::new(e_chirho),
                    }),
                    arg_chirho: Box::new(rest_chirho),
                }
            }
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                bind_chirho: bind_selected_chirho,
                fail_chirho: fail_selected_chirho,
                ..
            } => {
                // p <- e; stmts → (>>=) e (\p -> do stmts). Non-Var patterns
                // destructure through a fresh lambda binder + case (they were
                // previously bound whole and silently never matched). IO keeps
                // BindIOChirho via the STG name fallback (INV-001).
                // workflow: monadic-dispatch-chirho
                let e_chirho = self.desugar_expr_chirho(expr_chirho);
                let bind_id_chirho = self.resolve_selected_operation_chirho(
                    bind_selected_chirho.as_ref(),
                    qualifier_chirho,
                    ">>=",
                );
                // A LAZY pattern scrutinises nothing at the binding site: each
                // variable becomes a selector thunk, so an unused one forces
                // nothing, demanding one selects its field, and a mismatching
                // constructor fails only on demand. Taken before the general
                // path, which would build a case and force at the bind.
                // workflow: monadic-dispatch-chirho
                // The selectors are built FIRST, because building them binds the
                // pattern's variables in scope; the rest of the block is
                // desugared afterwards so its references resolve to those thunks.
                if let Some((scrutinee_binder_chirho, selector_binds_chirho)) =
                    self.lazy_bind_prepare_chirho(pat_chirho)
                {
                    let rest_chirho = self.desugar_do_chirho(&stmts_chirho[1..], qualifier_chirho);
                    let lam_chirho = Self::lazy_bind_lambda_chirho(
                        scrutinee_binder_chirho,
                        selector_binds_chirho,
                        rest_chirho,
                    );
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(bind_id_chirho)),
                            arg_chirho: Box::new(e_chirho),
                        }),
                        arg_chirho: Box::new(lam_chirho),
                    };
                }
                let lam_chirho = match pat_chirho {
                    PatChirho::VarChirho(n_chirho) => {
                        let var_name_chirho = n_chirho.text_chirho().to_string();
                        let binder_chirho = self.fresh_binder_chirho(
                            &var_name_chirho,
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        self.bind_in_scope_chirho(&var_name_chirho, binder_chirho.id_chirho);
                        // Compute rest AFTER binding is in scope so subsequent
                        // statements can reference the bound variable.
                        let rest_chirho =
                            self.desugar_do_chirho(&stmts_chirho[1..], qualifier_chirho);
                        CoreExprChirho::LamChirho {
                            binder_chirho,
                            body_chirho: Box::new(rest_chirho),
                        }
                    }
                    PatChirho::WildcardChirho(_) => {
                        let binder_chirho = self.fresh_binder_chirho(
                            "_bind",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let rest_chirho =
                            self.desugar_do_chirho(&stmts_chirho[1..], qualifier_chirho);
                        CoreExprChirho::LamChirho {
                            binder_chirho,
                            body_chirho: Box::new(rest_chirho),
                        }
                    }
                    _ => {
                        // Any other pattern is one ROW: `\$bindpat ->` the
                        // pattern matched in full, the rest of the block once its
                        // variables are bound, and a mismatch ANYWHERE in the
                        // pattern - not only at its top - going to the failure.
                        // workflow: language-features-chirho/pattern-matching-chirho
                        let scrut_binder_chirho = self.row_binder_chirho("$bindpat");
                        let scrut_id_chirho = scrut_binder_chirho.id_chirho;
                        // A statement that SELECTED `fail` fails through it. One
                        // whose pattern cannot fail selected none - the
                        // failability pass cleared the reservation - so no `fail`
                        // has to resolve for a monad that does not have one.
                        //
                        // "Selected none" and "never asked" are different, and
                        // only the first may drop it: a statement that never went
                        // through lowering's selector has no BIND selection
                        // either (Template Haskell builds statements directly),
                        // and keeps the `fail` it has always had.
                        let selector_ran_chirho = bind_selected_chirho.is_some();
                        let emit_fail_chirho =
                            fail_selected_chirho.is_some() || !selector_ran_chirho;
                        // The failure is a placeholder while the rest of the block
                        // is desugared, and `fail` is resolved afterwards, in the
                        // order this statement has always minted its occurrences.
                        let fail_binder_chirho = self.row_binder_chirho("$dofail");
                        let failure_chirho = if emit_fail_chirho {
                            CoreExprChirho::VarChirho(fail_binder_chirho.id_chirho)
                        } else {
                            // Irrefutable by GHC's rule: never reached.
                            self.case_failure_chirho("Non-exhaustive patterns in do binding")
                        };
                        self.push_scope_chirho();
                        let matched_chirho = self.match_row_chirho(
                            pat_chirho,
                            scrut_id_chirho,
                            &failure_chirho,
                            &mut |ctx_chirho| {
                                ctx_chirho.desugar_do_chirho(&stmts_chirho[1..], qualifier_chirho)
                            },
                        );
                        self.pop_scope_chirho();
                        let body_chirho = if emit_fail_chirho {
                            let fail_id_chirho = self.resolve_selected_operation_chirho(
                                fail_selected_chirho.as_ref(),
                                qualifier_chirho,
                                "fail",
                            );
                            let fail_expr_chirho = CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(fail_id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::LitChirho(
                                    CoreLitChirho::StringChirho(
                                        "Pattern match failure in do expression".to_string(),
                                    ),
                                )),
                            };
                            CoreExprChirho::LetChirho {
                                rec_chirho: false,
                                binds_chirho: vec![(fail_binder_chirho, fail_expr_chirho)],
                                body_chirho: Box::new(matched_chirho),
                            }
                        } else {
                            matched_chirho
                        };
                        CoreExprChirho::LamChirho {
                            binder_chirho: scrut_binder_chirho,
                            body_chirho: Box::new(body_chirho),
                        }
                    }
                };
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(bind_id_chirho)),
                        arg_chirho: Box::new(e_chirho),
                    }),
                    arg_chirho: Box::new(lam_chirho),
                }
            }
            // let binds; stmts → let binds in do stmts
            StmtChirho::LetChirho { binds_chirho, .. } => self
                .desugar_let_around_chirho(binds_chirho, |ctx_chirho| {
                    ctx_chirho.desugar_do_chirho(&stmts_chirho[1..], qualifier_chirho)
                }),
        }
    }
}
