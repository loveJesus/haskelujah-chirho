// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Parts of expressions: quoted names, record fields and updates, case alternatives, literals and
//! type-lambda binders.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::FieldDeclChirho;
use haskelujah_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, FieldAssignChirho, GuardedExprChirho, RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::lit_chirho::{
    LitChirho, parse_haskell_char_body_chirho, unescape_string_body_chirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::LowerCtxChirho;
use super::bracketed_exprs_chirho::QualPartChirho;
use super::context_chirho::is_pat_kind_chirho;

impl LowerCtxChirho {
    pub(super) fn lower_quoted_name_expr_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ExprChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut quote_count_chirho = 0usize;
        let mut inside_parens_chirho = false;
        let mut paren_text_chirho = String::new();
        let mut name_text_chirho: Option<String> = None;

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::TickChirho => {
                        quote_count_chirho += 1;
                    }
                    TokenKindChirho::LeftParenChirho => {
                        inside_parens_chirho = true;
                    }
                    TokenKindChirho::RightParenChirho => {
                        inside_parens_chirho = false;
                    }
                    TokenKindChirho::VarIdChirho
                    | TokenKindChirho::ConIdChirho
                    | TokenKindChirho::QualifiedVarIdChirho
                    | TokenKindChirho::QualifiedConIdChirho => {
                        if inside_parens_chirho {
                            paren_text_chirho.push_str(tok_chirho.text_chirho());
                        } else if name_text_chirho.is_none() {
                            name_text_chirho = Some(tok_chirho.text_chirho().to_string());
                        }
                    }
                    TokenKindChirho::VarSymChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::QualifiedVarSymChirho
                    | TokenKindChirho::QualifiedConSymChirho => {
                        if inside_parens_chirho {
                            paren_text_chirho.push_str(tok_chirho.text_chirho());
                        } else if name_text_chirho.is_none() {
                            name_text_chirho = Some(tok_chirho.text_chirho().to_string());
                        }
                    }
                    TokenKindChirho::TildeChirho => {
                        if inside_parens_chirho {
                            paren_text_chirho.push_str(tok_chirho.text_chirho());
                        } else if name_text_chirho.is_none() {
                            name_text_chirho = Some(tok_chirho.text_chirho().to_string());
                        }
                    }
                    TokenKindChirho::CommaChirho => {
                        if inside_parens_chirho {
                            paren_text_chirho.push(',');
                        }
                    }
                    _ => {}
                }
            }
        }

        let quoted_name_text_chirho = if !paren_text_chirho.is_empty() {
            if quote_count_chirho >= 2 && paren_text_chirho.chars().all(|c_chirho| c_chirho == ',')
            {
                format!("({paren_text_chirho})")
            } else {
                paren_text_chirho
            }
        } else {
            name_text_chirho.unwrap_or_else(|| "_".to_string())
        };

        ExprChirho::AppChirho {
            fun_chirho: Box::new(self.reference_expr_chirho(NameChirho::RawChirho(
                RawNameChirho::unqualified_chirho("mkName", span_chirho),
            ))),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::StringChirho(
                quoted_name_text_chirho,
                span_chirho,
                Some(self.fresh_origin_chirho()),
            ))),
            span_chirho,
        }
    }

    /// Lower a `RecordFieldsChirho` CST node to a vec of `FieldDeclChirho`.
    /// CST structure: LeftBrace FieldDeclChirho ... RightBrace
    pub(super) fn lower_record_fields_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<FieldDeclChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut result_chirho = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                && n_chirho.kind_chirho() == SyntaxKindChirho::FieldDeclChirho
            {
                result_chirho
                    .push(self.lower_field_decl_chirho(n_chirho, child_chirho.start_chirho));
            }
        }
        result_chirho
    }

    /// Lower a `FieldAssignChirho` CST node to an AST `FieldAssignChirho`.
    /// CST structure: VarIdChirho EqualsChirho <expr tokens>
    pub(super) fn lower_field_assign_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> FieldAssignChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut field_name_chirho: Option<NameChirho> = None;
        let mut value_chirho: Option<ExprChirho> = None;
        let mut saw_equals_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if !saw_equals_chirho
                        && (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedVarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::QualifiedConIdChirho)
                        && field_name_chirho.is_none()
                    {
                        field_name_chirho =
                            Some(self.name_from_token_chirho(tok_chirho, span_chirho));
                    }
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    }
                    // After =, a literal token is a valid value
                    if saw_equals_chirho && value_chirho.is_none() {
                        match tok_chirho.kind_chirho() {
                            TokenKindChirho::IntegerLiteralChirho => {
                                value_chirho =
                                    Some(ExprChirho::LitChirho(self.integer_literal_chirho(
                                        tok_chirho.text_chirho(),
                                        span_chirho,
                                    )));
                            }
                            TokenKindChirho::StringLiteralChirho => {
                                // MagicHash: strip trailing # (e.g. "hello"#)
                                let raw_chirho = tok_chirho.text_chirho().trim_end_matches('#');
                                let s_chirho =
                                    if raw_chirho.starts_with('"') && raw_chirho.ends_with('"') {
                                        raw_chirho[1..raw_chirho.len() - 1].to_string()
                                    } else {
                                        raw_chirho.to_string()
                                    };
                                value_chirho =
                                    Some(ExprChirho::LitChirho(LitChirho::StringChirho(
                                        s_chirho,
                                        span_chirho,
                                        Some(self.fresh_origin_chirho()),
                                    )));
                            }
                            TokenKindChirho::FloatLiteralChirho => {
                                let clean_chirho: String = tok_chirho
                                    .text_chirho()
                                    .trim_end_matches('#')
                                    .chars()
                                    .filter(|c_chirho| *c_chirho != '_')
                                    .collect();
                                let f_chirho = clean_chirho.parse::<f64>().unwrap_or(0.0);
                                value_chirho = Some(ExprChirho::LitChirho(LitChirho::FloatChirho(
                                    f_chirho,
                                    span_chirho,
                                    Some(self.fresh_origin_chirho()),
                                )));
                            }
                            TokenKindChirho::CharLiteralChirho => {
                                let raw_chirho = tok_chirho.text_chirho().trim_end_matches('#');
                                let c_chirho = if raw_chirho.starts_with('\'')
                                    && raw_chirho.ends_with('\'')
                                    && raw_chirho.len() >= 3
                                {
                                    raw_chirho.chars().nth(1).unwrap_or(' ')
                                } else {
                                    ' '
                                };
                                value_chirho = Some(ExprChirho::LitChirho(LitChirho::CharChirho(
                                    c_chirho,
                                    span_chirho,
                                    Some(self.fresh_origin_chirho()),
                                )));
                            }
                            TokenKindChirho::VarIdChirho
                            | TokenKindChirho::UnderscoreReservedIdChirho => {
                                // UnderscoreReservedIdChirho: typed holes (_) in expression position
                                value_chirho = Some(self.reference_expr_chirho(
                                    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                                        tok_chirho.text_chirho().to_string(),
                                        span_chirho,
                                    )),
                                ));
                            }
                            TokenKindChirho::ConIdChirho => {
                                value_chirho =
                                    Some(ExprChirho::ConChirho(self.reference_name_chirho(
                                        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                                            tok_chirho.text_chirho().to_string(),
                                            span_chirho,
                                        )),
                                    )));
                            }
                            _ => {}
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_equals_chirho && value_chirho.is_none() {
                        value_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        // NamedFieldPuns: if no `=` was found, the value is a variable with the
        // same name as the field, e.g. `Con { x }` means `Con { x = x }`
        let final_value_chirho = value_chirho.unwrap_or_else(|| {
            if let Some(ref fname_chirho) = field_name_chirho {
                self.reference_expr_chirho(fname_chirho.clone())
            } else {
                self.placeholder_expr_chirho()
            }
        });

        FieldAssignChirho {
            name_chirho: field_name_chirho.unwrap_or_else(|| {
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    "_".to_string(),
                    span_chirho,
                ))
            }),
            value_chirho: final_value_chirho,
            span_chirho,
        }
    }

    pub(super) fn lower_case_alt_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> AltChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut pat_chirho = None;
        let mut rhs_chirho = None;
        let mut where_binds_chirho = Vec::new();
        let mut saw_arrow_chirho = false;
        let mut saw_pipe_chirho = false;
        let mut in_guard_body_chirho = false;
        let mut current_qual_parts_chirho: Vec<QualPartChirho> = Vec::new();
        let mut current_guard_quals_chirho: Vec<StmtChirho> = Vec::new();
        let mut current_guard_body_chirho = None;
        let mut guarded_arms_chirho: Vec<GuardedExprChirho> = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho {
                        saw_pipe_chirho = true;
                        in_guard_body_chirho = false;
                        if current_guard_body_chirho.is_some() {
                            guarded_arms_chirho.push(GuardedExprChirho {
                                quals_chirho: std::mem::take(&mut current_guard_quals_chirho),
                                body_chirho: current_guard_body_chirho
                                    .take()
                                    .unwrap_or_else(|| self.placeholder_expr_chirho()),
                                span_chirho,
                            });
                        }
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho
                        && saw_pipe_chirho
                        && !in_guard_body_chirho
                    {
                        if let Some(qual_chirho) =
                            self.flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
                        {
                            current_guard_quals_chirho.push(qual_chirho);
                        }
                        current_qual_parts_chirho.clear();
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftArrowChirho
                        && saw_pipe_chirho
                        && !in_guard_body_chirho
                    {
                        current_qual_parts_chirho.push(QualPartChirho::ArrowChirho);
                    } else if saw_pipe_chirho
                        && !in_guard_body_chirho
                        && matches!(
                            tok_chirho.kind_chirho(),
                            TokenKindChirho::ConSymChirho | TokenKindChirho::QualifiedConSymChirho
                        )
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
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho {
                        if saw_pipe_chirho {
                            if let Some(qual_chirho) = self
                                .flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
                            {
                                current_guard_quals_chirho.push(qual_chirho);
                            }
                            current_qual_parts_chirho.clear();
                            in_guard_body_chirho = true;
                        }
                        saw_arrow_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if !saw_arrow_chirho && pat_chirho.is_none() {
                        pat_chirho =
                            Some(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_pipe_chirho
                        && !in_guard_body_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::LetStmtChirho
                    {
                        if let Some(qual_chirho) =
                            self.flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
                        {
                            current_guard_quals_chirho.push(qual_chirho);
                        }
                        current_qual_parts_chirho.clear();
                        current_guard_quals_chirho.push(StmtChirho::LetChirho {
                            binds_chirho: self
                                .lower_let_stmt_binds_chirho(n_chirho, child_chirho.start_chirho),
                            span_chirho: self
                                .span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                        });
                    } else if saw_pipe_chirho && !in_guard_body_chirho {
                        if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                            current_qual_parts_chirho.push(QualPartChirho::PatChirho(
                                self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                            ));
                        } else {
                            current_qual_parts_chirho.push(QualPartChirho::ExprChirho(
                                self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                            ));
                        }
                    } else if saw_arrow_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                    {
                        where_binds_chirho =
                            self.lower_where_clause_chirho(n_chirho, child_chirho.start_chirho);
                    } else if saw_pipe_chirho
                        && in_guard_body_chirho
                        && current_guard_body_chirho.is_none()
                    {
                        current_guard_body_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_arrow_chirho && rhs_chirho.is_none() {
                        rhs_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        let rhs_chirho = if saw_pipe_chirho {
            if !current_qual_parts_chirho.is_empty()
                && let Some(qual_chirho) =
                    self.flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
            {
                current_guard_quals_chirho.push(qual_chirho);
            }
            if !current_guard_quals_chirho.is_empty() || current_guard_body_chirho.is_some() {
                guarded_arms_chirho.push(GuardedExprChirho {
                    quals_chirho: current_guard_quals_chirho,
                    body_chirho: current_guard_body_chirho
                        .unwrap_or_else(|| self.placeholder_expr_chirho()),
                    span_chirho,
                });
            }
            RhsChirho::GuardedChirho(guarded_arms_chirho)
        } else {
            RhsChirho::UnguardedChirho(rhs_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()))
        };

        AltChirho {
            pat_chirho: pat_chirho.unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
            rhs_chirho,
            where_binds_chirho,
            span_chirho,
        }
    }

    pub(super) fn lower_lit_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> LitChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let span_chirho =
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::IntegerLiteralChirho => {
                        return self.integer_literal_chirho(tok_chirho.text_chirho(), span_chirho);
                    }
                    TokenKindChirho::FloatLiteralChirho => {
                        let clean_chirho: String = tok_chirho
                            .text_chirho()
                            .trim_end_matches('#')
                            .chars()
                            .filter(|c_chirho| *c_chirho != '_')
                            .collect();
                        let val_chirho = clean_chirho.parse::<f64>().unwrap_or(0.0);
                        return LitChirho::FloatChirho(
                            val_chirho,
                            span_chirho,
                            Some(self.fresh_origin_chirho()),
                        );
                    }
                    TokenKindChirho::CharLiteralChirho => {
                        let text_chirho = tok_chirho.text_chirho().trim_end_matches('#');
                        // Exactly one quote at each end: `'\''` keeps its escaped quote.
                        let inner_chirho = text_chirho
                            .strip_prefix('\'')
                            .and_then(|body_chirho| body_chirho.strip_suffix('\''))
                            .unwrap_or(text_chirho);
                        let ch_chirho =
                            parse_haskell_char_body_chirho(inner_chirho).unwrap_or('\u{FFFD}');
                        return LitChirho::CharChirho(
                            ch_chirho,
                            span_chirho,
                            Some(self.fresh_origin_chirho()),
                        );
                    }
                    TokenKindChirho::StringLiteralChirho => {
                        // MagicHash: strip trailing # before quote extraction
                        let text_chirho = tok_chirho.text_chirho().trim_end_matches('#');
                        let raw_chirho = text_chirho
                            .strip_prefix('"')
                            .and_then(|s_chirho| s_chirho.strip_suffix('"'))
                            .unwrap_or(text_chirho);
                        let inner_chirho = unescape_string_body_chirho(raw_chirho);
                        return LitChirho::StringChirho(
                            inner_chirho,
                            span_chirho,
                            Some(self.fresh_origin_chirho()),
                        );
                    }
                    _ => {}
                }
            }
        }
        LitChirho::IntChirho(
            0,
            SpanChirho::DUMMY_CHIRHO,
            Some(self.fresh_origin_chirho()),
        )
    }

    // workflow: language-features-chirho/rank-n-visible-type-application-chirho
    pub(super) fn extract_type_lambda_binder_name_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> NameChirho {
        for child_chirho in self.semantic_children_chirho(node_chirho, base_chirho) {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
                {
                    let span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    return self.name_from_token_chirho(token_chirho, span_chirho);
                }
                GreenElementChirho::NodeChirho(child_node_chirho)
                    if child_node_chirho.kind_chirho() == SyntaxKindChirho::ParenTypeChirho =>
                {
                    if let Some(type_var_chirho) = self
                        .try_lower_kind_annotated_forall_binder_chirho(
                            child_node_chirho,
                            child_chirho.start_chirho,
                        )
                    {
                        return type_var_chirho.name_chirho;
                    }
                }
                _ => {}
            }
        }
        self.dummy_name_chirho()
    }
}
