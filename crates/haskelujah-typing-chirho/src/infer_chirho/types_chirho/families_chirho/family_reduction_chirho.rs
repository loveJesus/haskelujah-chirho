// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pure, bounded reduction of one nominal type-family definition (row 686b).
//!
//! The classifier must carry positive, canonical identity evidence. Unknown constructors and
//! unsupported syntax remain stuck. Matching is one-way: only equation variables receive
//! bindings; target variables, including inference metas, remain rigid identities here.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use super::closed_family_apartness_chirho::{
    FamilyHeadRoleChirho, FamilyPatternRelationChirho, FamilyUnificationLimitsChirho,
    GenerativeIdentityChirho, family_patterns_relation_with_limits_chirho,
};
use super::family_definitions_chirho::{TypeFamilyDefinitionChirho, TypeFamilyReductionChirho};
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitChirho as MatchLimitChirho;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub(super) struct FamilyReductionLimitsChirho {
    pub(super) steps_chirho: usize,
    pub(super) nodes_chirho: usize,
    pub(super) depth_chirho: usize,
}

impl Default for FamilyReductionLimitsChirho {
    fn default() -> Self {
        Self {
            steps_chirho: 1_000_000,
            nodes_chirho: 65_536,
            depth_chirho: 128,
        }
    }
}

#[cfg(test)]
pub(super) fn reduce_family_definition_chirho(
    definition_chirho: &TypeFamilyDefinitionChirho,
    arguments_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
) -> Option<TyChirho> {
    reduce_family_with_limits_chirho(
        definition_chirho,
        arguments_chirho,
        classify_chirho,
        FamilyReductionLimitsChirho::default(),
    )
}

/// A completed bounded comparison is distinct from unsupported or exhausted work.
/// `Blocked` is NOT itself a class-unsolvability proof: callers must retain the
/// signature's rigid scope and establish irreducibility of nested applications.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum FamilyReductionDecisionChirho {
    ReducedChirho(TyChirho),
    NoMatchChirho,
    BlockedChirho,
    UnknownChirho,
}

pub(super) fn decide_family_with_limits_chirho(
    definition_chirho: &TypeFamilyDefinitionChirho,
    arguments_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
    limits_chirho: FamilyReductionLimitsChirho,
) -> FamilyReductionDecisionChirho {
    let mut limits_chirho = limits_chirho;
    try_family_decision_chirho(
        definition_chirho,
        arguments_chirho,
        classify_chirho,
        &mut limits_chirho,
        false,
    )
    .unwrap_or(FamilyReductionDecisionChirho::UnknownChirho)
}

/// Borrow one continuation allowance. Partial failures retain all consumed work
/// and output; returned strings are preflighted against this same output counter.
/// The value-taking compatibility API above keeps its original node-only contract.
pub(super) fn decide_family_with_budget_chirho(
    definition_chirho: &TypeFamilyDefinitionChirho,
    arguments_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
    limits_chirho: &mut FamilyReductionLimitsChirho,
) -> Result<FamilyReductionDecisionChirho, MatchFailureChirho> {
    try_family_decision_chirho(
        definition_chirho,
        arguments_chirho,
        classify_chirho,
        limits_chirho,
        true,
    )
}

#[cfg(test)]
pub(super) fn reduce_family_with_limits_chirho(
    definition_chirho: &TypeFamilyDefinitionChirho,
    arguments_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
    limits_chirho: FamilyReductionLimitsChirho,
) -> Option<TyChirho> {
    match decide_family_with_limits_chirho(
        definition_chirho,
        arguments_chirho,
        classify_chirho,
        limits_chirho,
    ) {
        FamilyReductionDecisionChirho::ReducedChirho(result_chirho) => Some(result_chirho),
        _ => None,
    }
}

