// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pattern-variable binding and constructor-field expected types.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::*;

impl InferCtxChirho {
    /// Whether a scrutinee is `Char#`, where a character literal pattern is
    /// MagicHash's unboxed `'x'#` rather than a boxed `Char`. Only `Char#` is
    /// exempt: GHC rejects `'a'#` on `Addr#` and every unboxed string pattern.
    /// workflow: language-features-chirho/pattern-matching-chirho
    fn is_unboxed_char_chirho(&self, ty_chirho: &TyChirho) -> bool {
        matches!(self.normalize_ty_chirho(ty_chirho), TyChirho::ConChirho(name_chirho) if name_chirho == "Char#")
    }

    /// Bind pattern variables to the given type in the current scope.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn bind_pat_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        ty_chirho: &TyChirho,
    ) -> SubstChirho {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                // For higher-rank types: if the type is ForallChirho, bind as a
                // polymorphic scheme so each use gets a fresh instantiation.
                let scheme_chirho = match ty_chirho {
                    TyChirho::ForallChirho {
                        vars_chirho,
                        body_chirho,
                    } => SchemeChirho {
                        vars_chirho: vars_chirho.clone(),
                        preds_chirho: vec![],
                        ty_chirho: (**body_chirho).clone(),
                    },
                    _ => SchemeChirho::mono_chirho(ty_chirho.clone()),
                };
                self.env_chirho
                    .bind_chirho(name_chirho.text_chirho().to_string(), scheme_chirho);
                SubstChirho::empty_chirho()
            }
            PatChirho::WildcardChirho { .. } => {
                // Wildcards don't bind anything
                SubstChirho::empty_chirho()
            }
            PatChirho::LitChirho(lit_chirho) => {
                // Characters and ordinary strings have fixed pattern types.
                // Numeric and overloaded-string obligations stay on their
                // existing path; they must not be fixed to Int or String.
                // A character literal on a `Char#` scrutinee is MagicHash's
                // `'x'#`, which the parser lowers to the same literal.
                let fixed_chirho = (matches!(lit_chirho, LitChirho::CharChirho(..))
                    && !self.is_unboxed_char_chirho(ty_chirho))
                    || matches!(lit_chirho, LitChirho::StringChirho(..))
                        && !self.overloaded_strings_chirho;
                if fixed_chirho {
                    self.unify_constructor_result_chirho(
                        false,
                        &infer_lit_chirho(lit_chirho),
                        ty_chirho,
                        lit_chirho.span_chirho(),
                    )
                    .unwrap_or_else(SubstChirho::empty_chirho)
                } else {
                    SubstChirho::empty_chirho()
                }
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho: inner_chirho,
                ..
            } => {
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(ty_chirho.clone()),
                );
                self.bind_pat_chirho(inner_chirho, ty_chirho)
            }
            PatChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            } => {
                // Each element gets a fresh type variable
                let elem_tys_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|_| self.fresh_var_chirho())
                    .collect();
                let tuple_ty_chirho = TyChirho::TupleChirho(elem_tys_chirho.clone());
                let mut subst_chirho = self
                    .unify_constructor_result_chirho(
                        false,
                        &tuple_ty_chirho,
                        ty_chirho,
                        *span_chirho,
                    )
                    .unwrap_or_else(SubstChirho::empty_chirho);
                self.apply_subst_all_chirho(&subst_chirho);
                for (elem_pat_chirho, elem_ty_chirho) in
                    elements_chirho.iter().zip(elem_tys_chirho.iter())
                {
                    let elem_ty_sub_chirho = subst_chirho.apply_ty_chirho(elem_ty_chirho);
                    let s_chirho = self.bind_pat_chirho(elem_pat_chirho, &elem_ty_sub_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);
                }
                subst_chirho
            }
            PatChirho::ConChirho {
                con_chirho,
                args_chirho,
                span_chirho,
            } => {
                let args_chirho: Vec<&PatChirho> = args_chirho.iter().collect();
                self.bind_constructor_pattern_chirho(
                    con_chirho,
                    &args_chirho,
                    ty_chirho,
                    *span_chirho,
                )
            }
            PatChirho::RecordChirho {
                con_chirho,
                fields_chirho,
                has_wildcard_chirho,
                span_chirho: pat_span_chirho,
            } => {
                self.report_record_pattern_fields_chirho(con_chirho, fields_chirho);
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut field_type_map_chirho = HashMap::new();
                if let Some((field_names_chirho, field_tys_chirho, result_ty_chirho)) =
                    self.lookup_record_constructor_field_bundle_chirho(con_chirho)
                {
                    let refining_chirho = self.constructor_name_refines_chirho(con_chirho);
                    let result_subst_opt_chirho = self.unify_constructor_result_chirho(
                        refining_chirho,
                        &result_ty_chirho,
                        ty_chirho,
                        *pat_span_chirho,
                    );
                    if let Some(result_subst_chirho) = result_subst_opt_chirho {
                        self.apply_subst_all_chirho(&result_subst_chirho);
                        subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                    }
                    for (field_name_chirho, field_ty_chirho) in field_names_chirho
                        .into_iter()
                        .zip(field_tys_chirho.into_iter())
                    {
                        field_type_map_chirho.insert(
                            field_name_chirho,
                            subst_chirho.apply_ty_chirho(&field_ty_chirho),
                        );
                    }
                } else if let Some(scheme_chirho) = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &con_chirho.full_name_chirho(),
                        con_chirho.text_chirho(),
                    )
                    .filter(|scheme_chirho| !is_placeholder_import_scheme_chirho(scheme_chirho))
                {
                    // Positional C {} still constrains the scrutinee even
                    // though there is no declared record-field bundle.
                    let con_ty_chirho = self.instantiate_chirho(&scheme_chirho, *pat_span_chirho);
                    let result_ty_chirho = self.constructor_result_type_chirho(con_ty_chirho);
                    if let Some(result_subst_chirho) = self.unify_constructor_result_chirho(
                        constructor_result_refines_chirho(&scheme_chirho.ty_chirho),
                        &result_ty_chirho,
                        ty_chirho,
                        *pat_span_chirho,
                    ) {
                        self.apply_subst_all_chirho(&result_subst_chirho);
                        subst_chirho = result_subst_chirho;
                    }
                }
                for field_chirho in fields_chirho {
                    let field_name_key_chirho = record_field_key_chirho(&field_chirho.name_chirho);
                    let field_ty_chirho = field_type_map_chirho
                        .get(&field_name_key_chirho)
                        .cloned()
                        .unwrap_or_else(|| self.fresh_var_chirho());
                    let field_ty_chirho = subst_chirho.apply_ty_chirho(&field_ty_chirho);
                    let s_chirho =
                        self.bind_pat_chirho(&field_chirho.pattern_chirho, &field_ty_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);
                }
                if *has_wildcard_chirho {
                    let explicit_names_chirho: std::collections::HashSet<String> = fields_chirho
                        .iter()
                        .map(|f_chirho| record_field_key_chirho(&f_chirho.name_chirho))
                        .collect();
                    if let Some(all_fields_chirho) = self
                        .con_field_names_chirho
                        .get(con_chirho.text_chirho())
                        .cloned()
                    {
                        for field_name_chirho in &all_fields_chirho {
                            if !explicit_names_chirho.contains(field_name_chirho) {
                                let fresh_chirho = field_type_map_chirho
                                    .get(field_name_chirho)
                                    .cloned()
                                    .unwrap_or_else(|| self.fresh_var_chirho());
                                self.env_chirho.bind_chirho(
                                    field_name_chirho.clone(),
                                    SchemeChirho::mono_chirho(
                                        subst_chirho.apply_ty_chirho(&fresh_chirho),
                                    ),
                                );
                            }
                        }
                    }
                }
                subst_chirho
            }
            PatChirho::InfixConChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => self.bind_constructor_pattern_chirho(
                op_chirho,
                &[left_chirho.as_ref(), right_chirho.as_ref()],
                ty_chirho,
                *span_chirho,
            ),
            PatChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => {
                let elem_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = self
                    .unify_constructor_result_chirho(
                        false,
                        &TyChirho::ListChirho(Box::new(elem_ty_chirho.clone())),
                        ty_chirho,
                        *span_chirho,
                    )
                    .unwrap_or_else(SubstChirho::empty_chirho);
                self.apply_subst_all_chirho(&subst_chirho);
                for elem_pat_chirho in elements_chirho {
                    let elem_ty_sub_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    let s_chirho = self.bind_pat_chirho(elem_pat_chirho, &elem_ty_sub_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);
                }
                subst_chirho
            }
            PatChirho::NegChirho { .. } => {
                // Negated literal: no new bindings
                SubstChirho::empty_chirho()
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.bind_pat_chirho(inner_chirho, ty_chirho)
            }
            PatChirho::LazyChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. } => {
                self.bind_pat_chirho(inner_chirho, ty_chirho)
            }
            PatChirho::ViewChirho {
                expr_chirho,
                pat_chirho: inner_pat_chirho,
                span_chirho,
            } => {
                // View pattern (expr -> pat): the view expression must accept
                // the scrutinee type and produce the inner pattern type.
                let result_ty_chirho = self.fresh_var_chirho();
                let (view_subst_chirho, view_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                self.apply_subst_all_chirho(&view_subst_chirho);

                let expected_view_ty_chirho = TyChirho::fun_chirho(
                    view_subst_chirho.apply_ty_chirho(ty_chirho),
                    view_subst_chirho.apply_ty_chirho(&result_ty_chirho),
                );
                let mut subst_chirho = view_subst_chirho.clone();
                match self.unify_normalized_chirho(
                    &view_subst_chirho.apply_ty_chirho(&view_ty_chirho),
                    &expected_view_ty_chirho,
                    *span_chirho,
                ) {
                    Ok(su_chirho) => {
                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }

                let inner_ty_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                let inner_subst_chirho = self.bind_pat_chirho(inner_pat_chirho, &inner_ty_chirho);
                self.apply_subst_all_chirho(&inner_subst_chirho);
                inner_subst_chirho.compose_chirho(&subst_chirho)
            }
            PatChirho::TypeAnnotChirho {
                pat_chirho: inner_pat_chirho,
                ..
            } => {
                // Type-annotated pattern (x :: T): bind inner pattern with the
                // given type. The annotation is used for scoped type variables
                // but the underlying binding semantics are unchanged.
                self.bind_pat_chirho(inner_pat_chirho, ty_chirho)
            }
        }
    }

    /// Match the constructor's own result before checking its fields. Only
    /// that result may introduce GADT givens; child patterns see those givens
    /// and the growing ordinary substitution, in source order.
    /// workflow: language-features-chirho/pattern-matching-chirho
    fn bind_constructor_pattern_chirho(
        &mut self,
        con_chirho: &NameChirho,
        args_chirho: &[&PatChirho],
        scrutinee_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let scheme_chirho = self
            .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                &con_chirho.full_name_chirho(),
                con_chirho.text_chirho(),
            )
            .filter(|scheme_chirho| !is_placeholder_import_scheme_chirho(scheme_chirho));
        let mut field_tys_chirho = Vec::with_capacity(args_chirho.len());
        let mut subst_chirho = SubstChirho::empty_chirho();
        if let Some(scheme_chirho) = scheme_chirho {
            let instantiated_chirho = self.instantiate_chirho(&scheme_chirho, span_chirho);
            let mut remaining_chirho = self.open_constructor_forall_chirho(instantiated_chirho);
            let mut complete_chirho = true;
            for _arg_chirho in args_chirho {
                remaining_chirho = self.open_constructor_forall_chirho(remaining_chirho);
                match remaining_chirho {
                    TyChirho::FunChirho(field_ty_chirho, result_ty_chirho, _) => {
                        // Do not open a forall inside this higher-rank field.
                        field_tys_chirho.push(*field_ty_chirho);
                        remaining_chirho = *result_ty_chirho;
                    }
                    other_chirho => {
                        complete_chirho = false;
                        field_tys_chirho.push(self.fresh_var_chirho());
                        remaining_chirho = other_chirho;
                    }
                }
            }
            if complete_chirho {
                let result_chirho = self.open_constructor_forall_chirho(remaining_chirho);
                if let Some(result_subst_chirho) = self.unify_constructor_result_chirho(
                    constructor_result_refines_chirho(&scheme_chirho.ty_chirho),
                    &result_chirho,
                    scrutinee_chirho,
                    span_chirho,
                ) {
                    self.apply_subst_all_chirho(&result_subst_chirho);
                    subst_chirho = result_subst_chirho;
                }
            }
        } else {
            // Imported placeholders do not supply trusted constructor shape.
            field_tys_chirho.extend(args_chirho.iter().map(|_| self.fresh_var_chirho()));
        }
        for (arg_chirho, field_ty_chirho) in args_chirho.iter().zip(field_tys_chirho) {
            let expected_chirho = subst_chirho.apply_ty_chirho(&field_ty_chirho);
            let field_subst_chirho = self.bind_pat_chirho(arg_chirho, &expected_chirho);
            self.apply_subst_all_chirho(&field_subst_chirho);
            subst_chirho = field_subst_chirho.compose_chirho(&subst_chirho);
        }
        subst_chirho
    }
}
