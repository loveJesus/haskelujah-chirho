// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One source of truth for a GADT constructor's prefixes, arguments and result.
//! The type signature is reconstructed for existing type consumers; named
//! arguments and field annotations stay available to constructor consumers.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use std::collections::HashSet;

use super::{
    FieldDeclChirho, GadtProjectionBudgetChirho, GadtProjectionErrorChirho, StrictnessChirho,
    TyVarChirho, check_projection_chirho,
};
use crate::name_chirho::NameChirho;
use crate::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

#[derive(Debug, Clone, PartialEq)]
pub enum GadtConPrefixChirho {
    ForallChirho {
        vars_chirho: Vec<TyVarChirho>,
        required_chirho: bool,
        span_chirho: SpanChirho,
    },
    ContextChirho {
        constraints_chirho: Vec<ConstraintChirho>,
        span_chirho: SpanChirho,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct GadtConFieldChirho {
    pub strictness_chirho: StrictnessChirho,
    pub multiplicity_chirho: Option<MultiplicityChirho>,
    pub ty_chirho: TypeChirho,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GadtConArgsChirho {
    PositionalChirho(Vec<GadtConFieldChirho>),
    RecordChirho(Vec<FieldDeclChirho>),
}

/// The producer retains why a constructor signature cannot be consumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GadtConMalformedChirho {
    IncompleteChirho,
    IllegalRecordArrowChirho,
    UnsupportedMultiplicityChirho,
    ResourceChirho,
}

impl GadtConMalformedChirho {
    pub fn ghc_code_chirho(self) -> Option<&'static str> {
        match self {
            Self::IncompleteChirho => Some("GHC-58481"),
            Self::IllegalRecordArrowChirho => Some("GHC-37475"),
            Self::UnsupportedMultiplicityChirho | Self::ResourceChirho => None,
        }
    }

    pub fn description_chirho(self) -> &'static str {
        match self {
            Self::IncompleteChirho => "Malformed GADT constructor signature",
            Self::IllegalRecordArrowChirho => {
                "Record constructors in GADTs must use an ordinary, non-linear arrow"
            }
            Self::UnsupportedMultiplicityChirho => "Unsupported GADT constructor multiplicity",
            Self::ResourceChirho => {
                "GADT constructor signature exceeds the compiler's bounded parser work"
            }
        }
    }
}

/// A retained malformed producer and a resource refusal are distinct failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructorSignatureErrorChirho {
    MalformedChirho {
        reason_chirho: GadtConMalformedChirho,
        span_chirho: SpanChirho,
    },
    ProjectionChirho(GadtProjectionErrorChirho),
}

impl ConstructorSignatureErrorChirho {
    pub fn span_chirho(self) -> SpanChirho {
        match self {
            Self::MalformedChirho { span_chirho, .. } => span_chirho,
            Self::ProjectionChirho(error_chirho) => error_chirho.span_chirho,
        }
    }
    pub fn ghc_code_chirho(self) -> Option<&'static str> {
        match self {
            Self::MalformedChirho { reason_chirho, .. } => reason_chirho.ghc_code_chirho(),
            Self::ProjectionChirho(_) => None,
        }
    }
}
impl From<GadtProjectionErrorChirho> for ConstructorSignatureErrorChirho {
    fn from(error_chirho: GadtProjectionErrorChirho) -> Self {
        Self::ProjectionChirho(error_chirho)
    }
}
impl std::fmt::Display for ConstructorSignatureErrorChirho {
    fn fmt(&self, formatter_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedChirho { reason_chirho, .. } => {
                if let Some(ghc_code_chirho) = reason_chirho.ghc_code_chirho() {
                    write!(
                        formatter_chirho,
                        "{} ({ghc_code_chirho})",
                        reason_chirho.description_chirho()
                    )
                } else {
                    formatter_chirho.write_str(reason_chirho.description_chirho())
                }
            }
            Self::ProjectionChirho(error_chirho) => {
                std::fmt::Display::fmt(error_chirho, formatter_chirho)
            }
        }
    }
}
impl std::error::Error for ConstructorSignatureErrorChirho {}

