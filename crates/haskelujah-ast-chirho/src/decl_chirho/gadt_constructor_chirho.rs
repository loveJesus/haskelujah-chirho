// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One source of truth for a GADT constructor's prefixes, arguments and result.
//! The type signature is reconstructed for existing type consumers; named
//! arguments and field annotations stay available to constructor consumers.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use std::collections::HashSet;

use super::{DeclChirho, FieldDeclChirho, StrictnessChirho, TyVarChirho};
use crate::module_chirho::ModuleChirho;
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

#[derive(Debug, Clone, PartialEq)]
pub struct GadtConSignatureChirho {
    pub prefix_chirho: Vec<GadtConPrefixChirho>,
    pub args_chirho: GadtConArgsChirho,
    pub result_ty_chirho: TypeChirho,
    /// Recovery must not fabricate a valid zero-argument constructor.
    pub malformed_chirho: bool,
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
            malformed_chirho: false,
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

    /// A selector may expose only variables recoverable from the written
    /// result through positively known generative constructors. Type-synonym
    /// families and heads whose role is unknown supply no proof.
    /// The same decision is read by typing, Core and source interfaces.
    pub fn field_selector_available_chirho(
        &self,
        field_chirho: &FieldDeclChirho,
        generative_head_chirho: &impl Fn(&NameChirho) -> bool,
    ) -> bool {
        let mut recoverable_chirho = HashSet::new();
        let mut remaining_chirho = 65_536_usize;
        if !recoverable_result_vars_chirho(
            &self.result_ty_chirho,
            generative_head_chirho,
            &mut recoverable_chirho,
            &mut remaining_chirho,
        ) {
            return false;
        }
        let mut field_vars_chirho = HashSet::new();
        if !field_free_vars_chirho(
            &field_chirho.ty_chirho,
            &mut HashSet::new(),
            &mut field_vars_chirho,
            &mut remaining_chirho,
        ) {
            return false;
        }
        field_vars_chirho.is_subset(&recoverable_chirho)
    }

    /// Reconstruct the signature at the old Type consumer boundary. Field
    /// strictness is constructor metadata, not an ordinary type wrapper.
    pub fn as_type_chirho(&self) -> TypeChirho {
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

/// Source declarations are positive generativity authority. Merely knowing a
/// type's spelling, including an imported abstract type or synonym, is not.
/// Structural list, tuple and function types are handled by the shared walk.
pub fn local_generative_type_names_chirho(module_chirho: &ModuleChirho) -> HashSet<String> {
    let mut names_chirho = HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::DataDeclChirho { name_chirho, .. }
        | DeclChirho::NewtypeDeclChirho { name_chirho, .. } = decl_chirho
        {
            names_chirho.insert(name_chirho.text_chirho().to_string());
            names_chirho.insert(format!(
                "{}.{}",
                module_chirho.name_chirho.full_name_chirho(),
                name_chirho.text_chirho()
            ));
        }
    }
    names_chirho
}

fn spend_selector_work_chirho(remaining_chirho: &mut usize) -> bool {
    if *remaining_chirho == 0 {
        false
    } else {
        *remaining_chirho -= 1;
        true
    }
}

fn recoverable_result_vars_chirho(
    ty_chirho: &TypeChirho,
    generative_head_chirho: &impl Fn(&NameChirho) -> bool,
    variables_chirho: &mut HashSet<String>,
    remaining_chirho: &mut usize,
) -> bool {
    if !spend_selector_work_chirho(remaining_chirho) {
        return false;
    }
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            variables_chirho.insert(name_chirho.text_chirho().to_string());
        }
        TypeChirho::AppChirho { .. } => {
            let mut head_chirho = ty_chirho;
            let mut args_chirho = Vec::new();
            while let TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } = head_chirho
            {
                if !spend_selector_work_chirho(remaining_chirho) {
                    return false;
                }
                args_chirho.push(arg_chirho.as_ref());
                head_chirho = fun_chirho;
            }
            if let TypeChirho::ConChirho(name_chirho) = head_chirho
                && generative_head_chirho(name_chirho)
            {
                for arg_chirho in args_chirho {
                    if !recoverable_result_vars_chirho(
                        arg_chirho,
                        generative_head_chirho,
                        variables_chirho,
                        remaining_chirho,
                    ) {
                        return false;
                    }
                }
            }
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                if !recoverable_result_vars_chirho(
                    element_chirho,
                    generative_head_chirho,
                    variables_chirho,
                    remaining_chirho,
                ) {
                    return false;
                }
            }
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => {
            return recoverable_result_vars_chirho(
                element_chirho,
                generative_head_chirho,
                variables_chirho,
                remaining_chirho,
            );
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
            return recoverable_result_vars_chirho(
                arg_chirho,
                generative_head_chirho,
                variables_chirho,
                remaining_chirho,
            ) && recoverable_result_vars_chirho(
                result_chirho,
                generative_head_chirho,
                variables_chirho,
                remaining_chirho,
            );
        }
        _ => {}
    }
    true
}

