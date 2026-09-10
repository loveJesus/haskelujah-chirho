// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Surface type conversion with lexical scope for both forall visibilities.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};

use super::InferCtxChirho;
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};

impl InferCtxChirho {
    /// Open every declaration-head binder lexically, but apply only visible
    /// parameters to the constructor result. Used by data and newtype schemes
    /// (and consequently their record selectors); see declaration-kinds-chirho.
    pub(super) fn data_head_type_chirho(
        &mut self,
        name_chirho: &str,
        binders_chirho: &[AstTyVarChirho],
    ) -> (TyChirho, HashMap<String, TyVarChirho>) {
        let mut variables_chirho = HashMap::with_capacity(binders_chirho.len());
        let mut result_chirho = TyChirho::ConChirho(name_chirho.to_owned());
        for binder_chirho in binders_chirho {
            let variable_chirho = TyVarChirho(self.next_var_chirho);
            self.next_var_chirho += 1;
            variables_chirho.insert(binder_chirho.text_chirho().to_owned(), variable_chirho);
            if binder_chirho.is_visible_chirho() {
                result_chirho = TyChirho::AppChirho(
                    Box::new(result_chirho),
                    Box::new(TyChirho::VarChirho(variable_chirho)),
                );
            }
        }
        (result_chirho, variables_chirho)
    }

