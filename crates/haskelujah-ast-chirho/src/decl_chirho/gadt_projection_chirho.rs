// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Preflight canonical GADT expansion without cloning syntax or expanding groups.
//! Keep one budget for a complete module's constructor stream. Work bounds the
//! borrowed scan; output bounds copied syntax nodes plus string bytes. A target
//! representation factor also bounds its deeper/wider encoding (AST: 1, TH: 3).
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use std::fmt;

use super::{
    AssocTfInstanceChirho, AssocTypeFamilyChirho, ClassMethodChirho, ConDeclChirho,
    DataKindSigChirho, DeclChirho, FieldDeclChirho, GadtConArgsChirho, GadtConPrefixChirho,
    GadtConSignatureChirho, TyVarChirho, TypeFamilyEquationChirho,
};
use crate::module_chirho::ModuleChirho;
use crate::name_chirho::{
    NameChirho, RawNameChirho, TypeConstructorDeclarationChirho, TypeConstructorIdentityChirho,
};
use crate::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
use crate::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GadtProjectionLimitsChirho {
    pub work_chirho: usize,
    pub output_chirho: usize,
    pub depth_chirho: usize,
}

impl Default for GadtProjectionLimitsChirho {
    fn default() -> Self {
        Self {
            work_chirho: 1 << 25,
            output_chirho: 1 << 20,
            depth_chirho: 512,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GadtProjectionLimitChirho {
    WorkChirho,
    OutputChirho,
    DepthChirho,
    ArithmeticChirho,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GadtProjectionErrorChirho {
    pub span_chirho: SpanChirho,
    pub limit_chirho: GadtProjectionLimitChirho,
}

impl fmt::Display for GadtProjectionErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        let limit_chirho = match self.limit_chirho {
            GadtProjectionLimitChirho::WorkChirho => "input-work",
            GadtProjectionLimitChirho::OutputChirho => "expanded output-size",
            GadtProjectionLimitChirho::DepthChirho => "expanded depth",
            GadtProjectionLimitChirho::ArithmeticChirho => "checked size-arithmetic",
        };
        write!(
            formatter_chirho,
            "GADT constructor projection exceeded its {limit_chirho} bound"
        )
    }
}

impl std::error::Error for GadtProjectionErrorChirho {}

#[derive(Debug)]
pub struct GadtProjectionBudgetChirho {
    work_chirho: usize,
    output_chirho: usize,
    depth_chirho: usize,
    exhausted_chirho: Option<GadtProjectionErrorChirho>,
}

impl Default for GadtProjectionBudgetChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl GadtProjectionBudgetChirho {
    /// A caller may lend its remaining allowance to this common borrowed walk
    /// and charge the exact debit before cloning the checked syntax.
    pub fn remaining_limits_chirho(&self) -> GadtProjectionLimitsChirho {
        GadtProjectionLimitsChirho {
            work_chirho: self.work_chirho,
            output_chirho: self.output_chirho,
            depth_chirho: self.depth_chirho,
        }
    }

    /// Share the same latched allowance with transport-generated names and
    /// typed terms, before allocating their new output.
    pub fn reserve_transport_chirho(
        &mut self,
        weight_chirho: usize,
        depth_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Result<(), GadtProjectionErrorChirho> {
        checked_chirho(span_chirho, self, |budget_chirho| {
            budget_chirho.depth_chirho(depth_chirho, 1, 1)?;
            budget_chirho.charge_chirho(weight_chirho, 1, 1)
        })
    }

    pub fn new_chirho() -> Self {
        Self::with_limits_chirho(GadtProjectionLimitsChirho::default())
    }

    pub fn with_limits_chirho(limits_chirho: GadtProjectionLimitsChirho) -> Self {
        Self {
            work_chirho: limits_chirho.work_chirho,
            output_chirho: limits_chirho.output_chirho,
            depth_chirho: limits_chirho.depth_chirho,
            exhausted_chirho: None,
        }
    }

    fn work_chirho(&mut self, amount_chirho: usize) -> Result<(), GadtProjectionLimitChirho> {
        self.work_chirho = self
            .work_chirho
            .checked_sub(amount_chirho)
            .ok_or(GadtProjectionLimitChirho::WorkChirho)?;
        Ok(())
    }

    fn charge_chirho(
        &mut self,
        weight_chirho: usize,
        copies_chirho: usize,
        factor_chirho: usize,
    ) -> Result<(), GadtProjectionLimitChirho> {
        let output_chirho = weight_chirho
            .checked_mul(copies_chirho)
            .and_then(|amount_chirho| amount_chirho.checked_mul(factor_chirho))
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        self.work_chirho(weight_chirho)?;
        self.output_chirho = self
            .output_chirho
            .checked_sub(output_chirho)
            .ok_or(GadtProjectionLimitChirho::OutputChirho)?;
        Ok(())
    }

    fn depth_chirho(
        &self,
        depth_chirho: usize,
        copies_chirho: usize,
        factor_chirho: usize,
    ) -> Result<(), GadtProjectionLimitChirho> {
        let depth_chirho = depth_chirho
            .checked_mul(if copies_chirho == 0 { 1 } else { factor_chirho })
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        if depth_chirho > self.depth_chirho {
            Err(GadtProjectionLimitChirho::DepthChirho)
        } else {
            Ok(())
        }
    }
}

/// Preflight a retained type before transport clones it. All kind arguments,
/// quantifier annotations and constraint contexts use the common borrowed walk.
pub fn check_type_projection_chirho(
    ty_chirho: &TypeChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
) -> Result<(), GadtProjectionErrorChirho> {
    checked_chirho(ty_chirho.span_chirho(), budget_chirho, |budget_chirho| {
        walk_chirho(
            ProjectionNodeChirho::TypeChirho(ty_chirho),
            1,
            1,
            1,
            budget_chirho,
        )
    })
}

pub fn check_name_projection_chirho(
    name_chirho: &NameChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
) -> Result<(), GadtProjectionErrorChirho> {
    checked_chirho(name_chirho.span_chirho(), budget_chirho, |budget_chirho| {
        walk_chirho(
            ProjectionNodeChirho::NameChirho(name_chirho),
            1,
            1,
            1,
            budget_chirho,
        )
    })
}

/// Check every source component and the entire expanded signature before any
/// consumer clones it. No partial type, success permit or fallback is produced
/// on exhaustion. A failed stream stays failed until its owner creates a new
/// budget. The representation factor must be positive.
pub fn check_projection_chirho(
    signature_chirho: &GadtConSignatureChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    representation_factor_chirho: usize,
) -> Result<(), GadtProjectionErrorChirho> {
    checked_chirho(
        signature_chirho.span_chirho,
        budget_chirho,
        |budget_chirho| {
            check_signature_chirho(
                signature_chirho,
                budget_chirho,
                representation_factor_chirho,
            )
        },
    )
}

/// Check a constructor without fabricating its parent declaration's result.
/// Header-aware consumers must use the declaration or module gateway below.
pub fn check_constructor_projection_chirho(
    constructor_chirho: &ConDeclChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    factor_chirho: usize,
) -> Result<(), GadtProjectionErrorChirho> {
    let (name_chirho, span_chirho) = match constructor_chirho {
        ConDeclChirho::OrdinaryChirho {
            name_chirho,
            span_chirho,
            ..
        }
        | ConDeclChirho::RecordChirho {
            name_chirho,
            span_chirho,
            ..
        }
        | ConDeclChirho::GadtChirho {
            name_chirho,
            span_chirho,
            ..
        } => (name_chirho, *span_chirho),
    };
    checked_chirho(span_chirho, budget_chirho, |budget_chirho| {
        if factor_chirho == 0 {
            return Err(GadtProjectionLimitChirho::ArithmeticChirho);
        }
        walk_chirho(
            ProjectionNodeChirho::NameChirho(name_chirho),
            1,
            1,
            factor_chirho,
            budget_chirho,
        )?;
        if let ConDeclChirho::OrdinaryChirho { prefix_chirho, .. }
        | ConDeclChirho::RecordChirho { prefix_chirho, .. } = constructor_chirho
        {
            budget_chirho.charge_chirho(1, 1, factor_chirho)?;
            walk_chirho(
                ProjectionNodeChirho::BindersChirho(&prefix_chirho.existential_vars_chirho),
                1,
                1,
                factor_chirho,
                budget_chirho,
            )?;
            walk_chirho(
                ProjectionNodeChirho::ConstraintsChirho(&prefix_chirho.context_chirho),
                1,
                1,
                factor_chirho,
                budget_chirho,
            )?;
        }
        match constructor_chirho {
            ConDeclChirho::GadtChirho {
                signature_chirho, ..
            } => check_signature_chirho(signature_chirho, budget_chirho, factor_chirho),
            ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                budget_chirho.depth_chirho(
                    next_depth_chirho(fields_chirho.len())?,
                    1,
                    factor_chirho,
                )?;
                for (index_chirho, (_, ty_chirho)) in fields_chirho.iter().enumerate() {
                    check_field_chirho(
                        ty_chirho,
                        None,
                        1,
                        next_depth_chirho(next_depth_chirho(index_chirho)?)?,
                        factor_chirho,
                        budget_chirho,
                    )?;
                }
                Ok(())
            }
            ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                let mut through_chirho = 0_usize;
                for field_chirho in fields_chirho {
                    let copies_chirho = field_chirho.names_chirho.len();
                    through_chirho = through_chirho
                        .checked_add(copies_chirho)
                        .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
                    budget_chirho.depth_chirho(
                        next_depth_chirho(through_chirho)?,
                        1,
                        factor_chirho,
                    )?;
                    walk_chirho(
                        ProjectionNodeChirho::NamesChirho(&field_chirho.names_chirho),
                        1,
                        1,
                        factor_chirho,
                        budget_chirho,
                    )?;
                    check_field_chirho(
                        &field_chirho.ty_chirho,
                        field_chirho.multiplicity_chirho.as_ref(),
                        copies_chirho,
                        if copies_chirho == 0 {
                            1
                        } else {
                            next_depth_chirho(through_chirho)?
                        },
                        factor_chirho,
                        budget_chirho,
                    )?;
                }
                Ok(())
            }
        }
    })
}

fn checked_chirho(
    span_chirho: SpanChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    operation_chirho: impl FnOnce(
        &mut GadtProjectionBudgetChirho,
    ) -> Result<(), GadtProjectionLimitChirho>,
) -> Result<(), GadtProjectionErrorChirho> {
    if let Some(error_chirho) = budget_chirho.exhausted_chirho {
        return Err(error_chirho);
    }
    let result_chirho =
        operation_chirho(budget_chirho).map_err(|limit_chirho| GadtProjectionErrorChirho {
            span_chirho,
            limit_chirho,
        });
    if let Err(error_chirho) = result_chirho {
        budget_chirho.exhausted_chirho = Some(error_chirho);
    }
    result_chirho
}

/// Data/newtype headers supply the return type absent from ordinary and record
/// constructors. Account for that copied header for every such constructor.
/// Other type-bearing declaration headers use the same borrowed preflight;
/// value-binding expression bodies remain outside this type projection policy.
pub fn check_declaration_projection_chirho(
    declaration_chirho: &DeclChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    factor_chirho: usize,
) -> Result<(), GadtProjectionErrorChirho> {
    let (name_chirho, binders_chirho, kind_chirho, deriving_chirho, constructors_chirho) =
        match declaration_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                deriving_chirho,
                constructors_chirho,
                ..
            } => (
                name_chirho,
                type_vars_chirho.as_slice(),
                kind_sig_chirho.as_ref(),
                deriving_chirho.as_slice(),
                constructors_chirho.as_slice(),
            ),
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                deriving_chirho,
                constructor_chirho,
                ..
            } => (
                name_chirho,
                type_vars_chirho.as_slice(),
                kind_sig_chirho.as_ref(),
                deriving_chirho.as_slice(),
                std::slice::from_ref(constructor_chirho),
            ),
            _ => {
                return checked_chirho(
                    declaration_chirho.span_chirho(),
                    budget_chirho,
                    |budget_chirho| {
                        if factor_chirho == 0 {
                            return Err(GadtProjectionLimitChirho::ArithmeticChirho);
                        }
                        match declaration_chirho {
                            DeclChirho::TypeSigChirho { .. }
                            | DeclChirho::ForeignDeclChirho { .. }
                            | DeclChirho::DefaultDeclChirho { .. }
                            | DeclChirho::TypeAliasDeclChirho { .. }
                            | DeclChirho::TypeFamilyDeclChirho { .. }
                            | DeclChirho::TypeFamilyInstanceDeclChirho { .. }
                            | DeclChirho::ClassDeclChirho { .. }
                            | DeclChirho::InstanceDeclChirho { .. }
                            | DeclChirho::StandaloneDerivingDeclChirho { .. } => walk_chirho(
                                ProjectionNodeChirho::DeclarationChirho(declaration_chirho),
                                1,
                                1,
                                factor_chirho,
                                budget_chirho,
                            ),
                            _ => budget_chirho.work_chirho(1),
                        }
                    },
                );
            }
        };
    checked_chirho(
        declaration_chirho.span_chirho(),
        budget_chirho,
        |budget_chirho| {
            if factor_chirho == 0 {
                return Err(GadtProjectionLimitChirho::ArithmeticChirho);
            }
            let mut ordinary_count_chirho = 0_usize;
            let mut max_arity_chirho = 0_usize;
            for constructor_chirho in constructors_chirho {
                budget_chirho.work_chirho(1)?;
                let arity_chirho = match constructor_chirho {
                    ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho.len(),
                    ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                        let mut arity_chirho = 0_usize;
                        for field_chirho in fields_chirho {
                            budget_chirho.work_chirho(1)?;
                            arity_chirho = arity_chirho
                                .checked_add(field_chirho.names_chirho.len())
                                .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
                        }
                        arity_chirho
                    }
                    ConDeclChirho::GadtChirho { .. } => continue,
                };
                ordinary_count_chirho = next_depth_chirho(ordinary_count_chirho)?;
                max_arity_chirho = max_arity_chirho.max(arity_chirho);
            }
            let mut visible_chirho = 0_usize;
            for binder_chirho in binders_chirho {
                budget_chirho.work_chirho(1)?;
                if binder_chirho.is_visible_chirho() {
                    visible_chirho = next_depth_chirho(visible_chirho)?;
                }
            }
            let copies_chirho = next_depth_chirho(ordinary_count_chirho)?; // declaration plus ordinary constructor results
            let result_depth_chirho = if ordinary_count_chirho == 0 {
                1
            } else {
                max_arity_chirho
                    .checked_add(visible_chirho)
                    .and_then(|depth_chirho| depth_chirho.checked_add(2))
                    .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?
            };
            budget_chirho.depth_chirho(result_depth_chirho, 1, factor_chirho)?;
            let result_nodes_chirho = visible_chirho
                .checked_mul(2)
                .and_then(|nodes_chirho| nodes_chirho.checked_add(1))
                .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
            budget_chirho.charge_chirho(
                result_nodes_chirho,
                ordinary_count_chirho,
                factor_chirho,
            )?;
            walk_chirho(
                ProjectionNodeChirho::NameChirho(name_chirho),
                result_depth_chirho,
                copies_chirho,
                factor_chirho,
                budget_chirho,
            )?;
            walk_chirho(
                ProjectionNodeChirho::BindersChirho(binders_chirho),
                result_depth_chirho,
                copies_chirho,
                factor_chirho,
                budget_chirho,
            )?;
            walk_chirho(
                ProjectionNodeChirho::NamesChirho(deriving_chirho),
                1,
                1,
                factor_chirho,
                budget_chirho,
            )?;
            if let Some(kind_chirho) = kind_chirho {
                match kind_chirho {
                    DataKindSigChirho::ResultChirho(result_chirho) => walk_chirho(
                        ProjectionNodeChirho::TypeChirho(result_chirho),
                        result_depth_chirho,
                        copies_chirho,
                        factor_chirho,
                        budget_chirho,
                    )?,
                    DataKindSigChirho::StandaloneChirho {
                        signature_chirho,
                        result_chirho,
                    } => {
                        walk_chirho(
                            ProjectionNodeChirho::TypeChirho(signature_chirho),
                            result_depth_chirho,
                            copies_chirho,
                            factor_chirho,
                            budget_chirho,
                        )?;
                        if let Some(result_chirho) = result_chirho {
                            walk_chirho(
                                ProjectionNodeChirho::TypeChirho(result_chirho),
                                result_depth_chirho,
                                copies_chirho,
                                factor_chirho,
                                budget_chirho,
                            )?;
                        }
                    }
                }
            }
            Ok(())
        },
    )?;
    for constructor_chirho in constructors_chirho {
        check_constructor_projection_chirho(constructor_chirho, budget_chirho, factor_chirho)?;
    }
    Ok(())
}

