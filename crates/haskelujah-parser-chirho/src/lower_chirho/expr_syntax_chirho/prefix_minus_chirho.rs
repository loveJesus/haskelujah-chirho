// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Prefix minus against the fixity table (row 555). The CST's negation node holds only
//! the operand written after `-`, so `-3 ^ 2` reached fixity resolution as the chain
//! `negate 3`, `^`, `2` and grouped as `(negate 3) ^ 2`. Haskell 2010 (§10.6) resolves
//! prefix minus as an `infixl 6` operator: a negation takes in each following operator
//! that binds tighter, so `-3 ^ 2` is `negate (3 ^ 2)` and ``-7 `mod` 3`` is
//! ``negate (7 `mod` 3)``. An operator at precedence 6 or below stays outside it. A
//! parenthesized negation lowers to `ParenChirho`, never to a bare operand, so
//! `(-3) ^ 2` keeps its grouping, as does the application `negate 3 ^ 2`.
//! workflow: compiler-pipeline-chirho/operator-fixity-chirho

use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ExprChirho, LowerCtxChirho, NameChirho, parse_integer_magnitude_chirho};

/// Prefix minus resolves as `infixl 6` (Haskell 2010, §10.6).
const PREFIX_MINUS_PRECEDENCE_CHIRHO: u8 = 6;

impl LowerCtxChirho {
    /// Preserve the one magnitude that a positive `i64` cannot hold but a direct
    /// prefix minus can: `2^63`. The AST keeps the negation node for fixity; the
    /// inner `i64::MIN` is a marker consumed by Core lowering, never a positive
    /// source literal.
    /// workflow: compiler-pipeline-chirho/operator-fixity-chirho
    pub(super) fn preserve_minimum_word_operand_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        expr_chirho: ExprChirho,
    ) -> ExprChirho {
        let ExprChirho::LitChirho(LitChirho::IntChirho(value_chirho, span_chirho, origin_chirho)) =
            expr_chirho
        else {
            return expr_chirho;
        };
        if value_chirho != 0 || single_integer_magnitude_chirho(node_chirho) != Some(1_u64 << 63) {
            return ExprChirho::LitChirho(LitChirho::IntChirho(
                value_chirho,
                span_chirho,
                origin_chirho,
            ));
        }
        ExprChirho::LitChirho(LitChirho::IntChirho(i64::MIN, span_chirho, origin_chirho))
    }

    /// Gives each bare negation in a flat operator chain the run of operators after it
    /// that bind tighter than prefix minus, with their operands, resolved as one chain
    /// under the negation. The rest of the chain is returned as it was, for the
    /// precedence split.
    pub(super) fn absorb_prefix_minus_chirho(
        &self,
        exprs_chirho: Vec<ExprChirho>,
        ops_chirho: Vec<NameChirho>,
    ) -> (Vec<ExprChirho>, Vec<NameChirho>) {
        let binds_tighter_chirho = |op_chirho: &NameChirho| {
            self.operator_fixity_chirho(op_chirho.text_chirho()).0 > PREFIX_MINUS_PRECEDENCE_CHIRHO
        };
        let mut exprs_left_chirho = exprs_chirho.into_iter();
        let mut ops_left_chirho = ops_chirho.into_iter().peekable();
        let mut kept_exprs_chirho = Vec::new();
        let mut kept_ops_chirho = Vec::new();
        while let Some(expr_chirho) = exprs_left_chirho.next() {
            kept_exprs_chirho.push(match expr_chirho {
                ExprChirho::NegChirho {
                    expr_chirho: operand_chirho,
                    span_chirho: minus_span_chirho,
                } if ops_left_chirho.peek().is_some_and(binds_tighter_chirho) => {
                    let mut chain_span_chirho = operand_chirho.span_chirho();
                    let mut chain_exprs_chirho =
                        vec![bounded_chain_operand_chirho(*operand_chirho)];
                    let mut chain_ops_chirho = Vec::new();
                    while let Some(op_chirho) = ops_left_chirho.next_if(binds_tighter_chirho) {
                        chain_ops_chirho.push(op_chirho);
                        if let Some(next_chirho) = exprs_left_chirho.next() {
                            chain_span_chirho = chain_span_chirho
                                .merge_chirho(next_chirho.span_chirho())
                                .unwrap_or(chain_span_chirho);
                            chain_exprs_chirho.push(next_chirho);
                        }
                    }
                    ExprChirho::NegChirho {
                        expr_chirho: Box::new(self.resolve_infix_precedence_chirho(
                            chain_exprs_chirho,
                            chain_ops_chirho,
                            chain_span_chirho,
                        )),
                        span_chirho: minus_span_chirho
                            .merge_chirho(chain_span_chirho)
                            .unwrap_or(minus_span_chirho),
                    }
                }
                other_chirho => other_chirho,
            });
            kept_ops_chirho.extend(ops_left_chirho.next());
        }
        kept_ops_chirho.extend(ops_left_chirho);
        (kept_exprs_chirho, kept_ops_chirho)
    }
}

/// The minimum-word marker is valid only when its prefix-minus node remains
/// direct. If fixity absorbs a tighter operator, restore the positive-overflow
/// boundary's former zero instead of leaking `i64::MIN` into that inner chain.
fn bounded_chain_operand_chirho(expr_chirho: ExprChirho) -> ExprChirho {
    match expr_chirho {
        ExprChirho::LitChirho(LitChirho::IntChirho(i64::MIN, span_chirho, origin_chirho)) => {
            ExprChirho::LitChirho(LitChirho::IntChirho(0, span_chirho, origin_chirho))
        }
        other_chirho => other_chirho,
    }
}

fn single_integer_magnitude_chirho(node_chirho: &GreenNodeChirho) -> Option<u64> {
    fn visit_chirho(
        node_chirho: &GreenNodeChirho,
        count_chirho: &mut usize,
        magnitude_chirho: &mut Option<u64>,
    ) {
        for child_chirho in node_chirho.children_chirho() {
            match child_chirho {
                GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.kind_chirho() == TokenKindChirho::IntegerLiteralChirho =>
                {
                    *count_chirho += 1;
                    if *count_chirho == 1 {
                        *magnitude_chirho =
                            parse_integer_magnitude_chirho(token_chirho.text_chirho());
                    }
                }
                GreenElementChirho::NodeChirho(inner_chirho) => {
                    visit_chirho(inner_chirho, count_chirho, magnitude_chirho);
                }
                GreenElementChirho::TokenChirho(_) => {}
            }
        }
    }

    let mut count_chirho = 0;
    let mut magnitude_chirho = None;
    visit_chirho(node_chirho, &mut count_chirho, &mut magnitude_chirho);
    (count_chirho == 1).then_some(magnitude_chirho).flatten()
}
