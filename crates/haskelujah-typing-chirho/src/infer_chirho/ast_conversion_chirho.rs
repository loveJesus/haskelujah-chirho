// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Surface type conversion with lexical scope for both forall visibilities.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};

use super::{InferCtxChirho, subst_named_var_chirho};
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum TypeConversionChirho {
    SignatureChirho,
    FamilyEquationChirho,
}

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
        for binder_chirho in binders_chirho {
            let variable_chirho = TyVarChirho(self.next_var_chirho);
            self.next_var_chirho += 1;
            variables_chirho.insert(binder_chirho.text_chirho().to_owned(), variable_chirho);
        }
        let mut result_chirho =
            self.data_head_kind_arguments_chirho(name_chirho, &mut variables_chirho);
        for binder_chirho in binders_chirho {
            if binder_chirho.is_visible_chirho() {
                let variable_chirho = variables_chirho[binder_chirho.text_chirho()];
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
        self.ast_type_with_policy_chirho(
            ast_ty_chirho,
            var_map_chirho,
            TypeConversionChirho::SignatureChirho,
        )
    }

    /// Equation syntax shares solved nominal arguments with signatures, but
    /// must not reduce a family while constructing its own stored matching row.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn ast_type_with_policy_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        policy_chirho: TypeConversionChirho,
    ) -> TyChirho {
        if let Some(ty_chirho) =
            self.elaborated_head_application_chirho(ast_ty_chirho, var_map_chirho, policy_chirho)
        {
            return self.expand_source_type_synonyms_chirho(&ty_chirho);
        }
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
                self.expand_source_type_synonyms_chirho(&raw_chirho)
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let f_chirho =
                    self.ast_type_with_policy_chirho(fun_chirho, var_map_chirho, policy_chirho);
                let a_chirho =
                    self.ast_type_with_policy_chirho(arg_chirho, var_map_chirho, policy_chirho);
                let raw_chirho = TyChirho::AppChirho(Box::new(f_chirho), Box::new(a_chirho));
                // Expand parameterised type synonyms (e.g. Pair Int → (Int, Int))
                let expanded_chirho = self.expand_source_type_synonyms_chirho(&raw_chirho);
                // Reduce type family applications (e.g. F Int → Bool)
                if policy_chirho == TypeConversionChirho::FamilyEquationChirho {
                    expanded_chirho
                } else {
                    self.reduce_type_families_in_ty_chirho(&expanded_chirho)
                }
            }
            TypeChirho::KindAppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => TyChirho::KindAppChirho(
                Box::new(self.ast_type_with_policy_chirho(
                    fun_chirho,
                    var_map_chirho,
                    policy_chirho,
                )),
                Box::new(self.ast_type_with_policy_chirho(
                    arg_chirho,
                    var_map_chirho,
                    policy_chirho,
                )),
            ),
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                let a_chirho =
                    self.ast_type_with_policy_chirho(arg_chirho, var_map_chirho, policy_chirho);
                let r_chirho =
                    self.ast_type_with_policy_chirho(result_chirho, var_map_chirho, policy_chirho);
                let m_chirho = if mult_chirho.as_ref().is_some_and(
                    haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::is_explicit_one_chirho,
                ) {
                    MultChirho::OneChirho
                } else {
                    MultChirho::ManyChirho
                };
                TyChirho::FunChirho(Box::new(a_chirho), Box::new(r_chirho), m_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let elems_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|e_chirho| {
                        self.ast_type_with_policy_chirho(e_chirho, var_map_chirho, policy_chirho)
                    })
                    .collect();
                TyChirho::TupleChirho(elems_chirho)
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                let elem_chirho =
                    self.ast_type_with_policy_chirho(element_chirho, var_map_chirho, policy_chirho);
                TyChirho::ListChirho(Box::new(elem_chirho))
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.ast_type_with_policy_chirho(inner_chirho, var_map_chirho, policy_chirho)
            }
            TypeChirho::QualChirho { body_chirho, .. } => {
                // Qualified types: convert the body, constraints are handled separately
                self.ast_type_with_policy_chirho(body_chirho, var_map_chirho, policy_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let (bound_vars_chirho, body_ty_chirho) = self.ast_forall_body_chirho(
                    vars_chirho,
                    body_chirho,
                    var_map_chirho,
                    policy_chirho,
                );
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
                let (bound_vars_chirho, body_ty_chirho) = self.ast_forall_body_chirho(
                    vars_chirho,
                    body_chirho,
                    var_map_chirho,
                    policy_chirho,
                );
                TyChirho::RequiredForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(body_ty_chirho),
                }
            }
            // DataKinds: promoted constructor is a type-level constant
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                promoted_constructor_type_chirho(name_chirho)
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
                        let elem_ty_chirho = self.ast_type_with_policy_chirho(
                            elem_chirho,
                            var_map_chirho,
                            policy_chirho,
                        );
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
                if policy_chirho == TypeConversionChirho::FamilyEquationChirho {
                    return fresh_ty_chirho;
                }
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
        policy_chirho: TypeConversionChirho,
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
                context_chirho.ast_type_with_policy_chirho(body_chirho, map_chirho, policy_chirho)
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

/// Convert stored synonym/equation syntax without inventing a type constructor
/// for an anonymous pattern. Workflow: declaration-kinds-chirho.
#[derive(Default)]
pub(super) struct SynonymTypeConverterChirho {
    next_wildcard_chirho: usize,
}

pub(super) fn ast_type_to_syn_rhs_chirho(
    ty_chirho: &TypeChirho,
    params_chirho: &[String],
) -> TyChirho {
    SynonymTypeConverterChirho::default().convert_chirho(ty_chirho, params_chirho)
}

