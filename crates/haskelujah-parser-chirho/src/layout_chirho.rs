// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Haskell 2010 Layout Rule
//!
//! Implements the layout rule from Section 2.7 / 10.3 of the Haskell 2010
//! Report. Transforms a raw token stream by inserting virtual braces (`{`, `}`)
//! and semicolons (`;`) based on indentation.
//!
//! ## Algorithm overview
//!
//! The layout rule uses an indentation context stack. Each entry is either:
//! - `LayoutContextChirho::Implicit(col)` — a layout block opened by a keyword
//!   (`where`, `let`, `do`, `of`) where `col` is the column of the first token
//!   in the block.
//! - `LayoutContextChirho::Explicit` — a block opened by an explicit `{`.
//!
//! After a layout keyword:
//! - If the next non-trivia token is `{`, push `Explicit` (no virtual tokens).
//! - Otherwise, push `Implicit(col)` where `col` is the column of that token,
//!   and insert a virtual `{`.
//!
//! For each subsequent non-trivia token at column `c`:
//! - If `c == indent` of the current implicit context → insert virtual `;`
//! - If `c < indent` of the current implicit context → insert virtual `}` and
//!   pop the context, then re-process the token.
//! - If `c > indent` or we're in an explicit context → continue normally.
//!
//! Empty layout: If after a layout keyword the next token would close the
//! context immediately (column ≤ enclosing context), insert `{}`.

use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};

use crate::lexer_chirho::{RawTokenChirho, RawTokenKindChirho};

// ---------------------------------------------------------------------------
// LayoutContextChirho — indentation context stack entries
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayoutContextChirho {
    /// An implicit layout block opened by a layout keyword.
    /// Fields: column (1-based), is_let, is_of, is_where, paren_depth.
    /// - `is_let`: true if opened by `let` (closed by `in`).
    /// - `is_of`: true if opened by `of` (case alternatives can have
    ///   where-clauses, so `where` should not unconditionally close these).
    /// - `is_where`: true if opened by `where` (nested where clauses
    ///   should not close the enclosing where context when deeper).
    /// - `paren_depth`: the `(` nesting depth when the implicit block opened.
    ///   Parenthesized `do`/`case-of` blocks must close before the matching `)`.
    ImplicitChirho(u32, bool, bool, bool, u32),
    /// An explicit block opened by `{`.
    ExplicitChirho,
}

impl LayoutContextChirho {
    fn paren_depth_chirho(&self) -> Option<u32> {
        match self {
            Self::ImplicitChirho(_, _, _, _, paren_depth_chirho) => Some(*paren_depth_chirho),
            Self::ExplicitChirho => None,
        }
    }

    fn closes_on_right_paren_chirho(&self) -> bool {
        match self {
            Self::ImplicitChirho(
                _,
                is_let_chirho,
                is_of_chirho,
                is_where_chirho,
                paren_depth_chirho,
            ) => {
                *paren_depth_chirho > 0 && (*is_of_chirho || (!*is_let_chirho && !*is_where_chirho))
            }
            Self::ExplicitChirho => false,
        }
    }
}

// ---------------------------------------------------------------------------
// LayoutRuleChirho — the layout processor
// ---------------------------------------------------------------------------

/// Processes a raw token stream and inserts virtual layout tokens according
/// to the Haskell 2010 layout rule.
pub struct LayoutRuleChirho<'src> {
    /// The source text (needed for column computation).
    source_chirho: &'src str,
    /// The raw tokens (including trivia).
    raw_tokens_chirho: Vec<RawTokenChirho>,
    /// File ID for generating virtual token spans.
    file_id_chirho: FileIdChirho,
}

impl<'src> LayoutRuleChirho<'src> {
    pub fn new_chirho(
        source_chirho: &'src str,
        raw_tokens_chirho: Vec<RawTokenChirho>,
        file_id_chirho: FileIdChirho,
    ) -> Self {
        Self {
            source_chirho,
            raw_tokens_chirho,
            file_id_chirho,
        }
    }

