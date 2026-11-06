// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The monomorphism restriction (Haskell 2010 section 4.5.5) as GHC applies it in every
//! compiled module (row 692, gpt_chirho #32508-#32510).
//!
//! **Rule 1.** A declaration group is restricted when one of its members is a simple pattern
//! binding without a signature. A restricted group never generalizes a constrained type
//! variable. The variable stays in the binding's monomorphic type, so a later use in the module
//! can fix it. Without the rule, an unsigned `main = id 7` became a function of its `Num`
//! dictionary, and an IO entry whose result a literal constrained never ran.
//!
//! **Rule 2.** Whatever is left of those variables when the module's inference ends is resolved
//! here, before the legacy defaults run. They never reach the legacy Monad-to-IO,
//! user-class-leniency or Integral/Bounded paths. The checks run in this order:
//! 1. A variable may default only when every class constraining it is standard and at least
//!    one is numeric (or, under OverloadedStrings, `IsString`). Each class must also be a
//!    proven compiler class: the predicate carries the compiler identity, and the active class
//!    record is that compiler record. A program's own or unknown class never defaults.
//! 2. The candidates are the module's `default` list when one is declared, even an empty one;
//!    otherwise Integer, then Double, then String under OverloadedStrings. A candidate must
//!    satisfy every original predicate substituted, each through exactly one instance of that
//!    same compiler class.
//! 3. Anything else is an ambiguous type variable, and the program is refused.
//!
//! **Scope of the first brick:** top-level function-syntax and simple pattern bindings, in the
//! SCC loop. Still open:
//! - Pattern bindings (always restricted) and nested bindings.
//! - The executable entry's obligation (Report section 5: `main` is `IO t`). It needs an
//!   explicit caller purpose, so a bare `main = return 5` stays ambiguous until then.
//! workflow: language-features-chirho/fundep-generalization-chirho

use std::collections::BTreeSet;
use std::sync::Arc;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerClassChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
};
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

use super::{InferCtxChirho, UNSATISFIED_CONSTRAINT_CODE_CHIRHO};
use crate::class_chirho::{ClassEnvChirho, PredChirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};

/// How deep an instance's context is followed while a default is judged.
const INSTANCE_DEPTH_CHIRHO: usize = 16;

/// One module's restriction facts, and the variables its restricted groups withheld.
#[derive(Debug, Default)]
pub(super) struct RestrictionStateChirho {
    enabled_chirho: bool,
    default_declared_chirho: bool,
    /// Kept current through every substitution, so the set names live variables.
    withheld_chirho: BTreeSet<TyVarChirho>,
}

impl RestrictionStateChirho {
    /// Read the module's facts once, before its first group is generalized.
    pub(super) fn begin_module_chirho(&mut self, module_chirho: &ModuleChirho) {
        *self = Self {
            enabled_chirho: restriction_enabled_chirho(&module_chirho.extensions_chirho),
            default_declared_chirho: module_chirho
                .decls_chirho
                .iter()
                .any(|decl_chirho| matches!(decl_chirho, DeclChirho::DefaultDeclChirho { .. })),
            withheld_chirho: BTreeSet::new(),
        };
    }

    pub(super) fn enabled_chirho(&self) -> bool {
        self.enabled_chirho
    }

    /// Rule 1 kept these variables monomorphic.
    pub(super) fn withhold_chirho(
        &mut self,
        variables_chirho: impl IntoIterator<Item = TyVarChirho>,
    ) {
        self.withheld_chirho.extend(variables_chirho);
    }

    /// A substitution moves a withheld variable onto the variables of its image.
    pub(super) fn follow_substitution_chirho(&mut self, subst_chirho: &SubstChirho) {
        if self.withheld_chirho.is_empty() || subst_chirho.is_empty_chirho() {
            return;
        }
        self.withheld_chirho = self
            .withheld_chirho
            .iter()
            .flat_map(|variable_chirho| {
                subst_chirho
                    .apply_ty_chirho(&TyChirho::VarChirho(*variable_chirho))
                    .free_vars_chirho()
            })
            .collect();
    }

