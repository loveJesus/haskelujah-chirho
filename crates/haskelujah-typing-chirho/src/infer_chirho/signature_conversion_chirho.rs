// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Signature schemes, predicate conversion, and signature-level binder scope.
//! Workflow: language-features-chirho/rank-n-visible-type-application-chirho.md.

use std::collections::HashMap;

use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho as AstConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

use super::{InferCtxChirho, collect_app_spine_chirho, signature_binders_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

impl InferCtxChirho {
    /// Convert an AST `TypeChirho` to a `SchemeChirho` (with quantified variables
    /// and constraints extracted).
    pub fn ast_type_to_scheme_chirho(&mut self, ast_ty_chirho: &TypeChirho) -> SchemeChirho {
        self.ast_type_to_scheme_with_var_map_chirho(ast_ty_chirho).0
    }

    /// `ast_type_to_scheme_chirho`, also returning a name-to-variable map.
    /// The first syntactically outermost forall group's identities are retained
    /// for `scope_signature_for_body_chirho`; callers must use that operation,
    /// not export this entire map into the definition's lexical scope.
    pub fn ast_type_to_scheme_with_var_map_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
    ) -> (SchemeChirho, HashMap<String, TyVarChirho>) {
        let seed_chirho = self.scoped_tyvars_chirho.clone();
        self.ast_type_to_scheme_seeded_chirho(ast_ty_chirho, &seed_chirho, true)
    }

    /// Convert a signature to a scheme with `seed_chirho` pre-binding some
    /// type-variable names. When `seeded_are_bound_chirho` is true the seeded
    /// variables belong to an enclosing scope (ScopedTypeVariables) and are
    /// never quantified here; when false they are ordinary binders of this
    /// scheme (a class method signature quantifies its class's variables).
    pub(super) fn ast_type_to_scheme_seeded_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        seed_chirho: &HashMap<String, TyVarChirho>,
        seeded_are_bound_chirho: bool,
    ) -> (SchemeChirho, HashMap<String, TyVarChirho>) {
        // Seed var_map with scoped type variables so that where-clause annotations
        // referring to the enclosing function's forall-bound vars reuse the same TyVarChirho.
        let mut var_map_chirho: HashMap<String, TyVarChirho> = seed_chirho.clone();
        let mut explicit_forall_vars_chirho = Vec::new();
        let mut scheme_preds_chirho = Vec::new();
        let mut equality_pairs_chirho = Vec::new();
        let body_ast_chirho = self.prepare_signature_body_chirho(
            ast_ty_chirho,
            &mut var_map_chirho,
            &mut explicit_forall_vars_chirho,
            &mut scheme_preds_chirho,
            &mut equality_pairs_chirho,
        );
        // Only the first syntactically outermost forall scopes the RHS. A
        // later leading forall can shadow its spelling within the signature,
        // so retain the first binder identities before the returned map is used.
        let body_bindings_chirho: Vec<_> = match ast_ty_chirho {
            TypeChirho::ForallChirho { vars_chirho, .. } => vars_chirho
                .iter()
                .zip(explicit_forall_vars_chirho.iter())
                .map(|(name_chirho, var_chirho)| {
                    (name_chirho.text_chirho().to_string(), *var_chirho)
                })
                .collect(),
            _ => Vec::new(),
        };
        let ty_chirho = self.ast_type_to_ty_chirho(body_ast_chirho, &mut var_map_chirho);
        self.collect_result_spine_constraints_chirho(
            body_ast_chirho,
            &ty_chirho,
            &mut var_map_chirho,
            &mut scheme_preds_chirho,
            &mut equality_pairs_chirho,
        );

        // Apply equality constraints: for `(a ~ b) => T`, unify `a` and `b`
        // so the scheme body reflects the equality. Keep a local substitution
        // because these variables can be bound by the outer source `forall`;
        // applying only to the global inference state does not rewrite the
        // scheme we are about to return.
        let mut equality_subst_chirho = SubstChirho::empty_chirho();
        for (lhs_chirho, rhs_chirho) in &equality_pairs_chirho {
            let lhs_sub_chirho = equality_subst_chirho.apply_ty_chirho(lhs_chirho);
            let rhs_sub_chirho = equality_subst_chirho.apply_ty_chirho(rhs_chirho);
            // An equality that plain unification cannot decide here — one side
            // is a stuck type-family application (`F a ~ Bool`) — is kept as
            // a `~` predicate: a given rewrite rule when the signature is
            // checked, a deferred equality when it is used.
            if Self::ty_is_type_family_app_chirho(&self.normalize_ty_chirho(&lhs_sub_chirho))
                || Self::ty_is_type_family_app_chirho(&self.normalize_ty_chirho(&rhs_sub_chirho))
                || self.ty_is_family_app_chirho(&lhs_sub_chirho)
                || self.ty_is_family_app_chirho(&rhs_sub_chirho)
            {
                scheme_preds_chirho.push(SchemePredChirho {
                    class_name_chirho: "~".to_string(),
                    ty_chirho: lhs_sub_chirho,
                    extra_tys_chirho: vec![rhs_sub_chirho],
                });
                continue;
            }
            match self.unify_normalized_chirho(
                &lhs_sub_chirho,
                &rhs_sub_chirho,
                SpanChirho::DUMMY_CHIRHO,
            ) {
                Ok(subst_chirho) => {
                    equality_subst_chirho = subst_chirho.compose_chirho(&equality_subst_chirho);
                    self.apply_subst_all_chirho(&subst_chirho);
                }
                // `(Int ~ Bool) =>`: an insoluble given. GHC accepts the
                // binding (its body is unreachable); keep the equality so
                // the body is checked under the rewrite it states.
                Err(_err_chirho) => scheme_preds_chirho.push(SchemePredChirho {
                    class_name_chirho: "~".to_string(),
                    ty_chirho: lhs_sub_chirho,
                    extra_tys_chirho: vec![rhs_sub_chirho],
                }),
            }
        }

        // Peel off the outermost ForallChirho produced by ast_type_to_ty_chirho.
        // The vars in a top-level ForallChirho become scheme vars; nested ForallChirho
        // (inside function args) are preserved for rank-N polymorphism.
        let (outer_forall_vars_chirho, inner_ty_chirho) = Self::peel_forall_chirho(ty_chirho);
        let outer_forall_vars_chirho = if explicit_forall_vars_chirho.is_empty() {
            outer_forall_vars_chirho
        } else {
            let mut all_vars_chirho = explicit_forall_vars_chirho;
            for var_chirho in outer_forall_vars_chirho {
                if !all_vars_chirho.contains(&var_chirho) {
                    all_vars_chirho.push(var_chirho);
                }
            }
            all_vars_chirho
        };
        let inner_ty_chirho = equality_subst_chirho.apply_ty_chirho(&inner_ty_chirho);
        let scheme_preds_chirho: Vec<SchemePredChirho> = scheme_preds_chirho
            .into_iter()
            .map(|pred_chirho| SchemePredChirho {
                class_name_chirho: pred_chirho.class_name_chirho,
                ty_chirho: equality_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|ty_chirho| equality_subst_chirho.apply_ty_chirho(ty_chirho))
                    .collect(),
            })
            .collect();

        // Only quantify over vars that are free in the resulting type.
        // Vars bound by inner ForallChirho are NOT free and should not be in scheme vars.
        let free_in_ty_chirho = inner_ty_chirho.free_vars_chirho();
        // Source order is not allocation order: infix lowering moves the
        // operator before its left operand, and predicates convert after the body.
        // Workflow: language-features-chirho/rank-n-visible-type-application-chirho
        let vars_chirho = signature_binders_chirho::ordered_scheme_vars_chirho(
            ast_ty_chirho,
            &var_map_chirho,
            outer_forall_vars_chirho,
            &free_in_ty_chirho,
        );

        // ScopedTypeVariables: a variable bound by an enclosing signature (or
        // instance head) is never re-quantified here, and one already made
        // rigid is that rigid constant here too.
        // workflow: language-features-chirho/rigid-type-variables-chirho
        let scoped_ids_chirho: Vec<TyVarChirho> = if seeded_are_bound_chirho {
            seed_chirho.values().copied().collect()
        } else {
            Vec::new()
        };
        let skolem_bindings_chirho = self.skolem_bindings_chirho.clone();
        let vars_chirho: Vec<TyVarChirho> = vars_chirho
            .into_iter()
            .filter(|var_chirho| {
                !scoped_ids_chirho.contains(var_chirho)
                    && skolem_bindings_chirho.lookup_chirho(var_chirho).is_none()
            })
            .collect();
        var_map_chirho.extend(body_bindings_chirho);
        if skolem_bindings_chirho.is_empty_chirho() {
            return (
                SchemeChirho {
                    vars_chirho,
                    preds_chirho: scheme_preds_chirho,
                    ty_chirho: inner_ty_chirho,
                },
                var_map_chirho,
            );
        }
        (
            SchemeChirho {
                vars_chirho,
                preds_chirho: scheme_preds_chirho
                    .into_iter()
                    .map(|pred_chirho| SchemePredChirho {
                        class_name_chirho: pred_chirho.class_name_chirho,
                        ty_chirho: skolem_bindings_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                        extra_tys_chirho: pred_chirho
                            .extra_tys_chirho
                            .iter()
                            .map(|ty_chirho| skolem_bindings_chirho.apply_ty_chirho(ty_chirho))
                            .collect(),
                    })
                    .collect(),
                ty_chirho: skolem_bindings_chirho.apply_ty_chirho(&inner_ty_chirho),
            },
            var_map_chirho,
        )
    }

    fn convert_signature_constraint_chirho(
        &mut self,
        constraint_chirho: &AstConstraintChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        scheme_preds_chirho: &mut Vec<SchemePredChirho>,
        equality_pairs_chirho: &mut Vec<(TyChirho, TyChirho)>,
    ) {
        let AstConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } = constraint_chirho
        else {
            return;
        };
        let class_name_chirho = class_chirho.text_chirho().to_string();
        if class_name_chirho == "~" && args_chirho.len() >= 2 {
            // Type equality constraint: a ~ b
            let lhs_chirho = self.ast_type_to_ty_chirho(&args_chirho[0], var_map_chirho);
            let rhs_chirho = self.ast_type_to_ty_chirho(&args_chirho[1], var_map_chirho);
            equality_pairs_chirho.push((lhs_chirho, rhs_chirho));
        } else if let Some(expanded_preds_chirho) = self.expand_constraint_alias_pred_chirho(
            &class_name_chirho,
            args_chirho,
            var_map_chirho,
        ) {
            scheme_preds_chirho.extend(expanded_preds_chirho);
        } else {
            let pred_ty_chirho = if let Some(first_arg_chirho) = args_chirho.first() {
                self.ast_type_to_ty_chirho(first_arg_chirho, var_map_chirho)
            } else {
                self.fresh_var_chirho()
            };
            let extra_tys_chirho: Vec<TyChirho> = args_chirho
                .iter()
                .skip(1)
                .map(|arg_chirho| self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho))
                .collect();
            scheme_preds_chirho.push(SchemePredChirho {
                class_name_chirho,
                ty_chirho: pred_ty_chirho,
                extra_tys_chirho,
            });
        }
    }

    fn expand_constraint_alias_pred_chirho(
        &mut self,
        class_name_chirho: &str,
        args_chirho: &[TypeChirho],
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> Option<Vec<SchemePredChirho>> {
        self.lookup_type_synonym_chirho(class_name_chirho)?;

        let mut alias_ty_chirho = TyChirho::ConChirho(class_name_chirho.to_string());
        for arg_chirho in args_chirho {
            alias_ty_chirho = TyChirho::AppChirho(
                Box::new(alias_ty_chirho),
                Box::new(self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho)),
            );
        }

        let expanded_chirho = self.expand_type_synonyms_chirho(&alias_ty_chirho);
        let mut preds_chirho = Vec::new();
        self.scheme_preds_from_constraint_ty_chirho(&expanded_chirho, &mut preds_chirho);
        if preds_chirho.is_empty() {
            None
        } else {
            Some(preds_chirho)
        }
    }

    fn scheme_preds_from_constraint_ty_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        out_chirho: &mut Vec<SchemePredChirho>,
    ) {
        match ty_chirho {
            TyChirho::TupleChirho(elems_chirho) => {
                for elem_chirho in elems_chirho {
                    self.scheme_preds_from_constraint_ty_chirho(elem_chirho, out_chirho);
                }
            }
            TyChirho::AppChirho(_, _) => {
                let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
                let TyChirho::ConChirho(class_name_chirho) = head_chirho else {
                    return;
                };
                if class_name_chirho == "~" {
                    return;
                }
                let pred_ty_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or_else(|| self.fresh_var_chirho());
                out_chirho.push(SchemePredChirho {
                    class_name_chirho,
                    ty_chirho: pred_ty_chirho,
                    extra_tys_chirho: args_chirho.into_iter().skip(1).collect(),
                });
            }
            TyChirho::ConChirho(class_name_chirho) if class_name_chirho != "~" => {
                out_chirho.push(SchemePredChirho {
                    class_name_chirho: class_name_chirho.clone(),
                    ty_chirho: self.fresh_var_chirho(),
                    extra_tys_chirho: Vec::new(),
                });
            }
            _ => {}
        }
    }

    /// Peel off the outermost ForallChirho from a TyChirho, collecting bound vars.
    /// Returns (outer_vars, body). If no ForallChirho at top, returns (empty, original).
    pub(super) fn peel_forall_chirho(ty_chirho: TyChirho) -> (Vec<TyVarChirho>, TyChirho) {
        match ty_chirho {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let (mut inner_vars_chirho, inner_body_chirho) =
                    Self::peel_forall_chirho(*body_chirho);
                let mut all_vars_chirho = vars_chirho;
                all_vars_chirho.append(&mut inner_vars_chirho);
                (all_vars_chirho, inner_body_chirho)
            }
            other_chirho => (vec![], other_chirho),
        }
    }

    pub(super) fn scope_signature_for_body_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        var_map_chirho: &HashMap<String, TyVarChirho>,
    ) {
        if self.scoped_type_variables_chirho
            && let TypeChirho::ForallChirho { vars_chirho, .. } = ast_ty_chirho
        {
            for name_chirho in vars_chirho {
                if let Some(var_chirho) = var_map_chirho.get(name_chirho.text_chirho()) {
                    self.scoped_tyvars_chirho
                        .insert(name_chirho.text_chirho().to_string(), *var_chirho);
                }
            }
        }
    }

    /// Prepare leading quantifiers and predicates in lexical order. Parentheses
    /// are transparent for scheme quantification, but not for RHS scope export.
    /// Converting a context before entering the next forall prevents a later
    /// same-spelling binder from capturing that context's variables.
    fn prepare_signature_body_chirho<'a>(
        &mut self,
        ast_ty_chirho: &'a TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        explicit_vars_chirho: &mut Vec<TyVarChirho>,
        scheme_preds_chirho: &mut Vec<SchemePredChirho>,
        equality_pairs_chirho: &mut Vec<(TyChirho, TyChirho)>,
    ) -> &'a TypeChirho {
        match ast_ty_chirho {
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    self.convert_signature_constraint_chirho(
                        constraint_chirho,
                        var_map_chirho,
                        scheme_preds_chirho,
                        equality_pairs_chirho,
                    );
                }
                self.prepare_signature_body_chirho(
                    body_chirho,
                    var_map_chirho,
                    explicit_vars_chirho,
                    scheme_preds_chirho,
                    equality_pairs_chirho,
                )
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                for name_chirho in vars_chirho {
                    let fresh_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    let text_chirho = name_chirho.text_chirho().to_string();
                    var_map_chirho.insert(text_chirho.clone(), fresh_chirho);
                    self.tyvar_source_names_chirho
                        .insert(fresh_chirho, text_chirho);
                    explicit_vars_chirho.push(fresh_chirho);
                }
                self.prepare_signature_body_chirho(
                    body_chirho,
                    var_map_chirho,
                    explicit_vars_chirho,
                    scheme_preds_chirho,
                    equality_pairs_chirho,
                )
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => self.prepare_signature_body_chirho(
                inner_chirho,
                var_map_chirho,
                explicit_vars_chirho,
                scheme_preds_chirho,
                equality_pairs_chirho,
            ),
            other_chirho => other_chirho,
        }
    }

    /// Convert result-spine predicates under the binders of the converted type.
    /// A detached list of AST constraints loses that identity when a forall scope
    /// closes. Pairing the result spines visits each node once without snapshots
    /// of the growing environment or reallocation of quantified variables.
    fn collect_result_spine_constraints_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        ty_chirho: &TyChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        scheme_preds_chirho: &mut Vec<SchemePredChirho>,
        equality_pairs_chirho: &mut Vec<(TyChirho, TyChirho)>,
    ) {
        match (ast_ty_chirho, ty_chirho) {
            (
                TypeChirho::FunChirho { result_chirho, .. },
                TyChirho::FunChirho(_, result_ty_chirho, _),
            ) => self.collect_result_spine_constraints_chirho(
                result_chirho,
                result_ty_chirho,
                var_map_chirho,
                scheme_preds_chirho,
                equality_pairs_chirho,
            ),
            (
                TypeChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                    ..
                },
                _,
            ) => {
                for constraint_chirho in context_chirho {
                    self.convert_signature_constraint_chirho(
                        constraint_chirho,
                        var_map_chirho,
                        scheme_preds_chirho,
                        equality_pairs_chirho,
                    );
                }
                self.collect_result_spine_constraints_chirho(
                    body_chirho,
                    ty_chirho,
                    var_map_chirho,
                    scheme_preds_chirho,
                    equality_pairs_chirho,
                );
            }
            (TypeChirho::ParenChirho { inner_chirho, .. }, _) => {
                self.collect_result_spine_constraints_chirho(
                    inner_chirho,
                    ty_chirho,
                    var_map_chirho,
                    scheme_preds_chirho,
                    equality_pairs_chirho,
                );
            }
            (
                TypeChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                    ..
                },
                TyChirho::ForallChirho {
                    vars_chirho: bound_chirho,
                    body_chirho: body_ty_chirho,
                },
            )
            | (
                TypeChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                    ..
                },
                TyChirho::RequiredForallChirho {
                    vars_chirho: bound_chirho,
                    body_chirho: body_ty_chirho,
                },
            ) if vars_chirho.len() == bound_chirho.len() => {
                let bindings_chirho =
                    vars_chirho
                        .iter()
                        .zip(bound_chirho.iter())
                        .map(|(name_chirho, var_chirho)| {
                            (name_chirho.text_chirho().to_string(), *var_chirho)
                        });
                self.with_type_bindings_chirho(
                    bindings_chirho,
                    var_map_chirho,
                    |context_chirho, map_chirho| {
                        context_chirho.collect_result_spine_constraints_chirho(
                            body_chirho,
                            body_ty_chirho,
                            map_chirho,
                            scheme_preds_chirho,
                            equality_pairs_chirho,
                        );
                    },
                );
            }
            (
                TypeChirho::FunChirho { .. }
                | TypeChirho::ForallChirho { .. }
                | TypeChirho::RequiredForallChirho { .. },
                _,
            ) => {
                // The converter preserves these nodes. If that contract changes,
                // do not silently discard a constraint or attach it to another binder.
                self.diagnostics_chirho.push_chirho(
                    haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                        haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(
                            super::SIGNATURE_MISMATCH_CODE_CHIRHO,
                        ),
                        "signature result scope does not match its converted type",
                        ast_ty_chirho.span_chirho(),
                    ),
                );
            }
            _ => {}
        }
    }
}