fn field_free_vars_chirho(
    ty_chirho: &TypeChirho,
    bound_chirho: &mut HashSet<String>,
    free_chirho: &mut HashSet<String>,
    remaining_chirho: &mut usize,
) -> bool {
    if !spend_selector_work_chirho(remaining_chirho) {
        return false;
    }
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
            return field_free_vars_chirho(fun_chirho, bound_chirho, free_chirho, remaining_chirho)
                && field_free_vars_chirho(arg_chirho, bound_chirho, free_chirho, remaining_chirho);
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                if !field_free_vars_chirho(
                    element_chirho,
                    bound_chirho,
                    free_chirho,
                    remaining_chirho,
                ) {
                    return false;
                }
            }
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => {
            return field_free_vars_chirho(
                element_chirho,
                bound_chirho,
                free_chirho,
                remaining_chirho,
            );
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
            let added_chirho = vars_chirho
                .iter()
                .filter_map(|var_chirho| {
                    let name_chirho = var_chirho.name_chirho.text_chirho().to_string();
                    bound_chirho
                        .insert(name_chirho.clone())
                        .then_some(name_chirho)
                })
                .collect::<Vec<_>>();
            let success_chirho =
                field_free_vars_chirho(body_chirho, bound_chirho, free_chirho, remaining_chirho);
            for name_chirho in added_chirho {
                bound_chirho.remove(&name_chirho);
            }
            return success_chirho;
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for pred_chirho in context_chirho {
                if !constraint_free_vars_chirho(
                    pred_chirho,
                    bound_chirho,
                    free_chirho,
                    remaining_chirho,
                ) {
                    return false;
                }
            }
            return field_free_vars_chirho(
                body_chirho,
                bound_chirho,
                free_chirho,
                remaining_chirho,
            );
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
    remaining_chirho: &mut usize,
) -> bool {
    if !spend_selector_work_chirho(remaining_chirho) {
        return false;
    }
    match pred_chirho {
        ConstraintChirho::ClassChirho { args_chirho, .. } => args_chirho.iter().all(|arg_chirho| {
            field_free_vars_chirho(arg_chirho, bound_chirho, free_chirho, remaining_chirho)
        }),
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
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
            let success_chirho = context_chirho.iter().all(|pred_chirho| {
                constraint_free_vars_chirho(
                    pred_chirho,
                    bound_chirho,
                    free_chirho,
                    remaining_chirho,
                )
            }) && constraint_free_vars_chirho(
                body_chirho,
                bound_chirho,
                free_chirho,
                remaining_chirho,
            );
            for name_chirho in added_chirho {
                bound_chirho.remove(&name_chirho);
            }
            success_chirho
        }
    }
}

#[cfg(test)]
#[path = "gadt_constructor_tests_chirho.rs"]
mod tests_chirho;
