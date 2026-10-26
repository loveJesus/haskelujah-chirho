// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written declaration heads and their existing flat type token conversion.
//! workflow: language-features-chirho/flat-type-syntax-chirho

use super::*;

impl LowerCtxChirho {
    /// Keep the quote and the list's element boundaries in flat declaration
    /// heads, including a promoted list nested in another argument.
    /// workflow: language-features-chirho/flat-type-syntax-chirho
    pub(super) fn lower_promoted_list_atom_chirho(
        &self,
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        index_chirho: usize,
        fallback_span_chirho: SpanChirho,
    ) -> Option<(TypeChirho, usize)> {
        if tokens_chirho.get(index_chirho)?.0.kind_chirho() != TokenKindChirho::TickChirho
            || tokens_chirho.get(index_chirho + 1)?.0.kind_chirho()
                != TokenKindChirho::LeftBracketChirho
        {
            return None;
        }
        let end_chirho = Self::find_matching_token_index_chirho(
            tokens_chirho,
            index_chirho + 1,
            TokenKindChirho::LeftBracketChirho,
            TokenKindChirho::RightBracketChirho,
        )?;
        let span_chirho = tokens_chirho[index_chirho]
            .1
            .merge_chirho(tokens_chirho[end_chirho].1)
            .unwrap_or(fallback_span_chirho);
        let inner_chirho = &tokens_chirho[index_chirho + 2..end_chirho];
        let elements_chirho = if inner_chirho.is_empty() {
            Vec::new()
        } else {
            // The existing flat grammar splits only top-level commas. A
            // parenthesized tuple stays wrapped, so '[ (Int, Bool) ] has one
            // element while '[Int, Bool] has two.
            match self.lower_type_from_token_slice_chirho(inner_chirho, span_chirho) {
                TypeChirho::TupleChirho {
                    elements_chirho, ..
                } => elements_chirho,
                element_chirho => vec![element_chirho],
            }
        };
        Some((
            TypeChirho::PromotedListChirho {
                elements_chirho,
                span_chirho,
            },
            end_chirho + 1,
        ))
    }