fn try_family_decision_chirho(
    definition_chirho: &TypeFamilyDefinitionChirho,
    arguments_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
    limits_chirho: &mut FamilyReductionLimitsChirho,
    materialization_bytes_chirho: bool,
) -> Result<FamilyReductionDecisionChirho, MatchFailureChirho> {
    let reduction_chirho = definition_chirho.reduction_chirho();
    if matches!(
        reduction_chirho,
        TypeFamilyReductionChirho::IntrinsicChirho(_)
    ) || arguments_chirho.len() < definition_chirho.arity_chirho()
        || (reduction_chirho == TypeFamilyReductionChirho::OpenChirho
            && !definition_chirho.open_coherence_verified_chirho())
    {
        return Ok(FamilyReductionDecisionChirho::UnknownChirho);
    }
    let (prefix_chirho, extras_chirho) =
        arguments_chirho.split_at(definition_chirho.arity_chirho());
    let mut matcher_chirho = FamilyMatcherChirho::new_chirho(
        classify_chirho,
        limits_chirho,
        materialization_bytes_chirho,
    );
    if reduction_chirho == TypeFamilyReductionChirho::OpenChirho {
        for branch_chirho in definition_chirho.branches_chirho() {
            matcher_chirho.tick_chirho(0)?;
            if branch_chirho.held_chirho {
                return Ok(FamilyReductionDecisionChirho::UnknownChirho);
            }
        }
    }
    let mut targets_chirho = Vec::new();
    for argument_chirho in arguments_chirho {
        targets_chirho.push(matcher_chirho.convert_chirho(
            argument_chirho,
            ConversionModeChirho::TargetChirho,
            0,
        )?);
    }
    let mut blocked_chirho = false;
    let mut uncertain_chirho = false;
    for (_index_chirho, branch_chirho) in definition_chirho.reduction_candidates_chirho() {
        matcher_chirho.tick_chirho(0)?;
        let mut patterns_chirho = Vec::new();
        let mut supported_chirho = true;
        for pattern_chirho in &branch_chirho.lhs_chirho {
            match matcher_chirho.convert_chirho(
                pattern_chirho,
                ConversionModeChirho::PatternChirho,
                0,
            ) {
                Ok(pattern_chirho) => patterns_chirho.push(pattern_chirho),
                Err(MatchFailureChirho::UnsupportedChirho) => {
                    supported_chirho = false;
                    uncertain_chirho = true;
                    break;
                }
                Err(error_chirho) => return Err(error_chirho),
            }
        }
        if !supported_chirho {
            continue;
        }
        let bindings_chirho = match matcher_chirho
            .match_chirho(&patterns_chirho, &targets_chirho[..prefix_chirho.len()])
        {
            Ok(Some(bindings_chirho)) => bindings_chirho,
            Ok(None) => continue,
            Err(error_chirho) => return Err(error_chirho),
        };
        let mut apart_chirho = true;
        for blocker_chirho in &branch_chirho.blockers_chirho {
            // Reserve the full kernel allowance, even if it returns early. Thus a large
            // number of blockers cannot multiply an otherwise bounded matching budget.
            const BLOCKER_STEPS_CHIRHO: usize = 4096;
            matcher_chirho.charge_chirho(BLOCKER_STEPS_CHIRHO)?;
            let blocker_chirho = definition_chirho
                .branches_chirho()
                .get(*blocker_chirho)
                .ok_or(MatchFailureChirho::UnsupportedChirho)?;
            if blocker_chirho.held_chirho {
                uncertain_chirho = true;
                apart_chirho = false;
                break;
            }
            let verdict_chirho = family_patterns_relation_with_limits_chirho(
                &blocker_chirho.lhs_chirho,
                prefix_chirho,
                classify_chirho,
                FamilyUnificationLimitsChirho {
                    steps_chirho: BLOCKER_STEPS_CHIRHO,
                    nodes_chirho: matcher_chirho.limits_chirho.nodes_chirho,
                    depth_chirho: matcher_chirho.limits_chirho.depth_chirho,
                },
            );
            if verdict_chirho != FamilyPatternRelationChirho::SurelyApartChirho {
                uncertain_chirho |= verdict_chirho == FamilyPatternRelationChirho::UnknownChirho;
                blocked_chirho |= matches!(
                    verdict_chirho,
                    FamilyPatternRelationChirho::UnifiableChirho
                        | FamilyPatternRelationChirho::KnownFamilyOverlapChirho
                );
                apart_chirho = false;
                break;
            }
        }
        if !apart_chirho {
            continue;
        }
        let rhs_chirho = matcher_chirho.convert_chirho(
            &branch_chirho.rhs_chirho,
            ConversionModeChirho::ResultChirho,
            0,
        )?;
        let mut result_chirho =
            matcher_chirho.materialize_chirho(rhs_chirho, &bindings_chirho, extras_chirho.len())?;
        for (index_chirho, extra_chirho) in targets_chirho[prefix_chirho.len()..].iter().enumerate()
        {
            let depth_chirho = extras_chirho.len() - index_chirho;
            matcher_chirho.output_chirho(depth_chirho - 1)?;
            result_chirho = TyChirho::AppChirho(
                Box::new(result_chirho),
                Box::new(matcher_chirho.materialize_chirho(
                    *extra_chirho,
                    &bindings_chirho,
                    depth_chirho,
                )?),
            );
        }
        return Ok(FamilyReductionDecisionChirho::ReducedChirho(result_chirho));
    }
    Ok(
        if uncertain_chirho
            || definition_chirho
                .branches_chirho()
                .iter()
                .any(|branch_chirho| branch_chirho.held_chirho)
        {
            FamilyReductionDecisionChirho::UnknownChirho
        } else if blocked_chirho {
            FamilyReductionDecisionChirho::BlockedChirho
        } else {
            FamilyReductionDecisionChirho::NoMatchChirho
        },
    )
}

