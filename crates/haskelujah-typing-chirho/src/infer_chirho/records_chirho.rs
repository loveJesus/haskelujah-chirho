// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type-changing record update (Haskell 2010 §3.15.3) and record constructor
//! field bundles for constructors declared with a context.
//! workflow: language-features-chirho/rigid-type-variables-chirho

use super::*;

/// The names of a record constructor's strict fields (`f :: !T`, unpacked or not).
pub(super) fn strict_field_names_chirho(
    fields_chirho: &[haskelujah_ast_chirho::decl_chirho::FieldDeclChirho],
) -> Vec<String> {
    use haskelujah_ast_chirho::decl_chirho::StrictnessChirho;
    fields_chirho
        .iter()
        .filter(|fd_chirho| fd_chirho.strictness_chirho != StrictnessChirho::LazyChirho)
        .flat_map(|fd_chirho| {
            fd_chirho
                .names_chirho
                .iter()
                .map(|n_chirho| n_chirho.text_chirho().to_string())
        })
        .collect()
}

/// The constructor name of any constructor declaration shape.
pub(super) fn con_decl_name_chirho(
    con_chirho: &haskelujah_ast_chirho::decl_chirho::ConDeclChirho,
) -> &NameChirho {
    use haskelujah_ast_chirho::decl_chirho::ConDeclChirho;
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho,
    }
}

impl InferCtxChirho {
    /// Type-changing record update, as in Haskell 2010 §3.15.3: the input
    /// record and the result are two instantiations of the constructor's
    /// type; updated fields take their value's type in the result, and every
    /// other field keeps its type across both. `r { a = c }` on
    /// `Rec a b` with `a` only in the field `a` therefore yields `Rec c b`.
    /// Returns `None` when no constructor declaring all the updated fields is
    /// known, so the caller keeps its non-type-changing fallback.
    pub(super) fn infer_record_update_fields_chirho(
        &mut self,
        base_ty_chirho: &TyChirho,
        fields_chirho: &[haskelujah_ast_chirho::expr_chirho::FieldAssignChirho],
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        let updated_keys_chirho: Vec<String> = fields_chirho
            .iter()
            .map(|field_chirho| {
                strip_name_qualifier_chirho(&field_chirho.name_chirho.full_name_chirho())
                    .to_string()
            })
            .collect();
        // DETERMINISM: pick the owning constructor in a fixed order.
        let mut owners_chirho: Vec<&String> = self
            .con_field_names_chirho
            .iter()
            .filter(|(_con_chirho, names_chirho)| {
                updated_keys_chirho
                    .iter()
                    .all(|key_chirho| names_chirho.contains(key_chirho))
            })
            .map(|(con_chirho, _names_chirho)| con_chirho)
            .collect();
        owners_chirho.sort();
        let con_name_chirho = owners_chirho.first()?.to_string();
        let (names_in_chirho, tys_in_chirho, result_in_chirho) =
            self.record_constructor_bundle_by_name_chirho(&con_name_chirho, span_chirho)?;
        let (_names_out_chirho, tys_out_chirho, result_out_chirho) =
            self.record_constructor_bundle_by_name_chirho(&con_name_chirho, span_chirho)?;
        let mut subst_chirho =
            match self.unify_normalized_chirho(&result_in_chirho, base_ty_chirho, span_chirho) {
                Ok(s_chirho) => {
                    self.apply_subst_all_chirho(&s_chirho);
                    s_chirho
                }
                Err(err_chirho) => {
                    self.report_unify_error_chirho(&err_chirho);
                    SubstChirho::empty_chirho()
                }
            };
        for (index_chirho, name_chirho) in names_in_chirho.iter().enumerate() {
            let out_ty_chirho = subst_chirho.apply_ty_chirho(&tys_out_chirho[index_chirho]);
            let assigned_chirho = fields_chirho.iter().find(|field_chirho| {
                strip_name_qualifier_chirho(&field_chirho.name_chirho.full_name_chirho())
                    == name_chirho.as_str()
            });
            let unify_result_chirho = match assigned_chirho {
                Some(field_chirho) => {
                    let (value_subst_chirho, value_ty_chirho) =
                        self.infer_expr_chirho(&field_chirho.value_chirho);
                    subst_chirho = value_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&value_subst_chirho);
                    let out_ty_chirho = subst_chirho.apply_ty_chirho(&out_ty_chirho);
                    let value_ty_chirho = subst_chirho.apply_ty_chirho(&value_ty_chirho);
                    self.unify_normalized_chirho(
                        &value_ty_chirho,
                        &out_ty_chirho,
                        field_chirho.span_chirho,
                    )
                }
                None => {
                    let in_ty_chirho = subst_chirho.apply_ty_chirho(&tys_in_chirho[index_chirho]);
                    self.unify_normalized_chirho(&in_ty_chirho, &out_ty_chirho, span_chirho)
                }
            };
            match unify_result_chirho {
                Ok(s_chirho) => {
                    self.apply_subst_all_chirho(&s_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                }
                Err(err_chirho) => self.report_unify_error_chirho(&err_chirho),
            }
        }
        let result_ty_chirho = subst_chirho.apply_ty_chirho(&result_out_chirho);
        Some((subst_chirho, result_ty_chirho))
    }

