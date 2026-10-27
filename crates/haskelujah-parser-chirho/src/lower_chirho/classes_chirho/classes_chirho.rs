// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `class` and `instance` declarations, their methods and default signatures.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{LocalBindChirho, MatchArmChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::bindings_chirho::merge_fun_binds_chirho;
use super::{LowerCtxChirho, contexts_chirho};
use crate::type_member_lowering_chirho::partition_class_members_chirho;

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
            } else if tok_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho
                && let Some((tv_chirho, consumed_chirho)) =
                    self.try_parse_kind_annotated_tyvar_chirho(&children_chirho, *tok_idx_chirho)
            {
                type_vars_chirho.push(tv_chirho);
                skip_until_child_idx_chirho = Some(*tok_idx_chirho + consumed_chirho);
            }
        }

        // Extract method signatures (and default implementations) from the
        // where block.  The CST where-clause contains TypeSig and FunBind
        // children — we reuse `collect_instance_methods_chirho` to gather them.
        let mut where_decls_chirho: Vec<DeclChirho> = Vec::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho
                && n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
            {
                self.collect_instance_methods_chirho(
                    n_chirho,
                    child_chirho.start_chirho,
                    &mut where_decls_chirho,
                );
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
        let mut default_sigs_chirho: HashMap<String, String> = HashMap::new();
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::DefaultDeclChirho
                    && let Some((name_text_chirho, sig_ty_chirho)) =
                        self.try_extract_default_sig_chirho(n_chirho, child_chirho.start_chirho)
                {
                    default_sigs_chirho.insert(name_text_chirho, sig_ty_chirho);
                }
                // Also check inside WhereClauseChirho children
                if saw_where_chirho || n_chirho.kind_chirho() == SyntaxKindChirho::WhereClauseChirho
                {
                    let where_children_chirho =
                        self.semantic_children_chirho(n_chirho, child_chirho.start_chirho);
                    for wc_chirho in &where_children_chirho {
                        if let GreenElementChirho::NodeChirho(wn_chirho) = wc_chirho.element_chirho
                            && wn_chirho.kind_chirho() == SyntaxKindChirho::DefaultDeclChirho
                            && let Some((name_text_chirho, sig_ty_chirho)) = self
                                .try_extract_default_sig_chirho(wn_chirho, wc_chirho.start_chirho)
                        {
                            default_sigs_chirho.insert(name_text_chirho, sig_ty_chirho);
                        }
                    }
                }
            }
        }

        let (methods_chirho, assoc_tfs_chirho) = partition_class_members_chirho(
            where_decls_chirho,
            &default_impls_chirho,
            &default_sigs_chirho,
            &visible_kind_binder_names_chirho,
        );
        DeclChirho::ClassDeclChirho {
            context_chirho,
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

        // A leading `forall a b.` binds the instance's variables; it belongs
        // to neither the context nor the head.
        // Workflow: language-features-chirho/flat-type-syntax-chirho.
        let pre_arrow_tokens_chirho =
            contexts_chirho::strip_instance_forall_chirho(&pre_arrow_tokens_chirho);

        // If no `=>` was seen, everything is class + types.
        let head_tokens_chirho = if saw_fat_arrow_chirho {
            &post_arrow_tokens_chirho[..]
        } else {
            pre_arrow_tokens_chirho
        };

        // The context is lowered as a type and converted, like a signature's.
        let context_chirho = if saw_fat_arrow_chirho {
            self.build_instance_context_chirho(pre_arrow_tokens_chirho)
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
                // InstanceSigs: a method's signature stays beside its equations,
                // where the instance-method check reads it to scope the
                // signature's binders.
                DeclChirho::TypeSigChirho {
                    name_chirho,
                    ty_chirho,
                    span_chirho,
                } => Some(LocalBindChirho::TypeSigChirho {
                    name_chirho,
                    ty_chirho,
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
                // In instance where-blocks, `type Elem [a] = a` is parsed as
                // a TypeAliasDeclChirho (no "instance" keyword inside class body).
                // Convert it to an associated type family instance.
                DeclChirho::TypeAliasDeclChirho {
                    name_chirho: ta_name_chirho,
                    type_vars_chirho: ta_tvs_chirho,
                    rhs_chirho: ta_rhs_chirho,
                    span_chirho: ta_span_chirho,
                } => {
                    fn collect_type_var_names_from_ast_type_chirho(
                        ty_chirho: &TypeChirho,
                        out_chirho: &mut Vec<String>,
                    ) {
                        match ty_chirho {
                            TypeChirho::VarChirho(name_chirho) => {
                                let name_text_chirho = name_chirho.text_chirho().to_string();
                                if !out_chirho.contains(&name_text_chirho) {
                                    out_chirho.push(name_text_chirho);
                                }
                            }
                            TypeChirho::AppChirho {
                                fun_chirho,
                                arg_chirho,
                                ..
                            } => {
                                collect_type_var_names_from_ast_type_chirho(fun_chirho, out_chirho);
                                collect_type_var_names_from_ast_type_chirho(arg_chirho, out_chirho);
                            }
                            TypeChirho::FunChirho {
                                arg_chirho,
                                result_chirho,
                                ..
                            } => {
                                collect_type_var_names_from_ast_type_chirho(arg_chirho, out_chirho);
                                collect_type_var_names_from_ast_type_chirho(
                                    result_chirho,
                                    out_chirho,
                                );
                            }
                            TypeChirho::TupleChirho {
                                elements_chirho, ..
                            }
                            | TypeChirho::PromotedListChirho {
                                elements_chirho, ..
                            } => {
                                for elem_chirho in elements_chirho {
                                    collect_type_var_names_from_ast_type_chirho(
                                        elem_chirho,
                                        out_chirho,
                                    );
                                }
                            }
                            TypeChirho::ListChirho { element_chirho, .. }
                            | TypeChirho::ParenChirho {
                                inner_chirho: element_chirho,
                                ..
                            } => {
                                collect_type_var_names_from_ast_type_chirho(
                                    element_chirho,
                                    out_chirho,
                                );
                            }
                            TypeChirho::QualChirho {
                                context_chirho,
                                body_chirho,
                                ..
                            } => {
                                for constraint_chirho in context_chirho {
                                    collect_type_var_names_from_constraint_chirho(
                                        constraint_chirho,
                                        out_chirho,
                                    );
                                }
                                collect_type_var_names_from_ast_type_chirho(
                                    body_chirho,
                                    out_chirho,
                                );
                            }
                            TypeChirho::ForallChirho { body_chirho, .. }
                            | TypeChirho::RequiredForallChirho { body_chirho, .. } => {
                                collect_type_var_names_from_ast_type_chirho(
                                    body_chirho,
                                    out_chirho,
                                );
                            }
                            TypeChirho::ConChirho(_)
                            | TypeChirho::PromotedConChirho { .. }
                            | TypeChirho::WildcardChirho { .. }
                            | TypeChirho::LitChirho { .. } => {}
                        }
                    }

                    fn collect_type_var_names_from_constraint_chirho(
                        constraint_chirho: &ConstraintChirho,
                        out_chirho: &mut Vec<String>,
                    ) {
                        match constraint_chirho {
                            ConstraintChirho::ClassChirho { args_chirho, .. } => {
                                for arg_chirho in args_chirho {
                                    collect_type_var_names_from_ast_type_chirho(
                                        arg_chirho, out_chirho,
                                    );
                                }
                            }
                            ConstraintChirho::QuantifiedChirho {
                                context_chirho,
                                body_chirho,
                                ..
                            } => {
                                for inner_constraint_chirho in context_chirho {
                                    collect_type_var_names_from_constraint_chirho(
                                        inner_constraint_chirho,
                                        out_chirho,
                                    );
                                }
                                collect_type_var_names_from_constraint_chirho(
                                    body_chirho,
                                    out_chirho,
                                );
                            }
                        }
                    }

                    // In instance bodies, associated type family equations like
                    // `type Rep Proxy = ()` are temporarily lowered through the
                    // type-alias path. Concrete heads such as `Proxy` are not
                    // alias type variables. When the alias lowering captured
                    // only the free variables from a concrete head like
                    // `Product f g`, keep the enclosing instance head and only
                    // append any truly extra associated-family parameters.
                    let mut instance_head_var_names_chirho = Vec::new();
                    for instance_head_ty_chirho in &instance_type_chirho {
                        collect_type_var_names_from_ast_type_chirho(
                            instance_head_ty_chirho,
                            &mut instance_head_var_names_chirho,
                        );
                    }
                    let mut lhs_tys_chirho: Vec<TypeChirho> = if instance_type_chirho.is_empty() {
                        Vec::new()
                    } else {
                        instance_type_chirho.clone()
                    };
                    for tv_chirho in &ta_tvs_chirho {
                        let tv_name_text_chirho = tv_chirho.name_chirho.text_chirho().to_string();
                        if !instance_head_var_names_chirho.contains(&tv_name_text_chirho) {
                            lhs_tys_chirho
                                .push(TypeChirho::VarChirho(tv_chirho.name_chirho.clone()));
                        }
                    }
                    if lhs_tys_chirho.is_empty() {
                        lhs_tys_chirho = ta_tvs_chirho
                            .iter()
                            .map(|tv_chirho| TypeChirho::VarChirho(tv_chirho.name_chirho.clone()))
                            .collect();
                    }
                    assoc_tf_insts_chirho.push(
                        haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho {
                            family_name_chirho: ta_name_chirho,
                            lhs_types_chirho: lhs_tys_chirho,
                            rhs_chirho: ta_rhs_chirho,
                            span_chirho: ta_span_chirho,
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

    /// Collect FunBind (and other decl) children from a WhereClause or similar
    /// node, lowering each into a DeclChirho and appending to `out_chirho`.
    fn collect_instance_methods_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        out_chirho: &mut Vec<DeclChirho>,
    ) {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut saw_method_like_decl_chirho = false;
        for child_chirho in &children_chirho {
            if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho.element_chirho {
                if n_chirho.kind_chirho() == SyntaxKindChirho::TypeSigDeclChirho {
                    let type_sigs_chirho = self.lower_type_sigs_chirho(
                        n_chirho,
                        child_chirho.start_chirho,
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                    );
                    if type_sigs_chirho.is_empty() {
                        if saw_method_like_decl_chirho {
                            break;
                        }
                        continue;
                    }
                    saw_method_like_decl_chirho = true;
                    out_chirho.extend(type_sigs_chirho);
                } else if let Some(decl_chirho) =
                    self.lower_decl_chirho(n_chirho, child_chirho.start_chirho)
                {
                    let is_method_like_decl_chirho = matches!(
                        decl_chirho,
                        DeclChirho::FunBindChirho { .. }
                            | DeclChirho::DefaultDeclChirho { .. }
                            | DeclChirho::TypeAliasDeclChirho { .. }
                            | DeclChirho::TypeFamilyDeclChirho { .. }
                            | DeclChirho::TypeFamilyInstanceDeclChirho { .. }
                            | DeclChirho::DataDeclChirho { .. }
                    );
                    if !is_method_like_decl_chirho {
                        if saw_method_like_decl_chirho {
                            break;
                        }
                        continue;
                    }
                    saw_method_like_decl_chirho = true;
                    out_chirho.push(decl_chirho);
                }
            }
        }
    }

    /// Try to extract a DefaultSignatures-style default method signature from
    /// a DefaultDeclChirho CST node: `default methodName :: ConstrainedType`.
    /// Returns Some((method_name, raw_type_text)) if the node matches this
    /// pattern, None if it's a regular default declaration like `default (Int, Double)`.
    fn try_extract_default_sig_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<(String, String)> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        // Pattern: default VarId :: Type
        let mut saw_default_chirho = false;
        let mut method_name_chirho: Option<String> = None;
        let mut saw_double_colon_chirho = false;
        let mut type_text_parts_chirho: Vec<String> = Vec::new();

        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let kind_chirho = tok_chirho.kind_chirho();
                if kind_chirho == TokenKindChirho::DefaultKeywordChirho {
                    saw_default_chirho = true;
                    continue;
                }
                if saw_default_chirho && method_name_chirho.is_none() {
                    if kind_chirho == TokenKindChirho::VarIdChirho {
                        method_name_chirho = Some(tok_chirho.text_chirho().to_string());
                        continue;
                    } else if kind_chirho == TokenKindChirho::LeftParenChirho {
                        // This is `default (Int, Double)` — not a default sig
                        return None;
                    }
                }
                if method_name_chirho.is_some() && kind_chirho == TokenKindChirho::DoubleColonChirho
                {
                    saw_double_colon_chirho = true;
                    continue;
                }
                if saw_double_colon_chirho {
                    type_text_parts_chirho.push(tok_chirho.text_chirho().to_string());
                }
            }
        }

        if let Some(name_chirho) = method_name_chirho
            && saw_double_colon_chirho
            && !type_text_parts_chirho.is_empty()
        {
            let type_text_chirho = type_text_parts_chirho.join(" ");
            return Some((name_chirho, type_text_chirho));
        }
        None
    }
}