    /// Convert an AST `TypeChirho` (surface syntax) to an internal `TyChirho`.
    ///
    /// Named type variables are mapped to fresh unification variables via
    /// `var_map_chirho`. This ensures that `a -> a` uses the same variable
    /// for both occurrences of `a`.
    pub fn ast_type_to_ty_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyChirho {
        match ast_ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho().to_string();
                let tv_chirho = *var_map_chirho
                    .entry(text_chirho.clone())
                    .or_insert_with(|| {
                        let v_chirho = TyVarChirho(self.next_var_chirho);
                        self.next_var_chirho += 1;
                        v_chirho
                    });
                self.tyvar_source_names_chirho
                    .entry(tv_chirho)
                    .or_insert(text_chirho);
                TyChirho::VarChirho(tv_chirho)
            }
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho =
                    self.normalize_imported_type_name_chirho(&name_chirho.full_name_chirho());
                let raw_chirho = TyChirho::ConChirho(text_chirho);
                // Eagerly expand nullary type synonyms (e.g. String → [Char])
                self.expand_type_synonyms_chirho(&raw_chirho)
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let f_chirho = self.ast_type_to_ty_chirho(fun_chirho, var_map_chirho);
                let a_chirho = self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho);
                let raw_chirho = TyChirho::AppChirho(Box::new(f_chirho), Box::new(a_chirho));
                // Expand parameterised type synonyms (e.g. Pair Int → (Int, Int))
                let expanded_chirho = self.expand_type_synonyms_chirho(&raw_chirho);
                // Reduce type family applications (e.g. F Int → Bool)
                self.reduce_type_families_in_ty_chirho(&expanded_chirho)
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                let a_chirho = self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho);
                let r_chirho = self.ast_type_to_ty_chirho(result_chirho, var_map_chirho);
                let m_chirho = match mult_chirho {
                    Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho) => {
                        MultChirho::OneChirho
                    }
                    _ => MultChirho::ManyChirho,
                };
                TyChirho::FunChirho(Box::new(a_chirho), Box::new(r_chirho), m_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let elems_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|e_chirho| self.ast_type_to_ty_chirho(e_chirho, var_map_chirho))
                    .collect();
                TyChirho::TupleChirho(elems_chirho)
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                let elem_chirho = self.ast_type_to_ty_chirho(element_chirho, var_map_chirho);
                TyChirho::ListChirho(Box::new(elem_chirho))
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.ast_type_to_ty_chirho(inner_chirho, var_map_chirho)
            }
            TypeChirho::QualChirho { body_chirho, .. } => {
                // Qualified types: convert the body, constraints are handled separately
                self.ast_type_to_ty_chirho(body_chirho, var_map_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let (bound_vars_chirho, body_ty_chirho) =
                    self.ast_forall_body_chirho(vars_chirho, body_chirho, var_map_chirho);
                TyChirho::ForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(body_ty_chirho),
                }
            }
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let (bound_vars_chirho, body_ty_chirho) =
                    self.ast_forall_body_chirho(vars_chirho, body_chirho, var_map_chirho);
                TyChirho::RequiredForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(body_ty_chirho),
                }
            }
            // DataKinds: promoted constructor is a type-level constant
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                TyChirho::ConChirho(name_chirho.full_name_chirho())
            }
            // DataKinds: promoted list is represented as nested type application
            TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                // '[] → Con("'[]"), '[a, b] → App(App(Con("':"), a), App(App(Con("':"), b), Con("'[]")))
                let nil_chirho = TyChirho::ConChirho("'[]".to_string());
                elements_chirho
                    .iter()
                    .rev()
                    .fold(nil_chirho, |acc_chirho, elem_chirho| {
                        let elem_ty_chirho =
                            self.ast_type_to_ty_chirho(elem_chirho, var_map_chirho);
                        let cons_chirho = TyChirho::ConChirho("':".to_string());
                        TyChirho::AppChirho(
                            Box::new(TyChirho::AppChirho(
                                Box::new(cons_chirho),
                                Box::new(elem_ty_chirho),
                            )),
                            Box::new(acc_chirho),
                        )
                    })
            }
            // PartialTypeSignatures: `_` in a type signature is a wildcard that
            // becomes a fresh unification variable. Emit warning W4201 so the
            // programmer is informed of the inferred type position.
            TypeChirho::WildcardChirho { span_chirho } => {
                let fresh_ty_chirho = self.fresh_var_chirho();
                let diag_chirho = DiagnosticChirho::warning_with_code_chirho(
                    ErrorCodeChirho::warning_chirho(4201),
                    "found wildcard `_` in type signature (PartialTypeSignatures)".to_string(),
                    *span_chirho,
                );
                self.diagnostics_chirho.push_chirho(diag_chirho);
                fresh_ty_chirho
            }
            // Type-level literal (DataKinds): treat as a type-level constant.
            TypeChirho::LitChirho { value_chirho, .. } => TyChirho::ConChirho(value_chirho.clone()),
        }
    }

    /// Introduce a lexical scope without losing free variables discovered in its body.
    /// Workflow: language-features-chirho/rank-n-visible-type-application-chirho.md.
    ///
    /// Saving only the shadowed entries bounds extra work by the binder count,
    /// not by the accumulated environment. Restore in reverse order so repeated
    /// names also unwind as a stack. Fresh identities and their source names
    /// remain valid in the returned type after the lexical name scope closes.
    fn ast_forall_body_chirho(
        &mut self,
        vars_chirho: &[AstTyVarChirho],
        body_chirho: &TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> (Vec<TyVarChirho>, TyChirho) {
        let mut bound_vars_chirho = Vec::with_capacity(vars_chirho.len());
        for var_chirho in vars_chirho {
            let name_chirho = var_chirho.text_chirho().to_string();
            let fresh_chirho = TyVarChirho(self.next_var_chirho);
            self.next_var_chirho += 1;
            self.tyvar_source_names_chirho
                .insert(fresh_chirho, name_chirho);
            bound_vars_chirho.push(fresh_chirho);
        }
        let bindings_chirho = vars_chirho
            .iter()
            .zip(bound_vars_chirho.iter())
            .map(|(name_chirho, var_chirho)| (name_chirho.text_chirho().to_string(), *var_chirho));
        let body_ty_chirho = self.with_type_bindings_chirho(
            bindings_chirho,
            var_map_chirho,
            |context_chirho, map_chirho| {
                context_chirho.ast_type_to_ty_chirho(body_chirho, map_chirho)
            },
        );
        (bound_vars_chirho, body_ty_chirho)
    }

    /// Scope fresh binders during conversion, or existing binders when attaching
    /// predicates to the already-converted signature result spine.
    pub(super) fn with_type_bindings_chirho<ResultChirho>(
        &mut self,
        bindings_chirho: impl IntoIterator<Item = (String, TyVarChirho)>,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        action_chirho: impl FnOnce(&mut Self, &mut HashMap<String, TyVarChirho>) -> ResultChirho,
    ) -> ResultChirho {
        let prior_bindings_chirho: Vec<_> = bindings_chirho
            .into_iter()
            .map(|(name_chirho, var_chirho)| {
                let prior_chirho = var_map_chirho.insert(name_chirho.clone(), var_chirho);
                (name_chirho, prior_chirho)
            })
            .collect();
        let result_chirho = action_chirho(self, var_map_chirho);
        for (name_chirho, prior_chirho) in prior_bindings_chirho.into_iter().rev() {
            if let Some(prior_chirho) = prior_chirho {
                var_map_chirho.insert(name_chirho, prior_chirho);
            } else {
                var_map_chirho.remove(&name_chirho);
            }
        }
        result_chirho
    }
}