    pub(super) fn withheld_chirho(&self) -> &BTreeSet<TyVarChirho> {
        &self.withheld_chirho
    }
}

/// Whether the extension `name_chirho` is in effect: the last of `Name` and `NoName` wins,
/// and `default_chirho` holds when the module names neither.
pub(super) fn effective_flag_chirho(
    extensions_chirho: &[String],
    name_chirho: &str,
    default_chirho: bool,
) -> bool {
    extensions_chirho
        .iter()
        .fold(default_chirho, |enabled_chirho, flag_chirho| {
            if flag_chirho == name_chirho {
                true
            } else if flag_chirho
                .strip_prefix("No")
                .is_some_and(|negated_chirho| negated_chirho == name_chirho)
            {
                false
            } else {
                enabled_chirho
            }
        })
}

/// The restriction is on by default; the last of its two flags wins.
pub(super) fn restriction_enabled_chirho(extensions_chirho: &[String]) -> bool {
    effective_flag_chirho(extensions_chirho, "MonomorphismRestriction", true)
}

/// Rule 1 for one group, given each member's (argument-less, signed): restricted when any
/// argument-less member has no signature. A member with arguments never restricts its group.
pub(super) fn group_is_restricted_chirho(
    members_chirho: impl IntoIterator<Item = (bool, bool)>,
) -> bool {
    members_chirho
        .into_iter()
        .any(|(argument_less_chirho, signed_chirho)| argument_less_chirho && !signed_chirho)
}

/// The Report's numeric classes (section 4.3.4).
pub(super) fn numeric_class_chirho(identity_chirho: CompilerClassChirho) -> bool {
    matches!(
        identity_chirho,
        CompilerClassChirho::NumChirho
            | CompilerClassChirho::RealChirho
            | CompilerClassChirho::IntegralChirho
            | CompilerClassChirho::FractionalChirho
            | CompilerClassChirho::FloatingChirho
            | CompilerClassChirho::RealFracChirho
            | CompilerClassChirho::RealFloatChirho
    )
}

/// GHC's standard classes for defaulting (its `standardClassKeys`) that this compiler names.
/// `NFData` and the rest are not standard, so they never default.
pub(super) fn standard_class_chirho(identity_chirho: CompilerClassChirho) -> bool {
    numeric_class_chirho(identity_chirho)
        || matches!(
            identity_chirho,
            CompilerClassChirho::EqChirho
                | CompilerClassChirho::OrdChirho
                | CompilerClassChirho::EnumChirho
                | CompilerClassChirho::BoundedChirho
                | CompilerClassChirho::ShowChirho
                | CompilerClassChirho::ReadChirho
                | CompilerClassChirho::RandomGenChirho
                | CompilerClassChirho::FunctorChirho
                | CompilerClassChirho::MonadChirho
                | CompilerClassChirho::MonadPlusChirho
                | CompilerClassChirho::MonadFailChirho
                | CompilerClassChirho::SemigroupChirho
                | CompilerClassChirho::MonoidChirho
                | CompilerClassChirho::IsStringChirho
                | CompilerClassChirho::ApplicativeChirho
                | CompilerClassChirho::FoldableChirho
                | CompilerClassChirho::TraversableChirho
                | CompilerClassChirho::AlternativeChirho
        )
}

/// Whether `predicate_chirho` constrains `variable_chirho` through an ordinary argument, primary
/// or extra. A variable a wanted holds only in its kind arguments is the kind engine's, so the
/// restriction neither defaults nor refuses it there.
fn mentions_chirho(predicate_chirho: &PredChirho, variable_chirho: TyVarChirho) -> bool {
    std::iter::once(&predicate_chirho.ty_chirho)
        .chain(&predicate_chirho.extra_tys_chirho)
        .any(|ty_chirho| ty_chirho.free_vars_chirho().contains(&variable_chirho))
}

