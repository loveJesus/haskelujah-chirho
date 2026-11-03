// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Scheme lookups, and visible or required type applications.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{canonical_value_name_text_chirho, is_placeholder_import_scheme_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::promotion_chirho::unquoted_promoted_spelling_chirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    pub(super) fn lookup_value_scheme_with_qualified_suffix_fallback_chirho(
        &self,
        full_name_chirho: &str,
        bare_name_chirho: &str,
    ) -> Option<SchemeChirho> {
        let canonical_full_name_chirho = canonical_value_name_text_chirho(full_name_chirho);
        let canonical_bare_name_chirho = canonical_value_name_text_chirho(bare_name_chirho);
        if self
            .env_chirho
            .is_masked_chirho(&canonical_full_name_chirho)
        {
            return None;
        }
        if canonical_full_name_chirho == canonical_bare_name_chirho {
            return self
                .env_chirho
                .lookup_chirho(&canonical_bare_name_chirho)
                .cloned();
        }

        if let Some(scheme_chirho) = self.env_chirho.lookup_chirho(&canonical_full_name_chirho) {
            return Some(scheme_chirho.clone());
        }

        let suffix_chirho = format!(".{canonical_full_name_chirho}");
        let mut suffix_matches_chirho = self
            .env_chirho
            .all_bindings_chirho()
            .into_iter()
            .filter(|(name_chirho, _scheme_chirho)| name_chirho.ends_with(&suffix_chirho))
            .map(|(_name_chirho, scheme_chirho)| scheme_chirho.clone());
        let first_match_chirho = suffix_matches_chirho.next();
        if first_match_chirho.is_some() && suffix_matches_chirho.next().is_none() {
            return first_match_chirho;
        }

        // A qualified name never falls back to a local binder: `L.x` beside a
        // `let x` named the local (row 598).
        self.env_chirho
            .lookup_outermost_chirho(&canonical_bare_name_chirho)
            .cloned()
    }

    pub(super) fn lookup_record_constructor_field_bundle_chirho(
        &mut self,
        con_chirho: &NameChirho,
    ) -> Option<(Vec<String>, Vec<TyChirho>, TyChirho)> {
        let con_text_chirho = con_chirho.text_chirho();
        let con_full_name_chirho = con_chirho.full_name_chirho();
        let scheme_chirho = self.lookup_value_scheme_with_qualified_suffix_fallback_chirho(
            &con_full_name_chirho,
            con_text_chirho,
        )?;
        if is_placeholder_import_scheme_chirho(&scheme_chirho) {
            return None;
        }

        let mut field_names_chirho = self
            .con_field_names_chirho
            .get(con_text_chirho)
            .cloned()
            .or_else(|| {
                self.con_field_names_chirho
                    .get(&con_full_name_chirho)
                    .cloned()
            })?;
        let instantiated_chirho = self.instantiate_chirho(&scheme_chirho, con_chirho.span_chirho());
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

    fn peel_type_app_chain_chirho<'a>(
        expr_chirho: &'a ExprChirho,
        type_args_chirho: &mut Vec<&'a TypeChirho>,
    ) -> &'a ExprChirho {
        match expr_chirho {
            ExprChirho::TypeAppChirho {
                expr_chirho,
                ty_chirho,
                ..
            } => {
                let base_chirho = Self::peel_type_app_chain_chirho(expr_chirho, type_args_chirho);
                type_args_chirho.push(ty_chirho);
                base_chirho
            }
            _ => expr_chirho,
        }
    }

    pub(super) fn infer_named_type_app_chirho(
        &mut self,
        expr_chirho: &ExprChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        let mut type_args_ast_chirho = Vec::new();
        let base_expr_chirho =
            Self::peel_type_app_chain_chirho(expr_chirho, &mut type_args_ast_chirho);
        if type_args_ast_chirho.is_empty() {
            return None;
        }

        let scheme_chirho = match base_expr_chirho {
            ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => self
                .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                    &name_chirho.full_name_chirho(),
                    name_chirho.text_chirho(),
                )?,
            _ => {
                return None;
            }
        };

        // A name known only through a placeholder import scheme has no
        // quantifiers to apply type arguments to; its type stays unknown.
        if is_placeholder_import_scheme_chirho(&scheme_chirho) {
            let fresh_chirho = self.fresh_var_chirho();
            return Some((SubstChirho::empty_chirho(), fresh_chirho));
        }
        let explicit_tys_chirho: Vec<TyChirho> = type_args_ast_chirho
            .into_iter()
            .map(|ty_chirho| self.ast_type_to_ty_in_scope_chirho(ty_chirho))
            .collect();
        let (instantiated_ty_chirho, instantiated_preds_chirho) = self
            .instantiate_scheme_parts_with_type_args_chirho(&scheme_chirho, &explicit_tys_chirho)?;
        for pred_inst_chirho in instantiated_preds_chirho {
            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
        }
        Some((SubstChirho::empty_chirho(), instantiated_ty_chirho))
    }

    pub(super) fn expr_to_required_type_arg_chirho(
        &self,
        expr_chirho: &ExprChirho,
    ) -> Option<TyChirho> {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                Some(
                    self.scoped_tyvars_chirho
                        .get(text_chirho)
                        .copied()
                        .map(TyChirho::VarChirho)
                        .unwrap_or_else(|| {
                            // A constructor here is promoted, as an unquoted name in a type
                            // is (row 686).
                            TyChirho::ConChirho(
                                unquoted_promoted_spelling_chirho(
                                    name_chirho,
                                    self.promotion_scope_chirho.as_deref(),
                                )
                                .unwrap_or_else(|| name_chirho.full_name_chirho()),
                            )
                        }),
                )
            }
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => Some(TyChirho::AppChirho(
                Box::new(self.expr_to_required_type_arg_chirho(fun_chirho)?),
                Box::new(self.expr_to_required_type_arg_chirho(arg_chirho)?),
            )),
            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                // Preserve the old reverse freshening order; the shared
                // builder receives source order and folds from the right.
                let converted_chirho = elements_chirho
                    .iter()
                    .rev()
                    .map(|element_chirho| self.expr_to_required_type_arg_chirho(element_chirho))
                    .collect::<Option<Vec<_>>>()?;
                Some(TyChirho::promoted_list_chirho(
                    converted_chirho.into_iter().rev(),
                    self.promotion_scope_chirho.is_some(),
                ))
            }
            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .map(|element_chirho| self.expr_to_required_type_arg_chirho(element_chirho))
                .collect::<Option<Vec<_>>>()
                .map(TyChirho::TupleChirho),
            ExprChirho::ParenChirho { inner_chirho, .. }
            | ExprChirho::AnnChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.expr_to_required_type_arg_chirho(inner_chirho),
            ExprChirho::LitChirho(LitChirho::IntChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(value_chirho.to_string()))
            }
            ExprChirho::LitChirho(LitChirho::StringChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(format!("\"{value_chirho}\"")))
            }
            ExprChirho::LitChirho(LitChirho::CharChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(format!("'{value_chirho}'")))
            }
            _ => None,
        }
    }

    pub(super) fn apply_required_type_arg_chirho(
        &self,
        fun_ty_chirho: &TyChirho,
        arg_ty_chirho: TyChirho,
    ) -> Option<TyChirho> {
        let TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } = fun_ty_chirho
        else {
            return None;
        };
        let (first_var_chirho, remaining_vars_chirho) = vars_chirho.split_first()?;
        let mut subst_chirho = SubstChirho::empty_chirho();
        subst_chirho.insert_chirho(*first_var_chirho, arg_ty_chirho);
        let body_chirho = subst_chirho.apply_ty_chirho(body_chirho);
        if remaining_vars_chirho.is_empty() {
            Some(body_chirho)
        } else {
            Some(TyChirho::RequiredForallChirho {
                vars_chirho: remaining_vars_chirho.to_vec(),
                body_chirho: Box::new(body_chirho),
            })
        }
    }

    pub(super) fn apply_visible_type_arg_chirho(
        &self,
        fun_ty_chirho: &TyChirho,
        arg_ty_chirho: TyChirho,
    ) -> Option<TyChirho> {
        let TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } = fun_ty_chirho
        else {
            return None;
        };
        let (first_var_chirho, remaining_vars_chirho) = vars_chirho.split_first()?;
        let mut subst_chirho = SubstChirho::empty_chirho();
        subst_chirho.insert_chirho(*first_var_chirho, arg_ty_chirho);
        let body_chirho = subst_chirho.apply_ty_chirho(body_chirho);
        if remaining_vars_chirho.is_empty() {
            Some(body_chirho)
        } else {
            Some(TyChirho::ForallChirho {
                vars_chirho: remaining_vars_chirho.to_vec(),
                body_chirho: Box::new(body_chirho),
            })
        }
    }
}
