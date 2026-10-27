// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration contexts: class superclasses, instance contexts and
//! standalone-deriving contexts. They are written in the same syntax as the
//! context of a type signature, so they are lowered by the same grammar and
//! converted by the same function. A constraint cannot mean one thing before
//! the `=>` of a signature and another before the `=>` of a declaration.
//! Workflow: language-features-chirho/flat-type-syntax-chirho.

use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenTokenChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

/// The tokens a declaration wrote between its keyword and `where`, each with
/// its source span.
pub(super) type DeclTokensChirho<'t, 'g> = &'t [(&'g GreenTokenChirho, SpanChirho)];

/// A leading `forall a b.` on an instance binds the instance's type variables;
/// it is not part of the context and not part of the head. Returns the tokens
/// after the telescope, or all of them when there is none.
///
/// `instance forall a. Eq a => C [a]` has the context `Eq a`: reading the
/// telescope as part of the context would turn it into the quantified given
/// `forall a. Eq a`, which is a different (and unsatisfiable) premise.
pub(super) fn strip_instance_forall_chirho<'t, 'g>(
    tokens_chirho: DeclTokensChirho<'t, 'g>,
) -> DeclTokensChirho<'t, 'g> {
    let starts_with_forall_chirho = tokens_chirho.first().is_some_and(|(token_chirho, _)| {
        token_chirho.kind_chirho() == TokenKindChirho::ForallKeywordChirho
    });
    if !starts_with_forall_chirho {
        return tokens_chirho;
    }
    let mut depth_chirho = 0usize;
    for (index_chirho, (token_chirho, _)) in tokens_chirho.iter().enumerate().skip(1) {
        match token_chirho.kind_chirho() {
            TokenKindChirho::LeftParenChirho
            | TokenKindChirho::LeftBraceChirho
            | TokenKindChirho::LeftBracketChirho => depth_chirho += 1,
            TokenKindChirho::RightParenChirho
            | TokenKindChirho::RightBraceChirho
            | TokenKindChirho::RightBracketChirho => depth_chirho = depth_chirho.saturating_sub(1),
            TokenKindChirho::VarSymChirho
                if depth_chirho == 0 && token_chirho.text_chirho() == "." =>
            {
                return &tokens_chirho[index_chirho + 1..];
            }
            _ => {}
        }
    }
    tokens_chirho
}

impl LowerCtxChirho {
    /// Lower the tokens written before a declaration's `=>`.
    ///
    /// Serves class declarations (the superclass context), instance
    /// declarations and standalone deriving. The whole context is lowered as
    /// ONE type by the flat type grammar, then converted: parentheses, tuples,
    /// applications, concrete arguments, equalities and quantified constraints
    /// keep the shape they were written in.
    pub(super) fn build_instance_context_chirho(
        &self,
        tokens_chirho: DeclTokensChirho<'_, '_>,
    ) -> Vec<ConstraintChirho> {
        let Some((_, first_span_chirho)) = tokens_chirho.first() else {
            return Vec::new();
        };
        let span_chirho = tokens_chirho
            .last()
            .and_then(|(_, last_span_chirho)| first_span_chirho.merge_chirho(*last_span_chirho))
            .unwrap_or(*first_span_chirho);
        // Token clones share their text storage; offsets are the tokens' own,
        // so nothing is reparsed and no position is invented.
        let elements_chirho: Vec<GreenElementChirho> = tokens_chirho
            .iter()
            .map(|(token_chirho, _)| GreenElementChirho::TokenChirho((*token_chirho).clone()))
            .collect();
        let children_chirho: Vec<ChildChirho<'_>> = elements_chirho
            .iter()
            .zip(tokens_chirho)
            .map(|(element_chirho, (_, token_span_chirho))| ChildChirho {
                element_chirho,
                start_chirho: token_span_chirho.start_chirho().as_usize_chirho(),
                end_chirho: token_span_chirho.end_chirho().as_usize_chirho(),
            })
            .collect();
        let child_refs_chirho: Vec<&ChildChirho<'_>> = children_chirho.iter().collect();
        let context_ty_chirho =
            self.type_from_flat_children_chirho(&child_refs_chirho, span_chirho);
        Self::type_to_constraints_chirho(&context_ty_chirho, span_chirho)
    }

