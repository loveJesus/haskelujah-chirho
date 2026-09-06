// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rigid type variables in inference: skolemizing a signature for checking,
//! GADT refinement of a rigid scrutinee, and the "could not deduce" verdict
//! for a wanted over rigid variables.
//! workflow: language-features-chirho/rigid-type-variables-chirho

use super::*;

impl InferCtxChirho {
    /// Instantiate a signature scheme for CHECKING a binding against it:
    /// every quantified variable becomes a rigid skolem, so the body may use
    /// the variable but can never decide what it is (`f :: a -> a; f x = x + 1`
    /// is an error, not `a := Int`). A variable already made rigid by an
    /// enclosing signature — a scoped type variable —
    /// keeps that resolution instead of a fresh skolem. Returns the
    /// skolemized type and the context as givens over the skolems; the new
    /// skolems are remembered in `skolem_bindings_chirho` so that a scoped
    /// type variable reappearing in an inner signature meets the same rigid
    /// constant.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub(super) fn skolemize_scheme_parts_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (TyChirho, Vec<PredChirho>) {
        let mut instantiation_chirho = SubstChirho::empty_chirho();
        for var_chirho in &scheme_chirho.vars_chirho {
            if let Some(existing_chirho) = self.skolem_bindings_chirho.lookup_chirho(var_chirho) {
                instantiation_chirho.insert_chirho(*var_chirho, existing_chirho.clone());
                continue;
            }
            // Only a variable the programmer wrote becomes rigid. A variable
            // without a source spelling comes from synonym expansion or from a
            // lowering artifact (an empty-named binder), and is instantiated
            // flexibly as before.
            let Some(base_chirho) = self
                .tyvar_source_names_chirho
                .get(var_chirho)
                .filter(|name_chirho| !name_chirho.is_empty())
                .cloned()
            else {
                let fresh_chirho = self.fresh_var_chirho();
                instantiation_chirho.insert_chirho(*var_chirho, fresh_chirho);
                continue;
            };
            let skolem_chirho = TyChirho::ForallVarChirho(skolem_name_chirho(
                &base_chirho,
                self.next_skolem_chirho,
            ));
            self.next_skolem_chirho += 1;
            instantiation_chirho.insert_chirho(*var_chirho, skolem_chirho.clone());
            self.skolem_bindings_chirho
                .insert_chirho(*var_chirho, skolem_chirho);
        }
        let givens_chirho: Vec<PredChirho> = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| {
                let mut given_chirho = PredChirho::new_chirho(
                    &pred_chirho.class_name_chirho,
                    instantiation_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                );
                given_chirho.extra_tys_chirho = pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| instantiation_chirho.apply_ty_chirho(t_chirho))
                    .collect();
                given_chirho
            })
            .collect();
        (
            instantiation_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho),
            givens_chirho,
        )
    }

    /// Unify a constructor pattern's result type with the scrutinee type.
    /// When plain unification fails and the constructor is GADT-style (its
    /// declared result refines the type parameters), try a *refinement*
    /// instead: rigid variables in the scrutinee are locally equated with what
    /// the constructor demands, recorded in the innermost scope. Returns the
    /// substitution to compose, or `None` when neither succeeds (callers keep
    /// their existing lenient path).
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub(super) fn unify_constructor_result_chirho(
        &mut self,
        refining_chirho: bool,
        con_result_chirho: &TyChirho,
        scrutinee_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Option<SubstChirho> {
        let unify_err_chirho =
            match self.unify_normalized_chirho(con_result_chirho, scrutinee_chirho, span_chirho) {
                Ok(subst_chirho) => return Some(subst_chirho),
                Err(err_chirho) => err_chirho,
            };
        if refining_chirho {
            return self.refine_skolems_by_unification_chirho(con_result_chirho, scrutinee_chirho);
        }
        // A rigid variable can never be matched by a concrete constructor
        // pattern (`f :: a -> Int; f (Just x) = 1`): the constructor's result
        // is a known type and `a` may not become it.
        let scrutinee_norm_chirho = self.normalize_ty_chirho(scrutinee_chirho);
        if matches!(&scrutinee_norm_chirho, TyChirho::ForallVarChirho(name_chirho)
            if crate::skolem_chirho::is_skolem_name_chirho(name_chirho))
        {
            self.report_unify_error_chirho(&unify_err_chirho);
        }
        None
    }

    /// A case alternative binds its pattern to a fresh type and only then
    /// meets the scrutinee; when that meeting fails, a pattern containing a
    /// GADT-style constructor (`Just Refl`, `IntE n`) may refine the
    /// scrutinee's rigid variables for the alternative.
    pub(super) fn refine_scrutinee_by_pattern_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        pat_ty_chirho: &TyChirho,
        scrutinee_chirho: &TyChirho,
    ) -> Option<SubstChirho> {
        if !self.pattern_contains_refining_constructor_chirho(pat_chirho) {
            return None;
        }
        self.refine_skolems_by_unification_chirho(pat_ty_chirho, scrutinee_chirho)
    }

    /// Whether a pattern mentions, at any depth, a constructor whose declared
    /// result type refines its type parameters (see
    /// `constructor_result_refines_chirho`).
    pub(super) fn pattern_contains_refining_constructor_chirho(
        &self,
        pat_chirho: &PatChirho,
    ) -> bool {
        match pat_chirho {
            PatChirho::ConChirho {
                con_chirho,
                args_chirho,
                ..
            } => {
                self.constructor_name_refines_chirho(con_chirho)
                    || args_chirho.iter().any(|arg_chirho| {
                        self.pattern_contains_refining_constructor_chirho(arg_chirho)
                    })
            }
            PatChirho::RecordChirho {
                con_chirho,
                fields_chirho,
                ..
            } => {
                self.constructor_name_refines_chirho(con_chirho)
                    || fields_chirho.iter().any(|field_chirho| {
                        self.pattern_contains_refining_constructor_chirho(
                            &field_chirho.pattern_chirho,
                        )
                    })
            }
            PatChirho::InfixConChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                ..
            } => {
                self.constructor_name_refines_chirho(op_chirho)
                    || self.pattern_contains_refining_constructor_chirho(left_chirho)
                    || self.pattern_contains_refining_constructor_chirho(right_chirho)
            }
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. } => {
                self.pattern_contains_refining_constructor_chirho(inner_chirho)
            }
            PatChirho::AsChirho { pattern_chirho, .. } => {
                self.pattern_contains_refining_constructor_chirho(pattern_chirho)
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            }
            | PatChirho::ListChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .any(|elem_chirho| self.pattern_contains_refining_constructor_chirho(elem_chirho)),
            _ => false,
        }
    }

    pub(super) fn constructor_name_refines_chirho(&self, con_chirho: &NameChirho) -> bool {
        self.lookup_value_scheme_with_qualified_suffix_fallback_chirho(
            &con_chirho.full_name_chirho(),
            con_chirho.text_chirho(),
        )
        .is_some_and(|scheme_chirho| {
            !is_placeholder_import_scheme_chirho(&scheme_chirho)
                && constructor_result_refines_chirho(&scheme_chirho.ty_chirho)
        })
    }

    /// Unify two types while letting rigid variables be *locally* solved: the
    /// skolems are opened into fresh unification variables, an ordinary
    /// unifier is run, and the result is split into refinements (recorded in
    /// the innermost environment scope) and ordinary bindings (returned for
    /// the caller to compose). `None` when the types cannot meet at all.
    pub(super) fn refine_skolems_by_unification_chirho(
        &mut self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
    ) -> Option<SubstChirho> {
        let left_norm_chirho = self.normalize_ty_chirho(left_chirho);
        let right_norm_chirho = self.normalize_ty_chirho(right_chirho);
        if !contains_skolem_chirho(&left_norm_chirho) && !contains_skolem_chirho(&right_norm_chirho)
        {
            return None;
        }
        let mut opened_chirho = HashMap::new();
        let mut next_var_chirho = self.next_var_chirho;
        let mut fresh_chirho = || {
            let var_chirho = TyVarChirho(next_var_chirho);
            next_var_chirho += 1;
            var_chirho
        };
        let opened_left_chirho =
            open_skolems_chirho(&left_norm_chirho, &mut opened_chirho, &mut fresh_chirho);
        let opened_right_chirho =
            open_skolems_chirho(&right_norm_chirho, &mut opened_chirho, &mut fresh_chirho);
        self.next_var_chirho = next_var_chirho;
        let unifier_chirho = self
            .unify_normalized_chirho(
                &opened_left_chirho,
                &opened_right_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .ok()?;
        let outcome_chirho = split_refinement_unifier_chirho(&unifier_chirho, &opened_chirho);
        for (skolem_chirho, ty_chirho) in outcome_chirho.refinements_chirho {
            self.env_chirho
                .add_refinement_chirho(skolem_chirho, ty_chirho);
        }
        Some(outcome_chirho.residual_subst_chirho)
    }

    /// Whether a predicate mentions a rigid (skolem) type variable.
    pub(super) fn pred_mentions_rigid_chirho(pred_chirho: &PredChirho) -> bool {
        contains_skolem_chirho(&pred_chirho.ty_chirho)
            || pred_chirho
                .extra_tys_chirho
                .iter()
                .any(contains_skolem_chirho)
    }

    /// A predicate over rigid variables that no instance and no given can
    /// ever satisfy — GHC's "could not deduce". Deliberately narrow, because
    /// instance matching here is only trusted on simple shapes: the class must
    /// be fully visible, every argument must be built from constructors,
    /// lists, tuples, arrows and rigid variables (no unification variables,
    /// type families, synonyms, promoted constructors, implicit parameters or
    /// classes used as types), a bare rigid argument is matched only by a
    /// variable-headed instance, and a matching instance's context is
    /// checked the same way.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub(super) fn rigid_pred_certainly_undeducible_chirho(&self, pred_chirho: &PredChirho) -> bool {
        self.rigid_pred_certainly_undeducible_depth_chirho(pred_chirho, 0)
    }

    pub(super) fn rigid_pred_certainly_undeducible_depth_chirho(
        &self,
        pred_chirho: &PredChirho,
        depth_chirho: usize,
    ) -> bool {
        if depth_chirho > 16
            || !self.constraint_generation_faithful_chirho
            || pred_chirho.class_name_chirho == "~"
        {
            return false;
        }
        let class_name_chirho = pred_chirho.class_name_chirho.as_str();
        if !self
            .fully_known_class_names_chirho
            .contains(class_name_chirho)
            || Self::MAGIC_CLASSES_CHIRHO.contains(&class_name_chirho)
            || class_name_chirho.starts_with('?')
        {
            return false;
        }
        let args_chirho: Vec<TyChirho> = std::iter::once(&pred_chirho.ty_chirho)
            .chain(pred_chirho.extra_tys_chirho.iter())
            .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
            .collect();
        if !args_chirho
            .iter()
            .all(|arg_chirho| self.ty_is_simple_rigid_arg_chirho(arg_chirho))
        {
            return false;
        }
        if self
            .derived_instance_heads_chirho
            .contains(&(class_name_chirho.to_string(), "*".to_string()))
        {
            return false;
        }
        let normalized_pred_chirho = PredChirho {
            class_name_chirho: pred_chirho.class_name_chirho.clone(),
            ty_chirho: args_chirho[0].clone(),
            extra_tys_chirho: args_chirho[1..].to_vec(),
        };
        let Some(instances_chirho) = self
            .class_env_chirho
            .instances_chirho
            .get(class_name_chirho)
        else {
            return true;
        };
        for inst_chirho in instances_chirho {
            let inst_tys_chirho: Vec<&TyChirho> = std::iter::once(&inst_chirho.head_ty_chirho)
                .chain(inst_chirho.extra_head_tys_chirho.iter())
                .collect();
            if inst_tys_chirho.len() != args_chirho.len() {
                continue;
            }
            let heads_compatible_chirho = inst_tys_chirho.iter().zip(args_chirho.iter()).all(
                |(inst_ty_chirho, arg_chirho)| match arg_chirho {
                    TyChirho::ForallVarChirho(_) => {
                        matches!(inst_ty_chirho, TyChirho::VarChirho(_))
                    }
                    _ => match Self::ty_outer_head_name_chirho(inst_ty_chirho) {
                        None => true,
                        Some(inst_head_chirho) => Self::ty_outer_head_name_chirho(arg_chirho)
                            .is_none_or(|arg_head_chirho| arg_head_chirho == inst_head_chirho),
                    },
                },
            );
            if !heads_compatible_chirho {
                continue;
            }
            // The instance could apply. Only its context can still refute it,
            // and only when every sub-goal is itself certainly undeducible.
            let Some(sub_goals_chirho) = self
                .class_env_chirho
                .resolve_chirho(&normalized_pred_chirho)
            else {
                return false;
            };
            let refuted_chirho = sub_goals_chirho.iter().any(|sub_goal_chirho| {
                if self.pred_entailed_by_givens_chirho(sub_goal_chirho, &self.given_preds_chirho) {
                    return false;
                }
                if Self::pred_mentions_rigid_chirho(sub_goal_chirho) {
                    self.rigid_pred_certainly_undeducible_depth_chirho(
                        sub_goal_chirho,
                        depth_chirho + 1,
                    )
                } else {
                    false
                }
            });
            if !refuted_chirho {
                return false;
            }
        }
        true
    }

    /// Whether a type is a shape on which rigid-predicate refutation is
    /// trusted: rigid variables, known constructors, lists, tuples and arrows
    /// only. Anything the solver models imprecisely disqualifies the check.
    pub(super) fn ty_is_simple_rigid_arg_chirho(&self, ty_chirho: &TyChirho) -> bool {
        match ty_chirho {
            TyChirho::ForallVarChirho(name_chirho) => {
                crate::skolem_chirho::is_skolem_name_chirho(name_chirho)
            }
            TyChirho::VarChirho(_)
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => false,
            TyChirho::ConChirho(name_chirho) => {
                !name_chirho.starts_with('\'')
                    && !name_chirho.starts_with('?')
                    && !name_chirho.ends_with('#')
                    && !self.type_families_chirho.contains_key(name_chirho)
                    && !self.type_synonyms_chirho.contains_key(name_chirho)
                    && !self
                        .class_env_chirho
                        .classes_chirho
                        .contains_key(name_chirho)
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => {
                self.ty_is_simple_rigid_arg_chirho(fun_chirho)
                    && self.ty_is_simple_rigid_arg_chirho(arg_chirho)
            }
            TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                self.ty_is_simple_rigid_arg_chirho(arg_chirho)
                    && self.ty_is_simple_rigid_arg_chirho(result_chirho)
            }
            TyChirho::ListChirho(inner_chirho) => self.ty_is_simple_rigid_arg_chirho(inner_chirho),
            TyChirho::TupleChirho(elems_chirho) => elems_chirho
                .iter()
                .all(|elem_chirho| self.ty_is_simple_rigid_arg_chirho(elem_chirho)),
        }
    }

    /// Convert a type written inside a body (an annotation `e :: t` or a
    /// visible type application `@t`): a name bound by an enclosing signature
    /// under ScopedTypeVariables denotes that signature's rigid variable;
    /// any other name is a fresh unification variable.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub(super) fn ast_type_to_ty_in_scope_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
    ) -> TyChirho {
        let mut var_map_chirho = self.scoped_tyvars_chirho.clone();
        let ty_chirho = self.ast_type_to_ty_chirho(ast_ty_chirho, &mut var_map_chirho);
        self.skolem_bindings_chirho.apply_ty_chirho(&ty_chirho)
    }

    /// Check whether an AST type has an explicit `forall` at the top level.
    /// ScopedTypeVariables is on under GHC2021 (the default edition) and off
    /// under an explicit Haskell2010 / Haskell98, unless the module says
    /// otherwise; TypeAbstractions implies it.
    pub(super) fn scoped_type_variables_enabled_chirho(module_chirho: &ModuleChirho) -> bool {
        let mut explicit_chirho: Option<bool> = None;
        let mut pre_ghc2021_chirho = false;
        for extension_chirho in &module_chirho.extensions_chirho {
            match extension_chirho.as_str() {
                "ScopedTypeVariables" | "TypeAbstractions" => explicit_chirho = Some(true),
                "NoScopedTypeVariables" => explicit_chirho = Some(false),
                "Haskell2010" | "Haskell98" => pre_ghc2021_chirho = true,
                "GHC2021" | "GHC2024" => pre_ghc2021_chirho = false,
                _ => {}
            }
        }
        explicit_chirho.unwrap_or(!pre_ghc2021_chirho)
    }
}
