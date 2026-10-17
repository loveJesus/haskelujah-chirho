// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A `let` qualifier ends where its bindings do.
//!
//! The layout pass closes an implicit block only by indentation, or a `do` or
//! `case` block at the `)` of its parentheses; it knows nothing of a qualifier.
//! Written on one line, the block a qualifier's `let` opens would run on over
//! whatever follows it:
//! - in a guard, `| let y = x * 2, y > 10 = y` made `y > 10 = y` a binding that
//!   defined `>`;
//! - in a list comprehension, `[y | x <- xs, let y = x + 1]` left the block's
//!   close after the `]`, where it closed the enclosing do block, `where` or
//!   module instead; the separator that followed then belonged to the wrong
//!   block, and the next statement or declaration was lost or misread.
//! Haskell closes the block at the first token that cannot continue it, the
//! Report's parse-error(t) rule (Haskell 2010 §10.3): in a guard, the `,`
//! before the next qualifier or the `=` or `->` before the arm's body; in a
//! comprehension, the `,` before the next qualifier, the `|` of a parallel
//! branch, or the closing `]`. The parser closes it there. The virtual `}` the
//! layout pass placed for that block further on moves to this point, and the
//! virtual `;` it placed for the same block on the way are dropped: the lines
//! they separated now continue what follows the block, as they do in GHC.
//! workflow: language-features-chirho/pattern-matching-chirho (guards)
//! workflow: parallel-list-comprehensions-chirho (comprehensions)

use haskelujah_span_chirho::SpanChirho;

use super::ParserChirho;
use crate::lexer_chirho::{RawTokenChirho, RawTokenKindChirho};

impl ParserChirho<'_> {
    /// The `let` of a guard: it ends at the `,` before the next qualifier, or at
    /// the token before the arm's body, `=` in an equation or a pattern binding,
    /// `->` in a case alternative or a multi-way if.
    pub(super) fn parse_guard_let_binds_chirho(&mut self, arm_end_chirho: RawTokenKindChirho) {
        self.parse_qualifier_let_binds_chirho(&[RawTokenKindChirho::CommaChirho, arm_end_chirho]);
    }

    /// The `let` of a list comprehension: it ends at the `,` before the next
    /// qualifier, the `|` of a parallel branch, or the closing `]`.
    /// workflow: parallel-list-comprehensions-chirho
    pub(super) fn parse_comprehension_let_binds_chirho(&mut self) {
        self.parse_qualifier_let_binds_chirho(&[
            RawTokenKindChirho::CommaChirho,
            RawTokenKindChirho::PipeChirho,
            RawTokenKindChirho::RightBracketChirho,
        ]);
    }

    /// The bindings of a qualifier's `let`, just after the keyword, up to the
    /// first token in `ends_chirho` met between bindings.
    fn parse_qualifier_let_binds_chirho(&mut self, ends_chirho: &[RawTokenKindChirho]) {
        if !self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho) {
            // Explicit braces close themselves.
            self.parse_layout_block_chirho();
            return;
        }
        self.bump_chirho(); // virtual {
        self.eat_trivia_chirho();
        loop {
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho) {
                // Closed by indentation: the multi-line form.
                self.bump_chirho();
                return;
            }
            if self.at_eof_chirho() {
                return;
            }
            if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
                continue;
            }
            if ends_chirho
                .iter()
                .any(|end_chirho| self.at_chirho(*end_chirho))
            {
                self.close_implicit_block_here_chirho();
                self.bump_chirho(); // the virtual } moved here
                return;
            }
            let before_chirho = self.pos_chirho;
            self.parse_decl_chirho();
            self.eat_trivia_chirho();
            if self.pos_chirho == before_chirho {
                // Never loop in place on a token no binding can start with.
                self.bump_chirho();
            }
        }
    }

    /// Close the innermost implicit layout block at the current token (which
    /// exists: it is the token in `ends_chirho` that ended the block).
    fn close_implicit_block_here_chirho(&mut self) {
        let mut depth_chirho = 0usize;
        let mut dropped_chirho = Vec::new();
        let mut close_chirho = None;
        for idx_chirho in self.pos_chirho..self.tokens_chirho.len() {
            match self.tokens_chirho[idx_chirho].kind_chirho {
                RawTokenKindChirho::VirtualLeftBraceChirho => depth_chirho += 1,
                RawTokenKindChirho::VirtualRightBraceChirho if depth_chirho == 0 => {
                    close_chirho = Some(idx_chirho);
                    break;
                }
                RawTokenKindChirho::VirtualRightBraceChirho => depth_chirho -= 1,
                RawTokenKindChirho::VirtualSemicolonChirho if depth_chirho == 0 => {
                    dropped_chirho.push(idx_chirho);
                }
                _ => {}
            }
        }
        // The layout pass closes every implicit block by the end of input, so
        // the close is always found; without it nothing is dropped.
        if let Some(close_idx_chirho) = close_chirho {
            self.tokens_chirho.remove(close_idx_chirho);
            for idx_chirho in dropped_chirho.into_iter().rev() {
                self.tokens_chirho.remove(idx_chirho);
            }
        }
        let at_chirho = self.tokens_chirho[self.pos_chirho].span_chirho;
        self.tokens_chirho.insert(
            self.pos_chirho,
            RawTokenChirho {
                kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                span_chirho: SpanChirho::new_chirho(
                    at_chirho.file_id_chirho(),
                    at_chirho.start_chirho(),
                    at_chirho.start_chirho(),
                ),
            },
        );
    }
}