#[derive(Debug, Clone, PartialEq)]
pub struct GadtConSignatureChirho {
    pub prefix_chirho: Vec<GadtConPrefixChirho>,
    pub args_chirho: GadtConArgsChirho,
    pub result_ty_chirho: TypeChirho,
    /// Recovery must not fabricate a valid zero-argument constructor.
    pub malformed_chirho: Option<GadtConMalformedChirho>,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone, Copy)]
pub struct GadtConFieldRefChirho<'field_chirho> {
    pub strictness_chirho: StrictnessChirho,
    pub multiplicity_chirho: Option<&'field_chirho MultiplicityChirho>,
    pub ty_chirho: &'field_chirho TypeChirho,
    pub span_chirho: SpanChirho,
}

impl GadtConSignatureChirho {
    /// A type-only AST producer, such as TH, supplies positional arguments.
    /// Source parsing supplies field annotations and record structure directly.
    pub fn from_type_chirho(mut ty_chirho: TypeChirho) -> Self {
        let span_chirho = ty_chirho.span_chirho();
        let mut prefix_chirho = Vec::new();
        loop {
            match ty_chirho {
                TypeChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                    span_chirho,
                } => {
                    prefix_chirho.push(GadtConPrefixChirho::ForallChirho {
                        vars_chirho,
                        required_chirho: false,
                        span_chirho,
                    });
                    ty_chirho = *body_chirho;
                }
                TypeChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                    span_chirho,
                } => {
                    prefix_chirho.push(GadtConPrefixChirho::ForallChirho {
                        vars_chirho,
                        required_chirho: true,
                        span_chirho,
                    });
                    ty_chirho = *body_chirho;
                }
                TypeChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                    span_chirho,
                } => {
                    prefix_chirho.push(GadtConPrefixChirho::ContextChirho {
                        constraints_chirho: context_chirho,
                        span_chirho,
                    });
                    ty_chirho = *body_chirho;
                }
                TypeChirho::ParenChirho { inner_chirho, .. } => ty_chirho = *inner_chirho,
                _ => break,
            }
        }
        let mut fields_chirho = Vec::new();
        loop {
            match ty_chirho {
                TypeChirho::FunChirho {
                    arg_chirho,
                    mult_chirho,
                    result_chirho,
                    ..
                } => {
                    let field_span_chirho = arg_chirho.span_chirho();
                    fields_chirho.push(GadtConFieldChirho {
                        strictness_chirho: StrictnessChirho::LazyChirho,
                        multiplicity_chirho: mult_chirho,
                        ty_chirho: *arg_chirho,
                        span_chirho: field_span_chirho,
                    });
                    ty_chirho = *result_chirho;
                }
                TypeChirho::ParenChirho { inner_chirho, .. } => ty_chirho = *inner_chirho,
                _ => break,
            }
        }
        Self {
            prefix_chirho,
            args_chirho: GadtConArgsChirho::PositionalChirho(fields_chirho),
            result_ty_chirho: ty_chirho,
            malformed_chirho: None,
            span_chirho,
        }
    }

    /// Flatten grouped record labels into runtime argument order without
    /// allocating or storing a second copy of their types.
    pub fn positional_fields_chirho(
        &self,
    ) -> impl DoubleEndedIterator<Item = GadtConFieldRefChirho<'_>> {
        let (positional_chirho, records_chirho): (&[GadtConFieldChirho], &[FieldDeclChirho]) =
            match &self.args_chirho {
                GadtConArgsChirho::PositionalChirho(fields_chirho) => (fields_chirho, &[]),
                GadtConArgsChirho::RecordChirho(fields_chirho) => (&[], fields_chirho),
            };
        positional_chirho
            .iter()
            .map(|field_chirho| GadtConFieldRefChirho {
                strictness_chirho: field_chirho.strictness_chirho,
                multiplicity_chirho: field_chirho.multiplicity_chirho.as_ref(),
                ty_chirho: &field_chirho.ty_chirho,
                span_chirho: field_chirho.span_chirho,
            })
            .chain(records_chirho.iter().flat_map(|field_chirho| {
                field_chirho
                    .names_chirho
                    .iter()
                    .map(move |_| GadtConFieldRefChirho {
                        strictness_chirho: field_chirho.strictness_chirho,
                        multiplicity_chirho: field_chirho.multiplicity_chirho.as_ref(),
                        ty_chirho: &field_chirho.ty_chirho,
                        span_chirho: field_chirho.span_chirho,
                    })
            }))
    }

    pub fn record_fields_chirho(&self) -> Option<&[FieldDeclChirho]> {
        match &self.args_chirho {
            GadtConArgsChirho::RecordChirho(fields_chirho) => Some(fields_chirho),
            GadtConArgsChirho::PositionalChirho(_) => None,
        }
    }

    pub fn has_context_chirho(&self) -> bool {
        self.prefix_chirho.iter().any(|prefix_chirho| matches!(
            prefix_chirho,
            GadtConPrefixChirho::ContextChirho { constraints_chirho, .. } if !constraints_chirho.is_empty()
        ))
    }

    pub fn check_written_signature_chirho(&self) -> Result<(), ConstructorSignatureErrorChirho> {
        if let Some(reason_chirho) = self.malformed_chirho {
            return Err(ConstructorSignatureErrorChirho::MalformedChirho {
                reason_chirho,
                span_chirho: self.span_chirho,
            });
        }
        Ok(())
    }

    /// Decide once per stored field whether a selector can expose its type.
    /// Result recovery is scanned once, and grouped labels reuse one decision.
    /// Unknown semantic recovery is false; resource exhaustion is a named Err.
    pub fn selector_availability_chirho(
        &self,
        budget_chirho: &mut GadtProjectionBudgetChirho,
        generative_head_chirho: &impl Fn(&NameChirho) -> bool,
    ) -> Result<Vec<bool>, ConstructorSignatureErrorChirho> {
        self.check_written_signature_chirho()?;
        check_projection_chirho(self, budget_chirho, 1)?;
        let mut recoverable_chirho = HashSet::new();
        recoverable_result_vars_chirho(
            &self.result_ty_chirho,
            generative_head_chirho,
            &mut recoverable_chirho,
        );
        recoverable_binder_kinds_chirho(&self.prefix_chirho, &mut recoverable_chirho);
        let mut availability_chirho = Vec::new();
        for field_chirho in self.record_fields_chirho().unwrap_or_default() {
            let mut field_vars_chirho = HashSet::new();
            let faithful_chirho = field_free_vars_chirho(
                &field_chirho.ty_chirho,
                &mut HashSet::new(),
                &mut field_vars_chirho,
            );
            availability_chirho
                .push(faithful_chirho && field_vars_chirho.is_subset(&recoverable_chirho));
        }
        Ok(availability_chirho)
    }

    /// Reconstruct only after the whole amplified output has been charged.
    pub fn try_as_type_chirho(
        &self,
        budget_chirho: &mut GadtProjectionBudgetChirho,
    ) -> Result<TypeChirho, ConstructorSignatureErrorChirho> {
        self.check_written_signature_chirho()?;
        check_projection_chirho(self, budget_chirho, 1)?;
        Ok(self.reconstruct_type_chirho())
    }

    /// Reconstruct the signature at the old Type consumer boundary. Field
    /// strictness is constructor metadata, not an ordinary type wrapper.
    fn reconstruct_type_chirho(&self) -> TypeChirho {
        let mut ty_chirho = self.result_ty_chirho.clone();
        for field_chirho in self.positional_fields_chirho().rev() {
            let span_chirho = field_chirho
                .span_chirho
                .merge_chirho(ty_chirho.span_chirho())
                .unwrap_or(self.span_chirho);
            ty_chirho = TypeChirho::FunChirho {
                arg_chirho: Box::new(field_chirho.ty_chirho.clone()),
                mult_chirho: field_chirho.multiplicity_chirho.cloned(),
                result_chirho: Box::new(ty_chirho),
                span_chirho,
            };
        }
        for prefix_chirho in self.prefix_chirho.iter().rev() {
            ty_chirho = match prefix_chirho {
                GadtConPrefixChirho::ForallChirho {
                    vars_chirho,
                    required_chirho,
                    span_chirho,
                } => {
                    if *required_chirho {
                        TypeChirho::RequiredForallChirho {
                            vars_chirho: vars_chirho.clone(),
                            body_chirho: Box::new(ty_chirho),
                            span_chirho: *span_chirho,
                        }
                    } else {
                        TypeChirho::ForallChirho {
                            vars_chirho: vars_chirho.clone(),
                            body_chirho: Box::new(ty_chirho),
                            span_chirho: *span_chirho,
                        }
                    }
                }
                GadtConPrefixChirho::ContextChirho {
                    constraints_chirho,
                    span_chirho,
                } => TypeChirho::QualChirho {
                    context_chirho: constraints_chirho.clone(),
                    body_chirho: Box::new(ty_chirho),
                    span_chirho: *span_chirho,
                },
            };
        }
        ty_chirho
    }
}