    /// Run the layout rule and return a new token stream with virtual tokens
    /// inserted. Trivia tokens are preserved in-place.
    pub fn apply_chirho(&self) -> Vec<RawTokenChirho> {
        let mut output_chirho: Vec<RawTokenChirho> = Vec::new();
        let mut context_stack_chirho: Vec<LayoutContextChirho> = Vec::new();
        let mut paren_depth_chirho: u32 = 0;
        let mut after_layout_keyword_chirho = false;
        let mut after_let_keyword_chirho = false;
        let mut after_of_keyword_chirho = false;
        let mut after_where_keyword_chirho = false;
        let mut after_module_where_chirho = false;

        // Collect non-trivia token indices for lookahead
        let non_trivia_indices_chirho: Vec<usize> = self
            .raw_tokens_chirho
            .iter()
            .enumerate()
            .filter(|(_, t_chirho)| !t_chirho.kind_chirho.is_trivia_chirho())
            .map(|(i_chirho, _)| i_chirho)
            .collect();

        // Track which non-trivia index we are at
        let mut nt_pos_chirho: usize = 0;

        // Check if the module starts without a `module` keyword — if so, the
        // entire body is an implicit layout context at column 1. This handles
        // scripts without module headers.
        let first_non_trivia_chirho = non_trivia_indices_chirho.first().copied();
        let starts_with_module_chirho = first_non_trivia_chirho
            .map(|i_chirho| {
                self.raw_tokens_chirho[i_chirho].kind_chirho == RawTokenKindChirho::ModuleChirho
            })
            .unwrap_or(false);

        if !starts_with_module_chirho {
            // No module header — insert implicit layout at column of first token
            if let Some(first_idx_chirho) = first_non_trivia_chirho {
                let col_chirho = self.column_of_chirho(&self.raw_tokens_chirho[first_idx_chirho]);
                context_stack_chirho.push(LayoutContextChirho::ImplicitChirho(
                    col_chirho,
                    false,
                    false,
                    false,
                    paren_depth_chirho,
                ));
                let vspan_chirho = self.zero_span_at_chirho(
                    self.raw_tokens_chirho[first_idx_chirho]
                        .span_chirho
                        .start_chirho(),
                );
                output_chirho.push(RawTokenChirho {
                    kind_chirho: RawTokenKindChirho::VirtualLeftBraceChirho,
                    span_chirho: vspan_chirho,
                });
            }
        }

        for raw_idx_chirho in 0..self.raw_tokens_chirho.len() {
            let token_chirho = self.raw_tokens_chirho[raw_idx_chirho];

            // Pass trivia through unchanged
            if token_chirho.kind_chirho.is_trivia_chirho() {
                output_chirho.push(token_chirho);
                continue;
            }

            let col_chirho = self.column_of_chirho(&token_chirho);

            // After a layout keyword, the next non-trivia token determines
            // whether we get an implicit or explicit layout block.
            if after_layout_keyword_chirho {
                after_layout_keyword_chirho = false;

                if token_chirho.kind_chirho == RawTokenKindChirho::LeftBraceChirho {
                    // Explicit block — no virtual tokens, no implicit let/where tag
                    context_stack_chirho.push(LayoutContextChirho::ExplicitChirho);
                    after_let_keyword_chirho = false;
                    after_where_keyword_chirho = false;
                    output_chirho.push(token_chirho);
                    nt_pos_chirho += 1;
                    continue;
                }

                // Check for empty layout: if the token's column is ≤ the
                // enclosing context's column, insert {} immediately.
                let enclosing_col_chirho =
                    context_stack_chirho
                        .last()
                        .and_then(|ctx_chirho| match ctx_chirho {
                            LayoutContextChirho::ImplicitChirho(c_chirho, _, _, _, _) => {
                                Some(*c_chirho)
                            }
                            LayoutContextChirho::ExplicitChirho => None,
                        });

                let is_let_context_chirho = after_let_keyword_chirho;
                let is_of_context_chirho = after_of_keyword_chirho;
                let is_where_context_chirho = after_where_keyword_chirho;
                after_let_keyword_chirho = false;
                after_of_keyword_chirho = false;
                after_where_keyword_chirho = false;

                if let Some(enc_chirho) = enclosing_col_chirho {
                    if col_chirho <= enc_chirho && !after_module_where_chirho {
                        // Empty layout block
                        let vspan_chirho =
                            self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                        output_chirho.push(RawTokenChirho {
                            kind_chirho: RawTokenKindChirho::VirtualLeftBraceChirho,
                            span_chirho: vspan_chirho,
                        });
                        output_chirho.push(RawTokenChirho {
                            kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                            span_chirho: vspan_chirho,
                        });
                        // Don't push a new context — it was immediately closed.
                        // Fall through to process this token normally against
                        // the enclosing context.
                    } else {
                        // Normal implicit layout
                        context_stack_chirho.push(LayoutContextChirho::ImplicitChirho(
                            col_chirho,
                            is_let_context_chirho,
                            is_of_context_chirho,
                            is_where_context_chirho,
                            paren_depth_chirho,
                        ));
                        let vspan_chirho =
                            self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                        output_chirho.push(RawTokenChirho {
                            kind_chirho: RawTokenKindChirho::VirtualLeftBraceChirho,
                            span_chirho: vspan_chirho,
                        });
                    }
                } else {
                    // No enclosing implicit context (we're at top level or in
                    // explicit braces) — just open a new implicit context.
                    context_stack_chirho.push(LayoutContextChirho::ImplicitChirho(
                        col_chirho,
                        is_let_context_chirho,
                        is_of_context_chirho,
                        is_where_context_chirho,
                        paren_depth_chirho,
                    ));
                    let vspan_chirho =
                        self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                    output_chirho.push(RawTokenChirho {
                        kind_chirho: RawTokenKindChirho::VirtualLeftBraceChirho,
                        span_chirho: vspan_chirho,
                    });
                }

                after_module_where_chirho = false;
                // Fall through to process the token itself (semicolons, etc.)
            }

            // Before emitting the token, handle indentation-based layout.
            // Close contexts where the current column is less than the context
            // column, and insert semicolons where the column matches.
            // Skip this for EOF — the post-loop cleanup closes remaining contexts.
            if token_chirho.kind_chirho == RawTokenKindChirho::EofChirho {
                output_chirho.push(token_chirho);
                nt_pos_chirho += 1;
                continue;
            }

            if token_chirho.kind_chirho == RawTokenKindChirho::RightParenChirho
                && paren_depth_chirho > 0
            {
                let should_close_parenthesized_layout_chirho = context_stack_chirho
                    .iter()
                    .rev()
                    .take_while(|ctx_chirho| {
                        ctx_chirho.paren_depth_chirho() == Some(paren_depth_chirho)
                    })
                    .any(LayoutContextChirho::closes_on_right_paren_chirho);

                if should_close_parenthesized_layout_chirho {
                    while context_stack_chirho.last().is_some_and(|ctx_chirho| {
                        ctx_chirho.paren_depth_chirho() == Some(paren_depth_chirho)
                    }) {
                        let vspan_chirho =
                            self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                        output_chirho.push(RawTokenChirho {
                            kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                            span_chirho: vspan_chirho,
                        });
                        context_stack_chirho.pop();
                    }
                }
            }

            // Per Haskell 2010 §2.7: `in` unconditionally closes the
            // implicit layout context opened by the matching `let`.
            // It also closes any intervening implicit contexts (e.g. a
            // `case-of` block nested inside the `let` binding) until it
            // finds the `let` context.  This handles single-line patterns
            // like: `let d' = case x of P -> E in d'`
            if token_chirho.kind_chirho == RawTokenKindChirho::InChirho {
                // Close intervening non-let implicit contexts, then the let.
                while let Some(ctx_chirho) = context_stack_chirho.last() {
                    match ctx_chirho {
                        LayoutContextChirho::ImplicitChirho(_, true, _, _, _) => {
                            // Found the let context — close it and stop.
                            let vspan_chirho =
                                self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                            output_chirho.push(RawTokenChirho {
                                kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                                span_chirho: vspan_chirho,
                            });
                            context_stack_chirho.pop();
                            break;
                        }
                        LayoutContextChirho::ImplicitChirho(indent_chirho, _, _, _, _)
                            if *indent_chirho > 1 =>
                        {
                            // Close an intervening non-let implicit context
                            // (e.g. case-of, do, where inside the let body).
                            let vspan_chirho =
                                self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                            output_chirho.push(RawTokenChirho {
                                kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                                span_chirho: vspan_chirho,
                            });
                            context_stack_chirho.pop();
                        }
                        _ => {
                            // Explicit context or module-level — stop without closing.
                            break;
                        }
                    }
                }
            }

            // `where` closes non-let implicit layout contexts when the
            // `where` keyword is NOT indented deeper than the context.
            // When `where` appears indented deeper than the enclosing
            // context (col > context indent), it is a nested where clause
            // and must NOT close the enclosing context. This applies to
            // all context types: case-of, do, and where-opened contexts.
            // We preserve let-contexts (closed by `in`) and stop at the
            // module-level context (indent ≤ 1).
            if token_chirho.kind_chirho == RawTokenKindChirho::WhereChirho {
                while let Some(LayoutContextChirho::ImplicitChirho(
                    indent_chirho,
                    is_let_chirho,
                    is_of_chirho,
                    is_where_chirho,
                    _,
                )) = context_stack_chirho.last()
                {
                    // Don't close let contexts (those are closed by `in`)
                    // Don't close the outermost module-level context (indent ≤ 1)
                    if *is_let_chirho || *indent_chirho <= 1 {
                        break;
                    }
                    // For `of`/`where` contexts: only close if `where` is at
                    // or before the context indentation. When `where` is
                    // deeper, it is a nested where clause belonging to a
                    // binding inside this context.
                    // For `do`/other contexts: always close (where is not
                    // valid inside do-blocks).
                    if (*is_of_chirho || *is_where_chirho) && col_chirho > *indent_chirho {
                        break;
                    }
                    // Close this context
                    let vspan_chirho =
                        self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                    output_chirho.push(RawTokenChirho {
                        kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                        span_chirho: vspan_chirho,
                    });
                    context_stack_chirho.pop();
                }
            }

            loop {
                match context_stack_chirho.last() {
                    Some(LayoutContextChirho::ImplicitChirho(indent_chirho, _, _, _, _)) => {
                        let indent_chirho = *indent_chirho;
                        // `then`, `else`, `of` are always continuation keywords
                        // — they never start new statements or close layout blocks.
                        let is_cont_chirho = matches!(
                            token_chirho.kind_chirho,
                            RawTokenKindChirho::ThenChirho
                                | RawTokenKindChirho::ElseChirho
                                | RawTokenKindChirho::OfChirho
                        );
                        if is_cont_chirho {
                            break;
                        }
                        if col_chirho < indent_chirho {
                            // Close this layout block
                            let vspan_chirho =
                                self.zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                            output_chirho.push(RawTokenChirho {
                                kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                                span_chirho: vspan_chirho,
                            });
                            context_stack_chirho.pop();
                            // Re-check against the next context on the stack
                            continue;
                        } else if col_chirho == indent_chirho {
                            // Same indentation — insert semicolon before token
                            // But NOT for the very first token in the block
                            // (the virtual `{` was just emitted).
                            // Also skip semicolons before continuation keywords
                            // (`then`, `else`, `of`) which are always part of
                            // the current expression, not new statements.
                            let last_nt_kind_chirho = output_chirho
                                .iter()
                                .rev()
                                .find(|t_chirho| !t_chirho.kind_chirho.is_trivia_chirho())
                                .map(|t_chirho| t_chirho.kind_chirho);

                            let is_continuation_chirho = matches!(
                                token_chirho.kind_chirho,
                                RawTokenKindChirho::ThenChirho
                                    | RawTokenKindChirho::ElseChirho
                                    | RawTokenKindChirho::OfChirho
                            );

                            if !is_continuation_chirho
                                && last_nt_kind_chirho
                                    != Some(RawTokenKindChirho::VirtualLeftBraceChirho)
                            {
                                let vspan_chirho = self
                                    .zero_span_at_chirho(token_chirho.span_chirho.start_chirho());
                                output_chirho.push(RawTokenChirho {
                                    kind_chirho: RawTokenKindChirho::VirtualSemicolonChirho,
                                    span_chirho: vspan_chirho,
                                });
                            }
                            break;
                        } else {
                            // col > indent — inside the block, continue
                            break;
                        }
                    }
                    Some(LayoutContextChirho::ExplicitChirho) => {
                        // In explicit context, layout doesn't apply
                        break;
                    }
                    None => {
                        // No context — module top level
                        break;
                    }
                }
            }

            // Handle explicit brace tracking
            if token_chirho.kind_chirho == RawTokenKindChirho::LeftBraceChirho {
                context_stack_chirho.push(LayoutContextChirho::ExplicitChirho);
            } else if token_chirho.kind_chirho == RawTokenKindChirho::RightBraceChirho {
                // Close matching explicit context
                if context_stack_chirho.last() == Some(&LayoutContextChirho::ExplicitChirho) {
                    context_stack_chirho.pop();
                }
            }

            if token_chirho.kind_chirho == RawTokenKindChirho::LeftParenChirho {
                paren_depth_chirho += 1;
            } else if token_chirho.kind_chirho == RawTokenKindChirho::RightParenChirho {
                paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
            }

            // Check for layout keyword → next non-trivia token starts layout
            // Also handle \case (LambdaCase): `case` acts as a layout keyword
            // when the previous non-trivia token was `\` (backslash).
            let is_lambda_case_chirho = token_chirho.kind_chirho == RawTokenKindChirho::CaseChirho
                && nt_pos_chirho > 0
                && {
                    let prev_nt_idx_chirho = non_trivia_indices_chirho[nt_pos_chirho - 1];
                    self.raw_tokens_chirho[prev_nt_idx_chirho].kind_chirho
                        == RawTokenKindChirho::BackslashChirho
                };
            if token_chirho.kind_chirho.is_layout_keyword_chirho() || is_lambda_case_chirho {
                after_layout_keyword_chirho = true;
                after_let_keyword_chirho =
                    token_chirho.kind_chirho == RawTokenKindChirho::LetChirho;
                after_of_keyword_chirho = token_chirho.kind_chirho == RawTokenKindChirho::OfChirho
                    || is_lambda_case_chirho;
                after_where_keyword_chirho =
                    token_chirho.kind_chirho == RawTokenKindChirho::WhereChirho;

                // Track if this is the `where` after `module ... where`
                if token_chirho.kind_chirho == RawTokenKindChirho::WhereChirho {
                    let is_module_where_chirho = self.is_module_level_where_chirho(nt_pos_chirho);
                    after_module_where_chirho = is_module_where_chirho;
                }
            }

            output_chirho.push(token_chirho);
            nt_pos_chirho += 1;
        }

        // Close any remaining implicit contexts at EOF
        while let Some(ctx_chirho) = context_stack_chirho.pop() {
            if let LayoutContextChirho::ImplicitChirho(_, _, _, _, _) = ctx_chirho {
                let eof_offset_chirho =
                    ByteOffsetChirho::from_usize_chirho(self.source_chirho.len());
                let vspan_chirho = self.zero_span_at_chirho(eof_offset_chirho);
                // Insert before the EOF token if the last token is EOF
                let insert_pos_chirho = if output_chirho
                    .last()
                    .is_some_and(|t_chirho| t_chirho.kind_chirho == RawTokenKindChirho::EofChirho)
                {
                    output_chirho.len() - 1
                } else {
                    output_chirho.len()
                };
                output_chirho.insert(
                    insert_pos_chirho,
                    RawTokenChirho {
                        kind_chirho: RawTokenKindChirho::VirtualRightBraceChirho,
                        span_chirho: vspan_chirho,
                    },
                );
            }
        }

        output_chirho
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Compute the 1-based column number of a token by scanning backwards
    /// from its start offset to find the most recent newline.
    fn column_of_chirho(&self, token_chirho: &RawTokenChirho) -> u32 {
        let offset_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
        let bytes_chirho = self.source_chirho.as_bytes();

        // Scan backwards to find the start of the line
        let mut line_start_chirho = offset_chirho;
        while line_start_chirho > 0 && bytes_chirho[line_start_chirho - 1] != b'\n' {
            line_start_chirho -= 1;
        }

        // Column is 1-based
        (offset_chirho - line_start_chirho + 1) as u32
    }

    /// Create a zero-width span at a given offset (used for virtual tokens).
    fn zero_span_at_chirho(&self, offset_chirho: ByteOffsetChirho) -> SpanChirho {
        SpanChirho::new_chirho(self.file_id_chirho, offset_chirho, offset_chirho)
    }

    /// Check if the non-trivia token at `nt_pos_chirho` is the `where` that
    /// follows a `module Name` declaration (the top-level where).
    fn is_module_level_where_chirho(&self, nt_pos_chirho: usize) -> bool {
        // Walk backwards from this `where` through non-trivia tokens.
        // Skip tokens that can appear in a module header (module name,
        // export list parens, commas, identifiers, etc.). If we find
        // `module`, this is the module-level where. If we find any token
        // that cannot be in a module header, it is not.
        let non_trivia_chirho: Vec<&RawTokenChirho> = self
            .raw_tokens_chirho
            .iter()
            .filter(|t_chirho| !t_chirho.kind_chirho.is_trivia_chirho())
            .collect();

        if nt_pos_chirho == 0 {
            return false;
        }

        let mut i_chirho = nt_pos_chirho;
        while i_chirho > 0 {
            i_chirho -= 1;
            let kind_chirho = non_trivia_chirho[i_chirho].kind_chirho;
            if kind_chirho == RawTokenKindChirho::ModuleChirho {
                return true;
            }
            // Tokens that can appear between `module` and `where`:
            // module name, export list items, parens, commas, dots
            if matches!(
                kind_chirho,
                RawTokenKindChirho::ConIdChirho
                    | RawTokenKindChirho::VarIdChirho
                    | RawTokenKindChirho::LeftParenChirho
                    | RawTokenKindChirho::RightParenChirho
                    | RawTokenKindChirho::CommaChirho
                    | RawTokenKindChirho::DotDotChirho
                    | RawTokenKindChirho::VarSymChirho
                    | RawTokenKindChirho::ConSymChirho
            ) {
                continue;
            }
            // Hit a token that cannot be in a module header
            return false;
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Convenience function
// ---------------------------------------------------------------------------

/// Apply the Haskell 2010 layout rule to a raw token stream.
pub fn apply_layout_chirho(
    source_chirho: &str,
    raw_tokens_chirho: Vec<RawTokenChirho>,
    file_id_chirho: FileIdChirho,
) -> Vec<RawTokenChirho> {
    let rule_chirho =
        LayoutRuleChirho::new_chirho(source_chirho, raw_tokens_chirho, file_id_chirho);
    rule_chirho.apply_chirho()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::lexer_chirho::LexerChirho;

    /// Helper: lex + apply layout, return non-trivia token kinds.
    fn layout_kinds_chirho(source_chirho: &str) -> Vec<RawTokenKindChirho> {
        let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, file_id_chirho);
        let raw_chirho = lexer_chirho.lex_all_chirho();
        let laid_out_chirho = apply_layout_chirho(source_chirho, raw_chirho, file_id_chirho);
        laid_out_chirho
            .iter()
            .filter(|t_chirho| !t_chirho.kind_chirho.is_trivia_chirho())
            .map(|t_chirho| t_chirho.kind_chirho)
            .collect()
    }

    #[test]
    fn simple_module_layout_chirho() {
        // module Main where
        // main = putStrLn "hi"
        let kinds_chirho = layout_kinds_chirho("module Main where\nmain = putStrLn \"hi\"\n");
        // Should be:
        // module Main where { main = putStrLn "hi" }  EOF
        assert!(kinds_chirho.contains(&RawTokenKindChirho::ModuleChirho));
        assert!(kinds_chirho.contains(&RawTokenKindChirho::WhereChirho));
        assert!(kinds_chirho.contains(&RawTokenKindChirho::VirtualLeftBraceChirho));
        assert!(kinds_chirho.contains(&RawTokenKindChirho::VirtualRightBraceChirho));
    }

    #[test]
    fn multiple_decls_get_semicolons_chirho() {
        let source_chirho = "module M where\nf = 1\ng = 2\nh = 3\n";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        let semicolons_chirho = kinds_chirho
            .iter()
            .filter(|k_chirho| **k_chirho == RawTokenKindChirho::VirtualSemicolonChirho)
            .count();
        // Three declarations: f, g, h → two semicolons between them
        assert_eq!(
            semicolons_chirho, 2,
            "expected 2 semicolons, got {:?}",
            kinds_chirho
        );
    }

    #[test]
    fn script_without_module_gets_layout_chirho() {
        let kinds_chirho = layout_kinds_chirho("main = putStrLn \"hi\"\n");
        // No module keyword → implicit layout at column 1
        assert_eq!(kinds_chirho[0], RawTokenKindChirho::VirtualLeftBraceChirho);
        assert!(kinds_chirho.contains(&RawTokenKindChirho::VirtualRightBraceChirho));
    }

    #[test]
    fn do_block_layout_chirho() {
        let source_chirho = "module M where\nmain = do\n  putStrLn \"a\"\n  putStrLn \"b\"\n";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        // After `do`, a virtual { should be inserted
        let do_idx_chirho = kinds_chirho
            .iter()
            .position(|k_chirho| *k_chirho == RawTokenKindChirho::DoChirho)
            .expect("should find do");
        assert_eq!(
            kinds_chirho[do_idx_chirho + 1],
            RawTokenKindChirho::VirtualLeftBraceChirho,
            "virtual {{ should follow do"
        );

        // Two statements in do → one semicolon between them
        let semicolons_after_do_chirho = kinds_chirho[do_idx_chirho..]
            .iter()
            .filter(|k_chirho| **k_chirho == RawTokenKindChirho::VirtualSemicolonChirho)
            .count();
        assert_eq!(
            semicolons_after_do_chirho, 1,
            "expected 1 semicolon in do block"
        );
    }

    #[test]
    fn let_in_layout_chirho() {
        let source_chirho =
            "module M where\nf x = let a = 1\n          b = 2\n      in a + b + x\n";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        // After `let`, a virtual { should be inserted
        let let_idx_chirho = kinds_chirho
            .iter()
            .position(|k_chirho| *k_chirho == RawTokenKindChirho::LetChirho)
            .expect("should find let");
        assert_eq!(
            kinds_chirho[let_idx_chirho + 1],
            RawTokenKindChirho::VirtualLeftBraceChirho,
            "virtual {{ should follow let"
        );

        // `in` should close the let block
        let in_idx_chirho = kinds_chirho
            .iter()
            .position(|k_chirho| *k_chirho == RawTokenKindChirho::InChirho)
            .expect("should find in");
        // There should be a virtual } before `in`
        assert_eq!(
            kinds_chirho[in_idx_chirho - 1],
            RawTokenKindChirho::VirtualRightBraceChirho,
            "virtual }} should precede in"
        );
    }

    #[test]
    fn case_of_layout_chirho() {
        let source_chirho = "module M where\ng y = case y of\n  Nothing -> 0\n  Just x  -> x\n";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        let of_idx_chirho = kinds_chirho
            .iter()
            .position(|k_chirho| *k_chirho == RawTokenKindChirho::OfChirho)
            .expect("should find of");
        assert_eq!(
            kinds_chirho[of_idx_chirho + 1],
            RawTokenKindChirho::VirtualLeftBraceChirho,
            "virtual {{ should follow of"
        );
    }

    #[test]
    fn parenthesized_do_closes_before_right_paren_chirho() {
        let source_chirho = "module M where\nmain = id (do\n  putStrLn \"a\"\n  putStrLn \"b\")\n";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        let right_paren_idx_chirho = kinds_chirho
            .iter()
            .position(|k_chirho| *k_chirho == RawTokenKindChirho::RightParenChirho)
            .expect("should find closing paren");
        assert_eq!(
            kinds_chirho[right_paren_idx_chirho - 1],
            RawTokenKindChirho::VirtualRightBraceChirho,
            "parenthesized do should close before )"
        );
    }

    #[test]
    fn parenthesized_case_of_closes_before_right_paren_chirho() {
        let source_chirho =
            "module M where\nmain = f (case x of\n  Nothing -> 0\n  Just y  -> y)\n";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        let right_paren_idx_chirho = kinds_chirho
            .iter()
            .position(|k_chirho| *k_chirho == RawTokenKindChirho::RightParenChirho)
            .expect("should find closing paren");
        assert_eq!(
            kinds_chirho[right_paren_idx_chirho - 1],
            RawTokenKindChirho::VirtualRightBraceChirho,
            "parenthesized case-of should close before )"
        );
    }

    #[test]
    fn explicit_braces_bypass_layout_chirho() {
        let source_chirho = "module M where { f = 1 ; g = 2 }";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        // Should NOT have any virtual braces
        assert!(
            !kinds_chirho.contains(&RawTokenKindChirho::VirtualLeftBraceChirho),
            "explicit braces should not trigger virtual braces"
        );
        assert!(
            !kinds_chirho.contains(&RawTokenKindChirho::VirtualSemicolonChirho),
            "explicit semicolons should not trigger virtual semicolons"
        );
    }

    #[test]
    fn guard_layout_no_crash_chirho() {
        // Guards don't use layout keywords but they use | which could confuse
        // a naive implementation.
        let source_chirho = "module M where\ni x\n  | x > 0     = \"positive\"\n  | x < 0     = \"negative\"\n  | otherwise  = \"zero\"\n";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        // Should not crash and should contain the function name
        assert!(kinds_chirho.contains(&RawTokenKindChirho::VarIdChirho));
    }

    #[test]
    fn nested_where_clause_layout_chirho() {
        // Nested where clauses must create separate layout blocks.
        // The inner `where y = ...` at col 7 must NOT close the outer
        // `where` block at col 5.
        let source_chirho = "\
module M where
f xs = g xs
  where
    g (x:xs) = y
      where y = x + g xs
    g [] = 0
";
        let kinds_chirho = layout_kinds_chirho(source_chirho);

        // Count `where` keywords — should be exactly 3 (module + outer + inner)
        let where_count_chirho = kinds_chirho
            .iter()
            .filter(|k_chirho| **k_chirho == RawTokenKindChirho::WhereChirho)
            .count();
        assert_eq!(
            where_count_chirho, 3,
            "expected 3 where keywords (module + outer + inner)"
        );

        // Each `where` should be followed by a VirtualLeftBrace
        let where_positions_chirho: Vec<usize> = kinds_chirho
            .iter()
            .enumerate()
            .filter(|(_, k_chirho)| **k_chirho == RawTokenKindChirho::WhereChirho)
            .map(|(i_chirho, _)| i_chirho)
            .collect();
        for pos_chirho in &where_positions_chirho {
            assert_eq!(
                kinds_chirho[pos_chirho + 1],
                RawTokenKindChirho::VirtualLeftBraceChirho,
                "virtual {{ should follow where at position {}",
                pos_chirho,
            );
        }

        // Count virtual braces: should have 3 pairs (module, outer where, inner where)
        let vlb_count_chirho = kinds_chirho
            .iter()
            .filter(|k_chirho| **k_chirho == RawTokenKindChirho::VirtualLeftBraceChirho)
            .count();
        let vrb_count_chirho = kinds_chirho
            .iter()
            .filter(|k_chirho| **k_chirho == RawTokenKindChirho::VirtualRightBraceChirho)
            .count();
        assert_eq!(
            vlb_count_chirho, vrb_count_chirho,
            "VLB and VRB counts should match"
        );
        assert_eq!(
            vlb_count_chirho, 3,
            "expected 3 layout blocks (module + outer where + inner where)"
        );
    }
}
