// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Conservative closed-family compatibility and apartness (row 686b).
//!
//! The caller supplies authoritative constructor identities and roles. Raw spelling, suffix
//! equality and the general inference unifier are deliberately not evidence of generativity.
//! The three-way contract follows GHC 9.14.1's `GHC.Core.Unify` and `compatibleBranches`:
//! uncertain overlap retains a blocker; only finite unification permits comparing substituted
//! right sides. This leaf neither reduces families nor mutates the inference substitution.

use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use std::collections::{HashMap, VecDeque};

/// Identity strings are opaque. Aliases must receive the same identity from the caller.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum GenerativeIdentityChirho {
    NominalChirho(String),
    ListChirho,
    TupleChirho(usize),
    ArrowChirho(MultChirho),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FamilyHeadRoleChirho {
    GenerativeChirho {
        identity_chirho: GenerativeIdentityChirho,
    },
    FamilyChirho {
        identity_chirho: String,
        arity_chirho: usize,
    },
    UnknownChirho,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FamilyUnificationChirho {
    SurelyApartChirho,
    UnifiableChirho,
    UncertainChirho,
}

/// A known-family abstraction may have a finite overlap witness without proving
/// that the original family applications unify. Reduction retains that blocker;
/// scoped callers may separately establish that its nested applications are stuck.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FamilyPatternRelationChirho {
    SurelyApartChirho,
    UnifiableChirho,
    KnownFamilyOverlapChirho,
    UnknownChirho,
}

/// All work, including canonicalization and chasing substitutions, consumes this budget.
#[derive(Debug, Clone, Copy)]
pub(super) struct FamilyUnificationLimitsChirho {
    pub(super) steps_chirho: usize,
    pub(super) nodes_chirho: usize,
    pub(super) depth_chirho: usize,
}

impl Default for FamilyUnificationLimitsChirho {
    fn default() -> Self {
        Self {
            steps_chirho: 100_000,
            nodes_chirho: 16_384,
            depth_chirho: 128,
        }
    }
}

#[cfg(test)]
pub(super) fn family_patterns_unification_chirho(
    left_chirho: &[TyChirho],
    right_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
) -> FamilyUnificationChirho {
    family_patterns_with_limits_chirho(
        left_chirho,
        right_chirho,
        classify_chirho,
        FamilyUnificationLimitsChirho::default(),
    )
}

#[cfg(test)]
pub(super) fn family_patterns_with_limits_chirho(
    left_chirho: &[TyChirho],
    right_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
    limits_chirho: FamilyUnificationLimitsChirho,
) -> FamilyUnificationChirho {
    match family_patterns_relation_with_limits_chirho(
        left_chirho,
        right_chirho,
        classify_chirho,
        limits_chirho,
    ) {
        FamilyPatternRelationChirho::SurelyApartChirho => {
            FamilyUnificationChirho::SurelyApartChirho
        }
        FamilyPatternRelationChirho::UnifiableChirho => FamilyUnificationChirho::UnifiableChirho,
        FamilyPatternRelationChirho::KnownFamilyOverlapChirho
        | FamilyPatternRelationChirho::UnknownChirho => FamilyUnificationChirho::UncertainChirho,
    }
}

pub(super) fn family_patterns_relation_with_limits_chirho(
    left_chirho: &[TyChirho],
    right_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
    mut limits_chirho: FamilyUnificationLimitsChirho,
) -> FamilyPatternRelationChirho {
    family_patterns_relation_with_budget_chirho(
        left_chirho,
        right_chirho,
        classify_chirho,
        &mut limits_chirho,
    )
}

/// Share one existing proof allowance across alternatives. Unknown and partial
/// failures retain their actual work and DAG-node debt before returning.
pub(super) fn family_patterns_relation_with_budget_chirho(
    left_chirho: &[TyChirho],
    right_chirho: &[TyChirho],
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
    limits_chirho: &mut FamilyUnificationLimitsChirho,
) -> FamilyPatternRelationChirho {
    let mut kernel_chirho = ApartnessKernelChirho::new_chirho(classify_chirho, *limits_chirho);
    let mut left_context_chirho = kernel_chirho.context_chirho();
    let mut right_context_chirho = kernel_chirho.context_chirho();
    let result_chirho = kernel_chirho.unify_inputs_chirho(
        left_chirho,
        right_chirho,
        &mut left_context_chirho,
        &mut right_context_chirho,
    );
    limits_chirho.steps_chirho = kernel_chirho.limits_chirho.steps_chirho;
    limits_chirho.nodes_chirho = limits_chirho
        .nodes_chirho
        .saturating_sub(kernel_chirho.nodes_chirho.len());
    match result_chirho {
        Ok(FamilyUnificationChirho::SurelyApartChirho) => {
            FamilyPatternRelationChirho::SurelyApartChirho
        }
        Ok(FamilyUnificationChirho::UnifiableChirho)
            if kernel_chirho.flattened_chirho.is_empty() =>
        {
            FamilyPatternRelationChirho::UnifiableChirho
        }
        Ok(FamilyUnificationChirho::UnifiableChirho) => {
            FamilyPatternRelationChirho::KnownFamilyOverlapChirho
        }
        Ok(FamilyUnificationChirho::UncertainChirho) | Err(()) => {
            FamilyPatternRelationChirho::UnknownChirho
        }
    }
}

