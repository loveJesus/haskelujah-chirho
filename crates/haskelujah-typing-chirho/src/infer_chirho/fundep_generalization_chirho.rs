// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Functional dependencies at generalization and at the module-end check.
//!
//! A fundep `a -> b` makes `b` a function of `a`. In a wanted `C Ex t` of
//! `class C a b | a -> b`, the closed `Ex` fixes `t`, even when nothing else
//! mentions `t`. GHC therefore makes such a variable monomorphic instead of
//! quantifying it, and does not quantify a constraint that mentions one
//! (Note [Do not quantify over constraints that determine a variable] in
//! GHC.Tc.Solver). A constraint may still mention variables already free in
//! the environment, and the test is made for top-level declarations only: a
//! nested binding quantifies such a constraint when it mentions a quantified
//! variable, and its call sites solve or report it.
//!
//! What generalization declines stays deferred. At the module end, a
//! leftover wanted can never be solved when one of its fundeps has ground
//! determining arguments that no instance matches, while a determined
//! argument still holds a variable. That is GHC's "No instance for". Two
//! kinds of leftover are never refuted here. A class that another class
//! extends could be supplied by a given of the subclass through superclass
//! projection with its arguments permuted, and given entailment compares
//! argument vectors position by position. A wanted over a variable that a
//! nested binding quantified escaped that binding's scheme, so the binding's
//! call sites never judged it.
//! workflow: language-features-chirho/fundep-generalization-chirho

use std::collections::HashSet;

use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

use super::{InferCtxChirho, UNSATISFIED_CONSTRAINT_CODE_CHIRHO};
use crate::class_chirho::{PredChirho, match_ty_chirho};
use crate::skolem_chirho::contains_skolem_chirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemePredChirho, TyChirho, TyVarChirho};

/// Where a binding group is generalized. GHC's test for constraints over
/// newly monomorphic variables applies to top-level declarations only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GeneralizationScopeChirho {
    /// A top-level function group or pattern binding.
    TopLevelChirho,
    /// A `let` or `where` binding inside an expression.
    NestedChirho,
}

/// What one generalization knows about the fundeps of its deferred wanteds.
pub(super) struct FundepGeneralizationChirho<'a> {
    /// Variables only a fundep fixes, from monomorphic or closed arguments.
    pub(super) newly_mono_chirho: HashSet<TyVarChirho>,
    /// The variables this generalization quantifies.
    pub(super) quantified_chirho: &'a [TyVarChirho],
    pub(super) scope_chirho: GeneralizationScopeChirho,
}

/// A deferred wanted's normalized arguments beside its class's fundeps.
struct FundepWantedChirho<'a> {
    fundeps_chirho: &'a [(Vec<usize>, Vec<usize>)],
    args_chirho: Vec<TyChirho>,
}

/// A tuple or function type constructor written in prefix form.
fn is_prefix_tuple_or_arrow_chirho(name_chirho: &str) -> bool {
    name_chirho == "->"
        || name_chirho == "(->)"
        || (name_chirho.starts_with("(,") && name_chirho.ends_with(')'))
}

/// Whether a type still holds a variable: a unification variable, a rigid
/// skolem or a forall-bound name.
fn mentions_type_variable_chirho(ty_chirho: &TyChirho) -> bool {
    match ty_chirho {
        TyChirho::VarChirho(_) | TyChirho::ForallVarChirho(_) => true,
        TyChirho::ConChirho(_) => contains_skolem_chirho(ty_chirho),
        TyChirho::AppChirho(fun_chirho, arg_chirho)
        | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => {
            mentions_type_variable_chirho(fun_chirho) || mentions_type_variable_chirho(arg_chirho)
        }
        TyChirho::ListChirho(elem_chirho) => mentions_type_variable_chirho(elem_chirho),
        TyChirho::TupleChirho(elems_chirho) => {
            elems_chirho.iter().any(mentions_type_variable_chirho)
        }
        // A polytype in a determined position is outside this rule.
        TyChirho::ForallChirho { .. } | TyChirho::RequiredForallChirho { .. } => false,
    }
}

