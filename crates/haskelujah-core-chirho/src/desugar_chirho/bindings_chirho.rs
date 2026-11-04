// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Right-hand sides and local bindings, moved whole from the desugar root (row 691, D4):
//! function equations as match rows, guarded and `where`-scoped right-hand sides, and the
//! `let` around a body for a local binding group, recursive or not.

use super::*;

impl DesugarCtxChirho {
    /// Desugar a set of match arms into a Core expression.
    /// For single-equation functions: `\p1 p2 -> rhs`
    /// For multi-equation: case expressions.
    /// A function's equations: each is a row over the parameters, tried top to
    /// bottom, falling through on a failed pattern or a failed last guard.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn desugar_matches_chirho(
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
            // Simple value binding: no patterns — still process where-binds
            return self.desugar_arm_rhs_chirho(first_chirho);
        }

        // One parameter per column, named after the first equation's variable
        // where it has one; each row binds its own variables to them.
        let param_binders_chirho: Vec<BinderChirho> = (0..arity_chirho)
            .map(|i_chirho| {
                let name_chirho =
                    extract_var_name_from_pat_chirho(&first_chirho.pats_chirho[i_chirho])
                        .unwrap_or_else(|| format!("_arg{i_chirho}"));
                self.row_binder_chirho(&name_chirho)
            })
            .collect();
        let param_ids_chirho: Vec<CoreIdChirho> = param_binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.id_chirho)
            .collect();
        let rows_chirho: Vec<MatchRowChirho<'_>> = matches_chirho
            .iter()
            .map(|arm_chirho| MatchRowChirho {
                pats_chirho: arm_chirho.pats_chirho.iter().collect(),
                body_chirho: RowBodyChirho::Rhs(&arm_chirho.rhs_chirho),
                where_binds_chirho: &arm_chirho.where_binds_chirho,
            })
            .collect();
        let failure_chirho = self.case_failure_chirho("Non-exhaustive patterns in function");
        let mut result_chirho =
            self.compile_rows_chirho(&param_ids_chirho, &rows_chirho, failure_chirho);
        for binder_chirho in param_binders_chirho.into_iter().rev() {
            result_chirho = CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: Box::new(result_chirho),
            };
        }
        result_chirho
    }

    /// Desugar the RHS of a match arm, wrapping it in a let if where-binds
    /// are present.  Where-clause names are bound *before* the body is
    /// desugared so that references in the body resolve to the Let binders.
    fn desugar_arm_rhs_chirho(&mut self, arm_chirho: &MatchArmChirho) -> CoreExprChirho {
        self.desugar_rhs_in_where_chirho(
            &arm_chirho.rhs_chirho,
            &arm_chirho.where_binds_chirho,
            None,
        )
    }

    /// A right-hand side under its `where` bindings. `fallthrough_chirho` is
    /// where a failing last guard goes: the next row of a match, or, when
    /// `None`, the "Non-exhaustive guards" failure.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn desugar_rhs_in_where_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        where_binds_chirho: &[LocalBindChirho],
        fallthrough_chirho: Option<&CoreExprChirho>,
    ) -> CoreExprChirho {
        if where_binds_chirho.is_empty() {
            return self.desugar_rhs_or_fall_chirho(rhs_chirho, fallthrough_chirho);
        }

        self.push_scope_chirho();

        // Separate fun-binds from complex pat-binds (same as let handling)
        let mut fun_bind_indices_chirho: Vec<usize> = Vec::new();
        let mut pat_bind_indices_chirho: Vec<usize> = Vec::new();
        for (idx_chirho, bind_chirho) in where_binds_chirho.iter().enumerate() {
            match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho { .. } => {
                    fun_bind_indices_chirho.push(idx_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    ..
                } => {
                    if matches!(pat_chirho, PatChirho::VarChirho(_)) {
                        fun_bind_indices_chirho.push(idx_chirho);
                    } else {
                        pat_bind_indices_chirho.push(idx_chirho);
                    }
                }
                _ => {}
            }
        }

        // Pre-bind every variable of every pattern binding, so the body and
        // the function bindings resolve them.
        // workflow: language-features-chirho/pattern-matching-chirho
        let prebound_chirho =
            self.prebind_pattern_bindings_chirho(where_binds_chirho, &pat_bind_indices_chirho);

        // Pre-bind fun-bind names
        let pre_binders_chirho =
            self.prebind_where_names_chirho_filtered(where_binds_chirho, &fun_bind_indices_chirho);

        // Desugar body *after* where names are in scope.
        let body_core_chirho = self.desugar_rhs_or_fall_chirho(rhs_chirho, fallthrough_chirho);

        // Desugar fun-bind RHSes
        let mut core_binds_chirho =
            self.desugar_where_rhs_chirho(where_binds_chirho, pre_binders_chirho);

        // A where pattern binding is LAZY, like every Haskell pattern binding:
        // its variables are bound through one shared match of the WHOLE
        // pattern, performed when any of them is demanded. A banged one forces
        // that match before the body.
        let witnesses_chirho = self.desugar_pattern_bindings_chirho(
            where_binds_chirho,
            &pat_bind_indices_chirho,
            &prebound_chirho,
            &mut core_binds_chirho,
        );
        let result_body_chirho = self.force_witnesses_chirho(&witnesses_chirho, body_core_chirho);

        // Wrap in let if there are fun-binds
        let result_chirho = if core_binds_chirho.is_empty() {
            result_body_chirho
        } else {
            let rec_chirho = Self::is_recursive_binds_chirho(&core_binds_chirho);
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho: core_binds_chirho,
                body_chirho: Box::new(result_body_chirho),
            }
        };
        self.pop_scope_chirho();
        result_chirho
    }

    /// Pre-bind where-clause names for specific indices (fun-binds only).
    fn prebind_where_names_chirho_filtered(
        &mut self,
        where_binds_chirho: &[haskelujah_ast_chirho::expr_chirho::LocalBindChirho],
        indices_chirho: &[usize],
    ) -> Vec<(usize, BinderChirho)> {
        let mut pre_binders_chirho: Vec<(usize, BinderChirho)> = Vec::new();
        for &idx_chirho in indices_chirho {
            let bind_chirho = &where_binds_chirho[idx_chirho];
            match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho,
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(name_chirho.text_chirho(), binder_chirho.id_chirho);
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                // Simple var PatBindChirho treated as fun bind
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho: PatChirho::VarChirho(name_chirho),
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(name_chirho.text_chirho(), binder_chirho.id_chirho);
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                _ => {}
            }
        }
        pre_binders_chirho
    }

    /// Desugar all where-clause RHSes (names must already be in scope via
    /// `prebind_where_names_chirho`).
    fn desugar_where_rhs_chirho(
        &mut self,
        where_binds_chirho: &[haskelujah_ast_chirho::expr_chirho::LocalBindChirho],
        pre_binders_chirho: Vec<(usize, BinderChirho)>,
    ) -> Vec<(BinderChirho, CoreExprChirho)> {
        let mut core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
        for (idx_chirho, binder_chirho) in pre_binders_chirho {
            let rhs_chirho = match &where_binds_chirho[idx_chirho] {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    matches_chirho,
                    span_chirho,
                    ..
                } => self.desugar_matches_chirho(matches_chirho, *span_chirho),
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    rhs_chirho,
                    ..
                } => self.desugar_rhs_chirho(rhs_chirho),
                _ => continue,
            };
            core_binds_chirho.push((binder_chirho, rhs_chirho));
        }
        core_binds_chirho
    }

    /// `let binds in <body>`, the body built by `body_chirho`, once, after every
    /// binding is in scope. EVERY local `let` comes through here: a `let`
    /// expression, a `let` qualifier of a guard, a `let` statement of a do block
    /// and a `let` qualifier of a list comprehension. So every one of them binds
    /// its names before any right-hand side is desugared (a local function can
    /// recurse) and binds a pattern lazily through one match of the whole
    /// pattern. The bindings' right-hand sides come first, as in the source.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn desugar_let_around_chirho(
        &mut self,
        binds_chirho: &[LocalBindChirho],
        body_chirho: impl FnOnce(&mut Self) -> CoreExprChirho,
    ) -> CoreExprChirho {
        self.push_scope_chirho();

        // Separate function bindings (which become Core let) from
        // pattern bindings (which become case expressions wrapping
        // the body).
        let mut fun_bind_indices_chirho: Vec<usize> = Vec::new();
        let mut pat_bind_indices_chirho: Vec<usize> = Vec::new();
        for (idx_chirho, bind_chirho) in binds_chirho.iter().enumerate() {
            match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho { .. } => {
                    fun_bind_indices_chirho.push(idx_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    ..
                } => {
                    // Simple variable patterns can be handled as
                    // regular let bindings (equivalent to 0-arg
                    // function binding).
                    if matches!(pat_chirho, PatChirho::VarChirho(_)) {
                        fun_bind_indices_chirho.push(idx_chirho);
                    } else {
                        pat_bind_indices_chirho.push(idx_chirho);
                    }
                }
                _ => {}
            }
        }

        // Pre-bind every variable of every pattern binding, so the body
        // and the other bindings resolve them.
        // workflow: language-features-chirho/pattern-matching-chirho
        let prebound_chirho =
            self.prebind_pattern_bindings_chirho(binds_chirho, &pat_bind_indices_chirho);

        // Pass 1: Pre-bind function binding names.
        let mut pre_binders_chirho: Vec<(usize, BinderChirho)> = Vec::new();
        for &idx_chirho in &fun_bind_indices_chirho {
            match &binds_chirho[idx_chirho] {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho,
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(name_chirho.text_chirho(), binder_chirho.id_chirho);
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                // Simple var PatBindChirho treated as fun bind
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho: PatChirho::VarChirho(name_chirho),
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(name_chirho.text_chirho(), binder_chirho.id_chirho);
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                _ => {}
            }
        }

        // Pass 2: Desugar function-binding RHSes.
        let mut core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
        for (idx_chirho, binder_chirho) in pre_binders_chirho {
            let rhs_chirho = match &binds_chirho[idx_chirho] {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    matches_chirho,
                    span_chirho,
                    ..
                } => self.desugar_matches_chirho(matches_chirho, *span_chirho),
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    rhs_chirho,
                    ..
                } => self.desugar_rhs_chirho(rhs_chirho),
                _ => continue,
            };
            core_binds_chirho.push((binder_chirho, rhs_chirho));
        }

        // `let p = rhs in body` is LAZY: each variable of `p` is bound
        // through one shared match of the WHOLE pattern, performed when
        // any of them is demanded. A banged binding forces that match
        // before the body.
        let body_core_chirho = body_chirho(self);
        let witnesses_chirho = self.desugar_pattern_bindings_chirho(
            binds_chirho,
            &pat_bind_indices_chirho,
            &prebound_chirho,
            &mut core_binds_chirho,
        );
        let result_body_chirho = self.force_witnesses_chirho(&witnesses_chirho, body_core_chirho);

        let result_chirho = if core_binds_chirho.is_empty() {
            result_body_chirho
        } else {
            let rec_chirho = Self::is_recursive_binds_chirho(&core_binds_chirho);
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho: core_binds_chirho,
                body_chirho: Box::new(result_body_chirho),
            }
        };
        self.pop_scope_chirho();
        result_chirho
    }

    /// Desugar a right-hand side.
    pub(super) fn desugar_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) -> CoreExprChirho {
        self.desugar_rhs_or_fall_chirho(rhs_chirho, None)
    }

    /// A right-hand side whose guards, when all fail, continue with
    /// `fallthrough_chirho` (the next row) instead of failing outright.
    fn desugar_rhs_or_fall_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        fallthrough_chirho: Option<&CoreExprChirho>,
    ) -> CoreExprChirho {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.desugar_expr_chirho(expr_chirho),
            // Arms in order, each its qualifiers then its body; when every arm
            // fails, the fall-through (`desugar_chirho/patterns_chirho/guards_chirho.rs`).
            RhsChirho::GuardedChirho(guards_chirho) => {
                self.desugar_guards_chirho(guards_chirho, fallthrough_chirho)
            }
        }
    }
}