/// Check only the retained producer's malformed state, never broader language validity.
pub fn check_constructor_signature_chirho(
    con_chirho: &super::ConDeclChirho,
) -> Result<(), ConstructorSignatureErrorChirho> {
    if let super::ConDeclChirho::GadtChirho {
        signature_chirho, ..
    } = con_chirho
    {
        signature_chirho.check_written_signature_chirho()?;
    }
    Ok(())
}

pub fn check_module_constructor_signatures_chirho(
    module_chirho: &crate::module_chirho::ModuleChirho,
) -> Result<(), ConstructorSignatureErrorChirho> {
    for decl_chirho in &module_chirho.decls_chirho {
        let constructors_chirho = match decl_chirho {
            super::DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => constructors_chirho.as_slice(),
            super::DeclChirho::NewtypeDeclChirho {
                constructor_chirho, ..
            } => std::slice::from_ref(constructor_chirho),
            _ => continue,
        };
        for constructor_chirho in constructors_chirho {
            check_constructor_signature_chirho(constructor_chirho)?;
        }
    }
    Ok(())
}

fn recoverable_result_vars_chirho(
    ty_chirho: &TypeChirho,
    generative_head_chirho: &impl Fn(&NameChirho) -> bool,
    variables_chirho: &mut HashSet<String>,
) {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            variables_chirho.insert(name_chirho.text_chirho().to_string());
        }
        TypeChirho::AppChirho { .. } | TypeChirho::KindApplicationChirho { .. } => {
            let mut head_chirho = ty_chirho;
            let mut args_chirho = Vec::new();
            loop {
                match head_chirho {
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
                        args_chirho.push(arg_chirho.as_ref());
                        head_chirho = fun_chirho;
                    }
                    _ => break,
                }
            }
            if let TypeChirho::ConChirho(name_chirho) = head_chirho
                && generative_head_chirho(name_chirho)
            {
                for arg_chirho in args_chirho {
                    recoverable_result_vars_chirho(
                        arg_chirho,
                        generative_head_chirho,
                        variables_chirho,
                    );
                }
            }
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                recoverable_result_vars_chirho(
                    element_chirho,
                    generative_head_chirho,
                    variables_chirho,
                );
            }
        }
        TypeChirho::KindAnnotationChirho {
            ty_chirho: element_chirho,
            ..
        }
        | TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => {
            recoverable_result_vars_chirho(element_chirho, generative_head_chirho, variables_chirho)
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            if let Some(MultiplicityChirho::MultVarChirho(name_chirho)) = mult_chirho {
                variables_chirho.insert(name_chirho.text_chirho().to_string());
            }
            recoverable_result_vars_chirho(arg_chirho, generative_head_chirho, variables_chirho);
            recoverable_result_vars_chirho(result_chirho, generative_head_chirho, variables_chirho);
        }
        _ => {}
    }
}

