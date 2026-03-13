// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # CST → AST lowering
//!
//! Walks the immutable green tree produced by the CST parser and builds
//! the typed AST (`ModuleChirho`). Trivia, virtual layout tokens, and
//! punctuation are discarded; only semantic content survives.

use std::sync::Arc;

use rhasky_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, FixityChirho};
use rhasky_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, MatchArmChirho, RhsChirho, StmtChirho,
};
use rhasky_ast_chirho::lit_chirho::LitChirho;
use rhasky_ast_chirho::module_chirho::{ImportDeclChirho, ModuleChirho};
use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use rhasky_ast_chirho::pat_chirho::PatChirho;
use rhasky_ast_chirho::ty_chirho::TypeChirho;
use rhasky_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};
use rhasky_syntax_chirho::cst_chirho::SyntaxKindChirho;
use rhasky_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho, GreenTokenChirho};
use rhasky_syntax_chirho::token_chirho::TokenKindChirho;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Lower a green CST root into a typed AST module.
pub fn lower_module_chirho(
    root_chirho: &Arc<GreenNodeChirho>,
    file_id_chirho: FileIdChirho,
) -> ModuleChirho {
    let mut ctx_chirho = LowerCtxChirho::new_chirho(file_id_chirho);
    ctx_chirho.lower_source_file_chirho(root_chirho)
}

// ---------------------------------------------------------------------------
// LowerCtxChirho — lowering context that tracks byte offset
// ---------------------------------------------------------------------------

struct LowerCtxChirho {
    file_id_chirho: FileIdChirho,
    offset_chirho: usize,
}

impl LowerCtxChirho {
    fn new_chirho(file_id_chirho: FileIdChirho) -> Self {
        Self {
            file_id_chirho,
            offset_chirho: 0,
        }
    }

    fn span_chirho(&self, start_chirho: usize, end_chirho: usize) -> SpanChirho {
        SpanChirho::new_chirho(
            self.file_id_chirho,
            ByteOffsetChirho::from_usize_chirho(start_chirho),
            ByteOffsetChirho::from_usize_chirho(end_chirho),
        )
    }

    // -----------------------------------------------------------------------
    // Green tree traversal helpers
    // -----------------------------------------------------------------------

