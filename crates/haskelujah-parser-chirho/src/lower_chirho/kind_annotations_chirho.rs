// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written kind annotations; workflow: declaration-kinds-chirho.
use super::*;

impl LowerCtxChirho {
    /// Preserve both sides of a type ascription in structured and flat CST
    /// paths. Split before arrows: the arrow in `Int :: Type -> Type` belongs
    /// to the classifier, not to the annotated type's application spine.
    pub(super) fn type_ascription_from_children_chirho(
        &self,
        children_chirho: &[&ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> Option<TypeChirho> {
        let index_chirho =
            self.find_top_level_token_chirho(children_chirho, TokenKindChirho::DoubleColonChirho)?;
        let left_chirho = &children_chirho[..index_chirho];
        let right_chirho = &children_chirho[index_chirho + 1..];
        let child_span_chirho = |parts_chirho: &[&ChildChirho<'_>]| {
            Some(self.span_chirho(
                parts_chirho.first()?.start_chirho,
                parts_chirho.last()?.end_chirho,
            ))
        };
        Some(TypeChirho::KindAnnotChirho {
            type_chirho: Box::new(
                self.type_from_flat_children_chirho(left_chirho, child_span_chirho(left_chirho)?),
            ),
            kind_chirho: Box::new(
                self.type_from_flat_children_chirho(right_chirho, child_span_chirho(right_chirho)?),
            ),
            span_chirho,
        })
    }

    /// Try to convert a lowered `TypeChirho` into an `AstKindChirho` for kind
    /// annotations. Nominal names retain their source identity; applications
    /// retain both sides. Unsupported promoted syntax remains a separate
    /// boundary rather than masquerading as an ordinary kind variable.
    pub(super) fn try_type_to_ast_kind_chirho(ty_chirho: &TypeChirho) -> Option<AstKindChirho> {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.full_name_chirho();
                Some(match text_chirho.as_str() {
                    "Type" | "*" => AstKindChirho::StarChirho,
                    "Constraint" => AstKindChirho::ConstraintChirho,
                    _ => AstKindChirho::ConChirho(name_chirho.clone()),
                })
            }
            TypeChirho::VarChirho(name_chirho) => Some(AstKindChirho::VarChirho(
                name_chirho.text_chirho().to_string(),
            )),
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            } => Some(AstKindChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(Self::try_type_to_ast_kind_chirho(body_chirho)?),
                span_chirho: *span_chirho,
            }),
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            } => Some(AstKindChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(Self::try_type_to_ast_kind_chirho(body_chirho)?),
                span_chirho: *span_chirho,
            }),
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
            TypeChirho::KindAnnotChirho {
                type_chirho,
                kind_chirho,
                span_chirho,
            } => Some(AstKindChirho::KindAnnotChirho {
                type_chirho: Box::new(Self::try_type_to_ast_kind_chirho(type_chirho)?),
                kind_chirho: Box::new(Self::try_type_to_ast_kind_chirho(kind_chirho)?),
                span_chirho: *span_chirho,
            }),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => Some(AstKindChirho::AppChirho(
                Box::new(Self::try_type_to_ast_kind_chirho(fun_chirho)?),
                Box::new(Self::try_type_to_ast_kind_chirho(arg_chirho)?),
            )),
            TypeChirho::KindAppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => Some(AstKindChirho::KindAppChirho(
                Box::new(Self::try_type_to_ast_kind_chirho(fun_chirho)?),
                Box::new(Self::try_type_to_ast_kind_chirho(arg_chirho)?),
            )),
            TypeChirho::ListChirho {
                element_chirho,
                span_chirho,
            } => Some(AstKindChirho::AppChirho(
                Box::new(AstKindChirho::ConChirho(NameChirho::RawChirho(
                    RawNameChirho::unqualified_chirho("[]", *span_chirho),
                ))),
                Box::new(Self::try_type_to_ast_kind_chirho(element_chirho)?),
            )),
            TypeChirho::PromotedConChirho { .. }
            | TypeChirho::PromotedListChirho { .. }
            | TypeChirho::TupleChirho { .. }
            | TypeChirho::LitChirho { .. } => {
                Some(AstKindChirho::TypeSyntaxChirho(Box::new(ty_chirho.clone())))
            }
            _ => None,
        }
    }

    /// Try to parse a kind-annotated type variable from a parenthesized group:
    /// `( VarId :: Kind )`, sharing the complete type syntax and preserving
    /// nominal operators, explicit arguments and nested quantifier scopes.
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
        if idx_chirho + 4 >= children_chirho.len()
            || !matches!(children_chirho[idx_chirho].element_chirho,
                GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho)
        {
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
        if close_idx_chirho >= children_chirho.len() || close_idx_chirho == kind_start_chirho {
            return None;
        }

        // The whole bounded annotation uses the same lowering as declaration
        // signatures. A second, smaller kind parser silently dropped infix
        // contracts such as `(proof :: True :~: False)`.
        let kind_children_chirho: Vec<_> = children_chirho[kind_start_chirho..close_idx_chirho]
            .iter()
            .collect();
        let kind_span_chirho = self.span_chirho(
            children_chirho[kind_start_chirho].start_chirho,
            children_chirho[close_idx_chirho].start_chirho,
        );
        let kind_type_chirho =
            self.type_from_flat_children_chirho(&kind_children_chirho, kind_span_chirho);
        let kind_chirho = Self::try_type_to_ast_kind_chirho(&kind_type_chirho)?;

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
}
