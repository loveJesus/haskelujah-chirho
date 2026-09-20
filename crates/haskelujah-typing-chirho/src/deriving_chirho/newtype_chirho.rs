// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Syntax-preserving GND construction; checked kind selection lives in kind inference.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    ConDeclChirho, DeclChirho, NameChirho, RawNameChirho, SpanChirho, TyVarChirho, TypeChirho,
};
use haskelujah_ast_chirho::decl_chirho::AstKindChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho};
use std::collections::HashSet;

pub(crate) fn representation_chirho(constructor_chirho: &ConDeclChirho) -> Option<&TypeChirho> {
    match constructor_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } if fields_chirho.len() == 1 => {
            Some(&fields_chirho[0].1)
        }
        ConDeclChirho::RecordChirho { fields_chirho, .. }
            if fields_chirho.len() == 1 && fields_chirho[0].names_chirho.len() == 1 =>
        {
            Some(&fields_chirho[0].ty_chirho)
        }
        ConDeclChirho::GadtChirho { ty_chirho, .. } => {
            let mut current_chirho = ty_chirho;
            loop {
                match current_chirho.unannotated_chirho() {
                    TypeChirho::ForallChirho { body_chirho, .. }
                    | TypeChirho::QualChirho { body_chirho, .. } => current_chirho = body_chirho,
                    TypeChirho::FunChirho {
                        arg_chirho,
                        result_chirho,
                        ..
                    } if !matches!(
                        result_chirho.unannotated_chirho(),
                        TypeChirho::FunChirho { .. }
                    ) =>
                    {
                        return Some(arg_chirho);
                    }
                    _ => return None,
                }
            }
        }
        _ => None,
    }
}

const ETA_WORK_LIMIT_CHIRHO: &str = "newtype eta-reduction exceeded its work limit";
const ETA_SHAPE_ERROR_CHIRHO: &str = "cannot eta-reduce the representation type enough";

fn constructor_chirho(name_chirho: impl Into<String>) -> TypeChirho {
    TypeChirho::ConChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        name_chirho,
        SpanChirho::DUMMY_CHIRHO,
    )))
}

fn application_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// Remove only exact trailing variables. The residual representation cannot
/// mention any removed binder. Validate depth/work before cloning the tree;
/// sugar for unrestricted arrows, lists and tuples retains its constructor.
pub(crate) fn eta_reduce_chirho(
    representation_chirho: &TypeChirho,
    removed_chirho: &[TyVarChirho],
) -> Result<TypeChirho, &'static str> {
    let mut budget_chirho = 16_384usize;
    mentions_removed_chirho(
        representation_chirho,
        &HashSet::new(),
        &mut budget_chirho,
        128,
    )?;
    let mut residual_chirho = representation_chirho.clone();
    for variable_chirho in removed_chirho.iter().rev() {
        while let TypeChirho::ParenChirho { inner_chirho, .. }
        | TypeChirho::KindAnnotChirho {
            type_chirho: inner_chirho,
            ..
        } = residual_chirho
        {
            residual_chirho = *inner_chirho;
        }
        let (prefix_chirho, argument_chirho) = match residual_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => (*fun_chirho, *arg_chirho),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                mult_chirho: None | Some(MultiplicityChirho::ManyChirho),
                ..
            } => (
                application_chirho(constructor_chirho("->"), *arg_chirho),
                *result_chirho,
            ),
            TypeChirho::ListChirho { element_chirho, .. } => {
                (constructor_chirho("[]"), *element_chirho)
            }
            TypeChirho::TupleChirho {
                mut elements_chirho,
                ..
            } if elements_chirho.len() >= 2 => {
                let head_chirho =
                    constructor_chirho(format!("({})", ",".repeat(elements_chirho.len() - 1)));
                let last_chirho = elements_chirho.pop().expect("nonempty tuple");
                (
                    elements_chirho
                        .into_iter()
                        .fold(head_chirho, application_chirho),
                    last_chirho,
                )
            }
            _ => return Err(ETA_SHAPE_ERROR_CHIRHO),
        };
        if !matches!(argument_chirho.unannotated_chirho(), TypeChirho::VarChirho(name_chirho)
            if name_chirho.text_chirho() == variable_chirho.text_chirho())
        {
            return Err(ETA_SHAPE_ERROR_CHIRHO);
        }
        residual_chirho = prefix_chirho;
    }
    let removed_names_chirho = removed_chirho
        .iter()
        .map(|variable_chirho| variable_chirho.text_chirho())
        .collect();
    if mentions_removed_chirho(
        &residual_chirho,
        &removed_names_chirho,
        &mut budget_chirho,
        128,
    )? {
        return Err("cannot eta-reduce: a removed parameter occurs in the representation prefix");
    }
    Ok(residual_chirho)
}

