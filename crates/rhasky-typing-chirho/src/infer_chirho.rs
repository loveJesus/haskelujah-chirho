// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type inference engine (Algorithm W)
//!
//! Implements Hindley-Milner type inference with let-generalization.
//! Walks the AST and produces typed bindings via unification.

use rhasky_ast_chirho::decl_chirho::DeclChirho;
use rhasky_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho, StmtChirho};
use rhasky_ast_chirho::lit_chirho::LitChirho;
use rhasky_ast_chirho::module_chirho::ModuleChirho;
use rhasky_ast_chirho::pat_chirho::PatChirho;
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use rhasky_span_chirho::SpanChirho;

use crate::class_chirho::{ClassEnvChirho, PredChirho};
use crate::env_chirho::TyEnvChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use crate::unify_chirho::{UnifyErrorChirho, unify_chirho};

/// Error code range for type inference diagnostics.
const TYPE_MISMATCH_CODE_CHIRHO: u16 = 200;
const OCCURS_CHECK_CODE_CHIRHO: u16 = 201;
const UNBOUND_VAR_CODE_CHIRHO: u16 = 202;
const TUPLE_ARITY_CODE_CHIRHO: u16 = 203;
const UNSATISFIED_CONSTRAINT_CODE_CHIRHO: u16 = 204;

/// Result of type inference on a module.
#[derive(Debug)]
pub struct InferResultChirho {
    /// The final substitution after inference.
    pub subst_chirho: SubstChirho,
    /// The type environment after inference (with all top-level bindings).
    pub env_chirho: TyEnvChirho,
    /// The class environment after inference.
    pub class_env_chirho: ClassEnvChirho,
    /// Diagnostics collected during inference.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// The inference context — carries mutable state during inference.
pub struct InferCtxChirho {
    /// Fresh type variable counter.
    next_var_chirho: u32,
    /// Type environment (scoped).
    env_chirho: TyEnvChirho,
    /// Class environment (typeclasses and instances).
    class_env_chirho: ClassEnvChirho,
    /// Deferred (wanted) typeclass predicates collected during inference.
    deferred_preds_chirho: Vec<(PredChirho, SpanChirho)>,
    /// Accumulated diagnostics.
    diagnostics_chirho: DiagnosticBundleChirho,
}

impl InferCtxChirho {
    pub fn new_chirho() -> Self {
        let mut env_chirho = TyEnvChirho::new_chirho();
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();
        seed_builtins_chirho(&mut env_chirho);
        Self {
            next_var_chirho: 0,
            env_chirho,
            class_env_chirho,
            deferred_preds_chirho: Vec::new(),
            diagnostics_chirho: DiagnosticBundleChirho::empty_chirho(),
        }
    }

    /// Generate a fresh unification variable.
    pub fn fresh_var_chirho(&mut self) -> TyChirho {
        let var_chirho = TyVarChirho(self.next_var_chirho);
        self.next_var_chirho += 1;
        TyChirho::VarChirho(var_chirho)
    }

