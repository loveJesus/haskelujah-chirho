// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type operators retain their lexical namespace when normalized to application.
//! Workflow: spec-chirho/workflows-chirho/language-features-chirho/rank-n-visible-type-application-chirho.md

use super::{AssocChirho, ChildChirho, LowerCtxChirho, is_type_kind_chirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

struct TypeOperatorChirho {
    ty_chirho: TypeChirho,
    precedence_chirho: u8,
    associativity_chirho: AssocChirho,
}

#[derive(Default)]
struct TypeChainChirho {
    operands_chirho: Vec<TypeChirho>,
    operators_chirho: Vec<TypeOperatorChirho>,
}

impl LowerCtxChirho {
    pub(super) fn lower_infix_type_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> TypeChirho {
        let mut chain_chirho = TypeChainChirho::default();
        self.collect_infix_type_chain_chirho(
            node_chirho,
            base_chirho,
            span_chirho,
            &mut chain_chirho,
        );
        if chain_chirho.operands_chirho.len() != chain_chirho.operators_chirho.len() + 1 {
            return self.placeholder_type_chirho();
        }
        let mut operands_chirho = chain_chirho.operands_chirho.into_iter();
        let mut values_chirho = vec![
            operands_chirho
                .next()
                .expect("one more operand than operators"),
        ];
        let mut pending_chirho: Vec<TypeOperatorChirho> = Vec::new();
        // Each operator/operand is pushed and reduced once: linear work, no
        // recursive suffix copying. Parenthesized subexpressions are atoms.
        for (operator_chirho, right_chirho) in chain_chirho
            .operators_chirho
            .into_iter()
            .zip(operands_chirho)
        {
            while pending_chirho.last().is_some_and(|previous_chirho| {
                previous_chirho.precedence_chirho > operator_chirho.precedence_chirho
                    || (previous_chirho.precedence_chirho == operator_chirho.precedence_chirho
                        && previous_chirho.associativity_chirho == AssocChirho::LeftChirho)
            }) {
                reduce_type_operator_chirho(
                    &mut values_chirho,
                    pending_chirho.pop().unwrap(),
                    span_chirho,
                );
            }
            pending_chirho.push(operator_chirho);
            values_chirho.push(right_chirho);
        }
        while let Some(operator_chirho) = pending_chirho.pop() {
            reduce_type_operator_chirho(&mut values_chirho, operator_chirho, span_chirho);
        }
        values_chirho.pop().expect("a type chain has a result")
    }

    fn collect_infix_type_chain_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
        chain_chirho: &mut TypeChainChirho,
    ) {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let operator_chirho =
            children_chirho
                .iter()
                .enumerate()
                .find_map(|(index_chirho, child_chirho)| {
                    let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho
                    else {
                        return None;
                    };
                    let kind_chirho = token_chirho.kind_chirho();
                    if !matches!(
                        kind_chirho,
                        TokenKindChirho::VarIdChirho
                            | TokenKindChirho::ConIdChirho
                            | TokenKindChirho::QualifiedConIdChirho
                            | TokenKindChirho::VarSymChirho
                            | TokenKindChirho::ConSymChirho
                            | TokenKindChirho::QualifiedVarSymChirho
                            | TokenKindChirho::QualifiedConSymChirho
                            | TokenKindChirho::TildeChirho
                    ) {
                        return None;
                    }
                    let name_chirho = self.name_from_token_chirho(
                        token_chirho,
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                    );
                    // A backticked lowercase identifier is a quantified type variable.
                    // Symbols, including VarSym (`~>`), still name type constructors.
                    let (precedence_chirho, associativity_chirho) =
                        self.operator_fixity_chirho(name_chirho.text_chirho());
                    let promoted_start_chirho = index_chirho
                        .checked_sub(1)
                        .and_then(|previous_chirho| children_chirho.get(previous_chirho))
                        .filter(|previous_chirho| {
                            matches!(previous_chirho.element_chirho,
                            GreenElementChirho::TokenChirho(token_chirho)
                                if token_chirho.kind_chirho() == TokenKindChirho::TickChirho)
                        })
                        .map(|previous_chirho| previous_chirho.start_chirho);
                    let ty_chirho = if let Some(start_chirho) = promoted_start_chirho {
                        TypeChirho::PromotedConChirho {
                            name_chirho,
                            span_chirho: self.span_chirho(start_chirho, child_chirho.end_chirho),
                        }
                    } else if kind_chirho == TokenKindChirho::VarIdChirho {
                        TypeChirho::VarChirho(name_chirho)
                    } else {
                        TypeChirho::ConChirho(name_chirho)
                    };
                    Some((
                        index_chirho,
                        TypeOperatorChirho {
                            ty_chirho,
                            precedence_chirho,
                            associativity_chirho,
                        },
                    ))
                });
        let Some((index_chirho, operator_chirho)) = operator_chirho else {
            chain_chirho
                .operands_chirho
                .push(self.placeholder_type_chirho());
            return;
        };
        self.collect_infix_type_segment_chirho(
            &children_chirho[..index_chirho],
            span_chirho,
            chain_chirho,
        );
        chain_chirho.operators_chirho.push(operator_chirho);
        self.collect_infix_type_segment_chirho(
            &children_chirho[index_chirho + 1..],
            span_chirho,
            chain_chirho,
        );
    }

    fn collect_infix_type_segment_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
        chain_chirho: &mut TypeChainChirho,
    ) {
        let mut nodes_chirho = children_chirho.iter().filter_map(|child_chirho| {
            if let GreenElementChirho::NodeChirho(node_chirho) = child_chirho.element_chirho
                && is_type_kind_chirho(node_chirho.kind_chirho())
            {
                Some((node_chirho, child_chirho.start_chirho))
            } else {
                None
            }
        });
        if let Some((node_chirho, base_chirho)) = nodes_chirho.next()
            && node_chirho.kind_chirho() == SyntaxKindChirho::InfixTypeChirho
            && nodes_chirho.next().is_none()
        {
            self.collect_infix_type_chain_chirho(
                node_chirho,
                base_chirho,
                span_chirho,
                chain_chirho,
            );
            return;
        }
        chain_chirho
            .operands_chirho
            .push(self.lower_type_app_segment_chirho(children_chirho, span_chirho));
    }

    fn lower_type_app_segment_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> TypeChirho {
        let mut types_chirho = children_chirho
            .iter()
            .filter_map(|child_chirho| match child_chirho.element_chirho {
                GreenElementChirho::NodeChirho(node_chirho)
                    if is_type_kind_chirho(node_chirho.kind_chirho()) =>
                {
                    Some(self.lower_type_from_child_chirho(child_chirho))
                }
                _ => None,
            });
        let Some(first_chirho) = types_chirho.next() else {
            return self.placeholder_type_chirho();
        };
        types_chirho.fold(first_chirho, |fun_chirho, arg_chirho| {
            TypeChirho::AppChirho {
                fun_chirho: Box::new(fun_chirho),
                arg_chirho: Box::new(arg_chirho),
                span_chirho,
            }
        })
    }
}

fn reduce_type_operator_chirho(
    values_chirho: &mut Vec<TypeChirho>,
    operator_chirho: TypeOperatorChirho,
    span_chirho: SpanChirho,
) {
    let right_chirho = values_chirho
        .pop()
        .expect("an infix operator has a right operand");
    let left_chirho = values_chirho
        .pop()
        .expect("an infix operator has a left operand");
    values_chirho.push(binary_type_application_chirho(
        operator_chirho.ty_chirho,
        left_chirho,
        right_chirho,
        span_chirho,
    ));
}

pub(super) fn binary_type_application_chirho(
    operator_chirho: TypeChirho,
    left_chirho: TypeChirho,
    right_chirho: TypeChirho,
    span_chirho: SpanChirho,
) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(operator_chirho),
            arg_chirho: Box::new(left_chirho),
            span_chirho,
        }),
        arg_chirho: Box::new(right_chirho),
        span_chirho,
    }
}