impl InferCtxChirho {
    /// The first type variable a declaration group's inference can create: every later one
    /// is at or above it.
    pub(super) fn group_variable_mark_chirho(&self) -> TyVarChirho {
        TyVarChirho(self.elaboration_state_chirho.next_variable_chirho)
    }

    /// Rule 1 reaches every wanted a restricted group left behind, not only those its result
    /// type mentions. `text = show (read "1")` leaves `Read a, Show a` on a variable that no
    /// binding's type holds.
    ///
    /// The variables are chosen group-locally, but the scan is module-wide: it visits every
    /// deferred wanted once for each restricted group. That is the same visit generalization
    /// already makes for each member when it builds its predicate owners over all deferred
    /// wanteds (`generalize_group_member_chirho`), and no other budget is charged.
    ///
    /// Only a unification variable from a wanted's ordinary arguments (primary and extra) at
    /// or above `first_chirho` is withheld; Rule 2 then defaults or refuses it.
    /// - Kind arguments stay with the kind engine.
    /// - Rigid variables are not unification variables.
    /// - Givens are not deferred wanteds.
    /// - A quantified variable has already left with its scheme.
    /// - The variable supply only grows during a module's inference, so a variable from an
    ///   earlier group stays that group's own.
    /// workflow: language-features-chirho/fundep-generalization-chirho
    pub(super) fn withhold_group_residuals_chirho(&mut self, first_chirho: TyVarChirho) {
        let residual_chirho: Vec<TyVarChirho> = self
            .deferred_preds_chirho
            .iter()
            .flat_map(|(predicate_chirho, _)| {
                std::iter::once(&predicate_chirho.ty_chirho)
                    .chain(&predicate_chirho.extra_tys_chirho)
                    .flat_map(TyChirho::free_vars_chirho)
                    .collect::<Vec<_>>()
            })
            .filter(|variable_chirho| *variable_chirho >= first_chirho)
            .collect();
        self.restriction_chirho.withhold_chirho(residual_chirho);
    }

    /// Rule 2 at the module's end, before the legacy defaults: each withheld variable's default
    /// or refusal. Returns the substitution it applied; a refused variable's wanteds were
    /// reported and leave the legacy path.
    /// workflow: language-features-chirho/fundep-generalization-chirho
    pub(super) fn resolve_restricted_residuals_chirho(
        &mut self,
        final_subst_chirho: &SubstChirho,
    ) -> SubstChirho {
        self.restriction_chirho
            .follow_substitution_chirho(final_subst_chirho);
        if self.restriction_chirho.withheld_chirho().is_empty() {
            return SubstChirho::empty_chirho();
        }
        let current_chirho = final_subst_chirho;
        let mut defaults_chirho = SubstChirho::empty_chirho();
        let mut refused_chirho = BTreeSet::new();
        let withheld_chirho: Vec<TyVarChirho> = self
            .restriction_chirho
            .withheld_chirho()
            .iter()
            .copied()
            .collect();
        for variable_chirho in withheld_chirho {
            let residuals_chirho: Vec<(PredChirho, SpanChirho)> = self
                .deferred_preds_chirho
                .iter()
                .map(|(predicate_chirho, span_chirho)| {
                    (
                        predicate_chirho.mapped_chirho(|ty_chirho| {
                            self.normalize_ty_chirho(&current_chirho.apply_ty_chirho(ty_chirho))
                        }),
                        *span_chirho,
                    )
                })
                .filter(|(predicate_chirho, _)| mentions_chirho(predicate_chirho, variable_chirho))
                .collect();
            if residuals_chirho.is_empty() {
                continue;
            }
            match self.restricted_default_chirho(variable_chirho, &residuals_chirho) {
                Some(default_chirho) => {
                    defaults_chirho.insert_chirho(variable_chirho, default_chirho)
                }
                None => {
                    self.report_ambiguous_restricted_chirho(variable_chirho, &residuals_chirho);
                    refused_chirho.insert(variable_chirho);
                }
            }
        }
        if !refused_chirho.is_empty() {
            let deferred_chirho = std::mem::take(&mut self.deferred_preds_chirho);
            self.deferred_preds_chirho = deferred_chirho
                .into_iter()
                .filter(|(predicate_chirho, _)| {
                    let current_predicate_chirho = predicate_chirho
                        .mapped_chirho(|ty_chirho| current_chirho.apply_ty_chirho(ty_chirho));
                    !refused_chirho.iter().any(|variable_chirho| {
                        mentions_chirho(&current_predicate_chirho, *variable_chirho)
                    })
                })
                .collect();
        }
        if !defaults_chirho.is_empty_chirho() {
            self.apply_subst_all_chirho(&defaults_chirho);
        }
        defaults_chirho
    }