pub(super) fn family_branches_compatible_chirho(
    left_lhs_chirho: &[TyChirho],
    left_rhs_chirho: &TyChirho,
    right_lhs_chirho: &[TyChirho],
    right_rhs_chirho: &TyChirho,
    classify_chirho: &impl Fn(&str) -> FamilyHeadRoleChirho,
) -> bool {
    let mut kernel_chirho = ApartnessKernelChirho::new_chirho(
        classify_chirho,
        FamilyUnificationLimitsChirho::default(),
    );
    let mut left_context_chirho = kernel_chirho.context_chirho();
    let mut right_context_chirho = kernel_chirho.context_chirho();
    match kernel_chirho.unify_inputs_chirho(
        left_lhs_chirho,
        right_lhs_chirho,
        &mut left_context_chirho,
        &mut right_context_chirho,
    ) {
        Ok(FamilyUnificationChirho::SurelyApartChirho) => true,
        Ok(FamilyUnificationChirho::UnifiableChirho)
            if kernel_chirho.flattened_chirho.is_empty() =>
        {
            // Each RHS uses its own LHS's namespace, never a separately freshened namespace.
            let compared_chirho = (|| {
                let left_chirho =
                    kernel_chirho.convert_chirho(left_rhs_chirho, &mut left_context_chirho, 0)?;
                let right_chirho =
                    kernel_chirho.convert_chirho(right_rhs_chirho, &mut right_context_chirho, 0)?;
                let left_chirho = kernel_chirho.normalize_chirho(left_chirho, 0)?;
                let right_chirho = kernel_chirho.normalize_chirho(right_chirho, 0)?;
                let equal_chirho = kernel_chirho.equal_rhs_chirho(left_chirho, right_chirho, 0)?;
                Ok::<bool, ()>(equal_chirho && !kernel_chirho.uncertain_chirho)
            })();
            compared_chirho.unwrap_or(false)
        }
        Ok(FamilyUnificationChirho::UnifiableChirho | FamilyUnificationChirho::UncertainChirho)
        | Err(()) => false,
    }
}

type NodeIdChirho = usize;

/// A hash-consed DAG bounds storage when substitutions duplicate their arguments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NodeChirho {
    VariableChirho(usize),
    ConstructorChirho(GenerativeIdentityChirho),
    UnknownChirho {
        namespace_chirho: usize,
        name_chirho: String,
    },
    ApplicationChirho(NodeIdChirho, NodeIdChirho),
    BoundChirho {
        depth_chirho: usize,
        index_chirho: usize,
    },
    ForallChirho {
        required_chirho: bool,
        arity_chirho: usize,
        body_chirho: NodeIdChirho,
    },
    FamilyChirho {
        identity_chirho: String,
        arguments_chirho: Vec<NodeIdChirho>,
        scopes_chirho: Vec<usize>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SourceVariableChirho {
    NumericChirho(TyVarChirho),
    NamedChirho(String),
}

struct ConversionContextChirho {
    namespace_chirho: usize,
    variables_chirho: HashMap<SourceVariableChirho, NodeIdChirho>,
    binders_chirho: Vec<HashMap<TyVarChirho, usize>>,
    scopes_chirho: Vec<usize>,
}

struct ApartnessKernelChirho<'classifier_chirho, ClassifyChirho> {
    classify_chirho: &'classifier_chirho ClassifyChirho,
    limits_chirho: FamilyUnificationLimitsChirho,
    nodes_chirho: Vec<NodeChirho>,
    interned_chirho: HashMap<NodeChirho, NodeIdChirho>,
    substitution_chirho: HashMap<usize, NodeIdChirho>,
    flattened_chirho: Vec<(NodeIdChirho, NodeIdChirho)>,
    pending_chirho: VecDeque<(NodeIdChirho, NodeIdChirho)>,
    applications_chirho: Vec<(NodeIdChirho, NodeIdChirho, NodeIdChirho, NodeIdChirho)>,
    next_variable_chirho: usize,
    next_scope_chirho: usize,
    uncertain_chirho: bool,
}