    /// Convert a lowered type into a list of constraints for a qualified type
    /// context. Handles single constraints (`Eq a`), tupled constraints
    /// (`(Eq a, Show a)`), parenthesized single constraints (`(Eq a)`),
    /// equality constraints (`a ~ b`), and zero-arg constraints (`Typeable`).
    pub(super) fn type_to_constraints_chirho(
        ty_chirho: &TypeChirho,
        span_chirho: SpanChirho,
    ) -> Vec<ConstraintChirho> {
        match ty_chirho {
            // Tuple: multiple constraints like (Eq a, Show a)
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .flat_map(|e_chirho| Self::type_to_constraints_chirho(e_chirho, span_chirho))
                .collect(),
            // Parens: unwrap (Eq a) → Eq a
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::type_to_constraints_chirho(inner_chirho, span_chirho)
            }
            // QuantifiedConstraints: preserve `forall a. C a` as quantified
            // evidence rather than falling through to the synthetic `?` class.
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho: forall_span_chirho,
            } => {
                let (context_chirho, quantified_body_chirho) = match body_chirho.as_ref() {
                    TypeChirho::QualChirho {
                        context_chirho,
                        body_chirho,
                        ..
                    } => (context_chirho.clone(), body_chirho.as_ref()),
                    other_chirho => (Vec::new(), other_chirho),
                };
                Self::type_to_constraints_chirho(quantified_body_chirho, *forall_span_chirho)
                    .into_iter()
                    .map(
                        |body_constraint_chirho| ConstraintChirho::QuantifiedChirho {
                            vars_chirho: vars_chirho.clone(),
                            context_chirho: context_chirho.clone(),
                            body_chirho: Box::new(body_constraint_chirho),
                            span_chirho: *forall_span_chirho,
                        },
                    )
                    .collect()
            }
            // Application: Eq a, Monad m, etc. — collect class name and args
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho: app_span_chirho,
            } => {
                let (class_name_chirho, mut args_chirho) =
                    Self::collect_app_class_chirho(fun_chirho);
                args_chirho.push(arg_chirho.as_ref().clone());
                vec![ConstraintChirho::ClassChirho {
                    class_chirho: class_name_chirho,
                    args_chirho,
                    span_chirho: *app_span_chirho,
                }]
            }
            // Bare constructor: Typeable (zero-arg constraint)
            TypeChirho::ConChirho(name_chirho) => {
                vec![ConstraintChirho::ClassChirho {
                    class_chirho: name_chirho.clone(),
                    args_chirho: vec![],
                    span_chirho,
                }]
            }
            // Anything else — wrap as a single constraint with a synthetic name
            other_chirho => {
                vec![ConstraintChirho::ClassChirho {
                    class_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                        "?",
                        span_chirho,
                    )),
                    args_chirho: vec![other_chirho.clone()],
                    span_chirho,
                }]
            }
        }
    }

    /// Walk a left-nested `AppChirho` spine to collect the class name and
    /// preceding arguments. For `Monad m`, fun is `ConChirho("Monad")` and
    /// returns `("Monad", [])`. For `MonadReader r m`, fun is
    /// `AppChirho(ConChirho("MonadReader"), VarChirho("r"))` and returns
    /// `("MonadReader", [r])`.
    fn collect_app_class_chirho(ty_chirho: &TypeChirho) -> (NameChirho, Vec<TypeChirho>) {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => (name_chirho.clone(), vec![]),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let (name_chirho, mut args_chirho) = Self::collect_app_class_chirho(fun_chirho);
                args_chirho.push(arg_chirho.as_ref().clone());
                (name_chirho, args_chirho)
            }
            // Fallback: use the type as a synthetic name
            other_chirho => (
                NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    "?",
                    other_chirho.span_chirho(),
                )),
                vec![],
            ),
        }
    }
}
