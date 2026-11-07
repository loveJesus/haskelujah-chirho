// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Do blocks, typed as the expression their selected operations build (row 710).
//!
//! Lowering selects, once, the operation each statement stands for, and Core desugars a block
//! into that operation's application (Report section 3.14; GHC's QualifiedDo and RebindableSyntax
//! translations). The checker types that very expression:
//!
//! ```text
//! x <- e; s     bind e (\x -> s); a failable x: bind e (\v -> case v of x -> s; _ -> fail "...")
//! e; s          then e s
//! let ds; s     let ds in s
//! e             e
//! ```
//!
//! Each selected operation is an ordinary reference. The one lookup and instantiation the
//! reference path makes gives its type, defers its predicates as ordinary wanteds and captures its
//! occurrence record under the selection's own origin. A Monad method, a plain local function,
//! `($)`, an indexed or graded bind and a rank-N bind (GHC T21206) each type by their own schemes
//! through the application checker, with no block-wide monad. Each statement is cloned once.
//!
//! A block in which some statement carries NO selection (a hand-built AST, Template Haskell
//! conversion, the RecursiveDo knot), or whose last statement is not an expression, keeps the
//! typing it had before row 710: shape-only, with no wanted, no evidence, no proof and no spelling
//! lookup. Changing that policy waits for root's word.
//! workflow: language-features-chirho/qualified-do-chirho
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::expr_chirho::{AltChirho, ExprChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::stmt_operation_chirho::SelectedOperationChirho;
use haskelujah_span_chirho::SpanChirho;

use super::InferCtxChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;

/// The message a failed do pattern hands the selected `fail`, as Core hands it.
const PATTERN_FAILURE_CHIRHO: &str = "Pattern match failure in do expression";
/// The binder of a failable bind's continuation; `$` keeps it out of every source namespace.
const SCRUTINEE_CHIRHO: &str = "$do_scrutinee_chirho";

impl InferCtxChirho {
    /// A do block's type: its translation's, or the shape-only typing for a block the translation
    /// does not cover. Leading `let` statements are typed in the block's scope, as before, so a
    /// rigid expected type handed down by checking mode (`do_last_expected_chirho`) still checks the
    /// translated rest.
    /// workflow: language-features-chirho/qualified-do-chirho
    pub(super) fn infer_do_block_chirho(
        &mut self,
        stmts_chirho: &[StmtChirho],
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        let expected_chirho = self.do_last_expected_chirho.take();
        let leading_lets_chirho = stmts_chirho
            .iter()
            .take_while(|stmt_chirho| matches!(stmt_chirho, StmtChirho::LetChirho { .. }))
            .count();
        let Some(translated_chirho) = translate_do_chirho(&stmts_chirho[leading_lets_chirho..])
        else {
            return self.infer_shape_only_do_chirho(stmts_chirho, expected_chirho, span_chirho);
        };
        let type_pattern_scope_mark_chirho = self.type_pattern_alias_undo_chirho.len();
        self.env_chirho.push_scope_chirho();
        let mut subst_chirho = SubstChirho::empty_chirho();
        for stmt_chirho in &stmts_chirho[..leading_lets_chirho] {
            if let StmtChirho::LetChirho { binds_chirho, .. } = stmt_chirho {
                self.infer_local_binds_chirho(binds_chirho, &mut subst_chirho);
            }
        }
        let (rest_subst_chirho, ty_chirho) = match expected_chirho {
            Some(expected_chirho) => self.infer_expr_against_expected_chirho(
                &translated_chirho,
                &subst_chirho.apply_ty_chirho(&expected_chirho),
                span_chirho,
            ),
            None => self.infer_expr_chirho(&translated_chirho),
        };
        subst_chirho = rest_subst_chirho.compose_chirho(&subst_chirho);
        self.apply_subst_all_chirho(&rest_subst_chirho);
        self.restore_type_pattern_aliases_chirho(type_pattern_scope_mark_chirho);
        self.env_chirho.pop_scope_chirho();
        (subst_chirho, ty_chirho)
    }

    /// The typing every block had before row 710, kept for a block the translation does not cover
    /// (see the module note): one block-wide monad, shape-only, no wanted and no evidence.
    fn infer_shape_only_do_chirho(
        &mut self,
        stmts_chirho: &[StmtChirho],
        last_expected_chirho: Option<TyChirho>,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        // Monadic do-notation:
        //   do { expr }       ≡ expr                             (last stmt)
        //   do { expr; rest } ≡ expr >> do { rest }              (ExprStmt)
        //   do { x <- m; rest } ≡ m >>= \x -> do { rest }       (BindStmt)
        //   do { let binds; rest } ≡ let binds in do { rest }    (LetStmt)
        //
        // All monadic expressions in the block share the same monad
        // constructor `m_chirho`. We use a fresh type variable for it
        // and unify as we go. The bind pattern variable `x` gets the
        // unwrapped type `a` from `m a`.
        let m_chirho = self.fresh_var_chirho(); // the monad constructor
        // Only a bind or a non-final expression makes the block monadic (Report
        // section 3.14): `do {e}` is `e`, and lets then a final expression are
        // `let decls in e`, whose type is the expression's own, with no `m a` and no
        // Monad (row 692).
        let monadic_chirho = stmts_chirho
            .iter()
            .enumerate()
            .any(|(idx_chirho, stmt_chirho)| match stmt_chirho {
                StmtChirho::BindChirho { .. } => true,
                StmtChirho::ExprChirho { .. } => idx_chirho + 1 < stmts_chirho.len(),
                StmtChirho::LetChirho { .. } => false,
            });
        let mut subst_chirho = SubstChirho::empty_chirho();
        let mut last_ty_chirho = TyChirho::unit_chirho();

        let type_pattern_scope_mark_chirho = self.type_pattern_alias_undo_chirho.len();
        self.env_chirho.push_scope_chirho();

        for (idx_chirho, stmt_chirho) in stmts_chirho.iter().enumerate() {
            let is_last_chirho = idx_chirho == stmts_chirho.len() - 1;

            match stmt_chirho {
                StmtChirho::ExprChirho { expr_chirho, .. } => {
                    let (s_chirho, ty_chirho) = match (&last_expected_chirho, is_last_chirho) {
                        (Some(expected_chirho), true) => {
                            let expected_sub_chirho = subst_chirho.apply_ty_chirho(expected_chirho);
                            self.infer_expr_against_expected_chirho(
                                expr_chirho,
                                &expected_sub_chirho,
                                span_chirho,
                            )
                        }
                        _ => self.infer_expr_chirho(expr_chirho),
                    };
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);

                    if is_last_chirho && !monadic_chirho {
                        // `do {e}` and `do {let decls; e}`: no monad at all.
                        last_ty_chirho = ty_chirho;
                    } else if is_last_chirho {
                        // Last statement: its type IS the do-block type.
                        // Unify with `m_chirho a` to extract the monad.
                        let a_chirho = self.fresh_var_chirho();
                        let expected_chirho = TyChirho::AppChirho(
                            Box::new(m_chirho.clone()),
                            Box::new(a_chirho.clone()),
                        );
                        match self.unify_normalized_chirho(
                            &subst_chirho.apply_ty_chirho(&ty_chirho),
                            &subst_chirho.apply_ty_chirho(&expected_chirho),
                            span_chirho,
                        ) {
                            Ok(su_chirho) => {
                                subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                self.apply_subst_all_chirho(&su_chirho);
                            }
                            Err(_) => { /* allow fallthrough for simple expressions */ }
                        }
                        last_ty_chirho = ty_chirho;
                    } else {
                        // Non-last expression stmt: type should be `m_chirho _`
                        let discard_chirho = self.fresh_var_chirho();
                        let expected_chirho = TyChirho::AppChirho(
                            Box::new(m_chirho.clone()),
                            Box::new(discard_chirho),
                        );
                        if let Ok(su_chirho) = self.unify_normalized_chirho(
                            &subst_chirho.apply_ty_chirho(&ty_chirho),
                            &subst_chirho.apply_ty_chirho(&expected_chirho),
                            span_chirho,
                        ) {
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&su_chirho);
                        }
                        last_ty_chirho = ty_chirho;
                    }
                }
                StmtChirho::BindChirho {
                    pat_chirho,
                    expr_chirho,
                    ..
                } => {
                    let (s_chirho, ty_chirho) = self.infer_expr_chirho(expr_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);

                    // `x <- m`: m has type `M a`, bind x :: a
                    let elem_ty_chirho = self.fresh_var_chirho();
                    let expected_chirho = TyChirho::AppChirho(
                        Box::new(m_chirho.clone()),
                        Box::new(elem_ty_chirho.clone()),
                    );
                    if let Ok(su_chirho) = self.unify_normalized_chirho(
                        &subst_chirho.apply_ty_chirho(&ty_chirho),
                        &subst_chirho.apply_ty_chirho(&expected_chirho),
                        span_chirho,
                    ) {
                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                    }

                    // Bind the pattern variable(s) to the unwrapped type
                    let bound_ty_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    let sp_chirho = self.bind_pat_chirho(pat_chirho, &bound_ty_chirho);
                    subst_chirho = sp_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&sp_chirho);
                    last_ty_chirho = TyChirho::unit_chirho();
                }
                StmtChirho::LetChirho { binds_chirho, .. } => {
                    self.infer_local_binds_chirho(binds_chirho, &mut subst_chirho);
                }
            }
        }

        self.restore_type_pattern_aliases_chirho(type_pattern_scope_mark_chirho);
        self.env_chirho.pop_scope_chirho();
        (subst_chirho, last_ty_chirho)
    }
}