    /// Collect non-trivia children from a green node, recording each
    /// child's byte offset range.
    fn semantic_children_chirho<'a>(
        &self,
        node_chirho: &'a GreenNodeChirho,
        base_offset_chirho: usize,
    ) -> Vec<ChildChirho<'a>> {
        let mut result_chirho = Vec::new();
        let mut off_chirho = base_offset_chirho;
        for child_chirho in node_chirho.children_chirho() {
            let start_chirho = off_chirho;
            let len_chirho = child_chirho.text_len_chirho();
            off_chirho += len_chirho;
            if is_trivia_element_chirho(child_chirho) {
                continue;
            }
            result_chirho.push(ChildChirho {
                element_chirho: child_chirho,
                start_chirho,
                end_chirho: start_chirho + len_chirho,
            });
        }
        result_chirho
    }

    /// Get the first token text matching a given kind from a node's children.
    fn first_token_text_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        kind_chirho: TokenKindChirho,
    ) -> Option<String> {
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho {
                if tok_chirho.kind_chirho() == kind_chirho {
                    return Some(tok_chirho.text_chirho().to_string());
                }
            }
        }
        None
    }

    // -----------------------------------------------------------------------
    // SourceFile
    // -----------------------------------------------------------------------

    fn lower_source_file_chirho(&mut self, root_chirho: &GreenNodeChirho) -> ModuleChirho {
        let start_chirho = self.offset_chirho;
        let children_chirho = self.semantic_children_chirho(root_chirho, start_chirho);

        let mut module_name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "Main",
            SpanChirho::DUMMY_CHIRHO,
        ));
        let mut imports_chirho = Vec::new();
        let mut decls_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::NodeChirho(n_chirho) => {
                    match n_chirho.kind_chirho() {
                        SyntaxKindChirho::ModuleHeaderChirho => {
                            module_name_chirho =
                                self.lower_module_header_chirho(n_chirho, child_chirho.start_chirho);
                        }
                        SyntaxKindChirho::ImportDeclChirho => {
                            imports_chirho.push(self.lower_import_decl_chirho(
                                n_chirho,
                                child_chirho.start_chirho,
                            ));
                        }
                        _ => {
                            if let Some(decl_chirho) =
                                self.lower_decl_chirho(n_chirho, child_chirho.start_chirho)
                            {
                                decls_chirho.push(decl_chirho);
                            }
                        }
                    }
                }
                GreenElementChirho::TokenChirho(_) => {}
            }
        }

        let end_chirho = start_chirho + root_chirho.text_len_chirho();
        ModuleChirho {
            name_chirho: module_name_chirho,
            exports_chirho: None, // TODO: lower export list
            imports_chirho,
            decls_chirho,
            span_chirho: self.span_chirho(start_chirho, end_chirho),
        }
    }

    // -----------------------------------------------------------------------
    // Module header
    // -----------------------------------------------------------------------

    fn lower_module_header_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> NameChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        // Find the module name token (ConId or QualifiedConId after 'module' keyword)
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho);
                    }
                    _ => {}
                }
            }
        }
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "Main",
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    // -----------------------------------------------------------------------
    // Import declarations
    // -----------------------------------------------------------------------

    fn lower_import_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ImportDeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut qualified_chirho = false;
        let mut module_name_chirho = None;
        let mut alias_chirho = None;
        let mut saw_as_chirho = false;

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::VarIdChirho if tok_chirho.text_chirho() == "qualified" => {
                        qualified_chirho = true;
                    }
                    TokenKindChirho::VarIdChirho if tok_chirho.text_chirho() == "as" => {
                        saw_as_chirho = true;
                    }
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        let name_chirho =
                            self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho);
                        if saw_as_chirho {
                            alias_chirho = Some(name_chirho);
                            saw_as_chirho = false;
                        } else if module_name_chirho.is_none() {
                            module_name_chirho = Some(name_chirho);
                        }
                    }
                    _ => {}
                }
            }
        }

        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        ImportDeclChirho {
            module_chirho: module_name_chirho.unwrap_or_else(|| {
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    "",
                    SpanChirho::DUMMY_CHIRHO,
                ))
            }),
            qualified_chirho,
            alias_chirho,
            spec_chirho: None, // TODO: lower import spec
            span_chirho,
        }
    }

    // -----------------------------------------------------------------------
    // Declarations
    // -----------------------------------------------------------------------

    fn lower_decl_chirho(
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
                Some(self.lower_data_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::NewtypeDeclChirho => {
                Some(self.lower_newtype_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::TypeAliasDeclChirho => {
                Some(self.lower_type_alias_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::ClassDeclChirho => {
                Some(self.lower_class_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            SyntaxKindChirho::FixityDeclChirho => {
                Some(self.lower_fixity_decl_chirho(node_chirho, base_chirho, span_chirho))
            }
            _ => None,
        }
    }

    fn lower_type_sig_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut saw_double_colon_chirho = false;
        let mut type_children_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                        saw_double_colon_chirho = true;
                    } else if !saw_double_colon_chirho && name_chirho.is_none() {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
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
                self.placeholder_type_chirho()
            }
        } else {
            self.placeholder_type_chirho()
        };

        DeclChirho::TypeSigChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            ty_chirho,
            span_chirho,
        }
    }

    fn lower_fun_bind_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut matches_chirho = Vec::new();

        // The FunBind node contains Match children; the function name is the
        // first VarId or ConId token inside the first Match.
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::MatchChirho {
                    if name_chirho.is_none() {
                        name_chirho =
                            self.extract_fun_name_from_match_chirho(n_chirho, child_chirho.start_chirho);
                    }
                    matches_chirho
                        .push(self.lower_match_arm_chirho(n_chirho, child_chirho.start_chirho));
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
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                    || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                }
            }
        }
        None
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
        let mut saw_equals_chirho = false;
        let where_binds_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if !saw_equals_chirho && is_pat_kind_chirho(n_chirho.kind_chirho()) {
                        pats_chirho.push(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_equals_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                    {
                        // TODO: lower where binds
                    } else if saw_equals_chirho && rhs_expr_chirho.is_none() {
                        rhs_expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        MatchArmChirho {
            pats_chirho,
            rhs_chirho: RhsChirho::UnguardedChirho(
                rhs_expr_chirho.unwrap_or(self.placeholder_expr_chirho()),
            ),
            where_binds_chirho,
            span_chirho,
        }
    }

    fn lower_data_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        let mut constructors_chirho = Vec::new();
        let mut deriving_chirho = Vec::new();
        let mut saw_equals_chirho = false;
        let mut saw_data_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::DataKeywordChirho {
                        saw_data_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_data_chirho && !saw_equals_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            && name_chirho.is_none()
                        {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                            type_vars_chirho
                                .push(self.name_from_token_chirho(tok_chirho, s_chirho));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::ConDeclChirho {
                        constructors_chirho.push(
                            self.lower_con_decl_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::DerivingClauseChirho {
                        deriving_chirho =
                            self.lower_deriving_chirho(n_chirho, child_chirho.start_chirho);
                    }
                }
            }
        }

        DeclChirho::DataDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            span_chirho,
        }
    }

    fn lower_con_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ConDeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut name_chirho = None;
        let mut fields_chirho = Vec::new();
        let mut has_record_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                        && name_chirho.is_none()
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::RecordFieldsChirho {
                        has_record_chirho = true;
                        // TODO: lower record fields
                    } else if is_type_kind_chirho(n_chirho.kind_chirho()) {
                        fields_chirho
                            .push(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        let con_name_chirho = name_chirho.unwrap_or_else(|| self.dummy_name_chirho());
        if has_record_chirho {
            ConDeclChirho::RecordChirho {
                name_chirho: con_name_chirho,
                fields_chirho: vec![], // TODO
                span_chirho,
            }
        } else {
            ConDeclChirho::OrdinaryChirho {
                name_chirho: con_name_chirho,
                fields_chirho,
                span_chirho,
            }
        }
    }

    fn lower_deriving_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<NameChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut names_chirho = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    names_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                }
            }
        }
        names_chirho
    }

    fn lower_newtype_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        let mut constructor_chirho = None;
        let mut deriving_chirho = Vec::new();
        let mut saw_equals_chirho = false;
        let mut saw_newtype_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::NewtypeKeywordChirho {
                        saw_newtype_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_newtype_chirho && !saw_equals_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            && name_chirho.is_none()
                        {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                            type_vars_chirho
                                .push(self.name_from_token_chirho(tok_chirho, s_chirho));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::ConDeclChirho {
                        constructor_chirho =
                            Some(self.lower_con_decl_chirho(n_chirho, child_chirho.start_chirho));
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::DerivingClauseChirho {
                        deriving_chirho =
                            self.lower_deriving_chirho(n_chirho, child_chirho.start_chirho);
                    }
                }
            }
        }

        DeclChirho::NewtypeDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            constructor_chirho: constructor_chirho.unwrap_or_else(|| ConDeclChirho::OrdinaryChirho {
                name_chirho: self.dummy_name_chirho(),
                fields_chirho: vec![],
                span_chirho,
            }),
            deriving_chirho,
            span_chirho,
        }
    }

    fn lower_type_alias_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        let mut rhs_chirho = None;
        let mut saw_equals_chirho = false;
        let mut saw_type_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::TypeKeywordChirho {
                        saw_type_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_type_chirho && !saw_equals_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            && name_chirho.is_none()
                        {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                            type_vars_chirho
                                .push(self.name_from_token_chirho(tok_chirho, s_chirho));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) if saw_equals_chirho => {
                    if rhs_chirho.is_none() {
                        rhs_chirho =
                            Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
                _ => {}
            }
        }

        DeclChirho::TypeAliasDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            rhs_chirho: rhs_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
            span_chirho,
        }
    }

    fn lower_class_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        let mut saw_class_chirho = false;
        let mut saw_where_chirho = false;

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::ClassKeywordChirho {
                    saw_class_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::WhereKeywordChirho {
                    saw_where_chirho = true;
                } else if saw_class_chirho && !saw_where_chirho {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                        && name_chirho.is_none()
                    {
                        name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                        type_vars_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                }
            }
        }

        DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            methods_chirho: vec![], // TODO: lower methods from where block
            span_chirho,
        }
    }

    fn lower_fixity_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut fixity_chirho = FixityChirho::InfixlChirho;
        let mut precedence_chirho = None;
        let mut ops_chirho = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let s_chirho =
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::InfixKeywordChirho => fixity_chirho = FixityChirho::InfixChirho,
                    TokenKindChirho::InfixlKeywordChirho => {
                        fixity_chirho = FixityChirho::InfixlChirho
                    }
                    TokenKindChirho::InfixrKeywordChirho => {
                        fixity_chirho = FixityChirho::InfixrChirho
                    }
                    TokenKindChirho::IntegerLiteralChirho => {
                        precedence_chirho = tok_chirho.text_chirho().parse::<u8>().ok();
                    }
                    TokenKindChirho::VarSymChirho | TokenKindChirho::ConSymChirho => {
                        ops_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    _ => {}
                }
            }
        }

        DeclChirho::FixityDeclChirho {
            fixity_chirho,
            precedence_chirho,
            ops_chirho,
            span_chirho,
        }
    }

    // -----------------------------------------------------------------------
    // Types
    // -----------------------------------------------------------------------

    fn lower_type_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> TypeChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());

        match node_chirho.kind_chirho() {
            SyntaxKindChirho::FunTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();
                if type_nodes_chirho.len() >= 2 {
                    let arg_chirho = self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                    let result_chirho = self.lower_type_from_child_chirho(
                        type_nodes_chirho[type_nodes_chirho.len() - 1],
                    );
                    TypeChirho::FunChirho {
                        arg_chirho: Box::new(arg_chirho),
                        result_chirho: Box::new(result_chirho),
                        span_chirho,
                    }
                } else {
                    self.placeholder_type_chirho()
                }
            }
            SyntaxKindChirho::AppTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();
                if type_nodes_chirho.is_empty() {
                    return self.placeholder_type_chirho();
                }
                let mut result_chirho = self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                for tc_chirho in &type_nodes_chirho[1..] {
                    let arg_chirho = self.lower_type_from_child_chirho(tc_chirho);
                    result_chirho = TypeChirho::AppChirho {
                        fun_chirho: Box::new(result_chirho),
                        arg_chirho: Box::new(arg_chirho),
                        span_chirho,
                    };
                }
                result_chirho
            }
            SyntaxKindChirho::VarTypeChirho => {
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                TypeChirho::VarChirho(name_chirho)
            }
            SyntaxKindChirho::ConTypeChirho => {
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                TypeChirho::ConChirho(name_chirho)
            }
            SyntaxKindChirho::ParenTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                // Check for tuple type (multiple types separated by commas)
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();
                if type_nodes_chirho.len() > 1 {
                    let elements_chirho: Vec<_> = type_nodes_chirho
                        .iter()
                        .map(|tc_chirho| self.lower_type_from_child_chirho(tc_chirho))
                        .collect();
                    TypeChirho::TupleChirho {
                        elements_chirho,
                        span_chirho,
                    }
                } else if type_nodes_chirho.len() == 1 {
                    let inner_chirho = self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                    TypeChirho::ParenChirho {
                        inner_chirho: Box::new(inner_chirho),
                        span_chirho,
                    }
                } else {
                    // Unit type ()
                    TypeChirho::TupleChirho {
                        elements_chirho: vec![],
                        span_chirho,
                    }
                }
            }
            SyntaxKindChirho::ListTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let type_node_chirho = children_chirho.iter().find(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                });
                if let Some(tc_chirho) = type_node_chirho {
                    let element_chirho = self.lower_type_from_child_chirho(tc_chirho);
                    TypeChirho::ListChirho {
                        element_chirho: Box::new(element_chirho),
                        span_chirho,
                    }
                } else {
                    // [] — list type constructor
                    TypeChirho::ConChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                        "[]",
                        span_chirho,
                    )))
                }
            }
            SyntaxKindChirho::QualTypeChirho => {
                // context => type — for now, lower as just the body type
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();
                if let Some(last_chirho) = type_nodes_chirho.last() {
                    self.lower_type_from_child_chirho(last_chirho)
                } else {
                    self.placeholder_type_chirho()
                }
            }
            _ => {
                // Fallback: try to extract a name
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                if name_chirho.text_chirho().chars().next().map_or(false, |c| c.is_uppercase()) {
                    TypeChirho::ConChirho(name_chirho)
                } else {
                    TypeChirho::VarChirho(name_chirho)
                }
            }
        }
    }

    fn lower_type_from_child_chirho(&self, child_chirho: &ChildChirho<'_>) -> TypeChirho {
        if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
            self.lower_type_chirho(n_chirho, child_chirho.start_chirho)
        } else {
            self.placeholder_type_chirho()
        }
    }

    // -----------------------------------------------------------------------
    // Expressions
    // -----------------------------------------------------------------------

    fn lower_expr_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ExprChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());

        match node_chirho.kind_chirho() {
            SyntaxKindChirho::NameExprChirho => {
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                if name_chirho
                    .text_chirho()
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_uppercase())
                {
                    ExprChirho::ConChirho(name_chirho)
                } else {
                    ExprChirho::VarChirho(name_chirho)
                }
            }
            SyntaxKindChirho::LiteralExprChirho => {
                let lit_chirho = self.lower_lit_chirho(node_chirho, base_chirho);
                ExprChirho::LitChirho(lit_chirho)
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
                let mut result_chirho =
                    self.lower_expr_from_child_chirho(expr_nodes_chirho[0]);
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
                // For now, left-associate them
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut exprs_chirho = Vec::new();
                let mut ops_chirho = Vec::new();

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            exprs_chirho.push(
                                self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                            );
                        }
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                                || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                ops_chirho
                                    .push(self.name_from_token_chirho(tok_chirho, s_chirho));
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
                let mut result_chirho = exprs_chirho.remove(0);
                for (i_chirho, expr_chirho) in exprs_chirho.into_iter().enumerate() {
                    let op_chirho = ops_chirho
                        .get(i_chirho)
                        .cloned()
                        .unwrap_or_else(|| self.dummy_name_chirho());
                    result_chirho = ExprChirho::InfixChirho {
                        left_chirho: Box::new(result_chirho),
                        op_chirho,
                        right_chirho: Box::new(expr_chirho),
                        span_chirho,
                    };
                }
                result_chirho
            }
            SyntaxKindChirho::LambdaExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut pats_chirho = Vec::new();
                let mut body_chirho = None;
                let mut saw_arrow_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho {
                                saw_arrow_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if !saw_arrow_chirho && is_pat_kind_chirho(n_chirho.kind_chirho()) {
                                pats_chirho.push(
                                    self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            } else if saw_arrow_chirho && body_chirho.is_none() {
                                body_chirho = Some(
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }

                ExprChirho::LamChirho {
                    pats_chirho,
                    body_chirho: Box::new(
                        body_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
                    ),
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
                                alts_chirho.push(
                                    self.lower_case_alt_chirho(n_chirho, child_chirho.start_chirho),
                                );
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
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut stmts_chirho = Vec::new();

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                        match n_chirho.kind_chirho() {
                            SyntaxKindChirho::DoStmtChirho => {
                                let expr_chirho =
                                    self.lower_first_expr_in_node_chirho(n_chirho, child_chirho.start_chirho);
                                stmts_chirho.push(StmtChirho::ExprChirho(expr_chirho));
                            }
                            SyntaxKindChirho::BindStmtChirho => {
                                stmts_chirho.push(self.lower_bind_stmt_chirho(
                                    n_chirho,
                                    child_chirho.start_chirho,
                                ));
                            }
                            SyntaxKindChirho::LetStmtChirho => {
                                stmts_chirho.push(StmtChirho::LetChirho {
                                    binds_chirho: vec![], // TODO
                                    span_chirho: self.span_chirho(
                                        child_chirho.start_chirho,
                                        child_chirho.end_chirho,
                                    ),
                                });
                            }
                            _ => {
                                // Bare expression node
                                let expr_chirho =
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho);
                                stmts_chirho.push(StmtChirho::ExprChirho(expr_chirho));
                            }
                        }
                    }
                }

                ExprChirho::DoChirho {
                    stmts_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::LetExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut body_chirho = None;

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                        if is_expr_kind_chirho(n_chirho.kind_chirho()) && body_chirho.is_none() {
                            // Last expression child is the body (after "in")
                            body_chirho =
                                Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                        }
                    }
                }

                ExprChirho::LetChirho {
                    binds_chirho: vec![], // TODO: lower let binds
                    body_chirho: Box::new(
                        body_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
                    ),
                    span_chirho,
                }
            }
            SyntaxKindChirho::ParenExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let expr_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(_))
                    })
                    .collect();
                if expr_nodes_chirho.len() == 1 {
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
            SyntaxKindChirho::ListExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
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
            SyntaxKindChirho::NegateExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let expr_node_chirho = children_chirho.iter().find(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(_))
                });
                let inner_chirho = expr_node_chirho
                    .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                    .unwrap_or_else(|| self.placeholder_expr_chirho());
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
            _ => self.placeholder_expr_chirho(),
        }
    }

    fn lower_expr_from_child_chirho(&self, child_chirho: &ChildChirho<'_>) -> ExprChirho {
        if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
            self.lower_expr_chirho(n_chirho, child_chirho.start_chirho)
        } else {
            self.placeholder_expr_chirho()
        }
    }

    fn lower_first_expr_in_node_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ExprChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                return self.lower_expr_chirho(n_chirho, child_chirho.start_chirho);
            }
        }
        self.placeholder_expr_chirho()
    }

    fn lower_case_alt_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> AltChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut pat_chirho = None;
        let mut rhs_chirho = None;
        let mut saw_arrow_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho {
                        saw_arrow_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if !saw_arrow_chirho && pat_chirho.is_none() {
                        pat_chirho =
                            Some(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_arrow_chirho && rhs_chirho.is_none() {
                        rhs_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        AltChirho {
            pat_chirho: pat_chirho
                .unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
            rhs_chirho: RhsChirho::UnguardedChirho(
                rhs_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
            ),
            where_binds_chirho: vec![],
            span_chirho,
        }
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
            pat_chirho: pat_chirho
                .unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
            expr_chirho: expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho()),
            span_chirho,
        }
    }

    fn lower_lit_chirho(&self, node_chirho: &GreenNodeChirho, base_chirho: usize) -> LitChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let span_chirho =
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::IntegerLiteralChirho => {
                        let val_chirho =
                            tok_chirho.text_chirho().parse::<i64>().unwrap_or(0);
                        return LitChirho::IntChirho(val_chirho, span_chirho);
                    }
                    TokenKindChirho::FloatLiteralChirho => {
                        let val_chirho =
                            tok_chirho.text_chirho().parse::<f64>().unwrap_or(0.0);
                        return LitChirho::FloatChirho(val_chirho, span_chirho);
                    }
                    TokenKindChirho::CharLiteralChirho => {
                        let text_chirho = tok_chirho.text_chirho();
                        let ch_chirho = text_chirho
                            .trim_matches('\'')
                            .chars()
                            .next()
                            .unwrap_or('\0');
                        return LitChirho::CharChirho(ch_chirho, span_chirho);
                    }
                    TokenKindChirho::StringLiteralChirho => {
                        let text_chirho = tok_chirho.text_chirho();
                        let inner_chirho = text_chirho
                            .strip_prefix('"')
                            .and_then(|s_chirho| s_chirho.strip_suffix('"'))
                            .unwrap_or(text_chirho)
                            .to_string();
                        return LitChirho::StringChirho(inner_chirho, span_chirho);
                    }
                    _ => {}
                }
            }
        }
        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO)
    }

    // -----------------------------------------------------------------------
    // Patterns
    // -----------------------------------------------------------------------

    fn lower_pat_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> PatChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());

        match node_chirho.kind_chirho() {
            SyntaxKindChirho::VarPatChirho => {
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                PatChirho::VarChirho(name_chirho)
            }
            SyntaxKindChirho::ConPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut con_name_chirho = None;
                let mut args_chirho = Vec::new();

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
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
                            if is_pat_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            args_chirho.push(
                                self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                            );
                        }
                        _ => {}
                    }
                }

                PatChirho::ConChirho {
                    con_chirho: con_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    args_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::LitPatChirho => {
                let lit_chirho = self.lower_lit_chirho(node_chirho, base_chirho);
                PatChirho::LitChirho(lit_chirho)
            }
            SyntaxKindChirho::WildcardPatChirho => PatChirho::WildcardChirho(span_chirho),
            SyntaxKindChirho::AsPatChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut name_chirho = None;
                let mut inner_chirho = None;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if name_chirho.is_none()
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
                        GreenElementChirho::NodeChirho(n_chirho)
                            if is_pat_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            if inner_chirho.is_none() {
                                inner_chirho = Some(
                                    self.lower_pat_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                        _ => {}
                    }
                }

                PatChirho::AsChirho {
                    name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    pattern_chirho: Box::new(
                        inner_chirho
                            .unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
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
                    PatChirho::TupleChirho {
                        elements_chirho: vec![],
                        span_chirho,
                    }
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
                                && tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                op_chirho =
                                    Some(self.name_from_token_chirho(tok_chirho, s_chirho));
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
                        left_chirho
                            .unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
                    ),
                    op_chirho: op_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    right_chirho: Box::new(
                        right_chirho
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

    // -----------------------------------------------------------------------
    // Name helpers
    // -----------------------------------------------------------------------

    fn name_from_text_chirho(&self, text_chirho: &str, span_chirho: SpanChirho) -> NameChirho {
        // Split qualified names like "Data.List.sort"
        if let Some(dot_pos_chirho) = text_chirho.rfind('.') {
            let qualifier_chirho = &text_chirho[..dot_pos_chirho];
            let local_chirho = &text_chirho[dot_pos_chirho + 1..];
            NameChirho::RawChirho(RawNameChirho::qualified_chirho(
                qualifier_chirho,
                local_chirho,
                span_chirho,
            ))
        } else {
            NameChirho::RawChirho(RawNameChirho::unqualified_chirho(text_chirho, span_chirho))
        }
    }

    fn name_from_token_chirho(
        &self,
        tok_chirho: &GreenTokenChirho,
        span_chirho: SpanChirho,
    ) -> NameChirho {
        self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho)
    }

    fn extract_name_from_node_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> NameChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::VarIdChirho
                    | TokenKindChirho::ConIdChirho
                    | TokenKindChirho::QualifiedConIdChirho
                    | TokenKindChirho::QualifiedVarIdChirho
                    | TokenKindChirho::VarSymChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::UnderscoreReservedIdChirho => {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return self.name_from_token_chirho(tok_chirho, span_chirho);
                    }
                    _ => {}
                }
            }
        }
        self.dummy_name_chirho()
    }

    fn dummy_name_chirho(&self) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "",
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn placeholder_type_chirho(&self) -> TypeChirho {
        TypeChirho::VarChirho(self.dummy_name_chirho())
    }

    fn placeholder_expr_chirho(&self) -> ExprChirho {
        ExprChirho::VarChirho(self.dummy_name_chirho())
    }
}

