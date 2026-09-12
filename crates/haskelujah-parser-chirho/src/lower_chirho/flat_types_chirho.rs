// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Flat type syntax used by record fields and declaration heads.
//! workflow: language-features-chirho/flat-type-syntax-chirho

use haskelujah_ast_chirho::decl_chirho::{TyVarChirho, TyVarSpecificityChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho, is_type_kind_chirho, type_operators_chirho};

#[derive(Default)]
struct TypeAtomsChirho {
    values_chirho: Vec<(TypeChirho, bool)>,
    pending_kind_chirho: bool,
}

impl TypeAtomsChirho {
    fn push_chirho(&mut self, ty_chirho: TypeChirho) {
        self.values_chirho
            .push((ty_chirho, std::mem::take(&mut self.pending_kind_chirho)));
    }

    fn finish_chirho(self, span_chirho: SpanChirho) -> Option<TypeChirho> {
        if let [
            (left_chirho, false),
            (operator_chirho, false),
            (right_chirho, false),
        ] = self.values_chirho.as_slice()
            && super::is_qualified_symbol_type_operator_chirho(operator_chirho)
        {
            return Some(type_operators_chirho::binary_type_application_chirho(
                operator_chirho.clone(),
                left_chirho.clone(),
                right_chirho.clone(),
                span_chirho,
            ));
        }
        let mut values_chirho = self.values_chirho.into_iter();
        let (mut result_chirho, _) = values_chirho.next()?;
        for (arg_chirho, invisible_chirho) in values_chirho {
            let span_chirho = result_chirho
                .span_chirho()
                .merge_chirho(arg_chirho.span_chirho())
                .unwrap_or(span_chirho);
            result_chirho = if invisible_chirho {
                TypeChirho::KindAppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(arg_chirho),
                    span_chirho,
                }
            } else {
                TypeChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(arg_chirho),
                    span_chirho,
                }
            };
        }
        Some(result_chirho)
    }
}

/// Empty brackets name the constructor; a present element applies it. Shared
/// by structured and flat lowering, so one spelling cannot gain a fake argument.
pub(super) fn list_type_chirho(
    element_chirho: Option<TypeChirho>,
    span_chirho: SpanChirho,
) -> TypeChirho {
    match element_chirho {
        Some(element_chirho) => TypeChirho::ListChirho {
            element_chirho: Box::new(element_chirho),
            span_chirho,
        },
        None => TypeChirho::ConChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "[]",
            span_chirho,
        ))),
    }
}

