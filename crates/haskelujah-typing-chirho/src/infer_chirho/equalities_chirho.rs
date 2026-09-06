// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Given equalities as rewrite rules, deferred stuck-family equalities,
//! and discharge of wanteds under the givens in scope (through instances and
//! functional dependencies).
//! workflow: language-features-chirho/stuck-family-equalities-chirho

use super::*;

impl InferCtxChirho {
    /// Builtin type families that reduce only on literals; an application to
    /// an unsolved variable is stuck, not a mismatch.
    pub(super) const BUILTIN_TYPE_FAMILIES_CHIRHO: &'static [&'static str] = &[
        "+",
        "-",
        "*",
        "^",
        "<=?",
        "Div",
        "Mod",
        "Log2",
        "AppendSymbol",
        "CharToNat",
        "NatToChar",
        "CmpNat",
        "CmpSymbol",
    ];

    /// Whether `name_chirho` names a type family known here (declared,
    /// imported, or builtin).
    pub(super) fn is_type_family_name_chirho(&self, name_chirho: &str) -> bool {
        if self.type_families_chirho.contains_key(name_chirho) {
            return true;
        }
        let canonical_chirho = canonical_type_family_name_chirho(name_chirho);
        Self::BUILTIN_TYPE_FAMILIES_CHIRHO.contains(&canonical_chirho.as_str())
            || self
                .type_families_chirho
                .contains_key(canonical_chirho.as_str())
            || self
                .type_families_chirho
                .keys()
                .any(|key_chirho| strip_name_qualifier_chirho(key_chirho) == canonical_chirho)
    }

    /// Whether the type is an application headed by a known type family.
    pub(super) fn ty_is_family_app_chirho(&self, ty_chirho: &TyChirho) -> bool {
        let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
        match &head_chirho {
            TyChirho::ConChirho(name_chirho) => {
                !args_chirho.is_empty() && self.is_type_family_name_chirho(name_chirho)
                    || (args_chirho.is_empty()
                        && self.type_families_chirho.contains_key(name_chirho))
            }
            _ => false,
        }
    }

    /// Syntactic shape only (no environment): an application whose head is a
    /// builtin arithmetic / symbol family.
    pub(super) fn ty_is_type_family_app_chirho(ty_chirho: &TyChirho) -> bool {
        let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
        match &head_chirho {
            TyChirho::ConChirho(name_chirho) => {
                !args_chirho.is_empty()
                    && Self::BUILTIN_TYPE_FAMILIES_CHIRHO
                        .contains(&canonical_type_family_name_chirho(name_chirho).as_str())
            }
            _ => false,
        }
    }

    /// Whether a type family application occurs anywhere in the type.
    pub(super) fn ty_mentions_family_chirho(&self, ty_chirho: &TyChirho) -> bool {
        if self.ty_is_family_app_chirho(ty_chirho) {
            return true;
        }
        match ty_chirho {
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => false,
            TyChirho::AppChirho(fun_chirho, arg_chirho) => {
                self.ty_mentions_family_chirho(fun_chirho)
                    || self.ty_mentions_family_chirho(arg_chirho)
            }
            TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                self.ty_mentions_family_chirho(arg_chirho)
                    || self.ty_mentions_family_chirho(result_chirho)
            }
            TyChirho::TupleChirho(elems_chirho) => elems_chirho
                .iter()
                .any(|elem_chirho| self.ty_mentions_family_chirho(elem_chirho)),
            TyChirho::ListChirho(inner_chirho) => self.ty_mentions_family_chirho(inner_chirho),
            TyChirho::ForallChirho { body_chirho, .. }
            | TyChirho::RequiredForallChirho { body_chirho, .. } => {
                self.ty_mentions_family_chirho(body_chirho)
            }
        }
    }

    /// A family application that cannot reduce because an argument is still
    /// an unsolved unification variable: the equality it takes part in must
    /// wait, not fail.
    pub(super) fn ty_is_stuck_family_on_var_chirho(&self, ty_chirho: &TyChirho) -> bool {
        self.ty_is_family_app_chirho(ty_chirho) && ty_chirho.contains_var_chirho()
    }

    /// Rewrite every subterm equal to a given equality's left side to its
    /// right side (one pass; normalization iterates).
    pub(super) fn apply_given_rewrites_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        if self.given_rewrites_chirho.is_empty() {
            return ty_chirho.clone();
        }
        for (lhs_chirho, rhs_chirho) in &self.given_rewrites_chirho {
            if ty_chirho == lhs_chirho {
                return rhs_chirho.clone();
            }
        }
        match ty_chirho {
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {
                ty_chirho.clone()
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
                Box::new(self.apply_given_rewrites_chirho(fun_chirho)),
                Box::new(self.apply_given_rewrites_chirho(arg_chirho)),
            ),
            TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.apply_given_rewrites_chirho(arg_chirho)),
                Box::new(self.apply_given_rewrites_chirho(result_chirho)),
                *mult_chirho,
            ),
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|elem_chirho| self.apply_given_rewrites_chirho(elem_chirho))
                    .collect(),
            ),
            TyChirho::ListChirho(inner_chirho) => {
                TyChirho::ListChirho(Box::new(self.apply_given_rewrites_chirho(inner_chirho)))
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.apply_given_rewrites_chirho(body_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.apply_given_rewrites_chirho(body_chirho)),
            },
        }
    }

    /// Turn the `~` givens of a signature into rewrite rules. A rule rewrites
    /// the side that cannot otherwise be simplified — a rigid variable or a
    /// stuck family application — to the other side. The type-nat solver's
    /// interaction rule for `+` is applied to the givens: `a + b ~ a + c`
    /// (directly, or through a shared right-hand side) yields `b ~ c`.
    /// workflow: language-features-chirho/stuck-family-equalities-chirho
    pub(super) fn install_given_equalities_chirho(&mut self, preds_chirho: &[PredChirho]) {
        let mut equalities_chirho: Vec<(TyChirho, TyChirho)> = preds_chirho
            .iter()
            .filter(|pred_chirho| pred_chirho.class_name_chirho == "~")
            .filter_map(|pred_chirho| {
                pred_chirho
                    .extra_tys_chirho
                    .first()
                    .map(|rhs_chirho| (pred_chirho.ty_chirho.clone(), rhs_chirho.clone()))
            })
            .collect();
        if equalities_chirho.is_empty() {
            return;
        }
        // Interaction of `+` givens, to a small fixpoint.
        let mut changed_chirho = true;
        let mut rounds_chirho = 0;
        while changed_chirho && rounds_chirho < 8 {
            changed_chirho = false;
            rounds_chirho += 1;
            let snapshot_chirho = equalities_chirho.clone();
            for (index_chirho, (lhs_chirho, rhs_chirho)) in snapshot_chirho.iter().enumerate() {
                let mut candidates_chirho: Vec<(TyChirho, TyChirho)> = Vec::new();
                if let Some(derived_chirho) = Self::plus_interaction_chirho(lhs_chirho, rhs_chirho)
                {
                    candidates_chirho.push(derived_chirho);
                }
                for (other_lhs_chirho, other_rhs_chirho) in
                    snapshot_chirho.iter().skip(index_chirho + 1)
                {
                    if other_rhs_chirho == rhs_chirho
                        && let Some(derived_chirho) =
                            Self::plus_interaction_chirho(lhs_chirho, other_lhs_chirho)
                    {
                        candidates_chirho.push(derived_chirho);
                    }
                }
                for candidate_chirho in candidates_chirho {
                    if candidate_chirho.0 != candidate_chirho.1
                        && !equalities_chirho.contains(&candidate_chirho)
                    {
                        equalities_chirho.push(candidate_chirho);
                        changed_chirho = true;
                    }
                }
            }
        }
        for (lhs_chirho, rhs_chirho) in equalities_chirho {
            let lhs_norm_chirho = self.normalize_ty_chirho(&lhs_chirho);
            let rhs_norm_chirho = self.normalize_ty_chirho(&rhs_chirho);
            if lhs_norm_chirho == rhs_norm_chirho {
                continue;
            }
            let lhs_rigid_chirho = matches!(&lhs_norm_chirho, TyChirho::ForallVarChirho(_))
                || self.ty_is_family_app_chirho(&lhs_norm_chirho);
            let rhs_rigid_chirho = matches!(&rhs_norm_chirho, TyChirho::ForallVarChirho(_))
                || self.ty_is_family_app_chirho(&rhs_norm_chirho);
            // A rule whose right side contains its own left side
            // (`b ~ Maybe (Foo b)`) would rewrite forever; such a given is
            // consistent but is not usable as a rewrite.
            let rule_chirho = if lhs_rigid_chirho
                && !ty_contains_subterm_chirho(&rhs_norm_chirho, &lhs_norm_chirho)
            {
                Some((lhs_norm_chirho, rhs_norm_chirho))
            } else if rhs_rigid_chirho
                && !ty_contains_subterm_chirho(&lhs_norm_chirho, &rhs_norm_chirho)
            {
                Some((rhs_norm_chirho, lhs_norm_chirho))
            } else if !lhs_rigid_chirho
                && !rhs_rigid_chirho
                && !lhs_norm_chirho.contains_var_chirho()
                && !rhs_norm_chirho.contains_var_chirho()
            {
                // Two closed types that differ (`Int ~ Bool`): an insoluble
                // given; the body is typed under the stated equality.
                Some((lhs_norm_chirho, rhs_norm_chirho))
            } else {
                None
            };
            if let Some(rule_chirho) = rule_chirho {
                self.given_rewrites_chirho.push(rule_chirho);
            }
        }
    }

    /// `x + y ~ x + z` gives `y ~ z`; `y + x ~ z + x` gives `y ~ z`.
    pub(super) fn plus_interaction_chirho(
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
    ) -> Option<(TyChirho, TyChirho)> {
        let (left_head_chirho, left_args_chirho) = collect_app_spine_chirho(left_chirho);
        let (right_head_chirho, right_args_chirho) = collect_app_spine_chirho(right_chirho);
        let is_plus_chirho = |head_chirho: &TyChirho| {
            matches!(head_chirho, TyChirho::ConChirho(name_chirho)
                if canonical_type_family_name_chirho(name_chirho) == "+")
        };
        if !is_plus_chirho(&left_head_chirho)
            || !is_plus_chirho(&right_head_chirho)
            || left_args_chirho.len() != 2
            || right_args_chirho.len() != 2
        {
            return None;
        }
        if left_args_chirho[0] == right_args_chirho[0] {
            return Some((left_args_chirho[1].clone(), right_args_chirho[1].clone()));
        }
        if left_args_chirho[1] == right_args_chirho[1] {
            return Some((left_args_chirho[0].clone(), right_args_chirho[0].clone()));
        }
        None
    }

    /// At the end of the module, an equality still stuck on an unsolved
    /// variable whose other side is a concrete type (no variables, rigid or
    /// not) is GHC's "couldn't match" on an ambiguous family application
    /// (`EraName era0 ~ "Blah"`): report it. A rigid or open other side is
    /// left alone: the variable may be one this solver simply failed to
    /// determine, and ambiguity is not modelled.
    pub(super) fn report_unsolved_deferred_equalities_chirho(&mut self) {
        let pending_chirho = std::mem::take(&mut self.deferred_equalities_chirho);
        for (left_chirho, right_chirho, span_chirho) in pending_chirho {
            let left_norm_chirho = self.normalize_ty_chirho(&left_chirho);
            let right_norm_chirho = self.normalize_ty_chirho(&right_chirho);
            if left_norm_chirho == right_norm_chirho
                || Self::contains_polymorphic_component_chirho(&left_norm_chirho)
                || Self::contains_polymorphic_component_chirho(&right_norm_chirho)
            {
                continue;
            }
            let left_closed_chirho = !left_norm_chirho.contains_var_chirho()
                && !contains_skolem_chirho(&left_norm_chirho);
            let right_closed_chirho = !right_norm_chirho.contains_var_chirho()
                && !contains_skolem_chirho(&right_norm_chirho);
            if left_closed_chirho || right_closed_chirho {
                self.report_unify_error_chirho(&UnifyErrorChirho::MismatchChirho {
                    expected_chirho: left_norm_chirho,
                    actual_chirho: right_norm_chirho,
                    span_chirho,
                });
            }
        }
    }

    /// Retry the deferred equalities: any whose stuck side has since been
    /// solved (or is now rewritten by a given) is unified now; a definite
    /// mismatch is reported at the span where it arose. Returns the
    /// substitution the successful ones produced (already applied to the
    /// environment).
    pub(super) fn retry_deferred_equalities_chirho(&mut self) -> SubstChirho {
        let mut total_chirho = SubstChirho::empty_chirho();
        for _round_chirho in 0..8 {
            if self.deferred_equalities_chirho.is_empty() {
                break;
            }
            let pending_chirho = std::mem::take(&mut self.deferred_equalities_chirho);
            let mut progressed_chirho = false;
            let mut still_pending_chirho = Vec::new();
            for (left_chirho, right_chirho, span_chirho) in pending_chirho {
                let left_sub_chirho =
                    self.normalize_ty_chirho(&total_chirho.apply_ty_chirho(&left_chirho));
                let right_sub_chirho =
                    self.normalize_ty_chirho(&total_chirho.apply_ty_chirho(&right_chirho));
                if left_sub_chirho == right_sub_chirho {
                    progressed_chirho = true;
                    continue;
                }
                // A family applied to a polytype (`TF (forall b. …) ~ Y`) is an
                // impredicative instantiation this checker does not model;
                // it is neither solved nor reported.
                if Self::contains_polymorphic_component_chirho(&left_sub_chirho)
                    || Self::contains_polymorphic_component_chirho(&right_sub_chirho)
                {
                    progressed_chirho = true;
                    continue;
                }
                if self.ty_is_stuck_family_on_var_chirho(&left_sub_chirho)
                    || self.ty_is_stuck_family_on_var_chirho(&right_sub_chirho)
                {
                    still_pending_chirho.push((left_sub_chirho, right_sub_chirho, span_chirho));
                    continue;
                }
                progressed_chirho = true;
                match unify_chirho(&left_sub_chirho, &right_sub_chirho, span_chirho) {
                    Ok(subst_chirho) => {
                        self.apply_subst_all_chirho(&subst_chirho);
                        total_chirho = subst_chirho.compose_chirho(&total_chirho);
                    }
                    // `x ~ (Arg x -> Res x)` is not an infinite type: the
                    // occurrence sits under a type family, which the occurs
                    // check must not see through. Such an equality is
                    // consistent and left unsolved.
                    Err(UnifyErrorChirho::OccursCheckChirho { .. })
                        if self.ty_mentions_family_chirho(&left_sub_chirho)
                            || self.ty_mentions_family_chirho(&right_sub_chirho) => {}
                    Err(err_chirho) => self.report_unify_error_chirho(&err_chirho),
                }
            }
            // Anything deferred anew during this round comes after the survivors.
            let mut newly_chirho = std::mem::take(&mut self.deferred_equalities_chirho);
            still_pending_chirho.append(&mut newly_chirho);
            self.deferred_equalities_chirho = still_pending_chirho;
            if !progressed_chirho {
                break;
            }
        }
        total_chirho
    }

    /// Discharge deferred wanteds entailed by the givens in scope. Before
    /// that, a given of a class with functional dependencies *improves* any
    /// wanted that agrees with it on the determining positions
    /// (`MyReader r Int` given, `MyReader r v` wanted, `r -> v` ⇒ `v := Int`).
    /// The improvement is applied to the environment and deferred wanteds and
    /// returned so an inference loop can compose it into its running
    /// substitution.
    pub(super) fn discharge_deferred_with_current_givens_chirho(&mut self) -> SubstChirho {
        let givens_chirho = self.given_preds_chirho.clone();
        let mut improvement_chirho = SubstChirho::empty_chirho();
        for (pred_chirho, _span_chirho) in &self.deferred_preds_chirho {
            let pred_sub_chirho = PredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: improvement_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| improvement_chirho.apply_ty_chirho(t_chirho))
                    .collect(),
            };
            let step_chirho =
                self.fundep_improve_from_givens_chirho(&pred_sub_chirho, &givens_chirho);
            if !step_chirho.is_empty_chirho() {
                improvement_chirho = step_chirho.compose_chirho(&improvement_chirho);
            }
        }
        if !improvement_chirho.is_empty_chirho() {
            self.apply_subst_all_chirho(&improvement_chirho);
        }
        let deferred_preds_chirho = std::mem::take(&mut self.deferred_preds_chirho);
        self.deferred_preds_chirho = deferred_preds_chirho
            .into_iter()
            .filter(|(pred_chirho, _span_chirho)| {
                !self.givens_discharge_wanted_chirho(pred_chirho, &givens_chirho)
            })
            .collect();
        improvement_chirho
    }

    /// A wanted leaves the deferred list at a given scope's end only when the
    /// givens took part in solving it: directly or through a superclass,
    /// through an instance whose context they entail, or because it names a
    /// rigid variable that only they (or the rewrites they install) can
    /// serve. A wanted that instances alone settle — the `Num Int` of a
    /// literal — stays deferred: generalization absorbs it into the binding's
    /// scheme, and the dictionary pass reads such ground scheme predicates as
    /// its evidence for the binding's body. Discharging it here would leave
    /// that pass with a bare class method and no dictionary.
    fn givens_discharge_wanted_chirho(
        &self,
        pred_chirho: &PredChirho,
        givens_chirho: &[PredChirho],
    ) -> bool {
        match self.pred_entailment_under_givens_chirho(pred_chirho, givens_chirho, 0) {
            None => false,
            Some(true) => true,
            Some(false) => {
                contains_skolem_chirho(&pred_chirho.ty_chirho)
                    || pred_chirho
                        .extra_tys_chirho
                        .iter()
                        .any(contains_skolem_chirho)
            }
        }
    }

    /// Entailment with the givens in scope: directly by a given (or a
    /// superclass of one), or through an instance whose context is itself
    /// entailed under the givens (`New (T m) m` via
    /// `instance PrimMonad m => New (T m) m` and the given `PrimMonad m`).
    /// Solving such wanteds while their givens are in scope is what lets a
    /// signature's context serve a rigid variable at all. `None` is "not
    /// entailed (yet)"; `Some(used_given)` says whether any given took part,
    /// as opposed to instances settling the wanted on their own.
    pub(super) fn pred_entailment_under_givens_chirho(
        &self,
        pred_chirho: &PredChirho,
        givens_chirho: &[PredChirho],
        depth_chirho: usize,
    ) -> Option<bool> {
        if self.pred_entailed_by_givens_chirho(pred_chirho, givens_chirho) {
            return Some(true);
        }
        if depth_chirho > 16 || pred_chirho.class_name_chirho == "~" {
            return None;
        }
        // Only a wanted whose arguments are settled can be committed to an
        // instance here; one still carrying unification variables waits.
        if pred_chirho.ty_chirho.contains_var_chirho()
            || pred_chirho
                .extra_tys_chirho
                .iter()
                .any(|t_chirho| t_chirho.contains_var_chirho())
        {
            return None;
        }
        let normalized_chirho = PredChirho {
            class_name_chirho: pred_chirho.class_name_chirho.clone(),
            ty_chirho: self.normalize_ty_chirho(&pred_chirho.ty_chirho),
            extra_tys_chirho: pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
                .collect(),
        };
        let sub_goals_chirho = self.class_env_chirho.resolve_chirho(&normalized_chirho)?;
        let mut used_given_chirho = false;
        for sub_goal_chirho in &sub_goals_chirho {
            used_given_chirho |= self.pred_entailment_under_givens_chirho(
                sub_goal_chirho,
                givens_chirho,
                depth_chirho + 1,
            )?;
        }
        Some(used_given_chirho)
    }

    /// Improvement of a wanted by the givens of a class with functional
    /// dependencies: when a given agrees with the wanted on every position of
    /// a dependency's determining side, the determined positions must be
    /// equal, which binds the wanted's own unification variables.
    pub(super) fn fundep_improve_from_givens_chirho(
        &self,
        pred_chirho: &PredChirho,
        givens_chirho: &[PredChirho],
    ) -> SubstChirho {
        let mut improvement_chirho = SubstChirho::empty_chirho();
        let Some(class_chirho) = self
            .class_env_chirho
            .classes_chirho
            .get(&pred_chirho.class_name_chirho)
        else {
            return improvement_chirho;
        };
        if class_chirho.fundeps_chirho.is_empty() {
            return improvement_chirho;
        }
        let pred_tys_chirho: Vec<TyChirho> = pred_chirho
            .all_tys_chirho()
            .iter()
            .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
            .collect();
        let pred_vars_chirho: Vec<TyVarChirho> = pred_tys_chirho
            .iter()
            .flat_map(|t_chirho| t_chirho.free_vars_chirho())
            .collect();
        for given_chirho in givens_chirho {
            if given_chirho.class_name_chirho != pred_chirho.class_name_chirho {
                continue;
            }
            let given_tys_chirho: Vec<TyChirho> = given_chirho
                .all_tys_chirho()
                .iter()
                .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
                .collect();
            if given_tys_chirho.len() != pred_tys_chirho.len() {
                continue;
            }
            for (from_chirho, to_chirho) in &class_chirho.fundeps_chirho {
                let from_agree_chirho = from_chirho.iter().all(|&idx_chirho| {
                    idx_chirho < pred_tys_chirho.len()
                        && improvement_chirho.apply_ty_chirho(&pred_tys_chirho[idx_chirho])
                            == given_tys_chirho[idx_chirho]
                });
                if !from_agree_chirho {
                    continue;
                }
                for &idx_chirho in to_chirho {
                    if idx_chirho >= pred_tys_chirho.len() {
                        continue;
                    }
                    let wanted_ty_chirho =
                        improvement_chirho.apply_ty_chirho(&pred_tys_chirho[idx_chirho]);
                    if let Ok(unifier_chirho) = unify_chirho(
                        &wanted_ty_chirho,
                        &given_tys_chirho[idx_chirho],
                        SpanChirho::DUMMY_CHIRHO,
                    ) {
                        for (var_chirho, ty_chirho) in unifier_chirho.iter_chirho() {
                            if pred_vars_chirho.contains(var_chirho) {
                                improvement_chirho.insert_chirho(*var_chirho, ty_chirho.clone());
                            }
                        }
                    }
                }
            }
        }
        improvement_chirho
    }
}