// ---------------------------------------------------------------------------
// Helper types and predicates
// ---------------------------------------------------------------------------

struct ChildChirho<'a> {
    element_chirho: &'a GreenElementChirho,
    start_chirho: usize,
    end_chirho: usize,
}

fn is_trivia_element_chirho(element_chirho: &GreenElementChirho) -> bool {
    match element_chirho {
        GreenElementChirho::TokenChirho(tok_chirho) => is_trivia_token_kind_chirho(tok_chirho.kind_chirho()),
        GreenElementChirho::NodeChirho(_) => false,
    }
}

fn is_type_kind_chirho(kind_chirho: SyntaxKindChirho) -> bool {
    matches!(
        kind_chirho,
        SyntaxKindChirho::FunTypeChirho
            | SyntaxKindChirho::AppTypeChirho
            | SyntaxKindChirho::ParenTypeChirho
            | SyntaxKindChirho::TupleTypeChirho
            | SyntaxKindChirho::ListTypeChirho
            | SyntaxKindChirho::VarTypeChirho
            | SyntaxKindChirho::ConTypeChirho
            | SyntaxKindChirho::QualTypeChirho
            | SyntaxKindChirho::ForallTypeChirho
            | SyntaxKindChirho::KindAnnotTypeChirho
            | SyntaxKindChirho::ContextChirho
    )
}

