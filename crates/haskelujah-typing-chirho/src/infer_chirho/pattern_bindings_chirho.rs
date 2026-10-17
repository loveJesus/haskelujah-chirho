// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pattern-variable binding and constructor-field expected types.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::*;

impl InferCtxChirho {
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
            PatChirho::LitChirho { .. } => {
                // Literal patterns don't introduce bindings
                SubstChirho::empty_chirho()
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
                elements_chirho, ..
            } => {
                // Each element gets a fresh type variable
                let elem_tys_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|_| self.fresh_var_chirho())
                    .collect();
                let tuple_ty_chirho = TyChirho::TupleChirho(elem_tys_chirho.clone());
                let mut subst_chirho = match self.unify_normalized_chirho(
                    &tuple_ty_chirho,
                    ty_chirho,
                    SpanChirho::DUMMY_CHIRHO,
                ) {
                    Ok(su_chirho) => {
                        self.apply_subst_all_chirho(&su_chirho);
                        su_chirho
                    }
                    Err(_) => SubstChirho::empty_chirho(),
                };
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
                span_chirho: pat_span_chirho,
            } => {
                // Try to look up the constructor's type scheme to get
                // proper argument types (enables GADT field types and
                // higher-rank field extraction).
                let con_text_chirho = con_chirho.text_chirho();
                let con_full_name_chirho = con_chirho.full_name_chirho();
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &con_full_name_chirho,
                        con_text_chirho,
                    );
                let mut used_con_types_chirho = false;
                let mut subst_chirho = SubstChirho::empty_chirho();
                if let Some(scheme_chirho) = scheme_opt_chirho {
                    if !is_placeholder_import_scheme_chirho(&scheme_chirho) {
                        let refining_chirho =
                            constructor_result_refines_chirho(&scheme_chirho.ty_chirho)
                                || args_chirho.iter().any(|arg_chirho| {
                                    self.pattern_contains_refining_constructor_chirho(arg_chirho)
                                });
                        let con_ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, SpanChirho::DUMMY_CHIRHO);
                        let mut remaining_chirho = con_ty_chirho;
                        let mut ok_chirho = true;
                        for arg_chirho in args_chirho {
                            match remaining_chirho {
                                TyChirho::FunChirho(arg_ty_chirho, res_ty_chirho, _) => {
                                    let s_chirho = self.bind_pat_chirho(arg_chirho, &arg_ty_chirho);
                                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&s_chirho);
                                    remaining_chirho = *res_ty_chirho;
                                }
                                _ => {
                                    ok_chirho = false;
                                    let fresh_chirho = self.fresh_var_chirho();
                                    let s_chirho = self.bind_pat_chirho(arg_chirho, &fresh_chirho);
                                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&s_chirho);
                                }
                            }
                        }
                        if ok_chirho {
                            // Unify remaining (result type) with scrutinee type;
                            // a GADT constructor may instead refine a rigid
                            // scrutinee variable for this pattern's scope.
                            // The argument patterns may have solved variables
                            // shared with the result (`Just Refl`), so unify the
                            // substituted result, not the stale one.
                            let remaining_sub_chirho =
                                subst_chirho.apply_ty_chirho(&remaining_chirho);
                            let scrutinee_sub_chirho = subst_chirho.apply_ty_chirho(ty_chirho);
                            if let Some(su_chirho) = self.unify_constructor_result_chirho(
                                refining_chirho,
                                &remaining_sub_chirho,
                                &scrutinee_sub_chirho,
                                *pat_span_chirho,
                            ) {
                                subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                self.apply_subst_all_chirho(&su_chirho);
                            }
                        }
                        used_con_types_chirho = true;
                    }
                }
                if !used_con_types_chirho {
                    for arg_chirho in args_chirho {
                        let fresh_chirho = self.fresh_var_chirho();
                        let s_chirho = self.bind_pat_chirho(arg_chirho, &fresh_chirho);
                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&s_chirho);
                    }
                }
                subst_chirho
            }
            PatChirho::RecordChirho {
                con_chirho,
                fields_chirho,
                has_wildcard_chirho,
                span_chirho: pat_span_chirho,
            } => {
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut field_type_map_chirho = HashMap::new();
                if let Some((field_names_chirho, field_tys_chirho, result_ty_chirho)) =
                    self.lookup_record_constructor_field_bundle_chirho(con_chirho)
                {
                    let refining_chirho = self.constructor_name_refines_chirho(con_chirho)
                        || fields_chirho.iter().any(|field_chirho| {
                            self.pattern_contains_refining_constructor_chirho(
                                &field_chirho.pattern_chirho,
                            )
                        });
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
                }
                for field_chirho in fields_chirho {
                    let field_name_key_chirho = record_field_key_chirho(&field_chirho.name_chirho);
                    let field_ty_chirho = field_type_map_chirho
                        .get(&field_name_key_chirho)
                        .cloned()
                        .unwrap_or_else(|| self.fresh_var_chirho());
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
                                    SchemeChirho::mono_chirho(fresh_chirho),
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
                span_chirho: pat_span_chirho,
            } => {
                let op_text_chirho = op_chirho.text_chirho();
                let op_full_name_chirho = op_chirho.full_name_chirho();
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &op_full_name_chirho,
                        op_text_chirho,
                    );

                let mut used_con_types_chirho = false;
                let mut subst_chirho = SubstChirho::empty_chirho();
                if let Some(scheme_chirho) = scheme_opt_chirho {
                    if !scheme_chirho.vars_chirho.is_empty() {
                        let refining_chirho =
                            constructor_result_refines_chirho(&scheme_chirho.ty_chirho)
                                || self.pattern_contains_refining_constructor_chirho(left_chirho)
                                || self.pattern_contains_refining_constructor_chirho(right_chirho);
                        let con_ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, SpanChirho::DUMMY_CHIRHO);
                        let mut remaining_chirho = con_ty_chirho;
                        let mut ok_chirho = true;
                        for arg_pat_chirho in [left_chirho.as_ref(), right_chirho.as_ref()] {
                            match remaining_chirho {
                                TyChirho::FunChirho(arg_ty_chirho, res_ty_chirho, _) => {
                                    let s_chirho =
                                        self.bind_pat_chirho(arg_pat_chirho, &arg_ty_chirho);
                                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&s_chirho);
                                    remaining_chirho = *res_ty_chirho;
                                }
                                _ => {
                                    ok_chirho = false;
                                    let fresh_chirho = self.fresh_var_chirho();
                                    let s_chirho =
                                        self.bind_pat_chirho(arg_pat_chirho, &fresh_chirho);
                                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&s_chirho);
                                }
                            }
                        }
                        if ok_chirho {
                            // The argument patterns may have solved variables
                            // shared with the result (`Just Refl`), so unify the
                            // substituted result, not the stale one.
                            let remaining_sub_chirho =
                                subst_chirho.apply_ty_chirho(&remaining_chirho);
                            let scrutinee_sub_chirho = subst_chirho.apply_ty_chirho(ty_chirho);
                            if let Some(su_chirho) = self.unify_constructor_result_chirho(
                                refining_chirho,
                                &remaining_sub_chirho,
                                &scrutinee_sub_chirho,
                                *pat_span_chirho,
                            ) {
                                subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                self.apply_subst_all_chirho(&su_chirho);
                            }
                        }
                        used_con_types_chirho = true;
                    }
                }
                if !used_con_types_chirho {
                    let left_ty_chirho = self.fresh_var_chirho();
                    let right_ty_chirho = self.fresh_var_chirho();
                    let left_s_chirho = self.bind_pat_chirho(left_chirho, &left_ty_chirho);
                    self.apply_subst_all_chirho(&left_s_chirho);
                    let right_s_chirho = self.bind_pat_chirho(right_chirho, &right_ty_chirho);
                    self.apply_subst_all_chirho(&right_s_chirho);
                    subst_chirho = right_s_chirho.compose_chirho(&left_s_chirho);
                }
                subst_chirho
            }
            PatChirho::ListChirho {
                elements_chirho, ..
            } => {
                let elem_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = match self.unify_normalized_chirho(
                    &TyChirho::ListChirho(Box::new(elem_ty_chirho.clone())),
                    ty_chirho,
                    SpanChirho::DUMMY_CHIRHO,
                ) {
                    Ok(su_chirho) => {
                        self.apply_subst_all_chirho(&su_chirho);
                        su_chirho
                    }
                    Err(_) => SubstChirho::empty_chirho(),
                };
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
}