/// A shared module stream, including its copied module name, has one allowance.
pub fn check_module_projection_chirho(
    module_chirho: &ModuleChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    factor_chirho: usize,
) -> Result<(), GadtProjectionErrorChirho> {
    checked_chirho(module_chirho.span_chirho, budget_chirho, |budget_chirho| {
        if factor_chirho == 0 {
            return Err(GadtProjectionLimitChirho::ArithmeticChirho);
        }
        walk_chirho(
            ProjectionNodeChirho::NameChirho(&module_chirho.name_chirho),
            1,
            1,
            factor_chirho,
            budget_chirho,
        )
    })?;
    for declaration_chirho in &module_chirho.decls_chirho {
        check_declaration_projection_chirho(declaration_chirho, budget_chirho, factor_chirho)?;
    }
    for (type_name_chirho, class_name_chirho, via_chirho) in &module_chirho.deriving_via_chirho {
        checked_chirho(via_chirho.span_chirho(), budget_chirho, |budget_chirho| {
            walk_chirho(
                ProjectionNodeChirho::NameChirho(type_name_chirho),
                1,
                1,
                factor_chirho,
                budget_chirho,
            )?;
            walk_chirho(
                ProjectionNodeChirho::NameChirho(class_name_chirho),
                1,
                1,
                factor_chirho,
                budget_chirho,
            )?;
            walk_chirho(
                ProjectionNodeChirho::TypeChirho(via_chirho),
                1,
                1,
                factor_chirho,
                budget_chirho,
            )
        })?;
    }
    Ok(())
}