fn is_expr_kind_chirho(kind_chirho: SyntaxKindChirho) -> bool {
    matches!(
        kind_chirho,
        SyntaxKindChirho::AppExprChirho
            | SyntaxKindChirho::InfixExprChirho
            | SyntaxKindChirho::LambdaExprChirho
            | SyntaxKindChirho::LetExprChirho
            | SyntaxKindChirho::IfExprChirho
            | SyntaxKindChirho::CaseExprChirho
            | SyntaxKindChirho::DoExprChirho
            | SyntaxKindChirho::ParenExprChirho
            | SyntaxKindChirho::TupleExprChirho
            | SyntaxKindChirho::ListExprChirho
            | SyntaxKindChirho::ArithSeqExprChirho
            | SyntaxKindChirho::ListCompExprChirho
            | SyntaxKindChirho::SectionExprChirho
            | SyntaxKindChirho::TypeAnnotExprChirho
            | SyntaxKindChirho::NegateExprChirho
            | SyntaxKindChirho::RecordConExprChirho
            | SyntaxKindChirho::RecordUpdateExprChirho
            | SyntaxKindChirho::LiteralExprChirho
            | SyntaxKindChirho::NameExprChirho
    )
}

fn is_pat_kind_chirho(kind_chirho: SyntaxKindChirho) -> bool {
    matches!(
        kind_chirho,
        SyntaxKindChirho::ConPatChirho
            | SyntaxKindChirho::VarPatChirho
            | SyntaxKindChirho::LitPatChirho
            | SyntaxKindChirho::WildcardPatChirho
            | SyntaxKindChirho::AsPatChirho
            | SyntaxKindChirho::ParenPatChirho
            | SyntaxKindChirho::TuplePatChirho
            | SyntaxKindChirho::ListPatChirho
            | SyntaxKindChirho::NegPatChirho
            | SyntaxKindChirho::LazyPatChirho
            | SyntaxKindChirho::BangPatChirho
            | SyntaxKindChirho::RecordPatChirho
            | SyntaxKindChirho::InfixConPatChirho
    )
}

