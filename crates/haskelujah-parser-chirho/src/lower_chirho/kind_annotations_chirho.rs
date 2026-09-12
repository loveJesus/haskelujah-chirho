// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written kind annotations; workflow: declaration-kinds-chirho.
use super::*;

impl LowerCtxChirho {
    /// Try to convert a lowered `TypeChirho` into an `AstKindChirho` for kind
    /// annotations. Nominal names retain their source identity; applications
    /// retain both sides. Unsupported promoted/list syntax remains a separate
    /// boundary rather than masquerading as an ordinary kind variable.
    pub(super) fn try_type_to_ast_kind_chirho(ty_chirho: &TypeChirho) -> Option<AstKindChirho> {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                Some(match text_chirho {
                    "Type" | "*" => AstKindChirho::StarChirho,
                    "Constraint" => AstKindChirho::ConstraintChirho,
                    _ => AstKindChirho::ConChirho(name_chirho.clone()),
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
            // Lists, tuples etc. can't be represented as AstKindChirho —
            // return None so the kind checker infers the kind.
            _ => None,
        }
    }

    /// Parse a kind expression from flat tokens: `*`, `* -> *`, `(* -> *) -> *`.
    /// Returns the parsed kind and the index of the next unconsumed token.
    pub(super) fn parse_kind_tokens_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        start_chirho: usize,
    ) -> Option<(AstKindChirho, usize)> {
        let (mut lhs_chirho, mut pos_chirho) =
            self.parse_kind_atom_chirho(children_chirho, start_chirho)?;

        loop {
            let invisible_chirho = children_chirho.get(pos_chirho).is_some_and(|child_chirho| {
                matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::AtSignChirho)
            });
            let Some((arg_chirho, end_chirho)) = self.parse_kind_atom_chirho(
                children_chirho,
                pos_chirho + usize::from(invisible_chirho),
            ) else {
                if invisible_chirho {
                    return None;
                }
                break;
            };
            lhs_chirho = if invisible_chirho {
                AstKindChirho::KindAppChirho(Box::new(lhs_chirho), Box::new(arg_chirho))
            } else {
                AstKindChirho::AppChirho(Box::new(lhs_chirho), Box::new(arg_chirho))
            };
            pos_chirho = end_chirho;
        }

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
                else if matches!(
                    t_chirho.kind_chirho(),
                    TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
                ) {
                    let child_chirho = &children_chirho[pos_chirho];
                    Some((
                        AstKindChirho::ConChirho(self.name_from_token_chirho(
                            t_chirho,
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                        )),
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
}
