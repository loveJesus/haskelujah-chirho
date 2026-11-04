// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Types and kinds, and kind-annotated binders.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::TyVarChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho, flat_types_chirho, is_type_kind_chirho};

impl LowerCtxChirho {
    // -----------------------------------------------------------------------
    // Types
    // -----------------------------------------------------------------------

    pub(super) fn lower_type_chirho(
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

                let mult_chirho = self.lower_arrow_multiplicity_chirho(&children_chirho);

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
                let mut kind_argument_chirho = false;
                let mut lowered_types_chirho = Vec::new();
                for child_chirho in &children_chirho {
                    match child_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(token_chirho)
                            if token_chirho.kind_chirho() == TokenKindChirho::AtSignChirho =>
                        {
                            kind_argument_chirho = true;
                        }
                        GreenElementChirho::NodeChirho(node_chirho)
                            if is_type_kind_chirho(node_chirho.kind_chirho()) =>
                        {
                            lowered_types_chirho.push((
                                self.lower_type_from_child_chirho(child_chirho),
                                kind_argument_chirho,
                            ));
                            kind_argument_chirho = false;
                        }
                        _ => {}
                    }
                }
                if kind_argument_chirho {
                    lowered_types_chirho.push((self.placeholder_type_chirho(), true));
                }
                if let [
                    (left_chirho, false),
                    (op_chirho, false),
                    (right_chirho, false),
                ] = lowered_types_chirho.as_slice()
                    && is_qualified_symbol_type_operator_chirho(op_chirho)
                {
                    return TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(op_chirho.clone()),
                            arg_chirho: Box::new(left_chirho.clone()),
                            span_chirho,
                        }),
                        arg_chirho: Box::new(right_chirho.clone()),
                        span_chirho,
                    };
                }
                let mut type_iter_chirho = lowered_types_chirho.into_iter();
                let mut result_chirho = type_iter_chirho
                    .next()
                    .map(|(ty_chirho, _)| ty_chirho)
                    .unwrap_or_else(|| self.placeholder_type_chirho());
                for (arg_chirho, kind_argument_chirho) in type_iter_chirho {
                    result_chirho = if kind_argument_chirho {
                        TypeChirho::KindApplicationChirho {
                            fun_chirho: Box::new(result_chirho),
                            arg_chirho: Box::new(arg_chirho),
                            written_chirho: true,
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
            SyntaxKindChirho::KindAnnotTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                self.type_from_flat_children_chirho(
                    &children_chirho.iter().collect::<Vec<_>>(),
                    span_chirho,
                )
            }
            SyntaxKindChirho::ParenTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

                // Detect a top-level kind annotation `(... :: k)`. We keep the
                // full type expression to the left of the top-level `::`, not
                // just the first type node, so partially-applied operators like
                // `((:~~:) (a :: k1) :: k2 -> Type)` preserve their application
                // shape during lowering.
                let top_level_double_colon_index_chirho =
                    children_chirho.iter().position(|c_chirho| {
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

                if let Some(elements_chirho) =
                    self.annotated_tuple_elements_chirho(&children_chirho, span_chirho)
                {
                    TypeChirho::TupleChirho {
                        elements_chirho,
                        span_chirho,
                    }
                } else if let Some(double_colon_index_chirho) = top_level_double_colon_index_chirho
                {
                    let prefix_start_chirho = usize::from(matches!(
                        children_chirho.first().map(|child_chirho| child_chirho.element_chirho),
                        Some(GreenElementChirho::TokenChirho(token_chirho))
                            if token_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho
                    ));
                    let prefix_children_chirho: Vec<_> = children_chirho
                        [prefix_start_chirho..double_colon_index_chirho]
                        .iter()
                        .collect();
                    let inner_chirho =
                        self.type_from_flat_children_chirho(&prefix_children_chirho, span_chirho);
                    let kind_end_chirho = children_chirho.len()
                        - usize::from(
                            matches!(children_chirho.last().map(|child_chirho| child_chirho.element_chirho),
                            Some(GreenElementChirho::TokenChirho(token_chirho))
                                if token_chirho.kind_chirho() == TokenKindChirho::RightParenChirho),
                        );
                    let kind_children_chirho: Vec<_> = children_chirho
                        [double_colon_index_chirho + 1..kind_end_chirho]
                        .iter()
                        .collect();
                    TypeChirho::ParenChirho {
                        inner_chirho: Box::new(TypeChirho::KindAnnotationChirho {
                            ty_chirho: Box::new(inner_chirho),
                            kind_chirho: Box::new(self.type_from_flat_children_chirho(
                                &kind_children_chirho,
                                span_chirho,
                            )),
                            span_chirho,
                        }),
                        span_chirho,
                    }
                } else if type_nodes_chirho.len() > 1 {
                    let has_top_level_comma_chirho = children_chirho.iter().any(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho)
                    });
                    if has_top_level_comma_chirho {
                        let elements_chirho: Vec<_> = type_nodes_chirho
                            .iter()
                            .map(|tc_chirho| self.lower_type_from_child_chirho(tc_chirho))
                            .collect();
                        TypeChirho::TupleChirho {
                            elements_chirho,
                            span_chirho,
                        }
                    } else {
                        let mut inner_chirho =
                            self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                        for tc_chirho in &type_nodes_chirho[1..] {
                            inner_chirho = TypeChirho::AppChirho {
                                fun_chirho: Box::new(inner_chirho),
                                arg_chirho: Box::new(self.lower_type_from_child_chirho(tc_chirho)),
                                span_chirho,
                            };
                        }
                        TypeChirho::ParenChirho {
                            inner_chirho: Box::new(inner_chirho),
                            span_chirho,
                        }
                    }
                } else if type_nodes_chirho.len() == 1 {
                    let inner_chirho = self.lower_type_from_child_chirho(type_nodes_chirho[0]);
                    TypeChirho::ParenChirho {
                        inner_chirho: Box::new(inner_chirho),
                        span_chirho,
                    }
                } else {
                    let comma_count_chirho = children_chirho
                        .iter()
                        .filter(|c_chirho| {
                            matches!(c_chirho.element_chirho, GreenElementChirho::TokenChirho(tok_chirho)
                                if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho)
                        })
                        .count();
                    if comma_count_chirho > 0 {
                        let tuple_name_chirho = format!("({})", ",".repeat(comma_count_chirho));
                        return TypeChirho::ConChirho(NameChirho::RawChirho(
                            RawNameChirho::unqualified_chirho(&tuple_name_chirho, span_chirho),
                        ));
                    }
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
                flat_types_chirho::list_type_chirho(
                    type_node_chirho
                        .map(|child_chirho| self.lower_type_from_child_chirho(child_chirho)),
                    span_chirho,
                )
            }
            SyntaxKindChirho::QualTypeChirho => {
                let parts_chirho = self.lower_context_parts_chirho(node_chirho, base_chirho);
                let body_chirho = parts_chirho
                    .body_chirho
                    .map(|body_chirho| {
                        self.lower_type_chirho(body_chirho.node_chirho, body_chirho.base_chirho)
                    })
                    .unwrap_or_else(|| self.placeholder_type_chirho());
                let mut context_chirho = parts_chirho.constraints_chirho;
                if context_chirho.is_empty() {
                    body_chirho
                } else if let TypeChirho::QualChirho {
                    context_chirho: nested_context_chirho,
                    body_chirho: nested_body_chirho,
                    ..
                } = body_chirho
                {
                    context_chirho.extend(nested_context_chirho);
                    TypeChirho::QualChirho {
                        context_chirho,
                        body_chirho: nested_body_chirho,
                        span_chirho,
                    }
                } else {
                    TypeChirho::QualChirho {
                        context_chirho,
                        body_chirho: Box::new(body_chirho),
                        span_chirho,
                    }
                }
            }
            SyntaxKindChirho::ForallTypeChirho => {
                let parts_chirho = self.lower_forall_parts_chirho(node_chirho, base_chirho);
                let body_chirho = Box::new(
                    parts_chirho
                        .body_chirho
                        .map(|body_chirho| {
                            self.lower_type_chirho(body_chirho.node_chirho, body_chirho.base_chirho)
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
            SyntaxKindChirho::PromotedConTypeChirho => {
                // DataKinds promoted constructor: 'True, 'Just, ':, etc.
                // Children: Tick token, ConId/ConSym token
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let con_child_chirho = children_chirho.iter().find(|c_chirho| {
                    matches!(c_chirho.element_chirho, GreenElementChirho::TokenChirho(t_chirho)
                        if t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                           || t_chirho.kind_chirho() == TokenKindChirho::QualifiedConIdChirho
                           || t_chirho.kind_chirho() == TokenKindChirho::ConSymChirho
                           || t_chirho.kind_chirho() == TokenKindChirho::QualifiedConSymChirho)
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
                    // The unit '() and the tuple constructors '(,), '(,,), …: one comma
                    // per component after the first.
                    let commas_chirho = children_chirho
                        .iter()
                        .filter(|c_chirho| {
                            matches!(c_chirho.element_chirho, GreenElementChirho::TokenChirho(t_chirho)
                                if t_chirho.kind_chirho() == TokenKindChirho::CommaChirho)
                        })
                        .count();
                    promoted_tuple_constructor_chirho(commas_chirho, span_chirho)
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
            SyntaxKindChirho::PromotedTupleTypeChirho => {
                // DataKinds promoted tuple '(t1, …, tn): the tuple constructor '(,…,)
                // applied to its elements, the shape naming, kinds and the promotion
                // identity already handle (row 686).
                // workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let elements_chirho: Vec<TypeChirho> = children_chirho
                    .iter()
                    .filter(|c_chirho| {
                        matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho)
                            if is_type_kind_chirho(n_chirho.kind_chirho()))
                    })
                    .map(|tc_chirho| self.lower_type_from_child_chirho(tc_chirho))
                    .collect();
                if elements_chirho.len() < 2 {
                    // `'(t)` names no promoted tuple: the element stands alone.
                    elements_chirho
                        .into_iter()
                        .next()
                        .unwrap_or_else(|| self.placeholder_type_chirho())
                } else {
                    let constructor_chirho =
                        promoted_tuple_constructor_chirho(elements_chirho.len() - 1, span_chirho);
                    elements_chirho.into_iter().fold(
                        constructor_chirho,
                        |fun_chirho, arg_chirho| TypeChirho::AppChirho {
                            fun_chirho: Box::new(fun_chirho),
                            arg_chirho: Box::new(arg_chirho),
                            span_chirho,
                        },
                    )
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
                self.lower_infix_type_chirho(node_chirho, base_chirho, span_chirho)
            }
            _ => {
                // Fallback: try to extract a name
                let name_chirho = self.extract_name_from_node_chirho(node_chirho, base_chirho);
                if name_chirho
                    .text_chirho()
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_uppercase())
                {
                    TypeChirho::ConChirho(name_chirho)
                } else {
                    TypeChirho::VarChirho(name_chirho)
                }
            }
        }
    }

    pub(super) fn lower_type_from_child_chirho(
        &self,
        child_chirho: &ChildChirho<'_>,
    ) -> TypeChirho {
        if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
            self.lower_type_chirho(n_chirho, child_chirho.start_chirho)
        } else {
            self.placeholder_type_chirho()
        }
    }

    /// Try to parse a kind-annotated type variable from a parenthesized group:
    /// `( VarId :: Kind )`, retaining the complete type-syntax kind.
    ///
    /// `idx_chirho` is the index of the `(` token in `children_chirho`.
    /// Returns `Some((TyVarChirho, tokens_consumed))` on success, or `None` if
    /// the pattern doesn't match.
    pub(super) fn try_parse_kind_annotated_tyvar_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        idx_chirho: usize,
    ) -> Option<(TyVarChirho, usize)> {
        // Need at least ( VarId :: * ) = 5 tokens
        if idx_chirho + 4 >= children_chirho.len() {
            return None;
        }

        if !matches!(children_chirho[idx_chirho].element_chirho,
            GreenElementChirho::TokenChirho(token_chirho)
                if token_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho)
        {
            return None;
        }

        // The first child inside the balanced group is the written binder.
        // children_chirho[idx_chirho+1] must be VarId
        let var_child_chirho = &children_chirho[idx_chirho + 1];
        let var_tok_chirho = match var_child_chirho.element_chirho {
            GreenElementChirho::TokenChirho(t_chirho)
                if Self::is_head_binder_name_chirho(t_chirho.kind_chirho()) =>
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

        // Parse the kind expression starting at idx_chirho+3, stop at `)`.
        let kind_start_chirho = idx_chirho + 3;
        let mut close_idx_chirho = kind_start_chirho;
        let mut depth_chirho = 0usize;
        while close_idx_chirho < children_chirho.len() {
            match children_chirho[close_idx_chirho].element_chirho {
                GreenElementChirho::TokenChirho(t_chirho)
                    if t_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho =>
                {
                    depth_chirho += 1;
                }
                GreenElementChirho::TokenChirho(t_chirho)
                    if t_chirho.kind_chirho() == TokenKindChirho::RightParenChirho =>
                {
                    if depth_chirho == 0 {
                        break;
                    }
                    depth_chirho -= 1;
                }
                _ => {}
            }
            close_idx_chirho += 1;
        }
        if close_idx_chirho >= children_chirho.len() {
            return None;
        }

        let kind_children_chirho: Vec<_> = children_chirho[kind_start_chirho..close_idx_chirho]
            .iter()
            .collect();
        if kind_children_chirho.is_empty() {
            return None;
        }
        let kind_span_chirho = self.span_chirho(
            children_chirho[kind_start_chirho].start_chirho,
            children_chirho[close_idx_chirho - 1].end_chirho,
        );
        let kind_chirho =
            self.type_from_flat_children_chirho(&kind_children_chirho, kind_span_chirho);

        // children_chirho[close_idx_chirho] must be `)`
        match children_chirho[close_idx_chirho].element_chirho {
            GreenElementChirho::TokenChirho(t_chirho)
                if t_chirho.kind_chirho() == TokenKindChirho::RightParenChirho => {}
            _ => return None,
        }

        let s_chirho = self.span_chirho(var_child_chirho.start_chirho, var_child_chirho.end_chirho);
        let name_chirho = self.name_from_token_chirho(var_tok_chirho, s_chirho);
        let tv_chirho = TyVarChirho::annotated_chirho(name_chirho, kind_chirho);

        // Total tokens consumed: from `(` through `)` inclusive
        let consumed_chirho = close_idx_chirho - idx_chirho + 1;
        Some((tv_chirho, consumed_chirho))
    }

    /// Try to lower a `ParenTypeChirho` node as a kind-annotated forall binder:
    /// `(a :: k)` where the node contains VarType + `::` + kind-type children.
    /// Returns `Some(TyVarChirho)` with kind annotation if the pattern matches.
    pub(super) fn try_lower_kind_annotated_forall_binder_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<TyVarChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        let annotation_index_chirho = children_chirho.iter().position(|child_chirho| {
            matches!(child_chirho.element_chirho,
                GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho)
        });
        let binder_end_chirho = annotation_index_chirho.unwrap_or(children_chirho.len());
        let var_name_chirho = children_chirho[..binder_end_chirho].iter().find_map(
            |child_chirho| match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(token_chirho)
                    if Self::is_head_binder_name_chirho(token_chirho.kind_chirho()) =>
                {
                    Some(self.name_from_token_chirho(
                        token_chirho,
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                    ))
                }
                GreenElementChirho::NodeChirho(node_chirho)
                    if node_chirho.kind_chirho() == SyntaxKindChirho::VarTypeChirho =>
                {
                    Some(self.extract_name_from_node_chirho(node_chirho, child_chirho.start_chirho))
                }
                _ => None,
            },
        )?;
        let Some(annotation_index_chirho) = annotation_index_chirho else {
            return Some(TyVarChirho::plain_chirho(var_name_chirho));
        };
        let kind_end_chirho = children_chirho.len()
            - usize::from(
                matches!(children_chirho.last().map(|child_chirho| child_chirho.element_chirho),
                Some(GreenElementChirho::TokenChirho(token_chirho))
                    if token_chirho.kind_chirho() == TokenKindChirho::RightParenChirho),
            );
        let kind_children_chirho: Vec<_> = children_chirho
            [annotation_index_chirho + 1..kind_end_chirho]
            .iter()
            .collect();
        let kind_span_chirho = self.span_chirho(
            kind_children_chirho.first()?.start_chirho,
            kind_children_chirho.last()?.end_chirho,
        );
        Some(TyVarChirho::annotated_chirho(
            var_name_chirho,
            self.type_from_flat_children_chirho(&kind_children_chirho, kind_span_chirho),
        ))
    }
}

fn is_qualified_symbol_type_operator_chirho(ty_chirho: &TypeChirho) -> bool {
    let name_chirho = match ty_chirho {
        TypeChirho::ConChirho(name_chirho) | TypeChirho::VarChirho(name_chirho) => name_chirho,
        _ => return false,
    };
    let text_chirho = name_chirho.text_chirho();
    name_chirho.full_name_chirho() != text_chirho
        && !text_chirho.is_empty()
        && text_chirho
            .chars()
            .all(|char_chirho| !char_chirho.is_alphanumeric() && char_chirho != '_')
}

/// The promoted unit `'()` (no commas) or tuple constructor `'(,…,)` with `commas_chirho`
/// commas, as a promoted constructor name.
fn promoted_tuple_constructor_chirho(commas_chirho: usize, span_chirho: SpanChirho) -> TypeChirho {
    TypeChirho::PromotedConChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            format!("({})", ",".repeat(commas_chirho)),
            span_chirho,
        )),
        span_chirho,
    }
}