impl InferCtxChirho {
    /// A predicate's arguments in class order, each normalized.
    fn normalized_pred_args_chirho(&self, pred_chirho: &PredChirho) -> Vec<TyChirho> {
        std::iter::once(&pred_chirho.ty_chirho)
            .chain(&pred_chirho.extra_tys_chirho)
            .map(|arg_chirho| self.normalize_ty_chirho(arg_chirho))
            .collect()
    }

    /// The variables that become monomorphic only because a fundep of a
    /// deferred wanted fixes them. Every free variable of the determining
    /// arguments must be monomorphic already: in `mono0_chirho` (the
    /// environment and the givens), fixed by an earlier round, or absent
    /// because the arguments are closed. Iterated to a fixed point.
    pub(super) fn fundep_newly_mono_vars_chirho(
        &self,
        mono0_chirho: &[TyVarChirho],
    ) -> HashSet<TyVarChirho> {
        let wanteds_chirho: Vec<FundepWantedChirho<'_>> = self
            .deferred_preds_chirho
            .iter()
            .filter_map(|(pred_chirho, _span_chirho)| {
                let class_chirho = self
                    .class_env_chirho
                    .classes_chirho
                    .get(&pred_chirho.class_name_chirho)?;
                (!class_chirho.fundeps_chirho.is_empty()).then(|| FundepWantedChirho {
                    fundeps_chirho: &class_chirho.fundeps_chirho,
                    args_chirho: self.normalized_pred_args_chirho(pred_chirho),
                })
            })
            .collect();
        let mut mono_chirho: HashSet<TyVarChirho> = mono0_chirho.iter().copied().collect();
        let mut newly_chirho = HashSet::new();
        loop {
            let mut grew_chirho = false;
            for wanted_chirho in &wanteds_chirho {
                for (from_chirho, to_chirho) in wanted_chirho.fundeps_chirho {
                    let determined_chirho = from_chirho.iter().all(|&index_chirho| {
                        wanted_chirho
                            .args_chirho
                            .get(index_chirho)
                            .is_some_and(|arg_chirho| {
                                arg_chirho
                                    .free_vars_chirho()
                                    .iter()
                                    .all(|var_chirho| mono_chirho.contains(var_chirho))
                            })
                    });
                    if !determined_chirho {
                        continue;
                    }
                    for arg_chirho in to_chirho
                        .iter()
                        .filter_map(|&index_chirho| wanted_chirho.args_chirho.get(index_chirho))
                    {
                        for var_chirho in arg_chirho.free_vars_chirho() {
                            if mono_chirho.insert(var_chirho) {
                                newly_chirho.insert(var_chirho);
                                grew_chirho = true;
                            }
                        }
                    }
                }
            }
            if !grew_chirho {
                return newly_chirho;
            }
        }
    }

    /// Place a wanted that mentions a newly monomorphic variable. At top
    /// level it stays deferred. A nested binding quantifies it when it
    /// mentions a quantified variable, and otherwise defers it. A wanted over
    /// a rigid variable, or one that mentions no newly monomorphic variable,
    /// comes back for the existing partition.
    pub(super) fn place_fundep_wanted_chirho(
        &self,
        wanted_chirho: (PredChirho, SpanChirho),
        fundeps_chirho: &FundepGeneralizationChirho<'_>,
        scheme_preds_chirho: &mut Vec<SchemePredChirho>,
        remaining_chirho: &mut Vec<(PredChirho, SpanChirho)>,
    ) -> Option<(PredChirho, SpanChirho)> {
        if fundeps_chirho.newly_mono_chirho.is_empty()
            || Self::pred_mentions_rigid_chirho(&wanted_chirho.0)
        {
            return Some(wanted_chirho);
        }
        let vars_chirho: Vec<TyVarChirho> = self
            .normalized_pred_args_chirho(&wanted_chirho.0)
            .iter()
            .flat_map(TyChirho::free_vars_chirho)
            .collect();
        if !vars_chirho
            .iter()
            .any(|var_chirho| fundeps_chirho.newly_mono_chirho.contains(var_chirho))
        {
            return Some(wanted_chirho);
        }
        let quantify_chirho = fundeps_chirho.scope_chirho
            == GeneralizationScopeChirho::NestedChirho
            && vars_chirho
                .iter()
                .any(|var_chirho| fundeps_chirho.quantified_chirho.contains(var_chirho));
        if quantify_chirho {
            let (pred_chirho, _span_chirho) = wanted_chirho;
            scheme_preds_chirho.push(SchemePredChirho {
                class_name_chirho: pred_chirho.class_name_chirho,
                ty_chirho: pred_chirho.ty_chirho,
                extra_tys_chirho: pred_chirho.extra_tys_chirho,
            });
        } else {
            remaining_chirho.push(wanted_chirho);
        }
        None
    }