fn visit_chirho(budget_chirho: &mut usize, depth_chirho: usize) -> Result<usize, &'static str> {
    *budget_chirho = budget_chirho.checked_sub(1).ok_or(ETA_WORK_LIMIT_CHIRHO)?;
    depth_chirho.checked_sub(1).ok_or(ETA_WORK_LIMIT_CHIRHO)
}

fn mentions_removed_chirho(
    ty_chirho: &TypeChirho,
    names_chirho: &HashSet<&str>,
    budget_chirho: &mut usize,
    depth_chirho: usize,
) -> Result<bool, &'static str> {
    let depth_chirho = visit_chirho(budget_chirho, depth_chirho)?;
    let children_chirho: Vec<&TypeChirho> = match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            return Ok(names_chirho.contains(name_chirho.text_chirho()));
        }
        TypeChirho::ConChirho(_)
        | TypeChirho::PromotedConChirho { .. }
        | TypeChirho::LitChirho { .. }
        | TypeChirho::WildcardChirho { .. } => return Ok(false),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }
        | TypeChirho::KindAppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => vec![fun_chirho, arg_chirho],
        TypeChirho::KindAnnotChirho {
            type_chirho,
            kind_chirho,
            ..
        } => vec![type_chirho, kind_chirho],
        TypeChirho::ParenChirho { inner_chirho, .. } => vec![inner_chirho],
        TypeChirho::ListChirho { element_chirho, .. } => vec![element_chirho],
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                if mentions_removed_chirho(
                    element_chirho,
                    names_chirho,
                    budget_chirho,
                    depth_chirho,
                )? {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            let mut children_chirho = vec![arg_chirho.as_ref(), result_chirho.as_ref()];
            if let Some(MultiplicityChirho::ExpressionChirho(ty_chirho)) = mult_chirho {
                children_chirho.push(ty_chirho);
            }
            children_chirho
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
            let Some(visible_chirho) =
                binders_chirho(vars_chirho, names_chirho, budget_chirho, depth_chirho)?
            else {
                return Ok(true);
            };
            return mentions_removed_chirho(
                body_chirho,
                &visible_chirho,
                budget_chirho,
                depth_chirho,
            );
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for predicate_chirho in context_chirho {
                if constraint_mentions_chirho(
                    predicate_chirho,
                    names_chirho,
                    budget_chirho,
                    depth_chirho,
                )? {
                    return Ok(true);
                }
            }
            vec![body_chirho]
        }
    };
    for child_chirho in children_chirho {
        if mentions_removed_chirho(child_chirho, names_chirho, budget_chirho, depth_chirho)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn binders_chirho<'a>(
    vars_chirho: &[TyVarChirho],
    names_chirho: &HashSet<&'a str>,
    budget_chirho: &mut usize,
    depth_chirho: usize,
) -> Result<Option<HashSet<&'a str>>, &'static str> {
    let mut visible_chirho = names_chirho.clone();
    for binder_chirho in vars_chirho {
        visit_chirho(budget_chirho, depth_chirho)?;
        if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho
            && kind_mentions_chirho(kind_chirho, &visible_chirho, budget_chirho, depth_chirho)?
        {
            return Ok(None);
        }
        visible_chirho.remove(binder_chirho.text_chirho());
    }
    Ok(Some(visible_chirho))
}

