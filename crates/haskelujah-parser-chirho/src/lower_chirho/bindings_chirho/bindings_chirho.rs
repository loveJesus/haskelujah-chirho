// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pattern and function bindings: equations, guards, `where`, `let`, statements and `do`
//! segments, and the merge of equations.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{
    ExprChirho, GuardedExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::LowerCtxChirho;
use super::bracketed_exprs_chirho::QualPartChirho;
use super::context_chirho::{is_expr_kind_chirho, is_pat_kind_chirho};
use crate::rec_stmt_chirho::{DoSegmentChirho, transform_recursive_do_chirho};

impl LowerCtxChirho {
    fn lower_pat_bind_parts_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> LoweredPatBindPartsChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut lhs_pats_chirho = Vec::new();
        let mut rhs_expr_chirho = None;
        let mut guarded_rhs_chirho = None;
        let mut where_binds_chirho = Vec::new();
        let mut past_eq_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho =>
                {
                    past_eq_chirho = true;
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::GuardedRhsChirho {
                        guarded_rhs_chirho = Some(
                            self.lower_guarded_rhs_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    } else if !past_eq_chirho && guarded_rhs_chirho.is_none() {
                        if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                            lhs_pats_chirho
                                .push(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                        }
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho {
                        where_binds_chirho =
                            self.lower_where_clause_chirho(n_chirho, child_chirho.start_chirho);
                    } else if rhs_expr_chirho.is_none()
                        && is_expr_kind_chirho(n_chirho.kind_chirho())
                    {
                        rhs_expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
                _ => {}
            }
        }

        let had_guarded_rhs_chirho = guarded_rhs_chirho.is_some();
        LoweredPatBindPartsChirho {
            lhs_pats_chirho,
            rhs_chirho: guarded_rhs_chirho.unwrap_or_else(|| {
                RhsChirho::UnguardedChirho(rhs_expr_chirho.unwrap_or(ExprChirho::LitChirho(
                    LitChirho::IntChirho(0, span_chirho, Some(self.fresh_origin_chirho())),
                )))
            }),
            where_binds_chirho,
            had_guarded_rhs_chirho,
        }
    }

    fn pat_bind_head_name_chirho(pat_chirho: &PatChirho) -> Option<NameChirho> {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => Some(name_chirho.clone()),
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. } => {
                Self::pat_bind_head_name_chirho(inner_chirho)
            }
            PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
                Self::pat_bind_head_name_chirho(pat_chirho)
            }
            _ => None,
        }
    }

    fn lower_pat_bind_parts_to_local_bind_chirho(
        &self,
        parts_chirho: LoweredPatBindPartsChirho,
        span_chirho: SpanChirho,
    ) -> LocalBindChirho {
        if let Some(name_chirho) = parts_chirho
            .lhs_pats_chirho
            .first()
            .and_then(Self::pat_bind_head_name_chirho)
            .filter(|_| {
                parts_chirho.had_guarded_rhs_chirho || parts_chirho.lhs_pats_chirho.len() > 1
            })
        {
            return LocalBindChirho::FunBindChirho {
                name_chirho,
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: parts_chirho.lhs_pats_chirho.into_iter().skip(1).collect(),
                    rhs_chirho: parts_chirho.rhs_chirho,
                    where_binds_chirho: parts_chirho.where_binds_chirho,
                    span_chirho,
                }],
                span_chirho,
            };
        }

        let pat_final_chirho = parts_chirho
            .lhs_pats_chirho
            .into_iter()
            .next()
            .unwrap_or(PatChirho::WildcardChirho(span_chirho));
        let rhs_final_chirho = match parts_chirho.rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => {
                let expr_chirho = if parts_chirho.where_binds_chirho.is_empty() {
                    expr_chirho
                } else {
                    ExprChirho::LetChirho {
                        binds_chirho: merge_local_fun_binds_chirho(parts_chirho.where_binds_chirho),
                        body_chirho: Box::new(expr_chirho),
                        span_chirho,
                    }
                };
                RhsChirho::UnguardedChirho(expr_chirho)
            }
            rhs_chirho => rhs_chirho,
        };

        LocalBindChirho::PatBindChirho {
            pat_chirho: pat_final_chirho,
            rhs_chirho: rhs_final_chirho,
            span_chirho,
        }
    }

    fn lower_pat_bind_parts_to_decl_chirho(
        &self,
        parts_chirho: LoweredPatBindPartsChirho,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        if let Some(name_chirho) = parts_chirho
            .lhs_pats_chirho
            .first()
            .and_then(Self::pat_bind_head_name_chirho)
            .filter(|_| {
                parts_chirho.had_guarded_rhs_chirho || parts_chirho.lhs_pats_chirho.len() > 1
            })
        {
            return DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: parts_chirho.lhs_pats_chirho.into_iter().skip(1).collect(),
                    rhs_chirho: parts_chirho.rhs_chirho,
                    where_binds_chirho: parts_chirho.where_binds_chirho,
                    span_chirho,
                }],
                span_chirho,
            };
        }

        let pat_final_chirho = parts_chirho
            .lhs_pats_chirho
            .into_iter()
            .next()
            .unwrap_or(PatChirho::WildcardChirho(span_chirho));
        let rhs_final_chirho = match parts_chirho.rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => {
                let expr_chirho = if parts_chirho.where_binds_chirho.is_empty() {
                    expr_chirho
                } else {
                    ExprChirho::LetChirho {
                        binds_chirho: merge_local_fun_binds_chirho(parts_chirho.where_binds_chirho),
                        body_chirho: Box::new(expr_chirho),
                        span_chirho,
                    }
                };
                RhsChirho::UnguardedChirho(expr_chirho)
            }
            rhs_chirho => rhs_chirho,
        };

        DeclChirho::PatBindChirho {
            pat_chirho: pat_final_chirho,
            rhs_chirho: rhs_final_chirho,
            span_chirho,
        }
    }

    /// Lower a `PatBindChirho` CST node into a `LocalBindChirho::PatBindChirho`.
    /// Structure: pattern = rhs
    pub(super) fn lower_pat_bind_local_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> LocalBindChirho {
        let parts_chirho = self.lower_pat_bind_parts_chirho(node_chirho, base_chirho, span_chirho);
        self.lower_pat_bind_parts_to_local_bind_chirho(parts_chirho, span_chirho)
    }

    /// Lower a top-level `PatBindChirho` CST node into a `DeclChirho::PatBindChirho`.
    /// Structure: pattern = rhs
    pub(super) fn lower_pat_bind_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let parts_chirho = self.lower_pat_bind_parts_chirho(node_chirho, base_chirho, span_chirho);
        self.lower_pat_bind_parts_to_decl_chirho(parts_chirho, span_chirho)
    }

    pub(super) fn lower_fun_bind_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut matches_chirho = Vec::new();

        // The FunBind node contains Match children and an optional
        // WhereClause sibling; the function name is the first VarId or
        // ConId token inside the first Match.
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::MatchChirho {
                    if name_chirho.is_none() {
                        name_chirho = self.extract_fun_name_from_match_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                        );
                    }
                    matches_chirho
                        .push(self.lower_match_arm_chirho(n_chirho, child_chirho.start_chirho));
                } else if n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho {
                    // Attach where bindings to the last match arm
                    let where_binds_chirho =
                        self.lower_where_clause_chirho(n_chirho, child_chirho.start_chirho);
                    if let Some(last_arm_chirho) = matches_chirho.last_mut() {
                        last_arm_chirho.where_binds_chirho = where_binds_chirho;
                    }
                }
            }
        }

        DeclChirho::FunBindChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            matches_chirho,
            span_chirho,
        }
    }

    fn extract_fun_name_from_match_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<NameChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut saw_backtick_chirho = false;
        let mut saw_lhs_operand_chirho = false;
        let mut fallback_name_chirho = None;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho
                        || tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                    {
                        break;
                    }
                    if tok_chirho.kind_chirho() == TokenKindChirho::BacktickChirho {
                        saw_backtick_chirho = !saw_backtick_chirho;
                        continue;
                    }
                    if saw_backtick_chirho
                        && (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedVarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedVarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedConSymChirho)
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    if saw_lhs_operand_chirho
                        && (tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedVarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedConSymChirho)
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    if fallback_name_chirho.is_none()
                        && (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedVarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedVarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedConSymChirho)
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        fallback_name_chirho =
                            Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    if matches!(
                        tok_chirho.kind_chirho(),
                        TokenKindChirho::VarIdChirho
                            | TokenKindChirho::ConIdChirho
                            | TokenKindChirho::QualifiedVarIdChirho
                            | TokenKindChirho::QualifiedConIdChirho
                            | TokenKindChirho::UnderscoreReservedIdChirho
                            | TokenKindChirho::IntegerLiteralChirho
                            | TokenKindChirho::FloatLiteralChirho
                            | TokenKindChirho::CharLiteralChirho
                            | TokenKindChirho::StringLiteralChirho
                            | TokenKindChirho::LeftParenChirho
                            | TokenKindChirho::LeftBracketChirho
                            | TokenKindChirho::TildeChirho
                    ) {
                        saw_lhs_operand_chirho = true;
                    } else if matches!(
                        tok_chirho.kind_chirho(),
                        TokenKindChirho::VarSymChirho | TokenKindChirho::QualifiedVarSymChirho
                    ) {
                        let tok_text_chirho = self
                            .name_from_token_chirho(
                                tok_chirho,
                                self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                ),
                            )
                            .text_chirho()
                            .to_string();
                        if tok_text_chirho == "!" || tok_text_chirho == "-" {
                            saw_lhs_operand_chirho = true;
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                        saw_lhs_operand_chirho = true;
                    }
                }
            }
        }
        fallback_name_chirho
    }

    fn lower_match_arm_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> MatchArmChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut pats_chirho = Vec::new();
        let mut rhs_expr_chirho = None;
        let mut guarded_rhs_chirho: Option<RhsChirho> = None;
        let mut saw_equals_chirho = false;
        let mut where_binds_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::GuardedRhsChirho {
                        guarded_rhs_chirho = Some(
                            self.lower_guarded_rhs_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho {
                        where_binds_chirho =
                            self.lower_where_clause_chirho(n_chirho, child_chirho.start_chirho);
                    } else if !saw_equals_chirho && is_pat_kind_chirho(n_chirho.kind_chirho()) {
                        pats_chirho
                            .push(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_equals_chirho && rhs_expr_chirho.is_none() {
                        rhs_expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        let rhs_chirho = if let Some(guards_chirho) = guarded_rhs_chirho {
            guards_chirho
        } else {
            RhsChirho::UnguardedChirho(rhs_expr_chirho.unwrap_or(self.placeholder_expr_chirho()))
        };

        MatchArmChirho {
            pats_chirho,
            rhs_chirho,
            where_binds_chirho,
            span_chirho,
        }
    }

    /// Lower a `GuardedRhsChirho` CST node into its guarded arms, each with its
    /// qualifiers in source order (`| qual, ..., qual = body`). A pattern or
    /// `let` qualifier stays a qualifier, so a failing arm can go on to the next
    /// arm and, after the last, to the next row.
    /// workflow: language-features-chirho/pattern-matching-chirho
    fn lower_guarded_rhs_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> RhsChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut guards_chirho = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                && n_chirho.kind_chirho() == SyntaxKindChirho::GuardChirho
                && let Some(ge_chirho) =
                    self.lower_guard_chirho(n_chirho, child_chirho.start_chirho)
            {
                guards_chirho.push(ge_chirho);
            }
        }

        RhsChirho::GuardedChirho(guards_chirho)
    }

    /// Lower a single `GuardChirho` CST node into a qualifier sequence and body.
    /// Structure:
    ///   | guard_expr = body_expr
    ///   | pat <- expr, guard_expr = body_expr
    ///   | quals -> body_expr            (a multi-way if's arm)
    pub(super) fn lower_guard_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<GuardedExprChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut current_qual_parts_chirho: Vec<QualPartChirho> = Vec::new();
        let mut quals_chirho: Vec<StmtChirho> = Vec::new();
        let mut body_expr_chirho = None;
        let mut saw_equals_chirho = false;
        let mut saw_pipe_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho {
                        saw_pipe_chirho = true;
                    } else if matches!(
                        tok_chirho.kind_chirho(),
                        // `=` in an equation, `->` in a multi-way if's arm.
                        TokenKindChirho::EqualsChirho | TokenKindChirho::RightArrowChirho
                    ) {
                        if !current_qual_parts_chirho.is_empty() {
                            if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                &mut current_qual_parts_chirho,
                                span_chirho,
                            ) {
                                quals_chirho.push(qual_chirho);
                            }
                            current_qual_parts_chirho.clear();
                        }
                        saw_equals_chirho = true;
                    } else if saw_pipe_chirho && !saw_equals_chirho {
                        if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho {
                            if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                &mut current_qual_parts_chirho,
                                span_chirho,
                            ) {
                                quals_chirho.push(qual_chirho);
                            }
                            current_qual_parts_chirho.clear();
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftArrowChirho {
                            current_qual_parts_chirho.push(QualPartChirho::ArrowChirho);
                        } else if matches!(
                            tok_chirho.kind_chirho(),
                            TokenKindChirho::ConSymChirho | TokenKindChirho::QualifiedConSymChirho
                        ) {
                            current_qual_parts_chirho.push(QualPartChirho::PatOpChirho(
                                self.name_from_token_chirho(
                                    tok_chirho,
                                    self.span_chirho(
                                        child_chirho.start_chirho,
                                        child_chirho.end_chirho,
                                    ),
                                ),
                            ));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_pipe_chirho && !saw_equals_chirho {
                        if n_chirho.kind_chirho() == SyntaxKindChirho::LetStmtChirho {
                            if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                &mut current_qual_parts_chirho,
                                span_chirho,
                            ) {
                                quals_chirho.push(qual_chirho);
                            }
                            current_qual_parts_chirho.clear();
                            quals_chirho.push(StmtChirho::LetChirho {
                                binds_chirho: self.lower_let_stmt_binds_chirho(
                                    n_chirho,
                                    child_chirho.start_chirho,
                                ),
                                span_chirho: self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                ),
                            });
                        } else if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                            current_qual_parts_chirho.push(QualPartChirho::PatChirho(
                                self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                            ));
                        } else {
                            current_qual_parts_chirho.push(QualPartChirho::ExprChirho(
                                self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                            ));
                        }
                    } else if saw_equals_chirho && body_expr_chirho.is_none() {
                        body_expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        if !current_qual_parts_chirho.is_empty()
            && let Some(qual_chirho) =
                self.flush_qual_parts_chirho(&mut current_qual_parts_chirho, span_chirho)
        {
            quals_chirho.push(qual_chirho);
        }

        Some(GuardedExprChirho {
            quals_chirho,
            body_chirho: body_expr_chirho.unwrap_or(self.placeholder_expr_chirho()),
            span_chirho,
        })
    }

    /// Lower a `WhereClauseChirho` CST node into a list of local bindings.
    /// The node structure is: `where { decl ; decl ; ... }`
    pub(super) fn lower_where_clause_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<LocalBindChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut binds_chirho = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::FunBindChirho {
                    let child_span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    let decl_chirho = self.lower_fun_bind_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        child_span_chirho,
                    );
                    if let DeclChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho,
                        span_chirho,
                    } = decl_chirho
                        && !matches_chirho.is_empty()
                    {
                        binds_chirho.push(LocalBindChirho::FunBindChirho {
                            name_chirho,
                            matches_chirho,
                            span_chirho,
                        });
                    }
                } else if n_chirho.kind_chirho() == SyntaxKindChirho::PatBindChirho {
                    let child_span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    let pb_chirho = self.lower_pat_bind_local_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        child_span_chirho,
                    );
                    binds_chirho.push(pb_chirho);
                } else if n_chirho.kind_chirho() == SyntaxKindChirho::TypeSigDeclChirho {
                    let child_span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    let decls_chirho = self.lower_type_sigs_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        child_span_chirho,
                    );
                    for decl_chirho in decls_chirho {
                        if let DeclChirho::TypeSigChirho {
                            name_chirho,
                            ty_chirho,
                            span_chirho,
                        } = decl_chirho
                        {
                            binds_chirho.push(LocalBindChirho::TypeSigChirho {
                                name_chirho,
                                ty_chirho,
                                span_chirho,
                            });
                        }
                    }
                }
            }
        }

        merge_local_fun_binds_chirho(binds_chirho)
    }

    /// Extract bindings from a LetStmt CST node in do-notation.
    /// The LetStmt contains `let` keyword followed by FunBind children.
    pub(super) fn lower_let_stmt_binds_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<LocalBindChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut binds_chirho = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::FunBindChirho {
                    let child_span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    let decl_chirho = self.lower_fun_bind_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        child_span_chirho,
                    );
                    if let DeclChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho,
                        span_chirho,
                    } = decl_chirho
                        && !matches_chirho.is_empty()
                    {
                        binds_chirho.push(LocalBindChirho::FunBindChirho {
                            name_chirho,
                            matches_chirho,
                            span_chirho,
                        });
                    }
                } else if n_chirho.kind_chirho() == SyntaxKindChirho::TypeSigDeclChirho {
                    let child_span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    let decls_chirho = self.lower_type_sigs_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        child_span_chirho,
                    );
                    for decl_chirho in decls_chirho {
                        if let DeclChirho::TypeSigChirho {
                            name_chirho,
                            ty_chirho,
                            span_chirho,
                        } = decl_chirho
                        {
                            binds_chirho.push(LocalBindChirho::TypeSigChirho {
                                name_chirho,
                                ty_chirho,
                                span_chirho,
                            });
                        }
                    }
                } else if n_chirho.kind_chirho() == SyntaxKindChirho::PatBindChirho {
                    let child_span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    let pb_chirho = self.lower_pat_bind_local_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        child_span_chirho,
                    );
                    binds_chirho.push(pb_chirho);
                }
            }
        }

        merge_local_fun_binds_chirho(binds_chirho)
    }

    /// Lower a do-like CST block while preserving explicit RecursiveDo segments.
    ///
    /// The pure transformation in `rec_stmt_chirho` removes the segment marker before the
    /// typed AST leaves the parser crate.
    /// workflow: recursive-do-chirho
    pub(super) fn lower_do_segments_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<DoSegmentChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut segments_chirho = Vec::new();

        for child_chirho in &children_chirho {
            let GreenElementChirho::NodeChirho(node_chirho) = child_chirho.element_chirho else {
                continue;
            };
            let child_span_chirho =
                self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
            let stmt_chirho = match node_chirho.kind_chirho() {
                SyntaxKindChirho::DoStmtChirho => StmtChirho::expr_stmt_chirho(
                    self.lower_first_expr_in_node_chirho(node_chirho, child_chirho.start_chirho),
                ),
                SyntaxKindChirho::BindStmtChirho => {
                    self.lower_bind_stmt_chirho(node_chirho, child_chirho.start_chirho)
                }
                SyntaxKindChirho::LetStmtChirho => StmtChirho::LetChirho {
                    binds_chirho: self
                        .lower_let_stmt_binds_chirho(node_chirho, child_chirho.start_chirho),
                    span_chirho: child_span_chirho,
                },
                SyntaxKindChirho::RecStmtChirho => {
                    let nested_segments_chirho =
                        self.lower_do_segments_chirho(node_chirho, child_chirho.start_chirho);
                    let nested_stmts_chirho = transform_recursive_do_chirho(
                        nested_segments_chirho,
                        false,
                        child_span_chirho,
                        &mut self.origin_supply_chirho.borrow_mut(),
                    );
                    segments_chirho.push(DoSegmentChirho::RecursiveChirho {
                        stmts_chirho: nested_stmts_chirho,
                        span_chirho: child_span_chirho,
                    });
                    continue;
                }
                _ => StmtChirho::expr_stmt_chirho(
                    self.lower_expr_chirho(node_chirho, child_chirho.start_chirho),
                ),
            };
            segments_chirho.push(DoSegmentChirho::StatementChirho(stmt_chirho));
        }

        segments_chirho
    }

    fn lower_bind_stmt_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> StmtChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut pat_chirho = None;
        let mut expr_chirho = None;
        let mut saw_arrow_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::LeftArrowChirho {
                        saw_arrow_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if !saw_arrow_chirho && pat_chirho.is_none() {
                        pat_chirho =
                            Some(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_arrow_chirho && expr_chirho.is_none() {
                        expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        StmtChirho::BindChirho {
            bind_chirho: None,
            fail_chirho: None,
            pat_chirho: pat_chirho.unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
            expr_chirho: expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
            span_chirho,
        }
    }

    /// Extract the body expression from a FunBindChirho CST node that the
    /// parser placed after `in` in a let-expression.  The structure is
    /// typically `FunBindChirho > MatchChirho > <expr children>`, where the
    /// match contains the body expression (possibly as a bare variable
    /// or a full RHS).
    pub(super) fn lower_let_body_from_funbind_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ExprChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                && n_chirho.kind_chirho() == SyntaxKindChirho::MatchChirho
            {
                // Look inside the match for expression nodes or bare tokens
                let match_children_chirho =
                    self.semantic_children_chirho(n_chirho, child_chirho.start_chirho);
                for mc_chirho in &match_children_chirho {
                    if let GreenElementChirho::NodeChirho(inner_chirho) = mc_chirho.element_chirho
                        && is_expr_kind_chirho(inner_chirho.kind_chirho())
                    {
                        return self.lower_expr_chirho(inner_chirho, mc_chirho.start_chirho);
                    }
                    // Bare VarId or ConId token — treat as a variable expression
                    if let GreenElementChirho::TokenChirho(tok_chirho) = mc_chirho.element_chirho
                        && matches!(
                            tok_chirho.kind_chirho(),
                            TokenKindChirho::VarIdChirho
                                | TokenKindChirho::ConIdChirho
                                | TokenKindChirho::VarSymChirho
                        )
                    {
                        let s_chirho =
                            self.span_chirho(mc_chirho.start_chirho, mc_chirho.end_chirho);
                        let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                        return self.reference_expr_chirho(name_chirho);
                    }
                }
            }
        }
        self.placeholder_expr_chirho()
    }
}

