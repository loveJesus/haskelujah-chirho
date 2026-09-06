// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type-changing record update (Haskell 2010 §3.15.3) and record constructor
//! field bundles for constructors declared with a context.
//! workflow: language-features-chirho/rigid-type-variables-chirho

use super::*;

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