    pub(super) fn lower_instance_head_types_chirho(
        &self,
        head_tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        span_chirho: SpanChirho,
    ) -> (Option<NameChirho>, Vec<TypeChirho>) {
        let mut idx_chirho = 0usize;
        let mut class_chirho = None;
        if let Some((tok_chirho, tok_span_chirho)) = head_tokens_chirho.first()
            && matches!(
                tok_chirho.kind_chirho(),
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
            )
        {
            class_chirho = Some(self.name_from_token_chirho(tok_chirho, *tok_span_chirho));
            idx_chirho = 1;
        }

        let mut types_chirho = Vec::new();
        while idx_chirho < head_tokens_chirho.len() {
            match head_tokens_chirho[idx_chirho].0.kind_chirho() {
                TokenKindChirho::TickChirho => {
                    if let Some((ty_chirho, next_chirho)) = self.lower_promoted_list_atom_chirho(
                        head_tokens_chirho,
                        idx_chirho,
                        span_chirho,
                    ) {
                        types_chirho.push(ty_chirho);
                        idx_chirho = next_chirho;
                    } else {
                        idx_chirho += 1;
                    }
                }
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
                        let inner_tokens_chirho =
                            &head_tokens_chirho[idx_chirho + 1..end_idx_chirho];
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
                        let inner_tokens_chirho =
                            &head_tokens_chirho[idx_chirho + 1..end_idx_chirho];
                        let head_span_chirho = head_tokens_chirho[idx_chirho]
                            .1
                            .merge_chirho(head_tokens_chirho[end_idx_chirho].1)
                            .unwrap_or(span_chirho);
                        let element_chirho = (!inner_tokens_chirho.is_empty()).then(|| {
                            self.lower_type_from_token_slice_chirho(
                                inner_tokens_chirho,
                                head_span_chirho,
                            )
                        });
                        types_chirho.push(flat_types_chirho::list_type_chirho(
                            element_chirho,
                            head_span_chirho,
                        ));
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

    pub(super) fn lower_type_from_token_slice_chirho(
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
            if tuple_parts_chirho
                .iter()
                .all(|part_chirho| part_chirho.is_empty())
            {
                let commas_chirho = tuple_parts_chirho.len().saturating_sub(1);
                let tuple_name_chirho = format!("({})", ",".repeat(commas_chirho));
                return TypeChirho::ConChirho(NameChirho::RawChirho(
                    RawNameChirho::unqualified_chirho(&tuple_name_chirho, span_chirho),
                ));
            }
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

        if let Some(double_colon_idx_chirho) = self
            .find_top_level_token_index_in_token_slice_chirho(
                tokens_chirho,
                TokenKindChirho::DoubleColonChirho,
            )
            .filter(|idx_chirho| *idx_chirho > 0)
        {
            return self.lower_type_from_token_slice_chirho(
                &tokens_chirho[..double_colon_idx_chirho],
                span_chirho,
            );
        }

        if let Some(arrow_idx_chirho) = self
            .find_top_level_token_index_in_token_slice_chirho(
                tokens_chirho,
                TokenKindChirho::RightArrowChirho,
            )
            .filter(|idx_chirho| *idx_chirho > 0 && *idx_chirho + 1 < tokens_chirho.len())
        {
            let arg_chirho = self.lower_type_from_token_slice_chirho(
                &tokens_chirho[..arrow_idx_chirho],
                span_chirho,
            );
            let result_chirho = self.lower_type_from_token_slice_chirho(
                &tokens_chirho[arrow_idx_chirho + 1..],
                span_chirho,
            );
            return TypeChirho::FunChirho {
                arg_chirho: Box::new(arg_chirho),
                mult_chirho: None,
                result_chirho: Box::new(result_chirho),
                span_chirho,
            };
        }

        let mut paren_depth_chirho = 0usize;
        let mut bracket_depth_chirho = 0usize;
        for (idx_chirho, (tok_chirho, tok_span_chirho)) in tokens_chirho.iter().enumerate() {
            match tok_chirho.kind_chirho() {
                TokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                TokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                TokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                TokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                TokenKindChirho::ConSymChirho
                | TokenKindChirho::QualifiedConSymChirho
                | TokenKindChirho::VarSymChirho
                | TokenKindChirho::QualifiedVarSymChirho
                | TokenKindChirho::TildeChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && idx_chirho > 0
                        && idx_chirho + 1 < tokens_chirho.len() =>
                {
                    let left_ty_chirho = self.lower_type_from_token_slice_chirho(
                        &tokens_chirho[..idx_chirho],
                        span_chirho,
                    );
                    let right_ty_chirho = self.lower_type_from_token_slice_chirho(
                        &tokens_chirho[idx_chirho + 1..],
                        span_chirho,
                    );
                    let op_ty_chirho = match tok_chirho.kind_chirho() {
                        TokenKindChirho::ConSymChirho
                        | TokenKindChirho::QualifiedConSymChirho
                        | TokenKindChirho::TildeChirho => TypeChirho::ConChirho(
                            self.name_from_token_chirho(tok_chirho, *tok_span_chirho),
                        ),
                        _ => TypeChirho::VarChirho(
                            self.name_from_token_chirho(tok_chirho, *tok_span_chirho),
                        ),
                    };
                    return TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(op_ty_chirho),
                            arg_chirho: Box::new(left_ty_chirho),
                            span_chirho,
                        }),
                        arg_chirho: Box::new(right_ty_chirho),
                        span_chirho,
                    };
                }
                _ => {}
            }
        }

        let mut atom_tys_chirho = Vec::new();
        let mut idx_chirho = 0usize;
        while idx_chirho < tokens_chirho.len() {
            match tokens_chirho[idx_chirho].0.kind_chirho() {
                TokenKindChirho::TickChirho => {
                    if let Some((ty_chirho, next_chirho)) =
                        self.lower_promoted_list_atom_chirho(tokens_chirho, idx_chirho, span_chirho)
                    {
                        atom_tys_chirho.push(ty_chirho);
                        idx_chirho = next_chirho;
                    } else {
                        idx_chirho += 1;
                    }
                }
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
                TokenKindChirho::ConSymChirho
                | TokenKindChirho::QualifiedConSymChirho
                | TokenKindChirho::TildeChirho
                | TokenKindChirho::RightArrowChirho => {
                    let (tok_chirho, tok_span_chirho) = tokens_chirho[idx_chirho];
                    atom_tys_chirho.push(TypeChirho::ConChirho(
                        self.name_from_token_chirho(tok_chirho, tok_span_chirho),
                    ));
                    idx_chirho += 1;
                }
                TokenKindChirho::VarSymChirho | TokenKindChirho::QualifiedVarSymChirho
                    if !matches!(
                        tokens_chirho[idx_chirho].0.text_chirho(),
                        "." | "!" | "%" | "@" | "|" | "->" | "=>"
                    ) =>
                {
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
                        let inner_ty_chirho = self
                            .lower_type_from_token_slice_chirho(inner_tokens_chirho, span_chirho);
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
                        let element_chirho = (!inner_tokens_chirho.is_empty()).then(|| {
                            self.lower_type_from_token_slice_chirho(
                                inner_tokens_chirho,
                                span_chirho,
                            )
                        });
                        atom_tys_chirho.push(flat_types_chirho::list_type_chirho(
                            element_chirho,
                            span_chirho,
                        ));
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

    /// Lower `deriving instance [context =>] ClassName Type1 Type2...`
    pub(super) fn lower_standalone_deriving_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        let mut saw_instance_chirho = false;
        let mut saw_fat_arrow_chirho = false;
        let mut paren_depth_chirho = 0usize;

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
                if kind_chirho == TokenKindChirho::LeftParenChirho {
                    paren_depth_chirho += 1;
                }
                if kind_chirho == TokenKindChirho::RightParenChirho {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                // Only the declaration's own `=>` separates context from head;
                // one inside parentheses belongs to a quantified constraint.
                if kind_chirho == TokenKindChirho::DoubleArrowChirho
                    && paren_depth_chirho == 0
                    && !saw_fat_arrow_chirho
                {
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

        // Workflow: language-features-chirho/flat-type-syntax-chirho.
        let pre_arrow_tokens_chirho =
            contexts_chirho::strip_instance_forall_chirho(&pre_arrow_tokens_chirho);
        let head_tokens_chirho = if saw_fat_arrow_chirho {
            &post_arrow_tokens_chirho[..]
        } else {
            pre_arrow_tokens_chirho
        };

        let context_chirho = if saw_fat_arrow_chirho {
            self.build_instance_context_chirho(pre_arrow_tokens_chirho)
        } else {
            vec![]
        };

        // Preserve an applied head as one type, as ordinary instance lowering
        // already does. Splitting `(SBoolChirho b)` into two entries changes
        // the written method-row key and leaves the derived dictionary empty.
        // workflow: language-features-chirho/flat-type-syntax-chirho
        let (class_chirho, types_chirho) =
            self.lower_instance_head_types_chirho(head_tokens_chirho, span_chirho);

        DeclChirho::StandaloneDerivingDeclChirho {
            context_chirho,
            class_chirho: class_chirho.unwrap_or_else(|| {
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho("Unknown", span_chirho))
            }),
            types_chirho,
            span_chirho,
        }
    }
}