struct LoweredPatBindPartsChirho {
    lhs_pats_chirho: Vec<PatChirho>,
    rhs_chirho: RhsChirho,
    where_binds_chirho: Vec<LocalBindChirho>,
    had_guarded_rhs_chirho: bool,
}

/// Merge consecutive `DeclChirho::FunBindChirho` with the same name into a
/// single multi-equation binding.  Haskell requires that all equations for
/// the same function appear consecutively; this pass collapses them into one
/// `FunBindChirho` with `matches_chirho` holding every equation.
pub(super) fn merge_fun_binds_chirho(decls_chirho: Vec<DeclChirho>) -> Vec<DeclChirho> {
    let mut merged_chirho: Vec<DeclChirho> = Vec::with_capacity(decls_chirho.len());

    for decl_chirho in decls_chirho {
        let should_merge_chirho = if let DeclChirho::FunBindChirho {
            ref name_chirho, ..
        } = decl_chirho
        {
            if let Some(DeclChirho::FunBindChirho {
                name_chirho: prev_name_chirho,
                ..
            }) = merged_chirho.last()
            {
                name_chirho.text_chirho() == prev_name_chirho.text_chirho()
            } else {
                false
            }
        } else {
            false
        };

        if should_merge_chirho {
            if let DeclChirho::FunBindChirho {
                matches_chirho: new_matches_chirho,
                span_chirho: new_span_chirho,
                ..
            } = decl_chirho
                && let Some(DeclChirho::FunBindChirho {
                    matches_chirho,
                    span_chirho,
                    ..
                }) = merged_chirho.last_mut()
            {
                matches_chirho.extend(new_matches_chirho);
                // Extend the span to cover all equations
                if let Some(m_chirho) = span_chirho.merge_chirho(new_span_chirho) {
                    *span_chirho = m_chirho;
                }
            }
        } else {
            merged_chirho.push(decl_chirho);
        }
    }

    merged_chirho
}