impl<'classifier_chirho, ClassifyChirho: Fn(&str) -> FamilyHeadRoleChirho>
    ApartnessKernelChirho<'classifier_chirho, ClassifyChirho>
{
    fn new_chirho(
        classify_chirho: &'classifier_chirho ClassifyChirho,
        limits_chirho: FamilyUnificationLimitsChirho,
    ) -> Self {
        Self {
            classify_chirho,
            limits_chirho,
            nodes_chirho: Vec::new(),
            interned_chirho: HashMap::new(),
            substitution_chirho: HashMap::new(),
            flattened_chirho: Vec::new(),
            pending_chirho: VecDeque::new(),
            applications_chirho: Vec::new(),
            next_variable_chirho: 0,
            next_scope_chirho: 0,
            uncertain_chirho: false,
        }
    }

    fn context_chirho(&mut self) -> ConversionContextChirho {
        let namespace_chirho = self.next_scope_chirho;
        self.next_scope_chirho += 1;
        ConversionContextChirho {
            namespace_chirho,
            variables_chirho: HashMap::new(),
            binders_chirho: Vec::new(),
            scopes_chirho: Vec::new(),
        }
    }

    fn tick_chirho(&mut self, depth_chirho: usize) -> Result<(), ()> {
        if depth_chirho > self.limits_chirho.depth_chirho {
            return Err(());
        }
        self.limits_chirho.steps_chirho =
            self.limits_chirho.steps_chirho.checked_sub(1).ok_or(())?;
        Ok(())
    }

    fn intern_chirho(&mut self, node_chirho: NodeChirho) -> Result<NodeIdChirho, ()> {
        self.tick_chirho(0)?;
        let width_chirho = match &node_chirho {
            NodeChirho::ConstructorChirho(GenerativeIdentityChirho::NominalChirho(
                identity_chirho,
            )) => identity_chirho.len(),
            NodeChirho::UnknownChirho { name_chirho, .. } => name_chirho.len(),
            NodeChirho::FamilyChirho {
                identity_chirho,
                arguments_chirho,
                scopes_chirho,
            } => identity_chirho
                .len()
                .saturating_add(arguments_chirho.len())
                .saturating_add(scopes_chirho.len()),
            _ => 0,
        };
        self.limits_chirho.steps_chirho = self
            .limits_chirho
            .steps_chirho
            .checked_sub(width_chirho)
            .ok_or(())?;
        if let Some(id_chirho) = self.interned_chirho.get(&node_chirho) {
            return Ok(*id_chirho);
        }
        if self.nodes_chirho.len() >= self.limits_chirho.nodes_chirho {
            return Err(());
        }
        let id_chirho = self.nodes_chirho.len();
        self.nodes_chirho.push(node_chirho.clone());
        self.interned_chirho.insert(node_chirho, id_chirho);
        Ok(id_chirho)
    }

    fn variable_chirho(&mut self) -> Result<NodeIdChirho, ()> {
        let variable_chirho = self.next_variable_chirho;
        self.next_variable_chirho = variable_chirho.checked_add(1).ok_or(())?;
        self.intern_chirho(NodeChirho::VariableChirho(variable_chirho))
    }

    fn source_variable_chirho(
        &mut self,
        variable_chirho: SourceVariableChirho,
        context_chirho: &mut ConversionContextChirho,
    ) -> Result<NodeIdChirho, ()> {
        if let Some(id_chirho) = context_chirho.variables_chirho.get(&variable_chirho) {
            return Ok(*id_chirho);
        }
        let id_chirho = self.variable_chirho()?;
        context_chirho
            .variables_chirho
            .insert(variable_chirho, id_chirho);
        Ok(id_chirho)
    }

    fn constructor_chirho(
        &mut self,
        name_chirho: &str,
        context_chirho: &ConversionContextChirho,
    ) -> Result<NodeIdChirho, ()> {
        // These syntax constructors are unambiguous. Their tags cannot collide with an
        // opaque nominal identity; the callback can use the same tags for resolved aliases.
        let identity_chirho = match name_chirho {
            "[]" => Some(GenerativeIdentityChirho::ListChirho),
            "()" => Some(GenerativeIdentityChirho::TupleChirho(0)),
            "->" => Some(GenerativeIdentityChirho::ArrowChirho(
                MultChirho::ManyChirho,
            )),
            _ if name_chirho.starts_with('(')
                && name_chirho.ends_with(')')
                && name_chirho[1..name_chirho.len() - 1]
                    .chars()
                    .all(|character_chirho| character_chirho == ',')
                && name_chirho.len() > 2 =>
            {
                Some(GenerativeIdentityChirho::TupleChirho(name_chirho.len() - 1))
            }
            _ => None,
        };
        if let Some(identity_chirho) = identity_chirho {
            return self.intern_chirho(NodeChirho::ConstructorChirho(identity_chirho));
        }
        match (self.classify_chirho)(name_chirho) {
            FamilyHeadRoleChirho::GenerativeChirho { identity_chirho } => {
                self.intern_chirho(NodeChirho::ConstructorChirho(identity_chirho))
            }
            FamilyHeadRoleChirho::FamilyChirho {
                identity_chirho,
                arity_chirho: 0,
            } => self.intern_chirho(NodeChirho::FamilyChirho {
                identity_chirho,
                arguments_chirho: Vec::new(),
                scopes_chirho: context_chirho.scopes_chirho.clone(),
            }),
            FamilyHeadRoleChirho::FamilyChirho { .. } => Err(()),
            FamilyHeadRoleChirho::UnknownChirho => {
                self.uncertain_chirho = true;
                self.intern_chirho(NodeChirho::UnknownChirho {
                    namespace_chirho: context_chirho.namespace_chirho,
                    name_chirho: name_chirho.to_owned(),
                })
            }
        }
    }

    fn convert_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        context_chirho: &mut ConversionContextChirho,
        depth_chirho: usize,
    ) -> Result<NodeIdChirho, ()> {
        self.tick_chirho(depth_chirho)?;
        match ty_chirho {
            TyChirho::QualChirho { .. } => Err(()),
            TyChirho::VarChirho(variable_chirho) => {
                for (bound_depth_chirho, binders_chirho) in
                    context_chirho.binders_chirho.iter().rev().enumerate()
                {
                    self.tick_chirho(depth_chirho)?;
                    if let Some(index_chirho) = binders_chirho.get(variable_chirho) {
                        return self.intern_chirho(NodeChirho::BoundChirho {
                            depth_chirho: bound_depth_chirho,
                            index_chirho: *index_chirho,
                        });
                    }
                }
                self.source_variable_chirho(
                    SourceVariableChirho::NumericChirho(*variable_chirho),
                    context_chirho,
                )
            }
            TyChirho::ForallVarChirho(name_chirho) => self.source_variable_chirho(
                SourceVariableChirho::NamedChirho(name_chirho.clone()),
                context_chirho,
            ),
            TyChirho::ConChirho(name_chirho) => {
                self.constructor_chirho(name_chirho, context_chirho)
            }
            TyChirho::AppChirho(_, _) => {
                let mut head_chirho = ty_chirho;
                let mut arguments_chirho = Vec::new();
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    self.tick_chirho(depth_chirho + arguments_chirho.len())?;
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                if let TyChirho::ConChirho(name_chirho) = head_chirho
                    && let FamilyHeadRoleChirho::FamilyChirho {
                        identity_chirho,
                        arity_chirho,
                    } = (self.classify_chirho)(name_chirho)
                {
                    if arguments_chirho.len() < arity_chirho {
                        return Err(());
                    }
                    let mut converted_chirho = Vec::new();
                    for argument_chirho in &arguments_chirho[..arity_chirho] {
                        converted_chirho.push(self.convert_chirho(
                            argument_chirho,
                            context_chirho,
                            depth_chirho + 1,
                        )?);
                    }
                    let mut result_chirho = self.intern_chirho(NodeChirho::FamilyChirho {
                        identity_chirho,
                        arguments_chirho: converted_chirho,
                        scopes_chirho: context_chirho.scopes_chirho.clone(),
                    })?;
                    for argument_chirho in &arguments_chirho[arity_chirho..] {
                        let argument_chirho =
                            self.convert_chirho(argument_chirho, context_chirho, depth_chirho + 1)?;
                        result_chirho = self.intern_chirho(NodeChirho::ApplicationChirho(
                            result_chirho,
                            argument_chirho,
                        ))?;
                    }
                    return Ok(result_chirho);
                }
                if let TyChirho::ConChirho(name_chirho) = head_chirho
                    && matches!(
                        (self.classify_chirho)(name_chirho),
                        FamilyHeadRoleChirho::UnknownChirho
                    )
                    && !matches!(name_chirho.as_str(), "[]" | "()" | "->")
                    && !(name_chirho.starts_with('(')
                        && name_chirho.ends_with(')')
                        && name_chirho.len() > 2
                        && name_chirho[1..name_chirho.len() - 1]
                            .chars()
                            .all(|character_chirho| character_chirho == ','))
                {
                    // An unknown head might be a non-injective family. Decomposing its
                    // arguments would make U Int versus U Bool a false apartness proof.
                    return Err(());
                }
                let mut result_chirho =
                    self.convert_chirho(head_chirho, context_chirho, depth_chirho + 1)?;
                for argument_chirho in arguments_chirho {
                    let argument_chirho =
                        self.convert_chirho(argument_chirho, context_chirho, depth_chirho + 1)?;
                    result_chirho = self.intern_chirho(NodeChirho::ApplicationChirho(
                        result_chirho,
                        argument_chirho,
                    ))?;
                }
                Ok(result_chirho)
            }
            TyChirho::ListChirho(element_chirho) => {
                let head_chirho = self.intern_chirho(NodeChirho::ConstructorChirho(
                    GenerativeIdentityChirho::ListChirho,
                ))?;
                let element_chirho =
                    self.convert_chirho(element_chirho, context_chirho, depth_chirho + 1)?;
                self.intern_chirho(NodeChirho::ApplicationChirho(head_chirho, element_chirho))
            }
            TyChirho::TupleChirho(elements_chirho) => {
                if elements_chirho.len() == 1 {
                    return Err(());
                }
                let identity_chirho = GenerativeIdentityChirho::TupleChirho(elements_chirho.len());
                let mut result_chirho =
                    self.intern_chirho(NodeChirho::ConstructorChirho(identity_chirho))?;
                for element_chirho in elements_chirho {
                    let element_chirho =
                        self.convert_chirho(element_chirho, context_chirho, depth_chirho + 1)?;
                    result_chirho = self.intern_chirho(NodeChirho::ApplicationChirho(
                        result_chirho,
                        element_chirho,
                    ))?;
                }
                Ok(result_chirho)
            }
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                let head_chirho = self.intern_chirho(NodeChirho::ConstructorChirho(
                    GenerativeIdentityChirho::ArrowChirho(*multiplicity_chirho),
                ))?;
                let argument_chirho =
                    self.convert_chirho(argument_chirho, context_chirho, depth_chirho + 1)?;
                let result_chirho =
                    self.convert_chirho(result_chirho, context_chirho, depth_chirho + 1)?;
                let head_chirho = self
                    .intern_chirho(NodeChirho::ApplicationChirho(head_chirho, argument_chirho))?;
                self.intern_chirho(NodeChirho::ApplicationChirho(head_chirho, result_chirho))
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let scope_chirho = self.next_scope_chirho;
                self.next_scope_chirho = scope_chirho.checked_add(1).ok_or(())?;
                let mut binders_chirho = HashMap::new();
                for (index_chirho, variable_chirho) in vars_chirho.iter().enumerate() {
                    self.tick_chirho(depth_chirho)?;
                    if binders_chirho
                        .insert(*variable_chirho, index_chirho)
                        .is_some()
                    {
                        return Err(());
                    }
                }
                context_chirho.binders_chirho.push(binders_chirho);
                context_chirho.scopes_chirho.push(scope_chirho);
                let body_chirho =
                    self.convert_chirho(body_chirho, context_chirho, depth_chirho + 1);
                context_chirho.scopes_chirho.pop();
                context_chirho.binders_chirho.pop();
                self.intern_chirho(NodeChirho::ForallChirho {
                    required_chirho: matches!(ty_chirho, TyChirho::RequiredForallChirho { .. }),
                    arity_chirho: vars_chirho.len(),
                    body_chirho: body_chirho?,
                })
            }
        }
    }

    fn root_chirho(&mut self, mut node_chirho: NodeIdChirho) -> Result<NodeIdChirho, ()> {
        loop {
            self.tick_chirho(0)?;
            if let NodeChirho::VariableChirho(variable_chirho) = self.nodes_chirho[node_chirho]
                && let Some(replacement_chirho) = self.substitution_chirho.get(&variable_chirho)
            {
                node_chirho = *replacement_chirho;
            } else {
                return Ok(node_chirho);
            }
        }
    }

    fn normalize_chirho(
        &mut self,
        node_chirho: NodeIdChirho,
        depth_chirho: usize,
    ) -> Result<NodeIdChirho, ()> {
        self.tick_chirho(depth_chirho)?;
        let node_chirho = self.root_chirho(node_chirho)?;
        let rebuilt_chirho = match self.nodes_chirho[node_chirho].clone() {
            NodeChirho::ApplicationChirho(function_chirho, argument_chirho) => {
                NodeChirho::ApplicationChirho(
                    self.normalize_chirho(function_chirho, depth_chirho + 1)?,
                    self.normalize_chirho(argument_chirho, depth_chirho + 1)?,
                )
            }
            NodeChirho::FamilyChirho {
                identity_chirho,
                arguments_chirho,
                scopes_chirho,
            } => {
                let mut normalized_chirho = Vec::new();
                for argument_chirho in arguments_chirho {
                    normalized_chirho
                        .push(self.normalize_chirho(argument_chirho, depth_chirho + 1)?);
                }
                NodeChirho::FamilyChirho {
                    identity_chirho,
                    arguments_chirho: normalized_chirho,
                    scopes_chirho,
                }
            }
            NodeChirho::ForallChirho {
                required_chirho,
                arity_chirho,
                body_chirho,
            } => NodeChirho::ForallChirho {
                required_chirho,
                arity_chirho,
                body_chirho: self.normalize_chirho(body_chirho, depth_chirho + 1)?,
            },
            _ => return Ok(node_chirho),
        };
        self.intern_chirho(rebuilt_chirho)
    }

    fn flatten_chirho(&mut self, family_chirho: NodeIdChirho) -> Result<NodeIdChirho, ()> {
        let family_chirho = self.normalize_chirho(family_chirho, 0)?;
        for index_chirho in 0..self.flattened_chirho.len() {
            let (prior_chirho, variable_chirho) = self.flattened_chirho[index_chirho];
            if self.normalize_chirho(prior_chirho, 0)? == family_chirho {
                return self.root_chirho(variable_chirho);
            }
        }
        let variable_chirho = self.variable_chirho()?;
        self.flattened_chirho.push((family_chirho, variable_chirho));
        Ok(variable_chirho)
    }

    /// A substitution can make previously distinct applications identical. Preserve that
    /// congruence even when neither application is visited again by the input worklist.
    fn coalesce_families_chirho(&mut self) -> Result<(), ()> {
        let mut seen_chirho = HashMap::new();
        for index_chirho in 0..self.flattened_chirho.len() {
            let (family_chirho, variable_chirho) = self.flattened_chirho[index_chirho];
            let family_chirho = self.normalize_chirho(family_chirho, 0)?;
            let variable_chirho = self.root_chirho(variable_chirho)?;
            if let Some(prior_chirho) = seen_chirho.insert(family_chirho, variable_chirho)
                && prior_chirho != variable_chirho
            {
                self.pending_chirho
                    .push_back((prior_chirho, variable_chirho));
            }
        }
        Ok(())
    }

    fn occurs_chirho(
        &mut self,
        variable_chirho: usize,
        node_chirho: NodeIdChirho,
        depth_chirho: usize,
    ) -> Result<bool, ()> {
        self.tick_chirho(depth_chirho)?;
        let node_chirho = self.root_chirho(node_chirho)?;
        match self.nodes_chirho[node_chirho].clone() {
            NodeChirho::VariableChirho(other_chirho) => Ok(variable_chirho == other_chirho),
            NodeChirho::ApplicationChirho(function_chirho, argument_chirho) => Ok(self
                .occurs_chirho(variable_chirho, function_chirho, depth_chirho + 1)?
                || self.occurs_chirho(variable_chirho, argument_chirho, depth_chirho + 1)?),
            NodeChirho::FamilyChirho {
                arguments_chirho, ..
            } => {
                for argument_chirho in arguments_chirho {
                    if self.occurs_chirho(variable_chirho, argument_chirho, depth_chirho + 1)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            NodeChirho::ForallChirho { body_chirho, .. } => {
                self.occurs_chirho(variable_chirho, body_chirho, depth_chirho + 1)
            }
            _ => Ok(false),
        }
    }

    fn escapes_chirho(
        &mut self,
        node_chirho: NodeIdChirho,
        nested_chirho: usize,
        depth_chirho: usize,
    ) -> Result<bool, ()> {
        self.tick_chirho(depth_chirho)?;
        let node_chirho = self.root_chirho(node_chirho)?;
        match self.nodes_chirho[node_chirho].clone() {
            NodeChirho::BoundChirho { depth_chirho, .. } => Ok(depth_chirho >= nested_chirho),
            NodeChirho::ApplicationChirho(function_chirho, argument_chirho) => Ok(self
                .escapes_chirho(function_chirho, nested_chirho, depth_chirho + 1)?
                || self.escapes_chirho(argument_chirho, nested_chirho, depth_chirho + 1)?),
            NodeChirho::FamilyChirho {
                arguments_chirho, ..
            } => {
                for argument_chirho in arguments_chirho {
                    if self.escapes_chirho(argument_chirho, nested_chirho, depth_chirho + 1)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            NodeChirho::ForallChirho { body_chirho, .. } => {
                self.escapes_chirho(body_chirho, nested_chirho + 1, depth_chirho + 1)
            }
            _ => Ok(false),
        }
    }

    fn bind_chirho(
        &mut self,
        variable_chirho: usize,
        target_chirho: NodeIdChirho,
    ) -> Result<(), ()> {
        if self.occurs_chirho(variable_chirho, target_chirho, 0)?
            || self.escapes_chirho(target_chirho, 0, 0)?
        {
            // Keep other constraints and substitutions: a later nominal clash can still
            // establish apartness. Never insert a cycle or let a bound variable escape.
            self.uncertain_chirho = true;
            return Ok(());
        }
        self.substitution_chirho
            .insert(variable_chirho, target_chirho);
        self.coalesce_families_chirho()
    }

    fn generative_head_chirho(&mut self, mut node_chirho: NodeIdChirho) -> Result<bool, ()> {
        loop {
            self.tick_chirho(0)?;
            node_chirho = self.root_chirho(node_chirho)?;
            match self.nodes_chirho[node_chirho] {
                NodeChirho::ApplicationChirho(function_chirho, _) => node_chirho = function_chirho,
                NodeChirho::ConstructorChirho(_) => return Ok(true),
                _ => return Ok(false),
            }
        }
    }

    fn unify_inputs_chirho(
        &mut self,
        left_chirho: &[TyChirho],
        right_chirho: &[TyChirho],
        left_context_chirho: &mut ConversionContextChirho,
        right_context_chirho: &mut ConversionContextChirho,
    ) -> Result<FamilyUnificationChirho, ()> {
        // Arity belongs to the registry. Unequal vectors are unsupported here, not a
        // mathematical apartness proof that could forgive an invalid equation.
        if left_chirho.len() != right_chirho.len() {
            return Ok(FamilyUnificationChirho::UncertainChirho);
        }
        for (left_chirho, right_chirho) in left_chirho.iter().zip(right_chirho) {
            let left_chirho = self.convert_chirho(left_chirho, left_context_chirho, 0)?;
            let right_chirho = self.convert_chirho(right_chirho, right_context_chirho, 0)?;
            self.pending_chirho.push_back((left_chirho, right_chirho));
        }
        loop {
            while let Some((left_chirho, right_chirho)) = self.pending_chirho.pop_front() {
                self.tick_chirho(0)?;
                let mut left_chirho = self.normalize_chirho(left_chirho, 0)?;
                let mut right_chirho = self.normalize_chirho(right_chirho, 0)?;
                if left_chirho == right_chirho {
                    continue;
                }
                if matches!(
                    self.nodes_chirho[left_chirho],
                    NodeChirho::FamilyChirho { .. }
                ) {
                    left_chirho = self.flatten_chirho(left_chirho)?;
                }
                if matches!(
                    self.nodes_chirho[right_chirho],
                    NodeChirho::FamilyChirho { .. }
                ) {
                    right_chirho = self.flatten_chirho(right_chirho)?;
                }
                if left_chirho == right_chirho {
                    continue;
                }
                match (
                    self.nodes_chirho[left_chirho].clone(),
                    self.nodes_chirho[right_chirho].clone(),
                ) {
                    (NodeChirho::VariableChirho(variable_chirho), _) => {
                        self.bind_chirho(variable_chirho, right_chirho)?
                    }
                    (_, NodeChirho::VariableChirho(variable_chirho)) => {
                        self.bind_chirho(variable_chirho, left_chirho)?
                    }
                    (NodeChirho::ConstructorChirho(_), NodeChirho::ConstructorChirho(_)) => {
                        return Ok(FamilyUnificationChirho::SurelyApartChirho);
                    }
                    (
                        NodeChirho::ApplicationChirho(left_function_chirho, left_argument_chirho),
                        NodeChirho::ApplicationChirho(right_function_chirho, right_argument_chirho),
                    ) => {
                        self.pending_chirho
                            .push_back((left_function_chirho, right_function_chirho));
                        // A variable head can later become Unknown. Compare heads first and
                        // retain the parents: an argument clash is evidence only after both
                        // normalized application heads are positively generative.
                        self.applications_chirho.push((
                            left_chirho,
                            right_chirho,
                            left_argument_chirho,
                            right_argument_chirho,
                        ));
                    }
                    (
                        NodeChirho::ForallChirho {
                            required_chirho: left_required_chirho,
                            arity_chirho: left_arity_chirho,
                            body_chirho: left_body_chirho,
                        },
                        NodeChirho::ForallChirho {
                            required_chirho: right_required_chirho,
                            arity_chirho: right_arity_chirho,
                            body_chirho: right_body_chirho,
                        },
                    ) if left_required_chirho == right_required_chirho
                        && left_arity_chirho == right_arity_chirho =>
                    {
                        self.pending_chirho
                            .push_back((left_body_chirho, right_body_chirho))
                    }
                    (NodeChirho::ApplicationChirho(_, _), NodeChirho::ConstructorChirho(_))
                    | (NodeChirho::ConstructorChirho(_), NodeChirho::ApplicationChirho(_, _)) => {
                        if self.generative_head_chirho(left_chirho)?
                            && self.generative_head_chirho(right_chirho)?
                        {
                            return Ok(FamilyUnificationChirho::SurelyApartChirho);
                        }
                        self.uncertain_chirho = true;
                    }
                    _ => self.uncertain_chirho = true,
                }
            }
            let applications_chirho = std::mem::take(&mut self.applications_chirho);
            for (left_chirho, right_chirho, left_argument_chirho, right_argument_chirho) in
                applications_chirho
            {
                self.tick_chirho(0)?;
                if self.generative_head_chirho(left_chirho)?
                    && self.generative_head_chirho(right_chirho)?
                {
                    self.pending_chirho
                        .push_back((left_argument_chirho, right_argument_chirho));
                } else {
                    self.applications_chirho.push((
                        left_chirho,
                        right_chirho,
                        left_argument_chirho,
                        right_argument_chirho,
                    ));
                }
            }
            if self.pending_chirho.is_empty() {
                self.uncertain_chirho |= !self.applications_chirho.is_empty();
                break;
            }
        }
        Ok(if self.uncertain_chirho {
            FamilyUnificationChirho::UncertainChirho
        } else {
            FamilyUnificationChirho::UnifiableChirho
        })
    }

    fn equal_rhs_chirho(
        &mut self,
        left_chirho: NodeIdChirho,
        right_chirho: NodeIdChirho,
        depth_chirho: usize,
    ) -> Result<bool, ()> {
        self.tick_chirho(depth_chirho)?;
        if left_chirho == right_chirho {
            return Ok(true);
        }
        match (
            self.nodes_chirho[left_chirho].clone(),
            self.nodes_chirho[right_chirho].clone(),
        ) {
            (
                NodeChirho::ApplicationChirho(left_function_chirho, left_argument_chirho),
                NodeChirho::ApplicationChirho(right_function_chirho, right_argument_chirho),
            ) => Ok(self.equal_rhs_chirho(
                left_function_chirho,
                right_function_chirho,
                depth_chirho + 1,
            )? && self.equal_rhs_chirho(
                left_argument_chirho,
                right_argument_chirho,
                depth_chirho + 1,
            )?),
            (
                NodeChirho::ForallChirho {
                    required_chirho: left_required_chirho,
                    arity_chirho: left_arity_chirho,
                    body_chirho: left_body_chirho,
                },
                NodeChirho::ForallChirho {
                    required_chirho: right_required_chirho,
                    arity_chirho: right_arity_chirho,
                    body_chirho: right_body_chirho,
                },
            ) => Ok(left_required_chirho == right_required_chirho
                && left_arity_chirho == right_arity_chirho
                && self.equal_rhs_chirho(left_body_chirho, right_body_chirho, depth_chirho + 1)?),
            (
                NodeChirho::FamilyChirho {
                    identity_chirho: left_identity_chirho,
                    arguments_chirho: left_arguments_chirho,
                    ..
                },
                NodeChirho::FamilyChirho {
                    identity_chirho: right_identity_chirho,
                    arguments_chirho: right_arguments_chirho,
                    ..
                },
            ) => {
                if left_identity_chirho != right_identity_chirho
                    || left_arguments_chirho.len() != right_arguments_chirho.len()
                {
                    return Ok(false);
                }
                for (left_chirho, right_chirho) in left_arguments_chirho
                    .into_iter()
                    .zip(right_arguments_chirho)
                {
                    if !self.equal_rhs_chirho(left_chirho, right_chirho, depth_chirho + 1)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