    /// The default for one withheld variable, or `None` when Rule 2 cannot choose one.
    fn restricted_default_chirho(
        &self,
        variable_chirho: TyVarChirho,
        residuals_chirho: &[(PredChirho, SpanChirho)],
    ) -> Option<TyChirho> {
        let mut constraints_chirho = Vec::new();
        for (predicate_chirho, _) in residuals_chirho {
            if !self.reduce_to_variable_chirho(
                predicate_chirho,
                variable_chirho,
                0,
                &mut constraints_chirho,
            ) {
                return None;
            }
        }
        // OverloadedStrings extends the trigger to IsString and the standard list to String.
        let strings_chirho = self.overloaded_strings_chirho;
        let mut triggered_chirho = false;
        for constraint_chirho in &constraints_chirho {
            let identity_chirho = self.active_compiler_class_chirho(constraint_chirho)?;
            if !standard_class_chirho(identity_chirho) {
                return None;
            }
            triggered_chirho |= numeric_class_chirho(identity_chirho)
                || (strings_chirho && identity_chirho == CompilerClassChirho::IsStringChirho);
        }
        if !triggered_chirho {
            return None;
        }
        let candidates_chirho = if self.restriction_chirho.default_declared_chirho {
            self.module_default_types_chirho.clone()
        } else {
            let mut standard_chirho = vec![
                TyChirho::ConChirho("Integer".to_string()),
                TyChirho::double_chirho(),
            ];
            if strings_chirho {
                standard_chirho.push(TyChirho::string_chirho());
            }
            standard_chirho
        };
        candidates_chirho.into_iter().find(|candidate_chirho| {
            let mut chosen_chirho = SubstChirho::empty_chirho();
            chosen_chirho.insert_chirho(variable_chirho, candidate_chirho.clone());
            residuals_chirho.iter().all(|(predicate_chirho, _)| {
                self.satisfied_by_compiler_instance_chirho(
                    &predicate_chirho
                        .mapped_chirho(|ty_chirho| chosen_chirho.apply_ty_chirho(ty_chirho)),
                    0,
                )
            })
        })
    }

    /// Collect the `C v` constraints `predicate_chirho` puts on `variable_chirho`, reducing a
    /// structured one (`Show [v]`) through its one compiler instance. False when the variable
    /// sits anywhere else, as in `C v w` or an unreduced head.
    fn reduce_to_variable_chirho(
        &self,
        predicate_chirho: &PredChirho,
        variable_chirho: TyVarChirho,
        depth_chirho: usize,
        constraints_chirho: &mut Vec<PredChirho>,
    ) -> bool {
        if !mentions_chirho(predicate_chirho, variable_chirho) {
            return true;
        }
        if depth_chirho > INSTANCE_DEPTH_CHIRHO {
            return false;
        }
        let bare_chirho = matches!(
            self.normalize_ty_chirho(&predicate_chirho.ty_chirho),
            TyChirho::VarChirho(found_chirho) if found_chirho == variable_chirho
        );
        let elsewhere_chirho = predicate_chirho
            .kind_tys_chirho
            .iter()
            .chain(&predicate_chirho.extra_tys_chirho)
            .any(|ty_chirho| ty_chirho.free_vars_chirho().contains(&variable_chirho));
        if bare_chirho && !elsewhere_chirho {
            constraints_chirho.push(predicate_chirho.clone());
            return true;
        }
        self.compiler_instance_goals_chirho(predicate_chirho)
            .is_some_and(|goals_chirho| {
                goals_chirho.iter().all(|goal_chirho| {
                    self.reduce_to_variable_chirho(
                        goal_chirho,
                        variable_chirho,
                        depth_chirho + 1,
                        constraints_chirho,
                    )
                })
            })
    }