/// Precharge record getter/setter generation, which repeats N field binders
/// for every one of N labels. This bounds implementation work/output, not a
/// Haskell arity rule. Core callers should supply their module's larger limits
/// (for example work 2^27, output 2^24); the AST/TH defaults need not change.
/// Generated Case binders and ConApp arguments are flat, so their depth is a
/// fixed bound rather than N times the syntax projection's depth.
pub fn check_record_accessors_projection_chirho(
    constructor_name_chirho: &NameChirho,
    fields_chirho: &[FieldDeclChirho],
    span_chirho: SpanChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
) -> Result<(), GadtProjectionErrorChirho> {
    checked_chirho(span_chirho, budget_chirho, |budget_chirho| {
        let mut count_chirho = 0_usize;
        let mut field_names_chirho = 0_usize;
        for field_chirho in fields_chirho {
            budget_chirho.work_chirho(1)?;
            for name_chirho in &field_chirho.names_chirho {
                let weight_chirho = name_weight_chirho(name_chirho)?;
                budget_chirho.work_chirho(weight_chirho)?;
                count_chirho = next_depth_chirho(count_chirho)?;
                field_names_chirho = field_names_chirho
                    .checked_add(weight_chirho)
                    .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
            }
        }
        let constructor_weight_chirho = name_weight_chirho(constructor_name_chirho)?;
        budget_chirho.work_chirho(constructor_weight_chirho)?;
        let mut digits_chirho = 1_usize;
        let mut magnitude_chirho = count_chirho;
        while magnitude_chirho >= 10 {
            budget_chirho.work_chirho(1)?;
            magnitude_chirho /= 10;
            digits_chirho = next_depth_chirho(digits_chirho)?;
        }
        // Per label pair: two sets of binders/type variables, generated names
        // (including decimal indices), and setter arguments. The linear term
        // covers prebindings, lambdas, cases and fixed generated-name bytes.
        let pair_weight_chirho = digits_chirho
            .checked_mul(8)
            .and_then(|weight_chirho| weight_chirho.checked_add(64))
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        let pairs_chirho = count_chirho
            .checked_mul(count_chirho)
            .and_then(|count_chirho| count_chirho.checked_mul(pair_weight_chirho))
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        let linear_chirho = count_chirho
            .checked_mul(128)
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        let fields_weight_chirho = field_names_chirho
            .checked_mul(16)
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        // Getter alternatives and setter alternatives/reconstruction each own
        // a constructor spelling; flattening retains one additional copy.
        let constructors_weight_chirho = count_chirho
            .checked_mul(3)
            .and_then(|count_chirho| count_chirho.checked_add(1))
            .and_then(|count_chirho| count_chirho.checked_mul(constructor_weight_chirho))
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        let total_chirho = pairs_chirho
            .checked_add(linear_chirho)
            .and_then(|weight_chirho| weight_chirho.checked_add(fields_weight_chirho))
            .and_then(|weight_chirho| weight_chirho.checked_add(constructors_weight_chirho))
            .and_then(|weight_chirho| weight_chirho.checked_add(fields_chirho.len()))
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
        budget_chirho.depth_chirho(if count_chirho == 0 { 1 } else { 8 }, 1, 1)?;
        budget_chirho.charge_chirho(total_chirho, 1, 1)
    })
}

