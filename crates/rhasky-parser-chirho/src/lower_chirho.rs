// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # CST → AST lowering
//!
//! Walks the immutable green tree produced by the CST parser and builds
//! the typed AST (`ModuleChirho`). Trivia, virtual layout tokens, and
//! punctuation are discarded; only semantic content survives.

use std::collections::HashMap;
use std::sync::Arc;

use rhasky_ast_chirho::decl_chirho::{ClassMethodChirho, ConDeclChirho, DeclChirho, FieldDeclChirho, FixityChirho, ForeignDirectionChirho};
use rhasky_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, FieldAssignChirho, GuardedExprChirho, LocalBindChirho, MatchArmChirho,
    RhsChirho, StmtChirho,
};
use rhasky_ast_chirho::ty_chirho::ConstraintChirho;
use rhasky_ast_chirho::lit_chirho::LitChirho;
use rhasky_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho, ModuleChirho,
};
use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use rhasky_ast_chirho::pat_chirho::{PatChirho, PatFieldChirho};
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
    // Pragma extraction
    // -----------------------------------------------------------------------

    /// Walk all pragma tokens in the CST and extract LANGUAGE extension names.
    /// Pragma tokens have the text `{-# LANGUAGE Ext1, Ext2 #-}`.
    fn extract_pragma_extensions_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
    ) -> Vec<String> {
        let mut extensions_chirho = Vec::new();
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::PragmaChirho {
                    let text_chirho = tok_chirho.text_chirho();
                    // Strip {-# and #-}
                    let inner_chirho = text_chirho
                        .strip_prefix("{-#")
                        .and_then(|s_chirho| s_chirho.strip_suffix("#-}"))
                        .unwrap_or("")
                        .trim();
                    // Check for LANGUAGE pragma
                    if let Some(rest_chirho) = inner_chirho.strip_prefix("LANGUAGE") {
                        let rest_chirho = rest_chirho.trim();
                        for ext_chirho in rest_chirho.split(',') {
                            let ext_chirho = ext_chirho.trim();
                            if !ext_chirho.is_empty() {
                                extensions_chirho.push(ext_chirho.to_string());
                            }
                        }
                    }
                    // OPTIONS, INLINE, etc. — silently ignore for now
                }
            }
        }
        extensions_chirho
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

        // Merge consecutive FunBindChirho with the same name into a single
        // multi-equation binding.  In Haskell, `f pat1 = rhs1; f pat2 = rhs2`
        // must be grouped before further analysis.
        let decls_chirho = merge_fun_binds_chirho(decls_chirho);

        let end_chirho = start_chirho + root_chirho.text_len_chirho();
        // Extract LANGUAGE extensions from pragma tokens
        let extensions_chirho = self.extract_pragma_extensions_chirho(root_chirho);

        ModuleChirho {
            name_chirho: module_name_chirho,
            exports_chirho: None, // TODO: lower export list
            imports_chirho,
            decls_chirho,
            extensions_chirho,
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

        // Lower import specification if present
        let mut spec_chirho = None;
        let mut hiding_chirho = false;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                        && tok_chirho.text_chirho() == "hiding" =>
                {
                    hiding_chirho = true;
                }
                GreenElementChirho::NodeChirho(n_chirho)
                    if n_chirho.kind_chirho() == SyntaxKindChirho::ImportSpecListChirho =>
                {
                    let items_chirho = self.lower_import_spec_list_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                    );
                    spec_chirho = Some(ImportSpecChirho {
                        hiding_chirho,
                        items_chirho,
                    });
                }
                _ => {}
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
            spec_chirho,
            span_chirho,
        }
    }

    fn lower_import_spec_list_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<ImportItemChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut items_chirho = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::ImportSpecChirho {
                    if let Some(item_chirho) =
                        self.lower_import_spec_item_chirho(n_chirho, child_chirho.start_chirho)
                    {
                        items_chirho.push(item_chirho);
                    }
                }
            }
        }

        items_chirho
    }

    fn lower_import_spec_item_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<ImportItemChirho> {
        let mut first_name_chirho: Option<(String, SpanChirho, bool)> = None; // (text, span, is_con)
        let mut has_parens_chirho = false;
        let mut has_dotdot_chirho = false;
        let mut members_chirho: Vec<NameChirho> = Vec::new();
        let mut in_parens_chirho = false;

        let mut offset_chirho = base_chirho;
        for elem_chirho in node_chirho.children_chirho() {
            let elem_start_chirho = offset_chirho;
            let elem_end_chirho = offset_chirho + elem_chirho.text_len_chirho();
            match elem_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    match tok_chirho.kind_chirho() {
                        TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                            let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                            let is_con_chirho = true;
                            if first_name_chirho.is_none() && !in_parens_chirho {
                                first_name_chirho = Some((
                                    tok_chirho.text_chirho().to_string(),
                                    span_chirho,
                                    is_con_chirho,
                                ));
                            } else if in_parens_chirho {
                                members_chirho.push(self.name_from_text_chirho(
                                    tok_chirho.text_chirho(),
                                    span_chirho,
                                ));
                            }
                        }
                        TokenKindChirho::VarIdChirho => {
                            let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                            if first_name_chirho.is_none() && !in_parens_chirho {
                                first_name_chirho = Some((
                                    tok_chirho.text_chirho().to_string(),
                                    span_chirho,
                                    false,
                                ));
                            } else if in_parens_chirho {
                                members_chirho.push(self.name_from_text_chirho(
                                    tok_chirho.text_chirho(),
                                    span_chirho,
                                ));
                            }
                        }
                        TokenKindChirho::VarSymChirho | TokenKindChirho::ConSymChirho => {
                            let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                            if first_name_chirho.is_none() && !in_parens_chirho {
                                first_name_chirho = Some((
                                    tok_chirho.text_chirho().to_string(),
                                    span_chirho,
                                    false,
                                ));
                            } else if in_parens_chirho {
                                members_chirho.push(self.name_from_text_chirho(
                                    tok_chirho.text_chirho(),
                                    span_chirho,
                                ));
                            }
                        }
                        TokenKindChirho::LeftParenChirho => {
                            has_parens_chirho = true;
                            in_parens_chirho = true;
                        }
                        TokenKindChirho::RightParenChirho => {
                            in_parens_chirho = false;
                        }
                        TokenKindChirho::DotDotChirho => {
                            has_dotdot_chirho = true;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            offset_chirho = elem_end_chirho;
        }

        let (text_chirho, span_chirho, is_con_chirho) = first_name_chirho?;
        let name_chirho = self.name_from_text_chirho(&text_chirho, span_chirho);

        if is_con_chirho && has_parens_chirho {
            // Type/class with members: T(..) or T(Con1, Con2)
            let members_spec_chirho = if has_dotdot_chirho {
                ExportMembersChirho::AllChirho
            } else if members_chirho.is_empty() {
                ExportMembersChirho::NoneChirho
            } else {
                ExportMembersChirho::SomeChirho(members_chirho)
            };
            Some(ImportItemChirho::TyConChirho {
                name_chirho,
                members_chirho: members_spec_chirho,
            })
        } else if is_con_chirho {
            // Type/class with no parens — just the name, no members
            Some(ImportItemChirho::TyConChirho {
                name_chirho,
                members_chirho: ExportMembersChirho::NoneChirho,
            })
        } else {
            // Variable
            Some(ImportItemChirho::VarChirho(name_chirho))
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

    /// Lower a `PatBindChirho` CST node into a `LocalBindChirho::PatBindChirho`.
    /// Structure: pattern = rhs
    fn lower_pat_bind_local_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> LocalBindChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut pat_chirho = None;
        let mut rhs_expr_chirho = None;
        let mut past_eq_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho =>
                {
                    past_eq_chirho = true;
                }
                GreenElementChirho::NodeChirho(n_chirho) if !past_eq_chirho => {
                    // Before '=': the pattern
                    if pat_chirho.is_none() {
                        pat_chirho = Some(self.lower_pat_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                        ));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) if past_eq_chirho => {
                    // After '=': the RHS expression
                    if rhs_expr_chirho.is_none() {
                        if is_expr_kind_chirho(n_chirho.kind_chirho()) {
                            rhs_expr_chirho = Some(self.lower_expr_chirho(
                                n_chirho,
                                child_chirho.start_chirho,
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        let pat_final_chirho =
            pat_chirho.unwrap_or(PatChirho::WildcardChirho(span_chirho));
        let rhs_final_chirho = match rhs_expr_chirho {
            Some(expr_chirho) => RhsChirho::UnguardedChirho(expr_chirho),
            None => RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                LitChirho::IntChirho(0, span_chirho),
            )),
        };

        LocalBindChirho::PatBindChirho {
            pat_chirho: pat_final_chirho,
            rhs_chirho: rhs_final_chirho,
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

        // The FunBind node contains Match children and an optional
        // WhereClause sibling; the function name is the first VarId or
        // ConId token inside the first Match.
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::MatchChirho {
                    if name_chirho.is_none() {
                        name_chirho =
                            self.extract_fun_name_from_match_chirho(n_chirho, child_chirho.start_chirho);
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
        let mut guarded_rhs_chirho: Option<Vec<GuardedExprChirho>> = None;
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
                        guarded_rhs_chirho = Some(self.lower_guarded_rhs_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                        ));
                    } else if !saw_equals_chirho && is_pat_kind_chirho(n_chirho.kind_chirho()) {
                        pats_chirho.push(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_equals_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                    {
                        where_binds_chirho = self.lower_where_clause_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                        );
                    } else if saw_equals_chirho && rhs_expr_chirho.is_none() {
                        rhs_expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        let rhs_chirho = if let Some(guards_chirho) = guarded_rhs_chirho {
            RhsChirho::GuardedChirho(guards_chirho)
        } else {
            RhsChirho::UnguardedChirho(
                rhs_expr_chirho.unwrap_or(self.placeholder_expr_chirho()),
            )
        };

        MatchArmChirho {
            pats_chirho,
            rhs_chirho,
            where_binds_chirho,
            span_chirho,
        }
    }

    /// Lower a `GuardedRhsChirho` CST node into a list of guarded expressions.
    fn lower_guarded_rhs_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<GuardedExprChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut guards_chirho = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::GuardChirho {
                    if let Some(ge_chirho) =
                        self.lower_guard_chirho(n_chirho, child_chirho.start_chirho)
                    {
                        guards_chirho.push(ge_chirho);
                    }
                }
            }
        }

        guards_chirho
    }

    /// Lower a single `GuardChirho` CST node into a `GuardedExprChirho`.
    /// Structure: `| guard_expr = body_expr`
    fn lower_guard_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<GuardedExprChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut guard_expr_chirho = None;
        let mut body_expr_chirho = None;
        let mut saw_equals_chirho = false;
        let mut saw_pipe_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho {
                        saw_pipe_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_pipe_chirho && !saw_equals_chirho {
                        // Token-level guard expression (e.g., `otherwise`, `True`)
                        if guard_expr_chirho.is_none() {
                            let s_chirho = self.span_chirho(
                                child_chirho.start_chirho,
                                child_chirho.end_chirho,
                            );
                            if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                                || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            {
                                let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                                guard_expr_chirho = Some(
                                    if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho {
                                        ExprChirho::ConChirho(name_chirho)
                                    } else {
                                        ExprChirho::VarChirho(name_chirho)
                                    },
                                );
                            }
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_pipe_chirho && !saw_equals_chirho && guard_expr_chirho.is_none() {
                        guard_expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_equals_chirho && body_expr_chirho.is_none() {
                        body_expr_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        Some(GuardedExprChirho {
            guard_chirho: guard_expr_chirho.unwrap_or(self.placeholder_expr_chirho()),
            body_chirho: body_expr_chirho.unwrap_or(self.placeholder_expr_chirho()),
            span_chirho,
        })
    }

    /// Lower a `WhereClauseChirho` CST node into a list of local bindings.
    /// The node structure is: `where { decl ; decl ; ... }`
    fn lower_where_clause_chirho(
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
                    {
                        if !matches_chirho.is_empty() {
                            binds_chirho.push(LocalBindChirho::FunBindChirho {
                                name_chirho,
                                matches_chirho,
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
    fn lower_let_stmt_binds_chirho(
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
                    {
                        if !matches_chirho.is_empty() {
                            binds_chirho.push(LocalBindChirho::FunBindChirho {
                                name_chirho,
                                matches_chirho,
                                span_chirho,
                            });
                        }
                    }
                }
            }
        }

        merge_local_fun_binds_chirho(binds_chirho)
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
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::GadtConDeclChirho {
                        constructors_chirho.push(
                            self.lower_gadt_con_decl_chirho(n_chirho, child_chirho.start_chirho),
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
        let mut record_fields_chirho: Vec<FieldDeclChirho> = Vec::new();

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
                        record_fields_chirho =
                            self.lower_record_fields_chirho(n_chirho, child_chirho.start_chirho);
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
                fields_chirho: record_fields_chirho,
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

    /// Lower a GADT constructor `Con :: forall a. Ctx => Arg -> ... -> T a`
    /// to a `ConDeclChirho::OrdinaryChirho` by extracting argument types from
    /// the function type signature (dropping the return type).
    fn lower_gadt_con_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ConDeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut name_chirho = None;
        let mut sig_type_chirho = None;

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
                GreenElementChirho::NodeChirho(n_chirho)
                    if is_type_kind_chirho(n_chirho.kind_chirho()) =>
                {
                    if sig_type_chirho.is_none() {
                        sig_type_chirho = Some(
                            self.lower_type_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    }
                }
                _ => {}
            }
        }

        // Extract argument types from function type: A -> B -> T a → [A, B]
        let mut arg_types_chirho = Vec::new();
        if let Some(ty_chirho) = sig_type_chirho {
            Self::extract_fun_args_chirho(&ty_chirho, &mut arg_types_chirho);
        }

        ConDeclChirho::OrdinaryChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            fields_chirho: arg_types_chirho,
            span_chirho,
        }
    }

    /// Extract argument types from a function type, discarding the final return type.
    /// `A -> B -> C` → `[A, B]` (C is the return type)
    fn extract_fun_args_chirho(ty_chirho: &TypeChirho, out_chirho: &mut Vec<TypeChirho>) {
        match ty_chirho {
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                out_chirho.push((**arg_chirho).clone());
                Self::extract_fun_args_chirho(result_chirho, out_chirho);
            }
            TypeChirho::ForallChirho { body_chirho, .. } => {
                Self::extract_fun_args_chirho(body_chirho, out_chirho);
            }
            TypeChirho::QualChirho { body_chirho, .. } => {
                Self::extract_fun_args_chirho(body_chirho, out_chirho);
            }
            // The final non-function type is the return type — discard it
            _ => {}
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
        let mut saw_pipe_chirho = false;

        // Fundep parsing state: after `|`, collect `a b -> c d, e -> f`
        let mut fundeps_chirho: Vec<(Vec<String>, Vec<String>)> = Vec::new();
        let mut fundep_from_chirho: Vec<String> = Vec::new();
        let mut fundep_to_chirho: Vec<String> = Vec::new();
        let mut in_to_chirho = false; // true after seeing `->`

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::ClassKeywordChirho {
                    saw_class_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::WhereKeywordChirho {
                    saw_where_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                    && saw_class_chirho
                    && !saw_where_chirho
                {
                    saw_pipe_chirho = true;
                } else if saw_pipe_chirho && !saw_where_chirho {
                    // Parsing functional dependencies
                    match tok_chirho.kind_chirho() {
                        TokenKindChirho::VarIdChirho => {
                            let var_name_chirho = tok_chirho.text_chirho().to_string();
                            if in_to_chirho {
                                fundep_to_chirho.push(var_name_chirho);
                            } else {
                                fundep_from_chirho.push(var_name_chirho);
                            }
                        }
                        TokenKindChirho::RightArrowChirho => {
                            in_to_chirho = true;
                        }
                        TokenKindChirho::CommaChirho => {
                            // Flush current fundep
                            if !fundep_from_chirho.is_empty() || !fundep_to_chirho.is_empty() {
                                fundeps_chirho.push((
                                    std::mem::take(&mut fundep_from_chirho),
                                    std::mem::take(&mut fundep_to_chirho),
                                ));
                            }
                            in_to_chirho = false;
                        }
                        _ => {}
                    }
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

        // Flush last fundep if any
        if !fundep_from_chirho.is_empty() || !fundep_to_chirho.is_empty() {
            fundeps_chirho.push((fundep_from_chirho, fundep_to_chirho));
        }

        // Extract method signatures (and default implementations) from the
        // where block.  The CST where-clause contains TypeSig and FunBind
        // children — we reuse `collect_instance_methods_chirho` to gather them.
        let mut where_decls_chirho: Vec<DeclChirho> = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if saw_where_chirho
                    || n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                {
                    self.collect_instance_methods_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        &mut where_decls_chirho,
                    );
                }
            }
        }

        // Merge multi-equation default method implementations.
        let where_decls_chirho = merge_fun_binds_chirho(where_decls_chirho);

        // Pair type signatures with optional default implementations.
        let mut default_impls_chirho: HashMap<String, Vec<MatchArmChirho>> = HashMap::new();
        for d_chirho in &where_decls_chirho {
            if let DeclChirho::FunBindChirho {
                name_chirho: m_name_chirho,
                matches_chirho,
                ..
            } = d_chirho
            {
                default_impls_chirho.insert(
                    m_name_chirho.text_chirho().to_string(),
                    matches_chirho.clone(),
                );
            }
        }

        let methods_chirho: Vec<ClassMethodChirho> = where_decls_chirho
            .into_iter()
            .filter_map(|d_chirho| match d_chirho {
                DeclChirho::TypeSigChirho {
                    name_chirho: sig_name_chirho,
                    ty_chirho: sig_ty_chirho,
                    span_chirho: sig_span_chirho,
                } => {
                    let default_chirho = default_impls_chirho
                        .get(sig_name_chirho.text_chirho())
                        .cloned();
                    Some(ClassMethodChirho {
                        name_chirho: sig_name_chirho,
                        ty_chirho: sig_ty_chirho,
                        default_chirho,
                        span_chirho: sig_span_chirho,
                    })
                }
                _ => None,
            })
            .collect();

        DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            methods_chirho,
            fundeps_chirho,
            span_chirho,
        }
    }

    /// Lower an instance declaration.
    ///
    /// ```haskell
    /// instance Eq a => Eq (Maybe a) where
    ///   (==) (Just x) (Just y) = x == y
    ///   (==) Nothing  Nothing  = True
    ///   (==) _        _        = False
    /// ```
    ///
    /// Tokens between `instance` and `where`:
    ///   - If `=>` is present, everything before it is the context,
    ///     everything after is class + types.
    ///   - Otherwise everything is class + types.
    ///
    /// The where block contains FunBind nodes that become LocalBindChirho.
    fn lower_instance_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        // Collect all ConId and VarId tokens between `instance` and `where`.
        // If `=>` appears, tokens before it form the context.
        let mut saw_instance_chirho = false;
        let mut saw_where_chirho = false;
        let mut saw_fat_arrow_chirho = false;

        // Tokens before `=>` (if any).
        let mut pre_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();
        // Tokens after `=>` (or all of them if no `=>`).
        let mut post_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();

        // Where-block sub-nodes (FunBind children inside WhereClause).
        let mut method_decls_chirho: Vec<DeclChirho> = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::InstanceKeywordChirho {
                        saw_instance_chirho = true;
                        continue;
                    }
                    if kind_chirho == TokenKindChirho::WhereKeywordChirho {
                        saw_where_chirho = true;
                        continue;
                    }
                    if kind_chirho == TokenKindChirho::DoubleArrowChirho {
                        saw_fat_arrow_chirho = true;
                        continue;
                    }
                    if saw_instance_chirho && !saw_where_chirho {
                        let s_chirho = self
                            .span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if saw_fat_arrow_chirho {
                            post_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                        } else {
                            pre_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_where_chirho || n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho {
                        // Recurse into the WhereClause to find FunBind children.
                        self.collect_instance_methods_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                            &mut method_decls_chirho,
                        );
                    } else if saw_instance_chirho && !saw_where_chirho {
                        // Type application node (e.g., parenthesized type).
                        // For now we skip sub-nodes in the head; they would
                        // need full type lowering.  Simple cases are covered
                        // by the token scan above.
                    }
                }
            }
        }

        // If no `=>` was seen, everything is class + types.
        let head_tokens_chirho = if saw_fat_arrow_chirho {
            &post_arrow_tokens_chirho
        } else {
            &pre_arrow_tokens_chirho
        };

        // Build context from pre-arrow tokens (simplified: "ClassName varName" pairs).
        let context_chirho = if saw_fat_arrow_chirho {
            self.build_instance_context_chirho(&pre_arrow_tokens_chirho)
        } else {
            vec![]
        };

        // First ConId in head_tokens is the class name; remaining are type args.
        // Handles simple types (VarId, ConId) and list types ([VarId], [ConId]).
        let mut class_chirho = None;
        let mut types_chirho = Vec::new();

        let mut idx_chirho = 0usize;
        while idx_chirho < head_tokens_chirho.len() {
            let (tok_chirho, s_chirho) = &head_tokens_chirho[idx_chirho];
            match tok_chirho.kind_chirho() {
                TokenKindChirho::ConIdChirho => {
                    if class_chirho.is_none() {
                        class_chirho =
                            Some(self.name_from_token_chirho(tok_chirho, *s_chirho));
                    } else {
                        types_chirho.push(TypeChirho::ConChirho(
                            self.name_from_token_chirho(tok_chirho, *s_chirho),
                        ));
                    }
                    idx_chirho += 1;
                }
                TokenKindChirho::VarIdChirho => {
                    types_chirho.push(TypeChirho::VarChirho(
                        self.name_from_token_chirho(tok_chirho, *s_chirho),
                    ));
                    idx_chirho += 1;
                }
                TokenKindChirho::LeftBracketChirho => {
                    // Parse [type] as a list type
                    if idx_chirho + 2 < head_tokens_chirho.len() {
                        let (inner_tok_chirho, inner_s_chirho) =
                            &head_tokens_chirho[idx_chirho + 1];
                        let (close_tok_chirho, _) = &head_tokens_chirho[idx_chirho + 2];
                        if close_tok_chirho.kind_chirho()
                            == TokenKindChirho::RightBracketChirho
                        {
                            let inner_type_chirho = match inner_tok_chirho.kind_chirho() {
                                TokenKindChirho::VarIdChirho => TypeChirho::VarChirho(
                                    self.name_from_token_chirho(
                                        inner_tok_chirho,
                                        *inner_s_chirho,
                                    ),
                                ),
                                TokenKindChirho::ConIdChirho => TypeChirho::ConChirho(
                                    self.name_from_token_chirho(
                                        inner_tok_chirho,
                                        *inner_s_chirho,
                                    ),
                                ),
                                _ => {
                                    idx_chirho += 1;
                                    continue;
                                }
                            };
                            types_chirho.push(TypeChirho::ListChirho {
                                element_chirho: Box::new(inner_type_chirho),
                                span_chirho,
                            });
                            idx_chirho += 3; // skip [, inner, ]
                            continue;
                        }
                    }
                    idx_chirho += 1;
                }
                _ => {
                    idx_chirho += 1;
                }
            }
        }

        // For multi-parameter type classes, keep each type argument as a
        // separate element so that the type checker can split them into
        // head_ty_chirho (first) and extra_head_tys_chirho (rest).
        let instance_type_chirho = types_chirho;

        // Merge consecutive multi-equation methods in instance declarations.
        let method_decls_chirho = merge_fun_binds_chirho(method_decls_chirho);
        // Convert method DeclChirho::FunBindChirho into LocalBindChirho::FunBindChirho.
        let methods_chirho: Vec<LocalBindChirho> = method_decls_chirho
            .into_iter()
            .filter_map(|d_chirho| match d_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => Some(LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                }),
                _ => None,
            })
            .collect();

        DeclChirho::InstanceDeclChirho {
            context_chirho,
            class_chirho: class_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            types_chirho: instance_type_chirho,
            methods_chirho,
            span_chirho,
        }
    }

    /// Build a simple context from pre-arrow tokens.
    /// Assumes pattern: `ClassName varName [, ClassName varName ...]` or
    /// `(ClassName varName, ...) =>`.
    fn build_instance_context_chirho(
        &self,
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
    ) -> Vec<ConstraintChirho> {
        let mut constraints_chirho = Vec::new();
        let mut current_class_chirho: Option<NameChirho> = None;
        let mut current_args_chirho: Vec<TypeChirho> = Vec::new();

        for (tok_chirho, s_chirho) in tokens_chirho {
            match tok_chirho.kind_chirho() {
                TokenKindChirho::ConIdChirho => {
                    // If we already have a class, flush it.
                    if let Some(cls_chirho) = current_class_chirho.take() {
                        constraints_chirho.push(ConstraintChirho {
                            class_chirho: cls_chirho,
                            args_chirho: std::mem::take(&mut current_args_chirho),
                            span_chirho: *s_chirho,
                        });
                    }
                    current_class_chirho =
                        Some(self.name_from_token_chirho(tok_chirho, *s_chirho));
                }
                TokenKindChirho::VarIdChirho => {
                    current_args_chirho.push(TypeChirho::VarChirho(
                        self.name_from_token_chirho(tok_chirho, *s_chirho),
                    ));
                }
                // Skip parens, commas, etc.
                _ => {}
            }
        }

        // Flush the last constraint.
        if let Some(cls_chirho) = current_class_chirho {
            constraints_chirho.push(ConstraintChirho {
                class_chirho: cls_chirho,
                args_chirho: current_args_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            });
        }

        constraints_chirho
    }

    /// Collect FunBind (and other decl) children from a WhereClause or similar
    /// node, lowering each into a DeclChirho and appending to `out_chirho`.
    fn collect_instance_methods_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        out_chirho: &mut Vec<DeclChirho>,
    ) {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if let Some(decl_chirho) =
                    self.lower_decl_chirho(n_chirho, child_chirho.start_chirho)
                {
                    out_chirho.push(decl_chirho);
                }
            }
        }
    }

    /// Lower a default declaration: `default (Int, Double)`.
    fn lower_default_decl_chirho(
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

    /// Lower a `foreign import/export` declaration from CST to AST.
    ///
    /// Expected token sequence (all trivia already filtered):
    ///   foreign  import|export  callconv  [safety]  ["c_name"]  hsName  ::  type...
    fn lower_foreign_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut direction_chirho = ForeignDirectionChirho::ImportChirho;
        let mut calling_conv_chirho = String::from("ccall");
        let mut safety_chirho: Option<String> = None;
        let mut foreign_name_chirho: Option<String> = None;
        let mut hs_name_chirho: Option<NameChirho> = None;
        let mut double_colon_idx_chirho: Option<usize> = None;

        // Phase tracking: 0=direction, 1=callconv, 2=safety/name, 3=hsName, 4=done
        let mut phase_chirho: u8 = 0;

        for (idx_chirho, child_chirho) in children_chirho.iter().enumerate() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let kind_chirho = tok_chirho.kind_chirho();
                let text_chirho = tok_chirho.text_chirho();
                let s_chirho =
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);

                if kind_chirho == TokenKindChirho::DoubleColonChirho {
                    double_colon_idx_chirho = Some(idx_chirho);
                    break;
                }

                match kind_chirho {
                    TokenKindChirho::ForeignKeywordChirho => {}
                    TokenKindChirho::ImportKeywordChirho => {
                        direction_chirho = ForeignDirectionChirho::ImportChirho;
                        phase_chirho = 1;
                    }
                    TokenKindChirho::VarIdChirho
                        if phase_chirho == 0 && text_chirho == "export" =>
                    {
                        direction_chirho = ForeignDirectionChirho::ExportChirho;
                        phase_chirho = 1;
                    }
                    TokenKindChirho::VarIdChirho if phase_chirho == 1 => {
                        match text_chirho {
                            "ccall" | "capi" | "stdcall" | "prim" | "javascript"
                            | "cplusplus" => {
                                calling_conv_chirho = text_chirho.to_string();
                                phase_chirho = 2;
                            }
                            _ => {
                                hs_name_chirho =
                                    Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                                phase_chirho = 4;
                            }
                        }
                    }
                    TokenKindChirho::VarIdChirho if phase_chirho == 2 => {
                        match text_chirho {
                            "safe" | "unsafe" | "interruptible" => {
                                safety_chirho = Some(text_chirho.to_string());
                                phase_chirho = 3;
                            }
                            _ => {
                                hs_name_chirho =
                                    Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                                phase_chirho = 4;
                            }
                        }
                    }
                    TokenKindChirho::StringLiteralChirho
                        if phase_chirho == 2 || phase_chirho == 3 =>
                    {
                        let raw_chirho = text_chirho;
                        let trimmed_chirho = raw_chirho
                            .strip_prefix('"')
                            .unwrap_or(raw_chirho)
                            .strip_suffix('"')
                            .unwrap_or(raw_chirho);
                        foreign_name_chirho = Some(trimmed_chirho.to_string());
                        phase_chirho = 3;
                    }
                    TokenKindChirho::VarIdChirho | TokenKindChirho::ConIdChirho
                        if phase_chirho == 3 || (phase_chirho >= 4 && hs_name_chirho.is_none()) =>
                    {
                        hs_name_chirho =
                            Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        phase_chirho = 4;
                    }
                    _ => {}
                }
            }
        }

        // Build the type from children after ::
        let ty_chirho = if let Some(dc_idx_chirho) = double_colon_idx_chirho {
            self.lower_foreign_type_chirho(&children_chirho[(dc_idx_chirho + 1)..], span_chirho)
        } else {
            self.placeholder_type_chirho()
        };

        DeclChirho::ForeignDeclChirho {
            direction_chirho,
            name_chirho: hs_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            ty_chirho,
            calling_conv_chirho,
            safety_chirho,
            foreign_name_chirho,
            span_chirho,
        }
    }

    /// Build a type from the children after `::` in a foreign declaration.
    fn lower_foreign_type_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> TypeChirho {
        let mut types_chirho: Vec<TypeChirho> = Vec::new();
        let mut arrow_positions_chirho: Vec<usize> = Vec::new();

        for child_chirho in children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    match tok_chirho.kind_chirho() {
                        TokenKindChirho::ConIdChirho
                        | TokenKindChirho::QualifiedConIdChirho => {
                            let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                            types_chirho.push(TypeChirho::ConChirho(name_chirho));
                        }
                        TokenKindChirho::VarIdChirho => {
                            let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                            types_chirho.push(TypeChirho::VarChirho(name_chirho));
                        }
                        TokenKindChirho::RightArrowChirho => {
                            arrow_positions_chirho.push(types_chirho.len());
                        }
                        _ => {}
                    }
                }
                GreenElementChirho::NodeChirho(sub_node_chirho) => {
                    types_chirho.push(
                        self.lower_type_chirho(sub_node_chirho, child_chirho.start_chirho),
                    );
                }
            }
        }

        if types_chirho.is_empty() {
            return self.placeholder_type_chirho();
        }

        if !arrow_positions_chirho.is_empty() && types_chirho.len() >= 2 {
            // Build right-associative function type.
            let mut result_chirho = types_chirho.pop().unwrap();
            while let Some(arg_chirho) = types_chirho.pop() {
                result_chirho = TypeChirho::FunChirho {
                    arg_chirho: Box::new(arg_chirho),
                    result_chirho: Box::new(result_chirho),
                    span_chirho,
                };
            }
            result_chirho
        } else if types_chirho.len() == 1 {
            types_chirho.pop().unwrap()
        } else {
            // Type application: T a b → App(App(T, a), b)
            let mut result_chirho = types_chirho.remove(0);
            for arg_chirho in types_chirho {
                result_chirho = TypeChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(arg_chirho),
                    span_chirho,
                };
            }
            result_chirho
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
            SyntaxKindChirho::ForallTypeChirho => {
                // forall a b . Type
                // Children: ForallKeyword, VarId*, VarSym("."), type nodes
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut vars_chirho = Vec::new();
                let mut saw_dot_chirho = false;
                let mut body_chirho = None;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            let kind_chirho = tok_chirho.kind_chirho();
                            if kind_chirho == TokenKindChirho::VarIdChirho && !saw_dot_chirho {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                vars_chirho.push(
                                    self.name_from_token_chirho(tok_chirho, s_chirho),
                                );
                            } else if kind_chirho == TokenKindChirho::VarSymChirho
                                && tok_chirho.text_chirho() == "."
                            {
                                saw_dot_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho)
                            if is_type_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            if body_chirho.is_none() && saw_dot_chirho {
                                body_chirho = Some(
                                    self.lower_type_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                        _ => {}
                    }
                }

                TypeChirho::ForallChirho {
                    vars_chirho,
                    body_chirho: Box::new(
                        body_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
                    ),
                    span_chirho,
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
                let children_chirho =
                    self.semantic_children_chirho(node_chirho, base_chirho);
                // Check for record construction: Con { f1 = e1, ... }
                let has_fields_chirho = children_chirho.iter().any(|c_chirho| {
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
                            {
                                if n_chirho.kind_chirho()
                                    == SyntaxKindChirho::FieldAssignChirho
                                {
                                    return Some(self.lower_field_assign_chirho(
                                        n_chirho,
                                        c_chirho.start_chirho,
                                    ));
                                }
                            }
                            None
                        })
                        .collect();
                    ExprChirho::RecordConChirho {
                        con_chirho: name_chirho,
                        fields_chirho,
                        span_chirho,
                    }
                } else if name_chirho
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
                // Resolve with operator precedence and associativity.
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut exprs_chirho: Vec<ExprChirho> = Vec::new();
                let mut ops_chirho: Vec<NameChirho> = Vec::new();

                let mut in_backtick_chirho = false;
                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            exprs_chirho.push(
                                self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                            );
                        }
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            if tok_chirho.kind_chirho() == TokenKindChirho::BacktickChirho {
                                in_backtick_chirho = !in_backtick_chirho;
                            } else if in_backtick_chirho
                                && (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                                    || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho)
                            {
                                // Backtick-enclosed name used as infix operator:
                                // `div`, `mod`, `elem`, etc.
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                ops_chirho
                                    .push(self.name_from_token_chirho(tok_chirho, s_chirho));
                            } else if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
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

                resolve_infix_precedence_chirho(exprs_chirho, ops_chirho, span_chirho)
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
                                {
                                    if alt_n_chirho.kind_chirho()
                                        == SyntaxKindChirho::CaseAltChirho
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
                let fresh_var_chirho = ExprChirho::VarChirho(fresh_name_chirho.clone());
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
                                    alts_chirho.push(
                                        self.lower_case_alt_chirho(n_chirho, child_chirho.start_chirho),
                                    );
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
                                let let_binds_chirho =
                                    self.lower_let_stmt_binds_chirho(
                                        n_chirho,
                                        child_chirho.start_chirho,
                                    );
                                stmts_chirho.push(StmtChirho::LetChirho {
                                    binds_chirho: let_binds_chirho,
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
                let mut binds_chirho: Vec<LocalBindChirho> = Vec::new();
                let mut body_chirho = None;
                let mut past_in_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho()
                                == TokenKindChirho::InKeywordChirho =>
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
                                {
                                    if !matches_chirho.is_empty() {
                                        binds_chirho.push(LocalBindChirho::FunBindChirho {
                                            name_chirho,
                                            matches_chirho,
                                            span_chirho,
                                        });
                                    }
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
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) if past_in_chirho => {
                            // After "in": the body expression
                            if body_chirho.is_none() {
                                if is_expr_kind_chirho(n_chirho.kind_chirho()) {
                                    body_chirho = Some(
                                        self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                    );
                                } else if n_chirho.kind_chirho()
                                    == SyntaxKindChirho::FunBindChirho
                                {
                                    // Parser may wrap body in FunBind > Match;
                                    // extract the expression from inside the match
                                    body_chirho = Some(
                                        self.lower_let_body_from_funbind_chirho(
                                            n_chirho,
                                            child_chirho.start_chirho,
                                        ),
                                    );
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
                                let expr_chirho =
                                    self.lower_expr_from_child_chirho(child_chirho);
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
                        let mut quals_chirho: Vec<StmtChirho> = Vec::new();

                        for child_chirho in &children_chirho {
                            match child_chirho.element_chirho {
                                GreenElementChirho::TokenChirho(tok_chirho)
                                    if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                                        && !past_pipe_chirho =>
                                {
                                    past_pipe_chirho = true;
                                }
                                GreenElementChirho::TokenChirho(tok_chirho)
                                    if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho
                                        && past_pipe_chirho =>
                                {
                                    // Flush current qualifier
                                    if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                        &current_qual_parts_chirho,
                                        span_chirho,
                                    ) {
                                        quals_chirho.push(qual_chirho);
                                    }
                                    current_qual_parts_chirho.clear();
                                }
                                GreenElementChirho::TokenChirho(tok_chirho)
                                    if tok_chirho.kind_chirho()
                                        == TokenKindChirho::LeftArrowChirho
                                        && past_pipe_chirho =>
                                {
                                    current_qual_parts_chirho
                                        .push(QualPartChirho::ArrowChirho);
                                }
                                GreenElementChirho::NodeChirho(_) => {
                                    let expr_chirho =
                                        self.lower_expr_from_child_chirho(child_chirho);
                                    if !past_pipe_chirho {
                                        body_expr_chirho = Some(expr_chirho);
                                    } else {
                                        current_qual_parts_chirho
                                            .push(QualPartChirho::ExprChirho(expr_chirho));
                                    }
                                }
                                _ => {}
                            }
                        }

                        // Flush last qualifier
                        if !current_qual_parts_chirho.is_empty() {
                            if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                &current_qual_parts_chirho,
                                span_chirho,
                            ) {
                                quals_chirho.push(qual_chirho);
                            }
                        }

                        let body_chirho = body_expr_chirho
                            .unwrap_or_else(|| self.placeholder_expr_chirho());

                        ExprChirho::ListCompChirho {
                            body_chirho: Box::new(body_chirho),
                            quals_chirho,
                            span_chirho,
                        }
                    } else {
                        let expr_nodes_chirho: Vec<_> = children_chirho
                            .iter()
                            .filter(|c_chirho| {
                                matches!(
                                    c_chirho.element_chirho,
                                    GreenElementChirho::NodeChirho(_)
                                )
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
            SyntaxKindChirho::RecordUpdateExprChirho => {
                // expr { f1 = e1, f2 = e2 }
                // Children: expression node(s), FieldAssignChirho nodes, braces/commas
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut base_expr_chirho = None;
                let mut fields_chirho = Vec::new();

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                        if n_chirho.kind_chirho() == SyntaxKindChirho::FieldAssignChirho {
                            fields_chirho.push(self.lower_field_assign_chirho(
                                n_chirho,
                                child_chirho.start_chirho,
                            ));
                        } else if base_expr_chirho.is_none()
                            && is_expr_kind_chirho(n_chirho.kind_chirho())
                        {
                            base_expr_chirho = Some(
                                self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                            );
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

    /// Lower a `RecordFieldsChirho` CST node to a vec of `FieldDeclChirho`.
    /// CST structure: LeftBrace FieldDeclChirho ... RightBrace
    fn lower_record_fields_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<FieldDeclChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut result_chirho = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::FieldDeclChirho {
                    result_chirho.push(
                        self.lower_field_decl_chirho(n_chirho, child_chirho.start_chirho),
                    );
                }
            }
        }
        result_chirho
    }

    /// Lower a `FieldDeclChirho` CST node to an AST `FieldDeclChirho`.
    /// CST structure: VarIdChirho [, VarIdChirho]* :: TypeChirho
    fn lower_field_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> FieldDeclChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut names_chirho = Vec::new();
        let mut ty_chirho: Option<TypeChirho> = None;
        let mut saw_double_colon_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho {
                        saw_double_colon_chirho = true;
                    } else if !saw_double_colon_chirho
                        && tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        names_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if saw_double_colon_chirho
                        && tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                        && ty_chirho.is_none()
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        ty_chirho = Some(TypeChirho::ConChirho(
                            self.name_from_token_chirho(tok_chirho, s_chirho),
                        ));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_double_colon_chirho && ty_chirho.is_none() {
                        ty_chirho = Some(
                            self.lower_type_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    }
                }
            }
        }

        FieldDeclChirho {
            names_chirho,
            ty_chirho: ty_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
            span_chirho,
        }
    }

    /// Lower a `FieldAssignChirho` CST node to an AST `FieldAssignChirho`.
    /// CST structure: VarIdChirho EqualsChirho <expr tokens>
    fn lower_field_assign_chirho(
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
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho)
                        && field_name_chirho.is_none()
                    {
                        field_name_chirho = Some(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                            tok_chirho.text_chirho().to_string(),
                            span_chirho,
                        )));
                    }
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    }
                    // After =, a literal token is a valid value
                    if saw_equals_chirho && value_chirho.is_none() {
                        match tok_chirho.kind_chirho() {
                            TokenKindChirho::IntegerLiteralChirho => {
                                let n_chirho = tok_chirho
                                    .text_chirho()
                                    .parse::<i64>()
                                    .unwrap_or(0);
                                value_chirho = Some(ExprChirho::LitChirho(
                                    LitChirho::IntChirho(n_chirho, span_chirho),
                                ));
                            }
                            TokenKindChirho::StringLiteralChirho => {
                                let raw_chirho = tok_chirho.text_chirho();
                                let s_chirho = if raw_chirho.starts_with('"')
                                    && raw_chirho.ends_with('"')
                                {
                                    raw_chirho[1..raw_chirho.len() - 1].to_string()
                                } else {
                                    raw_chirho.to_string()
                                };
                                value_chirho = Some(ExprChirho::LitChirho(
                                    LitChirho::StringChirho(s_chirho, span_chirho),
                                ));
                            }
                            TokenKindChirho::FloatLiteralChirho => {
                                let f_chirho = tok_chirho
                                    .text_chirho()
                                    .parse::<f64>()
                                    .unwrap_or(0.0);
                                value_chirho = Some(ExprChirho::LitChirho(
                                    LitChirho::FloatChirho(f_chirho, span_chirho),
                                ));
                            }
                            TokenKindChirho::CharLiteralChirho => {
                                let raw_chirho = tok_chirho.text_chirho();
                                let c_chirho = if raw_chirho.starts_with('\'')
                                    && raw_chirho.ends_with('\'')
                                    && raw_chirho.len() >= 3
                                {
                                    raw_chirho.chars().nth(1).unwrap_or(' ')
                                } else {
                                    ' '
                                };
                                value_chirho = Some(ExprChirho::LitChirho(
                                    LitChirho::CharChirho(c_chirho, span_chirho),
                                ));
                            }
                            TokenKindChirho::VarIdChirho => {
                                value_chirho = Some(ExprChirho::VarChirho(
                                    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                                        tok_chirho.text_chirho().to_string(),
                                        span_chirho,
                                    )),
                                ));
                            }
                            TokenKindChirho::ConIdChirho => {
                                value_chirho = Some(ExprChirho::ConChirho(
                                    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                                        tok_chirho.text_chirho().to_string(),
                                        span_chirho,
                                    )),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_equals_chirho && value_chirho.is_none() {
                        value_chirho = Some(
                            self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    }
                }
            }
        }

        FieldAssignChirho {
            name_chirho: field_name_chirho
                .unwrap_or_else(|| NameChirho::RawChirho(RawNameChirho::unqualified_chirho("_".to_string(), span_chirho))),
            value_chirho: value_chirho
                .unwrap_or_else(|| self.placeholder_expr_chirho()),
            span_chirho,
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
        let mut where_binds_chirho = Vec::new();
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
                    } else if saw_arrow_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                    {
                        where_binds_chirho = self.lower_where_clause_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                        );
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
            where_binds_chirho,
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
                        let inner_chirho = text_chirho.trim_matches('\'');
                        let ch_chirho = unescape_char_chirho(inner_chirho);
                        return LitChirho::CharChirho(ch_chirho, span_chirho);
                    }
                    TokenKindChirho::StringLiteralChirho => {
                        let text_chirho = tok_chirho.text_chirho();
                        let raw_chirho = text_chirho
                            .strip_prefix('"')
                            .and_then(|s_chirho| s_chirho.strip_suffix('"'))
                            .unwrap_or(text_chirho);
                        let inner_chirho = unescape_string_chirho(raw_chirho);
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
            SyntaxKindChirho::NegPatChirho => {
                // CST: NegPat = '-' apat   (where apat is typically a LitPat)
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut lit_chirho = None;
                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if n_chirho.kind_chirho() == SyntaxKindChirho::LitPatChirho
                                || is_pat_kind_chirho(n_chirho.kind_chirho())
                            {
                                let inner_lit_chirho =
                                    self.lower_lit_chirho(n_chirho, child_chirho.start_chirho);
                                lit_chirho = Some(inner_lit_chirho);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                let lit_chirho = lit_chirho.unwrap_or(LitChirho::IntChirho(0, span_chirho));
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
            SyntaxKindChirho::RecordPatChirho => {
                // Con { f1 = p1, f2 = p2 }
                // CST children are all flat tokens: ConId { VarId = VarId , VarId = VarId }
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut con_name_chirho = None;
                let mut fields_chirho = Vec::new();
                let mut current_field_name_chirho: Option<NameChirho> = None;
                let mut saw_equals_chirho = false;

                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            let kind_chirho = tok_chirho.kind_chirho();
                            if con_name_chirho.is_none()
                                && (kind_chirho == TokenKindChirho::ConIdChirho
                                    || kind_chirho == TokenKindChirho::QualifiedConIdChirho)
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                con_name_chirho = Some(
                                    self.name_from_token_chirho(tok_chirho, s_chirho),
                                );
                            } else if kind_chirho == TokenKindChirho::EqualsChirho {
                                saw_equals_chirho = true;
                            } else if kind_chirho == TokenKindChirho::VarIdChirho {
                                if saw_equals_chirho && current_field_name_chirho.is_some() {
                                    // VarId after '=' — this is the pattern variable
                                    let s_chirho = self.span_chirho(
                                        child_chirho.start_chirho,
                                        child_chirho.end_chirho,
                                    );
                                    let var_name_chirho =
                                        self.name_from_token_chirho(tok_chirho, s_chirho);
                                    let fname_chirho = current_field_name_chirho.take().unwrap();
                                    fields_chirho.push(PatFieldChirho {
                                        name_chirho: fname_chirho,
                                        pattern_chirho: PatChirho::VarChirho(var_name_chirho),
                                        span_chirho: s_chirho,
                                    });
                                    saw_equals_chirho = false;
                                } else {
                                    // VarId before '=' — this is the field name
                                    // (flush any pending punned field first)
                                    if let Some(fname_chirho) = current_field_name_chirho.take() {
                                        fields_chirho.push(PatFieldChirho {
                                            pattern_chirho: PatChirho::VarChirho(
                                                fname_chirho.clone(),
                                            ),
                                            name_chirho: fname_chirho,
                                            span_chirho,
                                        });
                                    }
                                    let s_chirho = self.span_chirho(
                                        child_chirho.start_chirho,
                                        child_chirho.end_chirho,
                                    );
                                    current_field_name_chirho = Some(
                                        self.name_from_token_chirho(tok_chirho, s_chirho),
                                    );
                                    saw_equals_chirho = false;
                                }
                            } else if kind_chirho == TokenKindChirho::CommaChirho {
                                // Flush pending punned field
                                if let Some(fname_chirho) = current_field_name_chirho.take() {
                                    fields_chirho.push(PatFieldChirho {
                                        pattern_chirho: PatChirho::VarChirho(fname_chirho.clone()),
                                        name_chirho: fname_chirho,
                                        span_chirho,
                                    });
                                }
                                saw_equals_chirho = false;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho)
                            if is_pat_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            // Pattern node after '=' (e.g. constructor pattern, literal, etc.)
                            let pat_chirho =
                                self.lower_pat_chirho(n_chirho, child_chirho.start_chirho);
                            if let Some(fname_chirho) = current_field_name_chirho.take() {
                                let field_span_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.start_chirho + n_chirho.text_len_chirho(),
                                );
                                fields_chirho.push(PatFieldChirho {
                                    name_chirho: fname_chirho,
                                    pattern_chirho: pat_chirho,
                                    span_chirho: field_span_chirho,
                                });
                            }
                            saw_equals_chirho = false;
                        }
                        _ => {}
                    }
                }
                // Handle trailing punned field
                if let Some(fname_chirho) = current_field_name_chirho.take() {
                    fields_chirho.push(PatFieldChirho {
                        pattern_chirho: PatChirho::VarChirho(fname_chirho.clone()),
                        name_chirho: fname_chirho,
                        span_chirho,
                    });
                }

                PatChirho::RecordChirho {
                    con_chirho: con_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    fields_chirho,
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

    /// Extract the body expression from a FunBindChirho CST node that the
    /// parser placed after `in` in a let-expression.  The structure is
    /// typically `FunBindChirho > MatchChirho > <expr children>`, where the
    /// match contains the body expression (possibly as a bare variable
    /// or a full RHS).
    fn lower_let_body_from_funbind_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ExprChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::MatchChirho {
                    // Look inside the match for expression nodes or bare tokens
                    let match_children_chirho =
                        self.semantic_children_chirho(n_chirho, child_chirho.start_chirho);
                    for mc_chirho in &match_children_chirho {
                        if let GreenElementChirho::NodeChirho(inner_chirho) =
                            mc_chirho.element_chirho
                        {
                            if is_expr_kind_chirho(inner_chirho.kind_chirho()) {
                                return self
                                    .lower_expr_chirho(inner_chirho, mc_chirho.start_chirho);
                            }
                        }
                        // Bare VarId or ConId token — treat as a variable expression
                        if let GreenElementChirho::TokenChirho(tok_chirho) =
                            mc_chirho.element_chirho
                        {
                            if matches!(
                                tok_chirho.kind_chirho(),
                                TokenKindChirho::VarIdChirho
                                    | TokenKindChirho::ConIdChirho
                                    | TokenKindChirho::VarSymChirho
                            ) {
                                let s_chirho =
                                    self.span_chirho(mc_chirho.start_chirho, mc_chirho.end_chirho);
                                let name_chirho =
                                    self.name_from_token_chirho(tok_chirho, s_chirho);
                                return ExprChirho::VarChirho(name_chirho);
                            }
                        }
                    }
                }
            }
        }
        self.placeholder_expr_chirho()
    }

    // -----------------------------------------------------------------------
    // Name helpers
    // -----------------------------------------------------------------------

    fn name_from_text_chirho(&self, text_chirho: &str, span_chirho: SpanChirho) -> NameChirho {
        // Split qualified names like "Data.List.sort", but NOT bare operators
        // like "." where both qualifier and local parts would be empty.
        if let Some(dot_pos_chirho) = text_chirho.rfind('.') {
            let qualifier_chirho = &text_chirho[..dot_pos_chirho];
            let local_chirho = &text_chirho[dot_pos_chirho + 1..];
            if !qualifier_chirho.is_empty() && !local_chirho.is_empty() {
                return NameChirho::RawChirho(RawNameChirho::qualified_chirho(
                    qualifier_chirho,
                    local_chirho,
                    span_chirho,
                ));
            }
        }
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(text_chirho, span_chirho))
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

    /// Convert accumulated qualifier parts into a `StmtChirho`.
    ///
    /// Parts are either expressions or a `<-` arrow marker. If an arrow
    /// is present, the qualifier is a generator (`pat <- expr`); otherwise
    /// it's a guard (plain expression).
    fn flush_qual_parts_chirho(
        &self,
        parts_chirho: &[QualPartChirho],
        span_chirho: SpanChirho,
    ) -> Option<StmtChirho> {
        if parts_chirho.is_empty() {
            return None;
        }

        // Find the arrow position, if any
        let arrow_pos_chirho = parts_chirho
            .iter()
            .position(|p_chirho| matches!(p_chirho, QualPartChirho::ArrowChirho));

        if let Some(pos_chirho) = arrow_pos_chirho {
            // Generator: parts before arrow form the pattern (as expr),
            // parts after arrow form the source expression.
            let pat_expr_chirho = parts_chirho
                .iter()
                .take(pos_chirho)
                .filter_map(|p_chirho| match p_chirho {
                    QualPartChirho::ExprChirho(e_chirho) => Some(e_chirho.clone()),
                    _ => None,
                })
                .next();

            let src_expr_chirho = parts_chirho
                .iter()
                .skip(pos_chirho + 1)
                .filter_map(|p_chirho| match p_chirho {
                    QualPartChirho::ExprChirho(e_chirho) => Some(e_chirho.clone()),
                    _ => None,
                })
                .next();

            // Convert the pattern expression to a PatChirho
            let pat_chirho = if let Some(expr_chirho) = pat_expr_chirho {
                Self::expr_to_pat_chirho(&expr_chirho)
            } else {
                PatChirho::WildcardChirho(span_chirho)
            };

            let expr_chirho = src_expr_chirho
                .unwrap_or_else(|| self.placeholder_expr_chirho());

            Some(StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                span_chirho,
            })
        } else {
            // Guard: single expression
            let expr_chirho = parts_chirho
                .iter()
                .filter_map(|p_chirho| match p_chirho {
                    QualPartChirho::ExprChirho(e_chirho) => Some(e_chirho.clone()),
                    _ => None,
                })
                .next()?;
            Some(StmtChirho::ExprChirho(expr_chirho))
        }
    }

    /// Convert a parsed expression back to a pattern.
    /// Used for list comprehension generators where the pattern
    /// was parsed as an expression by the CST parser.
    fn expr_to_pat_chirho(expr_chirho: &ExprChirho) -> PatChirho {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if text_chirho == "_" {
                    PatChirho::WildcardChirho(name_chirho.span_chirho())
                } else if text_chirho.chars().next().map_or(false, |c_chirho| c_chirho.is_uppercase()) {
                    // Constructor with no args
                    PatChirho::ConChirho {
                        con_chirho: name_chirho.clone(),
                        args_chirho: vec![],
                        span_chirho: name_chirho.span_chirho(),
                    }
                } else {
                    PatChirho::VarChirho(name_chirho.clone())
                }
            }
            ExprChirho::LitChirho(lit_chirho) => PatChirho::LitChirho(lit_chirho.clone()),
            ExprChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            } => PatChirho::TupleChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|e_chirho| Self::expr_to_pat_chirho(e_chirho))
                    .collect(),
                span_chirho: *span_chirho,
            },
            _ => {
                // Fallback: wildcard
                PatChirho::WildcardChirho(expr_chirho.span_chirho())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helper types and predicates
// ---------------------------------------------------------------------------

/// A part of a list comprehension qualifier, used during CST→AST lowering.
enum QualPartChirho {
    /// An expression or pattern.
    ExprChirho(ExprChirho),
    /// The `<-` arrow token separating pattern from source.
    ArrowChirho,
}

struct ChildChirho<'a> {
    element_chirho: &'a GreenElementChirho,
    start_chirho: usize,
    end_chirho: usize,
}

