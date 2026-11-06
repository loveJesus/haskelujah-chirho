// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `data` and `newtype` declarations: constructors, GADTs, records, fields and deriving.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DataKindSigChirho, DeclChirho, FieldDeclChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, TypeConstructorRoleChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho, is_type_kind_chirho};

impl LowerCtxChirho {
    pub(super) fn lower_data_decl_chirho(
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
        // A declaration's own kind signature can only appear BEFORE `where`
        // (`data T :: K where`). After it, every `::` belongs to a GADT
        // constructor.
        let mut saw_where_chirho = false;

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::DataKeywordChirho {
                        saw_data_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::WhereKeywordChirho {
                        saw_where_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho
                        && saw_data_chirho
                        && !saw_equals_chirho
                        && !saw_where_chirho
                    {
                        // A datatype context precedes the actual declaration
                        // head: `data (Show a) => T a = ...`. Discard names
                        // collected from the context before scanning `T a`.
                        name_chirho = None;
                        type_vars_chirho.clear();
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
                        && saw_data_chirho
                        && name_chirho.is_some()
                        && !saw_equals_chirho
                        && !saw_where_chirho
                    {
                        // Inline result kind: `data T a :: K where`.
                        // Retain the full signature so the shared flat-type
                        // reconstruction handles forall, arrows, applications,
                        // and symbolic type operators consistently.
                        saw_double_colon_chirho = true;
                        let mut kind_sig_children_chirho = Vec::new();
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
                                _ => kind_sig_children_chirho.push(kc_chirho),
                            }
                            j_chirho += 1;
                        }
                        if !kind_sig_children_chirho.is_empty() {
                            kind_sig_chirho = Some(DataKindSigChirho::ResultChirho(
                                self.declaration_kind_from_children_chirho(
                                    &kind_sig_children_chirho,
                                    span_chirho,
                                ),
                            ));
                        }
                        idx_chirho = j_chirho;
                        continue;
                    } else if saw_data_chirho
                        && !saw_equals_chirho
                        && !saw_double_colon_chirho
                        && !saw_where_chirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if (tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho)
                            && name_chirho.is_none()
                        {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if let Some((binder_chirho, consumed_chirho)) =
                            self.declaration_head_binder_chirho(&children_chirho, idx_chirho)
                        {
                            type_vars_chirho.push(binder_chirho);
                            idx_chirho += consumed_chirho;
                            continue;
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
                        deriving_chirho.extend(
                            self.lower_deriving_clause_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    }
                }
            }
            idx_chirho += 1;
        }

        let mut name_chirho = name_chirho.unwrap_or_else(|| self.dummy_name_chirho());
        if Self::ordinary_type_declaration_header_chirho(
            &children_chirho,
            TokenKindChirho::DataKeywordChirho,
        ) && !name_chirho.text_chirho().is_empty()
        {
            name_chirho
                .set_written_type_declaration_role_chirho(TypeConstructorRoleChirho::DataChirho);
        }
        DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            kind_sig_chirho,
            span_chirho,
        }
    }

    /// A parenthesised data-head binder whose kind annotation `parse_kind_atom_chirho`
    /// cannot read, e.g. `(xss :: [[a]])` or `(p :: forall k. k -> Type)`.
    ///
    /// Returns the index of the binder's name token and the index of the group's
    /// matching `)`, so the caller can bind the variable and skip the rest. Returns
    /// `None` when the group is not of that shape (no leading name, or unbalanced).
    pub(super) fn unreadable_kind_binder_chirho<'a>(
        children_chirho: &[ChildChirho<'a>],
        open_chirho: usize,
    ) -> Option<(&'a GreenTokenChirho, usize, usize)> {
        let name_idx_chirho = open_chirho + 1;
        let name_tok_chirho = match children_chirho.get(name_idx_chirho)?.element_chirho {
            GreenElementChirho::TokenChirho(t_chirho)
                if Self::is_head_binder_name_chirho(t_chirho.kind_chirho()) =>
            {
                t_chirho
            }
            _ => return None,
        };
        // The name must be followed by `::`, otherwise this is not a kind-annotated
        // binder at all and the caller's existing handling should stand.
        match children_chirho.get(name_idx_chirho + 1)?.element_chirho {
            GreenElementChirho::TokenChirho(t_chirho)
                if t_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho => {}
            _ => return None,
        }
        let mut depth_chirho = 0usize;
        let mut i_chirho = open_chirho;
        while i_chirho < children_chirho.len() {
            if let GreenElementChirho::TokenChirho(t_chirho) =
                children_chirho[i_chirho].element_chirho
            {
                match t_chirho.kind_chirho() {
                    TokenKindChirho::LeftParenChirho => depth_chirho += 1,
                    TokenKindChirho::RightParenChirho => {
                        depth_chirho = depth_chirho.saturating_sub(1);
                        if depth_chirho == 0 {
                            return Some((name_tok_chirho, name_idx_chirho, i_chirho));
                        }
                    }
                    _ => {}
                }
            }
            i_chirho += 1;
        }
        None
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

        let (prefix_chirho, start_idx_chirho) =
            self.con_prefix_chirho(&children_chirho, span_chirho);

        let infix_operator_idx_chirho = children_chirho
            .iter()
            .enumerate()
            .skip(start_idx_chirho)
            .find_map(
                |(idx_chirho, child_chirho)| match child_chirho.element_chirho {
                    GreenElementChirho::TokenChirho(tok_chirho)
                        if tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho =>
                    {
                        Some(idx_chirho)
                    }
                    _ => None,
                },
            );

        if let Some(infix_operator_idx_chirho) = infix_operator_idx_chirho
            && let GreenElementChirho::TokenChirho(operator_token_chirho) =
                children_chirho[infix_operator_idx_chirho].element_chirho
        {
            let operator_span_chirho = self.span_chirho(
                children_chirho[infix_operator_idx_chirho].start_chirho,
                children_chirho[infix_operator_idx_chirho].end_chirho,
            );
            let operator_name_chirho =
                self.name_from_token_chirho(operator_token_chirho, operator_span_chirho);
            let left_field_chirho = self.lower_constructor_field_from_children_chirho(
                &children_chirho[start_idx_chirho..infix_operator_idx_chirho],
                span_chirho,
            );
            let right_field_chirho = self.lower_constructor_field_from_children_chirho(
                &children_chirho[infix_operator_idx_chirho + 1..],
                span_chirho,
            );
            if let (Some(left_field_chirho), Some(right_field_chirho)) =
                (left_field_chirho, right_field_chirho)
            {
                return ConDeclChirho::OrdinaryChirho {
                    prefix_chirho,
                    name_chirho: operator_name_chirho,
                    fields_chirho: vec![left_field_chirho, right_field_chirho],
                    span_chirho,
                };
            }
        }

        let mut pending_annotation_chirho: Option<StrictnessChirho> = None;
        for child_chirho in children_chirho.iter().skip(start_idx_chirho) {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if (tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                        || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho)
                        && name_chirho.is_none()
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if let Some(annotation_chirho) = field_annotation_chirho(tok_chirho) {
                        pending_annotation_chirho = Some(annotation_chirho);
                    } else if name_chirho.is_some()
                        && (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho)
                    {
                        // Bare type variable/constructor tokens after the
                        // constructor name (e.g. the `a` in `a :< S a`).
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        let n_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                        let ty_chirho = if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                        {
                            TypeChirho::VarChirho(n_chirho)
                        } else {
                            TypeChirho::ConChirho(n_chirho)
                        };
                        let strictness_chirho = pending_annotation_chirho
                            .take()
                            .unwrap_or(StrictnessChirho::LazyChirho);
                        fields_chirho.push((strictness_chirho, ty_chirho));
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::RecordFieldsChirho {
                        has_record_chirho = true;
                        record_fields_chirho =
                            self.lower_record_fields_chirho(n_chirho, child_chirho.start_chirho);
                    } else if is_type_kind_chirho(n_chirho.kind_chirho()) {
                        let strictness_chirho = pending_annotation_chirho
                            .take()
                            .unwrap_or(StrictnessChirho::LazyChirho);
                        let ty_chirho = self.lower_type_chirho(n_chirho, child_chirho.start_chirho);
                        fields_chirho.push((strictness_chirho, ty_chirho));
                    }
                }
            }
        }

        let con_name_chirho = name_chirho.unwrap_or_else(|| self.dummy_name_chirho());

        // For infix data constructors (e.g. `a :< Seq a`), the parser may
        // split the right-hand type application into separate fields.
        // An infix constructor has exactly 2 fields: one before the operator
        // and one after.  Merge excess fields (idx >= 2) into the second
        // field via left-nested TypeChirho::AppChirho.
        if fields_chirho.len() > 2 && !has_record_chirho {
            let is_infix_chirho = con_name_chirho.text_chirho().starts_with(':')
                || con_name_chirho
                    .text_chirho()
                    .chars()
                    .next()
                    .is_some_and(|c_chirho| !c_chirho.is_alphanumeric() && c_chirho != '_');
            if is_infix_chirho {
                // fields[0] = left operand, fields[1..] = right operand parts
                let right_parts_chirho: Vec<_> = fields_chirho.drain(1..).collect();
                let strictness_chirho = right_parts_chirho[0].0;
                let mut combined_chirho = right_parts_chirho[0].1.clone();
                for part_chirho in &right_parts_chirho[1..] {
                    combined_chirho = TypeChirho::AppChirho {
                        fun_chirho: Box::new(combined_chirho),
                        arg_chirho: Box::new(part_chirho.1.clone()),
                        span_chirho,
                    };
                }
                fields_chirho.push((strictness_chirho, combined_chirho));
            }
        }

        if has_record_chirho {
            ConDeclChirho::RecordChirho {
                prefix_chirho,
                name_chirho: con_name_chirho,
                fields_chirho: record_fields_chirho,
                span_chirho,
            }
        } else {
            ConDeclChirho::OrdinaryChirho {
                prefix_chirho,
                name_chirho: con_name_chirho,
                fields_chirho,
                span_chirho,
            }
        }
    }

    fn lower_constructor_field_from_children_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> Option<(StrictnessChirho, TypeChirho)> {
        let mut pending_annotation_chirho: Option<StrictnessChirho> = None;
        let mut strictness_chirho = StrictnessChirho::LazyChirho;
        let mut parts_chirho: Vec<TypeChirho> = Vec::new();

        for child_chirho in children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => match tok_chirho.kind_chirho() {
                    TokenKindChirho::VarIdChirho | TokenKindChirho::ConIdChirho => {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                        let ty_chirho = if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                        {
                            TypeChirho::VarChirho(name_chirho)
                        } else {
                            TypeChirho::ConChirho(name_chirho)
                        };
                        if let Some(annotation_chirho) = pending_annotation_chirho.take() {
                            strictness_chirho = annotation_chirho;
                        }
                        parts_chirho.push(ty_chirho);
                    }
                    _ => {
                        if let Some(annotation_chirho) = field_annotation_chirho(tok_chirho) {
                            pending_annotation_chirho = Some(annotation_chirho);
                        }
                    }
                },
                GreenElementChirho::NodeChirho(node_chirho)
                    if is_type_kind_chirho(node_chirho.kind_chirho()) =>
                {
                    if let Some(annotation_chirho) = pending_annotation_chirho.take() {
                        strictness_chirho = annotation_chirho;
                    }
                    parts_chirho
                        .push(self.lower_type_chirho(node_chirho, child_chirho.start_chirho));
                }
                _ => {}
            }
        }

        let mut parts_iter_chirho = parts_chirho.into_iter();
        let mut combined_chirho = parts_iter_chirho.next()?;
        for part_chirho in parts_iter_chirho {
            combined_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(combined_chirho),
                arg_chirho: Box::new(part_chirho),
                span_chirho,
            };
        }

        Some((strictness_chirho, combined_chirho))
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

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho
                && matches!(
                    token_chirho.kind_chirho(),
                    TokenKindChirho::ConIdChirho
                        | TokenKindChirho::ConSymChirho
                        | TokenKindChirho::QualifiedConSymChirho
                )
                && name_chirho.is_none()
            {
                name_chirho = Some(self.name_from_token_chirho(
                    token_chirho,
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                ));
            }
        }
        ConDeclChirho::GadtChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            signature_chirho: Box::new(self.lower_gadt_signature_chirho(node_chirho, base_chirho)),
            span_chirho,
        }
    }

    /// Extract class names before "via" from a `deriving (Class) via Type` clause.
    /// Unlike `lower_deriving_clause_chirho`, this returns the classes when via is present.
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
    pub(super) fn extract_deriving_via_chirho(
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
                                    && let Some(via_type_chirho) = self
                                        .lower_deriving_via_type_chirho(
                                            sub_n_chirho,
                                            sc_chirho.start_chirho,
                                        )
                                    && let Some(ref tn_chirho) = type_name_chirho
                                {
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
        result_chirho
    }

    /// A skipped data/newtype instance may still have a data-shaped CST node.
    /// Only the actual ordinary defining header supplies the positive role.
    /// Workflow: compiler-pipeline-chirho/type-scope-resolution-chirho#ordinary-type-definition-proof-chirho.
    fn ordinary_type_declaration_header_chirho(
        children_chirho: &[ChildChirho<'_>],
        keyword_chirho: TokenKindChirho,
    ) -> bool {
        let mut saw_keyword_chirho = false;
        for child_chirho in children_chirho {
            if let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho {
                match token_chirho.kind_chirho() {
                    TokenKindChirho::InstanceKeywordChirho => return false,
                    // The legacy newtype lowerer has not retained the actual
                    // head after a declaration context. Do not certify its
                    // accidentally collected context-class name as a newtype.
                    TokenKindChirho::DoubleArrowChirho
                        if keyword_chirho == TokenKindChirho::NewtypeKeywordChirho =>
                    {
                        return false;
                    }
                    TokenKindChirho::EqualsChirho | TokenKindChirho::WhereKeywordChirho => break,
                    kind_chirho if kind_chirho == keyword_chirho => saw_keyword_chirho = true,
                    _ => {}
                }
            }
        }
        saw_keyword_chirho
    }

    pub(super) fn lower_newtype_decl_chirho(
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
        let mut kind_sig_children_chirho = Vec::new();
        let mut saw_equals_chirho = false;
        let mut saw_newtype_chirho = false;
        let mut saw_double_colon_chirho = false;
        let mut saw_where_chirho = false;

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    if tok_chirho.kind_chirho() == TokenKindChirho::NewtypeKeywordChirho {
                        saw_newtype_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::WhereKeywordChirho {
                        saw_where_chirho = true;
                    } else if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
                        && saw_newtype_chirho
                        && !saw_equals_chirho
                        && !saw_where_chirho
                    {
                        saw_double_colon_chirho = true;
                    } else if saw_double_colon_chirho && !saw_where_chirho {
                        kind_sig_children_chirho.push(child_chirho);
                    } else if saw_newtype_chirho && !saw_equals_chirho && !saw_where_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if (tok_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                            || tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho)
                            && name_chirho.is_none()
                        {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if let Some((binder_chirho, consumed_chirho)) =
                            self.declaration_head_binder_chirho(&children_chirho, idx_chirho)
                        {
                            type_vars_chirho.push(binder_chirho);
                            idx_chirho += consumed_chirho;
                            continue;
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_double_colon_chirho
                        && !saw_where_chirho
                        && is_type_kind_chirho(n_chirho.kind_chirho())
                    {
                        kind_sig_children_chirho.push(child_chirho);
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::ConDeclChirho {
                        constructor_chirho =
                            Some(self.lower_con_decl_chirho(n_chirho, child_chirho.start_chirho));
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::GadtConDeclChirho {
                        constructor_chirho = Some(
                            self.lower_gadt_con_decl_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    } else if n_chirho.kind_chirho() == SyntaxKindChirho::DerivingClauseChirho {
                        deriving_chirho.extend(
                            self.lower_deriving_clause_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    }
                }
            }
            idx_chirho += 1;
        }

        let kind_sig_chirho = if kind_sig_children_chirho.is_empty() {
            None
        } else {
            Some(DataKindSigChirho::ResultChirho(
                self.declaration_kind_from_children_chirho(&kind_sig_children_chirho, span_chirho),
            ))
        };

        let mut name_chirho = name_chirho.unwrap_or_else(|| self.dummy_name_chirho());
        if Self::ordinary_type_declaration_header_chirho(
            &children_chirho,
            TokenKindChirho::NewtypeKeywordChirho,
        ) && !name_chirho.text_chirho().is_empty()
        {
            name_chirho
                .set_written_type_declaration_role_chirho(TypeConstructorRoleChirho::NewtypeChirho);
        }
        DeclChirho::NewtypeDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructor_chirho: constructor_chirho.unwrap_or_else(|| {
                ConDeclChirho::OrdinaryChirho {
                    prefix_chirho: Default::default(),
                    name_chirho: self.dummy_name_chirho(),
                    fields_chirho: vec![],
                    span_chirho,
                }
            }),
            deriving_chirho,
            kind_sig_chirho,
            span_chirho,
        }
    }

    /// Lower a `FieldDeclChirho` CST node to an AST `FieldDeclChirho`.
    /// CST structure: VarIdChirho [, VarIdChirho]* :: TypeChirho
    ///
    /// The CST parser produces flat tokens for field types (no structured
    /// type subnodes), so we reconstruct the type from the flat sequence
    /// after `::` using `type_from_flat_children_chirho`.
    pub(super) fn lower_field_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> FieldDeclChirho {
        self.lower_field_decl_mode_chirho(node_chirho, base_chirho, false)
    }

    pub(super) fn lower_field_decl_mode_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        gadt_field_chirho: bool,
    ) -> FieldDeclChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut names_chirho = Vec::new();
        let mut ty_chirho: Option<TypeChirho> = None;
        let mut saw_double_colon_chirho = false;
        let mut type_children_chirho: Vec<&ChildChirho> = Vec::new();
        let mut annotation_chirho: Option<StrictnessChirho> = None;
        let mut in_multiplicity_chirho = false;
        let mut multiplicity_children_chirho = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    // Only the first `::` separates the field names from the
                    // type. A later one is a kind annotation inside the type
                    // and must reach the type grammar, or `(a :: Type)` loses
                    // its `::` and reads as the application `a Type`.
                    if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
                        && !saw_double_colon_chirho
                    {
                        saw_double_colon_chirho = true;
                    } else if gadt_field_chirho
                        && !saw_double_colon_chirho
                        && tok_chirho.text_chirho() == "%"
                    {
                        in_multiplicity_chirho = true;
                        multiplicity_children_chirho.push(ChildChirho {
                            element_chirho: child_chirho.element_chirho,
                            start_chirho: child_chirho.start_chirho,
                            end_chirho: child_chirho.end_chirho,
                        });
                    } else if gadt_field_chirho
                        && !saw_double_colon_chirho
                        && in_multiplicity_chirho
                    {
                        multiplicity_children_chirho.push(ChildChirho {
                            element_chirho: child_chirho.element_chirho,
                            start_chirho: child_chirho.start_chirho,
                            end_chirho: child_chirho.end_chirho,
                        });
                    } else if !saw_double_colon_chirho
                        && tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        names_chirho.push(self.name_from_token_chirho(tok_chirho, s_chirho));
                    } else if saw_double_colon_chirho {
                        // A strictness annotation on the field type: `!` wherever it
                        // stands, as before, and `~` only as the type's first token,
                        // since a `~` inside the type is type equality.
                        let leading_chirho = ty_chirho.is_none() && type_children_chirho.is_empty();
                        match field_annotation_chirho(tok_chirho) {
                            Some(StrictnessChirho::ExplicitLazyChirho) if !leading_chirho => {
                                type_children_chirho.push(child_chirho);
                            }
                            Some(found_chirho) => annotation_chirho = Some(found_chirho),
                            None => type_children_chirho.push(child_chirho),
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

        let unpacked_chirho = gadt_field_chirho && node_chirho.children_chirho().iter().any(|child_chirho| matches!(child_chirho,
            GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::PragmaChirho
                && token_chirho.text_chirho().strip_prefix("{-#").and_then(|body_chirho| body_chirho.strip_suffix("#-}")).is_some_and(|body_chirho| body_chirho.trim() == "UNPACK")
        ));
        let strictness_chirho =
            if annotation_chirho == Some(StrictnessChirho::StrictChirho) && unpacked_chirho {
                StrictnessChirho::UnpackChirho
            } else {
                annotation_chirho.unwrap_or(StrictnessChirho::LazyChirho)
            };

        FieldDeclChirho {
            names_chirho,
            ty_chirho: ty_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
            strictness_chirho,
            multiplicity_chirho: self
                .lower_arrow_multiplicity_chirho(&multiplicity_children_chirho),
            span_chirho,
        }
    }
}

/// The strictness a field annotation gives the field type after it: `!` makes the
/// field strict and `~` explicitly lazy (validity judges where GHC allows `~`).
/// Any other token is no annotation.
/// workflow: compiler-pipeline-chirho/declaration-validity-chirho
fn field_annotation_chirho(tok_chirho: &GreenTokenChirho) -> Option<StrictnessChirho> {
    match tok_chirho.kind_chirho() {
        TokenKindChirho::VarSymChirho if tok_chirho.text_chirho() == "!" => {
            Some(StrictnessChirho::StrictChirho)
        }
        TokenKindChirho::TildeChirho => Some(StrictnessChirho::ExplicitLazyChirho),
        _ => None,
    }
}
