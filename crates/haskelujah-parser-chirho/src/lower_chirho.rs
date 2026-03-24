// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # CST → AST lowering
//!
//! Walks the immutable green tree produced by the CST parser and builds
//! the typed AST (`ModuleChirho`). Trivia, virtual layout tokens, and
//! punctuation are discarded; only semantic content survives.

use std::collections::HashMap;
use std::sync::Arc;

use haskelujah_ast_chirho::decl_chirho::{
    AstKindChirho, ClassMethodChirho, ConDeclChirho, DeclChirho, FieldDeclChirho, FixityChirho,
    ForeignDirectionChirho, StrictnessChirho, TyVarChirho, TypeFamilyEquationChirho,
};
use haskelujah_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, FieldAssignChirho, GuardedExprChirho, LocalBindChirho, MatchArmChirho,
    RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
    InlinePragmaChirho, ModuleChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::{PatChirho, PatFieldChirho};
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

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

/// Parse an integer literal that may have a hex (0x/0X), octal (0o/0O),
/// or binary (0b/0B) prefix.
fn parse_integer_literal_chirho(text_chirho: &str) -> i64 {
    // Strip MagicHash trailing # and NumericUnderscores
    let clean_chirho: String = text_chirho
        .trim_end_matches('#')
        .chars()
        .filter(|c_chirho| *c_chirho != '_')
        .collect();
    let text_chirho = &clean_chirho;
    if let Some(hex_chirho) = text_chirho
        .strip_prefix("0x")
        .or_else(|| text_chirho.strip_prefix("0X"))
    {
        i64::from_str_radix(hex_chirho, 16).unwrap_or(0)
    } else if let Some(octal_chirho) = text_chirho
        .strip_prefix("0o")
        .or_else(|| text_chirho.strip_prefix("0O"))
    {
        i64::from_str_radix(octal_chirho, 8).unwrap_or(0)
    } else if let Some(binary_chirho) = text_chirho
        .strip_prefix("0b")
        .or_else(|| text_chirho.strip_prefix("0B"))
    {
        i64::from_str_radix(binary_chirho, 2).unwrap_or(0)
    } else {
        text_chirho.parse::<i64>().unwrap_or(0)
    }
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

    /// Check if a CST node contains a specific keyword/identifier text
    /// among its first few non-trivia tokens (after the leading keyword).
    fn node_has_keyword_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        keyword_chirho: &str,
    ) -> bool {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        // Check the first few non-trivia children for the keyword
        for child_chirho in children_chirho.iter().take(5) {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.text_chirho() == keyword_chirho {
                    return true;
                }
            }
        }
        false
    }

    // -----------------------------------------------------------------------
    // Pragma extraction
    // -----------------------------------------------------------------------

    /// Walk all pragma tokens in the CST and extract LANGUAGE extension names.
    /// Pragma tokens have the text `{-# LANGUAGE Ext1, Ext2 #-}`.
    fn extract_pragma_extensions_chirho(&self, node_chirho: &GreenNodeChirho) -> Vec<String> {
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
                                // GHC2021 / GHC2024 meta-extensions expand to their constituent set
                                if ext_chirho == "GHC2021" || ext_chirho == "GHC2024" {
                                    for sub_chirho in ghc2021_extensions_chirho() {
                                        extensions_chirho.push(sub_chirho.to_string());
                                    }
                                } else {
                                    extensions_chirho.push(ext_chirho.to_string());
                                }
                            }
                        }
                    }
                    // {-# OPTIONS_GHC -XFoo -XBar #-} → extract extensions
                    if let Some(rest_chirho) = inner_chirho
                        .strip_prefix("OPTIONS_GHC")
                        .or_else(|| inner_chirho.strip_prefix("OPTIONS"))
                    {
                        for word_chirho in rest_chirho.split_whitespace() {
                            if let Some(ext_chirho) = word_chirho.strip_prefix("-X") {
                                if !ext_chirho.is_empty() {
                                    extensions_chirho.push(ext_chirho.to_string());
                                }
                            }
                        }
                    }
                    // Other OPTIONS, etc. — silently ignore for now
                    // INLINE/NOINLINE/INLINABLE pragmas are extracted separately
                }
            }
        }
        extensions_chirho
    }

    /// Walk all pragma tokens and extract INLINE/NOINLINE/INLINABLE annotations.
    /// Returns a map from binding name to the inline pragma.
    fn extract_inline_pragmas_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
    ) -> HashMap<String, InlinePragmaChirho> {
        let mut pragmas_chirho = HashMap::new();
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::PragmaChirho {
                    let text_chirho = tok_chirho.text_chirho();
                    let inner_chirho = text_chirho
                        .strip_prefix("{-#")
                        .and_then(|s_chirho| s_chirho.strip_suffix("#-}"))
                        .unwrap_or("")
                        .trim();
                    // Check for NOINLINE first (longer match)
                    if let Some(rest_chirho) = inner_chirho.strip_prefix("NOINLINE") {
                        let name_chirho = rest_chirho.trim();
                        if !name_chirho.is_empty() {
                            pragmas_chirho.insert(
                                name_chirho.to_string(),
                                InlinePragmaChirho::NoInlineChirho,
                            );
                        }
                    } else if let Some(rest_chirho) = inner_chirho.strip_prefix("INLINABLE") {
                        let name_chirho = rest_chirho.trim();
                        if !name_chirho.is_empty() {
                            pragmas_chirho.insert(
                                name_chirho.to_string(),
                                InlinePragmaChirho::InlinableChirho,
                            );
                        }
                    } else if let Some(rest_chirho) = inner_chirho.strip_prefix("INLINE") {
                        let name_chirho = rest_chirho.trim();
                        if !name_chirho.is_empty() {
                            pragmas_chirho
                                .insert(name_chirho.to_string(), InlinePragmaChirho::InlineChirho);
                        }
                    }
                }
            }
        }
        pragmas_chirho
    }

    /// Walk all pragma tokens and extract SPECIALIZE/SPECIALISE annotations.
    /// Returns a map from binding name to the list of specialization type strings.
    fn extract_specialize_pragmas_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
    ) -> HashMap<String, Vec<String>> {
        let mut pragmas_chirho: HashMap<String, Vec<String>> = HashMap::new();
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::PragmaChirho {
                    let text_chirho = tok_chirho.text_chirho();
                    let inner_chirho = text_chirho
                        .strip_prefix("{-#")
                        .and_then(|s_chirho| s_chirho.strip_suffix("#-}"))
                        .unwrap_or("")
                        .trim();
                    // Accept both SPECIALIZE and SPECIALISE (British spelling)
                    let rest_chirho = inner_chirho
                        .strip_prefix("SPECIALIZE")
                        .or_else(|| inner_chirho.strip_prefix("SPECIALISE"));
                    if let Some(rest_chirho) = rest_chirho {
                        let rest_chirho = rest_chirho.trim();
                        // Format: "functionName :: Type"
                        if let Some(colons_pos_chirho) = rest_chirho.find("::") {
                            let name_chirho = rest_chirho[..colons_pos_chirho].trim();
                            let ty_text_chirho = rest_chirho[colons_pos_chirho + 2..].trim();
                            if !name_chirho.is_empty() && !ty_text_chirho.is_empty() {
                                pragmas_chirho
                                    .entry(name_chirho.to_string())
                                    .or_default()
                                    .push(ty_text_chirho.to_string());
                            }
                        }
                    }
                }
            }
        }
        pragmas_chirho
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
        let mut exports_chirho: Option<Vec<ExportSpecChirho>> = None;
        let mut imports_chirho = Vec::new();
        let mut decls_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::NodeChirho(n_chirho) => match n_chirho.kind_chirho() {
                    SyntaxKindChirho::ModuleHeaderChirho => {
                        let (name_val_chirho, exp_val_chirho) =
                            self.lower_module_header_chirho(n_chirho, child_chirho.start_chirho);
                        module_name_chirho = name_val_chirho;
                        exports_chirho = exp_val_chirho;
                    }
                    SyntaxKindChirho::ImportDeclChirho => {
                        imports_chirho.push(
                            self.lower_import_decl_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    }
                    SyntaxKindChirho::TypeSigDeclChirho => {
                        let decls_from_sig_chirho = self.lower_type_sigs_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                        );
                        decls_chirho.extend(decls_from_sig_chirho);
                    }
                    _ => {
                        if let Some(decl_chirho) =
                            self.lower_decl_chirho(n_chirho, child_chirho.start_chirho)
                        {
                            decls_chirho.push(decl_chirho);
                        }
                    }
                },
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
        // Extract INLINE/NOINLINE/INLINABLE pragmas
        let inline_pragmas_chirho = self.extract_inline_pragmas_chirho(root_chirho);
        // Extract SPECIALIZE pragmas
        let specialize_pragmas_chirho = self.extract_specialize_pragmas_chirho(root_chirho);

        // Extract foreign exports from ForeignDeclChirho entries
        let foreign_exports_chirho: Vec<(String, String, String)> = decls_chirho
            .iter()
            .filter_map(|d_chirho| {
                if let DeclChirho::ForeignDeclChirho {
                    direction_chirho: ForeignDirectionChirho::ExportChirho,
                    name_chirho,
                    calling_conv_chirho,
                    foreign_name_chirho,
                    ..
                } = d_chirho
                {
                    let haskell_name_chirho = name_chirho.text_chirho().to_string();
                    let c_name_chirho = foreign_name_chirho
                        .as_deref()
                        .unwrap_or(name_chirho.text_chirho())
                        .to_string();
                    Some((
                        haskell_name_chirho,
                        c_name_chirho,
                        calling_conv_chirho.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        // Extract DerivingVia entries from data/newtype decls
        let deriving_via_chirho = self.extract_deriving_via_chirho(root_chirho, start_chirho);

        ModuleChirho {
            name_chirho: module_name_chirho,
            exports_chirho,
            imports_chirho,
            decls_chirho,
            extensions_chirho,
            inline_pragmas_chirho,
            specialize_pragmas_chirho,
            foreign_exports_chirho,
            deriving_via_chirho,
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
    ) -> (NameChirho, Option<Vec<ExportSpecChirho>>) {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho: Option<NameChirho> = None;
        let mut exports_chirho: Option<Vec<ExportSpecChirho>> = None;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => match tok_chirho.kind_chirho() {
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
                        if name_chirho.is_none() =>
                    {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        name_chirho =
                            Some(self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho));
                    }
                    _ => {}
                },
                GreenElementChirho::NodeChirho(n_chirho)
                    if n_chirho.kind_chirho() == SyntaxKindChirho::ExportListChirho =>
                {
                    exports_chirho =
                        Some(self.lower_export_list_chirho(n_chirho, child_chirho.start_chirho));
                }
                _ => {}
            }
        }

        let result_name_chirho = name_chirho.unwrap_or_else(|| {
            NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                "Main",
                SpanChirho::DUMMY_CHIRHO,
            ))
        });
        (result_name_chirho, exports_chirho)
    }

    /// Lower an export list CST node (`(foo, Bar(..), module M)`) into a vector of
    /// `ExportSpecChirho`.  Returns an empty vec for `module M ()`.
    fn lower_export_list_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<ExportSpecChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut exports_chirho: Vec<ExportSpecChirho> = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::ExportSpecChirho {
                    if let Some(spec_chirho) =
                        self.lower_export_spec_item_chirho(n_chirho, child_chirho.start_chirho)
                    {
                        exports_chirho.push(spec_chirho);
                    }
                }
            }
        }

        exports_chirho
    }

    /// Lower a single export spec CST node into an `ExportSpecChirho`.
    ///
    /// Handles:
    /// - `foo`       — variable export (`VarChirho`)
    /// - `Foo`       — type/class, name only (`TyConChirho { NoneChirho }`)
    /// - `Foo(..)`   — type/class with all members (`TyConChirho { AllChirho }`)
    /// - `Foo(A, B)` — type/class with specific members (`TyConChirho { SomeChirho }`)
    /// - `module M`  — module re-export (`ModuleChirho`)
    fn lower_export_spec_item_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<ExportSpecChirho> {
        let mut first_name_chirho: Option<(String, SpanChirho, bool)> = None; // (text, span, is_con)
        let mut is_module_reexport_chirho = false;
        let mut has_parens_chirho = false;
        let mut has_dotdot_chirho = false;
        let mut members_chirho: Vec<NameChirho> = Vec::new();
        let mut in_parens_chirho = false;

        let mut offset_chirho = base_chirho;
        for elem_chirho in node_chirho.children_chirho() {
            let elem_start_chirho = offset_chirho;
            let elem_end_chirho = offset_chirho + elem_chirho.text_len_chirho();
            if !is_trivia_element_chirho(elem_chirho) {
                match elem_chirho {
                    GreenElementChirho::TokenChirho(tok_chirho) => {
                        match tok_chirho.kind_chirho() {
                            TokenKindChirho::ModuleKeywordChirho => {
                                is_module_reexport_chirho = true;
                            }
                            TokenKindChirho::ConIdChirho
                            | TokenKindChirho::QualifiedConIdChirho => {
                                let span_chirho =
                                    self.span_chirho(elem_start_chirho, elem_end_chirho);
                                if first_name_chirho.is_none() && !in_parens_chirho {
                                    first_name_chirho = Some((
                                        tok_chirho.text_chirho().to_string(),
                                        span_chirho,
                                        !is_module_reexport_chirho,
                                    ));
                                } else if in_parens_chirho {
                                    members_chirho.push(self.name_from_text_chirho(
                                        tok_chirho.text_chirho(),
                                        span_chirho,
                                    ));
                                }
                            }
                            TokenKindChirho::VarIdChirho => {
                                let span_chirho =
                                    self.span_chirho(elem_start_chirho, elem_end_chirho);
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
                                let span_chirho =
                                    self.span_chirho(elem_start_chirho, elem_end_chirho);
                                if first_name_chirho.is_none() {
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
            }
            offset_chirho = elem_end_chirho;
        }

        let (text_chirho, span_chirho, is_con_chirho) = first_name_chirho?;
        let name_chirho = self.name_from_text_chirho(&text_chirho, span_chirho);

        if is_module_reexport_chirho {
            Some(ExportSpecChirho::ModuleChirho(name_chirho))
        } else if is_con_chirho && has_parens_chirho {
            let members_spec_chirho = if has_dotdot_chirho {
                ExportMembersChirho::AllChirho
            } else if members_chirho.is_empty() {
                ExportMembersChirho::NoneChirho
            } else {
                ExportMembersChirho::SomeChirho(members_chirho)
            };
            Some(ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho: members_spec_chirho,
            })
        } else if is_con_chirho {
            Some(ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho: ExportMembersChirho::NoneChirho,
            })
        } else {
            Some(ExportSpecChirho::VarChirho(name_chirho))
        }
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
                    let items_chirho =
                        self.lower_import_spec_list_chirho(n_chirho, child_chirho.start_chirho);
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
                GreenElementChirho::TokenChirho(tok_chirho) => match tok_chirho.kind_chirho() {
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        let is_con_chirho = true;
                        if first_name_chirho.is_none() {
                            first_name_chirho = Some((
                                tok_chirho.text_chirho().to_string(),
                                span_chirho,
                                is_con_chirho,
                            ));
                        } else if in_parens_chirho {
                            members_chirho.push(
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho),
                            );
                        }
                    }
                    TokenKindChirho::VarIdChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() {
                            first_name_chirho =
                                Some((tok_chirho.text_chirho().to_string(), span_chirho, false));
                        } else if in_parens_chirho {
                            members_chirho.push(
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho),
                            );
                        }
                    }
                    TokenKindChirho::VarSymChirho | TokenKindChirho::ConSymChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() {
                            first_name_chirho = Some((
                                tok_chirho.text_chirho().to_string(),
                                span_chirho,
                                tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho,
                            ));
                        } else if in_parens_chirho {
                            members_chirho.push(
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho),
                            );
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
                },
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

    fn lower_type_sig_chirho(
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

    fn lower_type_sigs_chirho(
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
                self.placeholder_type_chirho()
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
                        pat_chirho =
                            Some(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) if past_eq_chirho => {
                    // After '=': the RHS expression
                    if rhs_expr_chirho.is_none() {
                        if is_expr_kind_chirho(n_chirho.kind_chirho()) {
                            rhs_expr_chirho =
                                Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                        }
                    }
                }
                _ => {}
            }
        }

        let pat_final_chirho = pat_chirho.unwrap_or(PatChirho::WildcardChirho(span_chirho));
        let rhs_final_chirho = match rhs_expr_chirho {
            Some(expr_chirho) => RhsChirho::UnguardedChirho(expr_chirho),
            None => RhsChirho::UnguardedChirho(ExprChirho::LitChirho(LitChirho::IntChirho(
                0,
                span_chirho,
            ))),
        };

        LocalBindChirho::PatBindChirho {
            pat_chirho: pat_final_chirho,
            rhs_chirho: rhs_final_chirho,
            span_chirho,
        }
    }

    /// Lower a top-level `PatBindChirho` CST node into a `DeclChirho::PatBindChirho`.
    /// Structure: pattern = rhs
    fn lower_pat_bind_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
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
                    if pat_chirho.is_none() {
                        pat_chirho =
                            Some(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) if past_eq_chirho => {
                    if rhs_expr_chirho.is_none() {
                        if is_expr_kind_chirho(n_chirho.kind_chirho()) {
                            rhs_expr_chirho =
                                Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                        }
                    }
                }
                _ => {}
            }
        }

        let pat_final_chirho = pat_chirho.unwrap_or(PatChirho::WildcardChirho(span_chirho));
        let rhs_final_chirho = match rhs_expr_chirho {
            Some(expr_chirho) => RhsChirho::UnguardedChirho(expr_chirho),
            None => RhsChirho::UnguardedChirho(ExprChirho::LitChirho(LitChirho::IntChirho(
                0,
                span_chirho,
            ))),
        };

        DeclChirho::PatBindChirho {
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
        let mut saw_lhs_pat_chirho = false;
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
                            || tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho)
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    if saw_lhs_pat_chirho
                        && (tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho)
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                    if !saw_lhs_pat_chirho
                        && (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho)
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                        saw_lhs_pat_chirho = true;
                    }
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
                    } else if !saw_equals_chirho && is_pat_kind_chirho(n_chirho.kind_chirho()) {
                        pats_chirho
                            .push(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_equals_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                    {
                        where_binds_chirho =
                            self.lower_where_clause_chirho(n_chirho, child_chirho.start_chirho);
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

    /// Lower a `GuardedRhsChirho` CST node into either:
    /// - ordinary guarded RHS arms (`| guard = body`)
    /// - or a single nested expression when pattern/let guards are present
    fn lower_guarded_rhs_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> RhsChirho {
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

        let all_simple_expr_guards_chirho = guards_chirho.iter().all(|guard_chirho| {
            guard_chirho.quals_chirho.len() == 1
                && matches!(
                    guard_chirho.quals_chirho.first(),
                    Some(StmtChirho::ExprChirho(_))
                )
        });

        if all_simple_expr_guards_chirho {
            let lowered_guards_chirho = guards_chirho
                .into_iter()
                .map(|guard_chirho| GuardedExprChirho {
                    guard_chirho: match guard_chirho.quals_chirho.into_iter().next() {
                        Some(StmtChirho::ExprChirho(expr_chirho)) => expr_chirho,
                        _ => self.placeholder_expr_chirho(),
                    },
                    body_chirho: guard_chirho.body_chirho,
                    span_chirho: guard_chirho.span_chirho,
                })
                .collect();
            RhsChirho::GuardedChirho(lowered_guards_chirho)
        } else {
            RhsChirho::UnguardedChirho(self.lower_guard_arms_to_expr_chirho(&guards_chirho))
        }
    }

    /// Lower a single `GuardChirho` CST node into a qualifier sequence and body.
    /// Structure:
    ///   | guard_expr = body_expr
    ///   | pat <- expr, guard_expr = body_expr
    fn lower_guard_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<LoweredGuardArmChirho> {
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
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        if !current_qual_parts_chirho.is_empty() {
                            if let Some(qual_chirho) = self.flush_qual_parts_chirho(
                                &current_qual_parts_chirho,
                                span_chirho,
                            ) {
                                quals_chirho.push(qual_chirho);
                            }
                            current_qual_parts_chirho.clear();
                        }
                        saw_equals_chirho = true;
                    } else if saw_pipe_chirho && !saw_equals_chirho {
                        if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho {
                            if let Some(qual_chirho) = self
                                .flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
                            {
                                quals_chirho.push(qual_chirho);
                            }
                            current_qual_parts_chirho.clear();
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftArrowChirho {
                            current_qual_parts_chirho.push(QualPartChirho::ArrowChirho);
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_pipe_chirho && !saw_equals_chirho {
                        if n_chirho.kind_chirho() == SyntaxKindChirho::LetStmtChirho {
                            if let Some(qual_chirho) = self
                                .flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
                            {
                                quals_chirho.push(qual_chirho);
                            }
                            current_qual_parts_chirho.clear();
                            quals_chirho.push(StmtChirho::LetChirho {
                                binds_chirho: self
                                    .lower_let_stmt_binds_chirho(n_chirho, child_chirho.start_chirho),
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

        if !current_qual_parts_chirho.is_empty() {
            if let Some(qual_chirho) =
                self.flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
            {
                quals_chirho.push(qual_chirho);
            }
        }

        Some(LoweredGuardArmChirho {
            quals_chirho,
            body_chirho: body_expr_chirho.unwrap_or(self.placeholder_expr_chirho()),
            span_chirho,
        })
    }

    fn lower_guard_arms_to_expr_chirho(
        &self,
        guards_chirho: &[LoweredGuardArmChirho],
    ) -> ExprChirho {
        if let Some((first_guard_chirho, rest_guards_chirho)) = guards_chirho.split_first() {
            let rest_expr_chirho = self.lower_guard_arms_to_expr_chirho(rest_guards_chirho);
            self.lower_guard_quals_to_expr_chirho(
                &first_guard_chirho.quals_chirho,
                first_guard_chirho.body_chirho.clone(),
                rest_expr_chirho,
                first_guard_chirho.span_chirho,
            )
        } else {
            let error_name_chirho =
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho("error", SpanChirho::DUMMY_CHIRHO));
            ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(error_name_chirho)),
                arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::StringChirho(
                    "Non-exhaustive guards".to_string(),
                    SpanChirho::DUMMY_CHIRHO,
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }
        }
    }

    fn lower_guard_quals_to_expr_chirho(
        &self,
        quals_chirho: &[StmtChirho],
        then_expr_chirho: ExprChirho,
        else_expr_chirho: ExprChirho,
        span_chirho: SpanChirho,
    ) -> ExprChirho {
        if let Some((first_qual_chirho, rest_quals_chirho)) = quals_chirho.split_first() {
            match first_qual_chirho {
                StmtChirho::ExprChirho(expr_chirho) => ExprChirho::IfChirho {
                    cond_chirho: Box::new(expr_chirho.clone()),
                    then_chirho: Box::new(self.lower_guard_quals_to_expr_chirho(
                        rest_quals_chirho,
                        then_expr_chirho,
                        else_expr_chirho.clone(),
                        span_chirho,
                    )),
                    else_chirho: Box::new(else_expr_chirho),
                    span_chirho,
                },
                StmtChirho::BindChirho {
                    pat_chirho,
                    expr_chirho,
                    span_chirho: bind_span_chirho,
                } => ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(expr_chirho.clone()),
                    alts_chirho: vec![
                        AltChirho {
                            pat_chirho: pat_chirho.clone(),
                            rhs_chirho: RhsChirho::UnguardedChirho(
                                self.lower_guard_quals_to_expr_chirho(
                                    rest_quals_chirho,
                                    then_expr_chirho,
                                    else_expr_chirho.clone(),
                                    span_chirho,
                                ),
                            ),
                            where_binds_chirho: vec![],
                            span_chirho: *bind_span_chirho,
                        },
                        AltChirho {
                            pat_chirho: PatChirho::WildcardChirho(*bind_span_chirho),
                            rhs_chirho: RhsChirho::UnguardedChirho(else_expr_chirho),
                            where_binds_chirho: vec![],
                            span_chirho: *bind_span_chirho,
                        },
                    ],
                    span_chirho: *bind_span_chirho,
                },
                StmtChirho::LetChirho {
                    binds_chirho,
                    span_chirho: let_span_chirho,
                } => ExprChirho::LetChirho {
                    binds_chirho: binds_chirho.clone(),
                    body_chirho: Box::new(self.lower_guard_quals_to_expr_chirho(
                        rest_quals_chirho,
                        then_expr_chirho,
                        else_expr_chirho,
                        span_chirho,
                    )),
                    span_chirho: *let_span_chirho,
                },
            }
        } else {
            then_expr_chirho
        }
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
        let mut kind_sig_chirho = None;
        let mut saw_equals_chirho = false;
        let mut saw_data_chirho = false;
        let mut saw_double_colon_chirho = false;

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::DataKeywordChirho {
                        saw_data_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
                        && saw_data_chirho
                        && name_chirho.is_some()
                        && !saw_equals_chirho
                    {
                        // Standalone kind signature: `data T :: K where`
                        // Collect token atoms between `::` and `where`/`=`, then
                        // build a TypeChirho from them (the CST stores these as
                        // flat tokens, not a parsed type node).
                        saw_double_colon_chirho = true;
                        let mut atoms_chirho: Vec<TypeChirho> = Vec::new();
                        let mut arrows_chirho: Vec<usize> = Vec::new(); // indices into atoms where -> appears
                        let mut j_chirho = idx_chirho + 1;
                        while j_chirho < children_chirho.len() {
                            let kc_chirho = &children_chirho[j_chirho];
                            match kc_chirho.element_chirho {
                                GreenElementChirho::TokenChirho(kt_chirho)
                                    if kt_chirho.kind_chirho()
                                        == TokenKindChirho::WhereKeywordChirho
                                        || kt_chirho.kind_chirho()
                                            == TokenKindChirho::EqualsChirho =>
                                {
                                    break;
                                }
                                GreenElementChirho::TokenChirho(kt_chirho)
                                    if kt_chirho.kind_chirho()
                                        == TokenKindChirho::RightArrowChirho =>
                                {
                                    arrows_chirho.push(atoms_chirho.len());
                                }
                                GreenElementChirho::TokenChirho(kt_chirho)
                                    if kt_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                                        || kt_chirho.kind_chirho()
                                            == TokenKindChirho::VarIdChirho =>
                                {
                                    let s_chirho = self
                                        .span_chirho(kc_chirho.start_chirho, kc_chirho.end_chirho);
                                    let nm_chirho =
                                        self.name_from_token_chirho(kt_chirho, s_chirho);
                                    if kt_chirho.kind_chirho() == TokenKindChirho::ConIdChirho {
                                        atoms_chirho.push(TypeChirho::ConChirho(nm_chirho));
                                    } else {
                                        atoms_chirho.push(TypeChirho::VarChirho(nm_chirho));
                                    }
                                }
                                GreenElementChirho::TokenChirho(kt_chirho)
                                    if (kt_chirho.kind_chirho()
                                        == TokenKindChirho::VarSymChirho
                                        || kt_chirho.kind_chirho()
                                            == TokenKindChirho::ConSymChirho)
                                        && kt_chirho.text_chirho() == "*" =>
                                {
                                    let s_chirho = self
                                        .span_chirho(kc_chirho.start_chirho, kc_chirho.end_chirho);
                                    atoms_chirho.push(TypeChirho::ConChirho(
                                        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                                            "*", s_chirho,
                                        )),
                                    ));
                                }
                                GreenElementChirho::NodeChirho(_) => {
                                    // If the parser wrapped part of the kind sig
                                    // in a node, lower it as a type.
                                    atoms_chirho.push(self.lower_type_from_child_chirho(kc_chirho));
                                }
                                _ => {}
                            }
                            j_chirho += 1;
                        }
                        // Build the kind type: fold right over arrows.
                        // `N -> Type` with atoms=[N, Type], arrows=[1 position]
                        // becomes FunChirho { arg: N, result: Type }.
                        if !atoms_chirho.is_empty() {
                            if arrows_chirho.is_empty() {
                                // Single atom, no arrows (e.g. `data T :: Type where`).
                                kind_sig_chirho = Some(atoms_chirho.remove(0));
                            } else {
                                // Split atoms by arrow positions and fold right.
                                // atoms_chirho contains the types between arrows.
                                // arrows_chirho[i] = the atom index at which arrow i occurs
                                // (arrows separate atoms, so atom count = arrow count + 1).
                                let mut result_chirho = atoms_chirho.pop().unwrap();
                                for a_chirho in atoms_chirho.into_iter().rev() {
                                    result_chirho = TypeChirho::FunChirho {
                                        arg_chirho: Box::new(a_chirho),
                                        mult_chirho: None,
                                        result_chirho: Box::new(result_chirho),
                                        span_chirho: span_chirho,
                                    };
                                }
                                kind_sig_chirho = Some(result_chirho);
                            }
                        }
                        idx_chirho = j_chirho;
                        continue;
                    } else if saw_data_chirho && !saw_equals_chirho && !saw_double_colon_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            && name_chirho.is_none()
                        {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                            type_vars_chirho
                                .push(self.name_from_token_chirho(tok_chirho, s_chirho).into());
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                            // KindSignatures: try to parse (varId :: kind)
                            if let Some((tv_chirho, skip_chirho)) = self
                                .try_parse_kind_annotated_tyvar_chirho(&children_chirho, idx_chirho)
                            {
                                type_vars_chirho.push(tv_chirho);
                                idx_chirho += skip_chirho;
                                continue;
                            }
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::ConDeclChirho {
                        constructors_chirho
                            .push(self.lower_con_decl_chirho(n_chirho, child_chirho.start_chirho));
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
            idx_chirho += 1;
        }

        DeclChirho::DataDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            kind_sig_chirho,
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
        let mut fields_chirho: Vec<(StrictnessChirho, TypeChirho)> = Vec::new();
        let mut has_record_chirho = false;
        let mut record_fields_chirho: Vec<FieldDeclChirho> = Vec::new();

        // ExistentialQuantification: skip past `forall ... . [Context =>]`
        // by finding the index after DoubleArrow (=>) if present, or after Dot
        // if forall is present without a context.
        let mut start_idx_chirho = 0;
        let has_forall_chirho = children_chirho.first().map_or(false, |c_chirho| {
            matches!(
                c_chirho.element_chirho,
                GreenElementChirho::TokenChirho(t_chirho) if t_chirho.kind_chirho() == TokenKindChirho::ForallKeywordChirho
            )
        });
        if has_forall_chirho {
            // Look for => first (forall with context), then . (forall without context)
            let mut found_double_arrow_chirho = false;
            for (idx_chirho, child_chirho) in children_chirho.iter().enumerate() {
                if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                    if tok_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho {
                        start_idx_chirho = idx_chirho + 1;
                        found_double_arrow_chirho = true;
                        break;
                    }
                }
            }
            if !found_double_arrow_chirho {
                // No context, just forall a .
                for (idx_chirho, child_chirho) in children_chirho.iter().enumerate() {
                    if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho
                    {
                        // The dot `.` in `forall a.` is a VarSymChirho token
                        if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho {
                            start_idx_chirho = idx_chirho + 1;
                            break;
                        }
                    }
                }
            }
        }

        let mut pending_strict_chirho = false;
        for child_chirho in children_chirho.iter().skip(start_idx_chirho) {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                        && name_chirho.is_none()
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                        && tok_chirho.text_chirho() == "!"
                    {
                        pending_strict_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::RecordFieldsChirho {
                        has_record_chirho = true;
                        record_fields_chirho =
                            self.lower_record_fields_chirho(n_chirho, child_chirho.start_chirho);
                    } else if is_type_kind_chirho(n_chirho.kind_chirho()) {
                        let strictness_chirho = if pending_strict_chirho {
                            pending_strict_chirho = false;
                            StrictnessChirho::StrictChirho
                        } else {
                            StrictnessChirho::LazyChirho
                        };
                        let ty_chirho = self.lower_type_chirho(n_chirho, child_chirho.start_chirho);
                        fields_chirho.push((strictness_chirho, ty_chirho));
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
    /// to a `ConDeclChirho::GadtChirho` preserving the full type signature
    /// for type refinement in pattern matching.
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
                        sig_type_chirho =
                            Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
                _ => {}
            }
        }

        let con_name_chirho = name_chirho.unwrap_or_else(|| self.dummy_name_chirho());
        match sig_type_chirho {
            Some(ty_chirho) => ConDeclChirho::GadtChirho {
                name_chirho: con_name_chirho,
                ty_chirho,
                span_chirho,
            },
            None => ConDeclChirho::OrdinaryChirho {
                name_chirho: con_name_chirho,
                fields_chirho: Vec::new(),
                span_chirho,
            },
        }
    }

    fn lower_deriving_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<NameChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut names_chirho = Vec::new();
        let mut saw_via_chirho = false;
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                // If we see "via", these classes belong to DerivingVia, not regular deriving
                if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                    && tok_chirho.text_chirho() == "via"
                {
                    saw_via_chirho = true;
                    continue;
                }
                if !saw_via_chirho && tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    names_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                }
            }
        }
        // If `via` was present, these classes are handled by DerivingVia, not regular deriving
        if saw_via_chirho {
            names_chirho.clear();
        }
        names_chirho
    }

    /// Extract class names before "via" from a `deriving (Class) via Type` clause.
    /// Unlike `lower_deriving_chirho`, this returns the classes even when via is present.
    fn lower_deriving_via_classes_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<NameChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut names_chirho = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                    && tok_chirho.text_chirho() == "via"
                {
                    break; // Stop at "via", but keep collected names
                }
                if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    names_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                }
            }
        }
        names_chirho
    }

    /// Extract the via type from a `deriving (Class) via Type` clause.
    /// Returns `Some(via_type)` if a `via` keyword is present, `None` otherwise.
    fn lower_deriving_via_type_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<TypeChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut saw_via_chirho = false;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                        && tok_chirho.text_chirho() == "via"
                    {
                        saw_via_chirho = true;
                    } else if saw_via_chirho
                        && tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                        return Some(TypeChirho::ConChirho(name_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_via_chirho {
                        return Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }
        None
    }

    /// Walk the green tree to extract `deriving (Class) via Type` entries.
    /// Returns `(type_name, class_name, via_type)` triples.
    fn extract_deriving_via_chirho(
        &self,
        root_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<(NameChirho, NameChirho, TypeChirho)> {
        let mut result_chirho = Vec::new();
        let children_chirho = self.semantic_children_chirho(root_chirho, base_chirho);
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                let kind_chirho = n_chirho.kind_chirho();
                if kind_chirho == SyntaxKindChirho::DataDeclChirho
                    || kind_chirho == SyntaxKindChirho::NewtypeDeclChirho
                {
                    // Extract the type name and any deriving-via clauses
                    let sub_children_chirho =
                        self.semantic_children_chirho(n_chirho, child_chirho.start_chirho);
                    let mut type_name_chirho: Option<NameChirho> = None;
                    for sc_chirho in &sub_children_chirho {
                        match sc_chirho.element_chirho {
                            GreenElementChirho::TokenChirho(tok_chirho) => {
                                if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                                    && type_name_chirho.is_none()
                                {
                                    let s_chirho = self
                                        .span_chirho(sc_chirho.start_chirho, sc_chirho.end_chirho);
                                    type_name_chirho =
                                        Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                                }
                            }
                            GreenElementChirho::NodeChirho(sub_n_chirho) => {
                                if sub_n_chirho.kind_chirho()
                                    == SyntaxKindChirho::DerivingClauseChirho
                                {
                                    if let Some(via_type_chirho) = self
                                        .lower_deriving_via_type_chirho(
                                            sub_n_chirho,
                                            sc_chirho.start_chirho,
                                        )
                                    {
                                        if let Some(ref tn_chirho) = type_name_chirho {
                                            // Extract class names before "via" directly
                                            let class_names_chirho = self
                                                .lower_deriving_via_classes_chirho(
                                                    sub_n_chirho,
                                                    sc_chirho.start_chirho,
                                                );
                                            for cn_chirho in class_names_chirho {
                                                result_chirho.push((
                                                    tn_chirho.clone(),
                                                    cn_chirho,
                                                    via_type_chirho.clone(),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        result_chirho
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

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
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
                                .push(self.name_from_token_chirho(tok_chirho, s_chirho).into());
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                            if let Some((tv_chirho, skip_chirho)) = self
                                .try_parse_kind_annotated_tyvar_chirho(&children_chirho, idx_chirho)
                            {
                                type_vars_chirho.push(tv_chirho);
                                idx_chirho += skip_chirho;
                                continue;
                            }
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
            idx_chirho += 1;
        }

        DeclChirho::NewtypeDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            constructor_chirho: constructor_chirho.unwrap_or_else(|| {
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: self.dummy_name_chirho(),
                    fields_chirho: vec![],
                    span_chirho,
                }
            }),
            deriving_chirho,
            kind_sig_chirho: None,
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

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::TypeKeywordChirho {
                        saw_type_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_type_chirho && !saw_equals_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if (tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho)
                            && name_chirho.is_none()
                        {
                            // TypeOperators: operator names (:+:, +, etc.) as type alias name
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                            type_vars_chirho
                                .push(self.name_from_token_chirho(tok_chirho, s_chirho).into());
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                            if let Some((tv_chirho, skip_chirho)) = self
                                .try_parse_kind_annotated_tyvar_chirho(&children_chirho, idx_chirho)
                            {
                                type_vars_chirho.push(tv_chirho);
                                idx_chirho += skip_chirho;
                                continue;
                            }
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
            idx_chirho += 1;
        }

        DeclChirho::TypeAliasDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            rhs_chirho: rhs_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
            span_chirho,
        }
    }

    fn lower_type_family_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        let mut equations_chirho: Vec<TypeFamilyEquationChirho> = Vec::new();
        let mut result_kind_chirho = None;
        let mut saw_family_chirho = false;
        let mut saw_where_chirho = false;
        let mut saw_double_colon_chirho = false;

        // Collect equation tokens: group tokens between ; markers
        let mut eq_lhs_types_chirho: Vec<TypeChirho> = Vec::new();
        let mut eq_rhs_chirho: Option<TypeChirho> = None;
        let mut eq_saw_equals_chirho = false;
        let mut eq_saw_family_name_chirho = false; // skip first ConId in each equation (family name)
        let mut eq_paren_depth_chirho: usize = 0; // parenthesized group depth
        let mut eq_paren_types_chirho: Vec<TypeChirho> = Vec::new();

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            // If we have a completed equation and encounter a non-semicolon
            // token (layout didn't insert virtual semicolons between equations),
            // finalize the previous equation.
            if saw_where_chirho && eq_saw_equals_chirho && eq_rhs_chirho.is_some() {
                let is_separator_chirho = matches!(child_chirho.element_chirho,
                    GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::VirtualSemicolonChirho
                        || tok_chirho.kind_chirho() == TokenKindChirho::VirtualRightBraceChirho
                );
                if !is_separator_chirho {
                    if let Some(rhs_chirho) = eq_rhs_chirho.take() {
                        equations_chirho.push(TypeFamilyEquationChirho {
                            lhs_types_chirho: std::mem::take(&mut eq_lhs_types_chirho),
                            rhs_chirho,
                            span_chirho: span_chirho.clone(),
                        });
                    }
                    eq_saw_equals_chirho = false;
                    eq_saw_family_name_chirho = false;
                }
            }
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::TypeKeywordChirho {
                        // skip
                    } else if tok_chirho.text_chirho() == "family" {
                        saw_family_chirho = true;
                    } else if kind_chirho == TokenKindChirho::WhereKeywordChirho {
                        saw_where_chirho = true;
                    } else if kind_chirho == TokenKindChirho::DoubleColonChirho && !saw_where_chirho
                    {
                        saw_double_colon_chirho = true;
                    } else if kind_chirho == TokenKindChirho::EqualsChirho {
                        if saw_where_chirho {
                            eq_saw_equals_chirho = true;
                        }
                    } else if kind_chirho == TokenKindChirho::VirtualSemicolonChirho
                        || kind_chirho == TokenKindChirho::VirtualRightBraceChirho
                    {
                        // Finish current equation if we had one
                        if eq_saw_equals_chirho {
                            if let Some(rhs_chirho) = eq_rhs_chirho.take() {
                                equations_chirho.push(TypeFamilyEquationChirho {
                                    lhs_types_chirho: std::mem::take(&mut eq_lhs_types_chirho),
                                    rhs_chirho,
                                    span_chirho: span_chirho.clone(),
                                });
                            }
                        }
                        eq_lhs_types_chirho.clear();
                        eq_rhs_chirho = None;
                        eq_saw_equals_chirho = false;
                        eq_saw_family_name_chirho = false;
                        eq_paren_depth_chirho = 0;
                        eq_paren_types_chirho.clear();
                    } else if saw_family_chirho && !saw_where_chirho && !saw_double_colon_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if kind_chirho == TokenKindChirho::ConIdChirho && name_chirho.is_none() {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if kind_chirho == TokenKindChirho::VarIdChirho {
                            type_vars_chirho
                                .push(self.name_from_token_chirho(tok_chirho, s_chirho).into());
                        } else if kind_chirho == TokenKindChirho::LeftParenChirho {
                            if let Some((tv_chirho, skip_chirho)) = self
                                .try_parse_kind_annotated_tyvar_chirho(&children_chirho, idx_chirho)
                            {
                                type_vars_chirho.push(tv_chirho);
                                idx_chirho += skip_chirho;
                                continue;
                            }
                        }
                    } else if saw_where_chirho && !eq_saw_equals_chirho {
                        // LHS type tokens in equation — flat token form.
                        // Handle parenthesized groups by collecting types until
                        // the matching `)` and combining as applications.
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if kind_chirho == TokenKindChirho::LeftParenChirho {
                            // Start collecting a parenthesized group
                            eq_paren_depth_chirho += 1;
                        } else if kind_chirho == TokenKindChirho::RightParenChirho {
                            if eq_paren_depth_chirho > 0 {
                                eq_paren_depth_chirho -= 1;
                                if eq_paren_depth_chirho == 0 && !eq_paren_types_chirho.is_empty() {
                                    // Build application from collected types
                                    let mut result_ty_chirho = eq_paren_types_chirho.remove(0);
                                    for t_chirho in eq_paren_types_chirho.drain(..) {
                                        result_ty_chirho = TypeChirho::AppChirho {
                                            fun_chirho: Box::new(result_ty_chirho),
                                            arg_chirho: Box::new(t_chirho),
                                            span_chirho: s_chirho,
                                        };
                                    }
                                    eq_lhs_types_chirho.push(result_ty_chirho);
                                }
                            }
                        } else if eq_paren_depth_chirho > 0 {
                            // Inside parens — collect types
                            if kind_chirho == TokenKindChirho::ConIdChirho {
                                let nm_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                                eq_paren_types_chirho.push(TypeChirho::ConChirho(nm_chirho));
                            } else if kind_chirho == TokenKindChirho::VarIdChirho {
                                let nm_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                                eq_paren_types_chirho.push(TypeChirho::VarChirho(nm_chirho));
                            }
                        } else if kind_chirho == TokenKindChirho::ConIdChirho {
                            if !eq_saw_family_name_chirho {
                                // First ConId in equation is the family name — skip it
                                eq_saw_family_name_chirho = true;
                            } else {
                                let nm_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                                eq_lhs_types_chirho.push(TypeChirho::ConChirho(nm_chirho));
                            }
                        } else if kind_chirho == TokenKindChirho::VarIdChirho {
                            let nm_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                            eq_lhs_types_chirho.push(TypeChirho::VarChirho(nm_chirho));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_double_colon_chirho && !saw_where_chirho && result_kind_chirho.is_none()
                    {
                        result_kind_chirho =
                            Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                        saw_double_colon_chirho = false; // consumed
                    } else if saw_where_chirho && eq_saw_equals_chirho && eq_rhs_chirho.is_none() {
                        eq_rhs_chirho =
                            Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_where_chirho && !eq_saw_equals_chirho {
                        // LHS node in equation — a parenthesized type like (Maybe a)
                        if !eq_saw_family_name_chirho {
                            eq_saw_family_name_chirho = true;
                        }
                        let ty_chirho = self.lower_type_chirho(n_chirho, child_chirho.start_chirho);
                        eq_lhs_types_chirho.push(ty_chirho);
                    }
                }
            }
            idx_chirho += 1;
        }
        // Finish last equation
        if eq_saw_equals_chirho {
            if let Some(rhs_chirho) = eq_rhs_chirho.take() {
                equations_chirho.push(TypeFamilyEquationChirho {
                    lhs_types_chirho: std::mem::take(&mut eq_lhs_types_chirho),
                    rhs_chirho,
                    span_chirho: span_chirho.clone(),
                });
            }
        }

        DeclChirho::TypeFamilyDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            result_kind_chirho,
            equations_chirho,
            span_chirho,
        }
    }

    fn lower_type_family_instance_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut family_name_chirho = None;
        let mut lhs_types_chirho = Vec::new();
        let mut rhs_chirho = None;
        let mut saw_instance_chirho = false;
        let mut saw_equals_chirho = false;
        let mut inst_paren_depth_chirho: usize = 0;
        let mut inst_paren_types_chirho: Vec<TypeChirho> = Vec::new();

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::TypeKeywordChirho {
                        // skip
                    } else if tok_chirho.text_chirho() == "instance" {
                        saw_instance_chirho = true;
                    } else if kind_chirho == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_instance_chirho && !saw_equals_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if kind_chirho == TokenKindChirho::LeftParenChirho {
                            inst_paren_depth_chirho += 1;
                        } else if kind_chirho == TokenKindChirho::RightParenChirho {
                            if inst_paren_depth_chirho > 0 {
                                inst_paren_depth_chirho -= 1;
                                if inst_paren_depth_chirho == 0
                                    && !inst_paren_types_chirho.is_empty()
                                {
                                    let mut result_ty_chirho = inst_paren_types_chirho.remove(0);
                                    for t_chirho in inst_paren_types_chirho.drain(..) {
                                        result_ty_chirho = TypeChirho::AppChirho {
                                            fun_chirho: Box::new(result_ty_chirho),
                                            arg_chirho: Box::new(t_chirho),
                                            span_chirho: s_chirho,
                                        };
                                    }
                                    lhs_types_chirho.push(result_ty_chirho);
                                }
                            }
                        } else if inst_paren_depth_chirho > 0 {
                            if kind_chirho == TokenKindChirho::ConIdChirho {
                                let nm_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                                inst_paren_types_chirho.push(TypeChirho::ConChirho(nm_chirho));
                            } else if kind_chirho == TokenKindChirho::VarIdChirho {
                                let nm_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                                inst_paren_types_chirho.push(TypeChirho::VarChirho(nm_chirho));
                            }
                        } else if kind_chirho == TokenKindChirho::ConIdChirho
                            && family_name_chirho.is_none()
                        {
                            family_name_chirho =
                                Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if family_name_chirho.is_some()
                            && (kind_chirho == TokenKindChirho::ConIdChirho
                                || kind_chirho == TokenKindChirho::VarIdChirho)
                        {
                            let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                            if kind_chirho == TokenKindChirho::ConIdChirho {
                                lhs_types_chirho.push(TypeChirho::ConChirho(name_chirho));
                            } else {
                                lhs_types_chirho.push(TypeChirho::VarChirho(name_chirho));
                            }
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_instance_chirho && !saw_equals_chirho && family_name_chirho.is_some() {
                        // LHS type argument (a parsed type node)
                        lhs_types_chirho
                            .push(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    } else if saw_equals_chirho && rhs_chirho.is_none() {
                        rhs_chirho =
                            Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
            idx_chirho += 1;
        }

        DeclChirho::TypeFamilyInstanceDeclChirho {
            family_name_chirho: family_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            lhs_types_chirho,
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
        let mut saw_class_chirho = false;
        let mut saw_where_chirho = false;
        let mut saw_fat_arrow_chirho = false;
        let mut saw_pipe_chirho = false;

        // Two-pass approach: collect tokens before and after `=>`.
        // Before `=>` = superclass context, after `=>` = class head (name + type vars).
        // If no `=>`, all tokens form the class head directly.
        let mut pre_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho, usize)> = Vec::new();
        let mut post_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho, usize)> = Vec::new();

        // Fundep parsing state: after `|`, collect `a b -> c d, e -> f`
        let mut fundeps_chirho: Vec<(Vec<String>, Vec<String>)> = Vec::new();
        let mut fundep_from_chirho: Vec<String> = Vec::new();
        let mut fundep_to_chirho: Vec<String> = Vec::new();
        let mut in_to_chirho = false; // true after seeing `->`
        let mut paren_depth_chirho = 0usize;

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                    paren_depth_chirho += 1;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::RightParenChirho {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                } else if tok_chirho.kind_chirho() == TokenKindChirho::ClassKeywordChirho {
                    saw_class_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::WhereKeywordChirho
                    && paren_depth_chirho == 0
                {
                    saw_where_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho
                    && saw_class_chirho
                    && !saw_where_chirho
                    && !saw_pipe_chirho
                    && paren_depth_chirho == 0
                {
                    saw_fat_arrow_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                    && saw_class_chirho
                    && !saw_where_chirho
                    && paren_depth_chirho == 0
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
                    if saw_fat_arrow_chirho {
                        post_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                    } else {
                        pre_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                    }
                }
            }
            idx_chirho += 1;
        }

        // Flush last fundep if any
        if !fundep_from_chirho.is_empty() || !fundep_to_chirho.is_empty() {
            fundeps_chirho.push((fundep_from_chirho, fundep_to_chirho));
        }

        // Build superclass context from pre-arrow tokens (if `=>` was present).
        let context_chirho = if saw_fat_arrow_chirho {
            let ctx_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = pre_arrow_tokens_chirho
                .iter()
                .map(|(t_chirho, s_chirho, _)| (*t_chirho, *s_chirho))
                .collect();
            self.build_instance_context_chirho(&ctx_tokens_chirho)
        } else {
            vec![]
        };

        // The head tokens are after `=>` if present, otherwise all pre-arrow tokens.
        let head_tokens_chirho = if saw_fat_arrow_chirho {
            &post_arrow_tokens_chirho
        } else {
            &pre_arrow_tokens_chirho
        };

        // Extract class name and type variables from head tokens.
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        for (tok_chirho, s_chirho, tok_idx_chirho) in head_tokens_chirho {
            if tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho && name_chirho.is_none() {
                name_chirho = Some(self.name_from_token_chirho(tok_chirho, *s_chirho));
            } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                type_vars_chirho.push(self.name_from_token_chirho(tok_chirho, *s_chirho).into());
            } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                if let Some((tv_chirho, _skip_chirho)) =
                    self.try_parse_kind_annotated_tyvar_chirho(&children_chirho, *tok_idx_chirho)
                {
                    type_vars_chirho.push(tv_chirho);
                }
            }
        }

        // Extract method signatures (and default implementations) from the
        // where block.  The CST where-clause contains TypeSig and FunBind
        // children — we reuse `collect_instance_methods_chirho` to gather them.
        let mut where_decls_chirho: Vec<DeclChirho> = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if saw_where_chirho || n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
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

        // Extract DefaultSignatures: `default methodName :: ConstrainedType`
        // from DefaultDeclChirho nodes in the where clause. These have a VarId
        // followed by `::` after the `default` keyword.
        let mut default_sigs_chirho: HashMap<String, String> = HashMap::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::DefaultDeclChirho {
                    if let Some((name_text_chirho, sig_ty_chirho)) =
                        self.try_extract_default_sig_chirho(n_chirho, child_chirho.start_chirho)
                    {
                        default_sigs_chirho.insert(name_text_chirho, sig_ty_chirho);
                    }
                }
                // Also check inside WhereClauseChirho children
                if saw_where_chirho || n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                {
                    let where_children_chirho =
                        self.semantic_children_chirho(n_chirho, child_chirho.start_chirho);
                    for wc_chirho in &where_children_chirho {
                        if let GreenElementChirho::NodeChirho(wn_chirho) = wc_chirho.element_chirho
                        {
                            if wn_chirho.kind_chirho() == SyntaxKindChirho::DefaultDeclChirho {
                                if let Some((name_text_chirho, sig_ty_chirho)) = self
                                    .try_extract_default_sig_chirho(
                                        wn_chirho,
                                        wc_chirho.start_chirho,
                                    )
                                {
                                    default_sigs_chirho.insert(name_text_chirho, sig_ty_chirho);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Extract associated type family declarations from where-block.
        let mut assoc_tfs_chirho: Vec<haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho> =
            Vec::new();

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
                    let default_sig_chirho = default_sigs_chirho
                        .get(sig_name_chirho.text_chirho())
                        .cloned();
                    Some(ClassMethodChirho {
                        name_chirho: sig_name_chirho,
                        ty_chirho: sig_ty_chirho,
                        default_chirho,
                        default_sig_chirho,
                        span_chirho: sig_span_chirho,
                    })
                }
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho: tf_name_chirho,
                    type_vars_chirho: tf_tvs_chirho,
                    equations_chirho,
                    span_chirho: tf_span_chirho,
                    ..
                } => {
                    let default_rhs_chirho = if equations_chirho.len() == 1 {
                        Some(equations_chirho[0].rhs_chirho.clone())
                    } else {
                        None
                    };
                    let tv_names_chirho: Vec<NameChirho> = tf_tvs_chirho
                        .iter()
                        .map(|tv_chirho| tv_chirho.name_chirho.clone())
                        .collect();
                    assoc_tfs_chirho.push(
                        haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                            name_chirho: tf_name_chirho,
                            type_vars_chirho: tv_names_chirho,
                            default_rhs_chirho,
                            span_chirho: tf_span_chirho,
                        },
                    );
                    None
                }
                // Inside class where-blocks, `type FamilyName a :: *` is parsed as
                // a TypeAliasDeclChirho. Convert it to an associated type family.
                // Also handle `type FamilyName a = DefaultType` as default assoc type.
                DeclChirho::TypeAliasDeclChirho {
                    name_chirho: ta_name_chirho,
                    type_vars_chirho: ta_tvs_chirho,
                    rhs_chirho: ta_rhs_chirho,
                    span_chirho: ta_span_chirho,
                } => {
                    let tv_names_chirho: Vec<NameChirho> = ta_tvs_chirho
                        .iter()
                        .map(|tv_chirho| tv_chirho.name_chirho.clone())
                        .collect();
                    assoc_tfs_chirho.push(
                        haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                            name_chirho: ta_name_chirho,
                            type_vars_chirho: tv_names_chirho,
                            default_rhs_chirho: Some(ta_rhs_chirho),
                            span_chirho: ta_span_chirho,
                        },
                    );
                    None
                }
                _ => None,
            })
            .collect();

        DeclChirho::ClassDeclChirho {
            context_chirho,
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            methods_chirho,
            associated_tfs_chirho: assoc_tfs_chirho,
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
        let mut paren_depth_chirho = 0usize;

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
                    if kind_chirho == TokenKindChirho::LeftParenChirho {
                        paren_depth_chirho += 1;
                    }
                    if kind_chirho == TokenKindChirho::RightParenChirho {
                        paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                    }
                    if kind_chirho == TokenKindChirho::InstanceKeywordChirho {
                        saw_instance_chirho = true;
                        continue;
                    }
                    if kind_chirho == TokenKindChirho::WhereKeywordChirho
                        && paren_depth_chirho == 0
                    {
                        saw_where_chirho = true;
                        continue;
                    }
                    if kind_chirho == TokenKindChirho::DoubleArrowChirho
                        && paren_depth_chirho == 0
                    {
                        saw_fat_arrow_chirho = true;
                        continue;
                    }
                    if saw_instance_chirho && !saw_where_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if saw_fat_arrow_chirho {
                            post_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                        } else {
                            pre_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_where_chirho
                        || n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                    {
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

        let (class_chirho, types_chirho) =
            self.lower_instance_head_types_chirho(head_tokens_chirho, span_chirho);

        // For multi-parameter type classes, keep each type argument as a
        // separate element so that the type checker can split them into
        // head_ty_chirho (first) and extra_head_tys_chirho (rest).
        let instance_type_chirho = types_chirho;

        // Merge consecutive multi-equation methods in instance declarations.
        let method_decls_chirho = merge_fun_binds_chirho(method_decls_chirho);

        // Extract associated type family instances (`type F Int = Bool`)
        // from the where-block declarations.
        let mut assoc_tf_insts_chirho: Vec<
            haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho,
        > = Vec::new();

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
                DeclChirho::TypeFamilyInstanceDeclChirho {
                    family_name_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    span_chirho,
                } => {
                    assoc_tf_insts_chirho.push(
                        haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho {
                            family_name_chirho,
                            lhs_types_chirho,
                            rhs_chirho,
                            span_chirho,
                        },
                    );
                    None
                }
                // In instance where-blocks, `type Elem [a] = a` is parsed as
                // a TypeAliasDeclChirho (no "instance" keyword inside class body).
                // Convert it to an associated type family instance.
                DeclChirho::TypeAliasDeclChirho {
                    name_chirho: ta_name_chirho,
                    type_vars_chirho: ta_tvs_chirho,
                    rhs_chirho: ta_rhs_chirho,
                    span_chirho: ta_span_chirho,
                } => {
                    // LHS types are the type variables from the alias declaration
                    let lhs_tys_chirho: Vec<TypeChirho> = ta_tvs_chirho
                        .iter()
                        .map(|tv_chirho| TypeChirho::VarChirho(tv_chirho.name_chirho.clone()))
                        .collect();
                    assoc_tf_insts_chirho.push(
                        haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho {
                            family_name_chirho: ta_name_chirho,
                            lhs_types_chirho: lhs_tys_chirho,
                            rhs_chirho: ta_rhs_chirho,
                            span_chirho: ta_span_chirho,
                        },
                    );
                    None
                }
                _ => None,
            })
            .collect();

        DeclChirho::InstanceDeclChirho {
            context_chirho,
            class_chirho: class_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            types_chirho: instance_type_chirho,
            methods_chirho,
            assoc_tf_instances_chirho: assoc_tf_insts_chirho,
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
        let mut segment_start_chirho = 0usize;
        let mut paren_depth_chirho = 0usize;

        for (idx_chirho, (tok_chirho, _span_chirho)) in tokens_chirho.iter().enumerate() {
            match tok_chirho.kind_chirho() {
                TokenKindChirho::LeftParenChirho => {
                    paren_depth_chirho += 1;
                }
                TokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                TokenKindChirho::CommaChirho if paren_depth_chirho == 0 => {
                    if let Some(constraint_chirho) = self
                        .parse_simple_constraint_segment_chirho(
                            &tokens_chirho[segment_start_chirho..idx_chirho],
                        )
                    {
                        constraints_chirho.push(constraint_chirho);
                    }
                    segment_start_chirho = idx_chirho + 1;
                }
                _ => {}
            }
        }

        if let Some(constraint_chirho) =
            self.parse_simple_constraint_segment_chirho(&tokens_chirho[segment_start_chirho..])
        {
            constraints_chirho.push(constraint_chirho);
        }

        constraints_chirho
    }

    fn parse_simple_constraint_segment_chirho(
        &self,
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
    ) -> Option<ConstraintChirho> {
        // Quantified/specialized superclass constraints are not representable in
        // ConstraintChirho yet. Ignore them here rather than mislowering them
        // into bogus simple class applications like deepseq's
        // `forall a. NFData a => NFData (f a)`.
        if tokens_chirho.iter().any(|(tok_chirho, _)| {
            matches!(
                tok_chirho.kind_chirho(),
                TokenKindChirho::ForallKeywordChirho | TokenKindChirho::DoubleArrowChirho
            )
        }) {
            return None;
        }

        let mut current_class_chirho: Option<NameChirho> = None;
        let mut current_args_chirho: Vec<TypeChirho> = Vec::new();
        let mut span_chirho = SpanChirho::DUMMY_CHIRHO;

        for (tok_chirho, token_span_chirho) in tokens_chirho {
            match tok_chirho.kind_chirho() {
                TokenKindChirho::ConSymChirho
                | TokenKindChirho::QualifiedConSymChirho
                | TokenKindChirho::ConIdChirho
                | TokenKindChirho::QualifiedConIdChirho => {
                    if current_class_chirho.is_none() {
                        current_class_chirho =
                            Some(self.name_from_token_chirho(tok_chirho, *token_span_chirho));
                        span_chirho = *token_span_chirho;
                    }
                }
                TokenKindChirho::VarIdChirho => {
                    current_args_chirho.push(TypeChirho::VarChirho(
                        self.name_from_token_chirho(tok_chirho, *token_span_chirho),
                    ));
                }
                _ => {}
            }
        }

        current_class_chirho.map(|class_chirho| ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho: current_args_chirho,
            span_chirho,
        })
    }

    fn lower_instance_head_types_chirho(
        &self,
        head_tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        span_chirho: SpanChirho,
    ) -> (Option<NameChirho>, Vec<TypeChirho>) {
        let mut idx_chirho = 0usize;
        let mut class_chirho = None;
        if let Some((tok_chirho, tok_span_chirho)) = head_tokens_chirho.first() {
            if matches!(
                tok_chirho.kind_chirho(),
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
            ) {
                class_chirho = Some(self.name_from_token_chirho(tok_chirho, *tok_span_chirho));
                idx_chirho = 1;
            }
        }

        let mut types_chirho = Vec::new();
        while idx_chirho < head_tokens_chirho.len() {
            match head_tokens_chirho[idx_chirho].0.kind_chirho() {
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                    let (tok_chirho, tok_span_chirho) = head_tokens_chirho[idx_chirho];
                    types_chirho.push(TypeChirho::ConChirho(
                        self.name_from_token_chirho(tok_chirho, tok_span_chirho),
                    ));
                    idx_chirho += 1;
                }
                TokenKindChirho::VarIdChirho => {
                    let (tok_chirho, tok_span_chirho) = head_tokens_chirho[idx_chirho];
                    types_chirho.push(TypeChirho::VarChirho(
                        self.name_from_token_chirho(tok_chirho, tok_span_chirho),
                    ));
                    idx_chirho += 1;
                }
                TokenKindChirho::LeftParenChirho => {
                    if let Some(end_idx_chirho) = Self::find_matching_token_index_chirho(
                        head_tokens_chirho,
                        idx_chirho,
                        TokenKindChirho::LeftParenChirho,
                        TokenKindChirho::RightParenChirho,
                    ) {
                        let inner_tokens_chirho = &head_tokens_chirho[idx_chirho + 1..end_idx_chirho];
                        let head_span_chirho = head_tokens_chirho[idx_chirho]
                            .1
                            .merge_chirho(head_tokens_chirho[end_idx_chirho].1)
                            .unwrap_or(span_chirho);
                        let inner_ty_chirho = self.lower_type_from_token_slice_chirho(
                            inner_tokens_chirho,
                            head_span_chirho,
                        );
                        types_chirho.push(TypeChirho::ParenChirho {
                            inner_chirho: Box::new(inner_ty_chirho),
                            span_chirho: head_span_chirho,
                        });
                        idx_chirho = end_idx_chirho + 1;
                    } else {
                        idx_chirho += 1;
                    }
                }
                TokenKindChirho::LeftBracketChirho => {
                    if let Some(end_idx_chirho) = Self::find_matching_token_index_chirho(
                        head_tokens_chirho,
                        idx_chirho,
                        TokenKindChirho::LeftBracketChirho,
                        TokenKindChirho::RightBracketChirho,
                    ) {
                        let inner_tokens_chirho = &head_tokens_chirho[idx_chirho + 1..end_idx_chirho];
                        let head_span_chirho = head_tokens_chirho[idx_chirho]
                            .1
                            .merge_chirho(head_tokens_chirho[end_idx_chirho].1)
                            .unwrap_or(span_chirho);
                        let element_ty_chirho = self.lower_type_from_token_slice_chirho(
                            inner_tokens_chirho,
                            head_span_chirho,
                        );
                        types_chirho.push(TypeChirho::ListChirho {
                            element_chirho: Box::new(element_ty_chirho),
                            span_chirho: head_span_chirho,
                        });
                        idx_chirho = end_idx_chirho + 1;
                    } else {
                        idx_chirho += 1;
                    }
                }
                _ => {
                    idx_chirho += 1;
                }
            }
        }

        (class_chirho, types_chirho)
    }

    fn lower_type_from_token_slice_chirho(
        &self,
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        span_chirho: SpanChirho,
    ) -> TypeChirho {
        if tokens_chirho.is_empty() {
            return self.placeholder_type_chirho();
        }

        let mut tuple_parts_chirho: Vec<&[(&GreenTokenChirho, SpanChirho)]> = Vec::new();
        let mut part_start_chirho = 0usize;
        let mut paren_depth_chirho = 0usize;
        let mut bracket_depth_chirho = 0usize;
        for (idx_chirho, (tok_chirho, _)) in tokens_chirho.iter().enumerate() {
            match tok_chirho.kind_chirho() {
                TokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                TokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                TokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                TokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                TokenKindChirho::CommaChirho
                    if paren_depth_chirho == 0 && bracket_depth_chirho == 0 =>
                {
                    tuple_parts_chirho.push(&tokens_chirho[part_start_chirho..idx_chirho]);
                    part_start_chirho = idx_chirho + 1;
                }
                _ => {}
            }
        }
        if part_start_chirho == 0 {
            tuple_parts_chirho.clear();
        } else {
            tuple_parts_chirho.push(&tokens_chirho[part_start_chirho..]);
        }
        if !tuple_parts_chirho.is_empty() {
            return TypeChirho::TupleChirho {
                elements_chirho: tuple_parts_chirho
                    .into_iter()
                    .map(|part_chirho| {
                        self.lower_type_from_token_slice_chirho(part_chirho, span_chirho)
                    })
                    .collect(),
                span_chirho,
            };
        }

        let mut atom_tys_chirho = Vec::new();
        let mut idx_chirho = 0usize;
        while idx_chirho < tokens_chirho.len() {
            match tokens_chirho[idx_chirho].0.kind_chirho() {
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                    let (tok_chirho, tok_span_chirho) = tokens_chirho[idx_chirho];
                    atom_tys_chirho.push(TypeChirho::ConChirho(
                        self.name_from_token_chirho(tok_chirho, tok_span_chirho),
                    ));
                    idx_chirho += 1;
                }
                TokenKindChirho::VarIdChirho => {
                    let (tok_chirho, tok_span_chirho) = tokens_chirho[idx_chirho];
                    atom_tys_chirho.push(TypeChirho::VarChirho(
                        self.name_from_token_chirho(tok_chirho, tok_span_chirho),
                    ));
                    idx_chirho += 1;
                }
                TokenKindChirho::LeftParenChirho => {
                    if let Some(end_idx_chirho) = Self::find_matching_token_index_chirho(
                        tokens_chirho,
                        idx_chirho,
                        TokenKindChirho::LeftParenChirho,
                        TokenKindChirho::RightParenChirho,
                    ) {
                        let inner_tokens_chirho = &tokens_chirho[idx_chirho + 1..end_idx_chirho];
                        let inner_ty_chirho =
                            self.lower_type_from_token_slice_chirho(inner_tokens_chirho, span_chirho);
                        atom_tys_chirho.push(TypeChirho::ParenChirho {
                            inner_chirho: Box::new(inner_ty_chirho),
                            span_chirho,
                        });
                        idx_chirho = end_idx_chirho + 1;
                    } else {
                        idx_chirho += 1;
                    }
                }
                TokenKindChirho::LeftBracketChirho => {
                    if let Some(end_idx_chirho) = Self::find_matching_token_index_chirho(
                        tokens_chirho,
                        idx_chirho,
                        TokenKindChirho::LeftBracketChirho,
                        TokenKindChirho::RightBracketChirho,
                    ) {
                        let inner_tokens_chirho = &tokens_chirho[idx_chirho + 1..end_idx_chirho];
                        let element_ty_chirho =
                            self.lower_type_from_token_slice_chirho(inner_tokens_chirho, span_chirho);
                        atom_tys_chirho.push(TypeChirho::ListChirho {
                            element_chirho: Box::new(element_ty_chirho),
                            span_chirho,
                        });
                        idx_chirho = end_idx_chirho + 1;
                    } else {
                        idx_chirho += 1;
                    }
                }
                _ => {
                    idx_chirho += 1;
                }
            }
        }

        if atom_tys_chirho.is_empty() {
            return self.placeholder_type_chirho();
        }
        let mut result_chirho = atom_tys_chirho.remove(0);
        for arg_ty_chirho in atom_tys_chirho {
            result_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(arg_ty_chirho),
                span_chirho,
            };
        }
        result_chirho
    }

    fn find_matching_token_index_chirho(
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        start_idx_chirho: usize,
        open_kind_chirho: TokenKindChirho,
        close_kind_chirho: TokenKindChirho,
    ) -> Option<usize> {
        let mut depth_chirho = 0usize;
        for (idx_chirho, (tok_chirho, _)) in tokens_chirho.iter().enumerate().skip(start_idx_chirho)
        {
            let kind_chirho = tok_chirho.kind_chirho();
            if kind_chirho == open_kind_chirho {
                depth_chirho += 1;
            } else if kind_chirho == close_kind_chirho {
                depth_chirho = depth_chirho.saturating_sub(1);
                if depth_chirho == 0 {
                    return Some(idx_chirho);
                }
            }
        }
        None
    }

    /// Convert a lowered type into a list of constraints for a qualified type
    /// context. Handles single constraints (`Eq a`), tupled constraints
    /// (`(Eq a, Show a)`), parenthesized single constraints (`(Eq a)`),
    /// equality constraints (`a ~ b`), and zero-arg constraints (`Typeable`).
    fn type_to_constraints_chirho(
        ty_chirho: &TypeChirho,
        span_chirho: SpanChirho,
    ) -> Vec<ConstraintChirho> {
        match ty_chirho {
            // Tuple: multiple constraints like (Eq a, Show a)
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .flat_map(|e_chirho| Self::type_to_constraints_chirho(e_chirho, span_chirho))
                .collect(),
            // Parens: unwrap (Eq a) → Eq a
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::type_to_constraints_chirho(inner_chirho, span_chirho)
            }
            // Application: Eq a, Monad m, etc. — collect class name and args
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho: app_span_chirho,
            } => {
                let (class_name_chirho, mut args_chirho) =
                    Self::collect_app_class_chirho(fun_chirho);
                args_chirho.push(arg_chirho.as_ref().clone());
                vec![ConstraintChirho::ClassChirho {
                    class_chirho: class_name_chirho,
                    args_chirho,
                    span_chirho: *app_span_chirho,
                }]
            }
            // Bare constructor: Typeable (zero-arg constraint)
            TypeChirho::ConChirho(name_chirho) => {
                vec![ConstraintChirho::ClassChirho {
                    class_chirho: name_chirho.clone(),
                    args_chirho: vec![],
                    span_chirho,
                }]
            }
            // Anything else — wrap as a single constraint with a synthetic name
            other_chirho => {
                vec![ConstraintChirho::ClassChirho {
                    class_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                        "?",
                        span_chirho,
                    )),
                    args_chirho: vec![other_chirho.clone()],
                    span_chirho,
                }]
            }
        }
    }

    /// Walk a left-nested `AppChirho` spine to collect the class name and
    /// preceding arguments. For `Monad m`, fun is `ConChirho("Monad")` and
    /// returns `("Monad", [])`. For `MonadReader r m`, fun is
    /// `AppChirho(ConChirho("MonadReader"), VarChirho("r"))` and returns
    /// `("MonadReader", [r])`.
    fn collect_app_class_chirho(ty_chirho: &TypeChirho) -> (NameChirho, Vec<TypeChirho>) {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => (name_chirho.clone(), vec![]),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let (name_chirho, mut args_chirho) = Self::collect_app_class_chirho(fun_chirho);
                args_chirho.push(arg_chirho.as_ref().clone());
                (name_chirho, args_chirho)
            }
            // Fallback: use the type as a synthetic name
            other_chirho => (
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    "?",
                    other_chirho.span_chirho(),
                )),
                vec![],
            ),
        }
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
                if n_chirho.kind_chirho() == SyntaxKindChirho::TypeSigDeclChirho {
                    out_chirho.extend(self.lower_type_sigs_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                    ));
                } else if let Some(decl_chirho) =
                    self.lower_decl_chirho(n_chirho, child_chirho.start_chirho)
                {
                    out_chirho.push(decl_chirho);
                }
            }
        }
    }

    /// Try to extract a DefaultSignatures-style default method signature from
    /// a DefaultDeclChirho CST node: `default methodName :: ConstrainedType`.
    /// Returns Some((method_name, raw_type_text)) if the node matches this
    /// pattern, None if it's a regular default declaration like `default (Int, Double)`.
    fn try_extract_default_sig_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<(String, String)> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        // Pattern: default VarId :: Type
        let mut saw_default_chirho = false;
        let mut method_name_chirho: Option<String> = None;
        let mut saw_double_colon_chirho = false;
        let mut type_text_parts_chirho: Vec<String> = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let kind_chirho = tok_chirho.kind_chirho();
                if kind_chirho == TokenKindChirho::DefaultKeywordChirho {
                    saw_default_chirho = true;
                    continue;
                }
                if saw_default_chirho && method_name_chirho.is_none() {
                    if kind_chirho == TokenKindChirho::VarIdChirho {
                        method_name_chirho = Some(tok_chirho.text_chirho().to_string());
                        continue;
                    } else if kind_chirho == TokenKindChirho::LeftParenChirho {
                        // This is `default (Int, Double)` — not a default sig
                        return None;
                    }
                }
                if method_name_chirho.is_some() && kind_chirho == TokenKindChirho::DoubleColonChirho
                {
                    saw_double_colon_chirho = true;
                    continue;
                }
                if saw_double_colon_chirho {
                    type_text_parts_chirho.push(tok_chirho.text_chirho().to_string());
                }
            }
        }

        if let Some(name_chirho) = method_name_chirho {
            if saw_double_colon_chirho && !type_text_parts_chirho.is_empty() {
                let type_text_chirho = type_text_parts_chirho.join(" ");
                return Some((name_chirho, type_text_chirho));
            }
        }
        None
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
    fn lower_pat_syn_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        let mut name_chirho: Option<NameChirho> = None;
        let mut args_chirho: Vec<NameChirho> = Vec::new();
        let mut saw_keyword_chirho = false;
        let mut saw_sep_chirho = false;
        let mut is_equals_chirho = false;
        let mut pat_chirho: Option<PatChirho> = None;
        let mut has_where_chirho = false;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
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
            dir_chirho,
            pat_chirho: pat_chirho.unwrap_or(PatChirho::WildcardChirho(span_chirho)),
            span_chirho,
        }
    }

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
                let s_chirho = self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);

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
                    TokenKindChirho::VarIdChirho if phase_chirho == 1 => match text_chirho {
                        "ccall" | "capi" | "stdcall" | "prim" | "javascript" | "cplusplus" => {
                            calling_conv_chirho = text_chirho.to_string();
                            phase_chirho = 2;
                        }
                        _ => {
                            hs_name_chirho =
                                Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            phase_chirho = 4;
                        }
                    },
                    TokenKindChirho::VarIdChirho if phase_chirho == 2 => match text_chirho {
                        "safe" | "unsafe" | "interruptible" => {
                            safety_chirho = Some(text_chirho.to_string());
                            phase_chirho = 3;
                        }
                        _ => {
                            hs_name_chirho =
                                Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            phase_chirho = 4;
                        }
                    },
                    TokenKindChirho::StringLiteralChirho
                        if phase_chirho == 2 || phase_chirho == 3 =>
                    {
                        let raw_chirho = text_chirho.trim_end_matches('#');
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
                        hs_name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
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

    /// Lower `deriving instance [context =>] ClassName Type1 Type2...`
    fn lower_standalone_deriving_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        let mut saw_instance_chirho = false;
        let mut saw_fat_arrow_chirho = false;

        let mut pre_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();
        let mut post_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let kind_chirho = tok_chirho.kind_chirho();
                if kind_chirho == TokenKindChirho::DerivingKeywordChirho {
                    continue;
                }
                if kind_chirho == TokenKindChirho::InstanceKeywordChirho {
                    saw_instance_chirho = true;
                    continue;
                }
                if kind_chirho == TokenKindChirho::DoubleArrowChirho {
                    saw_fat_arrow_chirho = true;
                    continue;
                }
                if saw_instance_chirho {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    if saw_fat_arrow_chirho {
                        post_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                    } else {
                        pre_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                    }
                }
            }
        }

        let head_tokens_chirho = if saw_fat_arrow_chirho {
            &post_arrow_tokens_chirho
        } else {
            &pre_arrow_tokens_chirho
        };

        let context_chirho = if saw_fat_arrow_chirho {
            self.build_instance_context_chirho(&pre_arrow_tokens_chirho)
        } else {
            vec![]
        };

        let mut class_chirho = None;
        let mut types_chirho = Vec::new();

        for (tok_chirho, s_chirho) in head_tokens_chirho {
            match tok_chirho.kind_chirho() {
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                    if class_chirho.is_none() {
                        class_chirho = Some(self.name_from_token_chirho(tok_chirho, *s_chirho));
                    } else {
                        types_chirho.push(TypeChirho::ConChirho(
                            self.name_from_token_chirho(tok_chirho, *s_chirho),
                        ));
                    }
                }
                TokenKindChirho::VarIdChirho => {
                    types_chirho.push(TypeChirho::VarChirho(
                        self.name_from_token_chirho(tok_chirho, *s_chirho),
                    ));
                }
                _ => {}
            }
        }

        DeclChirho::StandaloneDerivingDeclChirho {
            context_chirho,
            class_chirho: class_chirho.unwrap_or_else(|| {
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho("Unknown", span_chirho))
            }),
            types_chirho,
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
                        TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
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
                    types_chirho
                        .push(self.lower_type_chirho(sub_node_chirho, child_chirho.start_chirho));
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
                    mult_chirho: None,
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

    fn lower_type_chirho(&self, node_chirho: &GreenNodeChirho, base_chirho: usize) -> TypeChirho {
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

                // Detect multiplicity: look for ⊸ or %1/%Many/%m tokens
                let mut mult_chirho: Option<haskelujah_ast_chirho::ty_chirho::MultiplicityChirho> =
                    None;
                for child_chirho in &children_chirho {
                    if let GreenElementChirho::TokenChirho(tok_chirho) =
                        &child_chirho.element_chirho
                    {
                        let txt_chirho = tok_chirho.text_chirho();
                        if tok_chirho.kind_chirho() == TokenKindChirho::LinearArrowChirho {
                            // ⊸ = linear
                            mult_chirho = Some(
                                haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho,
                            );
                        } else if txt_chirho == "1" {
                            // %1 = linear
                            mult_chirho = Some(
                                haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho,
                            );
                        } else if txt_chirho == "Many" {
                            // %Many = unrestricted
                            mult_chirho = Some(
                                haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::ManyChirho,
                            );
                        }
                    }
                }

                if type_nodes_chirho.len() >= 2 {
                    let arg_chirho = self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                    let result_chirho = self.lower_type_from_child_chirho(
                        type_nodes_chirho[type_nodes_chirho.len() - 1],
                    );
                    TypeChirho::FunChirho {
                        arg_chirho: Box::new(arg_chirho),
                        mult_chirho,
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

                // Detect kind annotation: (a :: k) has a `::` token between two
                // type nodes. In type position, the kind annotation is metadata
                // for the kind checker — lower only the first type node.
                let has_double_colon_chirho = children_chirho.iter().any(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::TokenChirho(t_chirho)
                        if t_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho)
                });

                // Check for tuple type (multiple types separated by commas)
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();

                if has_double_colon_chirho && type_nodes_chirho.len() >= 1 {
                    // Kind-annotated type: (a :: k) — lower only the first type
                    let inner_chirho = self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                    TypeChirho::ParenChirho {
                        inner_chirho: Box::new(inner_chirho),
                        span_chirho,
                    }
                } else if type_nodes_chirho.len() > 1 {
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
                    // Check for operator-as-type-constructor: (:+:), (->), (+), etc.
                    let op_token_chirho = children_chirho.iter().find_map(|c_chirho| {
                        if let GreenElementChirho::TokenChirho(tok_chirho) = c_chirho.element_chirho
                        {
                            let k_chirho = tok_chirho.kind_chirho();
                            if k_chirho == TokenKindChirho::VarSymChirho
                                || k_chirho == TokenKindChirho::ConSymChirho
                                || k_chirho == TokenKindChirho::RightArrowChirho
                            {
                                let s_chirho =
                                    self.span_chirho(c_chirho.start_chirho, c_chirho.end_chirho);
                                return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            }
                        }
                        None
                    });
                    if let Some(op_name_chirho) = op_token_chirho {
                        TypeChirho::ConChirho(op_name_chirho)
                    } else {
                        // Unit type ()
                        TypeChirho::TupleChirho {
                            elements_chirho: vec![],
                            span_chirho,
                        }
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
                // context => type — lower with proper constraint preservation
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut saw_arrow_chirho = false;
                let mut pre_arrow_chirho: Vec<&ChildChirho<'_>> = Vec::new();
                let mut post_arrow_chirho: Vec<&ChildChirho<'_>> = Vec::new();

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho
                    {
                        if tok_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho {
                            saw_arrow_chirho = true;
                            continue;
                        }
                    }
                    if saw_arrow_chirho {
                        post_arrow_chirho.push(child_chirho);
                    } else {
                        pre_arrow_chirho.push(child_chirho);
                    }
                }

                // Lower body type from post-arrow children
                let body_chirho = if let Some(child_chirho) = post_arrow_chirho.iter().find(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                }) {
                    self.lower_type_from_child_chirho(child_chirho)
                } else {
                    self.placeholder_type_chirho()
                };

                // Lower context from pre-arrow children
                let context_type_chirho = pre_arrow_chirho.iter().find(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                }).map(|child_chirho| self.lower_type_from_child_chirho(child_chirho));

                // Convert context type to constraints
                let context_chirho = if let Some(ctx_ty_chirho) = context_type_chirho {
                    Self::type_to_constraints_chirho(&ctx_ty_chirho, span_chirho)
                } else {
                    Vec::new()
                };

                if context_chirho.is_empty() {
                    body_chirho
                } else {
                    TypeChirho::QualChirho {
                        context_chirho,
                        body_chirho: Box::new(body_chirho),
                        span_chirho,
                    }
                }
            }
            SyntaxKindChirho::ForallTypeChirho => {
                // forall a b (c :: k) . Type
                // Children: ForallKeyword, VarId*, ParenType*, VarSym("."), type nodes
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
                                vars_chirho
                                    .push(self.name_from_token_chirho(tok_chirho, s_chirho).into());
                            } else if kind_chirho == TokenKindChirho::VarSymChirho
                                && tok_chirho.text_chirho() == "."
                            {
                                saw_dot_chirho = true;
                            }
                        }
                        GreenElementChirho::NodeChirho(n_chirho) if !saw_dot_chirho => {
                            // Before the dot: handle kind-annotated binders (a :: k)
                            if n_chirho.kind_chirho() == SyntaxKindChirho::ParenTypeChirho {
                                if let Some(tv_chirho) = self
                                    .try_lower_kind_annotated_forall_binder_chirho(
                                        n_chirho,
                                        child_chirho.start_chirho,
                                    )
                                {
                                    vars_chirho.push(tv_chirho);
                                }
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
            SyntaxKindChirho::PromotedConTypeChirho => {
                // DataKinds promoted constructor: 'True, 'Just, etc.
                // Children: Tick token, ConId token
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let con_child_chirho = children_chirho.iter().find(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::TokenChirho(t_chirho)
                        if t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                           || t_chirho.kind_chirho() == TokenKindChirho::QualifiedConIdChirho)
                });
                if let Some(child_chirho) = con_child_chirho {
                    if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                        TypeChirho::PromotedConChirho {
                            name_chirho,
                            span_chirho,
                        }
                    } else {
                        self.placeholder_type_chirho()
                    }
                } else {
                    // Promoted tuple: '() — extract text from all tokens
                    TypeChirho::PromotedConChirho {
                        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                            "()",
                            span_chirho,
                        )),
                        span_chirho,
                    }
                }
            }
            SyntaxKindChirho::PromotedListTypeChirho => {
                // DataKinds promoted list: '[], '[Int, Bool], etc.
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho)
                            if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();
                let elements_chirho: Vec<_> = type_nodes_chirho
                    .iter()
                    .map(|tc_chirho| self.lower_type_from_child_chirho(tc_chirho))
                    .collect();
                TypeChirho::PromotedListChirho {
                    elements_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::WildcardTypeChirho => {
                // PartialTypeSignatures: `_` in type position lowers to a
                // wildcard that the type inferencer will replace with a fresh
                // unification variable (and optionally emit warning W4201).
                TypeChirho::WildcardChirho { span_chirho }
            }
            SyntaxKindChirho::LitTypeChirho => {
                // DataKinds: type-level literals (42, "hello", 'x')
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let value_chirho = children_chirho
                    .iter()
                    .find_map(|c_chirho| {
                        if let GreenElementChirho::TokenChirho(tok_chirho) = c_chirho.element_chirho
                        {
                            Some(tok_chirho.text_chirho().to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();
                TypeChirho::LitChirho {
                    value_chirho,
                    span_chirho,
                }
            }
            SyntaxKindChirho::InfixTypeChirho => {
                // TypeOperators: `a :+: b` or `a `Either` b`
                // Children: left-type, operator-token (or backtick-name-backtick), right-type
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho)
                            if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .collect();
                // Extract the operator name from tokens
                let op_name_chirho = children_chirho.iter().find_map(|c_chirho| {
                    if let GreenElementChirho::TokenChirho(tok_chirho) = c_chirho.element_chirho {
                        let k_chirho = tok_chirho.kind_chirho();
                        if k_chirho == TokenKindChirho::VarSymChirho
                            || k_chirho == TokenKindChirho::ConSymChirho
                            || k_chirho == TokenKindChirho::ConIdChirho
                            || k_chirho == TokenKindChirho::VarIdChirho
                            || k_chirho == TokenKindChirho::TildeChirho
                        {
                            let txt_chirho = tok_chirho.text_chirho();
                            if txt_chirho != "`" {
                                let s_chirho =
                                    self.span_chirho(c_chirho.start_chirho, c_chirho.end_chirho);
                                return Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            }
                        }
                    }
                    None
                });
                if type_nodes_chirho.len() >= 2 {
                    let left_chirho = self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                    let right_chirho = self.lower_type_from_child_chirho(
                        type_nodes_chirho[type_nodes_chirho.len() - 1],
                    );
                    let op_ty_chirho = if let Some(name_chirho) = op_name_chirho {
                        TypeChirho::ConChirho(name_chirho)
                    } else {
                        self.placeholder_type_chirho()
                    };
                    // Desugar: `a Op b` → `Op a b` = App(App(Op, a), b)
                    TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(op_ty_chirho),
                            arg_chirho: Box::new(left_chirho),
                            span_chirho,
                        }),
                        arg_chirho: Box::new(right_chirho),
                        span_chirho,
                    }
                } else {
                    self.placeholder_type_chirho()
                }
            }
            _ => {
                // Fallback: try to extract a name
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                if name_chirho
                    .text_chirho()
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_uppercase())
                {
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

    fn lower_expr_chirho(&self, node_chirho: &GreenNodeChirho, base_chirho: usize) -> ExprChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());

        match node_chirho.kind_chirho() {
            SyntaxKindChirho::NameExprChirho => {
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
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
                        con_chirho: name_chirho,
                        fields_chirho,
                        has_wildcard_chirho,
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
                                    || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho)
                            {
                                // Backtick-enclosed name used as infix operator:
                                // `div`, `mod`, `elem`, etc.
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                ops_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                            } else if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                                || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            {
                                let s_chirho = self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                );
                                ops_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
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
                                    if alt_n_chirho.kind_chirho() == SyntaxKindChirho::CaseAltChirho
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
            // MultiWayIf: if | g1 -> e1 | g2 -> e2  →  nested IfChirho
            SyntaxKindChirho::MultiWayIfExprChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                // Collect (guard, result) pairs from tokens/nodes after `if`
                let mut guards_chirho: Vec<(ExprChirho, ExprChirho)> = Vec::new();
                let mut expr_buf_chirho: Vec<ExprChirho> = Vec::new();
                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            let kind_chirho = tok_chirho.kind_chirho();
                            if kind_chirho == TokenKindChirho::PipeChirho {
                                // Flush previous pair if we have 2 exprs
                                if expr_buf_chirho.len() >= 2 {
                                    let result_chirho = expr_buf_chirho.pop().unwrap();
                                    let guard_chirho = expr_buf_chirho.pop().unwrap();
                                    guards_chirho.push((guard_chirho, result_chirho));
                                }
                                expr_buf_chirho.clear();
                            }
                            // skip if, ->, whitespace, etc.
                        }
                        GreenElementChirho::NodeChirho(n_chirho) => {
                            if is_expr_kind_chirho(n_chirho.kind_chirho()) {
                                expr_buf_chirho.push(
                                    self.lower_expr_chirho(n_chirho, child_chirho.start_chirho),
                                );
                            }
                        }
                    }
                }
                // Flush last pair
                if expr_buf_chirho.len() >= 2 {
                    let result_chirho = expr_buf_chirho.pop().unwrap();
                    let guard_chirho = expr_buf_chirho.pop().unwrap();
                    guards_chirho.push((guard_chirho, result_chirho));
                }

                // Build nested if-then-else from end
                if guards_chirho.is_empty() {
                    self.placeholder_expr_chirho()
                } else {
                    // Start with the last guard pair; use error as else for non-exhaustive
                    let (last_guard_chirho, last_result_chirho) = guards_chirho.pop().unwrap();
                    let mut acc_chirho = ExprChirho::IfChirho {
                        cond_chirho: Box::new(last_guard_chirho),
                        then_chirho: Box::new(last_result_chirho),
                        else_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(NameChirho::RawChirho(
                                RawNameChirho::unqualified_chirho("error", span_chirho),
                            ))),
                            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::StringChirho(
                                "Non-exhaustive guards in multi-way if".into(),
                                span_chirho,
                            ))),
                            span_chirho,
                        }),
                        span_chirho,
                    };
                    for (guard_chirho, result_chirho) in guards_chirho.into_iter().rev() {
                        acc_chirho = ExprChirho::IfChirho {
                            cond_chirho: Box::new(guard_chirho),
                            then_chirho: Box::new(result_chirho),
                            else_chirho: Box::new(acc_chirho),
                            span_chirho,
                        };
                    }
                    acc_chirho
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
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut stmts_chirho = Vec::new();

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                        match n_chirho.kind_chirho() {
                            SyntaxKindChirho::DoStmtChirho => {
                                let expr_chirho = self.lower_first_expr_in_node_chirho(
                                    n_chirho,
                                    child_chirho.start_chirho,
                                );
                                stmts_chirho.push(StmtChirho::ExprChirho(expr_chirho));
                            }
                            SyntaxKindChirho::BindStmtChirho => {
                                stmts_chirho.push(
                                    self.lower_bind_stmt_chirho(
                                        n_chirho,
                                        child_chirho.start_chirho,
                                    ),
                                );
                            }
                            SyntaxKindChirho::LetStmtChirho => {
                                let let_binds_chirho = self.lower_let_stmt_binds_chirho(
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
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

                // Detect operator sections by checking for operator tokens
                // outside of expression nodes.
                let mut leading_op_chirho: Option<String> = None;
                let mut trailing_op_chirho: Option<String> = None;
                let mut expr_nodes_chirho: Vec<&ChildChirho<'_>> = Vec::new();
                let mut inside_backticks_chirho = false;
                let mut backtick_op_text_chirho: Option<String> = None;

                for (_idx_chirho, child_chirho) in children_chirho.iter().enumerate() {
                    match &child_chirho.element_chirho {
                        GreenElementChirho::NodeChirho(_) => {
                            expr_nodes_chirho.push(child_chirho);
                        }
                        GreenElementChirho::TokenChirho(tok_chirho) => {
                            let kind_chirho = tok_chirho.kind_chirho();
                            if kind_chirho == TokenKindChirho::BacktickChirho {
                                if inside_backticks_chirho {
                                    if let Some(op_text_chirho) = backtick_op_text_chirho.take() {
                                        if expr_nodes_chirho.is_empty() {
                                            leading_op_chirho = Some(op_text_chirho);
                                        } else {
                                            trailing_op_chirho = Some(op_text_chirho);
                                        }
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
                                )
                            {
                                backtick_op_text_chirho =
                                    Some(tok_chirho.text_chirho().to_string());
                            } else if kind_chirho == TokenKindChirho::VarSymChirho
                                || kind_chirho == TokenKindChirho::ConSymChirho
                            {
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
                if let Some(ref op_text_chirho) = leading_op_chirho {
                    if let Some(arg_child_chirho) = expr_nodes_chirho.first() {
                        let arg_expr_chirho = self.lower_expr_from_child_chirho(arg_child_chirho);
                        return ExprChirho::LeftSectionChirho {
                            op_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                                op_text_chirho.clone(),
                                span_chirho,
                            )),
                            arg_chirho: Box::new(arg_expr_chirho),
                            span_chirho,
                        };
                    }
                }

                // Right section: (expr op) — operator token after an expr node
                if let Some(ref op_text_chirho) = trailing_op_chirho {
                    if let Some(arg_child_chirho) = expr_nodes_chirho.first() {
                        let arg_expr_chirho = self.lower_expr_from_child_chirho(arg_child_chirho);
                        return ExprChirho::RightSectionChirho {
                            arg_chirho: Box::new(arg_expr_chirho),
                            op_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                                op_text_chirho.clone(),
                                span_chirho,
                            )),
                            span_chirho,
                        };
                    }
                }

                // Fallback: check if the single child node is an InfixExprChirho
                // with a trailing operator (right section) or leading operator
                // (left section). The parser consumes the operator into an
                // InfixExprChirho, so it won't appear as a direct token child
                // of ParenExprChirho.
                if expr_nodes_chirho.len() == 1
                    && leading_op_chirho.is_none()
                    && trailing_op_chirho.is_none()
                {
                    if let GreenElementChirho::NodeChirho(inner_node_chirho) =
                        expr_nodes_chirho[0].element_chirho
                    {
                        if inner_node_chirho.kind_chirho() == SyntaxKindChirho::InfixExprChirho {
                            let inner_children_chirho = self.semantic_children_chirho(
                                inner_node_chirho,
                                expr_nodes_chirho[0].start_chirho,
                            );
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
                                            if inner_inside_backticks_chirho {
                                                if let Some(op_text_chirho) =
                                                    inner_backtick_op_text_chirho.take()
                                                {
                                                    let s_chirho = self.span_chirho(
                                                        ic_chirho.start_chirho,
                                                        ic_chirho.end_chirho,
                                                    );
                                                    inner_ops_chirho.push((op_text_chirho, s_chirho));
                                                }
                                            }
                                            inner_inside_backticks_chirho =
                                                !inner_inside_backticks_chirho;
                                        } else if inner_inside_backticks_chirho
                                            && matches!(
                                                k_chirho,
                                                TokenKindChirho::VarIdChirho
                                                    | TokenKindChirho::ConIdChirho
                                                    | TokenKindChirho::QualifiedVarIdChirho
                                                    | TokenKindChirho::QualifiedConIdChirho
                                                    | TokenKindChirho::VarSymChirho
                                                    | TokenKindChirho::ConSymChirho
                                            )
                                        {
                                            inner_backtick_op_text_chirho =
                                                Some(tok_chirho.text_chirho().to_string());
                                        } else if k_chirho == TokenKindChirho::VarSymChirho
                                            || k_chirho == TokenKindChirho::ConSymChirho
                                        {
                                            let s_chirho = self.span_chirho(
                                                ic_chirho.start_chirho,
                                                ic_chirho.end_chirho,
                                            );
                                            inner_ops_chirho.push((
                                                tok_chirho.text_chirho().to_string(),
                                                s_chirho,
                                            ));
                                        }
                                    }
                                }
                            }
                            // Right section: N exprs, N ops → trailing operator has no RHS
                            if !inner_ops_chirho.is_empty()
                                && inner_expr_nodes_chirho.len() == inner_ops_chirho.len()
                            {
                                let (op_text_chirho, _op_span_chirho) =
                                    inner_ops_chirho.last().unwrap().clone();
                                // Lower the expression part (everything before the trailing op)
                                let arg_expr_chirho = if inner_expr_nodes_chirho.len() == 1 {
                                    self.lower_expr_from_child_chirho(inner_expr_nodes_chirho[0])
                                } else {
                                    // Multiple exprs before trailing op: rebuild infix
                                    let sub_exprs_chirho: Vec<ExprChirho> = inner_expr_nodes_chirho
                                        .iter()
                                        .map(|c_chirho| self.lower_expr_from_child_chirho(c_chirho))
                                        .collect();
                                    let sub_ops_chirho: Vec<NameChirho> = inner_ops_chirho
                                        [..inner_ops_chirho.len() - 1]
                                        .iter()
                                        .map(|(t_chirho, s_chirho)| {
                                            NameChirho::RawChirho(
                                                RawNameChirho::unqualified_chirho(
                                                    t_chirho.clone(),
                                                    *s_chirho,
                                                ),
                                            )
                                        })
                                        .collect();
                                    resolve_infix_precedence_chirho(
                                        sub_exprs_chirho,
                                        sub_ops_chirho,
                                        span_chirho,
                                    )
                                };
                                return ExprChirho::RightSectionChirho {
                                    arg_chirho: Box::new(arg_expr_chirho),
                                    op_chirho: NameChirho::RawChirho(
                                        RawNameChirho::unqualified_chirho(
                                            op_text_chirho,
                                            span_chirho,
                                        ),
                                    ),
                                    span_chirho,
                                };
                            }
                            // Left section: 0 exprs before first op, 1+ exprs after
                            // (op expr) already handled above for direct tokens;
                            // this catches case where parse_expr made the op part of InfixExpr
                        }
                    }
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
                                    slots_chirho.push(Some(self.lower_expr_from_child_chirho(
                                        expr_nodes_chirho[expr_idx_chirho],
                                    )));
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
                                let n_chirho = NameChirho::RawChirho(
                                    RawNameChirho::unqualified_chirho(&name_chirho, span_chirho),
                                );
                                params_chirho.push(PatChirho::VarChirho(n_chirho.clone()));
                                elements_chirho.push(ExprChirho::VarChirho(n_chirho));
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
            SyntaxKindChirho::LeftSectionExprChirho | SyntaxKindChirho::RightSectionExprChirho => {
                // These are unused — sections are handled inside ParenExprChirho
                self.placeholder_expr_chirho()
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
                                    current_qual_parts_chirho.push(QualPartChirho::ArrowChirho);
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
                            if let Some(qual_chirho) = self
                                .flush_qual_parts_chirho(&current_qual_parts_chirho, span_chirho)
                            {
                                quals_chirho.push(qual_chirho);
                            }
                        }

                        let body_chirho =
                            body_expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho());

                        ExprChirho::ListCompChirho {
                            body_chirho: Box::new(body_chirho),
                            quals_chirho,
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
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                            if n_chirho.kind_chirho().is_type_chirho() {
                                return Some(
                                    self.lower_type_chirho(n_chirho, c_chirho.start_chirho),
                                );
                            }
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
                        if let GreenElementChirho::NodeChirho(n_chirho) = c_chirho.element_chirho {
                            if is_pat_kind_chirho(n_chirho.kind_chirho()) {
                                return Some(
                                    self.lower_pat_chirho(n_chirho, c_chirho.start_chirho),
                                );
                            }
                        }
                        None
                    })
                    .unwrap_or_else(|| PatChirho::WildcardChirho(span_chirho));
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
                    result_chirho
                        .push(self.lower_field_decl_chirho(n_chirho, child_chirho.start_chirho));
                }
            }
        }
        result_chirho
    }

    /// Lower a `FieldDeclChirho` CST node to an AST `FieldDeclChirho`.
    /// CST structure: VarIdChirho [, VarIdChirho]* :: TypeChirho
    ///
    /// The CST parser produces flat tokens for field types (no structured
    /// type subnodes), so we reconstruct the type from the flat sequence
    /// after `::` using `type_from_flat_children_chirho`.
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
        let mut type_children_chirho: Vec<&ChildChirho> = Vec::new();
        let mut pending_strict_chirho = false;

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
                    } else if saw_double_colon_chirho {
                        // Detect `!` strictness annotations on field types
                        if tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            && tok_chirho.text_chirho() == "!"
                        {
                            pending_strict_chirho = true;
                        } else {
                            type_children_chirho.push(child_chirho);
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_double_colon_chirho {
                        if ty_chirho.is_none() && is_type_kind_chirho(n_chirho.kind_chirho()) {
                            // Structured type node — use it directly
                            ty_chirho =
                                Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                        } else {
                            type_children_chirho.push(child_chirho);
                        }
                    }
                }
            }
        }

        // If no structured type node was found, reconstruct from flat tokens
        if ty_chirho.is_none() && !type_children_chirho.is_empty() {
            ty_chirho =
                Some(self.type_from_flat_children_chirho(&type_children_chirho, span_chirho));
        }

        let strictness_chirho = if pending_strict_chirho {
            StrictnessChirho::StrictChirho
        } else {
            StrictnessChirho::LazyChirho
        };

        FieldDeclChirho {
            names_chirho,
            ty_chirho: ty_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
            strictness_chirho,
            span_chirho,
        }
    }

    /// Reconstruct a `TypeChirho` from a flat sequence of tokens/nodes.
    /// Handles the common patterns found in record field declarations:
    ///   - `ConId` → ConChirho
    ///   - `VarId` → VarChirho
    ///   - `(types)` → parenthesized / tuple
    ///   - `[type]` → list
    ///   - `a -> b` → function
    ///   - `f a b` → application chain
    ///   - `forall a. type` → forall
    fn type_from_flat_children_chirho(
        &self,
        children_chirho: &[&ChildChirho],
        fallback_span_chirho: SpanChirho,
    ) -> TypeChirho {
        if children_chirho.is_empty() {
            return self.placeholder_type_chirho();
        }

        // Split on top-level `->` (function type)
        let arrow_idx_chirho = self.find_top_level_arrow_chirho(children_chirho);
        if let Some(idx_chirho) = arrow_idx_chirho {
            let lhs_chirho = &children_chirho[..idx_chirho];
            let rhs_chirho = &children_chirho[idx_chirho + 1..];
            let arg_chirho = self.type_from_flat_children_chirho(lhs_chirho, fallback_span_chirho);
            let result_chirho =
                self.type_from_flat_children_chirho(rhs_chirho, fallback_span_chirho);
            return TypeChirho::FunChirho {
                arg_chirho: Box::new(arg_chirho),
                mult_chirho: None,
                result_chirho: Box::new(result_chirho),
                span_chirho: fallback_span_chirho,
            };
        }

        // Split on top-level `=>` (qualified type with context)
        let double_arrow_idx_chirho =
            self.find_top_level_token_chirho(children_chirho, TokenKindChirho::DoubleArrowChirho);
        if let Some(idx_chirho) = double_arrow_idx_chirho {
            // Context before `=>` — skip it and parse just the result type
            let rhs_chirho = &children_chirho[idx_chirho + 1..];
            return self.type_from_flat_children_chirho(rhs_chirho, fallback_span_chirho);
        }

        // Handle `forall`
        if let Some(GreenElementChirho::TokenChirho(tok_chirho)) = children_chirho
            .first()
            .map(|c_chirho| c_chirho.element_chirho)
        {
            if tok_chirho.kind_chirho() == TokenKindChirho::ForallKeywordChirho {
                // Find the `.` (VarSym ".")
                let dot_idx_chirho = children_chirho.iter().position(|c_chirho| {
                    if let GreenElementChirho::TokenChirho(t_chirho) = c_chirho.element_chirho {
                        t_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                            && t_chirho.text_chirho() == "."
                    } else {
                        false
                    }
                });
                if let Some(di_chirho) = dot_idx_chirho {
                    // Collect type vars between forall and dot
                    let mut vars_chirho: Vec<haskelujah_ast_chirho::decl_chirho::TyVarChirho> =
                        Vec::new();
                    for c_chirho in &children_chirho[1..di_chirho] {
                        if let GreenElementChirho::TokenChirho(t_chirho) = c_chirho.element_chirho {
                            if t_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                                let s_chirho =
                                    self.span_chirho(c_chirho.start_chirho, c_chirho.end_chirho);
                                vars_chirho.push(
                                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(
                                        self.name_from_token_chirho(t_chirho, s_chirho),
                                    ),
                                );
                            }
                        }
                    }
                    let body_chirho = self.type_from_flat_children_chirho(
                        &children_chirho[di_chirho + 1..],
                        fallback_span_chirho,
                    );
                    return TypeChirho::ForallChirho {
                        vars_chirho,
                        body_chirho: Box::new(body_chirho),
                        span_chirho: fallback_span_chirho,
                    };
                }
            }
        }

        // Build application chain from atom types
        let atoms_chirho = self.collect_type_atoms_chirho(children_chirho, fallback_span_chirho);
        if atoms_chirho.is_empty() {
            return self.placeholder_type_chirho();
        }
        let mut result_chirho = atoms_chirho[0].clone();
        for atom_chirho in &atoms_chirho[1..] {
            result_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(atom_chirho.clone()),
                span_chirho: fallback_span_chirho,
            };
        }
        result_chirho
    }

    /// Find a top-level `->` token (not inside parens/brackets).
    fn find_top_level_arrow_chirho(&self, children_chirho: &[&ChildChirho]) -> Option<usize> {
        self.find_top_level_token_chirho(children_chirho, TokenKindChirho::RightArrowChirho)
    }

    /// Find a top-level token of a specific kind (not inside parens/brackets).
    fn find_top_level_token_chirho(
        &self,
        children_chirho: &[&ChildChirho],
        kind_chirho: TokenKindChirho,
    ) -> Option<usize> {
        let mut depth_chirho = 0i32;
        for (i_chirho, c_chirho) in children_chirho.iter().enumerate() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = c_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::LeftParenChirho | TokenKindChirho::LeftBracketChirho => {
                        depth_chirho += 1;
                    }
                    TokenKindChirho::RightParenChirho | TokenKindChirho::RightBracketChirho => {
                        depth_chirho -= 1;
                    }
                    k_chirho if k_chirho == kind_chirho && depth_chirho == 0 => {
                        return Some(i_chirho);
                    }
                    _ => {}
                }
            }
        }
        None
    }

    /// Parse atomic types from a flat token/node sequence.
    fn collect_type_atoms_chirho(
        &self,
        children_chirho: &[&ChildChirho],
        fallback_span_chirho: SpanChirho,
    ) -> Vec<TypeChirho> {
        let mut atoms_chirho = Vec::new();
        let mut i_chirho = 0;
        while i_chirho < children_chirho.len() {
            let child_chirho = children_chirho[i_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    match tok_chirho.kind_chirho() {
                        TokenKindChirho::VarIdChirho => {
                            atoms_chirho.push(TypeChirho::VarChirho(
                                self.name_from_token_chirho(tok_chirho, s_chirho),
                            ));
                        }
                        TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                            atoms_chirho.push(TypeChirho::ConChirho(
                                self.name_from_token_chirho(tok_chirho, s_chirho),
                            ));
                        }
                        TokenKindChirho::LeftParenChirho => {
                            // Collect everything until matching RightParen
                            let start_chirho = i_chirho;
                            let mut depth_chirho = 1i32;
                            i_chirho += 1;
                            while i_chirho < children_chirho.len() && depth_chirho > 0 {
                                if let GreenElementChirho::TokenChirho(t_chirho) =
                                    children_chirho[i_chirho].element_chirho
                                {
                                    if t_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                                        depth_chirho += 1;
                                    } else if t_chirho.kind_chirho()
                                        == TokenKindChirho::RightParenChirho
                                    {
                                        depth_chirho -= 1;
                                    }
                                }
                                if depth_chirho > 0 {
                                    i_chirho += 1;
                                }
                            }
                            // Inner children: start+1 .. i_chirho (exclusive of parens)
                            let inner_chirho: Vec<&ChildChirho> =
                                children_chirho[start_chirho + 1..i_chirho].to_vec();
                            // Check for tuple (top-level commas)
                            let comma_positions_chirho: Vec<usize> = {
                                let mut positions_chirho = Vec::new();
                                let mut d_chirho = 0i32;
                                for (j_chirho, c_chirho) in inner_chirho.iter().enumerate() {
                                    if let GreenElementChirho::TokenChirho(t_chirho) =
                                        c_chirho.element_chirho
                                    {
                                        match t_chirho.kind_chirho() {
                                            TokenKindChirho::LeftParenChirho
                                            | TokenKindChirho::LeftBracketChirho => {
                                                d_chirho += 1;
                                            }
                                            TokenKindChirho::RightParenChirho
                                            | TokenKindChirho::RightBracketChirho => {
                                                d_chirho -= 1;
                                            }
                                            TokenKindChirho::CommaChirho if d_chirho == 0 => {
                                                positions_chirho.push(j_chirho);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                positions_chirho
                            };
                            if !comma_positions_chirho.is_empty() {
                                // Tuple type
                                let mut elements_chirho = Vec::new();
                                let mut start_j_chirho = 0;
                                for &comma_j_chirho in &comma_positions_chirho {
                                    let segment_chirho: Vec<&ChildChirho> =
                                        inner_chirho[start_j_chirho..comma_j_chirho].to_vec();
                                    elements_chirho.push(self.type_from_flat_children_chirho(
                                        &segment_chirho,
                                        fallback_span_chirho,
                                    ));
                                    start_j_chirho = comma_j_chirho + 1;
                                }
                                // Last segment after final comma
                                let last_chirho: Vec<&ChildChirho> =
                                    inner_chirho[start_j_chirho..].to_vec();
                                elements_chirho.push(self.type_from_flat_children_chirho(
                                    &last_chirho,
                                    fallback_span_chirho,
                                ));
                                atoms_chirho.push(TypeChirho::TupleChirho {
                                    elements_chirho,
                                    span_chirho: fallback_span_chirho,
                                });
                            } else if inner_chirho.is_empty() {
                                // Unit type ()
                                atoms_chirho.push(TypeChirho::TupleChirho {
                                    elements_chirho: vec![],
                                    span_chirho: fallback_span_chirho,
                                });
                            } else {
                                // Parenthesized type
                                atoms_chirho.push(TypeChirho::ParenChirho {
                                    inner_chirho: Box::new(self.type_from_flat_children_chirho(
                                        &inner_chirho,
                                        fallback_span_chirho,
                                    )),
                                    span_chirho: fallback_span_chirho,
                                });
                            }
                            // Skip past the closing paren
                            i_chirho += 1;
                            continue;
                        }
                        TokenKindChirho::LeftBracketChirho => {
                            // List type [a]
                            i_chirho += 1;
                            let mut inner_chirho: Vec<&ChildChirho> = Vec::new();
                            while i_chirho < children_chirho.len() {
                                if let GreenElementChirho::TokenChirho(t_chirho) =
                                    children_chirho[i_chirho].element_chirho
                                {
                                    if t_chirho.kind_chirho() == TokenKindChirho::RightBracketChirho
                                    {
                                        break;
                                    }
                                }
                                inner_chirho.push(children_chirho[i_chirho]);
                                i_chirho += 1;
                            }
                            let elem_ty_chirho = self.type_from_flat_children_chirho(
                                &inner_chirho,
                                fallback_span_chirho,
                            );
                            atoms_chirho.push(TypeChirho::ListChirho {
                                element_chirho: Box::new(elem_ty_chirho),
                                span_chirho: fallback_span_chirho,
                            });
                            i_chirho += 1; // skip ]
                            continue;
                        }
                        // Ignore commas at top level (handled by caller)
                        TokenKindChirho::CommaChirho => {}
                        // Skip other tokens (arrows, etc.)
                        _ => {}
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    // Structured type node — lower it directly
                    if is_type_kind_chirho(n_chirho.kind_chirho()) {
                        atoms_chirho
                            .push(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
            i_chirho += 1;
        }
        atoms_chirho
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
                        field_name_chirho =
                            Some(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
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
                                let n_chirho =
                                    parse_integer_literal_chirho(tok_chirho.text_chirho());
                                value_chirho = Some(ExprChirho::LitChirho(LitChirho::IntChirho(
                                    n_chirho,
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
                                value_chirho = Some(ExprChirho::LitChirho(
                                    LitChirho::StringChirho(s_chirho, span_chirho),
                                ));
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
                                )));
                            }
                            TokenKindChirho::VarIdChirho
                            | TokenKindChirho::UnderscoreReservedIdChirho => {
                                // UnderscoreReservedIdChirho: typed holes (_) in expression position
                                value_chirho = Some(ExprChirho::VarChirho(NameChirho::RawChirho(
                                    RawNameChirho::unqualified_chirho(
                                        tok_chirho.text_chirho().to_string(),
                                        span_chirho,
                                    ),
                                )));
                            }
                            TokenKindChirho::ConIdChirho => {
                                value_chirho = Some(ExprChirho::ConChirho(NameChirho::RawChirho(
                                    RawNameChirho::unqualified_chirho(
                                        tok_chirho.text_chirho().to_string(),
                                        span_chirho,
                                    ),
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
                ExprChirho::VarChirho(fname_chirho.clone())
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
                        where_binds_chirho =
                            self.lower_where_clause_chirho(n_chirho, child_chirho.start_chirho);
                    } else if saw_arrow_chirho && rhs_chirho.is_none() {
                        rhs_chirho =
                            Some(self.lower_expr_chirho(n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        AltChirho {
            pat_chirho: pat_chirho.unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
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
            pat_chirho: pat_chirho.unwrap_or(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
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
                        let val_chirho = parse_integer_literal_chirho(tok_chirho.text_chirho());
                        return LitChirho::IntChirho(val_chirho, span_chirho);
                    }
                    TokenKindChirho::FloatLiteralChirho => {
                        let clean_chirho: String = tok_chirho
                            .text_chirho()
                            .trim_end_matches('#')
                            .chars()
                            .filter(|c_chirho| *c_chirho != '_')
                            .collect();
                        let val_chirho = clean_chirho.parse::<f64>().unwrap_or(0.0);
                        return LitChirho::FloatChirho(val_chirho, span_chirho);
                    }
                    TokenKindChirho::CharLiteralChirho => {
                        let text_chirho = tok_chirho.text_chirho().trim_end_matches('#');
                        let inner_chirho = text_chirho.trim_matches('\'');
                        let ch_chirho = unescape_char_chirho(inner_chirho);
                        return LitChirho::CharChirho(ch_chirho, span_chirho);
                    }
                    TokenKindChirho::StringLiteralChirho => {
                        // MagicHash: strip trailing # before quote extraction
                        let text_chirho = tok_chirho.text_chirho().trim_end_matches('#');
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

    fn lower_pat_chirho(&self, node_chirho: &GreenNodeChirho, base_chirho: usize) -> PatChirho {
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
                            args_chirho
                                .push(self.lower_pat_chirho(n_chirho, child_chirho.start_chirho));
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
                                con_name_chirho =
                                    Some(self.name_from_token_chirho(tok_chirho, s_chirho));
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
                                    current_field_name_chirho =
                                        Some(self.name_from_token_chirho(tok_chirho, s_chirho));
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

                // Detect `..` wildcard token (RecordWildCards)
                let mut has_wildcard_chirho = false;
                for child_chirho in &children_chirho {
                    if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho
                    {
                        if tok_chirho.kind_chirho() == TokenKindChirho::DotDotChirho {
                            has_wildcard_chirho = true;
                            break;
                        }
                    }
                }

                PatChirho::RecordChirho {
                    con_chirho: con_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
                    fields_chirho,
                    has_wildcard_chirho,
                    span_chirho,
                }
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
                    inner_pat_chirho.unwrap_or_else(|| PatChirho::WildcardChirho(span_chirho));
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
                        expr_chirho.unwrap_or(ExprChirho::VarChirho(self.dummy_name_chirho())),
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
                                let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
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
        // Split qualified names like "Data.List.sort" or "Data.List.+", but
        // do NOT treat bare operator names like ":.:" as qualified just
        // because they contain a dot character.
        if let Some(dot_pos_chirho) = text_chirho.rfind('.') {
            let qualifier_chirho = &text_chirho[..dot_pos_chirho];
            let local_chirho = &text_chirho[dot_pos_chirho + 1..];
            let qualifier_is_module_path_chirho = qualifier_chirho.split('.').all(|seg_chirho| {
                !seg_chirho.is_empty()
                    && seg_chirho
                        .chars()
                        .all(|c_chirho| c_chirho.is_ascii_alphanumeric() || c_chirho == '_')
            });
            if qualifier_is_module_path_chirho
                && !qualifier_chirho.is_empty()
                && !local_chirho.is_empty()
            {
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

    /// Try to parse a kind-annotated type variable from a parenthesized group:
    /// `( VarId :: Kind )` where Kind is `*`, `* -> *`, `(* -> *) -> *`, etc.
    ///
    /// `idx_chirho` is the index of the `(` token in `children_chirho`.
    /// Returns `Some((TyVarChirho, tokens_consumed))` on success, or `None` if
    /// the pattern doesn't match.
    fn try_parse_kind_annotated_tyvar_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        idx_chirho: usize,
    ) -> Option<(TyVarChirho, usize)> {
        // Need at least ( VarId :: * ) = 5 tokens
        if idx_chirho + 4 >= children_chirho.len() {
            return None;
        }

        // children_chirho[idx_chirho] is the `(` we already checked
        // children_chirho[idx_chirho+1] must be VarId
        let var_child_chirho = &children_chirho[idx_chirho + 1];
        let var_tok_chirho = match var_child_chirho.element_chirho {
            GreenElementChirho::TokenChirho(t_chirho)
                if t_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
            {
                t_chirho
            }
            _ => return None,
        };

        // children_chirho[idx_chirho+2] must be ::
        let dc_child_chirho = &children_chirho[idx_chirho + 2];
        match dc_child_chirho.element_chirho {
            GreenElementChirho::TokenChirho(t_chirho)
                if t_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho => {}
            _ => return None,
        }

        // Parse the kind expression starting at idx_chirho+3, stop at `)`
        let kind_start_chirho = idx_chirho + 3;
        let (kind_chirho, end_idx_chirho) =
            self.parse_kind_tokens_chirho(children_chirho, kind_start_chirho)?;

        // children_chirho[end_idx_chirho] must be `)`
        if end_idx_chirho >= children_chirho.len() {
            return None;
        }
        match children_chirho[end_idx_chirho].element_chirho {
            GreenElementChirho::TokenChirho(t_chirho)
                if t_chirho.kind_chirho() == TokenKindChirho::RightParenChirho => {}
            _ => return None,
        }

        let s_chirho = self.span_chirho(var_child_chirho.start_chirho, var_child_chirho.end_chirho);
        let name_chirho = self.name_from_token_chirho(var_tok_chirho, s_chirho);
        let tv_chirho = TyVarChirho::annotated_chirho(name_chirho, kind_chirho);

        // Total tokens consumed: from `(` through `)` inclusive
        let consumed_chirho = end_idx_chirho - idx_chirho + 1;
        Some((tv_chirho, consumed_chirho))
    }

    /// Try to lower a `ParenTypeChirho` node as a kind-annotated forall binder:
    /// `(a :: k)` where the node contains VarType + `::` + kind-type children.
    /// Returns `Some(TyVarChirho)` with kind annotation if the pattern matches.
    fn try_lower_kind_annotated_forall_binder_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<TyVarChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        // Look for pattern: ( VarId/VarType :: kind-type )
        let mut var_name_chirho: Option<NameChirho> = None;
        let mut saw_double_colon_chirho = false;
        let mut kind_type_node_chirho: Option<(&GreenNodeChirho, usize)> = None;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let k_chirho = tok_chirho.kind_chirho();
                    if k_chirho == TokenKindChirho::VarIdChirho && var_name_chirho.is_none() {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        var_name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if k_chirho == TokenKindChirho::DoubleColonChirho {
                        saw_double_colon_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    let nk_chirho = n_chirho.kind_chirho();
                    if !saw_double_colon_chirho && var_name_chirho.is_none() {
                        // VarType node before :: — extract variable name
                        if nk_chirho == SyntaxKindChirho::VarTypeChirho {
                            var_name_chirho = Some(self.extract_name_from_node_chirho(
                                n_chirho,
                                child_chirho.start_chirho,
                            ));
                        }
                    } else if saw_double_colon_chirho
                        && kind_type_node_chirho.is_none()
                        && is_type_kind_chirho(nk_chirho)
                    {
                        kind_type_node_chirho = Some((n_chirho, child_chirho.start_chirho));
                    }
                }
            }
        }

        let var_name_chirho = var_name_chirho?;
        if !saw_double_colon_chirho {
            // No kind annotation — plain parenthesized binder
            return Some(TyVarChirho::plain_chirho(var_name_chirho));
        }

        let kind_chirho = if let Some((kind_node_chirho, kind_base_chirho)) = kind_type_node_chirho
        {
            let ty_chirho = self.lower_type_chirho(kind_node_chirho, kind_base_chirho);
            Self::try_type_to_ast_kind_chirho(&ty_chirho)
        } else {
            None
        };

        match kind_chirho {
            Some(k_chirho) => Some(TyVarChirho::annotated_chirho(var_name_chirho, k_chirho)),
            None => Some(TyVarChirho::plain_chirho(var_name_chirho)),
        }
    }

    /// Try to convert a lowered `TypeChirho` into an `AstKindChirho` for kind
    /// annotations. Returns `None` for type-level constructs that cannot be
    /// represented as kinds (e.g. type applications like `Either x y`), which
    /// causes the binder to remain unannotated so the kind checker can infer it.
    fn try_type_to_ast_kind_chirho(ty_chirho: &TypeChirho) -> Option<AstKindChirho> {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                Some(match text_chirho {
                    "Type" | "*" => AstKindChirho::StarChirho,
                    "Constraint" => AstKindChirho::ConstraintChirho,
                    _ => AstKindChirho::VarChirho(text_chirho.to_string()),
                })
            }
            TypeChirho::VarChirho(name_chirho) => Some(AstKindChirho::VarChirho(
                name_chirho.text_chirho().to_string(),
            )),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                let a_chirho = Self::try_type_to_ast_kind_chirho(arg_chirho)?;
                let b_chirho = Self::try_type_to_ast_kind_chirho(result_chirho)?;
                Some(AstKindChirho::ArrowChirho(
                    Box::new(a_chirho),
                    Box::new(b_chirho),
                ))
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::try_type_to_ast_kind_chirho(inner_chirho)
            }
            // Type applications, lists, tuples etc. can't be represented as
            // AstKindChirho — return None so the kind checker infers the kind.
            _ => None,
        }
    }

    /// Parse a kind expression from flat tokens: `*`, `* -> *`, `(* -> *) -> *`.
    /// Returns the parsed kind and the index of the next unconsumed token.
    fn parse_kind_tokens_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        start_chirho: usize,
    ) -> Option<(AstKindChirho, usize)> {
        let (lhs_chirho, mut pos_chirho) =
            self.parse_kind_atom_chirho(children_chirho, start_chirho)?;

        // Check for `->` to form arrow kinds
        if pos_chirho < children_chirho.len() {
            if let GreenElementChirho::TokenChirho(t_chirho) =
                children_chirho[pos_chirho].element_chirho
            {
                if t_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho {
                    pos_chirho += 1; // consume `->`
                    let (rhs_chirho, end_chirho) =
                        self.parse_kind_tokens_chirho(children_chirho, pos_chirho)?;
                    return Some((
                        AstKindChirho::ArrowChirho(Box::new(lhs_chirho), Box::new(rhs_chirho)),
                        end_chirho,
                    ));
                }
            }
        }
        Some((lhs_chirho, pos_chirho))
    }

    /// Parse a single kind atom: `*`, `Type`, or `(kind)`.
    fn parse_kind_atom_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        pos_chirho: usize,
    ) -> Option<(AstKindChirho, usize)> {
        if pos_chirho >= children_chirho.len() {
            return None;
        }
        match children_chirho[pos_chirho].element_chirho {
            GreenElementChirho::TokenChirho(t_chirho) => {
                // `*` is a VarSym token with text "*"
                if t_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                    && t_chirho.text_chirho() == "*"
                {
                    Some((AstKindChirho::StarChirho, pos_chirho + 1))
                }
                // `Type` is a ConId token with text "Type"
                else if t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                    && t_chirho.text_chirho() == "Type"
                {
                    Some((AstKindChirho::StarChirho, pos_chirho + 1))
                }
                // `Constraint` is a ConId token with text "Constraint" (ConstraintKinds)
                else if t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                    && t_chirho.text_chirho() == "Constraint"
                {
                    Some((AstKindChirho::ConstraintChirho, pos_chirho + 1))
                }
                // Kind variable (PolyKinds): lowercase identifier like `k` in `(a :: k)`
                else if t_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                    Some((
                        AstKindChirho::VarChirho(t_chirho.text_chirho().to_string()),
                        pos_chirho + 1,
                    ))
                }
                // DataKinds: uppercase identifier like `Bool` in `(b :: Bool)`.
                // Any uppercase name in kind position that isn't `Type` or `Constraint`
                // is a promoted data type used as a kind.
                else if t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho {
                    Some((
                        AstKindChirho::VarChirho(t_chirho.text_chirho().to_string()),
                        pos_chirho + 1,
                    ))
                }
                // Parenthesized kind: `(kind)`
                else if t_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                    let (inner_chirho, after_chirho) =
                        self.parse_kind_tokens_chirho(children_chirho, pos_chirho + 1)?;
                    // Expect `)`
                    if after_chirho < children_chirho.len() {
                        if let GreenElementChirho::TokenChirho(close_chirho) =
                            children_chirho[after_chirho].element_chirho
                        {
                            if close_chirho.kind_chirho() == TokenKindChirho::RightParenChirho {
                                return Some((inner_chirho, after_chirho + 1));
                            }
                        }
                    }
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
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
            let pat_chirho = parts_chirho
                .iter()
                .take(pos_chirho)
                .find_map(|p_chirho| match p_chirho {
                    QualPartChirho::PatChirho(pat_chirho) => Some(pat_chirho.clone()),
                    QualPartChirho::ExprChirho(e_chirho) => Some(Self::expr_to_pat_chirho(e_chirho)),
                    QualPartChirho::ArrowChirho => None,
                })
                .unwrap_or(PatChirho::WildcardChirho(span_chirho));

            let src_expr_chirho = parts_chirho
                .iter()
                .skip(pos_chirho + 1)
                .filter_map(|p_chirho| match p_chirho {
                    QualPartChirho::ExprChirho(e_chirho) => Some(e_chirho.clone()),
                    _ => None,
                })
                .next();

            let expr_chirho = src_expr_chirho.unwrap_or_else(|| self.placeholder_expr_chirho());

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
                } else if text_chirho
                    .chars()
                    .next()
                    .map_or(false, |c_chirho| c_chirho.is_uppercase())
                {
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
            ExprChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => PatChirho::ListChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|e_chirho| Self::expr_to_pat_chirho(e_chirho))
                    .collect(),
                span_chirho: *span_chirho,
            },
            ExprChirho::ParenChirho {
                inner_chirho,
                span_chirho,
            } => PatChirho::ParenChirho {
                inner_chirho: Box::new(Self::expr_to_pat_chirho(inner_chirho)),
                span_chirho: *span_chirho,
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
                    app_args_chirho.push(Self::expr_to_pat_chirho(arg_chirho));
                    head_expr_chirho = fun_chirho;
                }
                app_args_chirho.reverse();
                match head_expr_chirho {
                    ExprChirho::ConChirho(con_chirho) => PatChirho::ConChirho {
                        con_chirho: con_chirho.clone(),
                        args_chirho: app_args_chirho,
                        span_chirho: expr_chirho.span_chirho(),
                    },
                    ExprChirho::VarChirho(name_chirho)
                        if name_chirho
                            .text_chirho()
                            .chars()
                            .next()
                            .map_or(false, |c_chirho| c_chirho.is_uppercase()) =>
                    {
                        PatChirho::ConChirho {
                            con_chirho: name_chirho.clone(),
                            args_chirho: app_args_chirho,
                            span_chirho: expr_chirho.span_chirho(),
                        }
                    }
                    _ => PatChirho::WildcardChirho(expr_chirho.span_chirho()),
                }
            }
            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => PatChirho::InfixConChirho {
                left_chirho: Box::new(Self::expr_to_pat_chirho(left_chirho)),
                op_chirho: op_chirho.clone(),
                right_chirho: Box::new(Self::expr_to_pat_chirho(right_chirho)),
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
    /// A real pattern parsed on the left of `<-`.
    PatChirho(PatChirho),
    /// The `<-` arrow token separating pattern from source.
    ArrowChirho,
}

#[derive(Debug, Clone)]
struct LoweredGuardArmChirho {
    quals_chirho: Vec<StmtChirho>,
    body_chirho: ExprChirho,
    span_chirho: SpanChirho,
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
        GreenElementChirho::TokenChirho(tok_chirho) => {
            is_trivia_token_kind_chirho(tok_chirho.kind_chirho())
        }
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
            | SyntaxKindChirho::PromotedConTypeChirho
            | SyntaxKindChirho::PromotedListTypeChirho
            | SyntaxKindChirho::InfixTypeChirho
            | SyntaxKindChirho::WildcardTypeChirho
            | SyntaxKindChirho::LitTypeChirho
    )
}

fn is_expr_kind_chirho(kind_chirho: SyntaxKindChirho) -> bool {
    matches!(
        kind_chirho,
        SyntaxKindChirho::AppExprChirho
            | SyntaxKindChirho::TypeAppExprChirho
            | SyntaxKindChirho::InfixExprChirho
            | SyntaxKindChirho::LambdaExprChirho
            | SyntaxKindChirho::LambdaCaseExprChirho
            | SyntaxKindChirho::LetExprChirho
            | SyntaxKindChirho::IfExprChirho
            | SyntaxKindChirho::MultiWayIfExprChirho
            | SyntaxKindChirho::CaseExprChirho
            | SyntaxKindChirho::DoExprChirho
            | SyntaxKindChirho::ParenExprChirho
            | SyntaxKindChirho::LeftSectionExprChirho
            | SyntaxKindChirho::RightSectionExprChirho
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
            | SyntaxKindChirho::SpliceExprChirho
            | SyntaxKindChirho::TypedSpliceExprChirho
            | SyntaxKindChirho::QuoteExprChirho
            | SyntaxKindChirho::QuoteDeclChirho
            | SyntaxKindChirho::QuoteTypeChirho
            | SyntaxKindChirho::QuotePatChirho
            | SyntaxKindChirho::TypedQuoteExprChirho
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
            | SyntaxKindChirho::ViewPatChirho
            | SyntaxKindChirho::SigPatChirho
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
    // Gracefully handle mismatched expr/op counts from malformed input.
    if exprs_chirho.is_empty() {
        return ExprChirho::LitChirho(LitChirho::IntChirho(0, span_chirho));
    }
    if ops_chirho.is_empty() || exprs_chirho.len() <= 1 {
        return exprs_chirho.into_iter().next().unwrap();
    }
    if ops_chirho.len() == 1 {
        let mut it_chirho = exprs_chirho.into_iter();
        let left_chirho = it_chirho.next().unwrap();
        let right_chirho = it_chirho.next().unwrap();
        let split_op_chirho = ops_chirho.into_iter().next().unwrap();
        return if split_op_chirho.text_chirho() == "$" {
            ExprChirho::AppChirho {
                fun_chirho: Box::new(left_chirho),
                arg_chirho: Box::new(right_chirho),
                span_chirho,
            }
        } else {
            ExprChirho::InfixChirho {
                left_chirho: Box::new(left_chirho),
                op_chirho: split_op_chirho,
                right_chirho: Box::new(right_chirho),
                span_chirho,
            }
        };
    }

    // Find the operator with the lowest precedence to split at.
    // For left-associative operators at the same precedence, pick the
    // rightmost occurrence so the left side groups first.
    // For right-associative, pick the leftmost occurrence.
    let mut split_idx_chirho = 0usize;
    let (mut split_prec_chirho, _) = operator_fixity_chirho(ops_chirho[0].text_chirho());

    for (i_chirho, op_chirho) in ops_chirho.iter().enumerate().skip(1) {
        let (prec_chirho, _assoc_chirho) = operator_fixity_chirho(op_chirho.text_chirho());
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

    // Guard: if exprs is shorter than expected, return left-to-right folded
    if split_idx_chirho + 1 >= exprs_chirho.len() {
        let mut result_chirho = exprs_chirho.into_iter().next().unwrap();
        for op_chirho in ops_chirho {
            result_chirho = ExprChirho::InfixChirho {
                left_chirho: Box::new(result_chirho),
                op_chirho,
                right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(0, span_chirho))),
                span_chirho,
            };
        }
        return result_chirho;
    }

    // Split exprs and ops around the split point
    let left_exprs_chirho: Vec<ExprChirho> = exprs_chirho[..=split_idx_chirho].to_vec();
    let right_exprs_chirho: Vec<ExprChirho> = exprs_chirho[split_idx_chirho + 1..].to_vec();
    let left_ops_chirho: Vec<NameChirho> = ops_chirho[..split_idx_chirho].to_vec();
    let right_ops_chirho: Vec<NameChirho> = ops_chirho[split_idx_chirho + 1..].to_vec();

    let left_chirho =
        resolve_infix_precedence_chirho(left_exprs_chirho, left_ops_chirho, span_chirho);
    let right_chirho =
        resolve_infix_precedence_chirho(right_exprs_chirho, right_ops_chirho, span_chirho);

    if split_op_chirho.text_chirho() == "$" {
        ExprChirho::AppChirho {
            fun_chirho: Box::new(left_chirho),
            arg_chirho: Box::new(right_chirho),
            span_chirho,
        }
    } else {
        ExprChirho::InfixChirho {
            left_chirho: Box::new(left_chirho),
            op_chirho: split_op_chirho,
            right_chirho: Box::new(right_chirho),
            span_chirho,
        }
    }
}

// ---------------------------------------------------------------------------
// Escape sequence processing
// ---------------------------------------------------------------------------

/// Collect decimal digits from an iterator and convert to a char.
fn collect_decimal_escape_chirho(
    first_digit_chirho: char,
    chars_chirho: &mut std::str::Chars<'_>,
) -> char {
    let mut num_chirho = String::new();
    num_chirho.push(first_digit_chirho);
    // Peek at subsequent digits — Haskell allows up to 7 decimal digits
    // but we consume greedily as long as digits continue
    while let Some(&next_chirho) = chars_chirho.as_str().as_bytes().first() {
        if next_chirho.is_ascii_digit() {
            num_chirho.push(next_chirho as char);
            chars_chirho.next();
        } else {
            break;
        }
    }
    num_chirho
        .parse::<u32>()
        .ok()
        .and_then(char::from_u32)
        .unwrap_or('\u{FFFD}')
}

/// Collect octal digits from an iterator and convert to a char.
fn collect_octal_escape_chirho(chars_chirho: &mut std::str::Chars<'_>) -> char {
    let mut num_chirho = String::new();
    while let Some(&next_chirho) = chars_chirho.as_str().as_bytes().first() {
        if (b'0'..=b'7').contains(&next_chirho) {
            num_chirho.push(next_chirho as char);
            chars_chirho.next();
        } else {
            break;
        }
    }
    u32::from_str_radix(&num_chirho, 8)
        .ok()
        .and_then(char::from_u32)
        .unwrap_or('\u{FFFD}')
}

/// Collect hexadecimal digits from an iterator and convert to a char.
fn collect_hex_escape_chirho(chars_chirho: &mut std::str::Chars<'_>) -> char {
    let mut num_chirho = String::new();
    while let Some(&next_chirho) = chars_chirho.as_str().as_bytes().first() {
        if (next_chirho as char).is_ascii_hexdigit() {
            num_chirho.push(next_chirho as char);
            chars_chirho.next();
        } else {
            break;
        }
    }
    u32::from_str_radix(&num_chirho, 16)
        .ok()
        .and_then(char::from_u32)
        .unwrap_or('\u{FFFD}')
}

fn named_ascii_escape_chirho(name_chirho: &str) -> Option<char> {
    match name_chirho {
        "NUL" => Some('\0'),
        "SOH" => Some('\u{0001}'),
        "STX" => Some('\u{0002}'),
        "ETX" => Some('\u{0003}'),
        "EOT" => Some('\u{0004}'),
        "ENQ" => Some('\u{0005}'),
        "ACK" => Some('\u{0006}'),
        "BEL" => Some('\u{0007}'),
        "BS" => Some('\u{0008}'),
        "HT" => Some('\t'),
        "LF" => Some('\n'),
        "VT" => Some('\u{000B}'),
        "FF" => Some('\u{000C}'),
        "CR" => Some('\r'),
        "SO" => Some('\u{000E}'),
        "SI" => Some('\u{000F}'),
        "DLE" => Some('\u{0010}'),
        "DC1" => Some('\u{0011}'),
        "DC2" => Some('\u{0012}'),
        "DC3" => Some('\u{0013}'),
        "DC4" => Some('\u{0014}'),
        "NAK" => Some('\u{0015}'),
        "SYN" => Some('\u{0016}'),
        "ETB" => Some('\u{0017}'),
        "CAN" => Some('\u{0018}'),
        "EM" => Some('\u{0019}'),
        "SUB" => Some('\u{001A}'),
        "ESC" => Some('\u{001B}'),
        "FS" => Some('\u{001C}'),
        "GS" => Some('\u{001D}'),
        "RS" => Some('\u{001E}'),
        "US" => Some('\u{001F}'),
        "SP" => Some(' '),
        "DEL" => Some('\u{007F}'),
        _ => None,
    }
}

/// Process Haskell escape sequences in a string literal.
/// Handles \n, \t, \r, \\, \", \', \0, \a, \b, \f, \v,
/// decimal (\65), octal (\o101), hex (\x41), and string gaps.
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
                Some('o') => result_chirho.push(collect_octal_escape_chirho(&mut chars_chirho)),
                Some('x') => result_chirho.push(collect_hex_escape_chirho(&mut chars_chirho)),
                Some(d_chirho) if d_chirho.is_ascii_digit() && d_chirho != '0' => {
                    result_chirho.push(collect_decimal_escape_chirho(d_chirho, &mut chars_chirho));
                }
                Some(c_chirho) if c_chirho.is_ascii_uppercase() => {
                    let mut name_chirho = String::from(c_chirho);
                    while let Some(next_chirho) = chars_chirho.clone().next() {
                        if next_chirho.is_ascii_uppercase() || next_chirho.is_ascii_digit() {
                            name_chirho.push(chars_chirho.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if let Some(ch_chirho) = named_ascii_escape_chirho(&name_chirho) {
                        result_chirho.push(ch_chirho);
                    } else {
                        result_chirho.push('\\');
                        result_chirho.push_str(&name_chirho);
                    }
                }
                Some(ws_chirho) if ws_chirho.is_ascii_whitespace() => {
                    // String gap: \<whitespace>\  — skip all whitespace until next backslash
                    for gap_c_chirho in chars_chirho.by_ref() {
                        if gap_c_chirho == '\\' {
                            break;
                        }
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
/// Handles \n, \t, \r, \\, \', \", \0, \a, \b, \f, \v,
/// decimal (\65), octal (\o101), hex (\x41).
fn unescape_char_chirho(s_chirho: &str) -> char {
    if s_chirho.starts_with('\\') {
        let rest_chirho = &s_chirho[1..];
        match rest_chirho.chars().next() {
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
            Some('o') => {
                let digits_chirho = &rest_chirho[1..];
                u32::from_str_radix(digits_chirho, 8)
                    .ok()
                    .and_then(char::from_u32)
                    .unwrap_or('\u{FFFD}')
            }
            Some('x') => {
                let digits_chirho = &rest_chirho[1..];
                u32::from_str_radix(digits_chirho, 16)
                    .ok()
                    .and_then(char::from_u32)
                    .unwrap_or('\u{FFFD}')
            }
            Some(d_chirho) if d_chirho.is_ascii_digit() && d_chirho != '0' => rest_chirho
                .parse::<u32>()
                .ok()
                .and_then(char::from_u32)
                .unwrap_or('\u{FFFD}'),
            Some(c_chirho) if c_chirho.is_ascii_uppercase() => {
                named_ascii_escape_chirho(rest_chirho).unwrap_or('\u{FFFD}')
            }
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
    fn lower_parsec_prim_state_record_decl_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module Text.Parsec.Prim where\n\
unknownErrorChirho :: StateChirho sChirho uChirho -> ParseErrorChirho\n\
unknownErrorChirho stateChirho = newErrorUnknownChirho (statePosChirho stateChirho)\n\
\n\
data StateChirho sChirho uChirho = StateChirho {\n\
      stateInputChirho :: sChirho,\n\
      statePosChirho   :: !SourcePosChirho,\n\
      stateUserChirho  :: !uChirho\n\
    }\n",
        );
        assert_eq!(module_chirho.decls_chirho.len(), 3);
        match &module_chirho.decls_chirho[2] {
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => {
                assert_eq!(constructors_chirho.len(), 1);
                match &constructors_chirho[0] {
                    ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                        assert_eq!(fields_chirho.len(), 3);
                        let lowered_field_names_chirho: Vec<Vec<String>> = fields_chirho
                            .iter()
                            .map(|fd_chirho| {
                                fd_chirho
                                    .names_chirho
                                    .iter()
                                    .map(|name_chirho| name_chirho.text_chirho().to_string())
                                    .collect()
                            })
                            .collect();
                        assert_eq!(
                            lowered_field_names_chirho,
                            vec![
                                vec!["stateInputChirho".to_string()],
                                vec!["statePosChirho".to_string()],
                                vec!["stateUserChirho".to_string()]
                            ]
                        );
                    }
                    other_chirho => panic!("expected RecordChirho, got {:?}", other_chirho),
                }
            }
            other_chirho => panic!("expected DataDeclChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_parsec_prim_trimmed_module_shape_chirho() {
        let module_chirho = parse_and_lower_chirho(
            r#"{-# LANGUAGE RankNTypes #-}
module Text.Parsec.Prim
    ( unknownErrorChirho
    , runParsecTChirho
    , mkPTChirho
    , StateChirho(..)
    ) where

unknownErrorChirho :: StateChirho sChirho uChirho -> ParseErrorChirho
unknownErrorChirho stateChirho = newErrorUnknownChirho (statePosChirho stateChirho)

{-# INLINABLE runParsecTChirho #-}
runParsecTChirho pChirho sChirho = unParserChirho pChirho sChirho cokChirho cerrChirho eokChirho eerrChirho
    where cokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          cerrChirho errChirho = return (ErrorChirho errChirho)
          eokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          eerrChirho errChirho = return (ErrorChirho errChirho)

{-# INLINABLE mkPTChirho #-}
mkPTChirho kChirho = ParsecTChirho $ \sChirho cokChirho cerrChirho eokChirho eerrChirho -> do
           consChirho <- kChirho sChirho
           case consChirho of
             ConsumedChirho mrepChirho -> do
                       repChirho <- mrepChirho
                       case repChirho of
                         OkChirho xChirho s'Chirho errChirho -> cokChirho xChirho s'Chirho errChirho
                         ErrorChirho errChirho -> cerrChirho errChirho
             EmptyChirho mrepChirho -> do
                       repChirho <- mrepChirho
                       case repChirho of
                         OkChirho xChirho s'Chirho errChirho -> eokChirho xChirho s'Chirho errChirho
                         ErrorChirho errChirho -> eerrChirho errChirho

data StateChirho sChirho uChirho = StateChirho {
      stateInputChirho :: sChirho,
      statePosChirho   :: !SourcePosChirho,
      stateUserChirho  :: !uChirho
    }
"#,
        );
        assert_eq!(module_chirho.name_chirho.full_name_chirho(), "Text.Parsec.Prim");
        assert!(
            module_chirho.decls_chirho.iter().any(|decl_chirho| matches!(
                decl_chirho,
                DeclChirho::DataDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "StateChirho"
            )),
            "expected StateChirho data declaration, got {:?}",
            module_chirho
                .decls_chirho
                .iter()
                .map(|decl_chirho| match decl_chirho {
                    DeclChirho::TypeSigChirho { name_chirho, .. } => format!("typesig:{}", name_chirho.text_chirho()),
                    DeclChirho::FunBindChirho { name_chirho, .. } => format!("fun:{}", name_chirho.text_chirho()),
                    DeclChirho::DataDeclChirho { name_chirho, .. } => format!("data:{}", name_chirho.text_chirho()),
                    DeclChirho::NewtypeDeclChirho { name_chirho, .. } => format!("newtype:{}", name_chirho.text_chirho()),
                    DeclChirho::PatBindChirho { .. } => "patbind".to_string(),
                    other_chirho => format!("{:?}", other_chirho),
                })
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn lower_where_block_stays_local_in_parsect_shape_chirho() {
        let module_chirho = parse_and_lower_chirho(
            r#"module M where
runParsecTChirho pChirho sChirho = unParserChirho pChirho sChirho cokChirho cerrChirho eokChirho eerrChirho
    where cokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          cerrChirho errChirho = return (ErrorChirho errChirho)
          eokChirho aChirho s'Chirho errChirho = return (OkChirho aChirho s'Chirho errChirho)
          eerrChirho errChirho = return (ErrorChirho errChirho)

data TailChirho = TailChirho
"#,
        );
        assert_eq!(module_chirho.decls_chirho.len(), 2, "{:?}", module_chirho.decls_chirho);
        match &module_chirho.decls_chirho[0] {
            DeclChirho::FunBindChirho { matches_chirho, .. } => {
                assert_eq!(matches_chirho.len(), 1);
                let local_bind_names_chirho: Vec<String> = matches_chirho[0]
                    .where_binds_chirho
                    .iter()
                    .filter_map(|bind_chirho| match bind_chirho {
                        LocalBindChirho::FunBindChirho { name_chirho, .. } => {
                            Some(name_chirho.text_chirho().to_string())
                        }
                        _ => None,
                    })
                    .collect();
                assert_eq!(
                    local_bind_names_chirho,
                    vec![
                        "cokChirho".to_string(),
                        "cerrChirho".to_string(),
                        "eokChirho".to_string(),
                        "eerrChirho".to_string()
                    ]
                );
            }
            other_chirho => panic!("expected FunBindChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_infix_fun_bind_with_backticked_name_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\ncatchEChirho :: aChirho -> bChirho -> cChirho\nmChirho `catchEChirho` hChirho = hChirho\n",
        );
        let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "catchEChirho")
        });
        assert!(decl_chirho.is_some(), "should lower infix funbind name");
        if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
            assert_eq!(
                matches_chirho[0].pats_chirho.len(),
                2,
                "infix funbind should lower both lhs operands as patterns"
            );
        }
    }

    #[test]
    fn lower_backticked_left_section_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\nrwhnfChirho = (`seq` ())\n");
        let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "rwhnfChirho"
            )
        });
        assert!(decl_chirho.is_some(), "should lower backticked left section");
        if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
            match &matches_chirho[0].rhs_chirho {
                RhsChirho::UnguardedChirho(ExprChirho::LeftSectionChirho {
                    op_chirho,
                    arg_chirho,
                    ..
                }) => {
                    assert_eq!(op_chirho.text_chirho(), "seq");
                    assert!(
                        matches!(
                            arg_chirho.as_ref(),
                            ExprChirho::TupleChirho { elements_chirho, .. }
                                if elements_chirho.is_empty()
                        ),
                        "expected unit tuple argument, got {:?}",
                        arg_chirho
                    );
                }
                other_chirho => panic!("expected backticked left section, got {:?}", other_chirho),
            }
        }
    }

    #[test]
    fn lower_infix_fun_bind_with_signature_and_do_rhs_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nimport Control.Monad\nnewtype ExceptTChirho eChirho mChirho aChirho = ExceptTChirho { runExceptTChirho :: mChirho (Either eChirho aChirho) }\ncatchEChirho :: Monad mChirho => ExceptTChirho eChirho mChirho aChirho -> (eChirho -> ExceptTChirho ePrimeChirho mChirho aChirho) -> ExceptTChirho ePrimeChirho mChirho aChirho\nmChirho `catchEChirho` hChirho = ExceptTChirho $ do\n  aChirho <- runExceptTChirho mChirho\n  case aChirho of\n    Left lChirho -> runExceptTChirho (hChirho lChirho)\n    Right rChirho -> return (Right rChirho)\n",
        );
        let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "catchEChirho")
        });
        assert!(decl_chirho.is_some(), "should lower catchE infix funbind");
        if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
            assert_eq!(
                matches_chirho[0].pats_chirho.len(),
                2,
                "signed infix catchE funbind should keep both lhs operands as patterns"
            );
            assert!(
                matches!(
                    &matches_chirho[0].rhs_chirho,
                    RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        arg_chirho,
                        ..
                    }) if matches!(&**arg_chirho, ExprChirho::DoChirho { .. })
                ),
                "signed infix catchE funbind should preserve the constructor/do RHS"
            );
        }
    }

    #[test]
    fn lower_infix_fun_bind_with_parenthesized_constructor_pats_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nnewtype ChanChirho pChirho aChirho = ChanChirho aChirho\naddChirho :: Num aChirho => ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho\n(ChanChirho aChirho) `addChirho` (ChanChirho bChirho) = ChanChirho (aChirho + bChirho)\n",
        );
        let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "addChirho")
        });
        assert!(
            decl_chirho.is_some(),
            "should lower constructor-pattern infix funbind name"
        );
        if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
            assert_eq!(
                matches_chirho[0].pats_chirho.len(),
                2,
                "constructor-pattern infix funbind should keep both lhs operands as patterns"
            );
        }
    }

    #[test]
    fn lower_infix_expr_with_lambda_rhs_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nonEChirho action1Chirho action2Chirho = action1Chirho `catchEChirho` \\eChirho -> action2Chirho >> throwEChirho eChirho\n",
        );
        let decl_chirho = module_chirho.decls_chirho.iter().find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "onEChirho")
        });
        assert!(decl_chirho.is_some(), "should lower onE infix expression");
        if let Some(DeclChirho::FunBindChirho { matches_chirho, .. }) = decl_chirho {
            assert!(
                matches!(
                    &matches_chirho[0].rhs_chirho,
                    RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                        right_chirho,
                        ..
                    }) if matches!(
                        &**right_chirho,
                        ExprChirho::LamChirho {
                            body_chirho,
                            ..
                        } if matches!(
                            &**body_chirho,
                            ExprChirho::InfixChirho {
                                op_chirho,
                                right_chirho,
                                ..
                            } if op_chirho.text_chirho() == ">>"
                                && matches!(
                                    &**right_chirho,
                                    ExprChirho::AppChirho {
                                        fun_chirho,
                                        arg_chirho,
                                        ..
                                    } if matches!(
                                        (&**fun_chirho, &**arg_chirho),
                                        (
                                            ExprChirho::VarChirho(fun_name_chirho),
                                            ExprChirho::VarChirho(arg_name_chirho),
                                        ) if fun_name_chirho.text_chirho() == "throwEChirho"
                                            && arg_name_chirho.text_chirho() == "eChirho"
                                    )
                                )
                        )
                    )
                ),
                "infix catchE expression should preserve the full lambda body in the lowered AST"
            );
        }
    }

    #[test]
    fn lower_type_sig_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\nfoo :: Int -> String\n");
        assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
        }));
    }

    #[test]
    fn lower_qualified_type_sig_chirho() {
        // Type signature with Eq a => context should produce QualChirho
        let module_chirho =
            parse_and_lower_chirho("module M where\nfoo :: Eq a => a -> a -> Bool\n");
        let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
        });
        assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
        if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
            assert!(
                matches!(ty_chirho, TypeChirho::QualChirho { context_chirho, .. } if !context_chirho.is_empty()),
                "type should be QualChirho with non-empty context, got: {:?}",
                ty_chirho
            );
        }
    }

    #[test]
    fn lower_qualified_type_sig_tuple_context_chirho() {
        // (Eq a, Show a) => should produce QualChirho with 2 constraints
        let module_chirho =
            parse_and_lower_chirho("module M where\nfoo :: (Eq a, Show a) => a -> String\n");
        let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
        });
        assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
        if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
            match ty_chirho {
                TypeChirho::QualChirho { context_chirho, .. } => {
                    assert_eq!(
                        context_chirho.len(),
                        2,
                        "should have 2 constraints (Eq and Show), got: {:?}",
                        context_chirho
                    );
                }
                other_chirho => panic!("expected QualChirho, got: {:?}", other_chirho),
            }
        }
    }

    #[test]
    fn lower_equality_constraint_type_sig_chirho() {
        // a ~ Int => should produce QualChirho with equality constraint
        let module_chirho = parse_and_lower_chirho("module M where\nfoo :: a ~ Int => a -> Int\n");
        let ty_sig_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "foo")
        });
        assert!(ty_sig_chirho.is_some(), "should have a TypeSig for foo");
        if let Some(DeclChirho::TypeSigChirho { ty_chirho, .. }) = ty_sig_chirho {
            match ty_chirho {
                TypeChirho::QualChirho { context_chirho, .. } => {
                    assert_eq!(context_chirho.len(), 1, "should have 1 constraint");
                    // The constraint should be the ~ operator applied to two types
                    let c_chirho = &context_chirho[0];
                    assert!(
                        matches!(
                            c_chirho,
                            ConstraintChirho::ClassChirho {
                                class_chirho,
                                args_chirho,
                                ..
                            } if class_chirho.text_chirho() == "~" && args_chirho.len() == 2
                        ),
                        "constraint should be simple (~) with 2 args, got {:?}",
                        c_chirho
                    );
                }
                other_chirho => panic!("expected QualChirho, got: {:?}", other_chirho),
            }
        }
    }

    #[test]
    fn lower_where_local_sig_preserves_qualified_tycon_spine_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nimport qualified Control.Monad.Trans.State.Lazy as LazyS\nf a = g a where\n  q :: (m (a, s) -> m (a, s)) -> LazyS.StateT s m a -> LazyS.StateT s m a\n  q u (LazyS.StateT b) = LazyS.StateT (u . b)\n  g x = x\n",
        );
        let fun_decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|decl_chirho| {
                matches!(
                    decl_chirho,
                    DeclChirho::FunBindChirho { name_chirho, .. }
                        if name_chirho.text_chirho() == "f"
                )
            })
            .expect("expected top-level f binding");
        let where_binds_chirho = match fun_decl_chirho {
            DeclChirho::FunBindChirho { matches_chirho, .. } => &matches_chirho[0].where_binds_chirho,
            other_chirho => panic!("expected function binding, got {:?}", other_chirho),
        };
        let q_sig_chirho = where_binds_chirho
            .iter()
            .find_map(|bind_chirho| match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                    name_chirho,
                    ty_chirho,
                    ..
                } if name_chirho.text_chirho() == "q" => Some(ty_chirho),
                _ => None,
            })
            .expect("expected local type signature for q");

        let arrow_arg_chirho = match q_sig_chirho {
            TypeChirho::FunChirho {
                result_chirho,
                ..
            } => match result_chirho.as_ref() {
                TypeChirho::FunChirho {
                    arg_chirho,
                    result_chirho,
                    ..
                } => {
                    assert!(
                        matches!(result_chirho.as_ref(), TypeChirho::AppChirho { .. }),
                        "final result should stay a fully applied qualified tycon, got {:?}",
                        result_chirho
                    );
                    arg_chirho.as_ref()
                }
                other_chirho => panic!("expected second arrow for q signature, got {:?}", other_chirho),
            },
            other_chirho => panic!("expected function type for q signature, got {:?}", other_chirho),
        };

        match arrow_arg_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                assert!(
                    matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "a"),
                    "qualified tycon should keep final type argument `a`, got {:?}",
                    arg_chirho
                );
                match fun_chirho.as_ref() {
                    TypeChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                        ..
                    } => {
                        assert!(
                            matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "m"),
                            "qualified tycon should keep middle type argument `m`, got {:?}",
                            arg_chirho
                        );
                        match fun_chirho.as_ref() {
                            TypeChirho::AppChirho {
                                fun_chirho,
                                arg_chirho,
                                ..
                            } => {
                                assert!(
                                    matches!(arg_chirho.as_ref(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "s"),
                                    "qualified tycon should keep first type argument `s`, got {:?}",
                                    arg_chirho
                                );
                                assert!(
                                    matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.full_name_chirho() == "LazyS.StateT"),
                                    "qualified constructor head should stay LazyS.StateT, got {:?}",
                                    fun_chirho
                                );
                            }
                            other_chirho => panic!(
                                "expected left-nested type application for LazyS.StateT s m a, got {:?}",
                                other_chirho
                            ),
                        }
                    }
                    other_chirho => panic!(
                        "expected second application layer for LazyS.StateT s m a, got {:?}",
                        other_chirho
                    ),
                }
            }
            other_chirho => panic!(
                "expected qualified type constructor application in q signature, got {:?}",
                other_chirho
            ),
        }
    }

    #[test]
    fn lower_grouped_type_sig_decl_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nwithBarChirho, withEmptyChirho :: [String] -> [String]\nwithBarChirho xsChirho = xsChirho\nwithEmptyChirho xsChirho = xsChirho\n",
        );
        let type_sig_names_chirho: Vec<String> = module_chirho
            .decls_chirho
            .iter()
            .filter_map(|decl_chirho| match decl_chirho {
                DeclChirho::TypeSigChirho { name_chirho, .. } => {
                    Some(name_chirho.text_chirho().to_string())
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            type_sig_names_chirho,
            vec!["withBarChirho".to_string(), "withEmptyChirho".to_string()]
        );
    }

    #[test]
    fn lower_nested_as_pattern_vars_remain_visible_to_where_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module Main where\ndata BitQueueB = BQB Int Int\nnewtype BitQueue = BQ BitQueueB\nunconsQ (BQ bq@(BQB _ lo)) = Just (hd, BQ tl)\n  where\n    hd = lo == 0\n    tl = bq\n",
        );
        let fun_decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|decl_chirho| {
                matches!(
                    decl_chirho,
                    DeclChirho::FunBindChirho { name_chirho, .. }
                        if name_chirho.text_chirho() == "unconsQ"
                )
            })
            .expect("expected unconsQ fun bind");

        let DeclChirho::FunBindChirho { matches_chirho, .. } = fun_decl_chirho else {
            panic!("expected function binding");
        };
        assert_eq!(matches_chirho.len(), 1);
        assert_eq!(matches_chirho[0].where_binds_chirho.len(), 2);

        let PatChirho::ParenChirho { inner_chirho, .. } = &matches_chirho[0].pats_chirho[0] else {
            panic!(
                "expected parenthesized outer pattern, got {:?}",
                matches_chirho[0].pats_chirho[0]
            );
        };
        let PatChirho::ConChirho { args_chirho, .. } = inner_chirho.as_ref() else {
            panic!("expected outer constructor pattern, got {:?}", inner_chirho);
        };
        let PatChirho::AsChirho {
            name_chirho,
            pattern_chirho,
            ..
        } = &args_chirho[0]
        else {
            panic!("expected nested as pattern");
        };
        assert_eq!(name_chirho.text_chirho(), "bq");

        let inner_pattern_chirho = match pattern_chirho.as_ref() {
            PatChirho::ParenChirho { inner_chirho, .. } => inner_chirho.as_ref(),
            other_chirho => other_chirho,
        };
        let PatChirho::ConChirho {
            args_chirho: inner_args_chirho,
            ..
        } = inner_pattern_chirho
        else {
            panic!("expected inner constructor pattern, got {:?}", inner_pattern_chirho);
        };
        assert_eq!(inner_args_chirho.len(), 2);
        assert!(matches!(
            &inner_args_chirho[0],
            PatChirho::WildcardChirho(_)
        ));
        assert!(matches!(
            &inner_args_chirho[1],
            PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "lo"
        ));
    }

    #[test]
    fn lower_containers_intset_retains_helper_funbinds_chirho() {
        let source_path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../.haskelujah-packages-chirho/containers-0.8/src/Data/IntSet/Internal.hs");
        let source_chirho =
            std::fs::read_to_string(&source_path_chirho).expect("expected containers IntSet source");
        let module_chirho = parse_and_lower_chirho(&source_chirho);
        let funbind_names_chirho: Vec<String> = module_chirho
            .decls_chirho
            .iter()
            .filter_map(|decl_chirho| match decl_chirho {
                DeclChirho::FunBindChirho { name_chirho, .. } => {
                    Some(name_chirho.text_chirho().to_string())
                }
                _ => None,
            })
            .collect();
        for required_name_chirho in [
            "withBar",
            "withEmpty",
            "bin",
            "tip",
            "prefixOf",
            "lowestBitSet",
            "highestBitSet",
            "takeWhileAntitoneBits",
        ] {
            assert!(
                funbind_names_chirho
                    .iter()
                    .any(|name_chirho| name_chirho == required_name_chirho),
                "expected helper `{}` in lowered IntSet module; sample={:?}",
                required_name_chirho,
                funbind_names_chirho.iter().rev().take(20).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn lower_import_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nimport Data.List\nimport qualified Data.Map as Map\n",
        );
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
        assert!(
            matches!(&spec0_chirho.items_chirho[0], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "sort")
        );
        assert!(
            matches!(&spec0_chirho.items_chirho[1], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "nub")
        );

        // Second import: hiding
        let spec1_chirho = module_chirho.imports_chirho[1].spec_chirho.as_ref();
        assert!(spec1_chirho.is_some(), "expected import spec for Data.Map");
        let spec1_chirho = spec1_chirho.unwrap();
        assert!(spec1_chirho.hiding_chirho);
        assert_eq!(spec1_chirho.items_chirho.len(), 1);
        assert!(
            matches!(&spec1_chirho.items_chirho[0], ImportItemChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == "map")
        );
    }

    #[test]
    fn lower_import_tycon_spec_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\nimport Data.Map (Map(..))\n");
        assert_eq!(module_chirho.imports_chirho.len(), 1);
        let spec_chirho = module_chirho.imports_chirho[0]
            .spec_chirho
            .as_ref()
            .unwrap();
        assert!(!spec_chirho.hiding_chirho);
        assert_eq!(spec_chirho.items_chirho.len(), 1);
        assert!(matches!(
            &spec_chirho.items_chirho[0],
            ImportItemChirho::TyConChirho { name_chirho, members_chirho: ExportMembersChirho::AllChirho }
                if name_chirho.text_chirho() == "Map"
        ));
    }

    #[test]
    fn lower_import_operator_tycon_spec_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module M where\nimport GHC.Generics ((:*:)(..))\n");
        assert_eq!(module_chirho.imports_chirho.len(), 1);
        let spec_chirho = module_chirho.imports_chirho[0]
            .spec_chirho
            .as_ref()
            .unwrap();
        assert!(!spec_chirho.hiding_chirho);
        assert_eq!(spec_chirho.items_chirho.len(), 1);
        assert!(matches!(
            &spec_chirho.items_chirho[0],
            ImportItemChirho::TyConChirho { name_chirho, members_chirho: ExportMembersChirho::AllChirho }
                if name_chirho.text_chirho() == ":*:"
        ));
    }

    #[test]
    fn lower_import_operator_tycon_with_dot_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module M where\nimport GHC.Generics ((:.:)(..))\n");
        assert_eq!(module_chirho.imports_chirho.len(), 1);
        let spec_chirho = module_chirho.imports_chirho[0]
            .spec_chirho
            .as_ref()
            .unwrap();
        assert_eq!(spec_chirho.items_chirho.len(), 1);
        assert!(matches!(
            &spec_chirho.items_chirho[0],
            ImportItemChirho::TyConChirho { name_chirho, members_chirho: ExportMembersChirho::AllChirho }
                if name_chirho.text_chirho() == ":.:"
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
        let module_chirho =
            parse_and_lower_chirho("module M where\nforeign export ccall mainChirho :: IO ()\n");
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
        let module_chirho = parse_and_lower_chirho("module Foo where\ndata Bar = Baz\n");
        let span_chirho = module_chirho.span_chirho;
        assert!(span_chirho.end_chirho().as_usize_chirho() > 0);
    }

    // -----------------------------------------------------------------------
    // Instance declarations
    // -----------------------------------------------------------------------

    #[test]
    fn lower_instance_simple_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module M where\ninstance Eq Bool where\n  eq x y = x\n");
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
                assert!(
                    matches!(
                        &context_chirho[0],
                        ConstraintChirho::ClassChirho {
                            class_chirho,
                            args_chirho,
                            ..
                        } if class_chirho.text_chirho() == "Eq" && args_chirho.len() == 1
                    ),
                    "expected simple Eq constraint, got {:?}",
                    context_chirho[0]
                );
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_instance_preserves_applied_head_type_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\ninstance MonadZip m => MonadZip (ExceptTChirho eChirho m) where\n  mzipWith fChirho xChirho yChirho = xChirho\n",
        );
        let inst_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|decl_chirho| matches!(decl_chirho, DeclChirho::InstanceDeclChirho { .. }))
            .expect("expected an InstanceDeclChirho");

        match inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                context_chirho,
                types_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "MonadZip");
                assert_eq!(context_chirho.len(), 1);
                assert_eq!(types_chirho.len(), 1);
                match &types_chirho[0] {
                    TypeChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
                        TypeChirho::AppChirho {
                            fun_chirho,
                            arg_chirho,
                            ..
                        } => {
                            assert!(matches!(
                                fun_chirho.as_ref(),
                                TypeChirho::AppChirho { .. }
                            ));
                            match arg_chirho.as_ref() {
                                TypeChirho::VarChirho(name_chirho) => {
                                    assert_eq!(name_chirho.text_chirho(), "m");
                                }
                                other_chirho => {
                                    panic!("expected final applied arg to be `m`, got {:?}", other_chirho)
                                }
                            }
                        }
                        other_chirho => panic!(
                            "expected applied type inside parens for instance head, got {:?}",
                            other_chirho
                        ),
                    },
                    other_chirho => panic!(
                        "expected parenthesized applied type for instance head, got {:?}",
                        other_chirho
                    ),
                }
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_single_monadzip_instance_method_does_not_leak_top_level_funbind_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nimport Control.Applicative\nimport Control.Monad.Zip (MonadZip(mzipWith))\nnewtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }\ninstance MonadZip m => MonadZip (ExceptTChirho e m) where\n  mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho\n",
        );
        let leaked_funbinds_chirho: Vec<String> = module_chirho
            .decls_chirho
            .iter()
            .filter_map(|decl_chirho| match decl_chirho {
                DeclChirho::FunBindChirho { name_chirho, .. } => {
                    Some(name_chirho.text_chirho().to_string())
                }
                _ => None,
            })
            .collect();
        assert!(
            leaked_funbinds_chirho.is_empty(),
            "single MonadZip instance source should not leak top-level funbinds: {:?}",
            leaked_funbinds_chirho
        );
    }

    #[test]
    fn lower_default_decl_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\ndefault (Int, Double)\n");
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
        let module_chirho = parse_and_lower_chirho("module M where\ninstance Show Int\n");
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

    // -----------------------------------------------------------------------
    // Export list lowering tests
    // -----------------------------------------------------------------------

    #[test]
    fn lower_export_list_var_chirho() {
        // module M (foo) where — exports a single variable
        let module_chirho = parse_and_lower_chirho(
            "module M (foo) where
foo = 1
",
        );
        let exports_chirho = module_chirho
            .exports_chirho
            .as_ref()
            .expect("expected an export list");
        assert_eq!(exports_chirho.len(), 1, "expected 1 export");
        match &exports_chirho[0] {
            ExportSpecChirho::VarChirho(name_chirho) => {
                assert_eq!(name_chirho.text_chirho(), "foo");
            }
            other_chirho => panic!("expected VarChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_export_list_tycon_all_chirho() {
        // module M (Color(..)) where — exports a type with all members
        let module_chirho = parse_and_lower_chirho(
            "module M (Color(..)) where
data Color = Red | Green | Blue
",
        );
        let exports_chirho = module_chirho
            .exports_chirho
            .as_ref()
            .expect("expected an export list");
        assert_eq!(exports_chirho.len(), 1, "expected 1 export");
        match &exports_chirho[0] {
            ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho,
            } => {
                assert_eq!(name_chirho.text_chirho(), "Color");
                assert_eq!(*members_chirho, ExportMembersChirho::AllChirho);
            }
            other_chirho => panic!("expected TyConChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_export_list_tycon_specific_chirho() {
        // module M (Color(Red, Green)) where — exports a type with specific members
        let module_chirho = parse_and_lower_chirho(
            "module M (Color(Red, Green)) where
data Color = Red | Green | Blue
",
        );
        let exports_chirho = module_chirho
            .exports_chirho
            .as_ref()
            .expect("expected an export list");
        assert_eq!(exports_chirho.len(), 1, "expected 1 export");
        match &exports_chirho[0] {
            ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho: ExportMembersChirho::SomeChirho(names_chirho),
            } => {
                assert_eq!(name_chirho.text_chirho(), "Color");
                assert_eq!(names_chirho.len(), 2);
                assert_eq!(names_chirho[0].text_chirho(), "Red");
                assert_eq!(names_chirho[1].text_chirho(), "Green");
            }
            other_chirho => panic!(
                "expected TyConChirho with SomeChirho, got {:?}",
                other_chirho
            ),
        }
    }

    #[test]
    fn lower_export_list_mixed_chirho() {
        // module M (foo, Bar(..)) where — exports a var and a type
        let module_chirho = parse_and_lower_chirho(
            "module M (foo, Bar(..)) where
foo = 1
data Bar = Bar
",
        );
        let exports_chirho = module_chirho
            .exports_chirho
            .as_ref()
            .expect("expected an export list");
        assert_eq!(exports_chirho.len(), 2, "expected 2 exports");
        match &exports_chirho[0] {
            ExportSpecChirho::VarChirho(name_chirho) => {
                assert_eq!(name_chirho.text_chirho(), "foo");
            }
            other_chirho => panic!("expected VarChirho, got {:?}", other_chirho),
        }
        match &exports_chirho[1] {
            ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho,
            } => {
                assert_eq!(name_chirho.text_chirho(), "Bar");
                assert_eq!(*members_chirho, ExportMembersChirho::AllChirho);
            }
            other_chirho => panic!("expected TyConChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_no_export_list_chirho() {
        // module M where — no export list means None
        let module_chirho = parse_and_lower_chirho(
            "module M where
foo = 1
",
        );
        assert!(
            module_chirho.exports_chirho.is_none(),
            "expected None export list when no parens"
        );
    }

    // ── KindSignatures parsing tests ────────────────────────────────────

    #[test]
    fn lower_kind_sig_data_star_chirho() {
        // data Proxy (a :: *) = MkProxy
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Proxy (a :: *) = MkProxy\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), "Proxy");
                assert_eq!(type_vars_chirho.len(), 1);
                assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
                assert_eq!(
                    type_vars_chirho[0].kind_annotation_chirho,
                    Some(AstKindChirho::StarChirho)
                );
                assert_eq!(constructors_chirho.len(), 1);
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_kind_sig_arrow_kind_chirho() {
        // data HKD (f :: * -> *) = MkHKD
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata HKD (f :: * -> *) = MkHKD\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                type_vars_chirho, ..
            } => {
                assert_eq!(type_vars_chirho.len(), 1);
                assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "f");
                assert_eq!(
                    type_vars_chirho[0].kind_annotation_chirho,
                    Some(AstKindChirho::ArrowChirho(
                        Box::new(AstKindChirho::StarChirho),
                        Box::new(AstKindChirho::StarChirho),
                    ))
                );
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_kind_sig_mixed_chirho() {
        // data Pair (a :: *) b = MkPair a b — mixed annotated and plain
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Pair (a :: *) b = MkPair a b\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                type_vars_chirho, ..
            } => {
                assert_eq!(type_vars_chirho.len(), 2);
                assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
                assert!(type_vars_chirho[0].kind_annotation_chirho.is_some());
                assert_eq!(type_vars_chirho[1].name_chirho.text_chirho(), "b");
                assert!(type_vars_chirho[1].kind_annotation_chirho.is_none());
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_kind_sig_newtype_chirho() {
        // newtype Id (a :: *) = MkId a
        let module_chirho =
            parse_and_lower_chirho("module M where\nnewtype Id (a :: *) = MkId a\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::NewtypeDeclChirho {
                type_vars_chirho, ..
            } => {
                assert_eq!(type_vars_chirho.len(), 1);
                assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
                assert_eq!(
                    type_vars_chirho[0].kind_annotation_chirho,
                    Some(AstKindChirho::StarChirho)
                );
            }
            other_chirho => panic!("expected NewtypeDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_newtype_record_tuple_type_application_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nnewtype WriterTChirho wChirho mChirho aChirho = WriterTChirho { runWriterTChirho :: mChirho (aChirho, wChirho) }\n",
        );
        match &module_chirho.decls_chirho[0] {
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho:
                    ConDeclChirho::RecordChirho { fields_chirho, .. },
                ..
            } => {
                assert_eq!(fields_chirho.len(), 1);
                match &fields_chirho[0].ty_chirho {
                    TypeChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                        ..
                    } => {
                        assert!(
                            matches!(
                                fun_chirho.as_ref(),
                                TypeChirho::VarChirho(name_chirho)
                                    if name_chirho.text_chirho() == "mChirho"
                            ),
                            "expected mChirho type constructor application head, got {:?}",
                            fun_chirho
                        );
                        match arg_chirho.as_ref() {
                            TypeChirho::TupleChirho { elements_chirho, .. } => {
                                assert_eq!(elements_chirho.len(), 2);
                                assert!(
                                    matches!(
                                        &elements_chirho[0],
                                        TypeChirho::VarChirho(name_chirho)
                                            if name_chirho.text_chirho() == "aChirho"
                                    ),
                                    "expected first tuple element aChirho, got {:?}",
                                    elements_chirho[0]
                                );
                                assert!(
                                    matches!(
                                        &elements_chirho[1],
                                        TypeChirho::VarChirho(name_chirho)
                                            if name_chirho.text_chirho() == "wChirho"
                                    ),
                                    "expected second tuple element wChirho, got {:?}",
                                    elements_chirho[1]
                                );
                            }
                            other_chirho => {
                                panic!("expected tuple type arg, got {:?}", other_chirho)
                            }
                        }
                    }
                    other_chirho => panic!("expected type application, got {:?}", other_chirho),
                }
            }
            other_chirho => panic!("expected record newtype, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_newtype_record_function_returning_tuple_application_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nnewtype StateTChirho sChirho mChirho aChirho = StateTChirho { runStateTChirho :: sChirho -> mChirho (aChirho, sChirho) }\n",
        );
        match &module_chirho.decls_chirho[0] {
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho:
                    ConDeclChirho::RecordChirho { fields_chirho, .. },
                ..
            } => {
                assert_eq!(fields_chirho.len(), 1);
                match &fields_chirho[0].ty_chirho {
                    TypeChirho::FunChirho {
                        arg_chirho,
                        result_chirho,
                        ..
                    } => {
                        assert!(
                            matches!(
                                arg_chirho.as_ref(),
                                TypeChirho::VarChirho(name_chirho)
                                    if name_chirho.text_chirho() == "sChirho"
                            ),
                            "expected function arg type sChirho, got {:?}",
                            arg_chirho
                        );
                        match result_chirho.as_ref() {
                            TypeChirho::AppChirho {
                                fun_chirho,
                                arg_chirho,
                                ..
                            } => {
                                assert!(
                                    matches!(
                                        fun_chirho.as_ref(),
                                        TypeChirho::VarChirho(name_chirho)
                                            if name_chirho.text_chirho() == "mChirho"
                                    ),
                                    "expected result head mChirho, got {:?}",
                                    fun_chirho
                                );
                                match arg_chirho.as_ref() {
                                    TypeChirho::TupleChirho { elements_chirho, .. } => {
                                        assert_eq!(elements_chirho.len(), 2);
                                        assert!(
                                            matches!(
                                                &elements_chirho[0],
                                                TypeChirho::VarChirho(name_chirho)
                                                    if name_chirho.text_chirho() == "aChirho"
                                            ),
                                            "expected first tuple element aChirho, got {:?}",
                                            elements_chirho[0]
                                        );
                                        assert!(
                                            matches!(
                                                &elements_chirho[1],
                                                TypeChirho::VarChirho(name_chirho)
                                                    if name_chirho.text_chirho() == "sChirho"
                                            ),
                                            "expected second tuple element sChirho, got {:?}",
                                            elements_chirho[1]
                                        );
                                    }
                                    other_chirho => {
                                        panic!("expected tuple type arg, got {:?}", other_chirho)
                                    }
                                }
                            }
                            other_chirho => {
                                panic!("expected result application, got {:?}", other_chirho)
                            }
                        }
                    }
                    other_chirho => panic!("expected function type, got {:?}", other_chirho),
                }
            }
            other_chirho => panic!("expected record newtype, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_state_constructor_composition_argument_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nnewtype StateTChirho sChirho mChirho aChirho = StateTChirho { runStateTChirho :: sChirho -> mChirho (aChirho, sChirho) }\nstateChirho fChirho = StateTChirho (return . fChirho)\n",
        );
        match &module_chirho.decls_chirho[1] {
            DeclChirho::FunBindChirho { matches_chirho, .. } => {
                match &matches_chirho[0].rhs_chirho {
                    RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                        ..
                    }) => {
                        assert!(matches!(
                            fun_chirho.as_ref(),
                            ExprChirho::ConChirho(name_chirho)
                                if name_chirho.text_chirho() == "StateTChirho"
                        ));
                        assert!(matches!(
                            arg_chirho.as_ref(),
                            ExprChirho::InfixChirho { op_chirho, .. }
                                if op_chirho.text_chirho() == "."
                        ) || matches!(
                            arg_chirho.as_ref(),
                            ExprChirho::ParenChirho { inner_chirho, .. }
                                if matches!(
                                    inner_chirho.as_ref(),
                                    ExprChirho::InfixChirho { op_chirho, .. }
                                        if op_chirho.text_chirho() == "."
                                )
                        ),
                        "expected StateTChirho argument to preserve composition, got {:?}",
                        arg_chirho
                        );
                    }
                    other_chirho => panic!("expected constructor application, got {:?}", other_chirho),
                }
            }
            other_chirho => panic!("expected function binding, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_kind_sig_no_annotation_unchanged_chirho() {
        // data Maybe a = Nothing | Just a — no kind annotation, should be None
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Maybe a = Nothing | Just a\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                type_vars_chirho, ..
            } => {
                assert_eq!(type_vars_chirho.len(), 1);
                assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "a");
                assert!(type_vars_chirho[0].kind_annotation_chirho.is_none());
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_kind_sig_constraint_chirho() {
        // data Dict (c :: Constraint) = MkDict — Constraint kind annotation
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Dict (c :: Constraint) = MkDict\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                type_vars_chirho, ..
            } => {
                assert_eq!(type_vars_chirho.len(), 1);
                assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "c");
                assert_eq!(
                    type_vars_chirho[0].kind_annotation_chirho,
                    Some(AstKindChirho::ConstraintChirho)
                );
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_kind_sig_star_to_constraint_chirho() {
        // data Proxy (c :: * -> Constraint) = MkProxy — arrow kind with Constraint
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Proxy (c :: * -> Constraint) = MkProxy\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                type_vars_chirho, ..
            } => {
                assert_eq!(type_vars_chirho.len(), 1);
                assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "c");
                let expected_chirho = AstKindChirho::ArrowChirho(
                    Box::new(AstKindChirho::StarChirho),
                    Box::new(AstKindChirho::ConstraintChirho),
                );
                assert_eq!(
                    type_vars_chirho[0].kind_annotation_chirho,
                    Some(expected_chirho)
                );
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_standalone_kind_sig_gadt_chirho() {
        // data V :: N -> Type where { VZ :: V Z }
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata V :: N -> Type where\n  VZ :: V Z\n");
        let data_decl_chirho = &module_chirho.decls_chirho[0];
        match data_decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                kind_sig_chirho,
                constructors_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), "V");
                assert!(
                    kind_sig_chirho.is_some(),
                    "standalone kind sig should be Some, got None. Decl: {:?}",
                    data_decl_chirho
                );
                assert!(
                    !constructors_chirho.is_empty(),
                    "should have GADT constructors"
                );
            }
            other_chirho => panic!("expected DataDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_specialize_pragma_chirho() {
        let src_chirho =
            "module M where\n{-# SPECIALIZE double :: Int -> Int #-}\ndouble x = x + x\n";
        let module_chirho = parse_and_lower_chirho(src_chirho);
        assert_eq!(module_chirho.specialize_pragmas_chirho.len(), 1);
        let specs_chirho = module_chirho
            .specialize_pragmas_chirho
            .get("double")
            .expect("missing double");
        assert_eq!(specs_chirho.len(), 1);
        assert_eq!(specs_chirho[0], "Int -> Int");
    }

    #[test]
    fn lower_specialise_british_spelling_chirho() {
        let src_chirho =
            "module M where\n{-# SPECIALISE sort :: [Int] -> [Int] #-}\nsort xs = xs\n";
        let module_chirho = parse_and_lower_chirho(src_chirho);
        assert_eq!(module_chirho.specialize_pragmas_chirho.len(), 1);
        let specs_chirho = module_chirho
            .specialize_pragmas_chirho
            .get("sort")
            .expect("missing sort");
        assert_eq!(specs_chirho.len(), 1);
        assert_eq!(specs_chirho[0], "[Int] -> [Int]");
    }

    #[test]
    fn lower_multiple_specialize_pragmas_chirho() {
        let src_chirho = "module M where\n{-# SPECIALIZE f :: Int -> Int #-}\n{-# SPECIALIZE f :: Double -> Double #-}\nf x = x\n";
        let module_chirho = parse_and_lower_chirho(src_chirho);
        let specs_chirho = module_chirho
            .specialize_pragmas_chirho
            .get("f")
            .expect("missing f");
        assert_eq!(specs_chirho.len(), 2);
        assert_eq!(specs_chirho[0], "Int -> Int");
        assert_eq!(specs_chirho[1], "Double -> Double");
    }

    #[test]
    fn lower_no_specialize_pragma_chirho() {
        let src_chirho = "module M where\nf x = x\n";
        let module_chirho = parse_and_lower_chirho(src_chirho);
        assert!(module_chirho.specialize_pragmas_chirho.is_empty());
    }

    #[test]
    fn lower_default_signature_chirho() {
        let src_chirho = "\
module M where
class Describable a where
  describe :: a -> Int
  default describe :: a -> Int
  describe x = 42
";
        let module_chirho = parse_and_lower_chirho(src_chirho);
        match &module_chirho.decls_chirho[0] {
            DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
                assert_eq!(methods_chirho.len(), 1);
                let method_chirho = &methods_chirho[0];
                assert_eq!(method_chirho.name_chirho.text_chirho(), "describe");
                assert!(
                    method_chirho.default_chirho.is_some(),
                    "default implementation should be present"
                );
                assert!(
                    method_chirho.default_sig_chirho.is_some(),
                    "default signature should be present"
                );
                let sig_text_chirho = method_chirho.default_sig_chirho.as_ref().unwrap();
                assert!(
                    sig_text_chirho.contains("Int"),
                    "default sig should contain 'Int': {}",
                    sig_text_chirho
                );
            }
            other_chirho => panic!("expected ClassDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_class_no_default_signature_chirho() {
        let src_chirho = "\
module M where
class Describable a where
  describe :: a -> Int
  describe x = 42
";
        let module_chirho = parse_and_lower_chirho(src_chirho);
        match &module_chirho.decls_chirho[0] {
            DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
                assert_eq!(methods_chirho.len(), 1);
                assert!(
                    methods_chirho[0].default_sig_chirho.is_none(),
                    "no default sig without 'default' keyword"
                );
            }
            other_chirho => panic!("expected ClassDecl, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_open_type_family_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\ntype family F a\n");
        assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeFamilyDeclChirho { name_chirho, equations_chirho, .. }
                if name_chirho.text_chirho() == "F" && equations_chirho.is_empty())
        }), "should have open type family F with no equations");
    }

    #[test]
    fn lower_closed_type_family_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\ntype family F a where\n  F Int = Bool\n  F Char = Int\n",
        );
        let decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::TypeFamilyDeclChirho { .. }));
        assert!(decl_chirho.is_some(), "should have TypeFamilyDecl");
        match decl_chirho.unwrap() {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                equations_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), "F");
                assert_eq!(equations_chirho.len(), 2, "should have 2 equations");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn lower_type_family_instance_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\ntype instance F Int = Bool\n");
        assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeFamilyInstanceDeclChirho { family_name_chirho, .. }
                if family_name_chirho.text_chirho() == "F")
        }), "should have type family instance for F");
    }

    #[test]
    fn lower_type_family_with_kind_sig_chirho() {
        let module_chirho = parse_and_lower_chirho("module M where\ntype family G a :: Type\n");
        let decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|d_chirho| matches!(d_chirho, DeclChirho::TypeFamilyDeclChirho { .. }));
        assert!(decl_chirho.is_some(), "should have TypeFamilyDecl");
        match decl_chirho.unwrap() {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                result_kind_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), "G");
                assert!(
                    result_kind_chirho.is_some(),
                    "should have result kind annotation"
                );
            }
            _ => unreachable!(),
        }
    }

    // -------------------------------------------------------------------
    // Template Haskell lowering tests
    // -------------------------------------------------------------------

    #[test]
    fn lower_th_splice_name_chirho() {
        // $name lowers to ExprChirho::SpliceChirho
        let module_chirho = parse_and_lower_chirho("module M where\nx = $foo\n");
        let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "x")
        });
        assert!(decl_chirho.is_some(), "should have FunBind for x");
        if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
            let rhs_chirho = &matches_chirho[0].rhs_chirho;
            if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
                assert!(
                    matches!(expr_chirho, ExprChirho::SpliceChirho { .. }),
                    "RHS should be SpliceChirho, got {:?}",
                    expr_chirho
                );
            } else {
                panic!("expected unguarded RHS");
            }
        }
    }

    #[test]
    fn lower_th_splice_parens_chirho() {
        // $(expr) lowers to ExprChirho::SpliceChirho wrapping the inner expression
        let module_chirho = parse_and_lower_chirho("module M where\nx = $(makeLenses foo)\n");
        let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "x")
        });
        assert!(decl_chirho.is_some(), "should have FunBind for x");
        if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
            let rhs_chirho = &matches_chirho[0].rhs_chirho;
            if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
                assert!(
                    matches!(expr_chirho, ExprChirho::SpliceChirho { .. }),
                    "RHS should be SpliceChirho for $(makeLenses foo), got {:?}",
                    expr_chirho
                );
            } else {
                panic!("expected unguarded RHS");
            }
        }
    }

    #[test]
    fn lower_th_splice_decl_chirho() {
        // Top-level $(expr) lowers to DeclChirho::SpliceDeclChirho
        let module_chirho = parse_and_lower_chirho("module M where\n$(makeLenses foo)\n");
        assert!(
            module_chirho
                .decls_chirho
                .iter()
                .any(|d_chirho| { matches!(d_chirho, DeclChirho::SpliceDeclChirho { .. }) }),
            "should have SpliceDeclChirho for top-level $(makeLenses foo): {:?}",
            module_chirho.decls_chirho
        );
    }

    #[test]
    fn lower_th_quote_expr_chirho() {
        // [| expr |] lowers to ExprChirho::QuoteExprChirho
        let module_chirho = parse_and_lower_chirho("module M where\nx = [| foo |]\n");
        let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "x")
        });
        assert!(decl_chirho.is_some(), "should have FunBind for x");
        if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
            let rhs_chirho = &matches_chirho[0].rhs_chirho;
            if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
                assert!(
                    matches!(expr_chirho, ExprChirho::QuoteExprChirho { .. }),
                    "RHS should be QuoteExprChirho for [| foo |], got {:?}",
                    expr_chirho
                );
            } else {
                panic!("expected unguarded RHS");
            }
        }
    }

    #[test]
    fn lower_th_typed_splice_chirho() {
        // $$name lowers to ExprChirho::TypedSpliceChirho
        let module_chirho = parse_and_lower_chirho("module M where\nx = $$bar\n");
        let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "x")
        });
        assert!(decl_chirho.is_some(), "should have FunBind for x");
        if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
            let rhs_chirho = &matches_chirho[0].rhs_chirho;
            if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
                assert!(
                    matches!(expr_chirho, ExprChirho::TypedSpliceChirho { .. }),
                    "RHS should be TypedSpliceChirho for $$bar, got {:?}",
                    expr_chirho
                );
            } else {
                panic!("expected unguarded RHS");
            }
        }
    }

    #[test]
    fn lower_th_quote_type_chirho() {
        // [t| type |] lowers to ExprChirho::QuoteTypeChirho
        let module_chirho = parse_and_lower_chirho("module M where\nx = [t| Int |]\n");
        let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "x")
        });
        assert!(decl_chirho.is_some(), "should have FunBind for x");
        if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
            let rhs_chirho = &matches_chirho[0].rhs_chirho;
            if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
                assert!(
                    matches!(expr_chirho, ExprChirho::QuoteTypeChirho { .. }),
                    "RHS should be QuoteTypeChirho for [t| Int |], got {:?}",
                    expr_chirho
                );
            } else {
                panic!("expected unguarded RHS");
            }
        }
    }

    #[test]
    fn lower_th_quote_pat_chirho() {
        // [p| pat |] lowers to ExprChirho::QuotePatChirho
        let module_chirho = parse_and_lower_chirho("module M where\nx = [p| y |]\n");
        let decl_chirho = module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "x")
        });
        assert!(decl_chirho.is_some(), "should have FunBind for x");
        if let DeclChirho::FunBindChirho { matches_chirho, .. } = decl_chirho.unwrap() {
            let rhs_chirho = &matches_chirho[0].rhs_chirho;
            if let RhsChirho::UnguardedChirho(expr_chirho) = rhs_chirho {
                assert!(
                    matches!(expr_chirho, ExprChirho::QuotePatChirho { .. }),
                    "RHS should be QuotePatChirho for [p| y |], got {:?}",
                    expr_chirho
                );
            } else {
                panic!("expected unguarded RHS");
            }
        }
    }

    #[test]
    fn lower_th_splice_decl_make_lenses_chirho() {
        // $(makeLenses ''Foo) at top level after a data declaration
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Foo = Foo\n$(makeLenses foo)\n");
        assert!(
            module_chirho
                .decls_chirho
                .iter()
                .any(|d_chirho| { matches!(d_chirho, DeclChirho::DataDeclChirho { .. }) }),
            "should have DataDecl"
        );
        assert!(
            module_chirho
                .decls_chirho
                .iter()
                .any(|d_chirho| { matches!(d_chirho, DeclChirho::SpliceDeclChirho { .. }) }),
            "should have SpliceDeclChirho"
        );
    }

    #[test]
    fn lower_strict_data_fields_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Pair = MkPair !Int !Int\n");
        assert_eq!(module_chirho.decls_chirho.len(), 1);
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => {
                assert_eq!(constructors_chirho.len(), 1);
                match &constructors_chirho[0] {
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => {
                        assert_eq!(name_chirho.text_chirho(), "MkPair");
                        assert_eq!(fields_chirho.len(), 2);
                        assert_eq!(fields_chirho[0].0, StrictnessChirho::StrictChirho);
                        assert_eq!(fields_chirho[1].0, StrictnessChirho::StrictChirho);
                    }
                    _ => panic!("expected OrdinaryChirho constructor"),
                }
            }
            _ => panic!("expected DataDeclChirho"),
        }
    }

    #[test]
    fn lower_mixed_strict_lazy_fields_chirho() {
        let module_chirho =
            parse_and_lower_chirho("module M where\ndata Mixed = MkMixed !Int Int\n");
        match &module_chirho.decls_chirho[0] {
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => match &constructors_chirho[0] {
                ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                    assert_eq!(fields_chirho.len(), 2);
                    assert_eq!(fields_chirho[0].0, StrictnessChirho::StrictChirho);
                    assert_eq!(fields_chirho[1].0, StrictnessChirho::LazyChirho);
                }
                _ => panic!("expected OrdinaryChirho"),
            },
            _ => panic!("expected DataDeclChirho"),
        }
    }

    #[test]
    fn lower_nested_dollar_to_right_associated_apps_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nx = WriterT $ callCC $ \\c -> runWriterT (f (\\a -> WriterT (c (a, mempty))))\n",
        );
        let decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|decl_chirho| matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "x"))
            .expect("expected x binding");
        let rhs_expr_chirho = match decl_chirho {
            DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
                RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
                other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
            },
            other_chirho => panic!("expected function binding, got {:?}", other_chirho),
        };

        match rhs_expr_chirho {
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                assert!(
                    matches!(&**fun_chirho, ExprChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "WriterT"),
                    "outer $ should lower to application of WriterT, got {:?}",
                    fun_chirho
                );
                assert!(
                    matches!(&**arg_chirho, ExprChirho::AppChirho { .. }),
                    "nested $ chain should stay as right-associated applications, got {:?}",
                    arg_chirho
                );
            }
            other_chirho => panic!("expected application tree for nested $, got {:?}", other_chirho),
        }
    }

    #[test]
    fn lower_pattern_guard_rhs_to_nested_case_chirho() {
        let module_chirho = parse_and_lower_chirho(
            "module M where\nfChirho xsChirho\n  | msgChirho : _ <- xsChirho, null msgChirho = 1\n  | otherwise = 0\n",
        );
        let decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|decl_chirho| {
                matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "fChirho")
            })
            .expect("expected fChirho binding");

        let rhs_expr_chirho = match decl_chirho {
            DeclChirho::FunBindChirho { matches_chirho, .. } => match &matches_chirho[0].rhs_chirho {
                RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
                other_chirho => panic!("expected lowered nested expression, got {:?}", other_chirho),
            },
            other_chirho => panic!("expected function binding, got {:?}", other_chirho),
        };

        match rhs_expr_chirho {
            ExprChirho::CaseChirho { alts_chirho, .. } => {
                assert_eq!(alts_chirho.len(), 2, "pattern guard should lower to case with fallback");
            }
            other_chirho => panic!("expected top-level case from pattern guard lowering, got {:?}", other_chirho),
            }
        }
    }