fn kind_mentions_chirho(
    kind_chirho: &AstKindChirho,
    names_chirho: &HashSet<&str>,
    budget_chirho: &mut usize,
    depth_chirho: usize,
) -> Result<bool, &'static str> {
    let depth_chirho = visit_chirho(budget_chirho, depth_chirho)?;
    match kind_chirho {
        AstKindChirho::StarChirho
        | AstKindChirho::ConstraintChirho
        | AstKindChirho::ConChirho(_) => Ok(false),
        AstKindChirho::VarChirho(name_chirho) => Ok(names_chirho.contains(name_chirho.as_str())),
        AstKindChirho::TypeSyntaxChirho(ty_chirho) => {
            mentions_removed_chirho(ty_chirho, names_chirho, budget_chirho, depth_chirho)
        }
        AstKindChirho::ArrowChirho(left_chirho, right_chirho)
        | AstKindChirho::AppChirho(left_chirho, right_chirho)
        | AstKindChirho::KindAppChirho(left_chirho, right_chirho)
        | AstKindChirho::KindAnnotChirho {
            type_chirho: left_chirho,
            kind_chirho: right_chirho,
            ..
        } => Ok(
            kind_mentions_chirho(left_chirho, names_chirho, budget_chirho, depth_chirho)?
                || kind_mentions_chirho(right_chirho, names_chirho, budget_chirho, depth_chirho)?,
        ),
        AstKindChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        }
        | AstKindChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let Some(visible_chirho) =
                binders_chirho(vars_chirho, names_chirho, budget_chirho, depth_chirho)?
            else {
                return Ok(true);
            };
            kind_mentions_chirho(body_chirho, &visible_chirho, budget_chirho, depth_chirho)
        }
    }
}

fn constraint_mentions_chirho(
    predicate_chirho: &ConstraintChirho,
    names_chirho: &HashSet<&str>,
    budget_chirho: &mut usize,
    depth_chirho: usize,
) -> Result<bool, &'static str> {
    let depth_chirho = visit_chirho(budget_chirho, depth_chirho)?;
    match predicate_chirho {
        ConstraintChirho::ClassChirho { args_chirho, .. } => {
            for argument_chirho in args_chirho {
                if mentions_removed_chirho(
                    argument_chirho,
                    names_chirho,
                    budget_chirho,
                    depth_chirho,
                )? {
                    return Ok(true);
                }
            }
        }
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => {
            let Some(visible_chirho) =
                binders_chirho(vars_chirho, names_chirho, budget_chirho, depth_chirho)?
            else {
                return Ok(true);
            };
            for given_chirho in context_chirho
                .iter()
                .chain(std::iter::once(body_chirho.as_ref()))
            {
                if constraint_mentions_chirho(
                    given_chirho,
                    &visible_chirho,
                    budget_chirho,
                    depth_chirho,
                )? {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

pub(crate) fn instance_chirho(
    class_chirho: &NameChirho,
    written_chirho: &[&TypeChirho],
    target_chirho: TypeChirho,
    representation_chirho: TypeChirho,
    span_chirho: SpanChirho,
) -> DeclChirho {
    let mut head_chirho: Vec<_> = written_chirho
        .iter()
        .map(|argument_chirho| (*argument_chirho).clone())
        .collect();
    let mut context_arguments_chirho = head_chirho.clone();
    head_chirho.push(target_chirho);
    context_arguments_chirho.push(representation_chirho);
    DeclChirho::InstanceDeclChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: class_chirho.clone(),
            args_chirho: context_arguments_chirho,
            span_chirho,
        }],
        class_chirho: class_chirho.clone(),
        types_chirho: head_chirho,
        methods_chirho: Vec::new(),
        assoc_tf_instances_chirho: Vec::new(),
        span_chirho,
    }
}
