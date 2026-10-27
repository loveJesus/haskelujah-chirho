// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Patterns: function-argument, infix, atomic, parenthesized, view, list and record patterns.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use super::cursor_chirho::qualified_name_is_consym_chirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Pattern parsing
    // -----------------------------------------------------------------------

    /// Parse a pattern. Handles infix constructor patterns (p1 :+: p2).
    pub(super) fn parse_pat_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_lpat_chirho();
        self.eat_trivia_chirho();

        // ScopedTypeVariables: pattern type annotation (p :: Type)
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::SigPatChirho);
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // the type annotation
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for infix constructor pattern
        if self.at_chirho(RawTokenKindChirho::ConSymChirho)
            || self.at_chirho(RawTokenKindChirho::BacktickChirho)
            || self.current_kind_chirho() == Some(RawTokenKindChirho::QualifiedIdChirho)
                && qualified_name_is_consym_chirho(self.current_text_chirho())
        {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixConPatChirho);

            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho(); // `
                self.eat_trivia_chirho();
                if !self.at_eof_chirho() {
                    self.bump_chirho(); // name
                }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                    self.bump_chirho(); // `
                }
            } else {
                self.bump_chirho(); // ConSym or qualified constructor symbol
            }
            self.eat_trivia_chirho();

            // Use parse_pat_chirho (not parse_lpat_chirho) so that chained
            // infix constructor patterns like `x:y:zs` are right-associative:
            // parsed as `x:(y:zs)` instead of `(x:y):zs`.
            self.parse_pat_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Parse a function argument pattern: like `parse_lpat_chirho` but does
    /// NOT greedily consume arguments after a constructor name. This ensures
    /// `f True True = ...` is parsed as two separate patterns, not
    /// `True applied_to True`. Handles as-patterns, negated literals,
    /// bang/lazy patterns, and parenthesized patterns.
    pub(super) fn parse_fun_arg_pat_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                // Variable or as-pattern
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::AtChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AsPatChirho);
                    self.bump_chirho(); // @
                    self.eat_trivia_chirho();
                    self.parse_apat_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                // Bare constructor pattern — NO argument consumption
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho(); // ConId
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    // Record pattern
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordPatChirho);
                    self.parse_record_pat_fields_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    // Nullary constructor pattern (no arguments)
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::ConPatChirho);
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "-" => {
                // Negated literal pattern — delegate to parse_apat_chirho
                // which handles the NegPat node construction
                self.parse_apat_chirho();
            }
            Some(RawTokenKindChirho::TildeChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LazyPatChirho);
                self.bump_chirho(); // ~
                self.eat_trivia_chirho();
                self.parse_fun_arg_pat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "!" => {
                // Bang pattern
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::BangPatChirho);
                self.bump_chirho(); // !
                self.eat_trivia_chirho();
                self.parse_fun_arg_pat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // Fallback to apat (parenthesized, tuple, list, literal, wildcard)
                self.parse_apat_chirho();
            }
        }
    }

    /// Parse a "left pattern": constructor application, negation, as,
    /// lazy (~pat), bang (!pat).
    pub(super) fn parse_lpat_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                // Could be as-pattern: x@pat or just variable
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarPatChirho);
                self.bump_chirho(); // var name
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();

                if self.at_chirho(RawTokenKindChirho::AtChirho) {
                    // As pattern: x@pat
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AsPatChirho);
                    self.bump_chirho(); // @
                    self.eat_trivia_chirho();
                    self.parse_apat_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                // Constructor pattern possibly with argument patterns
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho(); // ConId
                self.eat_trivia_chirho();

                // Check for record pattern
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordPatChirho);
                    self.parse_record_pat_fields_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    // Constructor pattern (with or without argument patterns)
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::ConPatChirho);
                    while self.can_start_fun_arg_pat_chirho()
                        || self.at_chirho(RawTokenKindChirho::AtChirho)
                    {
                        let before_chirho = self.pos_chirho;
                        // TyAppPat: @Type in constructor patterns
                        if self.at_chirho(RawTokenKindChirho::AtChirho) {
                            self.bump_chirho(); // @
                            self.eat_trivia_chirho();
                            // Parse the type argument (skip it for now —
                            // the type is used for type variable binding
                            // but doesn't affect the pattern structure)
                            if self.can_start_atype_chirho() {
                                self.parse_atype_chirho();
                            }
                            self.eat_trivia_chirho();
                            continue;
                        }
                        // Constructor arguments are atomic/function-argument
                        // patterns, and this path must preserve nested
                        // as-patterns like `BQ bq@(BQB _ lo)` instead of
                        // truncating them to `BQ bq`.
                        self.parse_fun_arg_pat_chirho();
                        self.eat_trivia_chirho();
                        if self.pos_chirho == before_chirho {
                            break;
                        }
                    }
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::TildeChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LazyPatChirho);
                self.bump_chirho(); // ~
                self.eat_trivia_chirho();
                self.parse_lpat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "!" => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::BangPatChirho);
                self.bump_chirho(); // !
                self.eat_trivia_chirho();
                self.parse_lpat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::VarSymChirho) if self.current_text_chirho() == "-" => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NegPatChirho);
                self.bump_chirho(); // -
                self.eat_trivia_chirho();
                self.parse_apat_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => self.parse_apat_chirho(),
        }
    }

    /// Parse an atomic pattern: variable, constructor, literal, wildcard,
    /// parenthesized, tuple, list.
    pub(super) fn parse_apat_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordPatChirho);
                    self.parse_record_pat_fields_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::ConPatChirho);
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::FloatLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LitPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::UnderscoreChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::WildcardPatChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.parse_paren_pat_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.parse_list_pat_chirho();
            }
            _ => {
                // Unexpected — wrap in error
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    /// Parse parenthesized, tuple, or view pattern.
    /// View pattern: `(expr -> pat)` requires ViewPatterns extension.
    fn parse_paren_pat_chirho(&mut self) {
        // Check if this is a view pattern: scan ahead for `->` at depth 0
        let is_view_chirho = self.scan_for_view_arrow_chirho();
        if is_view_chirho {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ViewPatChirho);
            self.bump_chirho(); // (
            self.eat_trivia_chirho();
            // Parse the view expression (everything up to `->`)
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
            // Consume the `->` (may be RightArrowChirho or VarSymChirho)
            if self.current_kind_chirho() == Some(RawTokenKindChirho::RightArrowChirho)
                || (self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
                    && self.current_text_chirho() == "->")
            {
                self.bump_chirho(); // ->
            }
            self.eat_trivia_chirho();
            // Parse the result pattern
            self.parse_pat_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenPatChirho);

        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        let is_unboxed_tuple_pat_chirho =
            self.at_chirho(RawTokenKindChirho::VarSymChirho) && self.current_text_chirho() == "#";
        if is_unboxed_tuple_pat_chirho {
            self.bump_chirho(); // leading #
            self.eat_trivia_chirho();

            if !self.at_chirho(RawTokenKindChirho::VarSymChirho)
                || self.current_text_chirho() != "#"
            {
                self.parse_pat_chirho();
                self.eat_trivia_chirho();

                while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho(); // ,
                    self.eat_trivia_chirho();
                    if !(self.at_chirho(RawTokenKindChirho::VarSymChirho)
                        && self.current_text_chirho() == "#")
                    {
                        self.parse_pat_chirho();
                        self.eat_trivia_chirho();
                    }
                }
            }

            if self.at_chirho(RawTokenKindChirho::VarSymChirho) && self.current_text_chirho() == "#"
            {
                self.bump_chirho(); // trailing #
                self.eat_trivia_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            // Unit pattern ()
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        if matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarSymChirho) | Some(RawTokenKindChirho::ConSymChirho)
        ) {
            let mut lookahead_idx_chirho = self.pos_chirho + 1;
            while lookahead_idx_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[lookahead_idx_chirho]
                    .kind_chirho
                    .is_trivia_chirho()
            {
                lookahead_idx_chirho += 1;
            }
            if lookahead_idx_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[lookahead_idx_chirho].kind_chirho
                    == RawTokenKindChirho::RightParenChirho
            {
                let pat_kind_chirho = if self.at_chirho(RawTokenKindChirho::VarSymChirho) {
                    SyntaxKindChirho::VarPatChirho
                } else {
                    SyntaxKindChirho::ConPatChirho
                };
                self.builder_chirho.start_node_chirho(pat_kind_chirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();
                self.bump_chirho(); // )
                self.builder_chirho.finish_node_chirho();
                return;
            }
        }

        self.parse_pat_chirho();
        self.eat_trivia_chirho();

        // Type-annotated pattern: (pat :: Type)
        // parse_pat_chirho already handled :: and produced a SigPatChirho
        // node inside this ParenPatChirho. Just close the paren.
        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }
        // Legacy fallback: if :: appears here (shouldn't with new parse_pat),
        // consume the type annotation tokens until closing )
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // Tuple pattern
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.parse_pat_chirho();
                    self.eat_trivia_chirho();
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Scan ahead from the current position to see if this is a view pattern `(expr -> pat)`.
    /// Returns true if we find a `->` at parenthesis depth 0 inside the outer parens.
    fn scan_for_view_arrow_chirho(&self) -> bool {
        if self.current_kind_chirho() != Some(RawTokenKindChirho::LeftParenChirho) {
            return false;
        }
        let mut pos_chirho = self.pos_chirho + 1;
        let mut depth_chirho: i32 = 1;
        // Track whether we've seen `::` at depth 1 — if so, any `->` after
        // it is a function arrow in a type annotation, not a view pattern arrow.
        let mut in_type_annotation_chirho = false;
        while pos_chirho < self.tokens_chirho.len() {
            let tok_chirho = &self.tokens_chirho[pos_chirho];
            match tok_chirho.kind_chirho {
                RawTokenKindChirho::LeftParenChirho | RawTokenKindChirho::LeftBracketChirho => {
                    depth_chirho += 1;
                }
                RawTokenKindChirho::RightParenChirho | RawTokenKindChirho::RightBracketChirho => {
                    depth_chirho -= 1;
                    if depth_chirho == 0 {
                        return false; // reached closing paren without finding ->
                    }
                }
                RawTokenKindChirho::ColonColonChirho if depth_chirho == 1 => {
                    // `::` at depth 1 means pattern type annotation — any
                    // subsequent `->` at this depth is part of the type, not
                    // a view pattern arrow.
                    in_type_annotation_chirho = true;
                }
                RawTokenKindChirho::RightArrowChirho if depth_chirho == 1 => {
                    if !in_type_annotation_chirho {
                        return true;
                    }
                }
                RawTokenKindChirho::VarSymChirho if depth_chirho == 1 => {
                    let text_chirho = self.token_text_chirho(tok_chirho);
                    if text_chirho == "->" && !in_type_annotation_chirho {
                        return true;
                    }
                }
                _ => {}
            }
            pos_chirho += 1;
        }
        false
    }

    /// Parse list pattern.
    fn parse_list_pat_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ListPatChirho);

        self.bump_chirho(); // [
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_pat_chirho();
        self.eat_trivia_chirho();

        while self.at_chirho(RawTokenKindChirho::CommaChirho) {
            self.bump_chirho(); // ,
            self.eat_trivia_chirho();
            self.parse_pat_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Record assignments use the ordinary pattern grammar, so their literals,
    /// constructors and nested delimiters survive into AST lowering.
    /// workflow: language-features-chirho/pattern-matching-chirho
    fn parse_record_pat_fields_chirho(&mut self) {
        self.bump_chirho(); // {
        self.eat_trivia_chirho();
        while !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            && !self.at_eof_chirho()
            && !self.at_decl_boundary_chirho()
        {
            let before_chirho = self.pos_chirho;
            if self.at_chirho(RawTokenKindChirho::DotDotChirho) {
                self.bump_chirho();
            } else if matches!(
                self.current_kind_chirho(),
                Some(RawTokenKindChirho::VarIdChirho | RawTokenKindChirho::QualifiedIdChirho)
            ) {
                self.bump_chirho(); // field name; without = it is a pun
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    self.parse_pat_chirho();
                }
            } else {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            } else if !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                && !self.at_eof_chirho()
                && !self.at_decl_boundary_chirho()
            {
                // A completed field must meet its separator. Do not silently
                // turn the remaining expression tokens into a field pun.
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                    && !self.at_eof_chirho()
                    && !self.at_decl_boundary_chirho()
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
            if self.pos_chirho == before_chirho {
                break;
            }
        }
        if self.at_chirho(RawTokenKindChirho::RightBraceChirho) {
            self.bump_chirho();
        }
    }

    /// Can the current token start an atomic pattern?
    pub(super) fn can_start_apat_chirho(&self) -> bool {
        matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::FloatLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::UnderscoreChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
                | Some(RawTokenKindChirho::TildeChirho)
        )
    }

    /// A `!` or `-` where a pattern must begin: a bang pattern or a negative
    /// literal.
    pub(super) fn at_prefix_pattern_operator_chirho(&self) -> bool {
        self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
            && matches!(self.current_text_chirho(), "!" | "-")
    }
}
