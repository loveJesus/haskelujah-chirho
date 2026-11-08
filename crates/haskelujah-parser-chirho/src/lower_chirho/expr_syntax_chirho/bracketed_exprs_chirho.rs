// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Parenthesized and bracketed expressions: sections, tuples, arithmetic sequences and
//! comprehensions.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::expr_chirho::{
    ComprehensionQualifierChirho, ComprehensionTransformChirho, ExprChirho, StmtChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::context_chirho::{is_expr_kind_chirho, is_pat_kind_chirho};
use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    /// A parenthesized expression: a left or right section, a tuple or tuple section, or a
    /// grouped expression. The ParenExpr arm of `lower_expr_chirho`, extracted in row 641.
    pub(super) fn lower_paren_expr_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> ExprChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        // Detect operator sections by checking for operator tokens
        // outside of expression nodes.
        let mut leading_op_chirho: Option<String> = None;
        let mut trailing_op_chirho: Option<String> = None;
        let mut expr_nodes_chirho: Vec<&ChildChirho<'_>> = Vec::new();
        let mut inside_backticks_chirho = false;
        let mut backtick_op_text_chirho: Option<String> = None;

        for child_chirho in children_chirho.iter() {
            match &child_chirho.element_chirho {
                GreenElementChirho::NodeChirho(_) => {
                    expr_nodes_chirho.push(child_chirho);
                }
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::BacktickChirho {
                        if inside_backticks_chirho
                            && let Some(op_text_chirho) = backtick_op_text_chirho.take()
                        {
                            if expr_nodes_chirho.is_empty() {
                                leading_op_chirho = Some(op_text_chirho);
                            } else {
                                trailing_op_chirho = Some(op_text_chirho);
                            }
                        }
                        inside_backticks_chirho = !inside_backticks_chirho;
                    } else if inside_backticks_chirho
                        && matches!(
                            kind_chirho,
                            TokenKindChirho::VarIdChirho
                                | TokenKindChirho::ConIdChirho
                                | TokenKindChirho::QualifiedVarIdChirho
                                | TokenKindChirho::QualifiedConIdChirho
                                | TokenKindChirho::VarSymChirho
                                | TokenKindChirho::ConSymChirho
                                | TokenKindChirho::QualifiedVarSymChirho
                                | TokenKindChirho::QualifiedConSymChirho
                        )
                    {
                        backtick_op_text_chirho = Some(tok_chirho.text_chirho().to_string());
                    } else if matches!(
                        kind_chirho,
                        TokenKindChirho::VarSymChirho
                            | TokenKindChirho::ConSymChirho
                            | TokenKindChirho::QualifiedVarSymChirho
                            | TokenKindChirho::QualifiedConSymChirho
                    ) {
                        if expr_nodes_chirho.is_empty() {
                            leading_op_chirho = Some(tok_chirho.text_chirho().to_string());
                        } else {
                            trailing_op_chirho = Some(tok_chirho.text_chirho().to_string());
                        }
                    }
                }
            }
        }

        // Left section: (op expr) — operator token before any expr node
        if let Some(ref op_text_chirho) = leading_op_chirho
            && let Some(arg_child_chirho) = expr_nodes_chirho.first()
        {
            let arg_expr_chirho = self.lower_expr_from_child_chirho(arg_child_chirho);
            return ExprChirho::LeftSectionChirho {
                op_chirho: self
                    .reference_name_chirho(self.name_from_text_chirho(op_text_chirho, span_chirho)),
                arg_chirho: Box::new(arg_expr_chirho),
                span_chirho,
            };
        }

        // Right section: (expr op) — operator token after an expr node
        if let Some(ref op_text_chirho) = trailing_op_chirho
            && let Some(arg_child_chirho) = expr_nodes_chirho.first()
        {
            let arg_expr_chirho = self.lower_expr_from_child_chirho(arg_child_chirho);
            return ExprChirho::RightSectionChirho {
                arg_chirho: Box::new(arg_expr_chirho),
                op_chirho: self
                    .reference_name_chirho(self.name_from_text_chirho(op_text_chirho, span_chirho)),
                span_chirho,
            };
        }

        // Fallback: check if the single child node is an InfixExprChirho
        // with a trailing operator (right section) or leading operator
        // (left section). The parser consumes the operator into an
        // InfixExprChirho, so it won't appear as a direct token child
        // of ParenExprChirho.
        if expr_nodes_chirho.len() == 1
            && leading_op_chirho.is_none()
            && trailing_op_chirho.is_none()
            && let GreenElementChirho::NodeChirho(inner_node_chirho) =
                expr_nodes_chirho[0].element_chirho
            && inner_node_chirho.kind_chirho() == SyntaxKindChirho::InfixExprChirho
        {
            let inner_children_chirho =
                self.semantic_children_chirho(inner_node_chirho, expr_nodes_chirho[0].start_chirho);
            // Collect expression nodes and operator tokens from InfixExpr
            let mut inner_expr_nodes_chirho: Vec<&ChildChirho<'_>> = Vec::new();
            let mut inner_ops_chirho: Vec<(String, SpanChirho)> = Vec::new();
            let mut inner_inside_backticks_chirho = false;
            let mut inner_backtick_op_text_chirho: Option<String> = None;
            for ic_chirho in &inner_children_chirho {
                match ic_chirho.element_chirho {
                    GreenElementChirho::NodeChirho(_) => {
                        inner_expr_nodes_chirho.push(ic_chirho);
                    }
                    GreenElementChirho::TokenChirho(tok_chirho) => {
                        let k_chirho = tok_chirho.kind_chirho();
                        if k_chirho == TokenKindChirho::BacktickChirho {
                            if inner_inside_backticks_chirho
                                && let Some(op_text_chirho) = inner_backtick_op_text_chirho.take()
                            {
                                let s_chirho =
                                    self.span_chirho(ic_chirho.start_chirho, ic_chirho.end_chirho);
                                inner_ops_chirho.push((op_text_chirho, s_chirho));
                            }
                            inner_inside_backticks_chirho = !inner_inside_backticks_chirho;
                        } else if inner_inside_backticks_chirho
                            && matches!(
                                k_chirho,
                                TokenKindChirho::VarIdChirho
                                    | TokenKindChirho::ConIdChirho
                                    | TokenKindChirho::QualifiedVarIdChirho
                                    | TokenKindChirho::QualifiedConIdChirho
                                    | TokenKindChirho::VarSymChirho
                                    | TokenKindChirho::ConSymChirho
                                    | TokenKindChirho::QualifiedVarSymChirho
                                    | TokenKindChirho::QualifiedConSymChirho
                            )
                        {
                            inner_backtick_op_text_chirho =
                                Some(tok_chirho.text_chirho().to_string());
                        } else if k_chirho == TokenKindChirho::VarSymChirho
                            || k_chirho == TokenKindChirho::ConSymChirho
                            || k_chirho == TokenKindChirho::QualifiedVarSymChirho
                            || k_chirho == TokenKindChirho::QualifiedConSymChirho
                        {
                            let s_chirho =
                                self.span_chirho(ic_chirho.start_chirho, ic_chirho.end_chirho);
                            inner_ops_chirho.push((tok_chirho.text_chirho().to_string(), s_chirho));
                        }
                    }
                }
            }
            // Right section: N exprs, N ops → trailing operator has no RHS
            if !inner_ops_chirho.is_empty()
                && inner_expr_nodes_chirho.len() == inner_ops_chirho.len()
            {
                let (op_text_chirho, _op_span_chirho) = inner_ops_chirho.last().unwrap().clone();
                // Lower the expression part (everything before the trailing op)
                let arg_expr_chirho = if inner_expr_nodes_chirho.len() == 1 {
                    self.lower_expr_from_child_chirho(inner_expr_nodes_chirho[0])
                } else {
                    // Multiple exprs before trailing op: rebuild infix
                    let sub_exprs_chirho: Vec<ExprChirho> = inner_expr_nodes_chirho
                        .iter()
                        .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                        .collect();
                    // A qualified operator keeps its qualifier here, as
                    // on the direct-token path (row 609).
                    let sub_ops_chirho: Vec<NameChirho> = inner_ops_chirho
                        [..inner_ops_chirho.len() - 1]
                        .iter()
                        .map(|(t_chirho, s_chirho)| {
                            self.reference_name_chirho(
                                self.name_from_text_chirho(t_chirho, *s_chirho),
                            )
                        })
                        .collect();
                    self.resolve_infix_precedence_chirho(
                        sub_exprs_chirho,
                        sub_ops_chirho,
                        span_chirho,
                    )
                };
                // `(2 P.-)` names Prelude's `-`, not one unqualified
                // `P.-` the desugarer took for a constructor (row 609).
                return ExprChirho::RightSectionChirho {
                    arg_chirho: Box::new(arg_expr_chirho),
                    op_chirho: self.reference_name_chirho(
                        self.name_from_text_chirho(&op_text_chirho, span_chirho),
                    ),
                    span_chirho,
                };
            }
            // Left section: 0 exprs before first op, 1+ exprs after
            // (op expr) already handled above for direct tokens;
            // this catches case where parse_expr made the op part of InfixExpr
        }

        // Normal parenthesized expr or tuple, or TupleSections
        // Count commas to detect tuple section gaps
        let comma_count_chirho = children_chirho
            .iter()
            .filter(|c_chirho| {
                matches!(
                    c_chirho.element_chirho,
                    GreenElementChirho::TokenChirho(t_chirho)
                        if t_chirho.kind_chirho() == TokenKindChirho::CommaChirho
                )
            })
            .count();

        let arity_chirho = if comma_count_chirho > 0 {
            comma_count_chirho + 1
        } else {
            0
        };

        // TupleSections: if there are commas and fewer expr nodes than
        // positions, there are gaps → desugar to lambda
        if arity_chirho > 0 && expr_nodes_chirho.len() < arity_chirho {
            // Build Vec<Option<ExprChirho>> by walking children
            let mut slots_chirho: Vec<Option<ExprChirho>> = Vec::new();
            let mut expr_idx_chirho = 0usize;
            let mut at_start_chirho = true;
            for child_chirho in &children_chirho {
                match child_chirho.element_chirho {
                    GreenElementChirho::TokenChirho(tok_chirho) => {
                        let k_chirho = tok_chirho.kind_chirho();
                        if k_chirho == TokenKindChirho::CommaChirho {
                            if at_start_chirho {
                                // Gap before first comma
                                slots_chirho.push(None);
                            }
                            at_start_chirho = true;
                        }
                    }
                    GreenElementChirho::NodeChirho(n_chirho) => {
                        if is_expr_kind_chirho(n_chirho.kind_chirho())
                            && expr_idx_chirho < expr_nodes_chirho.len()
                        {
                            slots_chirho.push(Some(
                                self.lower_expr_from_child_chirho(
                                    expr_nodes_chirho[expr_idx_chirho],
                                ),
                            ));
                            expr_idx_chirho += 1;
                            at_start_chirho = false;
                        }
                    }
                }
            }
            // If last position is a gap (trailing comma)
            if at_start_chirho && comma_count_chirho > 0 {
                slots_chirho.push(None);
            }

            // Generate lambda: \$ts_0 $ts_1 ... -> (e0_or_$ts, e1_or_$ts, ...)
            let mut gap_idx_chirho = 0usize;
            let mut params_chirho: Vec<PatChirho> = Vec::new();
            let mut elements_chirho: Vec<ExprChirho> = Vec::new();
            for slot_chirho in &slots_chirho {
                match slot_chirho {
                    Some(expr_chirho) => {
                        elements_chirho.push(expr_chirho.clone());
                    }
                    None => {
                        let name_chirho = format!("$ts_{}", gap_idx_chirho);
                        let n_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                            &name_chirho,
                            span_chirho,
                        ));
                        params_chirho.push(PatChirho::VarChirho(n_chirho.clone()));
                        elements_chirho.push(self.reference_expr_chirho(n_chirho));
                        gap_idx_chirho += 1;
                    }
                }
            }
            ExprChirho::LamChirho {
                pats_chirho: params_chirho,
                body_chirho: Box::new(ExprChirho::TupleChirho {
                    elements_chirho,
                    span_chirho,
                }),
                span_chirho,
            }
        } else if expr_nodes_chirho.len() == 1 && comma_count_chirho == 0 {
            let inner_chirho = self.lower_expr_from_child_chirho(expr_nodes_chirho[0]);
            ExprChirho::ParenChirho {
                inner_chirho: Box::new(inner_chirho),
                span_chirho,
            }
        } else if expr_nodes_chirho.len() > 1 {
            let elements_chirho: Vec<_> = expr_nodes_chirho
                .iter()
                .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                .collect();
            ExprChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            }
        } else {
            ExprChirho::TupleChirho {
                elements_chirho: vec![],
                span_chirho,
            }
        }
    }

    /// A bracketed list: an arithmetic sequence, a list comprehension, or a list literal. The
    /// ListExpr arm of `lower_expr_chirho`, extracted in row 641.
    pub(super) fn lower_list_expr_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> ExprChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        // Check if this is an arithmetic sequence (contains ..)
        let has_dotdot_chirho = children_chirho.iter().any(|c_chirho| {
            matches!(
                c_chirho.element_chirho,
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::DotDotChirho
            )
        });

        if has_dotdot_chirho {
            // Arithmetic sequence: collect expressions before and after ..
            // Forms: [a..], [a..b], [a,b..], [a,b..c]
            let mut exprs_before_chirho: Vec<ExprChirho> = Vec::new();
            let mut exprs_after_chirho: Vec<ExprChirho> = Vec::new();
            let mut past_dotdot_chirho = false;

            for child_chirho in &children_chirho {
                match child_chirho.element_chirho {
                    GreenElementChirho::TokenChirho(tok_chirho)
                        if tok_chirho.kind_chirho() == TokenKindChirho::DotDotChirho =>
                    {
                        past_dotdot_chirho = true;
                    }
                    GreenElementChirho::NodeChirho(_) => {
                        let expr_chirho = self.lower_expr_from_child_chirho(child_chirho);
                        if past_dotdot_chirho {
                            exprs_after_chirho.push(expr_chirho);
                        } else {
                            exprs_before_chirho.push(expr_chirho);
                        }
                    }
                    _ => {}
                }
            }

            let from_chirho = exprs_before_chirho
                .first()
                .cloned()
                .unwrap_or_else(|| self.placeholder_expr_chirho());
            let then_chirho = exprs_before_chirho.get(1).cloned();
            let to_chirho = exprs_after_chirho.first().cloned();

            ExprChirho::ArithSeqChirho {
                from_chirho: Box::new(from_chirho),
                then_chirho: then_chirho.map(Box::new),
                to_chirho: to_chirho.map(Box::new),
                span_chirho,
            }
        } else {
            // Check for list comprehension (contains |)
            let has_pipe_chirho = children_chirho.iter().any(|c_chirho| {
                matches!(
                    c_chirho.element_chirho,
                    GreenElementChirho::TokenChirho(tok_chirho)
                        if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                )
            });

            if has_pipe_chirho {
                // List comprehension: [body | qual1, qual2, ...]
                // Split children at PipeChirho: body is before, quals after.
                let mut body_expr_chirho: Option<ExprChirho> = None;
                let mut past_pipe_chirho = false;
                // Collect qualifier parts: each group separated by commas
                // is either a generator (contains LeftArrowChirho) or a guard.
                let mut current_qual_parts_chirho: Vec<QualPartChirho> = Vec::new();
                let mut current_qual_group_chirho: Vec<ComprehensionQualifierChirho> = Vec::new();
                let mut qual_groups_chirho: Vec<Vec<ComprehensionQualifierChirho>> = Vec::new();

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                                && !past_pipe_chirho =>
                        {
                            past_pipe_chirho = true;
                        }
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                                && past_pipe_chirho =>
                        {
                            if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                &mut current_qual_parts_chirho,
                                span_chirho,
                            ) {
                                current_qual_group_chirho.push(qual_chirho.into());
                            }
                            current_qual_parts_chirho.clear();
                            if !current_qual_group_chirho.is_empty() {
                                qual_groups_chirho
                                    .push(std::mem::take(&mut current_qual_group_chirho));
                            }
                        }
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho
                                && past_pipe_chirho =>
                        {
                            // Flush current qualifier
                            if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                &mut current_qual_parts_chirho,
                                span_chirho,
                            ) {
                                current_qual_group_chirho.push(qual_chirho.into());
                            }
                            current_qual_parts_chirho.clear();
                        }
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::LeftArrowChirho
                                && past_pipe_chirho =>
                        {
                            current_qual_parts_chirho.push(QualPartChirho::ArrowChirho);
                        }
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if past_pipe_chirho
                                && matches!(
                                    tok_chirho.kind_chirho(),
                                    TokenKindChirho::ConSymChirho
                                        | TokenKindChirho::QualifiedConSymChirho
                                ) =>
                        {
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
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if !past_pipe_chirho {
                                let expr_chirho =
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho);
                                body_expr_chirho = Some(expr_chirho);
                            } else if n_chirho.kind_chirho()
                                == SyntaxKindChirho::ComprehensionTransformChirho
                            {
                                if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                    &mut current_qual_parts_chirho,
                                    span_chirho,
                                ) {
                                    current_qual_group_chirho.push(qual_chirho.into());
                                }
                                current_qual_group_chirho.push(
                                    self.lower_comprehension_transform_chirho(
                                        n_chirho,
                                        child_chirho.start_chirho,
                                    ),
                                );
                            } else if n_chirho.kind_chirho() == SyntaxKindChirho::LetStmtChirho {
                                if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                    &mut current_qual_parts_chirho,
                                    span_chirho,
                                ) {
                                    current_qual_group_chirho.push(qual_chirho.into());
                                }
                                current_qual_parts_chirho.clear();
                                current_qual_group_chirho.push(
                                    StmtChirho::LetChirho {
                                        binds_chirho: self.lower_let_stmt_binds_chirho(
                                            n_chirho,
                                            child_chirho.start_chirho,
                                        ),
                                        span_chirho: self.span_chirho(
                                            child_chirho.start_chirho,
                                            child_chirho.end_chirho,
                                        ),
                                    }
                                    .into(),
                                );
                            } else if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                                current_qual_parts_chirho.push(QualPartChirho::PatChirho(
                                    self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                                ));
                            } else {
                                let expr_chirho =
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho);
                                current_qual_parts_chirho
                                    .push(QualPartChirho::ExprChirho(expr_chirho));
                            }
                        }
                        _ => {}
                    }
                }

                // Flush last qualifier
                if !current_qual_parts_chirho.is_empty()
                    && let Some(qual_chirho) =
                        self.flush_qual_parts_chirho(&mut current_qual_parts_chirho, span_chirho)
                {
                    current_qual_group_chirho.push(qual_chirho.into());
                }
                if !current_qual_group_chirho.is_empty() {
                    qual_groups_chirho.push(current_qual_group_chirho);
                }

                let body_chirho =
                    body_expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho());
                let (quals_chirho, parallel_quals_chirho) = if qual_groups_chirho.len() > 1 {
                    (Vec::new(), qual_groups_chirho)
                } else {
                    (qual_groups_chirho.pop().unwrap_or_default(), Vec::new())
                };

                ExprChirho::ListCompChirho {
                    body_chirho: Box::new(body_chirho),
                    quals_chirho,
                    parallel_quals_chirho,
                    span_chirho,
                }
            } else {
                let expr_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(_))
                    })
                    .collect();
                let elements_chirho: Vec<_> = expr_nodes_chirho
                    .iter()
                    .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                    .collect();
                ExprChirho::ListChirho {
                    elements_chirho,
                    span_chirho,
                }
            }
        }
    }

    pub(super) fn fold_qual_expr_parts_to_expr_chirho(
        &self,
        exprs_chirho: Vec<ExprChirho>,
        span_chirho: SpanChirho,
    ) -> Option<ExprChirho> {
        let mut exprs_iter_chirho = exprs_chirho.into_iter();
        let mut result_chirho = exprs_iter_chirho.next()?;
        for arg_chirho in exprs_iter_chirho {
            result_chirho = ExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(arg_chirho),
                span_chirho,
            };
        }
        Some(result_chirho)
    }

    fn lower_comprehension_transform_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ComprehensionQualifierChirho {
        let span_chirho = self.span_chirho(
            base_chirho,
            base_chirho + node_chirho.text_len_chirho(),
        );
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let group_chirho = children_chirho.iter().any(|child_chirho| matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::VarIdChirho && token_chirho.text_chirho() == "group"));
        let mut using_chirho = None;
        let mut by_chirho = None;
        for child_chirho in children_chirho {
            let GreenElementChirho::NodeChirho(payload_chirho) = child_chirho.element_chirho else {
                continue;
            };
            let target_chirho = match payload_chirho.kind_chirho() {
                SyntaxKindChirho::ComprehensionUsingChirho => &mut using_chirho,
                SyntaxKindChirho::ComprehensionByChirho => &mut by_chirho,
                _ => continue,
            };
            *target_chirho = self
                .semantic_children_chirho(payload_chirho, child_chirho.start_chirho)
                .iter()
                .find_map(|expression_chirho| {
                    let GreenElementChirho::NodeChirho(expression_node_chirho) =
                        expression_chirho.element_chirho
                    else {
                        return None;
                    };
                    is_expr_kind_chirho(expression_node_chirho.kind_chirho()).then(|| {
                        Box::new(self.lower_expr_chirho(
                            expression_node_chirho,
                            expression_chirho.start_chirho,
                        ))
                    })
                });
        }
        ComprehensionQualifierChirho::TransformChirho(ComprehensionTransformChirho {
            group_chirho,
            using_chirho,
            by_chirho,
            span_chirho,
        })
    }

    /// Convert accumulated qualifier parts into a `StmtChirho`.
    ///
    /// Parts are either expressions or a `<-` arrow marker. If an arrow
    /// is present, the qualifier is a generator (`pat <- expr`); otherwise
    /// it's a guard (plain expression).
    pub(super) fn flush_qual_parts_chirho(
        &self,
        parts_chirho: &mut Vec<QualPartChirho>,
        span_chirho: SpanChirho,
    ) -> Option<StmtChirho> {
        if parts_chirho.is_empty() {
            return None;
        }

        let mut parts_chirho = std::mem::take(parts_chirho);
        // Find the arrow position, if any
        let arrow_pos_chirho = parts_chirho
            .iter()
            .position(|p_chirho| matches!(p_chirho, QualPartChirho::ArrowChirho));

        if let Some(pos_chirho) = arrow_pos_chirho {
            // Generator: parts before arrow form the pattern (as expr),
            // parts after arrow form the source expression.
            let source_parts_chirho = parts_chirho.split_off(pos_chirho + 1);
            parts_chirho.pop(); // the actual arrow marker
            let pat_parts_chirho = parts_chirho;
            let pat_chirho = self
                .fold_qual_pat_parts_to_pat_chirho(pat_parts_chirho, span_chirho)
                .unwrap_or(PatChirho::WildcardChirho(span_chirho));

            let src_exprs_chirho: Vec<ExprChirho> = source_parts_chirho
                .into_iter()
                .filter_map(|p_chirho| match p_chirho {
                    QualPartChirho::ExprChirho(e_chirho) => Some(e_chirho),
                    _ => None,
                })
                .collect();

            let expr_chirho = self
                .fold_qual_expr_parts_to_expr_chirho(src_exprs_chirho, span_chirho)
                .unwrap_or_else(|| self.placeholder_expr_chirho());

            Some(StmtChirho::BindChirho {
                bind_chirho: None,
                fail_chirho: None,
                pat_chirho,
                expr_chirho,
                span_chirho,
            })
        } else {
            // Guard: single expression
            let exprs_chirho: Vec<ExprChirho> = parts_chirho
                .into_iter()
                .filter_map(|p_chirho| match p_chirho {
                    QualPartChirho::ExprChirho(e_chirho) => Some(e_chirho),
                    _ => None,
                })
                .collect();
            let expr_chirho =
                self.fold_qual_expr_parts_to_expr_chirho(exprs_chirho, span_chirho)?;
            Some(StmtChirho::expr_stmt_chirho(expr_chirho))
        }
    }
}

/// A part of a list comprehension qualifier, used during CST→AST lowering.
#[derive(Clone)]
#[allow(
    clippy::enum_variant_names,
    reason = "every variant carries the project's Chirho suffix"
)]
pub(super) enum QualPartChirho {
    /// An expression or pattern.
    ExprChirho(ExprChirho),
    /// A real pattern parsed on the left of `<-`.
    PatChirho(PatChirho),
    /// An infix constructor operator captured inside a generator pattern.
    PatOpChirho(NameChirho),
    /// The `<-` arrow token separating pattern from source.
    ArrowChirho,
}