impl LowerCtxChirho {
    /// Structured signatures and flat fields share the argument-visibility rule.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn lower_type_application_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> TypeChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        self.collect_type_atoms_chirho(&children_chirho.iter().collect::<Vec<_>>(), span_chirho)
            .finish_chirho(span_chirho)
            .unwrap_or_else(|| self.placeholder_type_chirho())
    }

    fn forall_from_flat_children_chirho(
        &self,
        children_chirho: &[&ChildChirho],
        span_chirho: SpanChirho,
    ) -> Option<TypeChirho> {
        if !matches!(children_chirho.first()?.element_chirho,
            GreenElementChirho::TokenChirho(token_chirho)
                if token_chirho.kind_chirho() == TokenKindChirho::ForallKeywordChirho)
        {
            return None;
        }
        let mut depth_chirho = 0usize;
        let delimiter_chirho = children_chirho.iter().enumerate().skip(1).find_map(
            |(index_chirho, child_chirho)| {
                let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho
                else {
                    return None;
                };
                match token_chirho.kind_chirho() {
                    TokenKindChirho::LeftParenChirho
                    | TokenKindChirho::LeftBraceChirho
                    | TokenKindChirho::LeftBracketChirho => depth_chirho += 1,
                    TokenKindChirho::RightParenChirho
                    | TokenKindChirho::RightBraceChirho
                    | TokenKindChirho::RightBracketChirho => {
                        depth_chirho = depth_chirho.saturating_sub(1)
                    }
                    TokenKindChirho::RightArrowChirho if depth_chirho == 0 => {
                        return Some((index_chirho, true));
                    }
                    TokenKindChirho::VarSymChirho
                        if depth_chirho == 0 && token_chirho.text_chirho() == "." =>
                    {
                        return Some((index_chirho, false));
                    }
                    _ => {}
                }
                None
            },
        )?;
        let binder_children_chirho: Vec<_> = children_chirho[1..delimiter_chirho.0]
            .iter()
            .map(|child_chirho| ChildChirho {
                element_chirho: child_chirho.element_chirho,
                start_chirho: child_chirho.start_chirho,
                end_chirho: child_chirho.end_chirho,
            })
            .collect();
        let mut vars_chirho = Vec::new();
        let mut index_chirho = 0;
        let mut inferred_chirho = false;
        while index_chirho < binder_children_chirho.len() {
            if let Some((mut binder_chirho, consumed_chirho)) =
                self.try_parse_kind_annotated_tyvar_chirho(&binder_children_chirho, index_chirho)
            {
                if inferred_chirho {
                    binder_chirho.specificity_chirho = TyVarSpecificityChirho::InferredChirho;
                }
                vars_chirho.push(binder_chirho);
                index_chirho += consumed_chirho;
                continue;
            }
            let child_chirho = &binder_children_chirho[index_chirho];
            if let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho {
                match token_chirho.kind_chirho() {
                    TokenKindChirho::LeftBraceChirho => inferred_chirho = true,
                    TokenKindChirho::RightBraceChirho => inferred_chirho = false,
                    TokenKindChirho::VarIdChirho => {
                        let mut binder_chirho =
                            TyVarChirho::plain_chirho(self.name_from_token_chirho(
                                token_chirho,
                                self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                ),
                            ));
                        if inferred_chirho {
                            binder_chirho.specificity_chirho =
                                TyVarSpecificityChirho::InferredChirho;
                        }
                        vars_chirho.push(binder_chirho);
                    }
                    _ => {}
                }
            }
            index_chirho += 1;
        }
        let body_chirho = Box::new(self.type_from_flat_children_chirho(
            &children_chirho[delimiter_chirho.0 + 1..],
            span_chirho,
        ));
        Some(if delimiter_chirho.1 {
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            }
        } else {
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            }
        })
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
    ///   - `forall a -> type` → required forall
    pub(super) fn type_from_flat_children_chirho(
        &self,
        children_chirho: &[&ChildChirho],
        fallback_span_chirho: SpanChirho,
    ) -> TypeChirho {
        if children_chirho.is_empty() {
            return self.placeholder_type_chirho();
        }

        // A leading forall scopes over arrows and constraints in its body.
        if let Some(forall_chirho) =
            self.forall_from_flat_children_chirho(children_chirho, fallback_span_chirho)
        {
            return forall_chirho;
        }

        // Split on top-level `=>` (qualified type with context).
        let double_arrow_idx_chirho =
            self.find_top_level_token_chirho(children_chirho, TokenKindChirho::DoubleArrowChirho);
        if let Some(idx_chirho) = double_arrow_idx_chirho {
            let lhs_chirho = &children_chirho[..idx_chirho];
            let rhs_chirho = &children_chirho[idx_chirho + 1..];
            let context_ty_chirho =
                self.type_from_flat_children_chirho(lhs_chirho, fallback_span_chirho);
            let body_chirho = self.type_from_flat_children_chirho(rhs_chirho, fallback_span_chirho);
            let mut context_chirho =
                Self::type_to_constraints_chirho(&context_ty_chirho, fallback_span_chirho);

            if context_chirho.is_empty() {
                return body_chirho;
            }

            if let TypeChirho::QualChirho {
                context_chirho: nested_context_chirho,
                body_chirho: nested_body_chirho,
                ..
            } = body_chirho
            {
                context_chirho.extend(nested_context_chirho);
                return TypeChirho::QualChirho {
                    context_chirho,
                    body_chirho: nested_body_chirho,
                    span_chirho: fallback_span_chirho,
                };
            }

            return TypeChirho::QualChirho {
                context_chirho,
                body_chirho: Box::new(body_chirho),
                span_chirho: fallback_span_chirho,
            };
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

        // Some declaration heads retain a kind signature as a flat token
        // sequence instead of an `InfixTypeChirho` CST node. Preserve the
        // operator as the application head: `left ~> right` means
        // `(~>) left right`, not `left right` with `~>` discarded.
        if let Some((left_end_chirho, right_start_chirho, operator_chirho)) =
            self.find_top_level_type_operator_chirho(children_chirho)
        {
            let left_chirho = self.type_from_flat_children_chirho(
                &children_chirho[..left_end_chirho],
                fallback_span_chirho,
            );
            let right_chirho = self.type_from_flat_children_chirho(
                &children_chirho[right_start_chirho..],
                fallback_span_chirho,
            );
            return type_operators_chirho::binary_type_application_chirho(
                TypeChirho::ConChirho(operator_chirho),
                left_chirho,
                right_chirho,
                fallback_span_chirho,
            );
        }

        // Build application chain from atom types
        self.collect_type_atoms_chirho(children_chirho, fallback_span_chirho)
            .finish_chirho(fallback_span_chirho)
            .unwrap_or_else(|| self.placeholder_type_chirho())
    }

    /// Find a top-level `->` token (not inside parens/brackets).
    fn find_top_level_arrow_chirho(&self, children_chirho: &[&ChildChirho]) -> Option<usize> {
        self.find_top_level_token_chirho(children_chirho, TokenKindChirho::RightArrowChirho)
    }

    fn find_top_level_type_operator_chirho(
        &self,
        children_chirho: &[&ChildChirho],
    ) -> Option<(usize, usize, NameChirho)> {
        let mut depth_chirho = 0i32;
        let mut index_chirho = 0usize;
        while index_chirho < children_chirho.len() {
            let child_chirho = children_chirho[index_chirho];
            let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho else {
                index_chirho += 1;
                continue;
            };
            match token_chirho.kind_chirho() {
                TokenKindChirho::LeftParenChirho | TokenKindChirho::LeftBracketChirho => {
                    depth_chirho += 1;
                }
                TokenKindChirho::RightParenChirho | TokenKindChirho::RightBracketChirho => {
                    depth_chirho -= 1;
                }
                TokenKindChirho::VarSymChirho
                | TokenKindChirho::ConSymChirho
                | TokenKindChirho::QualifiedVarSymChirho
                | TokenKindChirho::QualifiedConSymChirho
                | TokenKindChirho::TildeChirho
                    if depth_chirho == 0
                        && index_chirho > 0
                        && index_chirho + 1 < children_chirho.len()
                        && !matches!(
                            token_chirho.text_chirho(),
                            "." | "`" | "!" | "%" | "@" | "|" | "->" | "=>"
                        ) =>
                {
                    let span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    return Some((
                        index_chirho,
                        index_chirho + 1,
                        self.name_from_token_chirho(token_chirho, span_chirho),
                    ));
                }
                _ => {}
            }
            index_chirho += 1;
        }
        None
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
    ) -> TypeAtomsChirho {
        let mut atoms_chirho = TypeAtomsChirho::default();
        let mut i_chirho = 0;
        while i_chirho < children_chirho.len() {
            let child_chirho = children_chirho[i_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    match tok_chirho.kind_chirho() {
                        TokenKindChirho::VarIdChirho | TokenKindChirho::QualifiedVarIdChirho => {
                            atoms_chirho.push_chirho(TypeChirho::VarChirho(
                                self.name_from_token_chirho(tok_chirho, s_chirho),
                            ));
                        }
                        TokenKindChirho::VarSymChirho
                        | TokenKindChirho::ConSymChirho
                        | TokenKindChirho::QualifiedVarSymChirho
                        | TokenKindChirho::QualifiedConSymChirho
                        | TokenKindChirho::TildeChirho
                            if !matches!(
                                tok_chirho.text_chirho(),
                                "." | "!" | "%" | "@" | "|" | "->" | "=>"
                            ) =>
                        {
                            atoms_chirho.push_chirho(TypeChirho::ConChirho(
                                self.name_from_token_chirho(tok_chirho, s_chirho),
                            ));
                        }
                        TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                            atoms_chirho.push_chirho(TypeChirho::ConChirho(
                                self.name_from_token_chirho(tok_chirho, s_chirho),
                            ));
                        }
                        TokenKindChirho::AtSignChirho => {
                            atoms_chirho.pending_kind_chirho = true;
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
                                atoms_chirho.push_chirho(TypeChirho::TupleChirho {
                                    elements_chirho,
                                    span_chirho: fallback_span_chirho,
                                });
                            } else if inner_chirho.is_empty() {
                                // Unit type ()
                                atoms_chirho.push_chirho(TypeChirho::TupleChirho {
                                    elements_chirho: vec![],
                                    span_chirho: fallback_span_chirho,
                                });
                            } else {
                                // Parenthesized type
                                atoms_chirho.push_chirho(TypeChirho::ParenChirho {
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
                            // Keep the complete balanced group: an inner `]`
                            // belongs to its own list, not to this outer one.
                            let start_chirho = i_chirho;
                            i_chirho += 1;
                            let inner_start_chirho = i_chirho;
                            let mut depth_chirho = 1usize;
                            while i_chirho < children_chirho.len() {
                                if let GreenElementChirho::TokenChirho(t_chirho) =
                                    children_chirho[i_chirho].element_chirho
                                {
                                    match t_chirho.kind_chirho() {
                                        TokenKindChirho::LeftBracketChirho => depth_chirho += 1,
                                        TokenKindChirho::RightBracketChirho => {
                                            depth_chirho -= 1;
                                            if depth_chirho == 0 {
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                i_chirho += 1;
                            }
                            let inner_chirho = &children_chirho[inner_start_chirho..i_chirho];
                            let span_chirho = self.span_chirho(
                                children_chirho[start_chirho].start_chirho,
                                children_chirho
                                    .get(i_chirho)
                                    .or_else(|| children_chirho.last())
                                    .expect("the opening bracket is present")
                                    .end_chirho,
                            );
                            let element_chirho = (!inner_chirho.is_empty()).then(|| {
                                self.type_from_flat_children_chirho(inner_chirho, span_chirho)
                            });
                            atoms_chirho.push_chirho(list_type_chirho(element_chirho, span_chirho));
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
                        atoms_chirho.push_chirho(
                            self.lower_type_chirho(n_chirho, child_chirho.start_chirho),
                        );
                    }
                }
            }
            i_chirho += 1;
        }
        atoms_chirho
    }
}