impl SynonymTypeConverterChirho {
    pub(super) fn convert_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
        params_chirho: &[String],
    ) -> TyChirho {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho().to_string();
                if params_chirho.contains(&text_chirho) {
                    TyChirho::ForallVarChirho(text_chirho)
                } else {
                    // Unknown type variable — treat as Con (might be a bug upstream)
                    TyChirho::ConChirho(text_chirho)
                }
            }
            TypeChirho::ConChirho(name_chirho) => {
                TyChirho::ConChirho(name_chirho.full_name_chirho())
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                TyChirho::ListChirho(Box::new(self.convert_chirho(element_chirho, params_chirho)))
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                let m_chirho = if mult_chirho.as_ref().is_some_and(
                    haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::is_explicit_one_chirho,
                ) {
                    MultChirho::OneChirho
                } else {
                    MultChirho::ManyChirho
                };
                TyChirho::FunChirho(
                    Box::new(self.convert_chirho(arg_chirho, params_chirho)),
                    Box::new(self.convert_chirho(result_chirho, params_chirho)),
                    m_chirho,
                )
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|e_chirho| self.convert_chirho(e_chirho, params_chirho))
                    .collect(),
            ),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => TyChirho::AppChirho(
                Box::new(self.convert_chirho(fun_chirho, params_chirho)),
                Box::new(self.convert_chirho(arg_chirho, params_chirho)),
            ),
            TypeChirho::KindAppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => TyChirho::KindAppChirho(
                Box::new(self.convert_chirho(fun_chirho, params_chirho)),
                Box::new(self.convert_chirho(arg_chirho, params_chirho)),
            ),
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.convert_chirho(inner_chirho, params_chirho)
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                // Preserve constraint context but convert the body.
                // For now, just pass through the body since constraints in
                // synonym RHS are handled during instantiation.
                // TODO: properly preserve constraints as deferred predicates
                let _ = context_chirho;
                self.convert_chirho(body_chirho, params_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                // Preserve the forall structure in the synonym RHS so that
                // higher-rank type synonyms like `type Locker = forall a. IO a -> IO a`
                // retain their polymorphic structure when expanded.
                let forall_var_names_chirho: Vec<String> = vars_chirho
                    .iter()
                    .map(|v_chirho| v_chirho.text_chirho().to_string())
                    .collect();
                // Extend params with forall-bound vars so they become ForallVarChirho
                let mut extended_params_chirho = params_chirho.to_vec();
                extended_params_chirho.extend(forall_var_names_chirho.iter().cloned());
                let body_ty_chirho = self.convert_chirho(body_chirho, &extended_params_chirho);
                // Create ForallChirho with TyVarChirho identifiers for the bound vars
                let bound_vars_chirho: Vec<TyVarChirho> = forall_var_names_chirho
                    .iter()
                    .enumerate()
                    .map(|(i_chirho, _)| TyVarChirho(8000 + i_chirho as u32))
                    .collect();
                // Replace ForallVarChirho names with the TyVarChirho references
                let mut renamed_body_chirho = body_ty_chirho;
                for (name_chirho, tv_chirho) in
                    forall_var_names_chirho.iter().zip(bound_vars_chirho.iter())
                {
                    renamed_body_chirho = subst_named_var_chirho(
                        &renamed_body_chirho,
                        name_chirho,
                        &TyChirho::VarChirho(*tv_chirho),
                    );
                }
                TyChirho::ForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(renamed_body_chirho),
                }
            }
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let binder_names_chirho: Vec<String> = vars_chirho
                    .iter()
                    .map(|var_chirho| var_chirho.text_chirho().to_string())
                    .collect();
                let mut extended_params_chirho = params_chirho.to_vec();
                extended_params_chirho.extend(binder_names_chirho.iter().cloned());
                let body_ty_chirho = self.convert_chirho(body_chirho, &extended_params_chirho);
                let bound_vars_chirho: Vec<TyVarChirho> = binder_names_chirho
                    .iter()
                    .enumerate()
                    .map(|(index_chirho, _)| TyVarChirho(8500 + index_chirho as u32))
                    .collect();
                let mut renamed_body_chirho = body_ty_chirho;
                for (name_chirho, ty_var_chirho) in
                    binder_names_chirho.iter().zip(bound_vars_chirho.iter())
                {
                    renamed_body_chirho = subst_named_var_chirho(
                        &renamed_body_chirho,
                        name_chirho,
                        &TyChirho::VarChirho(*ty_var_chirho),
                    );
                }
                TyChirho::RequiredForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(renamed_body_chirho),
                }
            }
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                promoted_constructor_type_chirho(name_chirho)
            }
            TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                let nil_chirho = TyChirho::ConChirho("'[]".to_string());
                elements_chirho
                    .iter()
                    .rev()
                    .fold(nil_chirho, |acc_chirho, elem_chirho| {
                        let elem_ty_chirho = self.convert_chirho(elem_chirho, params_chirho);
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
            // Compiler-only spelling cannot collide with a source type variable.
            // One converter owns the complete equation, including all its patterns.
            TypeChirho::WildcardChirho { .. } => {
                let identity_chirho = self.next_wildcard_chirho;
                self.next_wildcard_chirho += 1;
                TyChirho::ForallVarChirho(format!("$wildcard_{identity_chirho}_chirho"))
            }
            // Type-level literal (DataKinds): treat as a type-level constant.
            TypeChirho::LitChirho { value_chirho, .. } => TyChirho::ConChirho(value_chirho.clone()),
        }
    }
}

/// Promotion identifies a distinct constructor namespace in both signatures
/// and stored family equations, preserving qualification in either producer.
fn promoted_constructor_type_chirho(
    name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
) -> TyChirho {
    TyChirho::ConChirho(format!("'{}", name_chirho.full_name_chirho()))
}
