// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expressions: infix chains, lambda, `let`, `if` and multiway `if`, `case` and `do`.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Expression parsing
    // -----------------------------------------------------------------------

    /// Parse an expression. Handles type annotations (expr :: type) and
    /// infix operators. Stops at expression boundaries.
    pub(super) fn parse_expr_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_infixexp_chirho();
        self.eat_trivia_chirho();

        // Type annotation: expr :: Type
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::TypeAnnotExprChirho);
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Parse an infix expression: lexp op lexp op ...
    /// Kept flat for CST — precedence resolved later.
    fn parse_infixexp_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_lexp_chirho();
        self.eat_trivia_chirho();

        let mut has_infix_chirho = false;

        while self.at_infix_op_chirho() {
            let before_chirho = self.pos_chirho;
            if !has_infix_chirho {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixExprChirho);
                has_infix_chirho = true;
            }

            // The operator (VarSym, ConSym, or backticked name)
            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho(); // `
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::BacktickChirho) && !self.at_eof_chirho() {
                    self.bump_chirho(); // the name
                }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                    self.bump_chirho(); // `
                }
            } else {
                self.bump_chirho(); // operator
            }
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::BackslashChirho)
                || self.at_chirho(RawTokenKindChirho::LetChirho)
                || self.at_chirho(RawTokenKindChirho::IfChirho)
                || self.at_chirho(RawTokenKindChirho::CaseChirho)
                || self.at_chirho(RawTokenKindChirho::DoChirho)
            {
                self.parse_expr_chirho();
            } else {
                self.parse_lexp_chirho();
            }
            self.eat_trivia_chirho();
            // Safety: break if no progress to avoid infinite loop
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if has_infix_chirho {
            self.builder_chirho.finish_node_chirho(); // InfixExpr
        }
    }

    /// Parse an lexp (left-hand expression): lambda, let, if, case, do,
    /// negation, or function application.
    fn parse_lexp_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::BackslashChirho) => self.parse_lambda_chirho(),
            Some(RawTokenKindChirho::LetChirho) => self.parse_let_expr_chirho(),
            Some(RawTokenKindChirho::IfChirho) => self.parse_if_expr_chirho(),
            Some(RawTokenKindChirho::CaseChirho) => self.parse_case_expr_chirho(),
            Some(RawTokenKindChirho::DoChirho) => self.parse_do_expr_chirho(),
            Some(RawTokenKindChirho::VarSymChirho)
                if self.current_text_chirho() == "-" && !self.at_decl_boundary_chirho() =>
            {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NegateExprChirho);
                self.bump_chirho(); // -
                self.eat_trivia_chirho();
                self.parse_fexp_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => self.parse_fexp_chirho(),
        }
    }

    /// Parse a lambda expression: \pat ... -> expr
    /// Also handles LambdaCase: \case { alts }
    pub(super) fn parse_lambda_chirho(&mut self) {
        // Peek ahead: if the token after `\` is `case`, this is \case
        if self.peek_after_backslash_is_case_chirho() {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::LambdaCaseExprChirho);
            self.bump_chirho(); // backslash
            self.eat_trivia_chirho();
            self.bump_chirho(); // case
            self.eat_trivia_chirho();
            // Parse `of` if present (both `\case { ... }` and `\case of { ... }` forms)
            if self.at_chirho(RawTokenKindChirho::OfChirho) {
                self.bump_chirho(); // of
                self.eat_trivia_chirho();
            }
            // Parse the case alternatives (layout block inserted by layout engine
            // because `case` after `\` is treated as a layout keyword)
            self.parse_case_alts_chirho();
            self.builder_chirho.finish_node_chirho();
        } else {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::LambdaExprChirho);

            self.bump_chirho(); // backslash
            self.eat_trivia_chirho();

            // workflow: language-features-chirho/rank-n-visible-type-application-chirho
            // Lambda parameters admit ordinary patterns and visible type
            // abstractions. Keep the type binder in its own CST node so
            // lowering can preserve binder order (`\ @a x -> ...`).
            while self.at_chirho(RawTokenKindChirho::AtChirho)
                || self.can_start_fun_arg_pat_chirho()
            {
                let before_chirho = self.pos_chirho;
                if self.at_chirho(RawTokenKindChirho::AtChirho) {
                    self.builder_chirho
                        .start_node_chirho(SyntaxKindChirho::TypeLambdaBinderChirho);
                    self.bump_chirho(); // @
                    self.eat_trivia_chirho();
                    if self.at_chirho(RawTokenKindChirho::VarIdChirho) {
                        self.bump_chirho();
                    } else if self.at_chirho(RawTokenKindChirho::LeftParenChirho)
                        && self.is_simple_kind_annotated_binder_chirho()
                    {
                        self.parse_paren_type_chirho();
                    }
                    self.builder_chirho.finish_node_chirho();
                } else {
                    self.parse_fun_arg_pat_chirho();
                }
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    break;
                }
            }

            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
                self.parse_expr_chirho();
            }

            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Check whether the next non-trivia token after `\` is the `case` keyword.
    fn peek_after_backslash_is_case_chirho(&self) -> bool {
        // Current token is `\`; look at the next non-trivia token
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if kind_chirho.is_trivia_chirho() {
                i_chirho += 1;
                continue;
            }
            return kind_chirho == RawTokenKindChirho::CaseChirho;
        }
        false
    }

    /// Parse a let expression: let { decls } in expr
    pub(super) fn parse_let_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::LetExprChirho);

        self.bump_chirho(); // let
        self.eat_trivia_chirho();

        // Parse the let-bindings layout block
        self.parse_layout_block_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::InChirho) {
            self.bump_chirho(); // in
            self.eat_trivia_chirho();
            let before_chirho = self.pos_chirho;
            self.parse_expr_chirho();
            if self.pos_chirho == before_chirho {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                self.builder_chirho.finish_node_chirho();
            }
        } else {
            // Expression-let requires `in`; do/comprehension qualifiers use
            // their own parser and do not pass through this production.
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
            self.builder_chirho.finish_node_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse an if expression: if expr then expr else expr
    /// Also handles MultiWayIf: if | g1 -> e1 | g2 -> e2
    pub(super) fn parse_if_expr_chirho(&mut self) {
        // Peek: if the token after `if` (skipping trivia) is `|`, parse as multi-way if
        if self.peek_after_if_is_pipe_chirho() {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::MultiWayIfExprChirho);
            self.bump_chirho(); // if
            self.eat_trivia_chirho();
            // Arms: `| quals -> expr`, each a guard node as an equation's
            // guards are, its qualifiers parsed as a case alternative's.
            // Only consume `|` if lookahead confirms it is followed by
            // `quals ->` (a MultiWayIf arm). A `|` whose qualifiers reach
            // `=` first belongs to an outer guard equation.
            // workflow: language-features-chirho/pattern-matching-chirho
            while self.at_chirho(RawTokenKindChirho::PipeChirho)
                && self.scan_multiway_if_alt_has_arrow_chirho()
            {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::GuardChirho);
                self.bump_chirho(); // |
                self.eat_trivia_chirho();
                self.parse_guard_expr_case_chirho();
                // Expect -> ; if missing, this `|` is not a MultiWayIf arm
                // (it may be an outer guard pipe that was consumed by mistake).
                let is_arm_chirho = self.at_chirho(RawTokenKindChirho::RightArrowChirho);
                if is_arm_chirho {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
                self.builder_chirho.finish_node_chirho();
                if !is_arm_chirho {
                    break;
                }
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::IfExprChirho);

        self.bump_chirho(); // if
        self.eat_trivia_chirho();

        self.parse_expr_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::ThenChirho) {
            self.bump_chirho(); // then
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::ElseChirho) {
            self.bump_chirho(); // else
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Check if `if` is followed by `|` (multi-way if).
    fn peek_after_if_is_pipe_chirho(&self) -> bool {
        // Current token should be `if`. Look at the next non-trivia token.
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if kind_chirho == RawTokenKindChirho::WhitespaceChirho
                || kind_chirho == RawTokenKindChirho::LineCommentChirho
                || kind_chirho == RawTokenKindChirho::BlockCommentChirho
                || kind_chirho == RawTokenKindChirho::VirtualLeftBraceChirho
                || kind_chirho == RawTokenKindChirho::VirtualSemicolonChirho
            {
                i_chirho += 1;
                continue;
            }
            return kind_chirho == RawTokenKindChirho::PipeChirho;
        }
        false
    }

    // -----------------------------------------------------------------------
    // Where blocks
    // -----------------------------------------------------------------------

    /// Lookahead: after the current `|` token, scan for `->` before `=` or
    /// another `|` at the top level. Returns `true` when the `|` looks like a
    /// MultiWayIf arm (`| quals -> result`), `false` when it looks like an
    /// outer guard (`| guard = rhs`). A top-level `,` separates qualifiers in
    /// either, so it decides nothing; an outer guard still reaches its `=`.
    fn scan_multiway_if_alt_has_arrow_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho + 1; // skip the current `|`
        let mut paren_depth_chirho = 0u32;
        let mut bracket_depth_chirho = 0u32;
        let mut brace_depth_chirho = 0u32;
        // A qualifier's `let` ends at its `,` or the arm's `->`, as the parser
        // ends it (let_qualifier_chirho.rs). Until then its bindings' `=` is not an
        // outer guard's, a comma inside one of its bindings' own guards is that
        // binding's, and the layout pass's later close of a block ended early is
        // owed, not a scope boundary.
        let mut let_depth_chirho = 0u32;
        let mut binding_guard_chirho = false;
        let mut owed_closes_chirho = 0u32;
        let mut previous_kind_chirho = None;

        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            let top_chirho =
                paren_depth_chirho == 0 && bracket_depth_chirho == 0 && brace_depth_chirho == 0;
            match kind_chirho {
                RawTokenKindChirho::VirtualLeftBraceChirho
                    if top_chirho
                        && previous_kind_chirho == Some(RawTokenKindChirho::LetChirho) =>
                {
                    let_depth_chirho += 1;
                }
                RawTokenKindChirho::VirtualRightBraceChirho
                    if top_chirho && (let_depth_chirho > 0 || owed_closes_chirho > 0) =>
                {
                    if let_depth_chirho > 0 {
                        // Closed by indentation: the multi-line form.
                        let_depth_chirho -= 1;
                        binding_guard_chirho = false;
                    } else {
                        owed_closes_chirho -= 1;
                    }
                }
                RawTokenKindChirho::PipeChirho if top_chirho && let_depth_chirho > 0 => {
                    binding_guard_chirho = true;
                }
                RawTokenKindChirho::EqualsChirho if top_chirho && let_depth_chirho > 0 => {
                    binding_guard_chirho = false;
                }
                RawTokenKindChirho::CommaChirho
                    if top_chirho && let_depth_chirho > 0 && !binding_guard_chirho =>
                {
                    let_depth_chirho -= 1;
                    owed_closes_chirho += 1;
                }
                RawTokenKindChirho::VirtualSemicolonChirho
                    if top_chirho && let_depth_chirho > 0 =>
                {
                    // The `let`'s next binding.
                }
                RawTokenKindChirho::RightArrowChirho if top_chirho => {
                    return true;
                }
                RawTokenKindChirho::EqualsChirho if top_chirho => {
                    return false;
                }
                RawTokenKindChirho::PipeChirho if top_chirho => {
                    // Another top-level `|` before any `->` or `=`;
                    // ambiguous, but assume this one is also a MultiWayIf alt.
                    return true;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    if brace_depth_chirho == 0 {
                        return false;
                    }
                    brace_depth_chirho -= 1;
                }
                RawTokenKindChirho::VirtualSemicolonChirho if top_chirho => {
                    // Layout semicolons at top level indicate we've left the
                    // MultiWayIf context.
                    return false;
                }
                _ => {}
            }
            if !kind_chirho.is_trivia_chirho() {
                previous_kind_chirho = Some(kind_chirho);
            }
            i_chirho += 1;
        }
        false
    }

    /// Parse a case expression: case expr of { alts }
    pub(super) fn parse_case_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::CaseExprChirho);

        self.bump_chirho(); // case
        self.eat_trivia_chirho();

        self.parse_expr_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::OfChirho) {
            self.bump_chirho(); // of
            self.eat_trivia_chirho();
            self.parse_case_alts_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse case alternatives layout block.
    fn parse_case_alts_chirho(&mut self) {
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho(); // {
        }
        self.eat_trivia_chirho();

        loop {
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.bump_chirho(); // }
                break;
            }
            if self.at_eof_chirho() {
                break;
            }
            if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
                continue;
            }

            let before_chirho = self.pos_chirho;
            self.parse_case_alt_chirho();
            self.eat_trivia_chirho();
            // Safety: if no progress, bump to avoid infinite loop
            if self.pos_chirho == before_chirho {
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                } else {
                    break;
                }
            }
        }
    }

    /// Parse a single case alternative: pat -> expr or pat | guard -> expr
    fn parse_case_alt_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::CaseAltChirho);

        // Parse the pattern. An alternative always begins with one, so a
        // leading `!` (a bang pattern) or `-` (a negative literal) starts it
        // here too; `can_start_apat_chirho` cannot say so for every caller,
        // because after a function name `x ! y = ...` defines an operator.
        // Leaving the `!` out made `!_ -> e` an alternative `_ -> e` that
        // forced nothing.
        // workflow: language-features-chirho/pattern-matching-chirho
        if self.can_start_apat_chirho() || self.at_prefix_pattern_operator_chirho() {
            self.parse_pat_chirho();
            self.eat_trivia_chirho();
        }

        // -> or guards
        if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
            self.bump_chirho(); // ->
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        } else if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            // Guarded case alt
            while self.at_chirho(RawTokenKindChirho::PipeChirho) {
                self.bump_chirho(); // |
                self.eat_trivia_chirho();
                self.parse_guard_expr_case_chirho();
                if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                    self.bump_chirho(); // ->
                    self.eat_trivia_chirho();
                    self.parse_expr_chirho();
                }
                self.eat_trivia_chirho();
            }
        }

        // Optional where in case alt
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse guard qualifiers in case alt (between | and ->).
    fn parse_guard_expr_case_chirho(&mut self) {
        while !self.at_chirho(RawTokenKindChirho::RightArrowChirho)
            && !self.at_eof_chirho()
            && !self.at_decl_boundary_chirho()
        {
            let before_chirho = self.pos_chirho;
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
                continue;
            }

            if self.at_chirho(RawTokenKindChirho::LetChirho) {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LetStmtChirho);
                self.bump_chirho(); // let
                self.eat_trivia_chirho();
                self.parse_guard_let_binds_chirho(RawTokenKindChirho::RightArrowChirho);
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();
            } else if self.scan_for_case_guard_bind_arrow_chirho() {
                self.parse_pat_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
                    self.bump_chirho(); // <-
                    self.eat_trivia_chirho();
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            } else {
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
            }

            if self.pos_chirho == before_chirho {
                break;
            }
        }
    }

    /// Scan ahead to see whether the current guarded case-alt qualifier
    /// contains a top-level `<-` before the next comma or `->`.
    fn scan_for_case_guard_bind_arrow_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho;
        let mut paren_depth_chirho = 0u32;
        let mut bracket_depth_chirho = 0u32;
        let mut brace_depth_chirho = 0u32;

        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            match kind_chirho {
                RawTokenKindChirho::LeftArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return true;
                }
                RawTokenKindChirho::CommaChirho | RawTokenKindChirho::RightArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return false;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::PipeChirho if brace_depth_chirho == 0 => {
                    return false;
                }
                _ => {}
            }
            i_chirho += 1;
        }

        false
    }

    /// Parse a do expression: `do { stmts }` or `Module.do { stmts }`.
    ///
    /// workflow: language-features-chirho/qualified-do-chirho
    pub(super) fn parse_do_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DoExprChirho);

        self.bump_chirho(); // do
        self.eat_trivia_chirho();
        self.parse_do_stmt_block_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a braced or layout-delimited sequence of do statements.
    ///
    /// Shared by `do`/`mdo` expressions and RecursiveDo's nested `rec` groups.
    /// workflow: recursive-do-chirho
    fn parse_do_stmt_block_chirho(&mut self) {
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho(); // {

            loop {
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                    || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                {
                    self.bump_chirho(); // }
                    break;
                }
                if self.at_eof_chirho() {
                    break;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    continue;
                }

                let before_chirho = self.pos_chirho;
                self.parse_do_stmt_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    if !self.at_eof_chirho() {
                        self.bump_chirho();
                    } else {
                        break;
                    }
                }
            }
        }
    }

    /// Parse a single do statement.
    fn parse_do_stmt_chirho(&mut self) {
        if self.at_chirho(RawTokenKindChirho::RecChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::RecStmtChirho);
            self.bump_chirho(); // rec
            self.eat_trivia_chirho();
            self.parse_do_stmt_block_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for let statement
        if self.at_chirho(RawTokenKindChirho::LetChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::LetStmtChirho);
            self.bump_chirho(); // let
            self.eat_trivia_chirho();
            self.parse_layout_block_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Try to detect bind statement: pat <- expr
        // We use a heuristic: parse as expression, then check for <-
        let cp_chirho = self.builder_chirho.checkpoint_chirho();
        let saved_pos_chirho = self.pos_chirho;

        // Check if there's a <- by scanning ahead (without building tree)
        let has_bind_chirho = self.scan_for_bind_arrow_chirho();

        if has_bind_chirho {
            // It's a bind statement: pat <- expr
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::BindStmtChirho);
            // We need to re-parse the position for the pattern
            // Actually we haven't bumped yet, so just parse normally
            self.parse_pat_chirho();
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
                self.bump_chirho(); // <-
                self.eat_trivia_chirho();
                self.parse_expr_chirho();
            }
            self.builder_chirho.finish_node_chirho();
        } else {
            // Regular expression statement
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::DoStmtChirho);
            // Restore position since scan_for_bind_arrow didn't consume
            self.pos_chirho = saved_pos_chirho;
            self.parse_expr_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Scan ahead (without consuming) to check if there's a <- in the
    /// current statement.
    fn scan_for_bind_arrow_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho;
        let mut paren_depth_chirho = 0u32;
        let mut bracket_depth_chirho = 0u32;
        let mut brace_depth_chirho = 0u32;

        while i_chirho < self.tokens_chirho.len() {
            let k_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            match k_chirho {
                RawTokenKindChirho::LeftArrowChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return true;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    if brace_depth_chirho == 0 {
                        return false;
                    }
                    brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                }
                // Statement boundaries
                RawTokenKindChirho::VirtualSemicolonChirho
                | RawTokenKindChirho::SemicolonChirho
                | RawTokenKindChirho::EofChirho
                    if paren_depth_chirho == 0
                        && bracket_depth_chirho == 0
                        && brace_depth_chirho == 0 =>
                {
                    return false;
                }
                _ => {}
            }
            i_chirho += 1;
        }
        false
    }
}