fn is_trivia_token_kind_chirho(kind_chirho: TokenKindChirho) -> bool {
    matches!(
        kind_chirho,
        TokenKindChirho::WhitespaceTriviaChirho
            | TokenKindChirho::LineCommentTriviaChirho
            | TokenKindChirho::BlockCommentTriviaChirho
            | TokenKindChirho::DocCommentTriviaChirho
            | TokenKindChirho::VirtualLeftBraceChirho
            | TokenKindChirho::VirtualRightBraceChirho
            | TokenKindChirho::VirtualSemicolonChirho
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::cst_parser_chirho::ParserChirho;

    fn parse_and_lower_chirho(source_chirho: &str) -> ModuleChirho {
        let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
        let green_chirho = parser_chirho.parse_chirho();
        lower_module_chirho(&green_chirho, file_id_chirho)
    }

    #[test]
    fn lower_module_name_chirho() {
        let module_chirho = parse_and_lower_chirho("module Foo where\n");
        assert_eq!(module_chirho.name_chirho.text_chirho(), "Foo");
    }

    #[test]
    fn lower_script_defaults_to_main_chirho() {
        let module_chirho = parse_and_lower_chirho("x = 1\n");
        assert_eq!(module_chirho.name_chirho.text_chirho(), "Main");
    }

    #[test]
    fn lower_data_decl_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Color = Red | Green | Blue\n");
        assert_eq!(module_chirho.decls_chirho.len(), 1);
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), "Color");
                assert_eq!(constructors_chirho.len(), 3);
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_fun_bind_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\nf x = x\n");
        assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "f")
        }));
    }

    #[test]
    fn lower_type_sig_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\nfoo :: Int -> String\n");
        assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
        }));
    }

    #[test]
    fn lower_import_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module M where\nimport Data.List\nimport qualified Data.Map as Map\n");
        assert_eq!(module_chirho.imports_chirho.len(), 2);
        assert_eq!(
            module_chirho.imports_chirho[0].module_chirho.text_chirho(),
            "List"
        );
        assert!(module_chirho.imports_chirho[1].qualified_chirho);
        assert_eq!(
            module_chirho.imports_chirho[1]
                .alias_chirho
                .as_ref()
                .unwrap()
                .text_chirho(),
            "Map"
        );
    }

    #[test]
    fn lower_type_alias_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\ntype Name = String\n");
        assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeAliasDeclChirho { name_chirho, .. } if name_chirho.text_chirho() == "Name")
        }));
    }

    #[test]
    fn lower_spans_are_nonzero_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module Foo where\ndata Bar = Baz\n");
        let span_chirho = module_chirho.span_chirho;
        assert!(span_chirho.end_chirho().as_usize_chirho() > 0);
    }
}