fn next_depth_chirho(depth_chirho: usize) -> Result<usize, GadtProjectionLimitChirho> {
    depth_chirho
        .checked_add(1)
        .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)
}

fn check_signature_chirho(
    signature_chirho: &GadtConSignatureChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    factor_chirho: usize,
) -> Result<(), GadtProjectionLimitChirho> {
    if factor_chirho == 0 {
        return Err(GadtProjectionLimitChirho::ArithmeticChirho);
    }
    budget_chirho.work_chirho(1)?;
    let prefix_count_chirho = signature_chirho.prefix_chirho.len();
    let mut argument_count_chirho = 0_usize;
    match &signature_chirho.args_chirho {
        GadtConArgsChirho::PositionalChirho(fields_chirho) => {
            argument_count_chirho = fields_chirho.len()
        }
        GadtConArgsChirho::RecordChirho(fields_chirho) => {
            for field_chirho in fields_chirho {
                budget_chirho.work_chirho(1)?;
                argument_count_chirho = argument_count_chirho
                    .checked_add(field_chirho.names_chirho.len())
                    .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
            }
        }
    }
    let result_depth_chirho = prefix_count_chirho
        .checked_add(argument_count_chirho)
        .and_then(|depth_chirho| depth_chirho.checked_add(1))
        .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
    budget_chirho.depth_chirho(result_depth_chirho, 1, factor_chirho)?;
    // Result and prefixes are charged even when there are no arguments.
    walk_chirho(
        ProjectionNodeChirho::TypeChirho(&signature_chirho.result_ty_chirho),
        result_depth_chirho,
        1,
        factor_chirho,
        budget_chirho,
    )?;
    for (index_chirho, prefix_chirho) in signature_chirho.prefix_chirho.iter().enumerate() {
        let depth_chirho = next_depth_chirho(index_chirho)?;
        budget_chirho.depth_chirho(depth_chirho, 1, factor_chirho)?;
        budget_chirho.charge_chirho(1, 1, factor_chirho)?;
        let node_chirho = match prefix_chirho {
            GadtConPrefixChirho::ForallChirho { vars_chirho, .. } => {
                ProjectionNodeChirho::BindersChirho(vars_chirho)
            }
            GadtConPrefixChirho::ContextChirho {
                constraints_chirho, ..
            } => ProjectionNodeChirho::ConstraintsChirho(constraints_chirho),
        };
        walk_chirho(
            node_chirho,
            next_depth_chirho(depth_chirho)?,
            1,
            factor_chirho,
            budget_chirho,
        )?;
    }
    let mut through_chirho = 0_usize;
    match &signature_chirho.args_chirho {
        GadtConArgsChirho::PositionalChirho(fields_chirho) => {
            for field_chirho in fields_chirho {
                through_chirho = next_depth_chirho(through_chirho)?;
                let depth_chirho = prefix_count_chirho
                    .checked_add(through_chirho)
                    .and_then(|depth_chirho| depth_chirho.checked_add(1))
                    .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
                check_field_chirho(
                    &field_chirho.ty_chirho,
                    field_chirho.multiplicity_chirho.as_ref(),
                    1,
                    depth_chirho,
                    factor_chirho,
                    budget_chirho,
                )?;
            }
        }
        GadtConArgsChirho::RecordChirho(fields_chirho) => {
            for field_chirho in fields_chirho {
                let copies_chirho = field_chirho.names_chirho.len();
                through_chirho = through_chirho
                    .checked_add(copies_chirho)
                    .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?;
                let depth_chirho = if copies_chirho == 0 {
                    1
                } else {
                    prefix_count_chirho
                        .checked_add(through_chirho)
                        .and_then(|depth_chirho| depth_chirho.checked_add(1))
                        .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?
                };
                // Labels are scanned once as input and charged as copied
                // metadata, even for a target that omits their source spelling.
                walk_chirho(
                    ProjectionNodeChirho::NamesChirho(&field_chirho.names_chirho),
                    1,
                    1,
                    factor_chirho,
                    budget_chirho,
                )?;
                check_field_chirho(
                    &field_chirho.ty_chirho,
                    field_chirho.multiplicity_chirho.as_ref(),
                    copies_chirho,
                    depth_chirho,
                    factor_chirho,
                    budget_chirho,
                )?;
            }
        }
    }
    Ok(())
}

