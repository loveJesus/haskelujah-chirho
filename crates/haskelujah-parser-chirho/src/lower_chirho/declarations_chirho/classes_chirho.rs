// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class and instance declaration ownership.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;

impl LowerCtxChirho {
    pub(super) fn lower_class_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut saw_class_chirho = false;
        let mut saw_where_chirho = false;
        let mut saw_fat_arrow_chirho = false;
        let mut saw_pipe_chirho = false;

        // Two-pass approach: collect tokens before and after `=>`.
        // Before `=>` = superclass context, after `=>` = class head (name + type vars).
        // If no `=>`, all tokens form the class head directly.
        let mut pre_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho, usize)> = Vec::new();
        let mut post_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho, usize)> = Vec::new();

        // Fundep parsing state: after `|`, collect `a b -> c d, e -> f`
        let mut fundeps_chirho: Vec<(Vec<String>, Vec<String>)> = Vec::new();
        let mut fundep_from_chirho: Vec<String> = Vec::new();
        let mut fundep_to_chirho: Vec<String> = Vec::new();
        let mut in_to_chirho = false; // true after seeing `->`
        let mut paren_depth_chirho = 0usize;

        let mut idx_chirho = 0;
        while idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[idx_chirho];
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                    if saw_class_chirho && !saw_where_chirho && !saw_pipe_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if saw_fat_arrow_chirho {
                            post_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                        } else {
                            pre_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                        }
                    }
                    paren_depth_chirho += 1;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::RightParenChirho {
                    if saw_class_chirho && !saw_where_chirho && !saw_pipe_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if saw_fat_arrow_chirho {
                            post_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                        } else {
                            pre_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                        }
                    }
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                } else if tok_chirho.kind_chirho() == TokenKindChirho::ClassKeywordChirho {
                    saw_class_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::WhereKeywordChirho
                    && paren_depth_chirho == 0
                {
                    saw_where_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho
                    && saw_class_chirho
                    && !saw_where_chirho
                    && !saw_pipe_chirho
                    && paren_depth_chirho == 0
                {
                    saw_fat_arrow_chirho = true;
                } else if tok_chirho.kind_chirho() == TokenKindChirho::PipeChirho
                    && saw_class_chirho
                    && !saw_where_chirho
                    && paren_depth_chirho == 0
                {
                    saw_pipe_chirho = true;
                } else if saw_pipe_chirho && !saw_where_chirho {
                    // Parsing functional dependencies
                    match tok_chirho.kind_chirho() {
                        TokenKindChirho::VarIdChirho => {
                            let var_name_chirho = tok_chirho.text_chirho().to_string();
                            if in_to_chirho {
                                fundep_to_chirho.push(var_name_chirho);
                            } else {
                                fundep_from_chirho.push(var_name_chirho);
                            }
                        }
                        TokenKindChirho::RightArrowChirho => {
                            in_to_chirho = true;
                        }
                        TokenKindChirho::CommaChirho => {
                            // Flush current fundep
                            if !fundep_from_chirho.is_empty() || !fundep_to_chirho.is_empty() {
                                fundeps_chirho.push((
                                    std::mem::take(&mut fundep_from_chirho),
                                    std::mem::take(&mut fundep_to_chirho),
                                ));
                            }
                            in_to_chirho = false;
                        }
                        _ => {}
                    }
                } else if saw_class_chirho && !saw_where_chirho {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    if saw_fat_arrow_chirho {
                        post_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                    } else {
                        pre_arrow_tokens_chirho.push((tok_chirho, s_chirho, idx_chirho));
                    }
                }
            }
            idx_chirho += 1;
        }

        // Flush last fundep if any
        if !fundep_from_chirho.is_empty() || !fundep_to_chirho.is_empty() {
            fundeps_chirho.push((fundep_from_chirho, fundep_to_chirho));
        }

        // Build superclass context from pre-arrow tokens (if `=>` was present).
        let context_chirho = if saw_fat_arrow_chirho {
            let ctx_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = pre_arrow_tokens_chirho
                .iter()
                .map(|(t_chirho, s_chirho, _)| (*t_chirho, *s_chirho))
                .collect();
            self.build_instance_context_chirho(&ctx_tokens_chirho)
        } else {
            vec![]
        };

        // The head tokens are after `=>` if present, otherwise all pre-arrow tokens.
        let head_tokens_chirho = if saw_fat_arrow_chirho {
            &post_arrow_tokens_chirho
        } else {
            &pre_arrow_tokens_chirho
        };

        // Extract class name and type variables from head tokens.
        let mut name_chirho = None;
        let mut type_vars_chirho = Vec::new();
        let mut skip_until_child_idx_chirho: Option<usize> = None;
        let mut next_binder_is_visible_kind_chirho = false;
        let mut visible_kind_binder_names_chirho = HashSet::new();
        for (tok_chirho, s_chirho, tok_idx_chirho) in head_tokens_chirho {
            if skip_until_child_idx_chirho
                .is_some_and(|skip_idx_chirho| *tok_idx_chirho < skip_idx_chirho)
            {
                continue;
            }
            if tok_chirho.kind_chirho() == TokenKindChirho::AtSignChirho {
                next_binder_is_visible_kind_chirho = true;
            } else if tok_chirho.kind_chirho() == TokenKindChirho::UnderscoreReservedIdChirho {
                next_binder_is_visible_kind_chirho = false;
            } else if (matches!(
                tok_chirho.kind_chirho(),
                TokenKindChirho::ConIdChirho
                    | TokenKindChirho::ConSymChirho
                    | TokenKindChirho::VarSymChirho
            ) || (tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                && tok_chirho
                    .text_chirho()
                    .chars()
                    .next()
                    .is_some_and(|c_chirho| c_chirho.is_uppercase())))
                && name_chirho.is_none()
            {
                name_chirho = Some(self.name_from_token_chirho(tok_chirho, *s_chirho));
            } else if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho {
                let type_var_name_chirho = self.name_from_token_chirho(tok_chirho, *s_chirho);
                if next_binder_is_visible_kind_chirho {
                    visible_kind_binder_names_chirho
                        .insert(type_var_name_chirho.text_chirho().to_string());
                    next_binder_is_visible_kind_chirho = false;
                } else {
                    type_vars_chirho.push(type_var_name_chirho.into());
                }
            } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                if let Some((tv_chirho, consumed_chirho)) =
                    self.declaration_head_binder_chirho(&children_chirho, *tok_idx_chirho)
                {
                    type_vars_chirho.push(tv_chirho);
                    skip_until_child_idx_chirho = Some(*tok_idx_chirho + consumed_chirho);
                }
            }
        }

        // Extract method signatures (and default implementations) from the
        // where block.  The CST where-clause contains TypeSig and FunBind
        // children — we reuse `collect_instance_methods_chirho` to gather them.
        let mut where_decls_chirho: Vec<DeclChirho> = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho {
                    self.collect_instance_methods_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        &mut where_decls_chirho,
                    );
                }
            }
        }

        // Merge multi-equation default method implementations.
        let where_decls_chirho = merge_fun_binds_chirho(where_decls_chirho);

        // Pair type signatures with optional default implementations.
        let mut default_impls_chirho: HashMap<String, Vec<MatchArmChirho>> = HashMap::new();
        for d_chirho in &where_decls_chirho {
            if let DeclChirho::FunBindChirho {
                name_chirho: m_name_chirho,
                matches_chirho,
                ..
            } = d_chirho
            {
                default_impls_chirho.insert(
                    m_name_chirho.text_chirho().to_string(),
                    matches_chirho.clone(),
                );
            }
        }

        // Extract DefaultSignatures: `default methodName :: ConstrainedType`
        // from DefaultDeclChirho nodes in the where clause. These have a VarId
        // followed by `::` after the `default` keyword.
        let mut default_sigs_chirho: HashMap<String, TypeChirho> = HashMap::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::DefaultDeclChirho {
                    if let Some((name_text_chirho, sig_ty_chirho)) =
                        self.try_extract_default_sig_chirho(n_chirho, child_chirho.start_chirho)
                    {
                        default_sigs_chirho.insert(name_text_chirho, sig_ty_chirho);
                    }
                }
                // Also check inside WhereClauseChirho children
                if saw_where_chirho || n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                {
                    let where_children_chirho =
                        self.semantic_children_chirho(n_chirho, child_chirho.start_chirho);
                    for wc_chirho in &where_children_chirho {
                        if let GreenElementChirho::NodeChirho(wn_chirho) = wc_chirho.element_chirho
                        {
                            if wn_chirho.kind_chirho() == SyntaxKindChirho::DefaultDeclChirho {
                                if let Some((name_text_chirho, sig_ty_chirho)) = self
                                    .try_extract_default_sig_chirho(
                                        wn_chirho,
                                        wc_chirho.start_chirho,
                                    )
                                {
                                    default_sigs_chirho.insert(name_text_chirho, sig_ty_chirho);
                                }
                            }
                        }
                    }
                }
            }
        }

        let (methods_chirho, mut assoc_tfs_chirho) = partition_class_members_chirho(
            where_decls_chirho,
            &default_impls_chirho,
            &default_sigs_chirho,
            &visible_kind_binder_names_chirho,
        );
        let data_spans_chirho = self.associated_data_spans_chirho(node_chirho, base_chirho);
        for family_chirho in &mut assoc_tfs_chirho {
            family_chirho.data_chirho |= data_spans_chirho.contains(&family_chirho.span_chirho);
        }
        DeclChirho::ClassDeclChirho {
            context_chirho,
            context_written_chirho: saw_fat_arrow_chirho,
            minimal_chirho: self.lower_minimal_chirho(node_chirho, base_chirho),
            name_chirho: name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            type_vars_chirho,
            methods_chirho,
            associated_tfs_chirho: assoc_tfs_chirho,
            fundeps_chirho,
            span_chirho,
        }
    }

    /// Lower an instance declaration.
    ///
    /// ```haskell
    /// instance Eq a => Eq (Maybe a) where
    ///   (==) (Just x) (Just y) = x == y
    ///   (==) Nothing  Nothing  = True
    ///   (==) _        _        = False
    /// ```
    ///
    /// Tokens between `instance` and `where`:
    ///   - If `=>` is present, everything before it is the context,
    ///     everything after is class + types.
    ///   - Otherwise everything is class + types.
    ///
    /// The where block contains FunBind nodes that become LocalBindChirho.
    pub(super) fn lower_instance_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);

        // Collect all ConId and VarId tokens between `instance` and `where`.
        // If `=>` appears, tokens before it form the context.
        let mut saw_instance_chirho = false;
        let mut saw_where_chirho = false;
        let mut saw_fat_arrow_chirho = false;
        let mut paren_depth_chirho = 0usize;

        // Tokens before `=>` (if any).
        let mut pre_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();
        // Tokens after `=>` (or all of them if no `=>`).
        let mut post_arrow_tokens_chirho: Vec<(&GreenTokenChirho, SpanChirho)> = Vec::new();

        // Where-block sub-nodes (FunBind children inside WhereClause).
        let mut method_decls_chirho: Vec<DeclChirho> = Vec::new();

        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let kind_chirho = tok_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::LeftParenChirho {
                        paren_depth_chirho += 1;
                    }
                    if kind_chirho == TokenKindChirho::RightParenChirho {
                        paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                    }
                    if kind_chirho == TokenKindChirho::InstanceKeywordChirho {
                        saw_instance_chirho = true;
                        continue;
                    }
                    if kind_chirho == TokenKindChirho::WhereKeywordChirho && paren_depth_chirho == 0
                    {
                        saw_where_chirho = true;
                        continue;
                    }
                    if kind_chirho == TokenKindChirho::DoubleArrowChirho && paren_depth_chirho == 0
                    {
                        saw_fat_arrow_chirho = true;
                        continue;
                    }
                    if saw_instance_chirho && !saw_where_chirho {
                        let s_chirho =
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                        if saw_fat_arrow_chirho {
                            post_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                        } else {
                            pre_arrow_tokens_chirho.push((tok_chirho, s_chirho));
                        }
                    }
                }
                GreenElementChirho::NodeChirho(n_chirho) => {
                    if n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho {
                        // Recurse into the WhereClause to find FunBind children.
                        self.collect_instance_methods_chirho(
                            n_chirho,
                            child_chirho.start_chirho,
                            &mut method_decls_chirho,
                        );
                    } else if saw_instance_chirho && !saw_where_chirho {
                        // Type application node (e.g., parenthesized type).
                        // For now we skip sub-nodes in the head; they would
                        // need full type lowering.  Simple cases are covered
                        // by the token scan above.
                    }
                }
            }
        }

        // If no `=>` was seen, everything is class + types.
        let head_tokens_chirho = if saw_fat_arrow_chirho {
            &post_arrow_tokens_chirho
        } else {
            &pre_arrow_tokens_chirho
        };

        // Build context from pre-arrow tokens (simplified: "ClassName varName" pairs).
        let context_chirho = if saw_fat_arrow_chirho {
            self.build_instance_context_chirho(&pre_arrow_tokens_chirho)
        } else {
            vec![]
        };

        let (class_chirho, types_chirho) =
            self.lower_instance_head_types_chirho(head_tokens_chirho, span_chirho);

        // For multi-parameter type classes, keep each type argument as a
        // separate element so that the type checker can split them into
        // head_ty_chirho (first) and extra_head_tys_chirho (rest).
        let instance_type_chirho = types_chirho;

        // Merge consecutive multi-equation methods in instance declarations.
        let method_decls_chirho = merge_fun_binds_chirho(method_decls_chirho);

        // Extract associated type family instances (`type F Int = Bool`)
        // from the where-block declarations.
        let mut assoc_tf_insts_chirho: Vec<
            haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho,
        > = Vec::new();

        // Convert method DeclChirho::FunBindChirho into LocalBindChirho::FunBindChirho.
        let methods_chirho: Vec<LocalBindChirho> = method_decls_chirho
            .into_iter()
            .filter_map(|d_chirho| match d_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => Some(LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                }),
                DeclChirho::TypeFamilyInstanceDeclChirho {
                    family_name_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    span_chirho,
                } => {
                    assoc_tf_insts_chirho.push(
                        haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho {
                            family_name_chirho,
                            lhs_types_chirho,
                            rhs_chirho,
                            span_chirho,
                        },
                    );
                    None
                }
                _ => None,
            })
            .collect();

        DeclChirho::InstanceDeclChirho {
            context_chirho,
            class_chirho: class_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            types_chirho: instance_type_chirho,
            methods_chirho,
            assoc_tf_instances_chirho: assoc_tf_insts_chirho,
            span_chirho,
        }
    }

    pub(super) fn lower_instance_head_types_chirho(
        &self,
        head_tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        span_chirho: SpanChirho,
    ) -> (Option<NameChirho>, Vec<TypeChirho>) {
        let mut idx_chirho = 0usize;
        let mut class_chirho = None;
        if let Some((tok_chirho, tok_span_chirho)) = head_tokens_chirho.first() {
            if matches!(
                tok_chirho.kind_chirho(),
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
            ) {
                class_chirho = Some(self.name_from_token_chirho(tok_chirho, *tok_span_chirho));
                idx_chirho = 1;
            }
        }

        let mut types_chirho = Vec::new();
        while idx_chirho < head_tokens_chirho.len() {
            match head_tokens_chirho[idx_chirho].0.kind_chirho() {
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
                        let head_span_chirho = head_tokens_chirho[idx_chirho]
                            .1
                            .merge_chirho(head_tokens_chirho[end_idx_chirho].1)
                            .unwrap_or(span_chirho);
                        // The shared grammar owns the parenthesized atom,
                        // including constructor sections such as (->) and (,).
                        let head_ty_chirho = self.lower_type_from_token_slice_chirho(
                            &head_tokens_chirho[idx_chirho..=end_idx_chirho],
                            head_span_chirho,
                        );
                        types_chirho.push(head_ty_chirho);
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

    pub(super) fn find_matching_token_index_chirho(
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
        start_idx_chirho: usize,
        open_kind_chirho: TokenKindChirho,
        close_kind_chirho: TokenKindChirho,
    ) -> Option<usize> {
        let mut depth_chirho = 0usize;
        for (idx_chirho, (tok_chirho, _)) in tokens_chirho.iter().enumerate().skip(start_idx_chirho)
        {
            let kind_chirho = tok_chirho.kind_chirho();
            if kind_chirho == open_kind_chirho {
                depth_chirho += 1;
            } else if kind_chirho == close_kind_chirho {
                depth_chirho = depth_chirho.saturating_sub(1);
                if depth_chirho == 0 {
                    return Some(idx_chirho);
                }
            }
        }
        None
    }

    /// Walk a left-nested `AppChirho` spine to collect the class name and
    /// preceding arguments. For `Monad m`, fun is `ConChirho("Monad")` and
    /// returns `("Monad", [])`. For `MonadReader r m`, fun is
    /// `AppChirho(ConChirho("MonadReader"), VarChirho("r"))` and returns
    /// `("MonadReader", [r])`.
    pub(super) fn collect_app_class_chirho(
        ty_chirho: &TypeChirho,
    ) -> (NameChirho, Vec<TypeChirho>) {
        match ty_chirho {
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::collect_app_class_chirho(inner_chirho)
            }
            TypeChirho::ConChirho(name_chirho) | TypeChirho::VarChirho(name_chirho) => {
                (name_chirho.clone(), vec![])
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let (name_chirho, mut args_chirho) = Self::collect_app_class_chirho(fun_chirho);
                args_chirho.push(arg_chirho.as_ref().clone());
                (name_chirho, args_chirho)
            }
            // Fallback: use the type as a synthetic name
            other_chirho => (
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    "?",
                    other_chirho.span_chirho(),
                )),
                vec![],
            ),
        }
    }
}
