// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A pattern's existential binders belong to its arguments, never its result.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};

use super::DeclarationErrorChirho;

/// Read the two retained context roles and the definition's actual arity.
/// workflow: compiler-pipeline-chirho/declaration-validity-chirho
pub(super) fn check_pattern_synonym_signatures_chirho(
    module_chirho: &ModuleChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let mut arities_chirho = HashMap::new();
    for declaration_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::PatSynDeclChirho {
            name_chirho,
            args_chirho,
            ..
        } = declaration_chirho
        {
            // Naming reports duplicate definitions before declaration validity.
            arities_chirho
                .entry(name_chirho.text_chirho())
                .or_insert(args_chirho.len());
        }
    }
    let mut work_chirho = 1_000_000usize;
    for declaration_chirho in &module_chirho.decls_chirho {
        let DeclChirho::PatSynSigDeclChirho {
            names_chirho,
            ty_chirho,
            span_chirho,
        } = declaration_chirho
        else {
            continue;
        };
        for name_chirho in names_chirho {
            let Some(&arity_chirho) = arities_chirho.get(name_chirho.text_chirho()) else {
                // A signature without its definition supplies no argument count.
                continue;
            };
            let finding_chirho = match pattern_result_chirho(
                ty_chirho,
                arity_chirho,
                &mut work_chirho,
            ) {
                Ok(Some(result_chirho)) => {
                    let mut scope_chirho = ResultScopeChirho {
                        existentials_chirho: result_chirho.existentials_chirho,
                        shadowed_chirho: HashMap::new(),
                        work_chirho: &mut work_chirho,
                    };
                    match scope_chirho.walk_type_chirho(result_chirho.ty_chirho, 0) {
                        Ok(Some(variable_chirho)) => Some(format!(
                            "pattern synonym `{}` has existential variable `{}` in its result type",
                            name_chirho.text_chirho(),
                            variable_chirho.text_chirho(),
                        )),
                        Ok(None) => None,
                        Err(()) => Some(RESOURCE_MESSAGE_CHIRHO.to_string()),
                    }
                }
                Ok(None) => None,
                Err(message_chirho) => Some(message_chirho.to_string()),
            };
            if let Some(message_chirho) = finding_chirho {
                errors_chirho.push(DeclarationErrorChirho {
                    ghc_code_chirho: None,
                    message_chirho,
                    span_chirho: *span_chirho,
                    related_span_chirho: None,
                });
            }
            if work_chirho == 0 {
                return;
            }
        }
    }
}

const RESOURCE_MESSAGE_CHIRHO: &str = "pattern synonym result validity resource bound exhausted";

fn unparenthesized_chirho<'source_chirho>(
    mut ty_chirho: &'source_chirho TypeChirho,
    work_chirho: &mut usize,
) -> Result<&'source_chirho TypeChirho, &'static str> {
    loop {
        if *work_chirho == 0 {
            return Err(RESOURCE_MESSAGE_CHIRHO);
        }
        *work_chirho -= 1;
        if let TypeChirho::ParenChirho { inner_chirho, .. } = ty_chirho {
            ty_chirho = inner_chirho;
        } else {
            return Ok(ty_chirho);
        }
    }
}

fn pattern_result_chirho<'source_chirho>(
    ty_chirho: &'source_chirho TypeChirho,
    arity_chirho: usize,
    work_chirho: &mut usize,
) -> Result<Option<PatternResultChirho<'source_chirho>>, &'static str> {
    let mut ty_chirho = unparenthesized_chirho(ty_chirho, work_chirho)?;
    // Leading quantifiers are universal. An explicit required context, even (),
    // is the boundary after which the retained invisible quantifiers are existential.
    while let TypeChirho::ForallChirho { body_chirho, .. } = ty_chirho {
        ty_chirho = unparenthesized_chirho(body_chirho, work_chirho)?;
    }
    let TypeChirho::QualChirho { body_chirho, .. } = ty_chirho else {
        return Ok(None);
    };
    ty_chirho = unparenthesized_chirho(body_chirho, work_chirho)?;
    let mut existentials_chirho = HashSet::new();
    while let TypeChirho::ForallChirho {
        vars_chirho,
        body_chirho,
        ..
    } = ty_chirho
    {
        for variable_chirho in vars_chirho {
            existentials_chirho.insert(variable_chirho.name_chirho.text_chirho());
        }
        ty_chirho = unparenthesized_chirho(body_chirho, work_chirho)?;
    }
    if existentials_chirho.is_empty() {
        return Ok(None);
    }
    if let TypeChirho::QualChirho { body_chirho, .. } = ty_chirho {
        ty_chirho = unparenthesized_chirho(body_chirho, work_chirho)?;
    }
    for _index_chirho in 0..arity_chirho {
        let TypeChirho::FunChirho { result_chirho, .. } = ty_chirho else {
            return Err("cannot determine pattern synonym result after its declared arguments");
        };
        ty_chirho = unparenthesized_chirho(result_chirho, work_chirho)?;
    }
    // Any remaining arrows belong to the result and must be walked, not peeled.
    Ok(Some(PatternResultChirho {
        existentials_chirho,
        ty_chirho,
    }))
}

struct PatternResultChirho<'source_chirho> {
    existentials_chirho: HashSet<&'source_chirho str>,
    ty_chirho: &'source_chirho TypeChirho,
}

struct ResultScopeChirho<'source_chirho, 'budget_chirho> {
    existentials_chirho: HashSet<&'source_chirho str>,
    shadowed_chirho: HashMap<&'source_chirho str, usize>,
    work_chirho: &'budget_chirho mut usize,
}