    /// Instantiate a type scheme with fresh unification variables.
    /// Any predicates on the scheme are also instantiated and deferred.
    pub fn instantiate_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
    ) -> TyChirho {
        if scheme_chirho.vars_chirho.is_empty() && scheme_chirho.preds_chirho.is_empty() {
            return scheme_chirho.ty_chirho.clone();
        }

        let mut subst_chirho = SubstChirho::empty_chirho();
        for v_chirho in &scheme_chirho.vars_chirho {
            subst_chirho.insert_chirho(*v_chirho, self.fresh_var_chirho());
        }

        // Instantiate and defer predicates
        for pred_chirho in &scheme_chirho.preds_chirho {
            let instantiated_ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            self.deferred_preds_chirho.push((
                PredChirho::new_chirho(&pred_chirho.class_name_chirho, instantiated_ty_chirho),
                span_chirho,
            ));
        }

        subst_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho)
    }

    /// Generalize a type into a scheme by quantifying over all free variables
    /// not free in the environment. Also partitions deferred predicates:
    /// predicates whose type variables are all being generalized become part
    /// of the scheme; the rest remain deferred.
    ///
    /// Deferred predicates are assumed to already have substitutions applied
    /// (via `apply_subst_all_chirho` during inference).
    pub fn generalize_chirho(&mut self, ty_chirho: &TyChirho) -> SchemeChirho {
        let env_fvs_chirho = self.env_chirho.free_vars_chirho();
        let ty_fvs_chirho = ty_chirho.free_vars_chirho();
        let vars_chirho: Vec<TyVarChirho> = ty_fvs_chirho
            .into_iter()
            .filter(|v_chirho| !env_fvs_chirho.contains(v_chirho))
            .collect();

        // Partition deferred predicates: those whose type vars are all being
        // generalized go into the scheme; the rest stay deferred.
        let mut scheme_preds_chirho = Vec::new();
        let mut remaining_chirho = Vec::new();
        for (pred_chirho, span_chirho) in self.deferred_preds_chirho.drain(..) {
            let pred_fvs_chirho = pred_chirho.ty_chirho.free_vars_chirho();
            if pred_fvs_chirho
                .iter()
                .all(|v_chirho| vars_chirho.contains(v_chirho))
            {
                scheme_preds_chirho.push(SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho,
                    ty_chirho: pred_chirho.ty_chirho,
                });
            } else {
                remaining_chirho.push((pred_chirho, span_chirho));
            }
        }
        self.deferred_preds_chirho = remaining_chirho;

        // Deduplicate predicates
        scheme_preds_chirho.dedup();

        SchemeChirho {
            vars_chirho,
            preds_chirho: scheme_preds_chirho,
            ty_chirho: ty_chirho.clone(),
        }
    }

    /// Apply a substitution to the type environment and deferred predicates.
    fn apply_subst_all_chirho(&mut self, subst_chirho: &SubstChirho) {
        self.env_chirho.apply_subst_chirho(subst_chirho);
        for (pred_chirho, _span_chirho) in &mut self.deferred_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
        }
    }

    /// Record a unification error as a diagnostic.
    fn report_unify_error_chirho(&mut self, err_chirho: &UnifyErrorChirho) {
        match err_chirho {
            UnifyErrorChirho::MismatchChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
                        format!(
                            "type mismatch: expected `{expected_chirho}`, found `{actual_chirho}`"
                        ),
                        *span_chirho,
                    ),
                );
            }
            UnifyErrorChirho::OccursCheckChirho {
                var_chirho,
                ty_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(OCCURS_CHECK_CODE_CHIRHO),
                        format!(
                            "infinite type: `{var_chirho}` occurs in `{ty_chirho}`"
                        ),
                        *span_chirho,
                    ),
                );
            }
            UnifyErrorChirho::TupleArityChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TUPLE_ARITY_CODE_CHIRHO),
                        format!(
                            "tuple arity mismatch: expected {expected_chirho} elements, found {actual_chirho}"
                        ),
                        *span_chirho,
                    ),
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Expression inference
    // -----------------------------------------------------------------------

    /// Infer the type of an expression, returning (substitution, type).
    pub fn infer_expr_chirho(
        &mut self,
        expr_chirho: &ExprChirho,
    ) -> (SubstChirho, TyChirho) {
        match expr_chirho {
            ExprChirho::LitChirho(lit_chirho) => {
                let ty_chirho = infer_lit_chirho(lit_chirho);
                (SubstChirho::empty_chirho(), ty_chirho)
            }

            ExprChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self.env_chirho.lookup_chirho(text_chirho).cloned();
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        let ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, span_chirho);
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => {
                        self.diagnostics_chirho.push_chirho(
                            DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(UNBOUND_VAR_CODE_CHIRHO),
                                format!("unbound variable: `{text_chirho}`"),
                                span_chirho,
                            ),
                        );
                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                    }
                }
            }

            ExprChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self.env_chirho.lookup_chirho(text_chirho).cloned();
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        let ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, span_chirho);
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => {
                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                    }
                }
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                let (s1_chirho, fun_ty_chirho) = self.infer_expr_chirho(fun_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let (s2_chirho, arg_ty_chirho) = self.infer_expr_chirho(arg_chirho);
                let fun_ty_sub_chirho = s2_chirho.apply_ty_chirho(&fun_ty_chirho);

                let result_ty_chirho = self.fresh_var_chirho();
                let expected_fun_chirho =
                    TyChirho::fun_chirho(arg_ty_chirho, result_ty_chirho.clone());

                match unify_chirho(&fun_ty_sub_chirho, &expected_fun_chirho, *span_chirho) {
                    Ok(s3_chirho) => {
                        let combined_chirho = s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(&result_ty_chirho);
                        self.apply_subst_all_chirho(&s3_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (s2_chirho.compose_chirho(&s1_chirho), self.fresh_var_chirho())
                    }
                }
            }

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => {
                self.env_chirho.push_scope_chirho();

                let mut param_tys_chirho = Vec::new();
                for pat_chirho in pats_chirho {
                    let pat_ty_chirho = self.fresh_var_chirho();
                    self.bind_pat_chirho(pat_chirho, &pat_ty_chirho);
                    param_tys_chirho.push(pat_ty_chirho);
                }

                let (body_subst_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                self.env_chirho.pop_scope_chirho();

                let mut result_chirho = body_ty_chirho;
                for param_ty_chirho in param_tys_chirho.into_iter().rev() {
                    let param_sub_chirho = body_subst_chirho.apply_ty_chirho(&param_ty_chirho);
                    result_chirho = TyChirho::fun_chirho(param_sub_chirho, result_chirho);
                }

                (body_subst_chirho, result_chirho)
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                span_chirho,
            } => {
                let (s1_chirho, cond_ty_chirho) = self.infer_expr_chirho(cond_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                // Condition must be Bool
                match unify_chirho(&cond_ty_chirho, &TyChirho::bool_chirho(), *span_chirho) {
                    Ok(sc_chirho) => {
                        self.apply_subst_all_chirho(&sc_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }

                let (s2_chirho, then_ty_chirho) = self.infer_expr_chirho(then_chirho);
                self.apply_subst_all_chirho(&s2_chirho);

                let (s3_chirho, else_ty_chirho) = self.infer_expr_chirho(else_chirho);

                // Then and else branches must have the same type
                let then_sub_chirho = s3_chirho.apply_ty_chirho(&then_ty_chirho);
                match unify_chirho(&then_sub_chirho, &else_ty_chirho, *span_chirho) {
                    Ok(s4_chirho) => {
                        let combined_chirho =
                            s4_chirho.compose_chirho(&s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho)));
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(&else_ty_chirho);
                        self.apply_subst_all_chirho(&s4_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        let combined_chirho = s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                        (combined_chirho, else_ty_chirho)
                    }
                }
            }

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                self.env_chirho.push_scope_chirho();
                let mut subst_chirho = SubstChirho::empty_chirho();

                for bind_chirho in binds_chirho {
                    match bind_chirho {
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            name_chirho,
                            matches_chirho,
                            span_chirho,
                        } => {
                            let (s_chirho, ty_chirho) =
                                self.infer_matches_chirho(matches_chirho, *span_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);

                            let gen_ty_chirho = self.generalize_chirho(&ty_chirho);
                            self.env_chirho
                                .bind_chirho(name_chirho.text_chirho().to_string(), gen_ty_chirho);
                        }
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                            pat_chirho,
                            rhs_chirho,
                            ..
                        } => {
                            let (s_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(rhs_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);
                            self.bind_pat_chirho(pat_chirho, &rhs_ty_chirho);
                        }
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho { .. } => {
                            // Type signatures are collected but not enforced yet
                        }
                    }
                }

                let (body_s_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                self.env_chirho.pop_scope_chirho();

                let combined_chirho = body_s_chirho.compose_chirho(&subst_chirho);
                (combined_chirho, body_ty_chirho)
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut elem_tys_chirho = Vec::new();

                for elem_chirho in elements_chirho {
                    let (s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);
                    elem_tys_chirho.push(ty_chirho);
                }

                // Apply accumulated substitution to element types
                let final_elems_chirho: Vec<TyChirho> = elem_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();

                (subst_chirho, TyChirho::TupleChirho(final_elems_chirho))
            }

            ExprChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => {
                let elem_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = SubstChirho::empty_chirho();

                for elem_chirho in elements_chirho {
                    let (s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);

                    let elem_sub_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    match unify_chirho(&elem_sub_chirho, &ty_chirho, *span_chirho) {
                        Ok(su_chirho) => {
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&su_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                let final_elem_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                (subst_chirho, TyChirho::ListChirho(Box::new(final_elem_chirho)))
            }

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                span_chirho,
            } => {
                let (s1_chirho, scrut_ty_chirho) = self.infer_expr_chirho(scrutinee_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let result_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = s1_chirho;

                for alt_chirho in alts_chirho {
                    self.env_chirho.push_scope_chirho();

                    let pat_ty_chirho = self.fresh_var_chirho();
                    self.bind_pat_chirho(&alt_chirho.pat_chirho, &pat_ty_chirho);

                    // Unify pattern type with scrutinee
                    let scrut_sub_chirho = subst_chirho.apply_ty_chirho(&scrut_ty_chirho);
                    match unify_chirho(&pat_ty_chirho, &scrut_sub_chirho, *span_chirho) {
                        Ok(sp_chirho) => {
                            subst_chirho = sp_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&sp_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    let (sr_chirho, alt_ty_chirho) = self.infer_rhs_chirho(&alt_chirho.rhs_chirho);
                    subst_chirho = sr_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&sr_chirho);

                    // Unify alt result with overall result type
                    let result_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                    match unify_chirho(&result_sub_chirho, &alt_ty_chirho, *span_chirho) {
                        Ok(sa_chirho) => {
                            subst_chirho = sa_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&sa_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    self.env_chirho.pop_scope_chirho();
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }

            ExprChirho::DoChirho {
                stmts_chirho,
                span_chirho,
            } => {
                // Simplified: infer each statement, return the type of the last
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut last_ty_chirho = TyChirho::unit_chirho();

                for stmt_chirho in stmts_chirho {
                    match stmt_chirho {
                        StmtChirho::ExprChirho(expr_chirho) => {
                            let (s_chirho, ty_chirho) = self.infer_expr_chirho(expr_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);
                            last_ty_chirho = ty_chirho;
                        }
                        StmtChirho::BindChirho {
                            pat_chirho,
                            expr_chirho,
                            ..
                        } => {
                            let (s_chirho, ty_chirho) = self.infer_expr_chirho(expr_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);
                            self.bind_pat_chirho(pat_chirho, &ty_chirho);
                            last_ty_chirho = TyChirho::unit_chirho();
                        }
                        StmtChirho::LetChirho { .. } => {
                            // Let in do: similar to LetExpr, skip for now
                            last_ty_chirho = TyChirho::unit_chirho();
                        }
                    }
                }

                let _ = span_chirho; // suppress unused warning
                (subst_chirho, last_ty_chirho)
            }

            ExprChirho::NegChirho { expr_chirho, .. } => {
                let (s_chirho, _ty_chirho) = self.infer_expr_chirho(expr_chirho);
                // Negation produces a Num type — simplified to Int
                (s_chirho, TyChirho::int_chirho())
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => self.infer_expr_chirho(inner_chirho),

            ExprChirho::AnnChirho { expr_chirho, .. } => {
                // Type annotation — infer the expression, skip annotation check for now
                self.infer_expr_chirho(expr_chirho)
            }

            // For remaining expression forms, return a fresh variable
            _ => (SubstChirho::empty_chirho(), self.fresh_var_chirho()),
        }
    }

    // -----------------------------------------------------------------------
    // Pattern binding
    // -----------------------------------------------------------------------

    /// Bind pattern variables to the given type in the current scope.
    fn bind_pat_chirho(&mut self, pat_chirho: &PatChirho, ty_chirho: &TyChirho) {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(ty_chirho.clone()),
                );
            }
            PatChirho::WildcardChirho { .. } => {
                // Wildcards don't bind anything
            }
            PatChirho::LitChirho { .. } => {
                // Literal patterns don't introduce bindings
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho: inner_chirho,
                ..
            } => {
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(ty_chirho.clone()),
                );
                self.bind_pat_chirho(inner_chirho, ty_chirho);
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                // Each element gets a fresh type variable
                let elem_tys_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|_| self.fresh_var_chirho())
                    .collect();
                for (elem_pat_chirho, elem_ty_chirho) in
                    elements_chirho.iter().zip(elem_tys_chirho.iter())
                {
                    self.bind_pat_chirho(elem_pat_chirho, elem_ty_chirho);
                }
            }
            PatChirho::ConChirho {
                args_chirho, ..
            } => {
                for arg_chirho in args_chirho {
                    let fresh_chirho = self.fresh_var_chirho();
                    self.bind_pat_chirho(arg_chirho, &fresh_chirho);
                }
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.bind_pat_chirho(inner_chirho, ty_chirho);
            }
            PatChirho::LazyChirho { inner_chirho, .. } | PatChirho::BangChirho { inner_chirho, .. } => {
                self.bind_pat_chirho(inner_chirho, ty_chirho);
            }
            _ => {
                // Other patterns: no bindings extracted yet
            }
        }
    }

    // -----------------------------------------------------------------------
    // RHS and match arm inference
    // -----------------------------------------------------------------------

    /// Infer the type of a right-hand side.
    fn infer_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) -> (SubstChirho, TyChirho) {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.infer_expr_chirho(expr_chirho),
            RhsChirho::GuardedChirho(guarded_chirho) => {
                // Infer the first guard's body as the type
                if let Some(first_chirho) = guarded_chirho.first() {
                    self.infer_expr_chirho(&first_chirho.body_chirho)
                } else {
                    (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                }
            }
        }
    }

    /// Infer the type of a set of match arms (function equations).
    fn infer_matches_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let first_chirho = &matches_chirho[0];
        let arity_chirho = first_chirho.pats_chirho.len();

        // Create fresh vars for parameters and result
        let param_tys_chirho: Vec<TyChirho> =
            (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect();
        let result_ty_chirho = self.fresh_var_chirho();
        let mut subst_chirho = SubstChirho::empty_chirho();

        for match_arm_chirho in matches_chirho {
            self.env_chirho.push_scope_chirho();

            // Bind pattern variables
            for (pat_chirho, pat_ty_chirho) in match_arm_chirho
                .pats_chirho
                .iter()
                .zip(param_tys_chirho.iter())
            {
                let pat_ty_sub_chirho = subst_chirho.apply_ty_chirho(pat_ty_chirho);
                self.bind_pat_chirho(pat_chirho, &pat_ty_sub_chirho);
            }

            // Infer RHS
            let (sr_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(&match_arm_chirho.rhs_chirho);
            subst_chirho = sr_chirho.compose_chirho(&subst_chirho);
            self.apply_subst_all_chirho(&sr_chirho);

            // Unify with result type
            let result_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
            match unify_chirho(&result_sub_chirho, &rhs_ty_chirho, span_chirho) {
                Ok(su_chirho) => {
                    subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&su_chirho);
                }
                Err(err_chirho) => {
                    self.report_unify_error_chirho(&err_chirho);
                }
            }

            self.env_chirho.pop_scope_chirho();
        }

        // Build the function type: param1 -> param2 -> ... -> result
        let final_params_chirho: Vec<TyChirho> = param_tys_chirho
            .iter()
            .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
            .collect();
        let final_result_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
        let fun_ty_chirho = TyChirho::fun_n_chirho(final_params_chirho, final_result_chirho);

        (subst_chirho, fun_ty_chirho)
    }

    // -----------------------------------------------------------------------
    // Module-level inference
    // -----------------------------------------------------------------------

    /// Run type inference on a module. Returns the result with environment,
    /// substitution, and diagnostics.
    pub fn infer_module_chirho(&mut self, module_chirho: &ModuleChirho) -> SubstChirho {
        let mut subst_chirho = SubstChirho::empty_chirho();

        // Phase 1: Bind data constructor types (simplified — constructors get fresh vars)
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } = decl_chirho
            {
                let result_ty_chirho = TyChirho::ConChirho(name_chirho.text_chirho().to_string());
                for con_chirho in constructors_chirho {
                    let con_name_chirho = match con_chirho {
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            // Constructor type: field1 -> field2 -> ... -> ResultType
                            let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                .iter()
                                .map(|_| self.fresh_var_chirho())
                                .collect();
                            let con_ty_chirho =
                                TyChirho::fun_n_chirho(field_tys_chirho, result_ty_chirho.clone());
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                            continue;
                        }
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho,
                            ..
                        } => name_chirho,
                    };
                    // Record constructor: simplified to just the result type
                    self.env_chirho.bind_chirho(
                        con_name_chirho.text_chirho().to_string(),
                        SchemeChirho::mono_chirho(result_ty_chirho.clone()),
                    );
                }
            }
        }

        // Phase 2: Infer function bindings
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                span_chirho,
            } = decl_chirho
            {
                // Pre-bind with a fresh variable for recursive calls
                let pre_ty_chirho = self.fresh_var_chirho();
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(pre_ty_chirho.clone()),
                );

                let (s_chirho, inferred_ty_chirho) =
                    self.infer_matches_chirho(matches_chirho, *span_chirho);
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&s_chirho);

                // Unify pre-bound type with inferred type
                let pre_sub_chirho = subst_chirho.apply_ty_chirho(&pre_ty_chirho);
                match unify_chirho(&pre_sub_chirho, &inferred_ty_chirho, *span_chirho) {
                    Ok(su_chirho) => {
                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }

                // Remove the mono pre-binding before generalizing so its
                // free variables don't prevent generalization of the inferred type.
                let binding_name_chirho = name_chirho.text_chirho().to_string();
                self.env_chirho.remove_chirho(&binding_name_chirho);

                // Generalize and rebind
                let final_ty_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);
                let gen_chirho = self.generalize_chirho(&final_ty_chirho);
                self.env_chirho
                    .bind_chirho(binding_name_chirho, gen_chirho);
            }
        }

        subst_chirho
    }

    /// Check remaining deferred predicates. Those that the class environment
    /// can fully entail are discharged; those that can't produce diagnostics.
    fn check_deferred_preds_chirho(&mut self, final_subst_chirho: &SubstChirho) {
        let preds_chirho: Vec<(PredChirho, SpanChirho)> =
            self.deferred_preds_chirho.drain(..).collect();

        for (pred_chirho, span_chirho) in preds_chirho {
            // Apply the final substitution to the predicate type
            let resolved_ty_chirho =
                final_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            let resolved_pred_chirho =
                PredChirho::new_chirho(&pred_chirho.class_name_chirho, resolved_ty_chirho);

            // If the type is still a variable, we can't check it yet — it's
            // polymorphic and will be checked at use sites
            if matches!(resolved_pred_chirho.ty_chirho, TyChirho::VarChirho(_)) {
                continue;
            }

            if !self.class_env_chirho.entails_chirho(&resolved_pred_chirho) {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!(
                            "no instance for `{}`",
                            resolved_pred_chirho
                        ),
                        span_chirho,
                    ),
                );
            }
        }
    }

    /// Consume the context and return the final result.
    pub fn finish_chirho(self) -> InferResultChirho {
        InferResultChirho {
            subst_chirho: SubstChirho::empty_chirho(),
            env_chirho: self.env_chirho,
            class_env_chirho: self.class_env_chirho,
            diagnostics_chirho: self.diagnostics_chirho,
        }
    }
}