    /// Like `lookup_record_constructor_field_bundle_chirho`, for a constructor
    /// known only by name (from the field-owner table).
    /// A record update names fields no constructor declares. When the record
    /// type's head is one this module declares — so its constructor list is
    /// complete — every such field is an error at its own span (GHC:
    /// "Constructor `P' does not have field `pz'"); for an imported or unknown
    /// head nothing is reported, because the list may be incomplete. Returns
    /// whether anything was reported.
    pub(super) fn report_undeclared_record_fields_chirho(
        &mut self,
        base_ty_chirho: &TyChirho,
        fields_chirho: &[haskelujah_ast_chirho::expr_chirho::FieldAssignChirho],
    ) -> bool {
        let mut head_chirho = self.normalize_ty_chirho(base_ty_chirho);
        while let TyChirho::AppChirho(fun_chirho, _arg_chirho) = head_chirho {
            head_chirho = *fun_chirho;
        }
        let TyChirho::ConChirho(type_name_chirho) = head_chirho else {
            return false;
        };
        let Some(constructors_chirho) = self.data_constructors_chirho.get(&type_name_chirho) else {
            return false;
        };
        let declared_chirho: HashSet<&String> = constructors_chirho
            .iter()
            .filter_map(|con_chirho| self.con_field_names_chirho.get(con_chirho))
            .flat_map(|names_chirho| names_chirho.iter())
            .collect();
        let mut reported_chirho = false;
        for field_chirho in fields_chirho {
            let field_name_chirho =
                strip_name_qualifier_chirho(&field_chirho.name_chirho.full_name_chirho())
                    .to_string();
            if declared_chirho.contains(&field_name_chirho) {
                continue;
            }
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(RECORD_FIELD_CODE_CHIRHO),
                    format!(
                        "no constructor of `{type_name_chirho}` has a field `{field_name_chirho}`"
                    ),
                    field_chirho.span_chirho,
                ));
            reported_chirho = true;
        }
        reported_chirho
    }

    /// Record construction rules that go by the constructor's FIELD SET, never
    /// its argument arity: a given field the constructor does not declare is
    /// an error (E0206); an omitted field the constructor declares strict is
    /// an error (E0207, GHC-95909); an omitted lazy field is allowed and
    /// becomes a named missing-field thunk in the desugarer.
    pub(super) fn report_record_construction_fields_chirho(
        &mut self,
        con_name_chirho: &str,
        ordered_field_names_chirho: &[String],
        fields_chirho: &[haskelujah_ast_chirho::expr_chirho::FieldAssignChirho],
        has_wildcard_chirho: bool,
        span_chirho: SpanChirho,
    ) {
        for field_chirho in fields_chirho {
            let given_chirho =
                strip_name_qualifier_chirho(&field_chirho.name_chirho.full_name_chirho())
                    .to_string();
            if !ordered_field_names_chirho.contains(&given_chirho) {
                self.report_field_not_declared_by_constructor_chirho(
                    con_name_chirho,
                    &given_chirho,
                    field_chirho.span_chirho,
                );
            }
        }
        if has_wildcard_chirho {
            return;
        }
        let strict_chirho = self
            .con_strict_fields_chirho
            .get(con_name_chirho)
            .cloned()
            .unwrap_or_default();
        for strict_field_chirho in strict_chirho {
            let given_chirho = fields_chirho.iter().any(|field_chirho| {
                strip_name_qualifier_chirho(&field_chirho.name_chirho.full_name_chirho())
                    == strict_field_chirho
            });
            if !given_chirho {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(RECORD_STRICT_FIELD_CODE_CHIRHO),
                        format!(
                            "constructor `{con_name_chirho}` does not have the required strict field `{strict_field_chirho}`"
                        ),
                        span_chirho,
                    ));
            }
        }
    }

    pub(super) fn record_constructor_bundle_by_name_chirho(
        &mut self,
        con_name_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Option<(Vec<String>, Vec<TyChirho>, TyChirho)> {
        let scheme_chirho = self.lookup_value_scheme_with_qualified_suffix_fallback_chirho(
            con_name_chirho,
            con_name_chirho,
        )?;
        if is_placeholder_import_scheme_chirho(&scheme_chirho) {
            return None;
        }
        let mut field_names_chirho = self.con_field_names_chirho.get(con_name_chirho).cloned()?;
        let instantiated_chirho = self.instantiate_chirho(&scheme_chirho, span_chirho);
        let mut remaining_ty_chirho = self.open_constructor_forall_chirho(instantiated_chirho);
        let mut field_tys_chirho = Vec::new();
        while field_tys_chirho.len() < field_names_chirho.len() {
            match remaining_ty_chirho {
                TyChirho::FunChirho(arg_ty_chirho, result_ty_chirho, _) => {
                    field_tys_chirho.push(*arg_ty_chirho);
                    remaining_ty_chirho = *result_ty_chirho;
                }
                _ => break,
            }
        }
        field_names_chirho.truncate(field_tys_chirho.len());
        Some((field_names_chirho, field_tys_chirho, remaining_ty_chirho))
    }

    /// `C {}` on a constructor without a declared field set (a positional
    /// constructor, or an imported one whose fields are unknown) supplies
    /// nothing, so every argument is a missing field and the construction has
    /// the constructor's result type: the instantiated type with its argument
    /// arrows peeled off.
    /// workflow: language-features-chirho/rigid-type-variables-chirho (records)
    pub(super) fn constructor_result_type_chirho(&mut self, con_ty_chirho: TyChirho) -> TyChirho {
        let mut remaining_chirho = self.open_constructor_forall_chirho(con_ty_chirho);
        while let TyChirho::FunChirho(_arg_chirho, result_chirho, _mult_chirho) = remaining_chirho {
            remaining_chirho = *result_chirho;
        }
        remaining_chirho
    }

    /// True when the constructor belongs to a type declared in this module, so
    /// its field set is fully known (a positional constructor has none).
    pub(super) fn is_local_constructor_chirho(&self, con_name_chirho: &str) -> bool {
        self.data_constructors_chirho.values().any(|cons_chirho| {
            cons_chirho
                .iter()
                .any(|con_chirho| con_chirho == con_name_chirho)
        })
    }

    /// Named fields given to a local constructor that declares none are all
    /// undeclared (E0206). Returns true when anything was reported.
    /// workflow: language-features-chirho/rigid-type-variables-chirho (records)
    pub(super) fn report_fields_on_positional_constructor_chirho(
        &mut self,
        con_name_chirho: &str,
        fields_chirho: &[haskelujah_ast_chirho::expr_chirho::FieldAssignChirho],
    ) -> bool {
        if fields_chirho.is_empty() || !self.is_local_constructor_chirho(con_name_chirho) {
            return false;
        }
        for field_chirho in fields_chirho {
            let field_name_chirho =
                strip_name_qualifier_chirho(&field_chirho.name_chirho.full_name_chirho())
                    .to_string();
            self.report_field_not_declared_by_constructor_chirho(
                con_name_chirho,
                &field_name_chirho,
                field_chirho.span_chirho,
            );
        }
        true
    }

    /// `C {}` on a local positional constructor with a strict argument omits
    /// that argument: E0207 (tcfail112; GHC: "does not have the required
    /// strict field(s)"). A constructor from another module is not judged.
    /// workflow: language-features-chirho/rigid-type-variables-chirho (records)
    pub(super) fn report_omitted_strict_positional_chirho(
        &mut self,
        con_name_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        if !self
            .strict_positional_constructors_chirho
            .contains(con_name_chirho)
        {
            return;
        }
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(RECORD_STRICT_FIELD_CODE_CHIRHO),
                format!(
                    "constructor `{con_name_chirho}` does not have the required strict field(s)"
                ),
                span_chirho,
            ));
    }

    /// E0206 at the field: GHC's "Constructor `C' does not have field `f'".
    fn report_field_not_declared_by_constructor_chirho(
        &mut self,
        con_name_chirho: &str,
        field_name_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(RECORD_FIELD_CODE_CHIRHO),
                format!(
                    "constructor `{con_name_chirho}` does not have a field `{field_name_chirho}`"
                ),
                span_chirho,
            ));
    }

    /// A constructor declared with a context or existential binders keeps a
    /// `forall` around its type; open it with fresh variables so the field
    /// arrows underneath are visible.
    pub(super) fn open_constructor_forall_chirho(&mut self, ty_chirho: TyChirho) -> TyChirho {
        let mut current_chirho = ty_chirho;
        while let TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } = current_chirho
        {
            let mut instantiation_chirho = SubstChirho::empty_chirho();
            for var_chirho in vars_chirho {
                let fresh_chirho = self.fresh_var_chirho();
                instantiation_chirho.insert_chirho(var_chirho, fresh_chirho);
            }
            current_chirho = instantiation_chirho.apply_ty_chirho(&body_chirho);
        }
        current_chirho
    }
}