impl<'source_chirho> ResultScopeChirho<'source_chirho, '_> {
    fn spend_chirho(&mut self, depth_chirho: usize) -> Result<(), ()> {
        if *self.work_chirho == 0 || depth_chirho > 128 {
            return Err(());
        }
        *self.work_chirho -= 1;
        Ok(())
    }

    fn variable_chirho(
        &self,
        name_chirho: &'source_chirho NameChirho,
    ) -> Option<&'source_chirho NameChirho> {
        (self.existentials_chirho.contains(name_chirho.text_chirho())
            && !self.shadowed_chirho.contains_key(name_chirho.text_chirho()))
        .then_some(name_chirho)
    }

    fn with_binders_chirho(
        &mut self,
        vars_chirho: &'source_chirho [TyVarChirho],
        depth_chirho: usize,
        body_chirho: impl FnOnce(&mut Self) -> Result<Option<&'source_chirho NameChirho>, ()>,
    ) -> Result<Option<&'source_chirho NameChirho>, ()> {
        for variable_chirho in vars_chirho {
            // A binder's annotation sees the preceding scope, then that binder
            // shadows an outer existential in the later telescope and body.
            if let Some(kind_chirho) = &variable_chirho.kind_annotation_chirho
                && let Some(escape_chirho) = self.walk_type_chirho(kind_chirho, depth_chirho + 1)?
            {
                return Ok(Some(escape_chirho));
            }
            *self
                .shadowed_chirho
                .entry(variable_chirho.name_chirho.text_chirho())
                .or_default() += 1;
        }
        let result_chirho = body_chirho(self);
        for variable_chirho in vars_chirho {
            let name_chirho = variable_chirho.name_chirho.text_chirho();
            if let Some(count_chirho) = self.shadowed_chirho.get_mut(name_chirho) {
                *count_chirho -= 1;
                if *count_chirho == 0 {
                    self.shadowed_chirho.remove(name_chirho);
                }
            }
        }
        result_chirho
    }

    fn walk_type_chirho(
        &mut self,
        ty_chirho: &'source_chirho TypeChirho,
        depth_chirho: usize,
    ) -> Result<Option<&'source_chirho NameChirho>, ()> {
        self.spend_chirho(depth_chirho)?;
        let next_chirho = depth_chirho + 1;
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => Ok(self.variable_chirho(name_chirho)),
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
            } => self.walk_pair_chirho(fun_chirho, arg_chirho, next_chirho),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                mult_chirho,
                ..
            } => {
                if let Some(MultiplicityChirho::MultVarChirho(name_chirho)) = mult_chirho
                    && let Some(escape_chirho) = self.variable_chirho(name_chirho)
                {
                    return Ok(Some(escape_chirho));
                }
                self.walk_pair_chirho(arg_chirho, result_chirho, next_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    if let Some(escape_chirho) =
                        self.walk_type_chirho(element_chirho, next_chirho)?
                    {
                        return Ok(Some(escape_chirho));
                    }
                }
                Ok(None)
            }
            TypeChirho::ListChirho { element_chirho, .. }
            | TypeChirho::ParenChirho {
                inner_chirho: element_chirho,
                ..
            } => self.walk_type_chirho(element_chirho, next_chirho),
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => self.with_binders_chirho(vars_chirho, next_chirho, |scope_chirho| {
                scope_chirho.walk_type_chirho(body_chirho, next_chirho)
            }),
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    if let Some(escape_chirho) =
                        self.walk_constraint_chirho(constraint_chirho, next_chirho)?
                    {
                        return Ok(Some(escape_chirho));
                    }
                }
                self.walk_type_chirho(body_chirho, next_chirho)
            }
            TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => Ok(None),
        }
    }

    fn walk_pair_chirho(
        &mut self,
        left_chirho: &'source_chirho TypeChirho,
        right_chirho: &'source_chirho TypeChirho,
        depth_chirho: usize,
    ) -> Result<Option<&'source_chirho NameChirho>, ()> {
        if let Some(escape_chirho) = self.walk_type_chirho(left_chirho, depth_chirho)? {
            return Ok(Some(escape_chirho));
        }
        self.walk_type_chirho(right_chirho, depth_chirho)
    }

    fn walk_constraint_chirho(
        &mut self,
        constraint_chirho: &'source_chirho ConstraintChirho,
        depth_chirho: usize,
    ) -> Result<Option<&'source_chirho NameChirho>, ()> {
        self.spend_chirho(depth_chirho)?;
        let next_chirho = depth_chirho + 1;
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                args_chirho,
                kind_args_chirho,
                ..
            } => {
                for argument_chirho in kind_args_chirho.iter().chain(args_chirho) {
                    if let Some(escape_chirho) =
                        self.walk_type_chirho(argument_chirho, next_chirho)?
                    {
                        return Ok(Some(escape_chirho));
                    }
                }
                Ok(None)
            }
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => {
                if let Some(escape_chirho) =
                    self.walk_constraint_chirho(constraint_chirho, next_chirho)?
                {
                    return Ok(Some(escape_chirho));
                }
                self.walk_type_chirho(kind_chirho, next_chirho)
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => self.with_binders_chirho(vars_chirho, next_chirho, |scope_chirho| {
                for predicate_chirho in context_chirho {
                    if let Some(escape_chirho) =
                        scope_chirho.walk_constraint_chirho(predicate_chirho, next_chirho)?
                    {
                        return Ok(Some(escape_chirho));
                    }
                }
                scope_chirho.walk_constraint_chirho(body_chirho, next_chirho)
            }),
        }
    }
}
