// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A signature's wanted that nothing in its scope can serve, reported before its givens leave
//! scope (row 686, control C08): GHC's "No instance for" over a family application that cannot
//! reduce. Only wanteds over the signature's own rigid variables are judged here; no other path
//! calls a stuck family unsolvable.
//!
//! Two boundaries are deliberate. A class's bare-variable instance heads are trusted only when it
//! is fully known or produced by the standard seed, and an orphan universal instance in an
//! uncompiled package stays outside that view. And once any pattern has matched a constructor
//! whose context or refinement the checker does not install as local evidence, every later
//! closure in the module stays off, so an unrelated later signature can stay unchecked. Installing
//! real pattern-local givens is the later repair.
//! workflow: language-features-chirho/stuck-family-equalities-chirho

use super::InferCtxChirho;
use super::UNSATISFIED_CONSTRAINT_CODE_CHIRHO;
use super::collect_app_spine_chirho;
use super::family_definitions_chirho::TypeFamilyReductionChirho;
use super::family_reduction_chirho::{
    FamilyReductionDecisionChirho, FamilyReductionLimitsChirho, decide_family_with_limits_chirho,
};
use super::family_registration_chirho::family_head_role_chirho;
use super::is_placeholder_import_scheme_chirho;
use crate::class_chirho::PredChirho;
use crate::skolem_chirho::rewrite_skolems_chirho;
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use std::collections::HashSet;

/// Family applications one argument's certificate may visit; past it, nothing is certified.
const CERTIFICATE_BUDGET_CHIRHO: usize = 64;

impl InferCtxChirho {
    /// The rigid variables a signature introduces, from its type and its context.
    pub(super) fn signature_scope_skolems_chirho(
        signature_ty_chirho: &TyChirho,
        givens_chirho: &[PredChirho],
    ) -> HashSet<String> {
        let mut skolems_chirho = HashSet::new();
        let types_chirho = std::iter::once(signature_ty_chirho).chain(
            givens_chirho.iter().flat_map(|given_chirho| {
                std::iter::once(&given_chirho.ty_chirho).chain(&given_chirho.extra_tys_chirho)
            }),
        );
        for ty_chirho in types_chirho {
            rewrite_skolems_chirho(ty_chirho, &mut |name_chirho| {
                skolems_chirho.insert(name_chirho.to_string());
                None
            });
        }
        skolems_chirho
    }

