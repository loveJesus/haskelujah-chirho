// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class/instance contexts use the same type syntax as qualified signatures.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    ChildChirho, ConstraintChirho, GreenElementChirho, GreenTokenChirho, LowerCtxChirho,
    NameChirho, RawNameChirho, SpanChirho, TokenKindChirho, TypeChirho,
};

impl LowerCtxChirho {
    pub(super) fn build_instance_context_chirho(
        &self,
        tokens_chirho: &[(&GreenTokenChirho, SpanChirho)],
    ) -> Vec<ConstraintChirho> {
        let Some((_, first_span_chirho)) = tokens_chirho.first() else {
            return Vec::new();
        };
        let span_chirho = first_span_chirho
            .merge_chirho(tokens_chirho.last().expect("nonempty context").1)
            .unwrap_or(*first_span_chirho);
        let context_chirho = self.lower_type_from_token_slice_chirho(tokens_chirho, span_chirho);
        Self::type_to_constraints_chirho(&context_chirho, span_chirho)
    }

    /// Context arrows delimit the constructor body with or without an explicit
    /// forall. The existing Ordinary/Record AST variants do not retain evidence
    /// contexts yet; this boundary must still never count a constraint as a field.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn constructor_body_start_chirho(children_chirho: &[ChildChirho<'_>]) -> usize {
        if let Some(index_chirho) = children_chirho.iter().position(|child_chirho| {
            matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho)
                if token_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho)
        }) {
            return index_chirho + 1;
        }
        if matches!(children_chirho.first().map(|child_chirho| child_chirho.element_chirho),
            Some(GreenElementChirho::TokenChirho(token_chirho))
                if token_chirho.kind_chirho() == TokenKindChirho::ForallKeywordChirho)
        {
            return children_chirho.iter().position(|child_chirho| {
                matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
                        && token_chirho.text_chirho() == ".")
            }).map_or(0, |index_chirho| index_chirho + 1);
        }
        0
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
            // In a context, (?label :: payload) binds evidence of a TYPE,
            // not a type variable with a KIND annotation.
            TypeChirho::KindAnnotChirho {
                type_chirho,
                kind_chirho,
                span_chirho,
            } if matches!(type_chirho.unannotated_chirho(), TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho().starts_with('?')) =>
            {
                let TypeChirho::VarChirho(name_chirho) = type_chirho.unannotated_chirho() else {
                    unreachable!("implicit parameter head was checked");
                };
                vec![ConstraintChirho::ClassChirho {
                    class_chirho: name_chirho.clone(),
                    args_chirho: vec![kind_chirho.as_ref().clone()],
                    span_chirho: *span_chirho,
                }]
            }
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
            // Bare constructor or predicate variable: `Typeable` or `c`.
            // Both are zero-argument predicates, never a synthetic `? c`.
            TypeChirho::ConChirho(name_chirho) | TypeChirho::VarChirho(name_chirho) => {
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
}
