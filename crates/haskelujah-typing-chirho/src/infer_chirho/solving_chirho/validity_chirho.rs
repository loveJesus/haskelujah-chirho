// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which predicates are certainly unsolvable, and which givens entail a predicate.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::UNSATISFIED_CONSTRAINT_CODE_CHIRHO;
use crate::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use crate::ty_chirho::TyChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    /// Rebound literals still require a separate producer. Polymorphic
    /// parameters now retain quantifiers and qualified contexts per use; their
    /// wanteds are checked inside the corresponding lexical given scope.
    pub(super) const UNFAITHFUL_CONSTRAINT_EXTENSIONS_CHIRHO: &'static [&'static str] =
        &["RebindableSyntax"];

    /// Classes the certainly-unsolvable check must never fire on: solved by
    /// compiler magic (or a user class colliding with such a name), not by
    /// declared instances.
    pub(super) const MAGIC_CLASSES_CHIRHO: &'static [&'static str] = &[
        "Coercible",
        "Typeable",
        "KnownNat",
        "KnownSymbol",
        "KnownChar",
        "HasField",
        "HasCallStack",
        "IP",
        "Unsatisfiable",
        "TypeError",
        "DataToTag",
        "WithDict",
    ];

    /// Outer head constructor name of a type: the base of the application
    /// spine when that base is concrete. Lists, tuples, and functions use
    /// canonical internal keys so both instance heads and predicates map to
    /// the same name.
    pub(super) fn ty_outer_head_name_chirho(ty_chirho: &TyChirho) -> Option<String> {
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => Some(name_chirho.clone()),
            TyChirho::AppChirho(fun_chirho, _) => Self::ty_outer_head_name_chirho(fun_chirho),
            TyChirho::ListChirho(_) => Some("[]".to_string()),
            TyChirho::TupleChirho(elems_chirho) => Some(format!("(tuple/{})", elems_chirho.len())),
            TyChirho::FunChirho(..) => Some("->".to_string()),
            _ => None,
        }
    }

    /// Whether every argument of the predicate is free of unification
    /// variables. A fully-ground predicate is invariant under all later
    /// substitution, so its solvability verdict cannot change.
    pub(super) fn pred_fully_ground_chirho(&self, pred_chirho: &PredChirho) -> bool {
        pred_chirho
            .kind_tys_chirho
            .iter()
            .chain(std::iter::once(&pred_chirho.ty_chirho))
            .chain(&pred_chirho.extra_tys_chirho)
            .all(|ty_chirho| ty_chirho.free_vars_chirho().is_empty())
            && !self.pred_mentions_rigid_chirho(pred_chirho)
    }

    /// Whether an instance could EVER match the predicate, comparing only
    /// outer head constructors per position. A variable-headed instance side
    /// matches anything; concrete heads must agree. Deliberately looser than
    /// real matching so partial matches always suppress the unsolvable error.
    fn instance_could_match_pred_chirho(
        inst_chirho: &InstDeclChirho,
        pred_chirho: &PredChirho,
    ) -> bool {
        if inst_chirho.extra_head_tys_chirho.len() != pred_chirho.extra_tys_chirho.len() {
            return false;
        }
        let position_compatible_chirho = |inst_ty_chirho: &TyChirho, pred_ty_chirho: &TyChirho| {
            match Self::ty_outer_head_name_chirho(inst_ty_chirho) {
                None => true,
                Some(inst_head_chirho) => Self::ty_outer_head_name_chirho(pred_ty_chirho)
                    .is_none_or(|pred_head_chirho| pred_head_chirho == inst_head_chirho),
            }
        };
        if !position_compatible_chirho(&inst_chirho.head_ty_chirho, &pred_chirho.ty_chirho) {
            return false;
        }
        inst_chirho
            .extra_head_tys_chirho
            .iter()
            .zip(pred_chirho.extra_tys_chirho.iter())
            .all(|(inst_extra_chirho, pred_extra_chirho)| {
                position_compatible_chirho(inst_extra_chirho, pred_extra_chirho)
            })
    }

    /// Normalization and apartness borrow the same shrinking proof allowance;
    /// failure settles its debt and leaves the optional certificate Unknown.
    fn normalize_instance_kind_prefix_chirho(
        &self,
        prefix_chirho: &[TyChirho],
        budget_chirho: &mut super::closed_family_apartness_chirho::FamilyUnificationLimitsChirho,
    ) -> Option<Vec<TyChirho>> {
        let mut normalization_chirho = super::EqualityBudgetChirho::limited_chirho(
            budget_chirho.steps_chirho,
            budget_chirho.nodes_chirho,
        );
        let result_chirho = prefix_chirho
            .iter()
            .map(|kind_chirho| {
                self.normalize_signature_public_image_chirho(kind_chirho, &mut normalization_chirho)
            })
            .collect::<Result<Vec<_>, _>>();
        (budget_chirho.steps_chirho, budget_chirho.nodes_chirho) =
            normalization_chirho.remaining_chirho();
        result_chirho.ok()
    }

    /// Only a checked full telescope can refute a wanted by its hidden prefix.
    /// Opaque visible heads and primitive literal policy supply no such proof.
    fn checked_instance_kind_prefixes_refute_chirho(&self, pred_chirho: &PredChirho) -> bool {
        use super::closed_family_apartness_chirho::{
            FamilyHeadRoleChirho, FamilyPatternRelationChirho, FamilyUnificationLimitsChirho,
            family_patterns_relation_with_budget_chirho,
        };
        use crate::ty_chirho::declarations_chirho::TelescopeVisibilityChirho;
        use haskelujah_ast_chirho::name_chirho::{
            ClassIdentityUnknownChirho, InstanceIdentityChirho, PredicateClassProofChirho,
        };
        let usable_chirho = |proof_chirho: &PredicateClassProofChirho| {
            proof_chirho.identity_chirho().is_some()
                || matches!(
                    proof_chirho,
                    PredicateClassProofChirho::UnknownChirho(
                        ClassIdentityUnknownChirho::LegacyChirho
                    )
                )
        };
        if pred_chirho.kind_tys_chirho.is_empty()
            || !self.local_class_prefix_instances_complete_chirho
            || self.type_normalization_exhausted_chirho.get()
            || self.class_env_chirho.registry_error_chirho().is_some()
            || !usable_chirho(pred_chirho.class_identity_chirho())
        {
            return false;
        }
        let mut budget_chirho = FamilyUnificationLimitsChirho::default();
        for (class_chirho, _) in &self.derived_instance_heads_chirho {
            let Some(remaining_chirho) = budget_chirho
                .steps_chirho
                .checked_sub(class_chirho.len().saturating_add(1))
            else {
                return false;
            };
            budget_chirho.steps_chirho = remaining_chirho;
            if class_chirho == &pred_chirho.class_name_chirho {
                return false;
            }
        }
        let Some(proof_chirho) =
            self.type_constructor_scope_chirho
                .as_ref()
                .and_then(|scope_chirho| {
                    scope_chirho.declaration_chirho(&pred_chirho.class_name_chirho)
                })
        else {
            return false;
        };
        let Some(schema_chirho) = self.type_families_chirho.declaration_schemes_chirho()
            .get_chirho(&proof_chirho.type_key_chirho())
            .filter(|schema_chirho| schema_chirho.owner_chirho.is_none()
                && !schema_chirho.held_chirho
                && schema_chirho.role_chirho == haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::ClassChirho)
        else { return false; };
        let Some(class_chirho) = self
            .class_env_chirho
            .classes_chirho()
            .get_chirho(&pred_chirho.class_name_chirho)
        else {
            return false;
        };
        if !usable_chirho(class_chirho.class_identity_chirho()) {
            return false;
        }
        let Some(remaining_chirho) = budget_chirho
            .steps_chirho
            .checked_sub(schema_chirho.binders_chirho.len())
        else {
            return false;
        };
        budget_chirho.steps_chirho = remaining_chirho;
        let hidden_chirho = schema_chirho
            .binders_chirho
            .iter()
            .filter(|binder_chirho| {
                binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
            })
            .count();
        if hidden_chirho != pred_chirho.kind_tys_chirho.len()
            || hidden_chirho != class_chirho.kind_vars_chirho.len()
            || schema_chirho.visible_arity_chirho != pred_chirho.extra_tys_chirho.len() + 1
            || schema_chirho.visible_arity_chirho != class_chirho.extra_vars_chirho.len() + 1
        {
            return false;
        }
        let Some(instances_chirho) = self
            .class_env_chirho
            .instances_chirho()
            .get_chirho(&pred_chirho.class_name_chirho)
            .filter(|instances_chirho| !instances_chirho.is_empty_chirho())
        else {
            return false;
        };
        let Some(wanted_chirho) = self.normalize_instance_kind_prefix_chirho(
            &pred_chirho.kind_tys_chirho,
            &mut budget_chirho,
        ) else {
            return false;
        };
        let apart_chirho = instances_chirho.iter_chirho().all(|instance_chirho| {
            if instance_chirho.kind_tys_chirho.len() != hidden_chirho
                || instance_chirho.extra_head_tys_chirho.len() + 1
                    != schema_chirho.visible_arity_chirho
                || !usable_chirho(instance_chirho.class_identity_chirho())
                || !matches!(
                    instance_chirho.instance_identity_chirho(),
                    InstanceIdentityChirho::SeedChirho(_)
                        | InstanceIdentityChirho::SourceChirho { .. }
                        | InstanceIdentityChirho::UnknownChirho(
                            ClassIdentityUnknownChirho::LegacyChirho
                        )
                )
            {
                return false;
            }
            let Some(prefix_chirho) = self.normalize_instance_kind_prefix_chirho(
                &instance_chirho.kind_tys_chirho,
                &mut budget_chirho,
            ) else {
                return false;
            };
            family_patterns_relation_with_budget_chirho(
                &prefix_chirho,
                &wanted_chirho,
                &|name_chirho| {
                    if self
                        .type_families_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(name_chirho)
                        .is_some_and(|schema_chirho| {
                            schema_chirho.owner_chirho.is_none() && !schema_chirho.held_chirho
                        })
                    {
                        super::family_registration_chirho::family_head_role_chirho(
                            &self.type_families_chirho,
                            name_chirho,
                        )
                    } else {
                        FamilyHeadRoleChirho::UnknownChirho
                    }
                },
                &mut budget_chirho,
            ) == FamilyPatternRelationChirho::SurelyApartChirho
        });
        apart_chirho && !self.type_normalization_exhausted_chirho.get()
    }

    /// A predicate is *certainly unsolvable* when its class's full instance
    /// universe is visible (standard-seeded or declared in this module —
    /// see `fully_known_class_names_chirho`), every argument has a fixed
    /// concrete outer head that is not an unexpanded synonym or stuck type
    /// family, no known or derived instance could ever match those heads, and
    /// the in-scope givens do not entail it. GHC reports exactly these as
    /// "No instance for ..." — accepting them silently is unsound.
    pub(super) fn is_certainly_unsolvable_pred_chirho(&self, pred_chirho: &PredChirho) -> bool {
        self.is_certainly_unsolvable_pred_depth_chirho(pred_chirho, 0)
    }

    /// The verdict a GROUND wanted gets where it is final: GHC's "No instance
    /// for" when it is certainly unsolvable, otherwise none at all. Top-level
    /// generalization and an instance method's leftovers share it, so a method
    /// is judged exactly as a signature-bound binding is. Returns whether the
    /// wanted was reported.
    /// workflow: language-features-chirho/instance-obligations-chirho
    pub(super) fn report_certainly_unsolvable_ground_chirho(
        &mut self,
        pred_chirho: &PredChirho,
        span_chirho: SpanChirho,
    ) -> bool {
        self.pred_fully_ground_chirho(pred_chirho)
            && self.report_certainly_unsolvable_wanted_chirho(pred_chirho, span_chirho)
    }

    /// Generalization cannot hide a wanted whose fixed argument heads no actual
    /// complete class instance can ever match. Open heads and unknown records
    /// remain unproved; non-ground predicates under givens stay in their scope.
    pub(super) fn report_certainly_unsolvable_wanted_chirho(
        &mut self,
        pred_chirho: &PredChirho,
        span_chirho: SpanChirho,
    ) -> bool {
        if !self.is_certainly_unsolvable_pred_chirho(pred_chirho) {
            return false;
        }
        let blamed_chirho = self.unsolvable_blame_pred_chirho(pred_chirho, 0);
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                format!("no instance for `{blamed_chirho}`"),
                span_chirho,
            ));
        true
    }

    /// How deep the sub-goals of matching instances are followed before the
    /// answer becomes "unproved" rather than "unsolvable".
    const UNSOLVABLE_SUB_GOAL_DEPTH_CHIRHO: usize = 8;

    fn is_certainly_unsolvable_pred_depth_chirho(
        &self,
        pred_chirho: &PredChirho,
        depth_chirho: usize,
    ) -> bool {
        if !self.constraint_generation_faithful_chirho
            || matches!(
                pred_chirho.class_identity_chirho(),
                haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho::UnknownChirho(
                    haskelujah_ast_chirho::name_chirho::ClassIdentityUnknownChirho::ResourceChirho(
                        _
                    )
                )
            )
        {
            // Soft metadata refusal never becomes a new ordinary type error.
            return false;
        }
        let class_name_chirho = pred_chirho.class_name_chirho.as_str();
        if class_name_chirho == "~"
            || !self
                .fully_known_class_names_chirho
                .contains(class_name_chirho)
            || Self::MAGIC_CLASSES_CHIRHO.contains(&class_name_chirho)
            || class_name_chirho.starts_with('?')
            || (!self.pred_fully_ground_chirho(pred_chirho) && !self.given_preds_chirho.is_empty())
        {
            return false;
        }
        let normalized_args_chirho: Vec<TyChirho> = std::iter::once(&pred_chirho.ty_chirho)
            .chain(pred_chirho.extra_tys_chirho.iter())
            .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
            .collect();
        let normalized_pred_chirho = PredChirho::from_proof_chirho(
            pred_chirho.class_identity_chirho().clone(),
            crate::class_chirho::PredPartsChirho {
                kind_tys_chirho: pred_chirho
                    .kind_tys_chirho
                    .iter()
                    .map(|kind_chirho| self.normalize_ty_chirho(kind_chirho))
                    .collect(),
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: normalized_args_chirho[0].clone(),
                extra_tys_chirho: normalized_args_chirho[1..].to_vec(),
            },
        );
        if !self.checked_instance_kind_prefixes_refute_chirho(&normalized_pred_chirho) {
            let mut head_names_chirho = Vec::new();
            for arg_chirho in &normalized_args_chirho {
                match Self::ty_outer_head_name_chirho(arg_chirho) {
                    None => return false,
                    Some(head_chirho) => {
                        // Unexpanded synonyms and stuck families could still
                        // reduce to an instance-matching head; `#`-suffixed
                        // primitive tycons (Int#, ByteArray#) go through solver
                        // paths we deliberately mismodel (unboxed literals are
                        // currently typed via Num), so no verdict on either.
                        if self.is_type_family_name_chirho(&head_chirho)
                            || self.type_synonyms_chirho.contains_key(&head_chirho)
                            || head_chirho.ends_with('#')
                        {
                            return false;
                        }
                        if !arg_chirho.free_vars_chirho().is_empty()
                            && !matches!(
                                arg_chirho,
                                TyChirho::FunChirho(..)
                                    | TyChirho::ListChirho(_)
                                    | TyChirho::TupleChirho(_)
                            )
                            && !self
                                .type_families_chirho
                                .declaration_schemes_chirho()
                                .get_chirho(&head_chirho)
                                .and_then(|scheme_chirho| scheme_chirho.declaration_chirho.as_ref())
                                .is_some_and(|declaration_chirho| {
                                    declaration_chirho.is_nominally_generative_chirho()
                                })
                        {
                            // A spelling without a checked nominal producer may be
                            // an unregistered imported family or a legacy placeholder.
                            return false;
                        }
                        head_names_chirho.push(head_chirho);
                    }
                }
            }
            if self
                .derived_instance_heads_chirho
                .contains(&(class_name_chirho.to_string(), head_names_chirho[0].clone()))
                || self
                    .derived_instance_heads_chirho
                    .contains(&(class_name_chirho.to_string(), "*".to_string()))
            {
                return false;
            }
            if let Some(instances_chirho) = self
                .class_env_chirho
                .instances_chirho()
                .get_chirho(class_name_chirho)
            {
                let candidates_chirho: Vec<&InstDeclChirho> = instances_chirho
                    .iter_chirho()
                    .map(|instance_chirho| instance_chirho.as_ref())
                    .filter(|inst_chirho| {
                        Self::instance_could_match_pred_chirho(inst_chirho, &normalized_pred_chirho)
                    })
                    .collect();
                if !candidates_chirho.is_empty() {
                    // An instance whose head matches does not make the predicate
                    // solvable: its own context has to hold too. `instance Convert a
                    // String => Render [a]` cannot serve `Render [Int]` when no
                    // `Convert Int String` exists (GHC-39999 at the use site). The
                    // verdict stays conservative: every candidate must carry a
                    // sub-goal that is itself certainly unsolvable, a candidate we
                    // cannot instantiate gives no verdict, and the depth bound ends
                    // in "unproved".
                    // workflow: language-features-chirho/instance-obligations-chirho
                    if depth_chirho >= Self::UNSOLVABLE_SUB_GOAL_DEPTH_CHIRHO {
                        return false;
                    }
                    let every_candidate_fails_chirho =
                        candidates_chirho.iter().all(|inst_chirho| {
                            match ClassEnvChirho::instance_sub_goals_chirho(
                                inst_chirho,
                                &normalized_pred_chirho,
                            ) {
                                Some(sub_goals_chirho) => {
                                    sub_goals_chirho.iter().any(|goal_chirho| {
                                        self.is_certainly_unsolvable_pred_depth_chirho(
                                            goal_chirho,
                                            depth_chirho + 1,
                                        )
                                    })
                                }
                                None => false,
                            }
                        });
                    if !every_candidate_fails_chirho {
                        return false;
                    }
                }
            }
        }
        let normalized_givens_chirho: Vec<PredChirho> = self
            .given_preds_chirho
            .iter()
            .map(|given_chirho| {
                PredChirho::from_proof_chirho(
                    given_chirho.class_identity_chirho().clone(),
                    crate::class_chirho::PredPartsChirho {
                        kind_tys_chirho: given_chirho
                            .kind_tys_chirho
                            .iter()
                            .map(|kind_chirho| self.normalize_ty_chirho(kind_chirho))
                            .collect(),
                        class_name_chirho: given_chirho.class_name_chirho.clone(),
                        ty_chirho: self.normalize_ty_chirho(&given_chirho.ty_chirho),
                        extra_tys_chirho: given_chirho
                            .extra_tys_chirho
                            .iter()
                            .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
                            .collect(),
                    },
                )
            })
            .collect();
        if self.pred_entailed_by_givens_chirho(pred_chirho, &self.given_preds_chirho)
            || self
                .pred_entailed_by_givens_chirho(&normalized_pred_chirho, &normalized_givens_chirho)
        {
            return false;
        }
        true
    }

    /// The predicate to NAME when one is certainly unsolvable: the sub-goal that
    /// makes it so, or the predicate itself. GHC reports the missing sub-goal
    /// ("No instance for `Convert Int String` arising from a use of `render`"),
    /// not the constraint that led to it.
    /// workflow: language-features-chirho/instance-obligations-chirho
    pub(super) fn unsolvable_blame_pred_chirho(
        &self,
        pred_chirho: &PredChirho,
        depth_chirho: usize,
    ) -> PredChirho {
        if depth_chirho >= Self::UNSOLVABLE_SUB_GOAL_DEPTH_CHIRHO {
            return pred_chirho.clone();
        }
        let normalized_pred_chirho = PredChirho::from_proof_chirho(
            pred_chirho.class_identity_chirho().clone(),
            crate::class_chirho::PredPartsChirho {
                kind_tys_chirho: pred_chirho
                    .kind_tys_chirho
                    .iter()
                    .map(|kind_chirho| self.normalize_ty_chirho(kind_chirho))
                    .collect(),
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: self.normalize_ty_chirho(&pred_chirho.ty_chirho),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|ty_chirho| self.normalize_ty_chirho(ty_chirho))
                    .collect(),
            },
        );
        let Some(instances_chirho) = self
            .class_env_chirho
            .instances_chirho()
            .get_chirho(pred_chirho.class_name_chirho.as_str())
        else {
            return pred_chirho.clone();
        };
        for inst_chirho in instances_chirho.iter_chirho().filter(|inst_chirho| {
            Self::instance_could_match_pred_chirho(inst_chirho, &normalized_pred_chirho)
        }) {
            let Some(sub_goals_chirho) =
                ClassEnvChirho::instance_sub_goals_chirho(inst_chirho, &normalized_pred_chirho)
            else {
                continue;
            };
            if let Some(goal_chirho) = sub_goals_chirho.iter().find(|goal_chirho| {
                self.is_certainly_unsolvable_pred_depth_chirho(goal_chirho, depth_chirho + 1)
            }) {
                return self.unsolvable_blame_pred_chirho(goal_chirho, depth_chirho + 1);
            }
        }
        pred_chirho.clone()
    }

    pub(super) fn pred_entailed_by_givens_chirho(
        &self,
        pred_chirho: &PredChirho,
        givens_chirho: &[PredChirho],
    ) -> bool {
        givens_chirho.iter().any(|given_chirho| {
            pred_chirho.ty_chirho == given_chirho.ty_chirho
                && pred_chirho.extra_tys_chirho == given_chirho.extra_tys_chirho
                && (pred_chirho.class_name_chirho == given_chirho.class_name_chirho
                    || self.is_transitive_superclass_chirho(
                        &pred_chirho.class_name_chirho,
                        &given_chirho.class_name_chirho,
                    ))
        })
    }
}