/// Merge consecutive `LocalBindChirho::FunBindChirho` with the same name.
pub(super) fn merge_local_fun_binds_chirho(
    binds_chirho: Vec<LocalBindChirho>,
) -> Vec<LocalBindChirho> {
    let mut merged_chirho: Vec<LocalBindChirho> = Vec::with_capacity(binds_chirho.len());

    for bind_chirho in binds_chirho {
        let should_merge_chirho = if let LocalBindChirho::FunBindChirho {
            ref name_chirho, ..
        } = bind_chirho
        {
            if let Some(LocalBindChirho::FunBindChirho {
                name_chirho: prev_name_chirho,
                ..
            }) = merged_chirho.last()
            {
                name_chirho.text_chirho() == prev_name_chirho.text_chirho()
            } else {
                false
            }
        } else {
            false
        };

        if should_merge_chirho {
            if let LocalBindChirho::FunBindChirho {
                matches_chirho: new_matches_chirho,
                span_chirho: new_span_chirho,
                ..
            } = bind_chirho
                && let Some(LocalBindChirho::FunBindChirho {
                    matches_chirho,
                    span_chirho,
                    ..
                }) = merged_chirho.last_mut()
            {
                matches_chirho.extend(new_matches_chirho);
                if let Some(m_chirho) = span_chirho.merge_chirho(new_span_chirho) {
                    *span_chirho = m_chirho;
                }
            }
        } else {
            merged_chirho.push(bind_chirho);
        }
    }

    merged_chirho
}