    /// Report a leftover wanted that a fundep makes unsolvable, after the
    /// final substitution and normalization of every argument. Returns
    /// whether it was reported.
    pub(super) fn report_unsatisfiable_fundep_wanted_chirho(
        &mut self,
        defaulted_pred_chirho: &PredChirho,
        final_subst_chirho: &SubstChirho,
        givens_chirho: &[PredChirho],
        span_chirho: SpanChirho,
    ) -> bool {
        let resolved_chirho = PredChirho {
            class_name_chirho: defaulted_pred_chirho.class_name_chirho.clone(),
            ty_chirho: final_subst_chirho.apply_ty_chirho(&defaulted_pred_chirho.ty_chirho),
            extra_tys_chirho: defaulted_pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|arg_chirho| final_subst_chirho.apply_ty_chirho(arg_chirho))
                .collect(),
        };
        let args_chirho = self.normalized_pred_args_chirho(&resolved_chirho);
        let pred_chirho = PredChirho {
            class_name_chirho: resolved_chirho.class_name_chirho,
            ty_chirho: args_chirho[0].clone(),
            extra_tys_chirho: args_chirho[1..].to_vec(),
        };
        if !self.fundep_wanted_unsatisfiable_chirho(&pred_chirho, &args_chirho, givens_chirho) {
            return false;
        }
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                format!("no instance for `{pred_chirho}`"),
                span_chirho,
            ));
        true
    }

    /// The module-end refutation, with every exclusion the ground check uses.
    fn fundep_wanted_unsatisfiable_chirho(
        &self,
        pred_chirho: &PredChirho,
        args_chirho: &[TyChirho],
        givens_chirho: &[PredChirho],
    ) -> bool {
        let class_name_chirho = pred_chirho.class_name_chirho.as_str();
        if !self.constraint_generation_faithful_chirho
            || class_name_chirho == "~"
            || class_name_chirho.starts_with('?')
            || Self::MAGIC_CLASSES_CHIRHO.contains(&class_name_chirho)
            || !self
                .fully_known_class_names_chirho
                .contains(class_name_chirho)
            || self.pred_entailed_by_givens_chirho(pred_chirho, givens_chirho)
            || self.is_extended_by_another_class_chirho(class_name_chirho)
            || self.mentions_locally_generalized_chirho(args_chirho)
        {
            return false;
        }
        let Some(class_chirho) = self.class_env_chirho.classes_chirho.get(class_name_chirho) else {
            return false;
        };
        class_chirho
            .fundeps_chirho
            .iter()
            .any(|(from_chirho, to_chirho)| {
                from_chirho.iter().all(|&index_chirho| {
                    args_chirho
                        .get(index_chirho)
                        .is_some_and(|arg_chirho| self.is_closed_determinant_chirho(arg_chirho))
                }) && to_chirho.iter().any(|&index_chirho| {
                    args_chirho
                        .get(index_chirho)
                        .is_some_and(mentions_type_variable_chirho)
                }) && !self.instance_may_match_determinants_chirho(
                    class_name_chirho,
                    from_chirho,
                    args_chirho,
                )
            })
    }

    /// Whether the wanted mentions a variable that a nested binding quantified.
    /// Such a wanted escaped that binding's scheme: a constraint over a rigid
    /// variable is never quantified there, so no scope judged it at the
    /// binding's call sites (GHC quantifies it and checks each call; R02 is
    /// valid that way). The refutation abstains and today's route stands.
    fn mentions_locally_generalized_chirho(&self, args_chirho: &[TyChirho]) -> bool {
        args_chirho
            .iter()
            .flat_map(TyChirho::free_vars_chirho)
            .any(|var_chirho| self.local_generalized_vars_chirho.contains(&var_chirho))
    }

    /// Whether some other known class has this one among its superclasses.
    fn is_extended_by_another_class_chirho(&self, class_name_chirho: &str) -> bool {
        self.class_env_chirho
            .classes_chirho
            .keys()
            .any(|other_chirho| {
                other_chirho != class_name_chirho
                    && self.is_transitive_superclass_chirho(class_name_chirho, other_chirho)
            })
    }

    /// A determining argument that no substitution can change: no variable,
    /// no rigid skolem, and no synonym, stuck family or primitive `#` head left
    /// after normalization.
    fn is_closed_determinant_chirho(&self, ty_chirho: &TyChirho) -> bool {
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                !contains_skolem_chirho(ty_chirho)
                    && !self.type_families_chirho.contains_key(name_chirho)
                    && !self.type_synonyms_chirho.contains_key(name_chirho)
                    && !name_chirho.ends_with('#')
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho)
            | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => {
                self.is_closed_determinant_chirho(fun_chirho)
                    && self.is_closed_determinant_chirho(arg_chirho)
            }
            TyChirho::ListChirho(elem_chirho) => self.is_closed_determinant_chirho(elem_chirho),
            TyChirho::TupleChirho(elems_chirho) => elems_chirho
                .iter()
                .all(|elem_chirho| self.is_closed_determinant_chirho(elem_chirho)),
            TyChirho::VarChirho(_)
            | TyChirho::ForallVarChirho(_)
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => false,
        }
    }

    /// Whether an instance could supply the determining positions, matched
    /// with ONE substitution across all of them. An instance this test cannot
    /// read with certainty counts as a match, so the refutation is withheld.
    fn instance_may_match_determinants_chirho(
        &self,
        class_name_chirho: &str,
        from_chirho: &[usize],
        args_chirho: &[TyChirho],
    ) -> bool {
        if self
            .derived_instance_heads_chirho
            .iter()
            .any(|(derived_class_chirho, _head_chirho)| derived_class_chirho == class_name_chirho)
        {
            return true;
        }
        let Some(instances_chirho) = self
            .class_env_chirho
            .instances_chirho
            .get(class_name_chirho)
        else {
            return false;
        };
        instances_chirho.iter().any(|instance_chirho| {
            let instance_args_chirho: Vec<TyChirho> =
                std::iter::once(&instance_chirho.head_ty_chirho)
                    .chain(&instance_chirho.extra_head_tys_chirho)
                    .map(|arg_chirho| self.normalize_ty_chirho(arg_chirho))
                    .collect();
            if instance_args_chirho.len() != args_chirho.len() {
                return true;
            }
            let mut matched_chirho = SubstChirho::empty_chirho();
            from_chirho.iter().all(|&index_chirho| {
                let pattern_chirho = &instance_args_chirho[index_chirho];
                if !self.is_matchable_instance_type_chirho(pattern_chirho)
                    || !self.is_matchable_instance_type_chirho(&args_chirho[index_chirho])
                {
                    return true;
                }
                match_ty_chirho(pattern_chirho, &args_chirho[index_chirho])
                    .is_some_and(|step_chirho| matched_chirho.merge_chirho(&step_chirho))
            })
        })
    }

    /// A type that matching reads with certainty: no synonym or family left,
    /// and no tuple or arrow written as a prefix constructor, which matching
    /// does not equate with the sugared form.
    fn is_matchable_instance_type_chirho(&self, ty_chirho: &TyChirho) -> bool {
        match ty_chirho {
            TyChirho::VarChirho(_) => true,
            TyChirho::ConChirho(name_chirho) => {
                !self.type_families_chirho.contains_key(name_chirho)
                    && !self.type_synonyms_chirho.contains_key(name_chirho)
                    && !is_prefix_tuple_or_arrow_chirho(name_chirho)
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho)
            | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => {
                self.is_matchable_instance_type_chirho(fun_chirho)
                    && self.is_matchable_instance_type_chirho(arg_chirho)
            }
            TyChirho::ListChirho(elem_chirho) => {
                self.is_matchable_instance_type_chirho(elem_chirho)
            }
            TyChirho::TupleChirho(elems_chirho) => elems_chirho
                .iter()
                .all(|elem_chirho| self.is_matchable_instance_type_chirho(elem_chirho)),
            TyChirho::ForallVarChirho(_)
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => false,
        }
    }
}
