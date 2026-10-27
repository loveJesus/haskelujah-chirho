// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type synonyms, type families and their instances.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TypeFamilyEquationChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::LowerCtxChirho;

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
        let mut saw_where_chirho = false;
        let mut saw_double_colon_chirho = false;
        let mut saw_injectivity_result_chirho = false;
        let mut skip_visible_binder_chirho = false;

        // Collect equation tokens: group tokens between ; markers
        let mut eq_lhs_types_chirho: Vec<TypeChirho> = Vec::new();
        let mut eq_rhs_chirho: Option<TypeChirho> = None;
        let mut eq_saw_equals_chirho = false;
        let mut eq_saw_family_name_chirho = false; // skip first ConId in each equation (family name)
        let mut eq_paren_depth_chirho: usize = 0; // parenthesized group depth
        let mut eq_paren_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();

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
                            span_chirho,
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
                        } else if saw_family_chirho {
                            // An open family's injectivity annotation starts
                            // with `= result | result -> parameter`. None of
                            // those names are additional family parameters.
                            saw_injectivity_result_chirho = true;
                        }
                    } else if kind_chirho == TokenKindChirho::VirtualSemicolonChirho
                        || kind_chirho == TokenKindChirho::VirtualRightBraceChirho
                    {
                        // Finish current equation if we had one
                        if eq_saw_equals_chirho && let Some(rhs_chirho) = eq_rhs_chirho.take() {
                            equations_chirho.push(TypeFamilyEquationChirho {
                                lhs_types_chirho: std::mem::take(&mut eq_lhs_types_chirho),
                                rhs_chirho,
                                span_chirho,
                            });
                        }
                        eq_lhs_types_chirho.clear();
                        eq_rhs_chirho = None;
                        eq_saw_equals_chirho = false;
                        eq_saw_family_name_chirho = false;
                        eq_paren_depth_chirho = 0;
                        eq_paren_tokens_chirho.clear();
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
                        {
                            type_vars_chirho.push(tv_chirho);
                            idx_chirho += skip_chirho;
                            continue;
                        }
                    } else if saw_where_chirho && !eq_saw_equals_chirho {
                        // LHS type tokens in equation — flat token form.
                        // Handle parenthesized groups by collecting types until
                        // the matching `)` and combining as applications.
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if kind_chirho == TokenKindChirho::LeftParenChirho {
                            if eq_paren_depth_chirho > 0 {
                                eq_paren_tokens_chirho.push((tok_chirho, s_chirho));
                            }
                            eq_paren_depth_chirho += 1;
                        } else if kind_chirho == TokenKindChirho::RightParenChirho {
                            if eq_paren_depth_chirho > 0 {
                                eq_paren_depth_chirho -= 1;
                                if eq_paren_depth_chirho > 0 {
                                    eq_paren_tokens_chirho.push((tok_chirho, s_chirho));
                                } else if !eq_paren_tokens_chirho.is_empty() {
                                    let paren_span_chirho = eq_paren_tokens_chirho
                                        .first()
                                        .and_then(|(_, first_span_chirho)| {
                                            first_span_chirho.merge_chirho(s_chirho)
                                        })
                                        .unwrap_or(s_chirho);
                                    let paren_ty_chirho = self.lower_type_from_token_slice_chirho(
                                        &eq_paren_tokens_chirho,
                                        paren_span_chirho,
                                    );
                                    eq_lhs_types_chirho.push(paren_ty_chirho);
                                    eq_paren_tokens_chirho.clear();
                                }
                            }
                        } else if eq_paren_depth_chirho > 0 {
                            eq_paren_tokens_chirho.push((tok_chirho, s_chirho));
                        } else if matches!(
                            kind_chirho,
                            TokenKindChirho::ConIdChirho
                                | TokenKindChirho::ConSymChirho
                                | TokenKindChirho::VarSymChirho
                                | TokenKindChirho::QualifiedConSymChirho
                                | TokenKindChirho::QualifiedVarSymChirho
                        ) {
                            if !eq_saw_family_name_chirho {
                                // First constructor/operator in an equation is
                                // the family name — skip it.
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
        if eq_saw_equals_chirho && let Some(rhs_chirho) = eq_rhs_chirho.take() {
            equations_chirho.push(TypeFamilyEquationChirho {
                lhs_types_chirho: std::mem::take(&mut eq_lhs_types_chirho),
                rhs_chirho,
                span_chirho,
            });
        }

        DeclChirho::TypeFamilyDeclChirho {
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            result_kind_chirho,
            equations_chirho,
            span_chirho,
        }
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
}
