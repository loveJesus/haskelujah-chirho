// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Keep signature binder order independent of type normalization and allocation.
//! Workflow: spec-chirho/workflows-chirho/language-features-chirho/rank-n-visible-type-application-chirho.md

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::ty_chirho::TyVarChirho;

pub(super) fn ordered_scheme_vars_chirho(
    signature_chirho: &TypeChirho,
    var_map_chirho: &HashMap<String, TyVarChirho>,
    mut explicit_vars_chirho: Vec<TyVarChirho>,
    free_vars_chirho: &[TyVarChirho],
) -> Vec<TyVarChirho> {
    let free_vars_chirho: HashSet<_> = free_vars_chirho.iter().copied().collect();
    let mut seen_chirho: HashSet<_> = explicit_vars_chirho.iter().copied().collect();
    let mut inventory_chirho = SignatureOccurrencesChirho::default();
    inventory_chirho.type_chirho(signature_chirho);
    // Stable sorting retains traversal order for synthetic/equal spans. Real
    // occurrence spans recover `a, op, b` from the normalized `op a b` spine.
    inventory_chirho
        .occurrences_chirho
        .sort_by_key(|(_, span_chirho)| span_chirho.start_chirho());
    for (name_chirho, _) in inventory_chirho.occurrences_chirho {
        if let Some(var_chirho) = var_map_chirho.get(name_chirho)
            && free_vars_chirho.contains(var_chirho)
            && seen_chirho.insert(*var_chirho)
        {
            explicit_vars_chirho.push(*var_chirho);
        }
    }
    // Expansion-generated variables have no source occurrence. Preserve the
    // deterministic ID fallback for those only; never iterate a HashMap into a
    // scheme's externally observable type-argument order.
    let mut generated_vars_chirho: Vec<_> = var_map_chirho
        .values()
        .copied()
        .filter(|var_chirho| {
            free_vars_chirho.contains(var_chirho) && !seen_chirho.contains(var_chirho)
        })
        .collect();
    generated_vars_chirho.sort_unstable();
    generated_vars_chirho.dedup();
    explicit_vars_chirho.extend(generated_vars_chirho);
    explicit_vars_chirho
}

#[derive(Default)]
struct SignatureOccurrencesChirho<'a> {
    bound_chirho: HashMap<&'a str, usize>,
    occurrences_chirho: Vec<(&'a str, SpanChirho)>,
}

impl<'a> SignatureOccurrencesChirho<'a> {
    fn name_chirho(&mut self, name_chirho: &'a NameChirho) {
        if !self.bound_chirho.contains_key(name_chirho.text_chirho()) {
            self.occurrences_chirho
                .push((name_chirho.text_chirho(), name_chirho.span_chirho()));
        }
    }

    fn enter_chirho(&mut self, vars_chirho: &'a [AstTyVarChirho]) {
        for var_chirho in vars_chirho {
            *self
                .bound_chirho
                .entry(var_chirho.text_chirho())
                .or_default() += 1;
        }
    }

    fn leave_chirho(&mut self, vars_chirho: &'a [AstTyVarChirho]) {
        for var_chirho in vars_chirho {
            let name_chirho = var_chirho.text_chirho();
            if let Some(count_chirho) = self.bound_chirho.get_mut(name_chirho) {
                *count_chirho -= 1;
                if *count_chirho == 0 {
                    self.bound_chirho.remove(name_chirho);
                }
            }
        }
    }

    fn type_chirho(&mut self, ty_chirho: &'a TypeChirho) {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => self.name_chirho(name_chirho),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindAppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                self.type_chirho(fun_chirho);
                self.type_chirho(arg_chirho);
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                self.type_chirho(arg_chirho);
                if let Some(MultiplicityChirho::ExpressionChirho(expression_chirho)) = mult_chirho {
                    self.type_chirho(expression_chirho);
                }
                self.type_chirho(result_chirho);
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.type_chirho(element_chirho);
                }
            }
            TypeChirho::ListChirho { element_chirho, .. }
            | TypeChirho::ParenChirho {
                inner_chirho: element_chirho,
                ..
            } => self.type_chirho(element_chirho),
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    self.constraint_chirho(constraint_chirho);
                }
                self.type_chirho(body_chirho);
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
                self.enter_chirho(vars_chirho);
                self.type_chirho(body_chirho);
                self.leave_chirho(vars_chirho);
            }
            TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => {}
        }
    }

    fn constraint_chirho(&mut self, constraint_chirho: &'a ConstraintChirho) {
        match constraint_chirho {
            ConstraintChirho::ClassChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.type_chirho(arg_chirho);
                }
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                self.enter_chirho(vars_chirho);
                for given_chirho in context_chirho {
                    self.constraint_chirho(given_chirho);
                }
                self.constraint_chirho(body_chirho);
                self.leave_chirho(vars_chirho);
            }
        }
    }
}
