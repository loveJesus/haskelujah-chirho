// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Operator fixity and the resolution of infix chains by precedence.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::FixityChirho;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_span_chirho::SpanChirho;

use super::{AssocChirho, LowerCtxChirho};

impl LowerCtxChirho {
    pub(super) fn operator_fixity_chirho(&self, op_chirho: &str) -> (u8, AssocChirho) {
        if let Some((precedence_chirho, fixity_chirho)) =
            self.fixity_overrides_chirho.get(op_chirho)
        {
            let assoc_chirho = match fixity_chirho {
                FixityChirho::InfixChirho => AssocChirho::NoneChirho,
                FixityChirho::InfixlChirho => AssocChirho::LeftChirho,
                FixityChirho::InfixrChirho => AssocChirho::RightChirho,
            };
            (*precedence_chirho, assoc_chirho)
        } else {
            builtin_operator_fixity_chirho(op_chirho)
        }
    }

    /// workflow: compiler-pipeline-chirho/operator-fixity-chirho
    pub(super) fn resolve_infix_precedence_chirho(
        &self,
        exprs_chirho: Vec<ExprChirho>,
        ops_chirho: Vec<NameChirho>,
        span_chirho: SpanChirho,
    ) -> ExprChirho {
        // Gracefully handle mismatched expr/op counts from malformed input.
        if exprs_chirho.is_empty() {
            return ExprChirho::LitChirho(LitChirho::IntChirho(
                0,
                span_chirho,
                Some(self.fresh_origin_chirho()),
            ));
        }
        let (exprs_chirho, ops_chirho) = self.absorb_prefix_minus_chirho(exprs_chirho, ops_chirho);
        if ops_chirho.is_empty() || exprs_chirho.len() <= 1 {
            return exprs_chirho.into_iter().next().unwrap();
        }
        if ops_chirho.len() == 1 {
            let mut it_chirho = exprs_chirho.into_iter();
            let left_chirho = it_chirho.next().unwrap();
            let right_chirho = it_chirho.next().unwrap();
            let split_op_chirho = ops_chirho.into_iter().next().unwrap();
            return ExprChirho::InfixChirho {
                left_chirho: Box::new(left_chirho),
                op_chirho: split_op_chirho,
                right_chirho: Box::new(right_chirho),
                span_chirho,
            };
        }

        // Find the operator with the lowest precedence to split at.
        // For left-associative operators at the same precedence, pick the
        // rightmost occurrence so the left side groups first.
        // For right-associative, pick the leftmost occurrence.
        let mut split_idx_chirho = 0usize;
        let (mut split_prec_chirho, _) = self.operator_fixity_chirho(ops_chirho[0].text_chirho());

        for (i_chirho, op_chirho) in ops_chirho.iter().enumerate().skip(1) {
            let (prec_chirho, _assoc_chirho) = self.operator_fixity_chirho(op_chirho.text_chirho());
            if prec_chirho < split_prec_chirho {
                // Strictly lower precedence — always split here
                split_idx_chirho = i_chirho;
                split_prec_chirho = prec_chirho;
            } else if prec_chirho == split_prec_chirho {
                // Same precedence — for left-associative, prefer rightmost
                // split (so the left side stays grouped); for right-
                // associative, keep the leftmost split.
                let (_, cur_assoc_chirho) =
                    self.operator_fixity_chirho(ops_chirho[split_idx_chirho].text_chirho());
                if cur_assoc_chirho == AssocChirho::LeftChirho {
                    split_idx_chirho = i_chirho;
                }
            }
        }

        let split_op_chirho = ops_chirho[split_idx_chirho].clone();

        // Guard: if exprs is shorter than expected, return left-to-right folded
        if split_idx_chirho + 1 >= exprs_chirho.len() {
            let mut result_chirho = exprs_chirho.into_iter().next().unwrap();
            for op_chirho in ops_chirho {
                result_chirho = ExprChirho::InfixChirho {
                    left_chirho: Box::new(result_chirho),
                    op_chirho,
                    right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                        0,
                        span_chirho,
                        Some(self.fresh_origin_chirho()),
                    ))),
                    span_chirho,
                };
            }
            return result_chirho;
        }

        // Split exprs and ops around the split point
        let left_exprs_chirho: Vec<ExprChirho> = exprs_chirho[..=split_idx_chirho].to_vec();
        let right_exprs_chirho: Vec<ExprChirho> = exprs_chirho[split_idx_chirho + 1..].to_vec();
        let left_ops_chirho: Vec<NameChirho> = ops_chirho[..split_idx_chirho].to_vec();
        let right_ops_chirho: Vec<NameChirho> = ops_chirho[split_idx_chirho + 1..].to_vec();

        let left_chirho =
            self.resolve_infix_precedence_chirho(left_exprs_chirho, left_ops_chirho, span_chirho);
        let right_chirho =
            self.resolve_infix_precedence_chirho(right_exprs_chirho, right_ops_chirho, span_chirho);

        ExprChirho::InfixChirho {
            left_chirho: Box::new(left_chirho),
            op_chirho: split_op_chirho,
            right_chirho: Box::new(right_chirho),
            span_chirho,
        }
    }
}

/// Return (precedence, associativity) for a known operator.
/// Follows Haskell's default fixities.
/// Workflow: compiler-pipeline-chirho/operator-fixity-chirho.
fn builtin_operator_fixity_chirho(op_chirho: &str) -> (u8, AssocChirho) {
    match op_chirho {
        "$" | "$!" | "$!!" => (0, AssocChirho::RightChirho),
        ">>" | ">>=" => (1, AssocChirho::LeftChirho),
        "=<<" | ">=>" | "<=<" => (1, AssocChirho::RightChirho),
        "||" => (2, AssocChirho::RightChirho),
        "&&" => (3, AssocChirho::RightChirho),
        "<|>" | "<!>" => (3, AssocChirho::LeftChirho),
        "==" | "/=" | "<" | "<=" | ">" | ">=" | "~" => (4, AssocChirho::NoneChirho),
        "<$>" | "<$" | "<*>" | "<*" | "*>" | "<@>" | "<@" | "@>" | "<@@>" | "<.>" | "<." | ".>"
        | "<.*>" | "<*.>" | "<<.>>" | "<<." | ".>>" => (4, AssocChirho::LeftChirho),
        ":" | ":|" | "++" | ".|." => (5, AssocChirho::RightChirho),
        "+" | "-" | "xor" => (6, AssocChirho::LeftChirho),
        // Lowered backtick names no longer include their delimiters.
        "*" | "/" | "div" | "mod" | "quot" | "rem" | ".&." => (7, AssocChirho::LeftChirho),
        "^" | "**" => (8, AssocChirho::RightChirho),
        "." => (9, AssocChirho::RightChirho),
        _ => (9, AssocChirho::LeftChirho), // default: infixl 9
    }
}