/// The expression a block whose every statement carries its selection stands for, or None for a
/// block with an unselected statement or a last statement that is not an expression.
fn translate_do_chirho(stmts_chirho: &[StmtChirho]) -> Option<ExprChirho> {
    let (last_chirho, init_chirho) = stmts_chirho.split_last()?;
    let StmtChirho::ExprChirho {
        expr_chirho: tail_chirho,
        then_chirho: None,
    } = last_chirho
    else {
        return None;
    };
    let mut body_chirho = tail_chirho.clone();
    for stmt_chirho in init_chirho.iter().rev() {
        body_chirho = match stmt_chirho {
            StmtChirho::ExprChirho {
                expr_chirho,
                then_chirho: Some(then_selected_chirho),
            } => apply_operation_chirho(
                then_selected_chirho,
                expr_chirho.clone(),
                body_chirho,
                expr_chirho.span_chirho(),
            ),
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                bind_chirho: Some(bind_selected_chirho),
                fail_chirho,
                span_chirho,
            } => {
                let continuation_chirho = match fail_chirho {
                    None => ExprChirho::LamChirho {
                        pats_chirho: vec![pat_chirho.clone()],
                        body_chirho: Box::new(body_chirho),
                        span_chirho: *span_chirho,
                    },
                    Some(fail_selected_chirho) => failable_continuation_chirho(
                        pat_chirho,
                        body_chirho,
                        fail_selected_chirho,
                        *span_chirho,
                    ),
                };
                apply_operation_chirho(
                    bind_selected_chirho,
                    expr_chirho.clone(),
                    continuation_chirho,
                    *span_chirho,
                )
            }
            StmtChirho::LetChirho {
                binds_chirho,
                span_chirho,
            } => ExprChirho::LetChirho {
                binds_chirho: binds_chirho.clone(),
                body_chirho: Box::new(body_chirho),
                span_chirho: *span_chirho,
            },
            _ => return None,
        };
    }
    Some(body_chirho)
}