/// Extensions enabled by GHC2021 (and GHC2024 which is a superset).
/// Reference: https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/control.html#extension-GHC2021
fn ghc2021_extensions_chirho() -> &'static [&'static str] {
    &[
        "BangPatterns",
        "BinaryLiterals",
        "ConstrainedClassMethods",
        "ConstraintKinds",
        "DeriveDataTypeable",
        "DeriveFoldable",
        "DeriveFunctor",
        "DeriveGeneric",
        "DeriveLift",
        "DeriveTraversable",
        "DoAndIfThenElse",
        "EmptyCase",
        "EmptyDataDecls",
        "EmptyDataDeriving",
        "ExistentialQuantification",
        "ExplicitForAll",
        "FlexibleContexts",
        "FlexibleInstances",
        "ForeignFunctionInterface",
        "GADTSyntax",
        "GeneralizedNewtypeDeriving",
        "HexFloatLiterals",
        "ImportQualifiedPost",
        "InstanceSigs",
        "KindSignatures",
        "MultiParamTypeClasses",
        "NamedFieldPuns",
        "NamedWildCards",
        "NumericUnderscores",
        "PolyKinds",
        "PostfixOperators",
        "RankNTypes",
        "ScopedTypeVariables",
        "StandaloneDeriving",
        "StandaloneKindSignatures",
        "TupleSections",
        "TypeApplications",
        "TypeOperators",
        "TypeSynonymInstances",
    ]
}