/// Whether `needle_chirho` occurs anywhere inside `haystack_chirho`.
fn ty_contains_subterm_chirho(haystack_chirho: &TyChirho, needle_chirho: &TyChirho) -> bool {
    if haystack_chirho == needle_chirho {
        return true;
    }
    match haystack_chirho {
        TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => false,
        TyChirho::AppChirho(fun_chirho, arg_chirho) => {
            ty_contains_subterm_chirho(fun_chirho, needle_chirho)
                || ty_contains_subterm_chirho(arg_chirho, needle_chirho)
        }
        TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
            ty_contains_subterm_chirho(arg_chirho, needle_chirho)
                || ty_contains_subterm_chirho(result_chirho, needle_chirho)
        }
        TyChirho::TupleChirho(elems_chirho) => elems_chirho
            .iter()
            .any(|elem_chirho| ty_contains_subterm_chirho(elem_chirho, needle_chirho)),
        TyChirho::ListChirho(inner_chirho) => {
            ty_contains_subterm_chirho(inner_chirho, needle_chirho)
        }
        TyChirho::ForallChirho { body_chirho, .. }
        | TyChirho::RequiredForallChirho { body_chirho, .. } => {
            ty_contains_subterm_chirho(body_chirho, needle_chirho)
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn plus_chirho(left_chirho: TyChirho, right_chirho: TyChirho) -> TyChirho {
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("+".to_string())),
                Box::new(left_chirho),
            )),
            Box::new(right_chirho),
        )
    }

    #[test]
    fn plus_interaction_derives_the_other_summand_chirho() {
        let a_chirho = TyChirho::ForallVarChirho("a%1".to_string());
        let b_chirho = TyChirho::ForallVarChirho("b%2".to_string());
        let c_chirho = TyChirho::ForallVarChirho("c%3".to_string());
        let derived_chirho = InferCtxChirho::plus_interaction_chirho(
            &plus_chirho(a_chirho.clone(), b_chirho.clone()),
            &plus_chirho(a_chirho.clone(), c_chirho.clone()),
        );
        assert_eq!(derived_chirho, Some((b_chirho.clone(), c_chirho.clone())));
        let swapped_chirho = InferCtxChirho::plus_interaction_chirho(
            &plus_chirho(b_chirho.clone(), a_chirho.clone()),
            &plus_chirho(c_chirho.clone(), a_chirho.clone()),
        );
        assert_eq!(swapped_chirho, Some((b_chirho.clone(), c_chirho.clone())));
        let unrelated_chirho = InferCtxChirho::plus_interaction_chirho(
            &plus_chirho(a_chirho.clone(), b_chirho),
            &plus_chirho(c_chirho, a_chirho),
        );
        assert_eq!(unrelated_chirho, None);
    }

    #[test]
    fn subterm_test_finds_a_cyclic_rewrite_chirho() {
        // `b ~ Maybe (Foo b)` must not become a rewrite rule.
        let b_chirho = TyChirho::ForallVarChirho("b%1".to_string());
        let rhs_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Foo".to_string())),
                Box::new(b_chirho.clone()),
            )),
        );
        assert!(ty_contains_subterm_chirho(&rhs_chirho, &b_chirho));
        assert!(!ty_contains_subterm_chirho(
            &TyChirho::int_chirho(),
            &b_chirho
        ));
    }

    #[test]
    fn scope_end_discharge_keeps_instance_only_wanteds_chirho() {
        // `Show a` is served by the given and leaves; the literal's `Num Int`
        // is settled by instances alone and must stay deferred so that
        // generalization absorbs it into the binding's scheme, where the
        // dictionary pass reads it as evidence (workflow:
        // language-features-chirho/rigid-type-variables-chirho).
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let num_int_chirho = PredChirho::new_chirho("Num", TyChirho::int_chirho());
        assert!(
            ctx_chirho
                .class_env_chirho
                .resolve_chirho(&num_int_chirho)
                .is_some(),
            "the seeded class environment must know `Num Int`"
        );
        let a_chirho = TyChirho::ForallVarChirho("a%1".to_string());
        let show_a_chirho = PredChirho::new_chirho("Show", a_chirho.clone());
        // `Show [a]` goes through the list instance, whose context the given
        // serves: discharged. `Show (Maybe Int)` goes through instances only.
        let show_list_a_chirho =
            PredChirho::new_chirho("Show", TyChirho::ListChirho(Box::new(a_chirho)));
        let show_maybe_int_chirho = PredChirho::new_chirho(
            "Show",
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(TyChirho::int_chirho()),
            ),
        );
        ctx_chirho.given_preds_chirho.push(show_a_chirho.clone());
        for pred_chirho in [
            show_a_chirho,
            show_list_a_chirho,
            num_int_chirho.clone(),
            show_maybe_int_chirho.clone(),
        ] {
            ctx_chirho
                .deferred_preds_chirho
                .push((pred_chirho, SpanChirho::DUMMY_CHIRHO));
        }
        let improvement_chirho = ctx_chirho.discharge_deferred_with_current_givens_chirho();
        assert!(improvement_chirho.is_empty_chirho());
        let remaining_chirho: Vec<PredChirho> = ctx_chirho
            .deferred_preds_chirho
            .iter()
            .map(|(pred_chirho, _)| pred_chirho.clone())
            .collect();
        assert_eq!(
            remaining_chirho,
            vec![num_int_chirho, show_maybe_int_chirho]
        );
    }

    #[test]
    fn cyclic_given_is_not_installed_as_a_rule_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let b_chirho = TyChirho::ForallVarChirho("b%1".to_string());
        let rhs_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Foo".to_string())),
                Box::new(b_chirho.clone()),
            )),
        );
        let mut given_chirho = PredChirho::new_chirho("~", b_chirho.clone());
        given_chirho.extra_tys_chirho = vec![rhs_chirho];
        ctx_chirho.install_given_equalities_chirho(&[given_chirho]);
        assert!(ctx_chirho.given_rewrites_chirho.is_empty());
        // A plain rigid-variable given does become a rule.
        let mut plain_chirho = PredChirho::new_chirho("~", b_chirho.clone());
        plain_chirho.extra_tys_chirho = vec![TyChirho::int_chirho()];
        ctx_chirho.install_given_equalities_chirho(&[plain_chirho]);
        assert_eq!(
            ctx_chirho.given_rewrites_chirho,
            vec![(b_chirho, TyChirho::int_chirho())]
        );
    }
}