/// `operation first second`, the selected operation an ordinary reference under its own origin.
fn apply_operation_chirho(
    selected_chirho: &SelectedOperationChirho,
    first_chirho: ExprChirho,
    second_chirho: ExprChirho,
    span_chirho: SpanChirho,
) -> ExprChirho {
    ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(selected_chirho.name_chirho.clone())),
            arg_chirho: Box::new(first_chirho),
            span_chirho,
        }),
        arg_chirho: Box::new(second_chirho),
        span_chirho,
    }
}

/// A failable pattern's continuation, `\v -> case v of pat -> rest; _ -> fail "..."`. The message
/// carries no span and no origin, so no evidence is keyed to it.
fn failable_continuation_chirho(
    pat_chirho: &PatChirho,
    rest_chirho: ExprChirho,
    fail_selected_chirho: &SelectedOperationChirho,
    span_chirho: SpanChirho,
) -> ExprChirho {
    let scrutinee_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        SCRUTINEE_CHIRHO,
        span_chirho,
    ));
    let failure_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(
            fail_selected_chirho.name_chirho.clone(),
        )),
        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::StringChirho(
            PATTERN_FAILURE_CHIRHO.to_string(),
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho,
    };
    let alternative_chirho = |pat_chirho: PatChirho, rhs_chirho: ExprChirho| AltChirho {
        pat_chirho,
        rhs_chirho: RhsChirho::UnguardedChirho(rhs_chirho),
        where_binds_chirho: Vec::new(),
        span_chirho,
    };
    ExprChirho::LamChirho {
        pats_chirho: vec![PatChirho::VarChirho(scrutinee_chirho.clone())],
        body_chirho: Box::new(ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(scrutinee_chirho)),
            alts_chirho: vec![
                alternative_chirho(pat_chirho.clone(), rest_chirho),
                alternative_chirho(PatChirho::WildcardChirho(span_chirho), failure_chirho),
            ],
            span_chirho,
        }),
        span_chirho,
    }
}