    /// The compiler class a predicate proves, when the active record under its name is that
    /// compiler record. A program's own or an unknown class proves none.
    fn active_compiler_class_chirho(
        &self,
        predicate_chirho: &PredChirho,
    ) -> Option<CompilerClassChirho> {
        let PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
            identity_chirho,
        )) = predicate_chirho.class_identity_chirho()
        else {
            return None;
        };
        let producer_chirho = self
            .class_env_chirho
            .compiler_class_record_chirho(*identity_chirho)?;
        let active_chirho = self
            .class_env_chirho
            .classes_chirho()
            .get_chirho(&predicate_chirho.class_name_chirho)?;
        Arc::ptr_eq(producer_chirho, active_chirho).then_some(*identity_chirho)
    }

    /// The context of the one instance of the predicate's own compiler class that matches it.
    /// None for an unproven class, no match, a match of another class, or several matches.
    fn compiler_instance_goals_chirho(
        &self,
        predicate_chirho: &PredChirho,
    ) -> Option<Vec<PredChirho>> {
        let identity_chirho = self.active_compiler_class_chirho(predicate_chirho)?;
        let instances_chirho = self
            .class_env_chirho
            .instances_chirho()
            .get_chirho(&predicate_chirho.class_name_chirho)?;
        let mut matched_chirho = None;
        for instance_chirho in instances_chirho.iter_chirho() {
            let Some(goals_chirho) =
                ClassEnvChirho::instance_sub_goals_chirho(instance_chirho, predicate_chirho)
            else {
                continue;
            };
            let same_class_chirho = matches!(
                instance_chirho.class_identity_chirho(),
                PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
                    found_chirho
                )) if *found_chirho == identity_chirho
            );
            if !same_class_chirho || matched_chirho.is_some() {
                return None;
            }
            matched_chirho = Some(goals_chirho);
        }
        matched_chirho
    }

    /// Whether compiler instances alone satisfy `predicate_chirho`, context included.
    fn satisfied_by_compiler_instance_chirho(
        &self,
        predicate_chirho: &PredChirho,
        depth_chirho: usize,
    ) -> bool {
        depth_chirho <= INSTANCE_DEPTH_CHIRHO
            && self
                .compiler_instance_goals_chirho(predicate_chirho)
                .is_some_and(|goals_chirho| {
                    goals_chirho.iter().all(|goal_chirho| {
                        self.satisfied_by_compiler_instance_chirho(goal_chirho, depth_chirho + 1)
                    })
                })
    }

    /// GHC's refusal: the restricted variable is ambiguous. It names every wanted it carries.
    fn report_ambiguous_restricted_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        residuals_chirho: &[(PredChirho, SpanChirho)],
    ) {
        let wanted_chirho = residuals_chirho
            .iter()
            .map(|(predicate_chirho, _)| format!("`{predicate_chirho}`"))
            .collect::<Vec<_>>()
            .join(", ");
        let span_chirho = residuals_chirho
            .first()
            .map(|(_, span_chirho)| *span_chirho)
            .unwrap_or(SpanChirho::DUMMY_CHIRHO);
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                format!(
                    "ambiguous type variable `{}` of a restricted binding: no default satisfies {wanted_chirho} (Haskell 2010 section 4.5.5)",
                    TyChirho::VarChirho(variable_chirho)
                ),
                span_chirho,
            ));
    }
}

#[cfg(test)]
#[path = "restriction_tests_chirho.rs"]
mod tests_chirho;