/// Merge consecutive `DeclChirho::FunBindChirho` with the same name into a
/// single multi-equation binding.  Haskell requires that all equations for
/// the same function appear consecutively; this pass collapses them into one
/// `FunBindChirho` with `matches_chirho` holding every equation.
fn merge_fun_binds_chirho(decls_chirho: Vec<DeclChirho>) -> Vec<DeclChirho> {
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
            {
                if let Some(DeclChirho::FunBindChirho {
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
            }
        } else {
            merged_chirho.push(decl_chirho);
        }
    }

    merged_chirho
}

/// Merge consecutive `LocalBindChirho::FunBindChirho` with the same name.
fn merge_local_fun_binds_chirho(binds_chirho: Vec<LocalBindChirho>) -> Vec<LocalBindChirho> {
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
            {
                if let Some(LocalBindChirho::FunBindChirho {
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
            }
        } else {
            merged_chirho.push(bind_chirho);
        }
    }

    merged_chirho
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
            | SyntaxKindChirho::LambdaCaseExprChirho
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
// Operator precedence resolution
// ---------------------------------------------------------------------------

/// Associativity of an operator.
#[derive(Clone, Copy, PartialEq)]
enum AssocChirho {
    LeftChirho,
    RightChirho,
    NoneChirho,
}

/// Return (precedence, associativity) for a known operator.
/// Follows Haskell's default fixities.
fn operator_fixity_chirho(op_chirho: &str) -> (u8, AssocChirho) {
    match op_chirho {
        "$" => (0, AssocChirho::RightChirho),
        "||" => (2, AssocChirho::RightChirho),
        "&&" => (3, AssocChirho::RightChirho),
        "==" | "/=" | "<" | "<=" | ">" | ">=" => (4, AssocChirho::NoneChirho),
        ":" | "++" => (5, AssocChirho::RightChirho),
        "+" | "-" => (6, AssocChirho::LeftChirho),
        "*" | "/" | "`div`" | "`mod`" => (7, AssocChirho::LeftChirho),
        "^" | "**" => (8, AssocChirho::RightChirho),
        "." => (9, AssocChirho::RightChirho),
        _ => (9, AssocChirho::LeftChirho), // default: infixl 9
    }
}

/// Resolve a flat chain of expressions and operators into a correctly
/// nested `InfixChirho` tree, respecting operator precedence and
/// associativity.
///
/// Algorithm: find the lowest-precedence operator to split at (using
/// associativity to break ties), then recursively resolve each side.
fn resolve_infix_precedence_chirho(
    exprs_chirho: Vec<ExprChirho>,
    ops_chirho: Vec<NameChirho>,
    span_chirho: SpanChirho,
) -> ExprChirho {
    debug_assert_eq!(exprs_chirho.len(), ops_chirho.len() + 1);
    if ops_chirho.is_empty() {
        return exprs_chirho.into_iter().next().unwrap();
    }
    if ops_chirho.len() == 1 {
        let mut it_chirho = exprs_chirho.into_iter();
        let left_chirho = it_chirho.next().unwrap();
        let right_chirho = it_chirho.next().unwrap();
        return ExprChirho::InfixChirho {
            left_chirho: Box::new(left_chirho),
            op_chirho: ops_chirho.into_iter().next().unwrap(),
            right_chirho: Box::new(right_chirho),
            span_chirho,
        };
    }

    // Find the operator with the lowest precedence to split at.
    // For left-associative operators at the same precedence, pick the
    // rightmost occurrence so the left side groups first.
    // For right-associative, pick the leftmost occurrence.
    let mut split_idx_chirho = 0usize;
    let (mut split_prec_chirho, _) =
        operator_fixity_chirho(ops_chirho[0].text_chirho());

    for (i_chirho, op_chirho) in ops_chirho.iter().enumerate().skip(1) {
        let (prec_chirho, _assoc_chirho) =
            operator_fixity_chirho(op_chirho.text_chirho());
        if prec_chirho < split_prec_chirho {
            // Strictly lower precedence — always split here
            split_idx_chirho = i_chirho;
            split_prec_chirho = prec_chirho;
        } else if prec_chirho == split_prec_chirho {
            // Same precedence — for left-associative, prefer rightmost
            // split (so the left side stays grouped); for right-
            // associative, keep the leftmost split.
            let (_, cur_assoc_chirho) =
                operator_fixity_chirho(ops_chirho[split_idx_chirho].text_chirho());
            if cur_assoc_chirho == AssocChirho::LeftChirho {
                split_idx_chirho = i_chirho;
            }
        }
    }

    let split_op_chirho = ops_chirho[split_idx_chirho].clone();

    // Split exprs and ops around the split point
    let left_exprs_chirho: Vec<ExprChirho> =
        exprs_chirho[..=split_idx_chirho].to_vec();
    let right_exprs_chirho: Vec<ExprChirho> =
        exprs_chirho[split_idx_chirho + 1..].to_vec();
    let left_ops_chirho: Vec<NameChirho> =
        ops_chirho[..split_idx_chirho].to_vec();
    let right_ops_chirho: Vec<NameChirho> =
        ops_chirho[split_idx_chirho + 1..].to_vec();

    let left_chirho =
        resolve_infix_precedence_chirho(left_exprs_chirho, left_ops_chirho, span_chirho);
    let right_chirho =
        resolve_infix_precedence_chirho(right_exprs_chirho, right_ops_chirho, span_chirho);

    ExprChirho::InfixChirho {
        left_chirho: Box::new(left_chirho),
        op_chirho: split_op_chirho,
        right_chirho: Box::new(right_chirho),
        span_chirho,
    }
}

// ---------------------------------------------------------------------------
// Escape sequence processing
// ---------------------------------------------------------------------------

/// Process Haskell escape sequences in a string literal.
/// Handles \n, \t, \r, \\, \", \', \0, \a, \b, \f, \v, and string gaps.
fn unescape_string_chirho(s_chirho: &str) -> String {
    let mut result_chirho = String::with_capacity(s_chirho.len());
    let mut chars_chirho = s_chirho.chars();
    while let Some(c_chirho) = chars_chirho.next() {
        if c_chirho == '\\' {
            match chars_chirho.next() {
                Some('n') => result_chirho.push('\n'),
                Some('t') => result_chirho.push('\t'),
                Some('r') => result_chirho.push('\r'),
                Some('\\') => result_chirho.push('\\'),
                Some('"') => result_chirho.push('"'),
                Some('\'') => result_chirho.push('\''),
                Some('0') => result_chirho.push('\0'),
                Some('a') => result_chirho.push('\x07'),
                Some('b') => result_chirho.push('\x08'),
                Some('f') => result_chirho.push('\x0C'),
                Some('v') => result_chirho.push('\x0B'),
                Some(ws_chirho) if ws_chirho.is_ascii_whitespace() => {
                    // String gap: \<whitespace>\  — skip all whitespace until next backslash
                    for gap_c_chirho in chars_chirho.by_ref() {
                        if gap_c_chirho == '\\' { break; }
                    }
                }
                Some(other_chirho) => {
                    result_chirho.push('\\');
                    result_chirho.push(other_chirho);
                }
                None => result_chirho.push('\\'),
            }
        } else {
            result_chirho.push(c_chirho);
        }
    }
    result_chirho
}

/// Process Haskell escape sequences in a character literal.
fn unescape_char_chirho(s_chirho: &str) -> char {
    if s_chirho.starts_with('\\') {
        match s_chirho.chars().nth(1) {
            Some('n') => '\n',
            Some('t') => '\t',
            Some('r') => '\r',
            Some('\\') => '\\',
            Some('\'') => '\'',
            Some('"') => '"',
            Some('0') => '\0',
            Some('a') => '\x07',
            Some('b') => '\x08',
            Some('f') => '\x0C',
            Some('v') => '\x0B',
            _ => s_chirho.chars().nth(1).unwrap_or('\0'),
        }
    } else {
        s_chirho.chars().next().unwrap_or('\0')
    }
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
    fn lower_import_spec_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nimport Data.List (sort, nub)\nimport Data.Map hiding (map)\n",
        );
        assert_eq!(module_chirho.imports_chirho.len(), 2);

        // First import: explicit list
        let spec0_chirho = module_chirho.imports_chirho[0].spec_chirho.as_ref();
        assert!(spec0_chirho.is_some(), "expected import spec for Data.List");
        let spec0_chirho = spec0_chirho.unwrap();
        assert!(!spec0_chirho.hiding_chirho);
        assert_eq!(spec0_chirho.items_chirho.len(), 2);
        assert!(matches!(&spec0_chirho.items_chirho[0], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "sort"));
        assert!(matches!(&spec0_chirho.items_chirho[1], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "nub"));

        // Second import: hiding
        let spec1_chirho = module_chirho.imports_chirho[1].spec_chirho.as_ref();
        assert!(spec1_chirho.is_some(), "expected import spec for Data.Map");
        let spec1_chirho = spec1_chirho.unwrap();
        assert!(spec1_chirho.hiding_chirho);
        assert_eq!(spec1_chirho.items_chirho.len(), 1);
        assert!(matches!(&spec1_chirho.items_chirho[0], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "map"));
    }

    #[test]
    fn lower_import_tycon_spec_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nimport Data.Map (Map(..))\n",
        );
        assert_eq!(module_chirho.imports_chirho.len(), 1);
        let spec_chirho = module_chirho.imports_chirho[0].spec_chirho.as_ref().unwrap();
        assert!(!spec_chirho.hiding_chirho);
        assert_eq!(spec_chirho.items_chirho.len(), 1);
        assert!(matches!(
            &spec_chirho.items_chirho[0],
            ImportItemChirho::TyConChirho { name_chirho, members_chirho: ExportMembersChirho::AllChirho }
                if name_chirho.text_chirho() == "Map"
        ));
    }

    #[test]
    fn lower_type_alias_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\ntype Name = String\n");
        assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeAliasDeclChirho { name_chirho, .. } if name_chirho.text_chirho() == "Name")
        }));
    }

    #[test]
    fn lower_foreign_import_basic_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nforeign import ccall unsafe \"sin\" sinChirho :: Double -> Double\n",
        );
        let foreign_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::ForeignDeclChirho { .. }));
        assert!(foreign_chirho.is_some(), "expected ForeignDeclChirho");
        match foreign_chirho.unwrap() {
            DeclChirho::ForeignDeclChirho {
                direction_chirho,
                name_chirho,
                calling_conv_chirho,
                safety_chirho,
                foreign_name_chirho,
                ..
            } => {
                assert_eq!(*direction_chirho, ForeignDirectionChirho::ImportChirho);
                assert_eq!(name_chirho.text_chirho(), "sinChirho");
                assert_eq!(calling_conv_chirho, "ccall");
                assert_eq!(safety_chirho.as_deref(), Some("unsafe"));
                assert_eq!(foreign_name_chirho.as_deref(), Some("sin"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_foreign_import_no_safety_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nforeign import ccall putStrChirho :: String -> IO ()\n",
        );
        let foreign_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::ForeignDeclChirho { .. }));
        assert!(foreign_chirho.is_some());
        match foreign_chirho.unwrap() {
            DeclChirho::ForeignDeclChirho {
                direction_chirho,
                name_chirho,
                calling_conv_chirho,
                safety_chirho,
                foreign_name_chirho,
                ..
            } => {
                assert_eq!(*direction_chirho, ForeignDirectionChirho::ImportChirho);
                assert_eq!(name_chirho.text_chirho(), "putStrChirho");
                assert_eq!(calling_conv_chirho, "ccall");
                assert_eq!(*safety_chirho, None);
                assert_eq!(*foreign_name_chirho, None);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_foreign_export_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nforeign export ccall mainChirho :: IO ()\n",
        );
        let foreign_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::ForeignDeclChirho { .. }));
        assert!(foreign_chirho.is_some());
        match foreign_chirho.unwrap() {
            DeclChirho::ForeignDeclChirho {
                direction_chirho,
                name_chirho,
                ..
            } => {
                assert_eq!(*direction_chirho, ForeignDirectionChirho::ExportChirho);
                assert_eq!(name_chirho.text_chirho(), "mainChirho");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_foreign_type_is_function_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nforeign import ccall sqrtChirho :: Double -> Double\n",
        );
        match &module_chirho.decls_chirho[0] {
            DeclChirho::ForeignDeclChirho { ty_chirho, .. } => {
                assert!(
                    matches!(ty_chirho, TypeChirho::FunChirho { .. }),
                    "expected function type, got {:?}",
                    ty_chirho
                );
            }
            other_chirho => panic!("expected ForeignDeclChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_spans_are_nonzero_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module Foo where\ndata Bar = Baz\n");
        let span_chirho = module_chirho.span_chirho;
        assert!(span_chirho.end_chirho().as_usize_chirho() > 0);
    }

    // -----------------------------------------------------------------------
    // Instance declarations
    // -----------------------------------------------------------------------

    #[test]
    fn lower_instance_simple_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\ninstance Eq Bool where\n  eq x y = x\n",
        );
        let inst_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::InstanceDeclChirho { .. }));
        assert!(inst_chirho.is_some(), "expected an InstanceDeclChirho");

        match inst_chirho.unwrap() {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                context_chirho,
                types_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Eq");
                assert!(context_chirho.is_empty());
                // "Bool" is the instance type
                assert!(!types_chirho.is_empty());
                // At least one method
                assert!(
                    !methods_chirho.is_empty(),
                    "expected at least one method binding"
                );
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_instance_with_context_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\ninstance Eq a => Eq (Maybe a) where\n  eq x y = x\n",
        );
        let inst_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::InstanceDeclChirho { .. }));
        assert!(inst_chirho.is_some(), "expected an InstanceDeclChirho");

        match inst_chirho.unwrap() {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                context_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Eq");
                assert_eq!(context_chirho.len(), 1);
                assert_eq!(context_chirho[0].class_chirho.text_chirho(), "Eq");
                assert_eq!(context_chirho[0].args_chirho.len(), 1);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_default_decl_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\ndefault (Int, Double)\n",
        );
        let default_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::DefaultDeclChirho { .. }));
        assert!(default_chirho.is_some(), "expected a DefaultDeclChirho");

        match default_chirho.unwrap() {
            DeclChirho::DefaultDeclChirho { types_chirho, .. } => {
                assert_eq!(types_chirho.len(), 2);
                match &types_chirho[0] {
                    TypeChirho::ConChirho(name_chirho) => {
                        assert_eq!(name_chirho.text_chirho(), "Int");
                    }
                    _ => panic!("expected ConChirho"),
                }
                match &types_chirho[1] {
                    TypeChirho::ConChirho(name_chirho) => {
                        assert_eq!(name_chirho.text_chirho(), "Double");
                    }
                    _ => panic!("expected ConChirho"),
                }
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_instance_no_methods_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\ninstance Show Int\n",
        );
        let inst_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::InstanceDeclChirho { .. }));
        assert!(inst_chirho.is_some(), "expected an InstanceDeclChirho");

        match inst_chirho.unwrap() {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Show");
                assert!(methods_chirho.is_empty());
            }
            _ => unreachable!(),
        }
    }
}