type MatchNodeIdChirho = usize;
type MatchBindingsChirho = HashMap<PatternVariableChirho, MatchNodeIdChirho>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum PatternVariableChirho {
    NumericChirho(TyVarChirho),
    NamedChirho(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum MatchNodeChirho {
    PatternVariableChirho(PatternVariableChirho),
    TargetVariableChirho(PatternVariableChirho),
    OpaqueTargetChirho(std::sync::Arc<TyChirho>),
    ConstructorChirho(GenerativeIdentityChirho),
    ApplicationChirho(MatchNodeIdChirho, MatchNodeIdChirho),
    FamilyChirho {
        identity_chirho: String,
        arguments_chirho: Vec<MatchNodeIdChirho>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ConversionModeChirho {
    PatternChirho,
    TargetChirho,
    ResultChirho,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MatchFailureChirho {
    UnsupportedChirho,
    LimitChirho(MatchLimitChirho),
}

struct FamilyMatcherChirho<'classifier_chirho, 'limits_chirho, ClassifyChirho> {
    classify_chirho: &'classifier_chirho ClassifyChirho,
    limits_chirho: &'limits_chirho mut FamilyReductionLimitsChirho,
    materialization_bytes_chirho: bool,
    nodes_chirho: Vec<MatchNodeChirho>,
    interned_chirho: HashMap<MatchNodeChirho, MatchNodeIdChirho>,
    nominal_names_chirho: HashMap<GenerativeIdentityChirho, String>,
    family_names_chirho: HashMap<String, String>,
}

impl<'classifier_chirho, 'limits_chirho, ClassifyChirho: Fn(&str) -> FamilyHeadRoleChirho>
    FamilyMatcherChirho<'classifier_chirho, 'limits_chirho, ClassifyChirho>
{
    fn new_chirho(
        classify_chirho: &'classifier_chirho ClassifyChirho,
        limits_chirho: &'limits_chirho mut FamilyReductionLimitsChirho,
        materialization_bytes_chirho: bool,
    ) -> Self {
        Self {
            classify_chirho,
            limits_chirho,
            materialization_bytes_chirho,
            nodes_chirho: Vec::new(),
            interned_chirho: HashMap::new(),
            nominal_names_chirho: HashMap::new(),
            family_names_chirho: HashMap::new(),
        }
    }

    fn charge_chirho(&mut self, work_chirho: usize) -> Result<(), MatchFailureChirho> {
        self.limits_chirho.steps_chirho = self
            .limits_chirho
            .steps_chirho
            .checked_sub(work_chirho)
            .ok_or(MatchFailureChirho::LimitChirho(
                MatchLimitChirho::WorkChirho,
            ))?;
        Ok(())
    }

    fn tick_chirho(&mut self, depth_chirho: usize) -> Result<(), MatchFailureChirho> {
        if depth_chirho > self.limits_chirho.depth_chirho {
            return Err(MatchFailureChirho::LimitChirho(
                MatchLimitChirho::DepthChirho,
            ));
        }
        self.charge_chirho(1)
    }

    fn child_depth_chirho(&self, depth_chirho: usize) -> Result<usize, MatchFailureChirho> {
        depth_chirho
            .checked_add(1)
            .filter(|depth_chirho| *depth_chirho <= self.limits_chirho.depth_chirho)
            .ok_or(MatchFailureChirho::LimitChirho(
                MatchLimitChirho::DepthChirho,
            ))
    }

    fn output_chirho(&mut self, depth_chirho: usize) -> Result<(), MatchFailureChirho> {
        self.tick_chirho(depth_chirho)?;
        self.limits_chirho.nodes_chirho = self.limits_chirho.nodes_chirho.checked_sub(1).ok_or(
            MatchFailureChirho::LimitChirho(MatchLimitChirho::OutputChirho),
        )?;
        Ok(())
    }

    fn output_bytes_chirho(&mut self, bytes_chirho: usize) -> Result<(), MatchFailureChirho> {
        if self.materialization_bytes_chirho {
            self.limits_chirho.nodes_chirho = self
                .limits_chirho
                .nodes_chirho
                .checked_sub(bytes_chirho)
                .ok_or(MatchFailureChirho::LimitChirho(
                    MatchLimitChirho::OutputChirho,
                ))?;
        }
        Ok(())
    }

    fn materialized_name_chirho(&mut self, bytes_chirho: usize) -> Result<(), MatchFailureChirho> {
        self.charge_chirho(bytes_chirho)?;
        self.output_bytes_chirho(bytes_chirho)
    }

    fn preflight_node_clone_chirho(
        &mut self,
        node_chirho: MatchNodeIdChirho,
    ) -> Result<(), MatchFailureChirho> {
        if !self.materialization_bytes_chirho {
            return Ok(());
        }
        let bytes_chirho = match &self.nodes_chirho[node_chirho] {
            MatchNodeChirho::PatternVariableChirho(PatternVariableChirho::NamedChirho(
                name_chirho,
            ))
            | MatchNodeChirho::TargetVariableChirho(PatternVariableChirho::NamedChirho(
                name_chirho,
            ))
            | MatchNodeChirho::ConstructorChirho(GenerativeIdentityChirho::NominalChirho(
                name_chirho,
            )) => name_chirho.len(),
            MatchNodeChirho::FamilyChirho {
                identity_chirho,
                arguments_chirho,
            } => identity_chirho
                .len()
                .checked_add(arguments_chirho.len())
                .ok_or(MatchFailureChirho::LimitChirho(
                    MatchLimitChirho::OutputChirho,
                ))?,
            _ => 0,
        };
        self.materialized_name_chirho(bytes_chirho)
    }

    fn intern_chirho(
        &mut self,
        node_chirho: MatchNodeChirho,
    ) -> Result<MatchNodeIdChirho, MatchFailureChirho> {
        self.tick_chirho(0)?;
        let width_chirho = match &node_chirho {
            MatchNodeChirho::PatternVariableChirho(PatternVariableChirho::NamedChirho(
                name_chirho,
            ))
            | MatchNodeChirho::TargetVariableChirho(PatternVariableChirho::NamedChirho(
                name_chirho,
            ))
            | MatchNodeChirho::ConstructorChirho(GenerativeIdentityChirho::NominalChirho(
                name_chirho,
            )) => name_chirho.len(),
            MatchNodeChirho::FamilyChirho {
                identity_chirho,
                arguments_chirho,
            } => identity_chirho.len().saturating_add(arguments_chirho.len()),
            _ => 0,
        };
        self.charge_chirho(width_chirho)?;
        if let Some(node_chirho) = self.interned_chirho.get(&node_chirho) {
            return Ok(*node_chirho);
        }
        self.output_chirho(0)?;
        let id_chirho = self.nodes_chirho.len();
        self.nodes_chirho.push(node_chirho.clone());
        self.interned_chirho.insert(node_chirho, id_chirho);
        Ok(id_chirho)
    }

    fn role_chirho(
        &mut self,
        name_chirho: &str,
    ) -> Result<FamilyHeadRoleChirho, MatchFailureChirho> {
        self.charge_chirho(name_chirho.len())?;
        let syntax_chirho = match name_chirho {
            "[]" => Some(GenerativeIdentityChirho::ListChirho),
            "()" => Some(GenerativeIdentityChirho::TupleChirho(0)),
            "->" => Some(GenerativeIdentityChirho::ArrowChirho(
                MultChirho::ManyChirho,
            )),
            _ if name_chirho.starts_with('(')
                && name_chirho.ends_with(')')
                && name_chirho.len() > 2
                && name_chirho[1..name_chirho.len() - 1]
                    .chars()
                    .all(|character_chirho| character_chirho == ',') =>
            {
                Some(GenerativeIdentityChirho::TupleChirho(name_chirho.len() - 1))
            }
            _ => None,
        };
        Ok(match syntax_chirho {
            Some(identity_chirho) => FamilyHeadRoleChirho::GenerativeChirho { identity_chirho },
            None => (self.classify_chirho)(name_chirho),
        })
    }

    fn constructor_chirho(
        &mut self,
        name_chirho: &str,
        mode_chirho: ConversionModeChirho,
    ) -> Result<MatchNodeIdChirho, MatchFailureChirho> {
        match self.role_chirho(name_chirho)? {
            FamilyHeadRoleChirho::GenerativeChirho { identity_chirho } => {
                self.nominal_names_chirho
                    .entry(identity_chirho.clone())
                    .or_insert_with(|| name_chirho.to_owned());
                self.intern_chirho(MatchNodeChirho::ConstructorChirho(identity_chirho))
            }
            FamilyHeadRoleChirho::FamilyChirho {
                identity_chirho,
                arity_chirho: 0,
            } if mode_chirho != ConversionModeChirho::PatternChirho => {
                self.family_names_chirho
                    .entry(identity_chirho.clone())
                    .or_insert_with(|| name_chirho.to_owned());
                self.intern_chirho(MatchNodeChirho::FamilyChirho {
                    identity_chirho,
                    arguments_chirho: Vec::new(),
                })
            }
            _ => Err(MatchFailureChirho::UnsupportedChirho),
        }
    }

    fn convert_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        mode_chirho: ConversionModeChirho,
        depth_chirho: usize,
    ) -> Result<MatchNodeIdChirho, MatchFailureChirho> {
        self.tick_chirho(depth_chirho)?;
        match ty_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                let variable_chirho = PatternVariableChirho::NumericChirho(*variable_chirho);
                self.intern_chirho(if mode_chirho == ConversionModeChirho::TargetChirho {
                    MatchNodeChirho::TargetVariableChirho(variable_chirho)
                } else {
                    MatchNodeChirho::PatternVariableChirho(variable_chirho)
                })
            }
            TyChirho::ForallVarChirho(name_chirho) => {
                self.charge_chirho(name_chirho.len())?;
                let variable_chirho = PatternVariableChirho::NamedChirho(name_chirho.clone());
                self.intern_chirho(if mode_chirho == ConversionModeChirho::TargetChirho {
                    MatchNodeChirho::TargetVariableChirho(variable_chirho)
                } else {
                    MatchNodeChirho::PatternVariableChirho(variable_chirho)
                })
            }
            TyChirho::ConChirho(name_chirho) => self.constructor_chirho(name_chirho, mode_chirho),
            TyChirho::AppChirho(_, _) => {
                let mut head_chirho = ty_chirho;
                let mut arguments_chirho = Vec::new();
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    let spine_depth_chirho =
                        depth_chirho.checked_add(arguments_chirho.len()).ok_or(
                            MatchFailureChirho::LimitChirho(MatchLimitChirho::DepthChirho),
                        )?;
                    self.tick_chirho(spine_depth_chirho)?;
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                let head_depth_chirho = depth_chirho.checked_add(arguments_chirho.len()).ok_or(
                    MatchFailureChirho::LimitChirho(MatchLimitChirho::DepthChirho),
                )?;
                self.tick_chirho(head_depth_chirho)?;
                if let TyChirho::ConChirho(name_chirho) = head_chirho
                    && let FamilyHeadRoleChirho::FamilyChirho {
                        identity_chirho,
                        arity_chirho,
                    } = self.role_chirho(name_chirho)?
                {
                    if mode_chirho == ConversionModeChirho::PatternChirho
                        || arguments_chirho.len() < arity_chirho
                    {
                        return Err(MatchFailureChirho::UnsupportedChirho);
                    }
                    self.family_names_chirho
                        .entry(identity_chirho.clone())
                        .or_insert_with(|| name_chirho.clone());
                    let mut converted_chirho = Vec::new();
                    for (index_chirho, argument_chirho) in
                        arguments_chirho[..arity_chirho].iter().enumerate()
                    {
                        converted_chirho.push(self.convert_chirho(
                            argument_chirho,
                            mode_chirho,
                            head_depth_chirho - index_chirho,
                        )?);
                    }
                    let mut result_chirho = self.intern_chirho(MatchNodeChirho::FamilyChirho {
                        identity_chirho,
                        arguments_chirho: converted_chirho,
                    })?;
                    for (index_chirho, argument_chirho) in
                        arguments_chirho[arity_chirho..].iter().enumerate()
                    {
                        let argument_chirho = self.convert_chirho(
                            argument_chirho,
                            mode_chirho,
                            head_depth_chirho - arity_chirho - index_chirho,
                        )?;
                        result_chirho = self.intern_chirho(MatchNodeChirho::ApplicationChirho(
                            result_chirho,
                            argument_chirho,
                        ))?;
                    }
                    return Ok(result_chirho);
                }
                let mut result_chirho =
                    self.convert_chirho(head_chirho, mode_chirho, head_depth_chirho)?;
                for (index_chirho, argument_chirho) in arguments_chirho.into_iter().enumerate() {
                    let argument_chirho = self.convert_chirho(
                        argument_chirho,
                        mode_chirho,
                        head_depth_chirho - index_chirho,
                    )?;
                    result_chirho = self.intern_chirho(MatchNodeChirho::ApplicationChirho(
                        result_chirho,
                        argument_chirho,
                    ))?;
                }
                Ok(result_chirho)
            }
            TyChirho::ListChirho(element_chirho) => {
                let head_chirho = self.intern_chirho(MatchNodeChirho::ConstructorChirho(
                    GenerativeIdentityChirho::ListChirho,
                ))?;
                let child_chirho = self.child_depth_chirho(depth_chirho)?;
                let element_chirho =
                    self.convert_chirho(element_chirho, mode_chirho, child_chirho)?;
                self.intern_chirho(MatchNodeChirho::ApplicationChirho(
                    head_chirho,
                    element_chirho,
                ))
            }
            TyChirho::TupleChirho(elements_chirho) => {
                if elements_chirho.len() == 1 {
                    return Err(MatchFailureChirho::UnsupportedChirho);
                }
                let child_chirho = self.child_depth_chirho(depth_chirho)?;
                let mut result_chirho = self.intern_chirho(MatchNodeChirho::ConstructorChirho(
                    GenerativeIdentityChirho::TupleChirho(elements_chirho.len()),
                ))?;
                for element_chirho in elements_chirho {
                    let element_chirho =
                        self.convert_chirho(element_chirho, mode_chirho, child_chirho)?;
                    result_chirho = self.intern_chirho(MatchNodeChirho::ApplicationChirho(
                        result_chirho,
                        element_chirho,
                    ))?;
                }
                Ok(result_chirho)
            }
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                let child_chirho = self.child_depth_chirho(depth_chirho)?;
                let head_chirho = self.intern_chirho(MatchNodeChirho::ConstructorChirho(
                    GenerativeIdentityChirho::ArrowChirho(*multiplicity_chirho),
                ))?;
                let argument_chirho =
                    self.convert_chirho(argument_chirho, mode_chirho, child_chirho)?;
                let result_chirho =
                    self.convert_chirho(result_chirho, mode_chirho, child_chirho)?;
                let head_chirho = self.intern_chirho(MatchNodeChirho::ApplicationChirho(
                    head_chirho,
                    argument_chirho,
                ))?;
                self.intern_chirho(MatchNodeChirho::ApplicationChirho(
                    head_chirho,
                    result_chirho,
                ))
            }
            TyChirho::QualChirho { .. }
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => {
                if mode_chirho != ConversionModeChirho::TargetChirho {
                    return Err(MatchFailureChirho::UnsupportedChirho);
                }
                // One payload copy and the intern table's two full hashes.
                for _copy_chirho in 0..3 {
                    self.charge_opaque_chirho(ty_chirho, depth_chirho)?;
                }
                self.intern_chirho(MatchNodeChirho::OpaqueTargetChirho(std::sync::Arc::new(
                    ty_chirho.clone(),
                )))
            }
        }
    }

    fn charge_opaque_chirho(
        &mut self,
        term_chirho: &TyChirho,
        depth_chirho: usize,
    ) -> Result<(), MatchFailureChirho> {
        let mut pending_chirho = vec![(term_chirho, depth_chirho)];
        while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
            self.output_chirho(depth_chirho)?;
            match term_chirho {
                TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                    self.materialized_name_chirho(name_chirho.len())?
                }
                TyChirho::VarChirho(_) => {}
                TyChirho::AppChirho(left_chirho, right_chirho)
                | TyChirho::FunChirho(left_chirho, right_chirho, _) => pending_chirho.extend([
                    (left_chirho.as_ref(), depth_chirho + 1),
                    (right_chirho.as_ref(), depth_chirho + 1),
                ]),
                TyChirho::ListChirho(element_chirho) => {
                    pending_chirho.push((element_chirho, depth_chirho + 1))
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    self.charge_chirho(elements_chirho.len())?;
                    pending_chirho.extend(
                        elements_chirho
                            .iter()
                            .map(|element_chirho| (element_chirho, depth_chirho + 1)),
                    );
                }
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    self.charge_chirho(vars_chirho.len())?;
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                }
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    self.charge_chirho(context_chirho.len())?;
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                    pending_chirho.extend(
                        context_chirho
                            .iter()
                            .map(|constraint_chirho| (constraint_chirho, depth_chirho + 1)),
                    );
                }
            }
        }
        Ok(())
    }

    fn match_chirho(
        &mut self,
        patterns_chirho: &[MatchNodeIdChirho],
        targets_chirho: &[MatchNodeIdChirho],
    ) -> Result<Option<MatchBindingsChirho>, MatchFailureChirho> {
        if patterns_chirho.len() != targets_chirho.len() {
            return Ok(None);
        }
        let mut pending_chirho: Vec<_> = patterns_chirho
            .iter()
            .copied()
            .zip(targets_chirho.iter().copied())
            .collect();
        let mut bindings_chirho = HashMap::new();
        while let Some((pattern_chirho, target_chirho)) = pending_chirho.pop() {
            self.tick_chirho(0)?;
            match (
                &self.nodes_chirho[pattern_chirho],
                &self.nodes_chirho[target_chirho],
            ) {
                (MatchNodeChirho::PatternVariableChirho(variable_chirho), _) => {
                    if let Some(prior_chirho) = bindings_chirho.get(variable_chirho) {
                        if *prior_chirho != target_chirho {
                            return Ok(None);
                        }
                    } else {
                        bindings_chirho.insert(variable_chirho.clone(), target_chirho);
                    }
                }
                (
                    MatchNodeChirho::ConstructorChirho(left_chirho),
                    MatchNodeChirho::ConstructorChirho(right_chirho),
                ) if left_chirho == right_chirho => {}
                (
                    MatchNodeChirho::ApplicationChirho(left_function_chirho, left_argument_chirho),
                    MatchNodeChirho::ApplicationChirho(
                        right_function_chirho,
                        right_argument_chirho,
                    ),
                ) => {
                    pending_chirho.push((*left_function_chirho, *right_function_chirho));
                    pending_chirho.push((*left_argument_chirho, *right_argument_chirho));
                }
                _ => return Ok(None),
            }
        }
        Ok(Some(bindings_chirho))
    }

    fn bound_node_chirho(
        &mut self,
        node_chirho: MatchNodeIdChirho,
        bindings_chirho: &MatchBindingsChirho,
    ) -> Result<MatchNodeIdChirho, MatchFailureChirho> {
        self.tick_chirho(0)?;
        if let MatchNodeChirho::PatternVariableChirho(variable_chirho) =
            &self.nodes_chirho[node_chirho]
        {
            bindings_chirho
                .get(variable_chirho)
                .copied()
                .ok_or(MatchFailureChirho::UnsupportedChirho)
        } else {
            Ok(node_chirho)
        }
    }

    fn materialize_chirho(
        &mut self,
        node_chirho: MatchNodeIdChirho,
        bindings_chirho: &MatchBindingsChirho,
        depth_chirho: usize,
    ) -> Result<TyChirho, MatchFailureChirho> {
        self.output_chirho(depth_chirho)?;
        let node_chirho = self.bound_node_chirho(node_chirho, bindings_chirho)?;
        self.preflight_node_clone_chirho(node_chirho)?;
        match self.nodes_chirho[node_chirho].clone() {
            MatchNodeChirho::TargetVariableChirho(PatternVariableChirho::NumericChirho(
                variable_chirho,
            )) => Ok(TyChirho::VarChirho(variable_chirho)),
            MatchNodeChirho::TargetVariableChirho(PatternVariableChirho::NamedChirho(
                name_chirho,
            )) => {
                if !self.materialization_bytes_chirho {
                    self.charge_chirho(name_chirho.len())?;
                }
                Ok(TyChirho::ForallVarChirho(name_chirho))
            }
            MatchNodeChirho::OpaqueTargetChirho(term_chirho) => {
                self.charge_opaque_chirho(&term_chirho, depth_chirho)?;
                Ok(term_chirho.as_ref().clone())
            }
            MatchNodeChirho::ConstructorChirho(identity_chirho) => {
                self.constructor_output_chirho(&identity_chirho)
            }
            MatchNodeChirho::FamilyChirho {
                identity_chirho,
                arguments_chirho,
            } => {
                let name_chirho = self
                    .family_names_chirho
                    .get(&identity_chirho)
                    .ok_or(MatchFailureChirho::UnsupportedChirho)?;
                self.materialized_name_chirho(name_chirho.len())?;
                let mut result_chirho =
                    TyChirho::ConChirho(self.family_names_chirho[&identity_chirho].clone());
                let head_depth_chirho = depth_chirho.checked_add(arguments_chirho.len()).ok_or(
                    MatchFailureChirho::LimitChirho(MatchLimitChirho::DepthChirho),
                )?;
                self.tick_chirho(head_depth_chirho)?;
                for (index_chirho, argument_chirho) in arguments_chirho.into_iter().enumerate() {
                    let child_chirho = head_depth_chirho - index_chirho;
                    self.output_chirho(child_chirho - 1)?;
                    result_chirho = TyChirho::AppChirho(
                        Box::new(result_chirho),
                        Box::new(self.materialize_chirho(
                            argument_chirho,
                            bindings_chirho,
                            child_chirho,
                        )?),
                    );
                }
                Ok(result_chirho)
            }
            MatchNodeChirho::ApplicationChirho(_, _) => {
                self.application_output_chirho(node_chirho, bindings_chirho, depth_chirho)
            }
            MatchNodeChirho::PatternVariableChirho(_) => Err(MatchFailureChirho::UnsupportedChirho),
        }
    }

    fn constructor_output_chirho(
        &mut self,
        identity_chirho: &GenerativeIdentityChirho,
    ) -> Result<TyChirho, MatchFailureChirho> {
        let name_chirho = match identity_chirho {
            GenerativeIdentityChirho::ListChirho => {
                if self.materialization_bytes_chirho {
                    self.materialized_name_chirho(2)?;
                }
                "[]".to_owned()
            }
            GenerativeIdentityChirho::TupleChirho(0) => {
                return Ok(TyChirho::TupleChirho(Vec::new()));
            }
            GenerativeIdentityChirho::TupleChirho(arity_chirho) => {
                if *arity_chirho < 2 {
                    return Err(MatchFailureChirho::UnsupportedChirho);
                }
                self.charge_chirho(*arity_chirho)?;
                self.output_bytes_chirho(arity_chirho.checked_mul(2).ok_or(
                    MatchFailureChirho::LimitChirho(MatchLimitChirho::OutputChirho),
                )?)?;
                format!("({})", ",".repeat(arity_chirho - 1))
            }
            GenerativeIdentityChirho::ArrowChirho(MultChirho::ManyChirho) => {
                if self.materialization_bytes_chirho {
                    self.materialized_name_chirho(2)?;
                }
                "->".to_owned()
            }
            GenerativeIdentityChirho::ArrowChirho(MultChirho::OneChirho) => {
                return Err(MatchFailureChirho::UnsupportedChirho);
            }
            GenerativeIdentityChirho::NominalChirho(_) => {
                let name_chirho = self
                    .nominal_names_chirho
                    .get(identity_chirho)
                    .ok_or(MatchFailureChirho::UnsupportedChirho)?;
                self.materialized_name_chirho(name_chirho.len())?;
                self.nominal_names_chirho[identity_chirho].clone()
            }
        };
        Ok(TyChirho::ConChirho(name_chirho))
    }

    fn application_output_chirho(
        &mut self,
        node_chirho: MatchNodeIdChirho,
        bindings_chirho: &MatchBindingsChirho,
        depth_chirho: usize,
    ) -> Result<TyChirho, MatchFailureChirho> {
        let mut head_chirho = node_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            let spine_depth_chirho = depth_chirho.checked_add(arguments_chirho.len()).ok_or(
                MatchFailureChirho::LimitChirho(MatchLimitChirho::DepthChirho),
            )?;
            self.tick_chirho(spine_depth_chirho)?;
            head_chirho = self.bound_node_chirho(head_chirho, bindings_chirho)?;
            if let MatchNodeChirho::ApplicationChirho(function_chirho, argument_chirho) =
                self.nodes_chirho[head_chirho]
            {
                arguments_chirho.push(argument_chirho);
                head_chirho = function_chirho;
            } else {
                break;
            }
        }
        arguments_chirho.reverse();
        let head_depth_chirho = depth_chirho.checked_add(arguments_chirho.len()).ok_or(
            MatchFailureChirho::LimitChirho(MatchLimitChirho::DepthChirho),
        )?;
        self.tick_chirho(head_depth_chirho)?;
        self.preflight_node_clone_chirho(head_chirho)?;
        let (mut result_chirho, consumed_chirho) = match self.nodes_chirho[head_chirho].clone() {
            MatchNodeChirho::ConstructorChirho(GenerativeIdentityChirho::ListChirho)
                if !arguments_chirho.is_empty() =>
            {
                (
                    TyChirho::ListChirho(Box::new(self.materialize_chirho(
                        arguments_chirho[0],
                        bindings_chirho,
                        head_depth_chirho,
                    )?)),
                    1,
                )
            }
            MatchNodeChirho::ConstructorChirho(GenerativeIdentityChirho::TupleChirho(
                arity_chirho,
            )) if arity_chirho != 1 && arguments_chirho.len() >= arity_chirho => {
                let mut elements_chirho = Vec::new();
                let child_chirho = head_depth_chirho
                    .checked_sub(arity_chirho)
                    .and_then(|depth_chirho| depth_chirho.checked_add(1))
                    .ok_or(MatchFailureChirho::LimitChirho(
                        MatchLimitChirho::DepthChirho,
                    ))?;
                for argument_chirho in &arguments_chirho[..arity_chirho] {
                    elements_chirho.push(self.materialize_chirho(
                        *argument_chirho,
                        bindings_chirho,
                        child_chirho,
                    )?);
                }
                (TyChirho::TupleChirho(elements_chirho), arity_chirho)
            }
            MatchNodeChirho::ConstructorChirho(GenerativeIdentityChirho::ArrowChirho(
                multiplicity_chirho,
            )) if arguments_chirho.len() >= 2 => (
                TyChirho::FunChirho(
                    Box::new(self.materialize_chirho(
                        arguments_chirho[0],
                        bindings_chirho,
                        head_depth_chirho - 1,
                    )?),
                    Box::new(self.materialize_chirho(
                        arguments_chirho[1],
                        bindings_chirho,
                        head_depth_chirho - 1,
                    )?),
                    multiplicity_chirho,
                ),
                2,
            ),
            _ => (
                self.materialize_chirho(head_chirho, bindings_chirho, head_depth_chirho)?,
                0,
            ),
        };
        for (index_chirho, argument_chirho) in
            arguments_chirho[consumed_chirho..].iter().enumerate()
        {
            let child_chirho = head_depth_chirho - consumed_chirho - index_chirho;
            self.output_chirho(child_chirho - 1)?;
            result_chirho = TyChirho::AppChirho(
                Box::new(result_chirho),
                Box::new(self.materialize_chirho(
                    *argument_chirho,
                    bindings_chirho,
                    child_chirho,
                )?),
            );
        }
        Ok(result_chirho)
    }
}
