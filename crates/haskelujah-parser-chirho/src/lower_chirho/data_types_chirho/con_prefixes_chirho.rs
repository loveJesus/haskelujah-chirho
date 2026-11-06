// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering of an H98 constructor's `forall vs. ctx =>` (row 694, #31811).
//!
//! The CST keeps the prefix as flat tokens before the constructor's name: `forall`, the
//! binders, the `.`, and, when there is a context, its tokens and `=>`. The binders become the
//! constructor's existential variables and the context lowers through the flat type path into
//! constraints. A binder that is not a plain variable, a context token outside plain type
//! syntax, or a context variable the lowering loses makes the whole prefix unread, so role
//! inference never reads a constructor it could not lower as one without a context.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use std::collections::HashSet;

use haskelujah_ast_chirho::decl_chirho::ConPrefixChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenTokenChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    /// The prefix of a constructor whose children start with `forall`: its binders up to the
    /// `.`, then the context up to `=>`, if there is one.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn con_prefix_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> ConPrefixChirho {
        let Some(dot_chirho) = (1..children_chirho.len()).find(|index_chirho| {
            token_at_chirho(children_chirho, *index_chirho).is_some_and(|token_chirho| {
                token_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                    && token_chirho.text_chirho() == "."
            })
        }) else {
            return ConPrefixChirho::unread_quantification_chirho();
        };
        let mut existential_vars_chirho = Vec::new();
        for index_chirho in 1..dot_chirho {
            let Some(token_chirho) = token_at_chirho(children_chirho, index_chirho)
                .filter(|token_chirho| token_chirho.kind_chirho() == TokenKindChirho::VarIdChirho)
            else {
                return ConPrefixChirho::unread_quantification_chirho();
            };
            let child_chirho = &children_chirho[index_chirho];
            let name_span_chirho =
                self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
            existential_vars_chirho
                .push(self.name_from_token_chirho(token_chirho, name_span_chirho));
        }
        let arrow_chirho = (dot_chirho + 1..children_chirho.len()).find(|index_chirho| {
            token_at_chirho(children_chirho, *index_chirho).is_some_and(|token_chirho| {
                token_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho
            })
        });
        let context_chirho = match arrow_chirho {
            None => Vec::new(),
            Some(arrow_chirho) => {
                match self.prefix_context_chirho(
                    &children_chirho[dot_chirho + 1..arrow_chirho],
                    span_chirho,
                ) {
                    Some(context_chirho) => context_chirho,
                    None => return ConPrefixChirho::unread_quantification_chirho(),
                }
            }
        };
        ConPrefixChirho {
            existential_vars_chirho,
            context_chirho,
            unread_chirho: false,
        }
    }

    /// The constraints a prefix's context region writes; `None` when the region holds syntax
    /// this path does not read, or the lowering loses one of the variables it writes.
    fn prefix_context_chirho(
        &self,
        region_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> Option<Vec<ConstraintChirho>> {
        let mut written_chirho = HashSet::new();
        for index_chirho in 0..region_chirho.len() {
            let token_chirho = token_at_chirho(region_chirho, index_chirho)?;
            match token_chirho.kind_chirho() {
                TokenKindChirho::VarIdChirho => {
                    written_chirho.insert(token_chirho.text_chirho().to_string());
                }
                TokenKindChirho::ConIdChirho
                | TokenKindChirho::QualifiedConIdChirho
                | TokenKindChirho::LeftParenChirho
                | TokenKindChirho::RightParenChirho
                | TokenKindChirho::LeftBracketChirho
                | TokenKindChirho::RightBracketChirho
                | TokenKindChirho::CommaChirho
                | TokenKindChirho::TildeChirho
                | TokenKindChirho::VarSymChirho
                | TokenKindChirho::ConSymChirho => {}
                _ => return None,
            }
        }
        if region_chirho.is_empty() {
            return None;
        }
        let flat_chirho: Vec<&ChildChirho<'_>> = region_chirho.iter().collect();
        let context_ty_chirho = self.type_from_flat_children_chirho(&flat_chirho, span_chirho);
        let constraints_chirho = Self::type_to_constraints_chirho(&context_ty_chirho, span_chirho);
        let mut lowered_chirho = HashSet::new();
        for constraint_chirho in &constraints_chirho {
            constraint_variables_chirho(constraint_chirho, &mut lowered_chirho);
        }
        written_chirho
            .is_subset(&lowered_chirho)
            .then_some(constraints_chirho)
    }
}

/// The token at `index_chirho`, if that child is one.
fn token_at_chirho<'a>(
    children_chirho: &[ChildChirho<'a>],
    index_chirho: usize,
) -> Option<&'a GreenTokenChirho> {
    match children_chirho.get(index_chirho)?.element_chirho {
        GreenElementChirho::TokenChirho(token_chirho) => Some(token_chirho),
        GreenElementChirho::NodeChirho(_) => None,
    }
}

/// Every type variable `constraint_chirho` names.
fn constraint_variables_chirho(
    constraint_chirho: &ConstraintChirho,
    out_chirho: &mut HashSet<String>,
) {
    match constraint_chirho {
        ConstraintChirho::ClassChirho { args_chirho, .. } => {
            for argument_chirho in args_chirho {
                type_variables_chirho(argument_chirho, out_chirho);
            }
        }
        ConstraintChirho::QuantifiedChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for premise_chirho in context_chirho {
                constraint_variables_chirho(premise_chirho, out_chirho);
            }
            constraint_variables_chirho(body_chirho, out_chirho);
        }
    }
}

/// Every type variable `type_chirho` names.
fn type_variables_chirho(type_chirho: &TypeChirho, out_chirho: &mut HashSet<String>) {
    match type_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            out_chirho.insert(name_chirho.text_chirho().to_string());
        }
        TypeChirho::ConChirho(_)
        | TypeChirho::PromotedConChirho { .. }
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => {}
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            type_variables_chirho(fun_chirho, out_chirho);
            type_variables_chirho(arg_chirho, out_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            type_variables_chirho(arg_chirho, out_chirho);
            type_variables_chirho(result_chirho, out_chirho);
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                type_variables_chirho(element_chirho, out_chirho);
            }
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            type_variables_chirho(element_chirho, out_chirho)
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            type_variables_chirho(inner_chirho, out_chirho)
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for constraint_chirho in context_chirho {
                constraint_variables_chirho(constraint_chirho, out_chirho);
            }
            type_variables_chirho(body_chirho, out_chirho);
        }
        TypeChirho::ForallChirho { body_chirho, .. }
        | TypeChirho::RequiredForallChirho { body_chirho, .. } => {
            type_variables_chirho(body_chirho, out_chirho)
        }
    }
}