// ---------------------------------------------------------------------------
// Built-in type environment
// ---------------------------------------------------------------------------

/// Seed the type environment with Haskell Prelude types.
fn seed_builtins_chirho(env_chirho: &mut TyEnvChirho) {
    // True :: Bool, False :: Bool
    env_chirho.bind_chirho(
        "True".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    env_chirho.bind_chirho(
        "False".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );

    // not :: Bool -> Bool
    env_chirho.bind_chirho(
        "not".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::bool_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // Numeric operators: forall a. Num a => a -> a -> a
    let num_v_chirho = TyVarChirho(1100);
    let num_binop_chirho = SchemeChirho {
        vars_chirho: vec![num_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Num".to_string(),
            ty_chirho: TyChirho::VarChirho(num_v_chirho),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(num_v_chirho),
                TyChirho::VarChirho(num_v_chirho),
            ],
            TyChirho::VarChirho(num_v_chirho),
        ),
    };
    env_chirho.bind_chirho("+".to_string(), num_binop_chirho.clone());
    env_chirho.bind_chirho("-".to_string(), num_binop_chirho.clone());
    env_chirho.bind_chirho("*".to_string(), num_binop_chirho);

    // Equality operators: forall a. Eq a => a -> a -> Bool
    let eq_v_chirho = TyVarChirho(1200);
    let eq_cmp_chirho = SchemeChirho {
        vars_chirho: vec![eq_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Eq".to_string(),
            ty_chirho: TyChirho::VarChirho(eq_v_chirho),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(eq_v_chirho),
                TyChirho::VarChirho(eq_v_chirho),
            ],
            TyChirho::bool_chirho(),
        ),
    };
    env_chirho.bind_chirho("==".to_string(), eq_cmp_chirho.clone());
    env_chirho.bind_chirho("/=".to_string(), eq_cmp_chirho);

    // Ordering operators: forall a. Ord a => a -> a -> Bool
    let ord_v_chirho = TyVarChirho(1300);
    let ord_cmp_chirho = SchemeChirho {
        vars_chirho: vec![ord_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Ord".to_string(),
            ty_chirho: TyChirho::VarChirho(ord_v_chirho),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(ord_v_chirho),
                TyChirho::VarChirho(ord_v_chirho),
            ],
            TyChirho::bool_chirho(),
        ),
    };
    env_chirho.bind_chirho(">".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho("<".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho(">=".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho("<=".to_string(), ord_cmp_chirho);

    // show :: forall a. Show a => a -> String
    let show_v_chirho = TyVarChirho(1400);
    env_chirho.bind_chirho(
        "show".to_string(),
        SchemeChirho {
            vars_chirho: vec![show_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(show_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(show_v_chirho),
                TyChirho::string_chirho(),
            ),
        },
    );

    // pure :: forall a. a -> a (simplified — no Applicative class yet)
    let pure_v_chirho = TyVarChirho(1500);
    env_chirho.bind_chirho(
        "pure".to_string(),
        SchemeChirho {
            vars_chirho: vec![pure_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(pure_v_chirho),
                TyChirho::VarChirho(pure_v_chirho),
            ),
        },
    );

    // id :: forall a. a -> a
    let id_var_chirho = TyVarChirho(1000);
    env_chirho.bind_chirho(
        "id".to_string(),
        SchemeChirho {
            vars_chirho: vec![id_var_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(id_var_chirho),
                TyChirho::VarChirho(id_var_chirho),
            ),
        },
    );

    // const :: forall a b. a -> b -> a
    let const_a_chirho = TyVarChirho(1001);
    let const_b_chirho = TyVarChirho(1002);
    env_chirho.bind_chirho(
        "const".to_string(),
        SchemeChirho {
            vars_chirho: vec![const_a_chirho, const_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(const_a_chirho),
                    TyChirho::VarChirho(const_b_chirho),
                ],
                TyChirho::VarChirho(const_a_chirho),
            ),
        },
    );
}

// ---------------------------------------------------------------------------
// Literal type inference
// ---------------------------------------------------------------------------

fn infer_lit_chirho(lit_chirho: &LitChirho) -> TyChirho {
    match lit_chirho {
        LitChirho::IntChirho(..) => TyChirho::int_chirho(),
        LitChirho::FloatChirho(..) => TyChirho::double_chirho(),
        LitChirho::CharChirho(..) => TyChirho::char_chirho(),
        LitChirho::StringChirho(..) => TyChirho::string_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Run type inference on a module. This is the main entry point.
pub fn infer_module_chirho(module_chirho: &ModuleChirho) -> InferResultChirho {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let subst_chirho = ctx_chirho.infer_module_chirho(module_chirho);
    ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
    let mut result_chirho = ctx_chirho.finish_chirho();
    result_chirho.subst_chirho = subst_chirho;
    result_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
    use rhasky_ast_chirho::module_chirho::ModuleChirho;
    use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn infer_literal_int_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO));
        let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert_eq!(ty_chirho, TyChirho::int_chirho());
    }

    #[test]
    fn infer_literal_string_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LitChirho(LitChirho::StringChirho("hello".to_string(), SpanChirho::DUMMY_CHIRHO));
        let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert_eq!(ty_chirho, TyChirho::string_chirho());
    }

    #[test]
    fn infer_identity_function_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // \x -> x
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);

        // Should be `a -> a` for some variable a
        match &final_ty_chirho {
            TyChirho::FunChirho(arg_chirho, result_chirho) => {
                assert_eq!(arg_chirho, result_chirho, "identity should have a -> a");
            }
            other_chirho => panic!("expected function type, got {other_chirho}"),
        }
    }

    #[test]
    fn infer_application_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // not True
        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("not"))),
            arg_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(final_ty_chirho, TyChirho::bool_chirho());
    }

    #[test]
    fn infer_if_expression_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // if True then 1 else 2
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(final_ty_chirho, TyChirho::int_chirho());
    }

    #[test]
    fn infer_type_mismatch_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // if 42 then 1 else 2  (condition is Int, not Bool)
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let _ = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let result_chirho = ctx_chirho.finish_chirho();
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "should report type mismatch for Int condition"
        );
    }

    #[test]
    fn infer_module_data_and_function_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Red"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
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
                },
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "module inference should succeed without errors"
        );

        // f should have type a -> a (identity)
        let f_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("f")
            .expect("f should be in environment");
        match &f_scheme_chirho.ty_chirho {
            TyChirho::FunChirho(arg_chirho, result_chirho) => {
                assert_eq!(
                    arg_chirho, result_chirho,
                    "f should have type a -> a"
                );
            }
            other_chirho => panic!("expected f to have function type, got {other_chirho}"),
        }

        // Red should be in environment as Color
        let red_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("Red")
            .expect("Red constructor should be in environment");
        assert_eq!(
            red_scheme_chirho.ty_chirho,
            TyChirho::ConChirho("Color".to_string())
        );
    }

    #[test]
    fn infer_let_generalization_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // let id = \x -> x in id 42
        let expr_chirho = ExprChirho::LetChirho {
            binds_chirho: vec![
                rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("myId"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            body_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("myId"))),
                arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(
            final_ty_chirho,
            TyChirho::int_chirho(),
            "let id = \\x -> x in id 42 should have type Int"
        );
    }

    #[test]
    fn infer_tuple_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // (1, True)
        let expr_chirho = ExprChirho::TupleChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::ConChirho(dummy_name_chirho("True")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(
            final_ty_chirho,
            TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()])
        );
    }

    #[test]
    fn infer_list_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // [1, 2, 3]
        let expr_chirho = ExprChirho::ListChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::LitChirho(LitChirho::IntChirho(3, SpanChirho::DUMMY_CHIRHO)),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(
            final_ty_chirho,
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho()))
        );
    }

    // -----------------------------------------------------------------------
    // Typeclass constraint tests
    // -----------------------------------------------------------------------

    #[test]
    fn overloaded_op_defers_predicate_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("x")),
                PatChirho::VarChirho(dummy_name_chirho("y")),
            ],
            body_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (_subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert!(!ctx_chirho.deferred_preds_chirho.is_empty());
        assert_eq!(ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho, "Num");
    }

    #[test]
    fn plus_on_ints_satisfies_num_chirho() {
        // f x y = x + y  applied to Int arguments should succeed
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("PlusInts"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("addChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "addChirho should infer without errors (Num constraint stays polymorphic): {:?}",
            result_chirho.diagnostics_chirho
        );

        // addChirho should have a Num constraint in its scheme
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("addChirho")
            .expect("addChirho should be in environment");
        assert!(
            !scheme_chirho.preds_chirho.is_empty(),
            "addChirho should have Num predicate, got: {scheme_chirho}"
        );
        assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Num");
    }

    #[test]
    fn eq_on_ints_satisfies_constraint_chirho() {
        // eqChirho x y = x == y
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("EqInts"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("eqChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("=="))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "eqChirho should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("eqChirho")
            .expect("eqChirho should be in environment");
        assert!(
            !scheme_chirho.preds_chirho.is_empty(),
            "eqChirho should have Eq predicate, got: {scheme_chirho}"
        );
        assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Eq");
    }

    #[test]
    fn class_env_is_seeded_chirho() {
        let result_chirho = infer_module_chirho(&ModuleChirho {
            name_chirho: dummy_name_chirho("Empty"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });

        assert!(result_chirho.class_env_chirho.has_class_chirho("Eq"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Ord"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Show"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Num"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Functor"));
    }
}
