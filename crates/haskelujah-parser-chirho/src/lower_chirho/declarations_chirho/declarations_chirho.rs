// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration dispatch, type and standalone kind signatures, `default`, fixity, pattern synonyms
//! and splices.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, FixityChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::LowerCtxChirho;
use super::context_chirho::is_pat_kind_chirho;

impl LowerCtxChirho {
    // -----------------------------------------------------------------------
    // Declarations
    // -----------------------------------------------------------------------

    pub(super) fn lower_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<DeclChirho> {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());

        match node_chirho.kind_chirho() {
            SyntaxKindChirho::TypeSigDeclChirho => {
                Some(self.lower_type_sig_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::FunBindChirho => {
                Some(self.lower_fun_bind_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::DataDeclChirho => {
                // Skip `data family` and `data instance` declarations
                if self.node_has_keyword_chirho(node_chirho, base_chirho, "family")
                    || self.node_has_keyword_chirho(node_chirho, base_chirho, "instance")
                {
                    None
                } else {
                    Some(self.lower_data_decl_chirho(node_chirho, base_chirho, span_chirho))
                }
            }
            SyntaxKindChirho::NewtypeDeclChirho => {
                // Skip `newtype instance` declarations
                if self.node_has_keyword_chirho(node_chirho, base_chirho, "instance") {
                    None
                } else {
                    Some(self.lower_newtype_decl_chirho(node_chirho, base_chirho, span_chirho))
                }
            }
            SyntaxKindChirho::TypeAliasDeclChirho => {
                Some(self.lower_type_alias_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::TypeFamilyDeclChirho => {
                Some(self.lower_type_family_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::TypeFamilyInstanceDeclChirho => Some(
                self.lower_type_family_instance_decl_chirho(node_chirho, base_chirho, span_chirho),
            ),
            SyntaxKindChirho::ClassDeclChirho => {
                Some(self.lower_class_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::InstanceDeclChirho => {
                Some(self.lower_instance_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::DefaultDeclChirho => {
                Some(self.lower_default_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::FixityDeclChirho => {
                Some(self.lower_fixity_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::ForeignDeclChirho => {
                Some(self.lower_foreign_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::PatSynDeclChirho => {
                Some(self.lower_pat_syn_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::SpliceDeclChirho => {
                Some(self.lower_splice_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::StandaloneDerivingDeclChirho => {
                Some(self.lower_standalone_deriving_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::PatBindChirho => {
                Some(self.lower_pat_bind_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            _ => None,
        }
    }

    pub(super) fn lower_type_sig_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        self.lower_type_sigs_chirho(node_chirho, base_chirho, span_chirho)
            .into_iter()
            .next()
            .unwrap_or_else(|| DeclChirho::TypeSigChirho {
                name_chirho: self.dummy_name_chirho(),
                ty_chirho: self.placeholder_type_chirho(),
                span_chirho,
            })
    }

    pub(super) fn lower_type_sigs_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Vec<DeclChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut names_chirho = Vec::new();
        let mut saw_double_colon_chirho = false;
        let mut type_children_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                        saw_double_colon_chirho = true;
                    } else if saw_double_colon_chirho {
                        type_children_chirho.push(child_chirho);
                    } else if !saw_double_colon_chirho
                        && tok_chirho.kind_chirho() != TokenKindChirho::LeftParenChirho
                        && tok_chirho.kind_chirho() != TokenKindChirho::RightParenChirho
                        && tok_chirho.kind_chirho() != TokenKindChirho::CommaChirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        names_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) if saw_double_colon_chirho => {
                    type_children_chirho.push(child_chirho);
                }
                _ => {}
            }
        }

        let ty_chirho = if let Some(tc_chirho) = type_children_chirho.first() {
            if let GreenElementChirho::NodeChirho(n_chirho) = tc_chirho.element_chirho {
                self.lower_type_chirho(n_chirho, tc_chirho.start_chirho)
            } else {
                self.type_from_flat_children_chirho(&type_children_chirho, span_chirho)
            }
        } else {
            self.placeholder_type_chirho()
        };

        if names_chirho.is_empty() {
            names_chirho.push(self.dummy_name_chirho());
        }

        names_chirho
            .into_iter()
            .map(|name_chirho| DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho: ty_chirho.clone(),
                span_chirho,
            })
            .collect()
    }

    pub(super) fn lower_standalone_kind_sig_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Option<(String, TypeChirho)> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut saw_type_keyword_chirho = false;
        let mut saw_double_colon_chirho = false;
        let mut name_chirho = None;
        let mut type_children_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if !saw_double_colon_chirho
                        && tok_chirho.kind_chirho() == TokenKindChirho::TypeKeywordChirho
                    {
                        saw_type_keyword_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                        saw_double_colon_chirho = true;
                    } else if saw_double_colon_chirho {
                        type_children_chirho.push(child_chirho);
                    } else if saw_type_keyword_chirho
                        && name_chirho.is_none()
                        && tok_chirho.kind_chirho() != TokenKindChirho::LeftParenChirho
                        && tok_chirho.kind_chirho() != TokenKindChirho::RightParenChirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(_) if saw_double_colon_chirho => {
                    type_children_chirho.push(child_chirho);
                }
                _ => {}
            }
        }

        if !saw_type_keyword_chirho || !saw_double_colon_chirho {
            return None;
        }
        let name_chirho = name_chirho?;
        let kind_chirho = if let Some(tc_chirho) = type_children_chirho.first() {
            if let GreenElementChirho::NodeChirho(n_chirho) = tc_chirho.element_chirho {
                self.lower_type_chirho(n_chirho, tc_chirho.start_chirho)
            } else {
                self.declaration_kind_from_children_chirho(&type_children_chirho, span_chirho)
            }
        } else {
            return None;
        };

        Some((name_chirho.text_chirho().to_string(), kind_chirho))
    }

    /// Lower a default declaration: `default (Int, Double)`.
    pub(super) fn lower_default_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut types_chirho = Vec::new();
        let mut saw_default_chirho = false;

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let kind_chirho = tok_chirho.kind_chirho();
                if kind_chirho == TokenKindChirho::DefaultKeywordChirho {
                    saw_default_chirho = true;
                    continue;
                }
                if saw_default_chirho && kind_chirho == TokenKindChirho::ConIdChirho {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    types_chirho.push(TypeChirho::ConChirho(
                        self.name_from_token_chirho(tok_chirho, s_chirho),
                    ));
                }
            }
        }

        DeclChirho::DefaultDeclChirho {
            types_chirho,
            span_chirho,
        }
    }

    fn lower_fixity_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let (fixity_chirho, precedence_chirho, ops_chirho) =
            self.extract_fixity_decl_parts_chirho(node_chirho, base_chirho);

        DeclChirho::FixityDeclChirho {
            fixity_chirho,
            precedence_chirho,
            ops_chirho,
            span_chirho,
        }
    }

    pub(super) fn extract_fixity_decl_parts_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> (FixityChirho, Option<u8>, Vec<NameChirho>) {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut fixity_chirho = FixityChirho::InfixlChirho;
        let mut precedence_chirho = None;
        let mut ops_chirho = Vec::new();
        let mut in_backticks_chirho = false;

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let s_chirho = self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::InfixKeywordChirho => {
                        fixity_chirho = FixityChirho::InfixChirho
                    }
                    TokenKindChirho::InfixlKeywordChirho => {
                        fixity_chirho = FixityChirho::InfixlChirho
                    }
                    TokenKindChirho::InfixrKeywordChirho => {
                        fixity_chirho = FixityChirho::InfixrChirho
                    }
                    TokenKindChirho::IntegerLiteralChirho => {
                        precedence_chirho = tok_chirho.text_chirho().parse::<u8>().ok();
                    }
                    TokenKindChirho::BacktickChirho => {
                        in_backticks_chirho = !in_backticks_chirho;
                    }
                    TokenKindChirho::VarSymChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::QualifiedVarSymChirho
                    | TokenKindChirho::QualifiedConSymChirho => {
                        ops_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    TokenKindChirho::VarIdChirho
                    | TokenKindChirho::ConIdChirho
                    | TokenKindChirho::QualifiedVarIdChirho
                    | TokenKindChirho::QualifiedConIdChirho
                        if in_backticks_chirho =>
                    {
                        ops_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    _ => {}
                }
            }
        }

        (fixity_chirho, precedence_chirho, ops_chirho)
    }

    /// Lower a `foreign import/export` declaration from CST to AST.
    ///
    /// Expected token sequence (all trivia already filtered):
    ///   foreign  import|export  callconv  [safety]  ["c_name"]  hsName  ::  type...
    fn lower_pat_syn_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        if children_chirho.iter().any(|child_chirho| {
            matches!(
                child_chirho.element_chirho,
                GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
            )
        }) {
            let mut names_chirho = Vec::new();
            let mut ty_chirho = None;
            let mut signature_chirho = false;
            for child_chirho in &children_chirho {
                match child_chirho.element_chirho {
                    GreenElementChirho::TokenChirho(token_chirho) => {
                        if token_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                            signature_chirho = true;
                        } else if !signature_chirho
                            && token_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                        {
                            names_chirho.push(self.name_from_token_chirho(
                                token_chirho,
                                self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                ),
                            ));
                        }
                    }
                    GreenElementChirho::NodeChirho(type_chirho)
                        if signature_chirho && ty_chirho.is_none() =>
                    {
                        ty_chirho = Some(self.lower_pattern_signature_type_chirho(
                            type_chirho,
                            child_chirho.start_chirho,
                        ));
                    }
                    _ => {}
                }
            }
            return DeclChirho::PatSynSigDeclChirho {
                names_chirho,
                ty_chirho: ty_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
                span_chirho,
            };
        }

        let mut name_chirho: Option<NameChirho> = None;
        let mut args_chirho: Vec<NameChirho> = Vec::new();
        let mut saw_keyword_chirho = false;
        let mut saw_sep_chirho = false;
        let mut is_equals_chirho = false;
        let mut pat_chirho: Option<PatChirho> = None;
        let mut has_where_chirho = false;
        let mut record_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::LeftBraceChirho && !saw_sep_chirho {
                        record_chirho = true;
                    }
                    if kind_chirho == TokenKindChirho::VarIdChirho && !saw_keyword_chirho {
                        // "pattern" keyword — skip
                        saw_keyword_chirho = true;
                    } else if kind_chirho == TokenKindChirho::ConIdChirho && name_chirho.is_none() {
                        // Pattern synonym name
                        let tok_span_chirho = self.span_chirho(
                            child_chirho.start_chirho,
                            child_chirho.start_chirho + tok_chirho.text_len_chirho(),
                        );
                        name_chirho = Some(
                            self.name_from_text_chirho(tok_chirho.text_chirho(), tok_span_chirho),
                        );
                    } else if kind_chirho == TokenKindChirho::VarIdChirho
                        && !saw_sep_chirho
                        && name_chirho.is_some()
                    {
                        // Pattern variable
                        let tok_span_chirho = self.span_chirho(
                            child_chirho.start_chirho,
                            child_chirho.start_chirho + tok_chirho.text_len_chirho(),
                        );
                        args_chirho.push(
                            self.name_from_text_chirho(tok_chirho.text_chirho(), tok_span_chirho),
                        );
                    } else if kind_chirho == TokenKindChirho::EqualsChirho && !saw_sep_chirho {
                        saw_sep_chirho = true;
                        is_equals_chirho = true;
                    } else if kind_chirho == TokenKindChirho::LeftArrowChirho && !saw_sep_chirho {
                        saw_sep_chirho = true;
                        is_equals_chirho = false;
                    } else if kind_chirho == TokenKindChirho::WhereKeywordChirho {
                        has_where_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_sep_chirho && pat_chirho.is_none() && !has_where_chirho {
                        // First node after separator is the pattern
                        if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                            pat_chirho =
                                Some(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                        } else {
                            // Might be an expression node — try to interpret as pattern
                            pat_chirho =
                                Some(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                        }
                    }
                }
            }
        }

        let dir_chirho = if is_equals_chirho {
            haskelujah_ast_chirho::decl_chirho::PatSynDirChirho::ImplBidirChirho
        } else if has_where_chirho {
            // Explicitly bidirectional — where clause bindings
            // For now, store empty; full support would parse the where block
            haskelujah_ast_chirho::decl_chirho::PatSynDirChirho::ExplBidirChirho {
                builder_binds_chirho: vec![],
            }
        } else {
            haskelujah_ast_chirho::decl_chirho::PatSynDirChirho::UnidirChirho
        };

        DeclChirho::PatSynDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            args_chirho,
            record_chirho,
            dir_chirho,
            pat_chirho: pat_chirho.unwrap_or(PatChirho::WildcardChirho(span_chirho)),
            span_chirho,
        }
    }

    /// Pattern signatures give their two contexts different roles. In
    /// particular, `() => C a => t` must keep its empty required context.
    fn lower_pattern_signature_type_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> TypeChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        match node_chirho.kind_chirho() {
            SyntaxKindChirho::QualTypeChirho => {
                let parts_chirho = self.lower_context_parts_chirho(node_chirho, base_chirho);
                let body_chirho = parts_chirho
                    .body_chirho
                    .map(|body_chirho| {
                        self.lower_pattern_signature_type_chirho(
                            body_chirho.node_chirho,
                            body_chirho.base_chirho,
                        )
                    })
                    .unwrap_or_else(|| self.placeholder_type_chirho());
                TypeChirho::QualChirho {
                    context_chirho: parts_chirho.constraints_chirho,
                    body_chirho: Box::new(body_chirho),
                    span_chirho,
                }
            }
            SyntaxKindChirho::ForallTypeChirho => {
                let parts_chirho = self.lower_forall_parts_chirho(node_chirho, base_chirho);
                let body_chirho = Box::new(
                    parts_chirho
                        .body_chirho
                        .map(|body_chirho| {
                            self.lower_pattern_signature_type_chirho(
                                body_chirho.node_chirho,
                                body_chirho.base_chirho,
                            )
                        })
                        .unwrap_or_else(|| self.placeholder_type_chirho()),
                );
                if parts_chirho.required_chirho {
                    TypeChirho::RequiredForallChirho {
                        vars_chirho: parts_chirho.vars_chirho,
                        body_chirho,
                        span_chirho,
                    }
                } else {
                    TypeChirho::ForallChirho {
                        vars_chirho: parts_chirho.vars_chirho,
                        body_chirho,
                        span_chirho,
                    }
                }
            }
            _ => self.lower_type_chirho(node_chirho, base_chirho),
        }
    }

    /// Lower a Template Haskell splice declaration: `$(expr)` or `$name` at top level.
    fn lower_splice_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let expr_chirho = children_chirho
            .iter()
            .find_map(|c_chirho| {
                if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                    Some(self.lower_expr_chirho(n_chirho, c_chirho.start_chirho))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.placeholder_expr_chirho());

        DeclChirho::SpliceDeclChirho {
            expr_chirho,
            span_chirho,
        }
    }
}
