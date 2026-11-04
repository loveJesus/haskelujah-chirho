// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type synonyms, type families and their instances.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::{
    DeclChirho, TyVarChirho, TyVarVisibilityChirho, TypeFamilyEquationChirho,
};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    pub(super) fn lower_type_alias_decl_chirho(
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
                        } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho
                            && let Some((tv_chirho, skip_chirho)) = self
                                .try_parse_kind_annotated_tyvar_chirho(&children_chirho, idx_chirho)
                        {
                            type_vars_chirho.push(tv_chirho);
                            idx_chirho += skip_chirho;
                            continue;
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

    /// A class body's associated type declaration or default,
    /// `type F binders [:: kind] [= rhs]`. Binders are read like declaration-head
    /// binders (an `@` binder is invisible, an unreadable kind is one skipped group),
    /// and the result kind after `::` binds nothing, so the family's arity counts
    /// only its written parameters: `type T1 (x :: f a) :: f a` has one (row 686,
    /// T17566). Ordinary synonyms keep `lower_type_alias_decl_chirho`.
    /// workflow: compiler-pipeline-chirho/declaration-validity-chirho
    pub(super) fn lower_associated_type_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        let mut rhs_chirho = None;
        let mut result_kind_chirho = None;
        let mut saw_type_chirho = false;
        let mut saw_result_kind_chirho = false;
        let mut saw_equals_chirho = false;
        let mut saw_injectivity_chirho = false;
        let mut kind_names_chirho = Vec::new();

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::TypeKeywordChirho {
                        saw_type_chirho = true;
                    } else if kind_chirho == TokenKindChirho::EqualsChirho {
                        saw_equals_chirho = true;
                    } else if saw_equals_chirho && kind_chirho == TokenKindChirho::PipeChirho {
                        saw_injectivity_chirho = true;
                    } else if saw_type_chirho && !saw_equals_chirho && !saw_result_kind_chirho {
                        if kind_chirho == TokenKindChirho::DoubleColonChirho {
                            saw_result_kind_chirho = true;
                            let kind_start_chirho = idx_chirho + 1;
                            let mut kind_end_chirho = kind_start_chirho;
                            while kind_end_chirho < children_chirho.len() {
                                if matches!(children_chirho[kind_end_chirho].element_chirho,
                                    GreenElementChirho::TokenChirho(token_chirho)
                                        if token_chirho.kind_chirho() == TokenKindChirho::EqualsChirho)
                                {
                                    break;
                                }
                                kind_end_chirho += 1;
                            }
                            let kind_children_chirho = children_chirho
                                [kind_start_chirho..kind_end_chirho]
                                .iter()
                                .collect::<Vec<_>>();
                            result_kind_chirho = Some(self.declaration_kind_from_children_chirho(
                                &kind_children_chirho,
                                span_chirho,
                            ));
                            idx_chirho = kind_end_chirho;
                            continue;
                        } else if name_chirho.is_none()
                            && matches!(
                                kind_chirho,
                                TokenKindChirho::ConIdChirho
                                    | TokenKindChirho::ConSymChirho
                                    | TokenKindChirho::VarSymChirho
                            )
                        {
                            let s_chirho = self
                                .span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if let Some((binder_chirho, consumed_chirho)) =
                            self.declaration_head_binder_chirho(&children_chirho, idx_chirho)
                        {
                            self.collect_group_kind_names_chirho(
                                &children_chirho[idx_chirho..idx_chirho + consumed_chirho],
                                &mut kind_names_chirho,
                            );
                            type_vars_chirho.push(binder_chirho);
                            idx_chirho += consumed_chirho;
                            continue;
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

        // The kind variables the binders' kinds write are bound by this equation
        // too. They are invisible binders, so never family parameters.
        for kind_name_chirho in kind_names_chirho {
            if !type_vars_chirho.iter().any(|binder_chirho: &TyVarChirho| {
                binder_chirho.name_chirho.text_chirho() == kind_name_chirho.text_chirho()
            }) {
                let mut binder_chirho = TyVarChirho::plain_chirho(kind_name_chirho);
                binder_chirho.visibility_chirho = TyVarVisibilityChirho::InvisibleChirho;
                type_vars_chirho.push(binder_chirho);
            }
        }
        // `= r | r -> a` names the result, never a default or parameter.
        // Its explicit kind is retained from the already-lowered RHS syntax.
        if saw_injectivity_chirho {
            if let Some(mut named_result_chirho) = rhs_chirho.take() {
                loop {
                    match named_result_chirho {
                        TypeChirho::ParenChirho { inner_chirho, .. } => {
                            named_result_chirho = *inner_chirho
                        }
                        TypeChirho::KindAnnotationChirho { kind_chirho, .. } => {
                            if result_kind_chirho.is_none() {
                                result_kind_chirho = Some(*kind_chirho);
                            }
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }
        let name_chirho = name_chirho.unwrap_or_else(|| self.dummy_name_chirho());
        let equations_chirho = if saw_equals_chirho && !saw_injectivity_chirho {
            vec![TypeFamilyEquationChirho {
                head_chirho: Some(name_chirho.clone()),
                kind_args_chirho: vec![],
                explicit_binders_chirho: None,
                unwritten_binders_chirho: vec![],
                lhs_types_chirho: type_vars_chirho
                    .iter()
                    .filter(|binder_chirho| binder_chirho.is_visible_chirho())
                    .map(|binder_chirho| TypeChirho::VarChirho(binder_chirho.name_chirho.clone()))
                    .collect(),
                malformed_chirho: rhs_chirho.is_none(),
                rhs_chirho: rhs_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
                span_chirho,
            }]
        } else {
            vec![]
        };
        DeclChirho::TypeFamilyDeclChirho {
            name_chirho,
            type_vars_chirho,
            result_kind_chirho,
            standalone_kind_chirho: None,
            data_family_chirho: false,
            closed_chirho: false,
            equations_chirho,
            span_chirho,
        }
    }

    /// The kind variables a binder group `(v :: kind)` writes free in its kind,
    /// read from the CST tokens, so a kind the binder grammar cannot read still
    /// binds them (row 686). A `forall` binds its own names up to the end of the
    /// parenthesized group it is written in.
    /// workflow: compiler-pipeline-chirho/type-scope-resolution-chirho
    fn collect_group_kind_names_chirho(
        &self,
        group_chirho: &[ChildChirho<'_>],
        out_chirho: &mut Vec<NameChirho>,
    ) {
        let Some(colon_chirho) = group_chirho.iter().position(|child_chirho| {
            matches!(
                child_chirho.element_chirho,
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
            )
        }) else {
            return;
        };
        let mut depth_chirho = 0usize;
        let mut foralls_chirho: Vec<(usize, Vec<String>)> = Vec::new();
        let mut in_telescope_chirho = false;
        for child_chirho in &group_chirho[colon_chirho + 1..] {
            let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho else {
                continue;
            };
            let kind_chirho = tok_chirho.kind_chirho();
            let text_chirho = tok_chirho.text_chirho();
            if kind_chirho == TokenKindChirho::ForallKeywordChirho
                || (kind_chirho == TokenKindChirho::VarIdChirho && text_chirho == "forall")
            {
                in_telescope_chirho = true;
                foralls_chirho.push((depth_chirho, Vec::new()));
                continue;
            }
            match kind_chirho {
                TokenKindChirho::LeftParenChirho | TokenKindChirho::LeftBracketChirho => {
                    depth_chirho += 1;
                }
                TokenKindChirho::RightParenChirho | TokenKindChirho::RightBracketChirho => {
                    depth_chirho = depth_chirho.saturating_sub(1);
                    foralls_chirho.retain(|(scope_chirho, _)| *scope_chirho <= depth_chirho);
                }
                TokenKindChirho::VarIdChirho if in_telescope_chirho => {
                    if let Some((_, bound_chirho)) = foralls_chirho.last_mut() {
                        bound_chirho.push(text_chirho.to_owned());
                    }
                }
                TokenKindChirho::VarIdChirho => {
                    let bound_by_forall_chirho = foralls_chirho.iter().any(|(_, bound_chirho)| {
                        bound_chirho
                            .iter()
                            .any(|bound_name_chirho| bound_name_chirho == text_chirho)
                    });
                    let seen_chirho = out_chirho
                        .iter()
                        .any(|name_chirho| name_chirho.text_chirho() == text_chirho);
                    if !bound_by_forall_chirho && !seen_chirho {
                        let span_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        out_chirho.push(self.name_from_token_chirho(tok_chirho, span_chirho));
                    }
                }
                TokenKindChirho::VarSymChirho if text_chirho == "." => in_telescope_chirho = false,
                TokenKindChirho::RightArrowChirho => in_telescope_chirho = false,
                _ => {}
            }
        }
    }

    pub(super) fn lower_type_family_decl_chirho(
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
        let mut data_family_chirho = false;
        let mut saw_where_chirho = false;
        let mut saw_double_colon_chirho = false;
        let mut saw_injectivity_result_chirho = false;
        let mut skip_visible_binder_chirho = false;

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::DataKeywordChirho {
                        data_family_chirho = true;
                    } else if kind_chirho == TokenKindChirho::TypeKeywordChirho {
                        // skip
                    } else if tok_chirho.text_chirho() == "family" {
                        saw_family_chirho = true;
                    } else if kind_chirho == TokenKindChirho::WhereKeywordChirho {
                        saw_where_chirho = true;
                    } else if kind_chirho == TokenKindChirho::DoubleColonChirho && !saw_where_chirho
                    {
                        saw_double_colon_chirho = true;
                    } else if kind_chirho == TokenKindChirho::EqualsChirho {
                        if !saw_where_chirho && saw_family_chirho {
                            // An open family's injectivity annotation starts
                            // with `= result | result -> parameter`. None of
                            // those names are additional family parameters.
                            saw_injectivity_result_chirho = true;
                        }
                    } else if saw_family_chirho
                        && !saw_where_chirho
                        && !saw_double_colon_chirho
                        && !saw_injectivity_result_chirho
                    {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if kind_chirho == TokenKindChirho::AtSignChirho {
                            // A declaration binder such as `@kind` names an
                            // inferred kind variable; it is not an additional
                            // visible family argument.
                            skip_visible_binder_chirho = true;
                        } else if matches!(
                            kind_chirho,
                            TokenKindChirho::ConIdChirho
                                | TokenKindChirho::ConSymChirho
                                | TokenKindChirho::VarSymChirho
                                | TokenKindChirho::QualifiedConSymChirho
                                | TokenKindChirho::QualifiedVarSymChirho
                        ) && name_chirho.is_none()
                        {
                            name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        } else if kind_chirho == TokenKindChirho::VarIdChirho {
                            if skip_visible_binder_chirho {
                                skip_visible_binder_chirho = false;
                            } else {
                                type_vars_chirho
                                    .push(self.name_from_token_chirho(tok_chirho, s_chirho).into());
                            }
                        } else if kind_chirho == TokenKindChirho::LeftParenChirho
                            && let Some((tv_chirho, skip_chirho)) = self
                                .try_parse_kind_annotated_tyvar_chirho(&children_chirho, idx_chirho)
                                .or_else(|| {
                                    self.unparsed_kind_binder_chirho(&children_chirho, idx_chirho)
                                })
                        {
                            // `@(k :: K)` names an inferred kind like `@k` does: the
                            // whole group is skipped, never a visible argument (T22560d).
                            if !std::mem::take(&mut skip_visible_binder_chirho) {
                                type_vars_chirho.push(tv_chirho);
                            }
                            idx_chirho += skip_chirho;
                            continue;
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if saw_double_colon_chirho && !saw_where_chirho && result_kind_chirho.is_none()
                    {
                        result_kind_chirho =
                            Some(self.lower_type_chirho(n_chirho, child_chirho.start_chirho));
                        saw_double_colon_chirho = false; // consumed
                    } else if saw_where_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::TypeFamilyEquationChirho
                    {
                        equations_chirho.push(self.lower_family_equation_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                        ));
                    }
                }
            }
            idx_chirho += 1;
        }

        DeclChirho::TypeFamilyDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            result_kind_chirho,
            data_family_chirho,
            standalone_kind_chirho: None,
            closed_chirho: saw_where_chirho,
            equations_chirho,
            span_chirho,
        }
    }

    /// A header group `( v :: kind )` whose kind the binder kind grammar does not parse,
    /// such as `(e :: Either a b)`: the binder `v`, unannotated as before, and the group's
    /// token count through its balanced `)`. Skipping the whole group keeps its `::` from
    /// being read as the result-kind marker, so the binders after it survive (row 686, H01).
    /// `None` unless the group opens with a binder name and `::`.
    /// workflow: compiler-pipeline-chirho/declaration-validity-chirho
    fn unparsed_kind_binder_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        idx_chirho: usize,
    ) -> Option<(TyVarChirho, usize)> {
        let binder_child_chirho = children_chirho.get(idx_chirho + 1)?;
        let GreenElementChirho::TokenChirho(binder_token_chirho) =
            binder_child_chirho.element_chirho
        else {
            return None;
        };
        if !Self::is_head_binder_name_chirho(binder_token_chirho.kind_chirho()) {
            return None;
        }
        let GreenElementChirho::TokenChirho(colons_chirho) =
            children_chirho.get(idx_chirho + 2)?.element_chirho
        else {
            return None;
        };
        if colons_chirho.kind_chirho() != TokenKindChirho::DoubleColonChirho {
            return None;
        }
        let mut depth_chirho = 0usize;
        for (offset_chirho, child_chirho) in children_chirho[idx_chirho..].iter().enumerate() {
            let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho else {
                continue;
            };
            match token_chirho.kind_chirho() {
                TokenKindChirho::LeftParenChirho => depth_chirho += 1,
                TokenKindChirho::RightParenChirho => {
                    depth_chirho -= 1;
                    if depth_chirho == 0 {
                        let span_chirho = self.span_chirho(
                            binder_child_chirho.start_chirho,
                            binder_child_chirho.end_chirho,
                        );
                        let binder_chirho =
                            self.name_from_token_chirho(binder_token_chirho, span_chirho);
                        return Some((binder_chirho.into(), offset_chirho + 1));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub(super) fn lower_type_family_instance_decl_chirho(
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
        let mut inst_paren_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();

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
                            if inst_paren_depth_chirho > 0 {
                                inst_paren_tokens_chirho.push((tok_chirho, s_chirho));
                            }
                            inst_paren_depth_chirho += 1;
                        } else if kind_chirho == TokenKindChirho::RightParenChirho {
                            if inst_paren_depth_chirho > 0 {
                                inst_paren_depth_chirho -= 1;
                                if inst_paren_depth_chirho > 0 {
                                    inst_paren_tokens_chirho.push((tok_chirho, s_chirho));
                                } else if family_name_chirho.is_none()
                                    && lhs_types_chirho.is_empty()
                                    && let [(operator_chirho, operator_span_chirho)] =
                                        inst_paren_tokens_chirho.as_slice()
                                    && matches!(
                                        operator_chirho.kind_chirho(),
                                        TokenKindChirho::ConSymChirho
                                            | TokenKindChirho::VarSymChirho
                                            | TokenKindChirho::QualifiedConSymChirho
                                            | TokenKindChirho::QualifiedVarSymChirho
                                    )
                                {
                                    // `type instance (~>) = (->)`: a parenthesized operator
                                    // before any argument names the family (row 686, T14010).
                                    family_name_chirho = Some(self.name_from_token_chirho(
                                        operator_chirho,
                                        *operator_span_chirho,
                                    ));
                                    inst_paren_tokens_chirho.clear();
                                } else if !inst_paren_tokens_chirho.is_empty() {
                                    let paren_span_chirho = inst_paren_tokens_chirho
                                        .first()
                                        .and_then(|(_, first_span_chirho)| {
                                            first_span_chirho.merge_chirho(s_chirho)
                                        })
                                        .unwrap_or(s_chirho);
                                    let paren_ty_chirho = self.lower_type_from_token_slice_chirho(
                                        &inst_paren_tokens_chirho,
                                        paren_span_chirho,
                                    );
                                    lhs_types_chirho.push(paren_ty_chirho);
                                    inst_paren_tokens_chirho.clear();
                                }
                            }
                        } else if inst_paren_depth_chirho > 0 {
                            inst_paren_tokens_chirho.push((tok_chirho, s_chirho));
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
                        } else if family_name_chirho.is_some()
                            && kind_chirho == TokenKindChirho::UnderscoreReservedIdChirho
                        {
                            // A wildcard argument keeps its place (row 686, V06).
                            lhs_types_chirho.push(TypeChirho::WildcardChirho {
                                span_chirho: s_chirho,
                            });
                        }
                    }
                }
                // An instance body's equation, in either spelling, is parsed like a
                // closed family's, so its written arguments keep their order and shape.
                GreenElementChirho::NodeChirho(n_chirho)
                    if n_chirho.kind_chirho() == SyntaxKindChirho::TypeFamilyEquationChirho =>
                {
                    let equation_chirho = self.lower_family_equation_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                    );
                    family_name_chirho = equation_chirho.head_chirho;
                    lhs_types_chirho = equation_chirho.lhs_types_chirho;
                    rhs_chirho = Some(equation_chirho.rhs_chirho);
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
            kind_args_chirho: Vec::new(),
            family_name_chirho: family_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            lhs_types_chirho,
            rhs_chirho: rhs_chirho.unwrap_or_else(|| self.placeholder_type_chirho()),
            span_chirho,
        }
    }
}