/// A result variable determines its own directly written kind. Only names
/// proved to be constructor telescope binders form dependencies; an applied
/// kind can be a noninjective family, so it supplies no decomposition proof.
/// The projection preflight already charged the stored names/kind nodes. Each
/// dependency and recovered binder is visited once in this bounded worklist.
fn recoverable_binder_kinds_chirho(
    prefix_chirho: &[GadtConPrefixChirho],
    recoverable_chirho: &mut HashSet<String>,
) {
    let binder_names_chirho = prefix_chirho
        .iter()
        .filter_map(|prefix_chirho| {
            if let GadtConPrefixChirho::ForallChirho { vars_chirho, .. } = prefix_chirho {
                Some(vars_chirho)
            } else {
                None
            }
        })
        .flatten()
        .map(|variable_chirho| variable_chirho.name_chirho.text_chirho().to_owned())
        .collect::<HashSet<_>>();
    let mut dependencies_chirho = std::collections::HashMap::<String, Vec<String>>::new();
    for prefix_chirho in prefix_chirho {
        if let GadtConPrefixChirho::ForallChirho { vars_chirho, .. } = prefix_chirho {
            for variable_chirho in vars_chirho {
                let mut names_chirho = Vec::new();
                if let Some(kind_chirho) = &variable_chirho.kind_annotation_chirho {
                    direct_kind_dependencies_chirho(
                        kind_chirho,
                        &binder_names_chirho,
                        &mut names_chirho,
                    );
                }
                // A later telescope binder replaces an outer same-named one,
                // including an unannotated binder with no dependencies.
                dependencies_chirho.insert(
                    variable_chirho.name_chirho.text_chirho().to_owned(),
                    names_chirho,
                );
            }
        }
    }
    let mut pending_chirho = recoverable_chirho
        .iter()
        .cloned()
        .collect::<std::collections::VecDeque<_>>();
    while let Some(name_chirho) = pending_chirho.pop_front() {
        if let Some(dependencies_chirho) = dependencies_chirho.remove(&name_chirho) {
            for dependency_chirho in dependencies_chirho {
                if recoverable_chirho.insert(dependency_chirho.clone()) {
                    pending_chirho.push_back(dependency_chirho);
                }
            }
        }
    }
}

