// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expressions: the dispatch over every expression form.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, GuardedExprChirho, LocalBindChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::bindings_chirho::merge_local_fun_binds_chirho;
use super::context_chirho::{is_expr_kind_chirho, is_pat_kind_chirho};
use super::{ChildChirho, LowerCtxChirho};
use crate::rec_stmt_chirho::transform_recursive_do_chirho;

impl LowerCtxChirho {
    // -----------------------------------------------------------------------
    // Expressions
    // -----------------------------------------------------------------------

    pub(super) fn lower_expr_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ExprChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());

        match node_chirho.kind_chirho() {
            SyntaxKindChirho::NameExprChirho => {
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                // Check for record construction: Con { f1 = e1, ... }. Empty
                // braces (`MkT {}`) are a record construction with no fields —
                // legal for lazy fields — not a bare constructor reference.
                let has_braces_chirho = children_chirho.iter().any(|c_chirho| {
                    matches!(
                        c_chirho.element_chirho,
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::LeftBraceChirho
                    )
                });
                let has_fields_chirho = has_braces_chirho
                    || children_chirho.iter().any(|c_chirho| {
                        matches!(
                            c_chirho.element_chirho,
                            GreenElementChirho::NodeChirho(n_chirho)
                                if n_chirho.kind_chirho()
                                    == SyntaxKindChirho::FieldAssignChirho
                        )
                    });
                if has_fields_chirho {
                    let fields_chirho = children_chirho
                        .iter()
                        .filter_map(|c_chirho| {
                            if let GreenElementChirho::NodeChirho(n_chirho) =
                                c_chirho.element_chirho
                                && n_chirho.kind_chirho()
                                    == SyntaxKindChirho::FieldAssignChirho
                                {
                                    // Skip FieldAssign nodes that are just DotDot wildcards
                                    let is_dotdot_chirho = n_chirho.children_chirho().iter().any(|gc_chirho| {
                                        matches!(
                                            gc_chirho,
                                            GreenElementChirho::TokenChirho(t_chirho)
                                                if t_chirho.kind_chirho() == TokenKindChirho::DotDotChirho
                                        )
                                    });
                                    if is_dotdot_chirho {
                                        return None;
                                    }
                                    return Some(self.lower_field_assign_chirho(
                                        n_chirho,
                                        c_chirho.start_chirho,
                                    ));
                                }
                            None
                        })
                        .collect();
                    // Detect `..` wildcard (RecordWildCards)
                    // Check both direct token children and inside FieldAssign nodes
                    let has_wildcard_chirho = children_chirho.iter().any(|c_chirho| match c_chirho
                        .element_chirho
                    {
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::DotDotChirho =>
                        {
                            true
                        }
                        GreenElementChirho::NodeChirho(n_chirho)
                            if n_chirho.kind_chirho() == SyntaxKindChirho::FieldAssignChirho =>
                        {
                            n_chirho.children_chirho().iter().any(|gc_chirho| {
                                matches!(
                                    gc_chirho,
                                    GreenElementChirho::TokenChirho(t_chirho)
                                        if t_chirho.kind_chirho() == TokenKindChirho::DotDotChirho
                                )
                            })
                        }
                        _ => false,
                    });
                    ExprChirho::RecordConChirho {
                        con_chirho: self.reference_name_chirho(name_chirho),
                        fields_chirho,
                        has_wildcard_chirho,
                        span_chirho,
                    }
                } else if name_chirho
                    .text_chirho()
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_uppercase())
                {
                    ExprChirho::ConChirho(self.reference_name_chirho(name_chirho))
                } else {
                    self.reference_expr_chirho(name_chirho)
                }
            }
            SyntaxKindChirho::LiteralExprChirho => {
                let lit_chirho = self.lower_lit_chirho(node_chirho, base_chirho);
                ExprChirho::LitChirho(lit_chirho)
            }
            SyntaxKindChirho::QuotedNameExprChirho => {
                self.lower_quoted_name_expr_chirho(node_chirho, base_chirho)
            }
            SyntaxKindChirho::TypeAppExprChirho => {
                // TypeApplications: expr @Type
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let node_children_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(_))
                    })
                    .collect();
                if node_children_chirho.is_empty() {
                    return self.placeholder_expr_chirho();
                }
                // First node child is the expression, last is the type
                let expr_chirho = self.lower_expr_from_child_chirho(node_children_chirho[0]);
                let ty_chirho = if node_children_chirho.len() >= 2 {
                    self.lower_type_from_child_chirho(
                        node_children_chirho[node_children_chirho.len() - 1],
                    )
                } else {
                    self.placeholder_type_chirho()
                };
                ExprChirho::TypeAppChirho {
                    expr_chirho: Box::new(expr_chirho),
                    ty_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::AppExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let expr_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(_))
                    })
                    .collect();
                if expr_nodes_chirho.is_empty() {
                    return self.placeholder_expr_chirho();
                }
                let mut result_chirho = self.lower_expr_from_child_chirho(expr_nodes_chirho[0]);
                for ec_chirho in &expr_nodes_chirho[1..] {
                    let arg_chirho = self.lower_expr_from_child_chirho(ec_chirho);
                    result_chirho = ExprChirho::AppChirho {
                        fun_chirho: Box::new(result_chirho),
                        arg_chirho: Box::new(arg_chirho),
                        span_chirho,
                    };
                }
                result_chirho
            }
            SyntaxKindChirho::InfixExprChirho => {
                // Flat list: expr op expr op expr ...
                // Resolve with operator precedence and associativity.
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut exprs_chirho: Vec<ExprChirho> = Vec::new();
                let mut ops_chirho: Vec<NameChirho> = Vec::new();

                let mut in_backtick_chirho = false;
                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            exprs_chirho
                                .push(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                        }
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::BacktickChirho {
                                in_backtick_chirho = !in_backtick_chirho;
                            } else if in_backtick_chirho
                                && (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                                    || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                                    || tok_chirho.kind_chirho()
                                        == TokenKindChirho::QualifiedVarIdChirho
                                    || tok_chirho.kind_chirho()
                                        == TokenKindChirho::QualifiedConIdChirho
                                    || tok_chirho.kind_chirho()
                                        == TokenKindChirho::QualifiedVarSymChirho
                                    || tok_chirho.kind_chirho()
                                        == TokenKindChirho::QualifiedConSymChirho)
                            {
                                // Backtick-enclosed name used as infix operator:
                                // `div`, `mod`, `elem`, etc.
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                ops_chirho.push(self.reference_name_chirho(
                                    self.name_from_token_chirho(tok_chirho, s_chirho),
                                ));
                            } else if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                                || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                                || tok_chirho.kind_chirho()
                                    == TokenKindChirho::QualifiedVarSymChirho
                                || tok_chirho.kind_chirho()
                                    == TokenKindChirho::QualifiedConSymChirho
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                ops_chirho.push(self.reference_name_chirho(
                                    self.name_from_token_chirho(tok_chirho, s_chirho),
                                ));
                            }
                        }
                    }
                }

                if exprs_chirho.len() < 2 {
                    return exprs_chirho
                        .into_iter()
                        .next()
                        .unwrap_or_else(|| self.placeholder_expr_chirho());
                }

                self.resolve_infix_precedence_chirho(exprs_chirho, ops_chirho, span_chirho)
            }
            SyntaxKindChirho::LambdaExprChirho => {
                // workflow: language-features-chirho/rank-n-visible-type-application-chirho
                enum LambdaBinderChirho {
                    TermChirho(PatChirho),
                    TypeChirho(NameChirho),
                }

                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut binders_chirho = Vec::new();
                let mut body_chirho = None;
                let mut saw_arrow_chirho = false;
                let mut has_type_binder_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho {
                                saw_arrow_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if !saw_arrow_chirho
                                && n_chirho.kind_chirho()
                                    == SyntaxKindChirho::TypeLambdaBinderChirho
                            {
                                has_type_binder_chirho = true;
                                binders_chirho.push(LambdaBinderChirho::TypeChirho(
                                    self.extract_type_lambda_binder_name_chirho(
                                        n_chirho,
                                        child_chirho.start_chirho,
                                    ),
                                ));
                            } else if !saw_arrow_chirho
                                && is_pat_kind_chirho(n_chirho.kind_chirho())
                            {
                                binders_chirho.push(LambdaBinderChirho::TermChirho(
                                    self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                                ));
                            } else if !saw_arrow_chirho
                                && is_expr_kind_chirho(n_chirho.kind_chirho())
                            {
                                let pat_expr_chirho =
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho);
                                binders_chirho.push(LambdaBinderChirho::TermChirho(
                                    Self::expr_to_pat_chirho(&pat_expr_chirho),
                                ));
                            } else if saw_arrow_chirho && body_chirho.is_none() {
                                body_chirho = Some(
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }

                let body_chirho = body_chirho.unwrap_or_else(|| self.placeholder_expr_chirho());
                if !has_type_binder_chirho {
                    let pats_chirho = binders_chirho
                        .into_iter()
                        .filter_map(|binder_chirho| match binder_chirho {
                            LambdaBinderChirho::TermChirho(pat_chirho) => Some(pat_chirho),
                            LambdaBinderChirho::TypeChirho(_) => None,
                        })
                        .collect();
                    ExprChirho::LamChirho {
                        pats_chirho,
                        body_chirho: Box::new(body_chirho),
                        span_chirho,
                    }
                } else {
                    binders_chirho.into_iter().rev().fold(
                        body_chirho,
                        |body_chirho, binder_chirho| match binder_chirho {
                            LambdaBinderChirho::TermChirho(pat_chirho) => ExprChirho::LamChirho {
                                pats_chirho: vec![pat_chirho],
                                body_chirho: Box::new(body_chirho),
                                span_chirho,
                            },
                            LambdaBinderChirho::TypeChirho(binder_chirho) => {
                                ExprChirho::TypeLamChirho {
                                    binder_chirho,
                                    body_chirho: Box::new(body_chirho),
                                    span_chirho,
                                }
                            }
                        },
                    )
                }
            }
            SyntaxKindChirho::LambdaCaseExprChirho => {
                // Desugar: \case { alts } → \$lc_chirho -> case $lc_chirho of { alts }
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut alts_chirho = Vec::new();

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                        if n_chirho.kind_chirho() == SyntaxKindChirho::LayoutBlockChirho {
                            // Descend into the layout block to find CaseAltChirho nodes
                            let block_children_chirho =
                                self.semantic_children_chirho(n_chirho, child_chirho.start_chirho);
                            for bc_chirho in &block_children_chirho {
                                if let GreenElementChirho::NodeChirho(alt_n_chirho) =
                                    bc_chirho.element_chirho
                                    && alt_n_chirho.kind_chirho() == SyntaxKindChirho::CaseAltChirho
                                {
                                    // Filter phantom alts (no arrow)
                                    let has_arrow_chirho = self
                                        .semantic_children_chirho(
                                            alt_n_chirho,
                                            bc_chirho.start_chirho,
                                        )
                                        .iter()
                                        .any(|c_chirho| {
                                            matches!(
                                                c_chirho.element_chirho,
                                                GreenElementChirho::TokenChirho(t_chirho)
                                                if t_chirho.kind_chirho()
                                                    == TokenKindChirho::RightArrowChirho
                                            )
                                        });
                                    if has_arrow_chirho {
                                        alts_chirho.push(self.lower_case_alt_chirho(
                                            alt_n_chirho,
                                            bc_chirho.start_chirho,
                                        ));
                                    }
                                }
                            }
                        } else if n_chirho.kind_chirho() == SyntaxKindChirho::CaseAltChirho {
                            let has_arrow_chirho = self
                                .semantic_children_chirho(n_chirho, child_chirho.start_chirho)
                                .iter()
                                .any(|c_chirho| {
                                    matches!(
                                        c_chirho.element_chirho,
                                        GreenElementChirho::TokenChirho(t_chirho)
                                        if t_chirho.kind_chirho()
                                            == TokenKindChirho::RightArrowChirho
                                    )
                                });
                            if has_arrow_chirho {
                                alts_chirho.push(
                                    self.lower_case_alt_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }

                let fresh_name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    "$lc_chirho",
                    span_chirho,
                ));
                let fresh_var_chirho = self.reference_expr_chirho(fresh_name_chirho.clone());
                let fresh_pat_chirho = PatChirho::VarChirho(fresh_name_chirho);

                let case_expr_chirho = ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(fresh_var_chirho),
                    alts_chirho,
                    span_chirho,
                };

                ExprChirho::LamChirho {
                    pats_chirho: vec![fresh_pat_chirho],
                    body_chirho: Box::new(case_expr_chirho),
                    span_chirho,
                }
            }
            SyntaxKindChirho::IfExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let expr_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(_))
                    })
                    .collect();

                let cond_chirho = expr_nodes_chirho
                    .first()
                    .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
                let then_chirho = expr_nodes_chirho
                    .get(1)
                    .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
                let else_chirho = expr_nodes_chirho
                    .get(2)
                    .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                    .unwrap_or_else(|| self.placeholder_expr_chirho());

                ExprChirho::IfChirho {
                    cond_chirho: Box::new(cond_chirho),
                    then_chirho: Box::new(then_chirho),
                    else_chirho: Box::new(else_chirho),
                    span_chirho,
                }
            }
            // MultiWayIf: if | quals -> e | quals -> e, each arm a guard node
            // whose qualifiers stay qualifiers.
            // workflow: language-features-chirho/pattern-matching-chirho
            SyntaxKindChirho::MultiWayIfExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let arms_chirho: Vec<GuardedExprChirho> = children_chirho
                    .iter()
                    .filter_map(|child_chirho| match child_chirho.element_chirho {
                        GreenElementChirho::NodeChirho(n_chirho)
                            if n_chirho.kind_chirho() == SyntaxKindChirho::GuardChirho =>
                        {
                            self.lower_guard_chirho(n_chirho, child_chirho.start_chirho)
                        }
                        _ => None,
                    })
                    .collect();
                if arms_chirho.is_empty() {
                    self.placeholder_expr_chirho()
                } else {
                    ExprChirho::MultiWayIfChirho {
                        arms_chirho,
                        span_chirho,
                    }
                }
            }
            SyntaxKindChirho::CaseExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut scrutinee_chirho = None;
                let mut alts_chirho = Vec::new();
                let mut saw_of_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::OfKeywordChirho {
                                saw_of_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if !saw_of_chirho && scrutinee_chirho.is_none() {
                                scrutinee_chirho = Some(
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            } else if saw_of_chirho
                                && n_chirho.kind_chirho() == SyntaxKindChirho::CaseAltChirho
                            {
                                // Check that this CaseAlt actually contains
                                // an arrow; phantom alts emitted by the
                                // layout engine for stray tokens have no
                                // arrow and would produce placeholder RHS.
                                let has_arrow_chirho = self
                                    .semantic_children_chirho(n_chirho, child_chirho.start_chirho)
                                    .iter()
                                    .any(|c_chirho| {
                                        matches!(
                                            c_chirho.element_chirho,
                                            GreenElementChirho::TokenChirho(t_chirho)
                                            if t_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho
                                        )
                                    });
                                if has_arrow_chirho {
                                    alts_chirho.push(self.lower_case_alt_chirho(
                                        n_chirho,
                                        child_chirho.start_chirho,
                                    ));
                                }
                            }
                        }
                    }
                }

                ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(
                        scrutinee_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
                    ),
                    alts_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::DoExprChirho => {
                // QualifiedDo's module prefix is semantic input to Core method
                // selection, so preserve it instead of reconstructing source.
                // workflow: language-features-chirho/qualified-do-chirho
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let is_mdo_chirho = children_chirho.iter().any(|child_chirho| {
                    matches!(
                    child_chirho.element_chirho,
                    GreenElementChirho::TokenChirho(token_chirho)
                        if token_chirho.kind_chirho() == TokenKindChirho::DoKeywordChirho
                            && token_chirho.text_chirho() == "mdo"
                    )
                });
                let qualifier_chirho = children_chirho.iter().find_map(|child_chirho| {
                    let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho
                    else {
                        return None;
                    };
                    if !matches!(
                        token_chirho.kind_chirho(),
                        TokenKindChirho::DoKeywordChirho | TokenKindChirho::QualifiedVarIdChirho
                    ) {
                        return None;
                    }
                    token_chirho.text_chirho().rsplit_once('.').and_then(
                        |(qualifier_chirho, local_chirho)| {
                            (local_chirho == "do" && !qualifier_chirho.is_empty())
                                .then(|| qualifier_chirho.to_string())
                        },
                    )
                });
                let segments_chirho = self.lower_do_segments_chirho(node_chirho, base_chirho);
                let mut stmts_chirho = transform_recursive_do_chirho(
                    segments_chirho,
                    is_mdo_chirho,
                    span_chirho,
                    &mut self.origin_supply_chirho.borrow_mut(),
                );
                // After the recursive-do knot, so the statements that get selections
                // are the ones that survive into the block.
                self.select_do_operations_chirho(&mut stmts_chirho, qualifier_chirho.as_deref());

                ExprChirho::DoChirho {
                    qualifier_chirho,
                    stmts_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::LetExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut binds_chirho: Vec<LocalBindChirho> = Vec::new();
                let mut body_chirho = None;
                let mut past_in_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::InKeywordChirho =>
                        {
                            past_in_chirho = true;
                        }
                        GreenElementChirho::NodeChirho(n_chirho) if !past_in_chirho => {
                            // Before "in": collect let-bindings
                            if n_chirho.kind_chirho() == SyntaxKindChirho::FunBindChirho {
                                let child_span_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
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
                                let child_span_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                let pb_chirho = self.lower_pat_bind_local_chirho(
                                    n_chirho,
                                    child_chirho.start_chirho,
                                    child_span_chirho,
                                );
                                binds_chirho.push(pb_chirho);
                            } else if n_chirho.kind_chirho() == SyntaxKindChirho::TypeSigDeclChirho
                            {
                                let child_span_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
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
                        GreenElementChirho::NodeChirho(n_chirho) if past_in_chirho => {
                            // After "in": the body expression
                            if body_chirho.is_none() {
                                if is_expr_kind_chirho(n_chirho.kind_chirho()) {
                                    body_chirho = Some(
                                        self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                    );
                                } else if n_chirho.kind_chirho() == SyntaxKindChirho::FunBindChirho
                                {
                                    // Parser may wrap body in FunBind > Match;
                                    // extract the expression from inside the match
                                    body_chirho = Some(self.lower_let_body_from_funbind_chirho(
                                        n_chirho,
                                        child_chirho.start_chirho,
                                    ));
                                }
                            }
                        }
                        _ => {}
                    }
                }

                let binds_chirho = merge_local_fun_binds_chirho(binds_chirho);
                ExprChirho::LetChirho {
                    binds_chirho,
                    body_chirho: Box::new(
                        body_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
                    ),
                    span_chirho,
                }
            }
            SyntaxKindChirho::ParenExprChirho => {
                self.lower_paren_expr_chirho(node_chirho, base_chirho, span_chirho)
            }
            SyntaxKindChirho::LeftSectionExprChirho | SyntaxKindChirho::RightSectionExprChirho => {
                // These are unused — sections are handled inside ParenExprChirho
                self.placeholder_expr_chirho()
            }
            SyntaxKindChirho::ListExprChirho => {
                self.lower_list_expr_chirho(node_chirho, base_chirho, span_chirho)
            }
            SyntaxKindChirho::NegateExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let expr_node_chirho = children_chirho.iter().find(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(_))
                });
                let inner_chirho = expr_node_chirho
                    .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
                let inner_chirho =
                    match expr_node_chirho.map(|child_chirho| child_chirho.element_chirho) {
                        Some(GreenElementChirho::NodeChirho(inner_node_chirho)) => self
                            .preserve_minimum_word_operand_chirho(inner_node_chirho, inner_chirho),
                        _ => inner_chirho,
                    };
                ExprChirho::NegChirho {
                    expr_chirho: Box::new(inner_chirho),
                    span_chirho,
                }
            }
            SyntaxKindChirho::TypeAnnotExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut expr_chirho = None;
                let mut ty_chirho = None;
                let mut saw_colon_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                                saw_colon_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if !saw_colon_chirho && expr_chirho.is_none() {
                                expr_chirho = Some(
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            } else if saw_colon_chirho && ty_chirho.is_none() {
                                ty_chirho = Some(
                                    self.lower_type_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }

                ExprChirho::AnnChirho {
                    expr_chirho: Box::new(
                        expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
                    ),
                    ty_chirho: ty_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
                    span_chirho,
                }
            }
            SyntaxKindChirho::RecordUpdateExprChirho => {
                // expr { f1 = e1, f2 = e2 }
                // Children: expression node(s), FieldAssignChirho nodes, braces/commas
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut base_expr_chirho = None;
                let mut fields_chirho = Vec::new();

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                        if n_chirho.kind_chirho() == SyntaxKindChirho::FieldAssignChirho {
                            fields_chirho.push(
                                self.lower_field_assign_chirho(n_chirho, child_chirho.start_chirho),
                            );
                        } else if base_expr_chirho.is_none()
                            && is_expr_kind_chirho(n_chirho.kind_chirho())
                        {
                            base_expr_chirho =
                                Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                        }
                    }
                }

                ExprChirho::RecordUpdateChirho {
                    expr_chirho: Box::new(
                        base_expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
                    ),
                    fields_chirho,
                    span_chirho,
                }
            }
            // Template Haskell splice expression: $name or $(expr)
            SyntaxKindChirho::SpliceExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                // Find the first expression child (either a NameExprChirho for $name
                // or the parsed expression for $(expr))
                let inner_chirho = children_chirho
                    .iter()
                    .find_map(|c_chirho| {
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                            Some(self.lower_expr_chirho(n_chirho, c_chirho.start_chirho))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
                ExprChirho::SpliceChirho {
                    expr_chirho: Box::new(inner_chirho),
                    span_chirho,
                }
            }
            // Template Haskell typed splice expression: $$name or $$(expr)
            SyntaxKindChirho::TypedSpliceExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let inner_chirho = children_chirho
                    .iter()
                    .find_map(|c_chirho| {
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                            Some(self.lower_expr_chirho(n_chirho, c_chirho.start_chirho))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
                ExprChirho::TypedSpliceChirho {
                    expr_chirho: Box::new(inner_chirho),
                    span_chirho,
                }
            }
            // Template Haskell expression quotation: [| expr |] or [e| expr |]
            SyntaxKindChirho::QuoteExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let inner_chirho = children_chirho
                    .iter()
                    .find_map(|c_chirho| {
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                            Some(self.lower_expr_chirho(n_chirho, c_chirho.start_chirho))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
                ExprChirho::QuoteExprChirho {
                    expr_chirho: Box::new(inner_chirho),
                    span_chirho,
                }
            }
            // Template Haskell declaration quotation: [d| decls |]
            SyntaxKindChirho::QuoteDeclChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let decls_chirho: Vec<DeclChirho> = children_chirho
                    .iter()
                    .filter_map(|c_chirho| {
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                            self.lower_decl_chirho(n_chirho, c_chirho.start_chirho)
                        } else {
                            None
                        }
                    })
                    .collect();
                ExprChirho::QuoteDeclChirho {
                    decls_chirho,
                    span_chirho,
                }
            }
            // Template Haskell type quotation: [t| type |]
            SyntaxKindChirho::QuoteTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let ty_chirho = children_chirho
                    .iter()
                    .find_map(|c_chirho| {
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho
                            && n_chirho.kind_chirho().is_type_chirho()
                        {
                            return Some(self.lower_type_chirho(n_chirho, c_chirho.start_chirho));
                        }
                        None
                    })
                    .unwrap_or_else(|| self.placeholder_type_chirho());
                ExprChirho::QuoteTypeChirho {
                    ty_chirho,
                    span_chirho,
                }
            }
            // Template Haskell pattern quotation: [p| pat |]
            SyntaxKindChirho::QuotePatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let pat_chirho = children_chirho
                    .iter()
                    .find_map(|c_chirho| {
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho
                            && is_pat_kind_chirho(n_chirho.kind_chirho())
                        {
                            return Some(self.lower_pat_chirho(n_chirho, c_chirho.start_chirho));
                        }
                        None
                    })
                    .unwrap_or(PatChirho::WildcardChirho(span_chirho));
                ExprChirho::QuotePatChirho {
                    pat_chirho,
                    span_chirho,
                }
            }
            // Template Haskell typed expression quotation: [|| expr ||]
            SyntaxKindChirho::TypedQuoteExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let inner_chirho = children_chirho
                    .iter()
                    .find_map(|c_chirho| {
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                            Some(self.lower_expr_chirho(n_chirho, c_chirho.start_chirho))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
                ExprChirho::QuoteExprChirho {
                    expr_chirho: Box::new(inner_chirho),
                    span_chirho,
                }
            }
            _ => self.placeholder_expr_chirho(),
        }
    }

    pub(super) fn lower_expr_from_child_chirho(
        &self,
        child_chirho: &ChildChirho<'_>,
    ) -> ExprChirho {
        if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
            self.lower_expr_chirho(n_chirho, child_chirho.start_chirho)
        } else {
            self.placeholder_expr_chirho()
        }
    }

    pub(super) fn lower_first_expr_in_node_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ExprChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let exprs_chirho: Vec<ExprChirho> = children_chirho
            .iter()
            .filter_map(|child_chirho| match child_chirho.element_chirho {
                GreenElementChirho::NodeChirho(n_chirho)
                    if is_expr_kind_chirho(n_chirho.kind_chirho()) =>
                {
                    Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho))
                }
                _ => None,
            })
            .collect();
        if let Some(expr_chirho) =
            self.fold_qual_expr_parts_to_expr_chirho(exprs_chirho, span_chirho)
        {
            return expr_chirho;
        }
        self.placeholder_expr_chirho()
    }
}