    /// At a signature scope's end, with its givens and equality rewrites still installed, report
    /// each deferred wanted that names this signature's rigid variables, carries no inference
    /// variable, is entailed neither by a given nor by an instance under the givens, and has an
    /// argument that is a certified irreducible family application no instance head of its
    /// (fully visible) class can match. A reported wanted leaves the deferred list; every other
    /// wanted stays there, exactly as before.
    pub(super) fn report_stuck_signature_wanteds_chirho(
        &mut self,
        scope_skolems_chirho: &HashSet<String>,
    ) {
        if !self.constraint_generation_faithful_chirho
            || self.unmodeled_pattern_evidence_seen_chirho
            || scope_skolems_chirho.is_empty()
        {
            return;
        }
        let givens_chirho = self.given_preds_chirho.clone();
        let normalized_givens_chirho: Vec<PredChirho> = givens_chirho
            .iter()
            .map(|given_chirho| self.normalized_pred_chirho(given_chirho))
            .collect();
        let deferred_chirho = std::mem::take(&mut self.deferred_preds_chirho);
        for (pred_chirho, span_chirho) in deferred_chirho {
            // Skolem ordinals are unique per signature: another scope's wanted is passed over
            // before any normalization is spent on it.
            if !Self::pred_names_scope_chirho(&pred_chirho, scope_skolems_chirho) {
                self.deferred_preds_chirho.push((pred_chirho, span_chirho));
                continue;
            }
            let normalized_chirho = self.normalized_pred_chirho(&pred_chirho);
            if self.signature_wanted_unservable_chirho(
                &normalized_chirho,
                scope_skolems_chirho,
                &givens_chirho,
                &normalized_givens_chirho,
            ) {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!("no instance for `{normalized_chirho}`"),
                        span_chirho,
                    ));
            } else {
                self.deferred_preds_chirho.push((pred_chirho, span_chirho));
            }
        }
    }

    /// Whether a predicate mentions one of the scope's rigid variables.
    fn pred_names_scope_chirho(
        pred_chirho: &PredChirho,
        scope_skolems_chirho: &HashSet<String>,
    ) -> bool {
        let mut names_scope_chirho = false;
        for argument_chirho in
            std::iter::once(&pred_chirho.ty_chirho).chain(&pred_chirho.extra_tys_chirho)
        {
            rewrite_skolems_chirho(argument_chirho, &mut |name_chirho| {
                names_scope_chirho |= scope_skolems_chirho.contains(name_chirho);
                None
            });
        }
        names_scope_chirho
    }

    fn normalized_pred_chirho(&self, pred_chirho: &PredChirho) -> PredChirho {
        PredChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: pred_chirho.class_name_chirho.clone(),
            ty_chirho: self.normalize_ty_chirho(&pred_chirho.ty_chirho),
            extra_tys_chirho: pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|ty_chirho| self.normalize_ty_chirho(ty_chirho))
                .collect(),
        }
    }

    fn signature_wanted_unservable_chirho(
        &self,
        pred_chirho: &PredChirho,
        scope_skolems_chirho: &HashSet<String>,
        givens_chirho: &[PredChirho],
        normalized_givens_chirho: &[PredChirho],
    ) -> bool {
        let class_chirho = pred_chirho.class_name_chirho.as_str();
        if class_chirho == "~"
            || class_chirho.starts_with('?')
            || Self::MAGIC_CLASSES_CHIRHO.contains(&class_chirho)
            || !self.class_instance_heads_visible_chirho(class_chirho)
            || self
                .derived_instance_heads_chirho
                .contains(&(class_chirho.to_string(), "*".to_string()))
        {
            return false;
        }
        let arguments_chirho: Vec<&TyChirho> = std::iter::once(&pred_chirho.ty_chirho)
            .chain(&pred_chirho.extra_tys_chirho)
            .collect();
        if arguments_chirho
            .iter()
            .any(|argument_chirho| argument_chirho.contains_var_chirho())
        {
            return false;
        }
        if !Self::pred_names_scope_chirho(pred_chirho, scope_skolems_chirho)
            || self
                .pred_entailment_under_givens_chirho(pred_chirho, givens_chirho, 0)
                .is_some()
            || self
                .pred_entailment_under_givens_chirho(pred_chirho, normalized_givens_chirho, 0)
                .is_some()
        {
            return false;
        }
        let class_instances_chirho = self.class_env_chirho.instances_chirho.get(class_chirho);
        arguments_chirho
            .iter()
            .enumerate()
            .any(|(position_chirho, argument_chirho)| {
                let mut budget_chirho = CERTIFICATE_BUDGET_CHIRHO;
                self.ty_is_family_app_chirho(argument_chirho)
                    && self.family_application_certified_chirho(argument_chirho, &mut budget_chirho)
                    && class_instances_chirho.is_none_or(|instances_chirho| {
                        // Only a bare-variable head can match an application that never reduces.
                        instances_chirho.iter().all(|instance_chirho| {
                            let head_chirho = match position_chirho {
                                0 => Some(&instance_chirho.head_ty_chirho),
                                _ => instance_chirho
                                    .extra_head_tys_chirho
                                    .get(position_chirho - 1),
                            };
                            !matches!(head_chirho, Some(TyChirho::VarChirho(_)))
                        })
                    })
            })
    }

    /// Whether every instance that could match an application that never reduces is visible
    /// here: the class is fully known, or the standard seed produced it and no source declaration
    /// or whole-environment replacement has displaced it. Base declares no bare-variable instance
    /// head for a seeded class.
    fn class_instance_heads_visible_chirho(&self, class_chirho: &str) -> bool {
        self.fully_known_class_names_chirho.contains(class_chirho)
            || self.seeded_class_names_chirho.contains(class_chirho)
    }

    /// A pattern matched `con_chirho`: if its context or refined result is evidence the checker
    /// does not install, or its scheme is unknown, every later signature closure in the module
    /// stays off. A nested signature can inherit the evidence before its own scope begins.
    pub(super) fn note_constructor_pattern_evidence_chirho(&mut self, con_chirho: &NameChirho) {
        if self.unmodeled_pattern_evidence_seen_chirho {
            return;
        }
        let context_free_chirho = self
            .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                &con_chirho.full_name_chirho(),
                con_chirho.text_chirho(),
            )
            .is_some_and(|scheme_chirho| {
                !is_placeholder_import_scheme_chirho(&scheme_chirho)
                    && scheme_chirho.preds_chirho.is_empty()
            });
        if !context_free_chirho
            || self
                .context_constructors_chirho
                .contains(con_chirho.text_chirho())
            || self.constructor_name_refines_chirho(con_chirho)
        {
            self.unmodeled_pattern_evidence_seen_chirho = true;
        }
    }

    /// The constructor-declaration producer's mark: a GADT constructor whose written type
    /// carries a leading context (`MkBox :: Num (F a) => F a -> Box a`). Its scheme drops the
    /// context, so a pattern on it must be counted as unmodeled evidence.
    pub(super) fn note_written_constructor_context_chirho(
        &mut self,
        con_chirho: &NameChirho,
        written_chirho: &TypeChirho,
    ) {
        let mut current_chirho = written_chirho;
        loop {
            match current_chirho {
                TypeChirho::ForallChirho { body_chirho, .. } => current_chirho = body_chirho,
                TypeChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                    ..
                } => {
                    if !context_chirho.is_empty() {
                        self.context_constructors_chirho
                            .insert(con_chirho.text_chirho().to_string());
                        return;
                    }
                    current_chirho = body_chirho;
                }
                _ => return,
            }
        }
    }

    /// The reducer's certificate that a family application cannot reduce under any refinement of
    /// the rigid variables in it. Its head must be an exact registry identity whose definition is
    /// not intrinsic, it must hold no inference variable, and the bounded decision must be
    /// `NoMatch` or `Blocked`. A definition with no equations at all can never match. Otherwise,
    /// every rigid variable must sit inside a nested application that is itself certified, since a
    /// pattern's local refinement of a bare rigid argument could select an equation; a blocker
    /// alone proves nothing. Unknown, held, unverified, intrinsic and exhausted work certifies
    /// nothing.
    fn family_application_certified_chirho(
        &self,
        ty_chirho: &TyChirho,
        budget_chirho: &mut usize,
    ) -> bool {
        let Some(remaining_chirho) = budget_chirho.checked_sub(1) else {
            return false;
        };
        *budget_chirho = remaining_chirho;
        if ty_chirho.contains_var_chirho() {
            return false;
        }
        let (head_chirho, arguments_chirho) = collect_app_spine_chirho(ty_chirho);
        let TyChirho::ConChirho(name_chirho) = &head_chirho else {
            return false;
        };
        let Some(definition_chirho) = self
            .family_name_key_chirho(name_chirho)
            .and_then(|key_chirho| self.type_families_chirho.definition_chirho(&key_chirho))
        else {
            return false;
        };
        if matches!(
            definition_chirho.reduction_chirho(),
            TypeFamilyReductionChirho::IntrinsicChirho(_)
        ) {
            return false;
        }
        let decision_chirho = decide_family_with_limits_chirho(
            definition_chirho,
            &arguments_chirho,
            &|name_chirho| family_head_role_chirho(&self.type_families_chirho, name_chirho),
            FamilyReductionLimitsChirho {
                steps_chirho: 100_000,
                nodes_chirho: 16_384,
                depth_chirho: 128,
            },
        );
        match decision_chirho {
            FamilyReductionDecisionChirho::NoMatchChirho
                if definition_chirho.branches_chirho().is_empty() =>
            {
                true
            }
            FamilyReductionDecisionChirho::NoMatchChirho
            | FamilyReductionDecisionChirho::BlockedChirho => {
                arguments_chirho.iter().all(|argument_chirho| {
                    self.rigid_only_under_certified_chirho(argument_chirho, budget_chirho)
                })
            }
            FamilyReductionDecisionChirho::ReducedChirho(_)
            | FamilyReductionDecisionChirho::UnknownChirho => false,
        }
    }

    /// Whether every rigid variable in `ty_chirho` sits inside a certified family application.
    fn rigid_only_under_certified_chirho(
        &self,
        ty_chirho: &TyChirho,
        budget_chirho: &mut usize,
    ) -> bool {
        if self.ty_is_family_app_chirho(ty_chirho) {
            return self.family_application_certified_chirho(ty_chirho, budget_chirho);
        }
        match ty_chirho {
            TyChirho::ConChirho(_) => true,
            TyChirho::VarChirho(_) | TyChirho::ForallVarChirho(_) => false,
            TyChirho::AppChirho(function_chirho, argument_chirho) => {
                self.rigid_only_under_certified_chirho(function_chirho, budget_chirho)
                    && self.rigid_only_under_certified_chirho(argument_chirho, budget_chirho)
            }
            TyChirho::FunChirho(parameter_chirho, result_chirho, _) => {
                self.rigid_only_under_certified_chirho(parameter_chirho, budget_chirho)
                    && self.rigid_only_under_certified_chirho(result_chirho, budget_chirho)
            }
            TyChirho::TupleChirho(elements_chirho) => {
                elements_chirho.iter().all(|element_chirho| {
                    self.rigid_only_under_certified_chirho(element_chirho, budget_chirho)
                })
            }
            TyChirho::ListChirho(element_chirho) => {
                self.rigid_only_under_certified_chirho(element_chirho, budget_chirho)
            }
            TyChirho::ForallChirho { .. } | TyChirho::RequiredForallChirho { .. } => false,
        }
    }
}
