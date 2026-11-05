// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The source file and module header, the export list and imports.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, ForeignDirectionChirho};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
    ModuleChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::LowerCtxChirho;
use super::bindings_chirho::merge_fun_binds_chirho;
use super::context_chirho::is_trivia_element_chirho;

impl LowerCtxChirho {
    // -----------------------------------------------------------------------
    // SourceFile
    // -----------------------------------------------------------------------

    pub(super) fn lower_source_file_chirho(
        &mut self,
        root_chirho: &GreenNodeChirho,
    ) -> ModuleChirho {
        let start_chirho = self.offset_chirho;
        let children_chirho = self.semantic_children_chirho(root_chirho, start_chirho);

        let mut module_name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "Main",
            SpanChirho::DUMMY_CHIRHO,
        ));
        let mut exports_chirho: Option<Vec<ExportSpecChirho>> = None;
        let mut imports_chirho = Vec::new();
        let mut decls_chirho = Vec::new();
        let mut standalone_kind_sigs_chirho: HashMap<String, TypeChirho> = HashMap::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                && n_chirho.kind_chirho() == SyntaxKindChirho::FixityDeclChirho
            {
                let (fixity_chirho, precedence_chirho, ops_chirho) =
                    self.extract_fixity_decl_parts_chirho(n_chirho, child_chirho.start_chirho);
                let stored_precedence_chirho = precedence_chirho.unwrap_or(9);
                for op_chirho in ops_chirho {
                    self.fixity_overrides_chirho.insert(
                        op_chirho.text_chirho().to_string(),
                        (stored_precedence_chirho, fixity_chirho),
                    );
                }
            }
        }

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
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if let Some((name_chirho, kind_chirho)) = self
                            .lower_standalone_kind_sig_chirho(
                                n_chirho,
                                child_chirho.start_chirho,
                                span_chirho,
                            )
                        {
                            standalone_kind_sigs_chirho.insert(name_chirho, kind_chirho);
                        } else {
                            let decls_from_sig_chirho = self.lower_type_sigs_chirho(
                                n_chirho,
                                child_chirho.start_chirho,
                                span_chirho,
                            );
                            decls_chirho.extend(decls_from_sig_chirho);
                        }
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
        let decls_chirho =
            self.attach_standalone_kind_sigs_chirho(decls_chirho, &standalone_kind_sigs_chirho);
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
            origin_supply_chirho: self.take_origin_supply_chirho(),
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
        let mut saw_module_keyword_chirho = false;
        let mut before_export_list_chirho = true;

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => match tok_chirho.kind_chirho() {
                    TokenKindChirho::ModuleKeywordChirho => {
                        saw_module_keyword_chirho = true;
                    }
                    TokenKindChirho::VarIdChirho
                    | TokenKindChirho::ConIdChirho
                    | TokenKindChirho::QualifiedVarIdChirho
                    | TokenKindChirho::QualifiedConIdChirho
                        if saw_module_keyword_chirho
                            && before_export_list_chirho
                            && name_chirho.is_none() =>
                    {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        name_chirho =
                            Some(self.module_name_from_text_chirho(
                                tok_chirho.text_chirho(),
                                span_chirho,
                            ));
                    }
                    _ => {}
                },
                GreenElementChirho::NodeChirho(n_chirho)
                    if n_chirho.kind_chirho() == SyntaxKindChirho::ExportListChirho =>
                {
                    before_export_list_chirho = false;
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
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                && n_chirho.kind_chirho() == SyntaxKindChirho::ExportSpecChirho
                && let Some(spec_chirho) =
                    self.lower_export_spec_item_chirho(n_chirho, child_chirho.start_chirho)
            {
                exports_chirho.push(spec_chirho);
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
        let mut explicit_type_chirho = false;
        let mut has_parens_chirho = false;
        let mut has_dotdot_chirho = false;
        let mut members_chirho: Vec<NameChirho> = Vec::new();
        let mut paren_depth_chirho = 0_usize;
        let mut next_member_type_chirho = false;

        let mut offset_chirho = base_chirho;
        for elem_chirho in node_chirho.children_chirho() {
            let elem_start_chirho = offset_chirho;
            let elem_end_chirho = offset_chirho + elem_chirho.text_len_chirho();
            if !is_trivia_element_chirho(elem_chirho)
                && let GreenElementChirho::TokenChirho(tok_chirho) = elem_chirho
            {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::TypeKeywordChirho => {
                        if first_name_chirho.is_none() {
                            explicit_type_chirho = true;
                        } else if paren_depth_chirho > 0 {
                            next_member_type_chirho = true;
                        }
                    }
                    TokenKindChirho::ModuleKeywordChirho => {
                        is_module_reexport_chirho = true;
                    }
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() && paren_depth_chirho == 0 {
                            first_name_chirho = Some((
                                tok_chirho.text_chirho().to_string(),
                                span_chirho,
                                !is_module_reexport_chirho,
                            ));
                        } else if paren_depth_chirho > 0 {
                            let mut member_chirho =
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho);
                            member_chirho.set_explicit_type_member_chirho(next_member_type_chirho);
                            members_chirho.push(member_chirho);
                            next_member_type_chirho = false;
                        }
                    }
                    TokenKindChirho::VarIdChirho | TokenKindChirho::QualifiedVarIdChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() && paren_depth_chirho == 0 {
                            first_name_chirho =
                                Some((tok_chirho.text_chirho().to_string(), span_chirho, false));
                        } else if paren_depth_chirho > 0 {
                            let mut member_chirho =
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho);
                            member_chirho.set_explicit_type_member_chirho(next_member_type_chirho);
                            members_chirho.push(member_chirho);
                            next_member_type_chirho = false;
                        }
                    }
                    TokenKindChirho::VarSymChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::QualifiedVarSymChirho
                    | TokenKindChirho::QualifiedConSymChirho => {
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() {
                            first_name_chirho =
                                Some((tok_chirho.text_chirho().to_string(), span_chirho, false));
                        } else if paren_depth_chirho > 0 {
                            let mut member_chirho =
                                self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho);
                            member_chirho.set_explicit_type_member_chirho(next_member_type_chirho);
                            members_chirho.push(member_chirho);
                            next_member_type_chirho = false;
                        }
                    }
                    TokenKindChirho::LeftParenChirho => {
                        has_parens_chirho = true;
                        paren_depth_chirho += 1;
                    }
                    TokenKindChirho::RightParenChirho => {
                        paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                    }
                    TokenKindChirho::CommaChirho => {
                        next_member_type_chirho = false;
                    }
                    TokenKindChirho::DotDotChirho => {
                        has_dotdot_chirho = true;
                    }
                    _ => {}
                }
            }
            offset_chirho = elem_end_chirho;
        }

        let (text_chirho, span_chirho, is_con_chirho) = first_name_chirho?;
        let is_con_chirho = is_con_chirho || explicit_type_chirho;
        let name_chirho = if is_module_reexport_chirho {
            self.module_name_from_text_chirho(&text_chirho, span_chirho)
        } else {
            self.name_from_text_chirho(&text_chirho, span_chirho)
        };

        let symbolic_member_export_chirho = !is_con_chirho
            && has_parens_chirho
            && (has_dotdot_chirho || !members_chirho.is_empty())
            && text_chirho
                .chars()
                .all(|char_chirho| !char_chirho.is_alphanumeric() && char_chirho != '_');

        if is_module_reexport_chirho {
            Some(ExportSpecChirho::ModuleChirho(name_chirho))
        } else if (is_con_chirho && has_parens_chirho) || symbolic_member_export_chirho {
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
        // SOURCE is pragma trivia to the ordinary semantic walk, but its
        // import edge must survive lowering for reached hs-boot contracts.
        let source_chirho = node_chirho.children_chirho().iter().any(|element_chirho| {
            matches!(element_chirho, GreenElementChirho::TokenChirho(token_chirho)
                if token_chirho.kind_chirho() == TokenKindChirho::PragmaChirho
                    && token_chirho.text_chirho().strip_prefix("{-#")
                        .and_then(|body_chirho| body_chirho.strip_suffix("#-}"))
                        .is_some_and(|body_chirho| body_chirho.trim() == "SOURCE"))
        });
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut package_chirho = None;
        let mut qualified_chirho = false;
        let mut module_name_chirho = None;
        let mut alias_chirho = None;
        let mut saw_as_chirho = false;

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::StringLiteralChirho if module_name_chirho.is_none() => {
                        let text_chirho = tok_chirho.text_chirho();
                        let body_chirho = text_chirho
                            .strip_prefix('"')
                            .and_then(|body_chirho| body_chirho.strip_suffix('"'))
                            .unwrap_or(text_chirho);
                        package_chirho = Some(
                            haskelujah_ast_chirho::lit_chirho::unescape_string_body_chirho(
                                body_chirho,
                            ),
                        );
                    }
                    TokenKindChirho::VarIdChirho if tok_chirho.text_chirho() == "qualified" => {
                        qualified_chirho = true;
                    }
                    TokenKindChirho::VarIdChirho if tok_chirho.text_chirho() == "as" => {
                        saw_as_chirho = true;
                    }
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        let name_chirho = if saw_as_chirho {
                            self.name_from_text_chirho(tok_chirho.text_chirho(), span_chirho)
                        } else {
                            self.module_name_from_text_chirho(tok_chirho.text_chirho(), span_chirho)
                        };
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
            package_chirho,
            source_chirho,
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
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                && n_chirho.kind_chirho() == SyntaxKindChirho::ImportSpecChirho
            {
                items_chirho
                    .push(self.lower_import_spec_item_chirho(n_chirho, child_chirho.start_chirho));
            }
        }

        items_chirho
    }

    fn lower_import_spec_item_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> ImportItemChirho {
        let mut first_name_chirho: Option<(String, SpanChirho, bool)> = None; // (text, span, is_con)
        let mut name_tokens_chirho = 0usize;
        let mut has_parens_chirho = false;
        let mut has_dotdot_chirho = false;
        let mut members_chirho: Vec<NameChirho> = Vec::new();
        let mut in_parens_chirho = false;
        // `data` ahead of the name puts the item in the value namespace.
        let mut data_namespace_chirho = false;
        // A paren after the name opens a member list; one before it wraps an
        // operator name, as in `(+)` or `data (:+)`.
        let mut member_list_chirho = false;

        let mut offset_chirho = base_chirho;
        for elem_chirho in node_chirho.children_chirho() {
            let elem_start_chirho = offset_chirho;
            let elem_end_chirho = offset_chirho + elem_chirho.text_len_chirho();
            if let GreenElementChirho::TokenChirho(tok_chirho) = elem_chirho {
                match tok_chirho.kind_chirho() {
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                        name_tokens_chirho += 1;
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
                        name_tokens_chirho += 1;
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
                    TokenKindChirho::VarSymChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::QualifiedVarSymChirho
                    | TokenKindChirho::QualifiedConSymChirho => {
                        name_tokens_chirho += 1;
                        let span_chirho = self.span_chirho(elem_start_chirho, elem_end_chirho);
                        if first_name_chirho.is_none() {
                            first_name_chirho = Some((
                                tok_chirho.text_chirho().to_string(),
                                span_chirho,
                                matches!(
                                    tok_chirho.kind_chirho(),
                                    TokenKindChirho::ConSymChirho
                                        | TokenKindChirho::QualifiedConSymChirho
                                ),
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
                        member_list_chirho |= first_name_chirho.is_some();
                    }
                    TokenKindChirho::RightParenChirho => {
                        in_parens_chirho = false;
                    }
                    TokenKindChirho::DotDotChirho => {
                        has_dotdot_chirho = true;
                    }
                    TokenKindChirho::DataKeywordChirho
                        if first_name_chirho.is_none() && !in_parens_chirho =>
                    {
                        data_namespace_chirho = true;
                    }
                    _ => {}
                }
            }
            offset_chirho = elem_end_chirho;
        }

        // An item with no faithful AST item is kept, not dropped or misread,
        // so a list holding it reads as incomplete.
        // workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
        let item_span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let Some((text_chirho, span_chirho, is_con_chirho)) = first_name_chirho else {
            // A nameless item, such as the namespace wildcard `data ..` or
            // `type ..`, names nothing lowering can represent.
            return ImportItemChirho::UnrepresentedChirho(item_span_chirho);
        };
        if data_namespace_chirho {
            // `data x` names the value `x` whatever its spelling: GHC 9.14.1
            // imports the constructor for `data Proxy`, never the type. A
            // member list or any other suffix after the name has no faithful
            // item, so it is not folded into the name.
            return if name_tokens_chirho == 1 && !member_list_chirho && !has_dotdot_chirho {
                ImportItemChirho::VarChirho(self.name_from_text_chirho(&text_chirho, span_chirho))
            } else {
                ImportItemChirho::UnrepresentedChirho(item_span_chirho)
            };
        }
        if !is_con_chirho && text_chirho == "pattern" && name_tokens_chirho > 1 {
            // `pattern P` imports a pattern synonym, not a variable named
            // `pattern`, and lowering has no item for it.
            return ImportItemChirho::UnrepresentedChirho(item_span_chirho);
        }
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
            ImportItemChirho::TyConChirho {
                name_chirho,
                members_chirho: members_spec_chirho,
            }
        } else if is_con_chirho {
            // Type/class with no parens — just the name, no members
            ImportItemChirho::TyConChirho {
                name_chirho,
                members_chirho: ExportMembersChirho::NoneChirho,
            }
        } else {
            // Variable
            ImportItemChirho::VarChirho(name_chirho)
        }
    }
}
