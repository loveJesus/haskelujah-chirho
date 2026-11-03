// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Types and kinds, and kind-annotated binders.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::{AstKindChirho, TyVarChirho};
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
                let mut skip_visible_type_app_arg_chirho = false;
                let type_nodes_chirho: Vec<_> = children_chirho
                    .iter()
                    .filter(|c_chirho| match c_chirho.element_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::AtSignChirho =>
                        {
                            skip_visible_type_app_arg_chirho = true;
                            false
                        }
                        GreenElementChirho::NodeChirho(n_chirho)
                            if is_type_kind_chirho(n_chirho.kind_chirho()) =>
                        {
                            if skip_visible_type_app_arg_chirho {
                                skip_visible_type_app_arg_chirho = false;
                                false
                            } else {
                                true
                            }
                        }
                        _ => false,
                    })
                    .collect();
                if type_nodes_chirho.is_empty() {
                    return self.placeholder_type_chirho();
                }
                let lowered_types_chirho = type_nodes_chirho
                    .iter()
                    .map(|tc_chirho| self.lower_type_from_child_chirho(tc_chirho))
                    .collect::<Vec<_>>();
                if let [left_chirho, op_chirho, right_chirho] = lowered_types_chirho.as_slice()
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
                    .unwrap_or_else(|| self.placeholder_type_chirho());
                for arg_chirho in type_iter_chirho {
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
            SyntaxKindChirho::KindAnnotTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let mut prefix_children_chirho: Vec<&ChildChirho<'_>> = Vec::new();
                let mut saw_double_colon_chirho = false;

                for child_chirho in &children_chirho {
                    if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho
                        && tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho
                    {
                        saw_double_colon_chirho = true;
                        break;
                    }
                    prefix_children_chirho.push(child_chirho);
                }

                if saw_double_colon_chirho {
                    self.type_from_flat_children_chirho(&prefix_children_chirho, span_chirho)
                } else {
                    self.type_from_flat_children_chirho(
                        &children_chirho.iter().collect::<Vec<_>>(),
                        span_chirho,
                    )
                }
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
                    let prefix_children_chirho = &children_chirho[..double_colon_index_chirho];
                    let prefix_type_nodes_chirho: Vec<_> = prefix_children_chirho
                        .iter()
                        .filter(|c_chirho| {
                            matches!(c_chirho.element_chirho, GreenElementChirho::NodeChirho(n_chirho) if is_type_kind_chirho(n_chirho.kind_chirho()))
                        })
                        .collect();
                    let has_top_level_comma_in_prefix_chirho =
                        prefix_children_chirho.iter().any(|c_chirho| {
                            matches!(c_chirho.element_chirho, GreenElementChirho::TokenChirho(tok_chirho)
                                if tok_chirho.kind_chirho() == TokenKindChirho::CommaChirho)
                        });
                    let inner_chirho = if prefix_type_nodes_chirho.is_empty() {
                        self.placeholder_type_chirho()
                    } else if prefix_type_nodes_chirho.len() == 1 {
                        self.lower_type_from_child_chirho(prefix_type_nodes_chirho[0])
                    } else if has_top_level_comma_in_prefix_chirho {
                        TypeChirho::TupleChirho {
                            elements_chirho: prefix_type_nodes_chirho
                                .iter()
                                .map(|tc_chirho| self.lower_type_from_child_chirho(tc_chirho))
                                .collect(),
                            span_chirho,
                        }
                    } else {
                        let mut combined_chirho =
                            self.lower_type_from_child_chirho(prefix_type_nodes_chirho[0]);
                        for tc_chirho in &prefix_type_nodes_chirho[1..] {
                            combined_chirho = TypeChirho::AppChirho {
                                fun_chirho: Box::new(combined_chirho),
                                arg_chirho: Box::new(self.lower_type_from_child_chirho(tc_chirho)),
                                span_chirho,
                            };
                        }
                        combined_chirho
                    };
                    TypeChirho::ParenChirho {
                        inner_chirho: Box::new(inner_chirho),
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
    /// `( VarId :: Kind )` where Kind is `*`, `* -> *`, `(* -> *) -> *`, etc.
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

        // children_chirho[idx_chirho] is the `(` we already checked
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

        let (kind_chirho, end_idx_chirho) =
            self.parse_kind_tokens_chirho(children_chirho, kind_start_chirho)?;
        if end_idx_chirho != close_idx_chirho {
            return None;
        }

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
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } if matches!(
                fun_chirho.as_ref(),
                TypeChirho::ConChirho(name_chirho)
                    if name_chirho.text_chirho() == "TYPE"
            ) =>
            {
                Some(AstKindChirho::AppChirho(
                    Box::new(Self::try_type_to_ast_kind_chirho(fun_chirho)?),
                    Box::new(Self::try_type_to_ast_kind_chirho(arg_chirho)?),
                ))
            }
            // Lists, tuples etc. can't be represented as AstKindChirho —
            // return None so the kind checker infers the kind.
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
        let (mut lhs_chirho, mut pos_chirho) =
            self.parse_kind_atom_chirho(children_chirho, start_chirho)?;

        // AstKindChirho only retains the runtime-representation application
        // `TYPE representation`. Other named/applied kinds remain unannotated
        // so the kind checker can infer them instead of collapsing them to `*`.
        if matches!(
            &lhs_chirho,
            AstKindChirho::VarChirho(name_chirho) if name_chirho == "TYPE"
        ) && let Some((arg_chirho, end_chirho)) =
            self.parse_kind_atom_chirho(children_chirho, pos_chirho)
        {
            lhs_chirho = AstKindChirho::AppChirho(Box::new(lhs_chirho), Box::new(arg_chirho));
            pos_chirho = end_chirho;
        }

        // Check for `->` to form arrow kinds
        if pos_chirho < children_chirho.len()
            && let GreenElementChirho::TokenChirho(t_chirho) =
                children_chirho[pos_chirho].element_chirho
            && t_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho
        {
            pos_chirho += 1; // consume `->`
            let (rhs_chirho, end_chirho) =
                self.parse_kind_tokens_chirho(children_chirho, pos_chirho)?;
            return Some((
                AstKindChirho::ArrowChirho(Box::new(lhs_chirho), Box::new(rhs_chirho)),
                end_chirho,
            ));
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
                // `*` is a VarSym token with text "*", and `Type` a ConId token with text "Type"
                if (t_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                    && t_chirho.text_chirho() == "*")
                    || (t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                        && t_chirho.text_chirho() == "Type")
                {
                    Some((AstKindChirho::StarChirho, pos_chirho + 1))
                }
                // `Constraint` is a ConId token with text "Constraint" (ConstraintKinds)
                else if t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                    && t_chirho.text_chirho() == "Constraint"
                {
                    Some((AstKindChirho::ConstraintChirho, pos_chirho + 1))
                }
                // Kind variable (PolyKinds): lowercase identifier like `k` in `(a :: k)`.
                // DataKinds: uppercase identifier like `Bool` in `(b :: Bool)`.
                // Any uppercase name in kind position that isn't `Type` or `Constraint`
                // is a promoted data type used as a kind.
                else if t_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                    || t_chirho.kind_chirho() == TokenKindChirho::ConIdChirho
                {
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
                    if after_chirho < children_chirho.len()
                        && let GreenElementChirho::TokenChirho(close_chirho) =
                            children_chirho[after_chirho].element_chirho
                        && close_chirho.kind_chirho() == TokenKindChirho::RightParenChirho
                    {
                        return Some((inner_chirho, after_chirho + 1));
                    }
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
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
