// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! List comprehensions, moved whole from the desugar root (row 691, D6): ordinary
//! comprehensions through a continuation over their qualifiers (generators, guards and
//! `let`s), parallel comprehensions by zipping each branch's bound names, and the append
//! helper kept for reference.

use super::*;
use haskelujah_ast_chirho::expr_chirho::ComprehensionQualifierChirho;

mod parallel_chirho;
mod transforms_chirho;

impl DesugarCtxChirho {
    /// Desugar list comprehension body + qualifiers.
    ///
    /// ```haskell
    /// [e | p <- xs, guard, let b = x, rest]
    /// ```
    /// becomes (recursively):
    /// - Generator: `concatMap (\p -> [e | rest]) xs`
    /// - Guard: `if guard then [e | rest] else []`
    /// - Let: `let binds in [e | rest]`
    /// - Empty quals: `[e]` (singleton list)
    ///
    /// workflow: parallel-list-comprehensions-chirho
    pub(super) fn desugar_list_comp_chirho(
        &mut self,
        body_chirho: &ExprChirho,
        quals_chirho: &[ComprehensionQualifierChirho],
    ) -> CoreExprChirho {
        use haskelujah_ast_chirho::expr_chirho::StmtChirho;

        if let Some(index_chirho) = quals_chirho.iter().rposition(|qualifier_chirho| {
            matches!(
                qualifier_chirho,
                ComprehensionQualifierChirho::TransformChirho(_)
            )
        }) {
            return self.desugar_comprehension_transform_chirho(
                &mut |context_chirho| context_chirho.desugar_expr_chirho(body_chirho),
                quals_chirho,
                index_chirho,
                Self::comprehension_nil_chirho(),
            );
        }
        if quals_chirho.is_empty() {
            // Base case: [e] — singleton list (:) e []
            let body_core_chirho = self.desugar_expr_chirho(body_chirho);
            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };
            return CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![body_core_chirho, nil_chirho],
            };
        }

        let rest_chirho = &quals_chirho[1..];

        let ComprehensionQualifierChirho::StatementChirho(statement_chirho) = &quals_chirho[0]
        else {
            unreachable!("transform dispatched before ordinary qualifier lowering")
        };
        match statement_chirho.as_ref() {
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                // p <- xs  →  letrec go = \$xs -> case $xs of
                //                [] -> []
                //                (:) p $rest -> <inner> ++ go $rest
                //   in go xs
                //
                // where <inner> is [e | rest_quals] (a list).
                //
                // For the common case of no remaining quals, <inner> is
                // the singleton [e], so the result is just (:) e (go $rest).
                // When there ARE remaining quals, <inner> can be empty (guards
                // may reject), so we need list append.  We inline a simple
                // recursive append to avoid needing a global concatMap.

                let xs_core_chirho = self.desugar_expr_chirho(expr_chirho);

                let var_name_chirho = match pat_chirho {
                    PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                    _ => "$elem".to_string(),
                };

                // Create the recursive "go" function
                let go_name_chirho = format!("$lc_go_{}", self.next_id_chirho);
                let go_id_chirho = {
                    let id_chirho = CoreIdChirho(self.next_id_chirho);
                    self.next_id_chirho += 1;
                    self.names_chirho.insert(id_chirho, go_name_chirho.clone());
                    id_chirho
                };

                let list_ty_chirho = TyChirho::VarChirho(
                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                );

                // \$xs -> case $xs of ...
                let xs_binder_chirho = self.fresh_binder_chirho(
                    "$xs",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let xs_id_chirho = xs_binder_chirho.id_chirho;

                let wild_chirho = self.fresh_binder_chirho(
                    "$w",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                // Element binder (p) and tail binder ($rest)
                self.push_scope_chirho();
                let elem_binder_chirho = self.fresh_binder_chirho(
                    &var_name_chirho,
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );

                let tail_binder_chirho = self.fresh_binder_chirho(
                    "$rest",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                // go $rest (recursive call on tail)
                let go_rest_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(tail_binder_chirho.id_chirho)),
                };

                // Build the cons-case RHS while var is in scope.
                // If rest_quals is empty, inner is a singleton [e] = (:) e [].
                // We optimize: (:) e (go $rest) directly.
                // If rest_quals is non-empty, we thread the continuation
                // (go $rest) as the nil-case of the inner loop, avoiding
                // the need for list append and nested letrecs with captures.
                // The element is matched against the generator's pattern: a
                // mismatch SKIPS it (`go $rest`), as Haskell specifies, and a
                // match binds the pattern's variables for the rest.
                // workflow: language-features-chirho/pattern-matching-chirho
                let skip_chirho = go_rest_chirho.clone();
                let elem_id_chirho = elem_binder_chirho.id_chirho;
                let cons_rhs_chirho = self.match_row_chirho(
                    pat_chirho,
                    elem_id_chirho,
                    &skip_chirho,
                    &mut |ctx_chirho| {
                        if rest_chirho.is_empty() {
                            // Simple case: (:) e (go $rest) — body desugared here
                            // while the loop variable is still in scope.
                            let body_core_chirho = ctx_chirho.desugar_expr_chirho(body_chirho);
                            CoreExprChirho::ConAppChirho {
                                con_name_chirho: ":".to_string(),
                                args_chirho: vec![body_core_chirho, go_rest_chirho.clone()],
                            }
                        } else {
                            // Multi-generator: thread the continuation through.
                            // [e | y <- ys, ...rest] with continuation k becomes:
                            //   letrec inner_go = \$ys -> case $ys of
                            //     [] -> k
                            //     (:) y $rest2 -> [e | ...rest] with cont (inner_go $rest2)
                            //   in inner_go ys
                            //
                            // For a single remaining bind qualifier y <- ys:
                            //   letrec inner_go = \$ys -> case $ys of
                            //     [] -> go $rest
                            //     (:) y $rest2 -> e : inner_go $rest2
                            //   in inner_go ys
                            ctx_chirho.desugar_list_comp_with_cont_chirho(
                                body_chirho,
                                rest_chirho,
                                go_rest_chirho.clone(),
                            )
                        }
                    },
                );
                self.pop_scope_chirho();

                // case $xs of { [] -> []; (:) p $rest -> <cons_rhs> }
                let case_body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_id_chirho)),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: list_ty_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::ConAppChirho {
                                con_name_chirho: "[]".to_string(),
                                args_chirho: vec![],
                            },
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho(":".to_string()),
                            binders_chirho: vec![elem_binder_chirho, tail_binder_chirho],
                            rhs_chirho: cons_rhs_chirho,
                        },
                    ],
                };

                let go_body_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: xs_binder_chirho,
                    body_chirho: Box::new(case_body_chirho),
                };

                let go_binder_chirho = BinderChirho {
                    id_chirho: go_id_chirho,
                    name_chirho: go_name_chirho,
                    ty_chirho: list_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };

                // letrec go = \$xs -> case ... in go xs
                CoreExprChirho::LetChirho {
                    rec_chirho: true,
                    binds_chirho: vec![(go_binder_chirho, go_body_chirho)],
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                        arg_chirho: Box::new(xs_core_chirho),
                    }),
                }
            }
            StmtChirho::ExprChirho {
                expr_chirho: guard_expr_chirho,
                ..
            } => {
                // Guard: if guard then [e | rest] else []
                let guard_core_chirho = self.desugar_expr_chirho(guard_expr_chirho);
                let then_chirho = self.desugar_list_comp_chirho(body_chirho, rest_chirho);
                let else_chirho = CoreExprChirho::ConAppChirho {
                    con_name_chirho: "[]".to_string(),
                    args_chirho: vec![],
                };

                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(guard_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: else_chirho,
                        },
                    ],
                }
            }
            // let binds in [e | rest]
            StmtChirho::LetChirho { binds_chirho, .. } => self
                .desugar_let_around_chirho(binds_chirho, |ctx_chirho| {
                    ctx_chirho.desugar_list_comp_chirho(body_chirho, rest_chirho)
                }),
        }
    }

    /// Desugar a list comprehension with a continuation expression.
    ///
    /// Instead of producing a standalone list and appending it to the
    /// continuation, this threads the continuation as the nil case of
    /// the innermost loop, avoiding the need for list append and
    /// the associated letrec-with-captures issue in the STG lowerer.
    ///
    /// `[e | q1, q2, ...] with continuation k` produces code where
    /// exhausting the list returns `k` instead of `[]`.
    fn desugar_list_comp_with_cont_chirho(
        &mut self,
        body_chirho: &ExprChirho,
        quals_chirho: &[ComprehensionQualifierChirho],
        cont_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        self.desugar_comprehension_result_chirho(
            quals_chirho,
            &mut |context_chirho| context_chirho.desugar_expr_chirho(body_chirho),
            cont_chirho,
        )
    }

    fn desugar_comprehension_result_chirho(
        &mut self,
        quals_chirho: &[ComprehensionQualifierChirho],
        result_chirho: &mut dyn FnMut(&mut Self) -> CoreExprChirho,
        cont_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        use haskelujah_ast_chirho::expr_chirho::StmtChirho;

        if let Some(index_chirho) = quals_chirho.iter().rposition(|qualifier_chirho| {
            matches!(
                qualifier_chirho,
                ComprehensionQualifierChirho::TransformChirho(_)
            )
        }) {
            return self.desugar_comprehension_transform_chirho(
                result_chirho,
                quals_chirho,
                index_chirho,
                cont_chirho,
            );
        }
        if quals_chirho.is_empty() {
            // Base case: [e] with continuation k = (:) e k
            let body_core_chirho = result_chirho(self);
            return CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![body_core_chirho, cont_chirho],
            };
        }

        let rest_chirho = &quals_chirho[1..];

        let ComprehensionQualifierChirho::StatementChirho(statement_chirho) = &quals_chirho[0]
        else {
            unreachable!("transform dispatched before ordinary qualifier lowering")
        };
        match statement_chirho.as_ref() {
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                // p <- xs with continuation k →
                //   letrec go = \$xs -> case $xs of
                //     [] -> k
                //     (:) p $rest -> [e | rest_quals] with cont (go $rest)
                //   in go xs

                let xs_core_chirho = self.desugar_expr_chirho(expr_chirho);

                let var_name_chirho = match pat_chirho {
                    PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                    _ => "$elem".to_string(),
                };

                let go_name_chirho = format!("$lc_go_{}", self.next_id_chirho);
                let go_id_chirho = {
                    let id_chirho = CoreIdChirho(self.next_id_chirho);
                    self.next_id_chirho += 1;
                    self.names_chirho.insert(id_chirho, go_name_chirho.clone());
                    id_chirho
                };

                let list_ty_chirho = TyChirho::VarChirho(
                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                );

                let xs_binder_chirho = self.fresh_binder_chirho(
                    "$xs",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let xs_id_chirho = xs_binder_chirho.id_chirho;

                let wild_chirho = self.fresh_binder_chirho(
                    "$w",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                self.push_scope_chirho();
                let elem_binder_chirho = self.fresh_binder_chirho(
                    &var_name_chirho,
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );

                let tail_binder_chirho = self.fresh_binder_chirho(
                    "$rest",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                let go_rest_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(tail_binder_chirho.id_chirho)),
                };

                // The element is matched against the generator's pattern: a
                // mismatch SKIPS it (`go $rest`), as Haskell specifies, and a
                // match binds the pattern's variables for the rest.
                // workflow: language-features-chirho/pattern-matching-chirho
                let skip_chirho = go_rest_chirho.clone();
                let elem_id_chirho = elem_binder_chirho.id_chirho;
                let cons_rhs_chirho = self.match_row_chirho(
                    pat_chirho,
                    elem_id_chirho,
                    &skip_chirho,
                    &mut |ctx_chirho| {
                        if rest_chirho.is_empty() {
                            let body_core_chirho = result_chirho(ctx_chirho);
                            CoreExprChirho::ConAppChirho {
                                con_name_chirho: ":".to_string(),
                                args_chirho: vec![body_core_chirho, go_rest_chirho.clone()],
                            }
                        } else {
                            ctx_chirho.desugar_comprehension_result_chirho(
                                rest_chirho,
                                result_chirho,
                                go_rest_chirho.clone(),
                            )
                        }
                    },
                );
                self.pop_scope_chirho();

                let case_body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_id_chirho)),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: list_ty_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: cont_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho(":".to_string()),
                            binders_chirho: vec![elem_binder_chirho, tail_binder_chirho],
                            rhs_chirho: cons_rhs_chirho,
                        },
                    ],
                };

                let go_body_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: xs_binder_chirho,
                    body_chirho: Box::new(case_body_chirho),
                };

                let go_binder_chirho = BinderChirho {
                    id_chirho: go_id_chirho,
                    name_chirho: go_name_chirho,
                    ty_chirho: list_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };

                CoreExprChirho::LetChirho {
                    rec_chirho: true,
                    binds_chirho: vec![(go_binder_chirho, go_body_chirho)],
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                        arg_chirho: Box::new(xs_core_chirho),
                    }),
                }
            }
            StmtChirho::ExprChirho {
                expr_chirho: guard_expr_chirho,
                ..
            } => {
                // Guard with continuation: if guard then [e | rest] with k else k
                let guard_core_chirho = self.desugar_expr_chirho(guard_expr_chirho);
                let then_chirho = self.desugar_comprehension_result_chirho(
                    rest_chirho,
                    result_chirho,
                    cont_chirho.clone(),
                );

                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(guard_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: cont_chirho,
                        },
                    ],
                }
            }
            // let binds in [e | rest] with continuation k
            StmtChirho::LetChirho { binds_chirho, .. } => {
                self.desugar_let_around_chirho(binds_chirho, |ctx_chirho| {
                    ctx_chirho.desugar_comprehension_result_chirho(
                        rest_chirho,
                        result_chirho,
                        cont_chirho,
                    )
                })
            }
        }
    }

    /// Build an inline list append: `xs ++ ys`.
    ///
    /// Produces:
    /// ```text
    /// letrec $append = \$as -> case $as of
    ///     [] -> ys
    ///     (:) $h $t -> (:) $h ($append $t)
    /// in $append xs
    /// ```
    #[allow(dead_code)]
    fn build_list_append_chirho(
        &mut self,
        xs_chirho: CoreExprChirho,
        ys_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let list_ty_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
            self.next_id_chirho,
        ));

        let append_name_chirho = format!("$lc_app_{}", self.next_id_chirho);
        let append_id_chirho = {
            let id_chirho = CoreIdChirho(self.next_id_chirho);
            self.next_id_chirho += 1;
            self.names_chirho
                .insert(id_chirho, append_name_chirho.clone());
            id_chirho
        };

        // Two-argument append: \$as $ys -> case $as of { ... }
        // This avoids capturing `ys` as a free variable which causes
        // issues with the STG letrec lowering inside function bodies.
        let as_binder_chirho =
            self.fresh_binder_chirho("$as", list_ty_chirho.clone(), SpanChirho::DUMMY_CHIRHO);
        let as_id_chirho = as_binder_chirho.id_chirho;

        let ys_binder_chirho =
            self.fresh_binder_chirho("$ys", list_ty_chirho.clone(), SpanChirho::DUMMY_CHIRHO);
        let ys_id_chirho = ys_binder_chirho.id_chirho;

        let wild_chirho =
            self.fresh_binder_chirho("$w", list_ty_chirho.clone(), SpanChirho::DUMMY_CHIRHO);

        let h_binder_chirho =
            self.fresh_binder_chirho("$h", list_ty_chirho.clone(), SpanChirho::DUMMY_CHIRHO);
        let t_binder_chirho =
            self.fresh_binder_chirho("$t", list_ty_chirho.clone(), SpanChirho::DUMMY_CHIRHO);

        // (:) $h (append $t $ys)
        let cons_rhs_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: ":".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(h_binder_chirho.id_chirho),
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(t_binder_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(ys_id_chirho)),
                },
            ],
        };

        let case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_id_chirho)),
            bind_chirho: wild_chirho,
            result_ty_chirho: list_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::VarChirho(ys_id_chirho),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_binder_chirho, t_binder_chirho],
                    rhs_chirho: cons_rhs_chirho,
                },
            ],
        };

        // \$as -> \$ys -> case $as of { ... }
        let append_body_chirho = CoreExprChirho::LamChirho {
            binder_chirho: as_binder_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: ys_binder_chirho,
                body_chirho: Box::new(case_chirho),
            }),
        };

        let append_binder_chirho = BinderChirho {
            id_chirho: append_id_chirho,
            name_chirho: append_name_chirho,
            ty_chirho: list_ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        // letrec append = \$as $ys -> ... in append xs ys
        CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![(append_binder_chirho, append_body_chirho)],
            body_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(xs_chirho),
                }),
                arg_chirho: Box::new(ys_chirho),
            }),
        }
    }
}