fn direct_kind_dependencies_chirho(
    kind_chirho: &TypeChirho,
    binders_chirho: &HashSet<String>,
    dependencies_chirho: &mut Vec<String>,
) {
    match kind_chirho {
        TypeChirho::VarChirho(name_chirho)
            if binders_chirho.contains(name_chirho.text_chirho()) =>
        {
            dependencies_chirho.push(name_chirho.text_chirho().to_owned());
        }
        TypeChirho::FunChirho {
            arg_chirho: left_chirho,
            result_chirho: right_chirho,
            ..
        } => {
            direct_kind_dependencies_chirho(left_chirho, binders_chirho, dependencies_chirho);
            direct_kind_dependencies_chirho(right_chirho, binders_chirho, dependencies_chirho);
        }
        TypeChirho::ParenChirho { inner_chirho, .. }
        | TypeChirho::KindAnnotationChirho {
            ty_chirho: inner_chirho,
            ..
        } => direct_kind_dependencies_chirho(inner_chirho, binders_chirho, dependencies_chirho),
        // A kind application can be a synonym family. Source shape alone
        // provides no injective decomposition proof; semantic facts follow688.
        _ => {}
    }
}

fn field_free_vars_chirho(
    ty_chirho: &TypeChirho,
    bound_chirho: &mut HashSet<String>,
    free_chirho: &mut HashSet<String>,
) -> bool {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            if !bound_chirho.contains(name_chirho.text_chirho()) {
                free_chirho.insert(name_chirho.text_chirho().to_string());
            }
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
        }
        | TypeChirho::KindAnnotationChirho {
            ty_chirho: fun_chirho,
            kind_chirho: arg_chirho,
            ..
        }
        | TypeChirho::FunChirho {
            arg_chirho: fun_chirho,
            result_chirho: arg_chirho,
            ..
        } => {
            if let TypeChirho::FunChirho {
                mult_chirho: Some(MultiplicityChirho::MultVarChirho(name_chirho)),
                ..
            } = ty_chirho
                && !bound_chirho.contains(name_chirho.text_chirho())
            {
                free_chirho.insert(name_chirho.text_chirho().to_string());
            }
            return field_free_vars_chirho(fun_chirho, bound_chirho, free_chirho)
                && field_free_vars_chirho(arg_chirho, bound_chirho, free_chirho);
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                if !field_free_vars_chirho(element_chirho, bound_chirho, free_chirho) {
                    return false;
                }
            }
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => return field_free_vars_chirho(element_chirho, bound_chirho, free_chirho),
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
            let added_chirho = vars_chirho
                .iter()
                .filter_map(|var_chirho| {
                    let name_chirho = var_chirho.name_chirho.text_chirho().to_string();
                    bound_chirho
                        .insert(name_chirho.clone())
                        .then_some(name_chirho)
                })
                .collect::<Vec<_>>();
            let mut faithful_chirho = true;
            for variable_chirho in vars_chirho {
                if let Some(kind_chirho) = &variable_chirho.kind_annotation_chirho {
                    faithful_chirho &=
                        field_free_vars_chirho(kind_chirho, bound_chirho, free_chirho);
                }
            }
            faithful_chirho &= field_free_vars_chirho(body_chirho, bound_chirho, free_chirho);
            for name_chirho in added_chirho {
                bound_chirho.remove(&name_chirho);
            }
            return faithful_chirho;
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for pred_chirho in context_chirho {
                if !constraint_free_vars_chirho(pred_chirho, bound_chirho, free_chirho) {
                    return false;
                }
            }
            return field_free_vars_chirho(body_chirho, bound_chirho, free_chirho);
        }
        TypeChirho::WildcardChirho { .. } => return false,
        _ => {}
    }
    true
}