fn check_field_chirho(
    ty_chirho: &TypeChirho,
    multiplicity_chirho: Option<&MultiplicityChirho>,
    copies_chirho: usize,
    depth_chirho: usize,
    factor_chirho: usize,
    budget_chirho: &mut GadtProjectionBudgetChirho,
) -> Result<(), GadtProjectionLimitChirho> {
    budget_chirho.charge_chirho(1, copies_chirho, factor_chirho)?; // arrow nodes
    walk_chirho(
        ProjectionNodeChirho::TypeChirho(ty_chirho),
        depth_chirho,
        copies_chirho,
        factor_chirho,
        budget_chirho,
    )?;
    if let Some(multiplicity_chirho) = multiplicity_chirho {
        walk_chirho(
            ProjectionNodeChirho::MultiplicityChirho(multiplicity_chirho),
            depth_chirho,
            copies_chirho,
            factor_chirho,
            budget_chirho,
        )?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum ProjectionNodeChirho<'syntax_chirho> {
    TypeChirho(&'syntax_chirho TypeChirho),
    TypesChirho(&'syntax_chirho [TypeChirho]),
    ConstraintChirho(&'syntax_chirho ConstraintChirho),
    ConstraintsChirho(&'syntax_chirho [ConstraintChirho]),
    BinderChirho(&'syntax_chirho TyVarChirho),
    BindersChirho(&'syntax_chirho [TyVarChirho]),
    DeclarationChirho(&'syntax_chirho DeclChirho),
    FamilyEquationChirho(&'syntax_chirho TypeFamilyEquationChirho),
    FamilyEquationsChirho(&'syntax_chirho [TypeFamilyEquationChirho]),
    AssociatedFamilyChirho(&'syntax_chirho AssocTypeFamilyChirho),
    AssociatedFamiliesChirho(&'syntax_chirho [AssocTypeFamilyChirho]),
    AssociatedInstanceChirho(&'syntax_chirho AssocTfInstanceChirho),
    AssociatedInstancesChirho(&'syntax_chirho [AssocTfInstanceChirho]),
    MethodChirho(&'syntax_chirho ClassMethodChirho),
    MethodsChirho(&'syntax_chirho [ClassMethodChirho]),
    StringsChirho(&'syntax_chirho [String]),
    StringChirho(&'syntax_chirho str),
    FundepsChirho(&'syntax_chirho [(Vec<String>, Vec<String>)]),
    NameChirho(&'syntax_chirho NameChirho),
    NamesChirho(&'syntax_chirho [NameChirho]),
    MultiplicityChirho(&'syntax_chirho MultiplicityChirho),
}

fn name_weight_chirho(name_chirho: &NameChirho) -> Result<usize, GadtProjectionLimitChirho> {
    let raw_chirho = match name_chirho {
        NameChirho::RawChirho(raw_chirho) => raw_chirho,
        NameChirho::ResolvedChirho(resolved_chirho) => &resolved_chirho.raw_chirho,
    };
    // A new name metadata field must force this clone-budget account to be
    // revisited; do not hide potentially owned strings behind `..` here.
    let RawNameChirho {
        text_chirho,
        qualifier_chirho,
        promotion_chirho,
        family_key_chirho,
        type_variable_chirho,
        type_constructor_chirho,
        type_constructor_opaque_chirho: _,
        span_chirho: _,
        origin_chirho: _,
        explicit_type_member_chirho: _,
        written_type_declaration_role_chirho: _,
    } = raw_chirho;
    [
        text_chirho.len(),
        qualifier_chirho.as_ref().map_or(0, String::len),
        family_key_chirho.as_ref().map_or(0, String::len),
        promotion_chirho.as_ref().map_or(0, |promotion_chirho| {
            promotion_chirho.key_chirho().as_str_chirho().len()
        }),
        match type_variable_chirho {
            Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
                declaration_chirho,
                index_chirho: _,
            }) => declaration_chirho.len(),
            Some(ScopedTypeVariableIdChirho::InferenceChirho {
                owner_chirho: _,
                variable_chirho: _,
            })
            | None => 0,
        },
        match type_constructor_chirho {
            Some(TypeConstructorDeclarationChirho {
                identity_chirho,
                role_chirho: _,
            }) => match identity_chirho {
                TypeConstructorIdentityChirho::SourceChirho {
                    defining_module_chirho,
                    name_chirho,
                } => defining_module_chirho
                    .len()
                    .checked_add(name_chirho.len())
                    .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)?,
                TypeConstructorIdentityChirho::CompilerChirho(_) => 0,
            },
            None => 0,
        },
    ]
    .into_iter()
    .try_fold(1_usize, |weight_chirho, bytes_chirho| {
        weight_chirho
            .checked_add(bytes_chirho)
            .ok_or(GadtProjectionLimitChirho::ArithmeticChirho)
    })
}

fn walk_chirho(
    node_chirho: ProjectionNodeChirho<'_>,
    depth_chirho: usize,
    copies_chirho: usize,
    factor_chirho: usize,
    budget_chirho: &mut GadtProjectionBudgetChirho,
) -> Result<(), GadtProjectionLimitChirho> {
    // Slice cursors enqueue only their first child and remaining borrowed
    // suffix. A wide tuple/context never allocates a work item per sibling.
    let mut pending_chirho = Vec::new();
    budget_chirho.work_chirho(1)?;
    pending_chirho.push((node_chirho, depth_chirho));
    while let Some((node_chirho, depth_chirho)) = pending_chirho.pop() {
        budget_chirho.work_chirho(1)?;
        budget_chirho.depth_chirho(depth_chirho, copies_chirho, factor_chirho)?;
        let mut push_chirho =
            |node_chirho, depth_chirho| -> Result<(), GadtProjectionLimitChirho> {
                budget_chirho.work_chirho(1)?;
                pending_chirho.push((node_chirho, depth_chirho));
                Ok(())
            };
        match node_chirho {
            ProjectionNodeChirho::TypesChirho(types_chirho) => {
                if let Some((first_chirho, rest_chirho)) = types_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(ProjectionNodeChirho::TypesChirho(rest_chirho), depth_chirho)?;
                    }
                    push_chirho(ProjectionNodeChirho::TypeChirho(first_chirho), depth_chirho)?;
                }
                continue;
            }
            ProjectionNodeChirho::ConstraintsChirho(constraints_chirho) => {
                if let Some((first_chirho, rest_chirho)) = constraints_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::ConstraintsChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(
                        ProjectionNodeChirho::ConstraintChirho(first_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            ProjectionNodeChirho::BindersChirho(binders_chirho) => {
                if let Some((first_chirho, rest_chirho)) = binders_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::BindersChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(
                        ProjectionNodeChirho::BinderChirho(first_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            ProjectionNodeChirho::NamesChirho(names_chirho) => {
                if let Some((first_chirho, rest_chirho)) = names_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(ProjectionNodeChirho::NamesChirho(rest_chirho), depth_chirho)?;
                    }
                    push_chirho(ProjectionNodeChirho::NameChirho(first_chirho), depth_chirho)?;
                }
                continue;
            }
            ProjectionNodeChirho::FamilyEquationsChirho(items_chirho) => {
                if let Some((first_chirho, rest_chirho)) = items_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::FamilyEquationsChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(
                        ProjectionNodeChirho::FamilyEquationChirho(first_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            ProjectionNodeChirho::AssociatedFamiliesChirho(items_chirho) => {
                if let Some((first_chirho, rest_chirho)) = items_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::AssociatedFamiliesChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(
                        ProjectionNodeChirho::AssociatedFamilyChirho(first_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            ProjectionNodeChirho::AssociatedInstancesChirho(items_chirho) => {
                if let Some((first_chirho, rest_chirho)) = items_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::AssociatedInstancesChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(
                        ProjectionNodeChirho::AssociatedInstanceChirho(first_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            ProjectionNodeChirho::MethodsChirho(items_chirho) => {
                if let Some((first_chirho, rest_chirho)) = items_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::MethodsChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(
                        ProjectionNodeChirho::MethodChirho(first_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            ProjectionNodeChirho::StringsChirho(items_chirho) => {
                if let Some((first_chirho, rest_chirho)) = items_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::StringsChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(
                        ProjectionNodeChirho::StringChirho(first_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            ProjectionNodeChirho::FundepsChirho(items_chirho) => {
                if let Some(((from_chirho, to_chirho), rest_chirho)) = items_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho(
                            ProjectionNodeChirho::FundepsChirho(rest_chirho),
                            depth_chirho,
                        )?;
                    }
                    push_chirho(ProjectionNodeChirho::StringsChirho(to_chirho), depth_chirho)?;
                    push_chirho(
                        ProjectionNodeChirho::StringsChirho(from_chirho),
                        depth_chirho,
                    )?;
                }
                continue;
            }
            _ => {}
        }
        // The cursor closure's borrow ends before charging this syntax node.
        budget_chirho.charge_chirho(1, copies_chirho, factor_chirho)?;
        let child_depth_chirho = next_depth_chirho(depth_chirho)?;
        let mut push_chirho = |node_chirho| -> Result<(), GadtProjectionLimitChirho> {
            budget_chirho.work_chirho(1)?;
            pending_chirho.push((node_chirho, child_depth_chirho));
            Ok(())
        };
        match node_chirho {
            ProjectionNodeChirho::TypeChirho(ty_chirho) => match ty_chirho {
                TypeChirho::VarChirho(name_chirho)
                | TypeChirho::ConChirho(name_chirho)
                | TypeChirho::PromotedConChirho { name_chirho, .. } => {
                    push_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?
                }
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                }
                | TypeChirho::KindApplicationChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(arg_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypeChirho(fun_chirho))?;
                }
                TypeChirho::KindAnnotationChirho {
                    ty_chirho,
                    kind_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(kind_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypeChirho(ty_chirho))?;
                }
                TypeChirho::FunChirho {
                    arg_chirho,
                    mult_chirho,
                    result_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(result_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypeChirho(arg_chirho))?;
                    if let Some(mult_chirho) = mult_chirho {
                        push_chirho(ProjectionNodeChirho::MultiplicityChirho(mult_chirho))?;
                    }
                }
                TypeChirho::TupleChirho {
                    elements_chirho, ..
                }
                | TypeChirho::PromotedListChirho {
                    elements_chirho, ..
                } => push_chirho(ProjectionNodeChirho::TypesChirho(elements_chirho))?,
                TypeChirho::ListChirho { element_chirho, .. } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(element_chirho))?
                }
                TypeChirho::ParenChirho { inner_chirho, .. } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(inner_chirho))?
                }
                TypeChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(body_chirho))?;
                    push_chirho(ProjectionNodeChirho::ConstraintsChirho(context_chirho))?;
                }
                TypeChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                    ..
                }
                | TypeChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(body_chirho))?;
                    push_chirho(ProjectionNodeChirho::BindersChirho(vars_chirho))?;
                }
                TypeChirho::LitChirho { value_chirho, .. } => {
                    budget_chirho.charge_chirho(value_chirho.len(), copies_chirho, factor_chirho)?
                }
                TypeChirho::WildcardChirho { .. } => {}
            },
            ProjectionNodeChirho::ConstraintChirho(constraint_chirho) => match constraint_chirho {
                ConstraintChirho::ClassChirho {
                    class_chirho,
                    kind_args_chirho,
                    args_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::NameChirho(class_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypesChirho(kind_args_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypesChirho(args_chirho))?;
                }
                ConstraintChirho::KindAnnotationChirho {
                    constraint_chirho,
                    kind_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::TypeChirho(kind_chirho))?;
                    push_chirho(ProjectionNodeChirho::ConstraintChirho(constraint_chirho))?;
                }
                ConstraintChirho::QuantifiedChirho {
                    vars_chirho,
                    context_chirho,
                    body_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::BindersChirho(vars_chirho))?;
                    push_chirho(ProjectionNodeChirho::ConstraintsChirho(context_chirho))?;
                    push_chirho(ProjectionNodeChirho::ConstraintChirho(body_chirho))?;
                }
            },
            ProjectionNodeChirho::BinderChirho(binder_chirho) => {
                push_chirho(ProjectionNodeChirho::NameChirho(&binder_chirho.name_chirho))?;
                if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                    push_chirho(ProjectionNodeChirho::TypeChirho(kind_chirho))?;
                }
            }
            ProjectionNodeChirho::DeclarationChirho(declaration_chirho) => match declaration_chirho
            {
                DeclChirho::PatSynSigDeclChirho {
                    names_chirho,
                    ty_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::NamesChirho(names_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypeChirho(ty_chirho))?;
                }
                DeclChirho::TypeSigChirho {
                    name_chirho,
                    ty_chirho,
                    ..
                }
                | DeclChirho::ForeignDeclChirho {
                    name_chirho,
                    ty_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypeChirho(ty_chirho))?;
                }
                DeclChirho::DefaultDeclChirho { types_chirho, .. } => {
                    push_chirho(ProjectionNodeChirho::TypesChirho(types_chirho))?;
                }
                DeclChirho::TypeAliasDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    rhs_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
                    push_chirho(ProjectionNodeChirho::BindersChirho(type_vars_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypeChirho(rhs_chirho))?;
                }
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    result_kind_chirho,
                    standalone_kind_chirho,
                    equations_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
                    push_chirho(ProjectionNodeChirho::BindersChirho(type_vars_chirho))?;
                    push_chirho(ProjectionNodeChirho::FamilyEquationsChirho(
                        equations_chirho,
                    ))?;
                    for kind_chirho in [result_kind_chirho, standalone_kind_chirho]
                        .into_iter()
                        .flatten()
                    {
                        push_chirho(ProjectionNodeChirho::TypeChirho(kind_chirho))?;
                    }
                }
                DeclChirho::TypeFamilyInstanceDeclChirho {
                    family_name_chirho,
                    kind_args_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::NameChirho(family_name_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypesChirho(kind_args_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypesChirho(lhs_types_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypeChirho(rhs_chirho))?;
                }
                DeclChirho::ClassDeclChirho {
                    standalone_kind_chirho,
                    context_chirho,
                    name_chirho,
                    type_vars_chirho,
                    methods_chirho,
                    associated_tfs_chirho,
                    fundeps_chirho,
                    ..
                } => {
                    if let Some(kind_chirho) = standalone_kind_chirho {
                        push_chirho(ProjectionNodeChirho::TypeChirho(kind_chirho))?;
                    }
                    push_chirho(ProjectionNodeChirho::ConstraintsChirho(context_chirho))?;
                    push_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
                    push_chirho(ProjectionNodeChirho::BindersChirho(type_vars_chirho))?;
                    push_chirho(ProjectionNodeChirho::MethodsChirho(methods_chirho))?;
                    push_chirho(ProjectionNodeChirho::AssociatedFamiliesChirho(
                        associated_tfs_chirho,
                    ))?;
                    push_chirho(ProjectionNodeChirho::FundepsChirho(fundeps_chirho))?;
                }
                DeclChirho::InstanceDeclChirho {
                    kind_args_chirho,
                    context_chirho,
                    class_chirho,
                    types_chirho,
                    assoc_tf_instances_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::TypesChirho(kind_args_chirho))?;
                    push_chirho(ProjectionNodeChirho::ConstraintsChirho(context_chirho))?;
                    push_chirho(ProjectionNodeChirho::NameChirho(class_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypesChirho(types_chirho))?;
                    push_chirho(ProjectionNodeChirho::AssociatedInstancesChirho(
                        assoc_tf_instances_chirho,
                    ))?;
                }
                DeclChirho::StandaloneDerivingDeclChirho {
                    context_chirho,
                    class_chirho,
                    types_chirho,
                    ..
                } => {
                    push_chirho(ProjectionNodeChirho::ConstraintsChirho(context_chirho))?;
                    push_chirho(ProjectionNodeChirho::NameChirho(class_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypesChirho(types_chirho))?;
                }
                _ => {}
            },
            ProjectionNodeChirho::FamilyEquationChirho(equation_chirho) => {
                if let Some(head_chirho) = &equation_chirho.head_chirho {
                    push_chirho(ProjectionNodeChirho::NameChirho(head_chirho))?;
                }
                if let Some(binders_chirho) = &equation_chirho.explicit_binders_chirho {
                    push_chirho(ProjectionNodeChirho::BindersChirho(binders_chirho))?;
                }
                push_chirho(ProjectionNodeChirho::NamesChirho(
                    &equation_chirho.unwritten_binders_chirho,
                ))?;
                push_chirho(ProjectionNodeChirho::TypesChirho(
                    &equation_chirho.kind_args_chirho,
                ))?;
                push_chirho(ProjectionNodeChirho::TypesChirho(
                    &equation_chirho.lhs_types_chirho,
                ))?;
                push_chirho(ProjectionNodeChirho::TypeChirho(
                    &equation_chirho.rhs_chirho,
                ))?;
            }
            ProjectionNodeChirho::AssociatedInstanceChirho(instance_chirho) => {
                push_chirho(ProjectionNodeChirho::NameChirho(
                    &instance_chirho.family_name_chirho,
                ))?;
                push_chirho(ProjectionNodeChirho::TypesChirho(
                    &instance_chirho.kind_args_chirho,
                ))?;
                push_chirho(ProjectionNodeChirho::TypesChirho(
                    &instance_chirho.lhs_types_chirho,
                ))?;
                push_chirho(ProjectionNodeChirho::TypeChirho(
                    &instance_chirho.rhs_chirho,
                ))?;
            }
            ProjectionNodeChirho::AssociatedFamilyChirho(family_chirho) => {
                push_chirho(ProjectionNodeChirho::NameChirho(&family_chirho.name_chirho))?;
                push_chirho(ProjectionNodeChirho::BindersChirho(
                    &family_chirho.type_vars_chirho,
                ))?;
                if let Some(kind_chirho) = &family_chirho.result_kind_chirho {
                    push_chirho(ProjectionNodeChirho::TypeChirho(kind_chirho))?;
                }
                push_chirho(ProjectionNodeChirho::NamesChirho(
                    &family_chirho.default_params_chirho,
                ))?;
                if let Some(binders_chirho) = &family_chirho.default_binders_chirho {
                    push_chirho(ProjectionNodeChirho::BindersChirho(binders_chirho))?;
                }
                if let Some(rhs_chirho) = &family_chirho.default_rhs_chirho {
                    push_chirho(ProjectionNodeChirho::TypeChirho(rhs_chirho))?;
                }
                if let Some((kinds_chirho, types_chirho)) = &family_chirho.default_arguments_chirho
                {
                    push_chirho(ProjectionNodeChirho::TypesChirho(kinds_chirho))?;
                    push_chirho(ProjectionNodeChirho::TypesChirho(types_chirho))?;
                }
            }
            ProjectionNodeChirho::MethodChirho(method_chirho) => {
                push_chirho(ProjectionNodeChirho::NameChirho(&method_chirho.name_chirho))?;
                push_chirho(ProjectionNodeChirho::TypeChirho(&method_chirho.ty_chirho))?;
                if let Some(signature_chirho) = &method_chirho.default_sig_chirho {
                    push_chirho(ProjectionNodeChirho::StringChirho(signature_chirho))?;
                }
            }
            ProjectionNodeChirho::StringChirho(text_chirho) => {
                budget_chirho.charge_chirho(text_chirho.len(), copies_chirho, factor_chirho)?
            }
            ProjectionNodeChirho::NameChirho(name_chirho) => budget_chirho.charge_chirho(
                name_weight_chirho(name_chirho)?,
                copies_chirho,
                factor_chirho,
            )?,
            ProjectionNodeChirho::MultiplicityChirho(MultiplicityChirho::MultVarChirho(
                name_chirho,
            )) => push_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?,
            ProjectionNodeChirho::MultiplicityChirho(
                MultiplicityChirho::OneChirho | MultiplicityChirho::ManyChirho,
            ) => {}
            ProjectionNodeChirho::TypesChirho(_)
            | ProjectionNodeChirho::ConstraintsChirho(_)
            | ProjectionNodeChirho::BindersChirho(_)
            | ProjectionNodeChirho::NamesChirho(_)
            | ProjectionNodeChirho::FamilyEquationsChirho(_)
            | ProjectionNodeChirho::AssociatedFamiliesChirho(_)
            | ProjectionNodeChirho::AssociatedInstancesChirho(_)
            | ProjectionNodeChirho::MethodsChirho(_)
            | ProjectionNodeChirho::StringsChirho(_)
            | ProjectionNodeChirho::FundepsChirho(_) => {}
        }
    }
    Ok(())
}
