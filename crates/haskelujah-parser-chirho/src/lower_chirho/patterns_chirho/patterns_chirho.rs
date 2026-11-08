// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Patterns, and expressions re-read as patterns.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::bracketed_exprs_chirho::QualPartChirho;
use super::context_chirho::is_pat_kind_chirho;
use super::{ChildChirho, LowerCtxChirho, is_type_kind_chirho};

impl LowerCtxChirho {
    // -----------------------------------------------------------------------
    // Patterns
    // -----------------------------------------------------------------------

    pub(super) fn lower_pat_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> PatChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());

        match node_chirho.kind_chirho() {
            SyntaxKindChirho::SplicePatChirho => {
                let expr_chirho = self
                    .semantic_children_chirho(node_chirho, base_chirho)
                    .iter()
                    .find_map(|child_chirho| {
                        if let GreenElementChirho::NodeChirho(payload_chirho) =
                            child_chirho.element_chirho
                            && payload_chirho.kind_chirho().is_expr_chirho()
                        {
                            Some(Box::new(self.lower_expr_chirho(
                                payload_chirho,
                                child_chirho.start_chirho,
                            )))
                        } else {
                            None
                        }
                    });
                PatChirho::SpliceChirho {
                    expr_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::VarPatChirho => {
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                PatChirho::VarChirho(name_chirho)
            }
            SyntaxKindChirho::ConPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut con_name_chirho = None;
                let mut args_chirho = Vec::new();
                let mut type_args_chirho = Vec::new();
                let mut type_argument_pending_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::AtSignChirho {
                                type_argument_pending_chirho = true;
                            }

                            if con_name_chirho.is_none()
                                && (tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                                    || tok_chirho.kind_chirho()
                                        == TokenKindChirho::QualifiedConIdChirho)
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                con_name_chirho =
                                    Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho)
                            if type_argument_pending_chirho
                                && super::is_type_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            type_args_chirho.push(
                                haskelujah_ast_chirho::pat_chirho::ConstructorTypePatternChirho {
                                    ty_chirho: self
                                        .lower_type_chirho(n_chirho, child_chirho.start_chirho),
                                    checked_binder_chirho: None,
                                },
                            );
                            type_argument_pending_chirho = false;
                        }
                        GreenElementChirho::NodeChirho(n_chirho)
                            if is_pat_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            args_chirho
                                .push(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                        }
                        _ => {}
                    }
                }

                PatChirho::ConChirho {
                    con_chirho: con_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    type_args_chirho,
                    args_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::LitPatChirho => {
                let lit_chirho = self.lower_lit_chirho(node_chirho, base_chirho);
                PatChirho::LitChirho(lit_chirho)
            }
            SyntaxKindChirho::NegPatChirho => {
                // CST: NegPat = '-' apat   (where apat is typically a LitPat)
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut lit_chirho = None;
                for child_chirho in &children_chirho {
                    if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                        && (n_chirho.kind_chirho() == SyntaxKindChirho::LitPatChirho
                            || is_pat_kind_chirho(n_chirho.kind_chirho()))
                    {
                        let inner_lit_chirho =
                            self.lower_lit_chirho(n_chirho, child_chirho.start_chirho);
                        lit_chirho = Some(inner_lit_chirho);
                        break;
                    }
                }
                let lit_chirho = lit_chirho.unwrap_or(LitChirho::IntChirho(
                    0,
                    span_chirho,
                    Some(self.fresh_origin_chirho()),
                ));
                PatChirho::NegChirho {
                    lit_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::WildcardPatChirho => PatChirho::WildcardChirho(span_chirho),
            SyntaxKindChirho::AsPatChirho => {
                // CST: AsPatChirho = VarPatChirho { VarId } '@' apat
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut name_chirho = None;
                let mut inner_chirho = None;
                let mut saw_at_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::AtSignChirho {
                                saw_at_chirho = true;
                            } else if name_chirho.is_none()
                                && tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                name_chirho =
                                    Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if !saw_at_chirho
                                && name_chirho.is_none()
                                && n_chirho.kind_chirho() == SyntaxKindChirho::VarPatChirho
                            {
                                // Extract the variable name from the
                                // VarPatChirho wrapper node.
                                name_chirho = Some(self.extract_name_from_node_chirho(
                                    n_chirho,
                                    child_chirho.start_chirho,
                                ));
                            } else if saw_at_chirho
                                && inner_chirho.is_none()
                                && is_pat_kind_chirho(n_chirho.kind_chirho())
                            {
                                inner_chirho = Some(
                                    self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }

                PatChirho::AsChirho {
                    name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    pattern_chirho: Box::new(
                        inner_chirho.unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
                    ),
                    span_chirho,
                }
            }
            SyntaxKindChirho::LazyPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let inner_chirho = children_chirho
                    .iter()
                    .find(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_pat_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .map(|c_chirho| self.lower_pat_from_child_chirho(c_chirho))
                    .unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO));
                PatChirho::LazyChirho {
                    inner_chirho: Box::new(inner_chirho),
                    span_chirho,
                }
            }
            SyntaxKindChirho::BangPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let inner_chirho = children_chirho
                    .iter()
                    .find(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_pat_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .map(|c_chirho| self.lower_pat_from_child_chirho(c_chirho))
                    .unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO));
                PatChirho::BangChirho {
                    inner_chirho: Box::new(inner_chirho),
                    span_chirho,
                }
            }
            SyntaxKindChirho::ParenPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

                // Check for type-annotated pattern: (pat :: Type)
                // The CST parser produces flat tokens for the `:: Type` part.
                let has_double_colon_chirho = children_chirho.iter().any(|c_chirho| {
                    matches!(
                        c_chirho.element_chirho,
                        GreenElementChirho::TokenChirho(t_chirho)
                            if t_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
                    )
                });

                if has_double_colon_chirho {
                    // Type-annotated pattern: collect pat before :: and type after ::
                    let mut inner_pat_chirho = None;
                    let mut saw_dc_chirho = false;
                    let mut type_children_chirho: Vec<&ChildChirho> = Vec::new();

                    for child_chirho in &children_chirho {
                        match child_chirho.element_chirho {
                            GreenElementChirho::TokenChirho(tok_chirho) => {
                                if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                                    saw_dc_chirho = true;
                                } else if saw_dc_chirho {
                                    // Token after :: — part of the type
                                    type_children_chirho.push(child_chirho);
                                }
                                // Skip parens/etc before ::
                            }
                            GreenElementChirho::NodeChirho(n_chirho) => {
                                if !saw_dc_chirho && is_pat_kind_chirho(n_chirho.kind_chirho()) {
                                    if inner_pat_chirho.is_none() {
                                        inner_pat_chirho =
                                            Some(self.lower_pat_chirho(
                                                n_chirho,
                                                child_chirho.start_chirho,
                                            ));
                                    }
                                } else if saw_dc_chirho {
                                    type_children_chirho.push(child_chirho);
                                }
                            }
                        }
                    }

                    let pat_chirho =
                        inner_pat_chirho.unwrap_or(PatChirho::WildcardChirho(span_chirho));
                    let ty_chirho = if type_children_chirho.is_empty() {
                        self.placeholder_type_chirho()
                    } else {
                        // Try structured node first, fall back to flat reconstruction
                        let first_chirho = type_children_chirho[0];
                        if let GreenElementChirho::NodeChirho(n_chirho) =
                            first_chirho.element_chirho
                        {
                            if is_type_kind_chirho(n_chirho.kind_chirho()) {
                                self.lower_type_chirho(n_chirho, first_chirho.start_chirho)
                            } else {
                                self.type_from_flat_children_chirho(
                                    &type_children_chirho,
                                    span_chirho,
                                )
                            }
                        } else {
                            self.type_from_flat_children_chirho(&type_children_chirho, span_chirho)
                        }
                    };

                    return PatChirho::TypeAnnotChirho {
                        pat_chirho: Box::new(pat_chirho),
                        ty_chirho,
                        span_chirho,
                    };
                }

                let pat_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_pat_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();
                if pat_nodes_chirho.len() == 1 {
                    let inner_chirho = self.lower_pat_from_child_chirho(pat_nodes_chirho[0]);
                    PatChirho::ParenChirho {
                        inner_chirho: Box::new(inner_chirho),
                        span_chirho,
                    }
                } else if pat_nodes_chirho.len() > 1 {
                    let elements_chirho: Vec<_> = pat_nodes_chirho
                        .iter()
                        .map(|c_chirho| self.lower_pat_from_child_chirho(c_chirho))
                        .collect();
                    PatChirho::TupleChirho {
                        elements_chirho,
                        span_chirho,
                    }
                } else {
                    let mut paren_op_pat_chirho = None;
                    for child_chirho in &children_chirho {
                        if let GreenElementChirho::TokenChirho(tok_chirho) =
                            child_chirho.element_chirho
                        {
                            let child_span_chirho = self
                                .span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                            match tok_chirho.kind_chirho() {
                                TokenKindChirho::VarSymChirho
                                | TokenKindChirho::QualifiedVarSymChirho => {
                                    paren_op_pat_chirho = Some(PatChirho::VarChirho(
                                        self.name_from_token_chirho(tok_chirho, child_span_chirho),
                                    ));
                                    break;
                                }
                                TokenKindChirho::ConSymChirho
                                | TokenKindChirho::QualifiedConSymChirho => {
                                    paren_op_pat_chirho = Some(PatChirho::ConChirho {
                                        type_args_chirho: Vec::new(),
                                        con_chirho: self
                                            .name_from_token_chirho(tok_chirho, child_span_chirho),
                                        args_chirho: vec![],
                                        span_chirho: child_span_chirho,
                                    });
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    paren_op_pat_chirho.unwrap_or(PatChirho::TupleChirho {
                        elements_chirho: vec![],
                        span_chirho,
                    })
                }
            }
            SyntaxKindChirho::ListPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let elements_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_pat_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .map(|c_chirho| self.lower_pat_from_child_chirho(c_chirho))
                    .collect();
                PatChirho::ListChirho {
                    elements_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::InfixConPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut left_chirho = None;
                let mut op_chirho = None;
                let mut right_chirho = None;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if op_chirho.is_none()
                                && matches!(
                                    tok_chirho.kind_chirho(),
                                    TokenKindChirho::ConSymChirho
                                        | TokenKindChirho::QualifiedConSymChirho
                                )
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                op_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho)
                            if is_pat_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            let p_chirho =
                                self.lower_pat_chirho(n_chirho, child_chirho.start_chirho);
                            if left_chirho.is_none() {
                                left_chirho = Some(p_chirho);
                            } else {
                                right_chirho = Some(p_chirho);
                            }
                        }
                        _ => {}
                    }
                }

                PatChirho::InfixConChirho {
                    left_chirho: Box::new(
                        left_chirho.unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
                    ),
                    op_chirho: op_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    right_chirho: Box::new(
                        right_chirho.unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
                    ),
                    span_chirho,
                }
            }
            SyntaxKindChirho::RecordPatChirho => {
                self.lower_record_pat_chirho(node_chirho, base_chirho)
            }
            SyntaxKindChirho::SigPatChirho => {
                // CST: SigPat = pat '::' type  (ScopedTypeVariables)
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut inner_pat_chirho = None;
                let mut sig_type_chirho = None;
                let mut saw_double_colon_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                                saw_double_colon_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if !saw_double_colon_chirho
                                && is_pat_kind_chirho(n_chirho.kind_chirho())
                            {
                                inner_pat_chirho = Some(
                                    self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            } else if saw_double_colon_chirho
                                && is_type_kind_chirho(n_chirho.kind_chirho())
                            {
                                sig_type_chirho = Some(
                                    self.lower_type_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }

                let inner_chirho =
                    inner_pat_chirho.unwrap_or(PatChirho::WildcardChirho(span_chirho));
                let ty_chirho = sig_type_chirho.unwrap_or_else(|| self.placeholder_type_chirho());
                PatChirho::TypeAnnotChirho {
                    pat_chirho: Box::new(inner_chirho),
                    ty_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::ViewPatChirho => {
                // CST: ViewPat = expr '->' pat  (inside parens)
                // The parser already validated the arrow; children in order:
                // node(expr), token(->), node(pat)
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut expr_chirho = None;
                let mut pat_inner_chirho = None;
                let mut saw_arrow_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                                || tok_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho
                            {
                                // In a ViewPatChirho node, any arrow-like token is the view arrow
                                saw_arrow_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if saw_arrow_chirho
                                && pat_inner_chirho.is_none()
                                && is_pat_kind_chirho(n_chirho.kind_chirho())
                            {
                                pat_inner_chirho = Some(
                                    self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            } else if !saw_arrow_chirho && expr_chirho.is_none() {
                                // The expression part before ->
                                expr_chirho = Some(
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }

                PatChirho::ViewChirho {
                    expr_chirho: Box::new(
                        expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
                    ),
                    pat_chirho: Box::new(
                        pat_inner_chirho
                            .unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
                    ),
                    span_chirho,
                }
            }
            _ => {
                // Fallback: try as variable pattern
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                PatChirho::VarChirho(name_chirho)
            }
        }
    }

    fn lower_pat_from_child_chirho(&self, child_chirho: &ChildChirho<'_>) -> PatChirho {
        if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
            self.lower_pat_chirho(n_chirho, child_chirho.start_chirho)
        } else {
            PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)
        }
    }

    pub(super) fn fold_qual_pat_parts_to_pat_chirho(
        &self,
        parts_chirho: Vec<QualPartChirho>,
        span_chirho: SpanChirho,
    ) -> Option<PatChirho> {
        let mut pats_chirho = Vec::new();
        let mut ops_chirho = Vec::new();

        for part_chirho in parts_chirho {
            match part_chirho {
                QualPartChirho::PatChirho(pat_chirho) => pats_chirho.push(pat_chirho),
                QualPartChirho::ExprChirho(expr_chirho) => {
                    pats_chirho.push(Self::expr_to_pat_chirho(expr_chirho));
                }
                QualPartChirho::PatOpChirho(op_chirho) => ops_chirho.push(op_chirho),
                QualPartChirho::ArrowChirho => {}
            }
        }

        if pats_chirho.is_empty() {
            return None;
        }

        if pats_chirho.len() == 2 && ops_chirho.len() == 1 {
            return Some(PatChirho::InfixConChirho {
                left_chirho: Box::new(pats_chirho.remove(0)),
                op_chirho: ops_chirho.remove(0),
                right_chirho: Box::new(pats_chirho.remove(0)),
                span_chirho,
            });
        }

        let mut merged_pat_chirho = pats_chirho.remove(0);
        if let PatChirho::ConChirho { args_chirho, .. } = &mut merged_pat_chirho {
            args_chirho.extend(pats_chirho);
        }
        Some(merged_pat_chirho)
    }

    /// Convert a parsed expression back to a pattern.
    /// Used for list comprehension generators where the pattern
    /// was parsed as an expression by the CST parser.
    pub(super) fn expr_to_pat_chirho(expr_chirho: ExprChirho) -> PatChirho {
        let span_chirho = expr_chirho.span_chirho();
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if text_chirho == "_" {
                    PatChirho::WildcardChirho(span_chirho)
                } else if text_chirho
                    .chars()
                    .next()
                    .is_some_and(|c_chirho| c_chirho.is_uppercase())
                {
                    PatChirho::ConChirho {
                        type_args_chirho: Vec::new(),
                        con_chirho: name_chirho.without_origin_chirho(),
                        args_chirho: vec![],
                        span_chirho,
                    }
                } else {
                    // Re-read as a binder; the same name ceases to be an occurrence.
                    PatChirho::VarChirho(name_chirho.without_origin_chirho())
                }
            }
            ExprChirho::LitChirho(lit_chirho) => PatChirho::LitChirho(lit_chirho),
            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => PatChirho::TupleChirho {
                elements_chirho: elements_chirho
                    .into_iter()
                    .map(Self::expr_to_pat_chirho)
                    .collect(),
                span_chirho,
            },
            ExprChirho::ListChirho {
                elements_chirho, ..
            } => PatChirho::ListChirho {
                elements_chirho: elements_chirho
                    .into_iter()
                    .map(Self::expr_to_pat_chirho)
                    .collect(),
                span_chirho,
            },
            ExprChirho::ParenChirho { inner_chirho, .. } => PatChirho::ParenChirho {
                inner_chirho: Box::new(Self::expr_to_pat_chirho(*inner_chirho)),
                span_chirho,
            },
            ExprChirho::AppChirho { .. } => {
                let mut app_args_chirho: Vec<PatChirho> = Vec::new();
                let mut head_expr_chirho = expr_chirho;
                while let ExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } = head_expr_chirho
                {
                    app_args_chirho.push(Self::expr_to_pat_chirho(*arg_chirho));
                    head_expr_chirho = *fun_chirho;
                }
                app_args_chirho.reverse();
                match head_expr_chirho {
                    ExprChirho::ConChirho(con_chirho) => PatChirho::ConChirho {
                        type_args_chirho: Vec::new(),
                        con_chirho,
                        args_chirho: app_args_chirho,
                        span_chirho,
                    },
                    ExprChirho::VarChirho(name_chirho)
                        if name_chirho
                            .text_chirho()
                            .chars()
                            .next()
                            .is_some_and(|c_chirho| c_chirho.is_uppercase()) =>
                    {
                        PatChirho::ConChirho {
                            type_args_chirho: Vec::new(),
                            con_chirho: name_chirho.without_origin_chirho(),
                            args_chirho: app_args_chirho,
                            span_chirho,
                        }
                    }
                    _ => PatChirho::WildcardChirho(span_chirho),
                }
            }
            ExprChirho::SpliceChirho { expr_chirho, .. } => PatChirho::SpliceChirho {
                // Moving the actual payload retains occurrences without an
                // unbounded recursive clone before the elaboration boundary.
                expr_chirho: Some(expr_chirho),
                span_chirho,
            },
            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                ..
            } => PatChirho::InfixConChirho {
                left_chirho: Box::new(Self::expr_to_pat_chirho(*left_chirho)),
                op_chirho: op_chirho.without_origin_chirho(),
                right_chirho: Box::new(Self::expr_to_pat_chirho(*right_chirho)),
                span_chirho,
            },
            _ => PatChirho::WildcardChirho(span_chirho),
        }
    }
}