fn constraint_free_vars_chirho(
    pred_chirho: &ConstraintChirho,
    bound_chirho: &mut HashSet<String>,
    free_chirho: &mut HashSet<String>,
) -> bool {
    match pred_chirho {
        ConstraintChirho::ClassChirho {
            args_chirho,
            kind_args_chirho,
            ..
        } => kind_args_chirho
            .iter()
            .chain(args_chirho)
            .all(|arg_chirho| field_free_vars_chirho(arg_chirho, bound_chirho, free_chirho)),
        ConstraintChirho::KindAnnotationChirho {
            constraint_chirho,
            kind_chirho,
            ..
        } => {
            constraint_free_vars_chirho(constraint_chirho, bound_chirho, free_chirho)
                && field_free_vars_chirho(kind_chirho, bound_chirho, free_chirho)
        }
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => {
            let added_chirho = vars_chirho
                .iter()
                .filter_map(|var_chirho| {
                    let name_chirho = var_chirho.name_chirho.text_chirho().to_owned();
                    bound_chirho
                        .insert(name_chirho.clone())
                        .then_some(name_chirho)
                })
                .collect::<Vec<_>>();
            let mut faithful_chirho = true;
            for variable_chirho in vars_chirho {
                if let Some(kind_chirho) = &variable_chirho.kind_annotation_chirho {
                    faithful_chirho &=
                        field_free_vars_chirho(kind_chirho, bound_chirho, free_chirho);
                }
            }
            faithful_chirho &=
                context_chirho.iter().all(|pred_chirho| {
                    constraint_free_vars_chirho(pred_chirho, bound_chirho, free_chirho)
                }) && constraint_free_vars_chirho(body_chirho, bound_chirho, free_chirho);
            for name_chirho in added_chirho {
                bound_chirho.remove(&name_chirho);
            }
            faithful_chirho
        }
    }
}

#[cfg(test)]
#[path = "gadt_constructor_tests_chirho.rs"]
mod tests_chirho;
