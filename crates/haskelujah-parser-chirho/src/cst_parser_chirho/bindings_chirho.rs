// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Value declarations: type signatures, function and pattern bindings, the infix-definition and
//! argument-pattern lookahead, guards and `where` blocks.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Value declarations (type sigs and bindings)
    // -----------------------------------------------------------------------

    pub(super) fn parse_value_decl_chirho(&mut self) {
        // BangPatterns: `let !p = e` is a STRICT binding. The `!` is kept, as the
        // BangPat around the binding's pattern, so the desugarer can force it.
        // It used to be skipped here, which made every banged `let`/`where`
        // binding silently lazy: `let !x = undefined in 5` printed 5.
        // workflow: language-features-chirho/pattern-matching-chirho
        if self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
            && self.current_text_chirho() == "!"
        {
            self.parse_pat_bind_cst_chirho();
            return;
        }
        if self.starts_annotated_pat_bind_chirho() {
            self.parse_pat_bind_cst_chirho();
        } else if self.is_type_sig_chirho() {
            self.parse_type_sig_chirho();
        } else if self.starts_pat_bind_chirho() {
            self.parse_pat_bind_cst_chirho();
        } else {
            self.parse_fun_bind_chirho();
        }
    }

    /// Detect whether the current declaration is a pattern binding rather than
    /// a function binding.  A pattern binding starts with a non-variable
    /// pattern head: tuple `(a, b) = ...`, wildcard `_ = ...`, list pattern
    /// `[x] = ...`, or literal.  A `VarIdChirho` head is always parsed as a
    /// function binding (a 0-arg function binding is semantically equivalent).
    fn starts_pat_bind_chirho(&self) -> bool {
        if self.starts_infix_fun_bind_chirho() {
            return false;
        }
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                let Some(lookahead_idx_chirho) =
                    self.peek_after_fun_arg_pat_chirho(self.pos_chirho)
                else {
                    return false;
                };
                // `whole@(a, b) = e`: a TIGHT `@` (no space on either side) is an
                // as-pattern, so this binds a pattern. `f @t ... = e` is a
                // function with a type argument, and keeps its path.
                // workflow: language-features-chirho/pattern-matching-chirho
                let tight_as_chirho =
                    self.tokens_chirho
                        .get(self.pos_chirho + 1)
                        .is_some_and(|token_chirho| {
                            token_chirho.kind_chirho == RawTokenKindChirho::AtChirho
                        })
                        && self.tokens_chirho.get(self.pos_chirho + 2).is_some_and(
                            |token_chirho| !token_chirho.kind_chirho.is_trivia_chirho(),
                        );
                let ends_pattern_chirho = self
                    .tokens_chirho
                    .get(self.skip_trivia_idx_chirho(lookahead_idx_chirho))
                    .is_some_and(|token_chirho| {
                        matches!(
                            token_chirho.kind_chirho,
                            RawTokenKindChirho::EqualsChirho | RawTokenKindChirho::PipeChirho
                        )
                    });
                if tight_as_chirho && ends_pattern_chirho {
                    return true;
                }
                let lookahead_idx_chirho = self.skip_trivia_idx_chirho(lookahead_idx_chirho);
                self.tokens_chirho
                    .get(lookahead_idx_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::ConSymChirho
                    })
            }
            // Tuple / parenthesised pattern — but not an operator section
            // like `(+) x = ...`. Heuristic: if the token right after `(`
            // is an operator symbol and is immediately closed by `)`, treat
            // it as an operator-in-parens binding head. Otherwise it is still
            // a parenthesized pattern such as `(!x, y) = ...`.
            Some(RawTokenKindChirho::LeftParenChirho) => {
                if self.starts_parenthesized_funlhs_chirho() {
                    return false;
                }
                let mut i_chirho = self.pos_chirho + 1;
                // Skip trivia
                while i_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
                {
                    i_chirho += 1;
                }
                if i_chirho >= self.tokens_chirho.len() {
                    return false;
                }
                let next_kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
                if matches!(
                    next_kind_chirho,
                    RawTokenKindChirho::VarSymChirho | RawTokenKindChirho::ConSymChirho
                ) {
                    let mut j_chirho = i_chirho + 1;
                    while j_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[j_chirho].kind_chirho.is_trivia_chirho()
                    {
                        j_chirho += 1;
                    }
                    !self
                        .tokens_chirho
                        .get(j_chirho)
                        .is_some_and(|token_chirho| {
                            token_chirho.kind_chirho == RawTokenKindChirho::RightParenChirho
                        })
                } else {
                    true
                }
            }
            // Constructor at the start of a binding: `Just x = ...`,
            // `A x = ...` — this is a constructor pattern binding, not a
            // function definition.  Type sigs (`A :: Type`) are already
            // caught by `is_type_sig_chirho` which runs first.
            Some(RawTokenKindChirho::ConIdChirho) => true,
            // Wildcard, list pattern, or literal — always a pattern binding.
            Some(RawTokenKindChirho::UnderscoreChirho)
            | Some(RawTokenKindChirho::LeftBracketChirho)
            | Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::FloatLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho)
            | Some(RawTokenKindChirho::TildeChirho) => true,
            _ => false,
        }
    }

    /// Parse a pattern binding: `pat = expr` wrapped in a `PatBindChirho` node.
    fn parse_pat_bind_cst_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::PatBindChirho);

        // Parse the LHS pattern
        self.parse_pat_chirho();
        self.eat_trivia_chirho();

        // Guarded right-hand sides, or `=` and one RHS expression
        if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            self.parse_guarded_rhs_chirho();
        } else if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        } else {
            // A binding without `=` or a guard has no RHS. Keep an error in
            // the lossless tree rather than letting lowering invent a value.
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
            self.builder_chirho.finish_node_chirho();
        }

        // Optional where clause
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_type_sig_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);

        loop {
            // Name (or operator in parens)
            if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                // Operator in parens: (+!)
                self.bump_chirho(); // (
                self.eat_trivia_chirho();
                while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho()
                {
                    self.bump_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.bump_chirho();
                }
            } else {
                self.bump_chirho(); // name
            }
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                continue;
            }
            break;
        }

        // ::
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            self.parse_type_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_fun_bind_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::FunBindChirho);

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::MatchChirho);

        if self.starts_infix_fun_bind_chirho() {
            self.parse_lpat_chirho();
            self.eat_trivia_chirho();

            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
                if !self.at_eof_chirho()
                    && (self.at_chirho(RawTokenKindChirho::VarIdChirho)
                        || self.at_chirho(RawTokenKindChirho::ConIdChirho)
                        || self.at_chirho(RawTokenKindChirho::VarSymChirho)
                        || self.at_chirho(RawTokenKindChirho::ConSymChirho))
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
            } else if self.at_chirho(RawTokenKindChirho::VarSymChirho)
                || self.at_chirho(RawTokenKindChirho::ConSymChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }

            if self.can_start_fun_arg_pat_chirho() {
                self.parse_lpat_chirho();
                self.eat_trivia_chirho();
            }
        } else {
            // Function name or pattern head
            if self.at_chirho(RawTokenKindChirho::VarIdChirho)
                || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            } else if self.starts_parenthesized_funlhs_chirho() {
                self.parse_parenthesized_funlhs_chirho();
            } else if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                let mut lookahead_idx_chirho = self.pos_chirho + 1;
                while lookahead_idx_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[lookahead_idx_chirho]
                        .kind_chirho
                        .is_trivia_chirho()
                {
                    lookahead_idx_chirho += 1;
                }
                let is_parenthesized_operator_name_chirho = lookahead_idx_chirho
                    < self.tokens_chirho.len()
                    && matches!(
                        self.tokens_chirho[lookahead_idx_chirho].kind_chirho,
                        RawTokenKindChirho::VarSymChirho | RawTokenKindChirho::ConSymChirho
                    );
                if is_parenthesized_operator_name_chirho {
                    self.bump_chirho(); // (
                    self.eat_trivia_chirho();
                    if self.at_chirho(RawTokenKindChirho::VarSymChirho)
                        || self.at_chirho(RawTokenKindChirho::ConSymChirho)
                    {
                        self.bump_chirho(); // operator name
                        self.eat_trivia_chirho();
                    }
                    if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                        self.bump_chirho(); // )
                        self.eat_trivia_chirho();
                    }
                } else {
                    // Tuple pattern or parenthesized argument
                    self.parse_apat_chirho();
                    self.eat_trivia_chirho();
                }
            } else if self.can_start_apat_chirho() {
                self.parse_apat_chirho();
                self.eat_trivia_chirho();
            }

            // Parse argument patterns until = or |
            // Use parse_fun_arg_pat_chirho which parses atomic patterns
            // without constructor application (so `f True True` gives two
            // separate patterns, not `True applied_to True`). Handles
            // as-patterns, negated literals, and bang patterns.
            while self.can_start_apat_chirho()
                || (self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
                    && (self.current_text_chirho() == "-" || self.current_text_chirho() == "!"))
            {
                let before_chirho = self.pos_chirho;
                self.parse_fun_arg_pat_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    break;
                }
            }
        }

        // Additional argument patterns after an infix lhs.
        while self.can_start_apat_chirho()
            || (self.current_kind_chirho() == Some(RawTokenKindChirho::VarSymChirho)
                && (self.current_text_chirho() == "-" || self.current_text_chirho() == "!"))
        {
            let before_chirho = self.pos_chirho;
            self.parse_fun_arg_pat_chirho();
            self.eat_trivia_chirho();
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        // Guards or = RHS
        if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            self.parse_guarded_rhs_chirho();
        } else if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
        } else {
            // A binding without `=` or a guard has no RHS. Keep an error in
            // the lossless tree rather than letting lowering invent a value.
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
            self.builder_chirho.finish_node_chirho();
        }

        self.builder_chirho.finish_node_chirho(); // Match

        // Optional where clause
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho(); // FunBind
    }

    /// A parenthesized function left side, Haskell 2010's `( funlhs ) apat { apat }`: the
    /// operator's own definition, as in `(f . g) x = g (f x)`. The inner `funlhs` is
    /// `pat varop pat`, or another parenthesized one. A `varop` is a variable operator or a
    /// backticked variable name, never a constructor operator, so `(x:_) = ...` stays a
    /// pattern binding. At least one argument pattern must follow, as GHC requires.
    /// workflow: language-features-chirho/pattern-matching-chirho
    fn starts_parenthesized_funlhs_chirho(&self) -> bool {
        self.peek_after_parenthesized_funlhs_chirho(self.pos_chirho)
            .is_some_and(|after_chirho| {
                self.can_start_fun_arg_pat_idx_chirho(self.skip_trivia_idx_chirho(after_chirho))
            })
    }

    /// The index after the `)` of a parenthesized `funlhs` that opens at `open_idx_chirho`.
    fn peek_after_parenthesized_funlhs_chirho(&self, open_idx_chirho: usize) -> Option<usize> {
        if self.tokens_chirho.get(open_idx_chirho)?.kind_chirho
            != RawTokenKindChirho::LeftParenChirho
        {
            return None;
        }
        let inner_chirho = self.skip_trivia_idx_chirho(open_idx_chirho + 1);
        let close_chirho = match self.peek_after_parenthesized_funlhs_chirho(inner_chirho) {
            Some(after_nested_chirho) => {
                let mut idx_chirho = self.skip_trivia_idx_chirho(after_nested_chirho);
                if !self.can_start_fun_arg_pat_idx_chirho(idx_chirho) {
                    return None;
                }
                while self.can_start_fun_arg_pat_idx_chirho(idx_chirho) {
                    let after_chirho = self.peek_after_fun_arg_pat_chirho(idx_chirho)?;
                    if after_chirho == idx_chirho {
                        return None;
                    }
                    idx_chirho = self.skip_trivia_idx_chirho(after_chirho);
                }
                idx_chirho
            }
            None => {
                let operator_chirho =
                    self.skip_trivia_idx_chirho(self.peek_after_fun_arg_pat_chirho(inner_chirho)?);
                let right_chirho =
                    self.skip_trivia_idx_chirho(self.peek_after_varop_chirho(operator_chirho)?);
                self.skip_trivia_idx_chirho(self.peek_after_fun_arg_pat_chirho(right_chirho)?)
            }
        };
        (self.tokens_chirho.get(close_chirho)?.kind_chirho == RawTokenKindChirho::RightParenChirho)
            .then_some(close_chirho + 1)
    }

    /// The index after a variable operator at `idx_chirho`: a variable symbol that is not a
    /// prefix pattern operator (`!x`, `-1`), or a backticked variable name.
    fn peek_after_varop_chirho(&self, idx_chirho: usize) -> Option<usize> {
        match self.tokens_chirho.get(idx_chirho)?.kind_chirho {
            RawTokenKindChirho::VarSymChirho
                if !self.starts_prefix_pattern_operator_chirho(idx_chirho) =>
            {
                Some(idx_chirho + 1)
            }
            RawTokenKindChirho::BacktickChirho => {
                let name_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
                let close_chirho = self.skip_trivia_idx_chirho(name_chirho + 1);
                (self.tokens_chirho.get(name_chirho)?.kind_chirho
                    == RawTokenKindChirho::VarIdChirho
                    && self.tokens_chirho.get(close_chirho)?.kind_chirho
                        == RawTokenKindChirho::BacktickChirho)
                    .then_some(close_chirho + 1)
            }
            _ => None,
        }
    }

    /// A parenthesized function left side, as one flat run in the Match: `(`, the left operand,
    /// the operator, the right operand and `)`, or a nested one and its arguments. Lowering
    /// names the binding by the operator after the first operand and takes every pattern in
    /// order, as it does for the prefix form `(op) x y z`.
    fn parse_parenthesized_funlhs_chirho(&mut self) {
        self.bump_chirho(); // (
        self.eat_trivia_chirho();
        if self
            .peek_after_parenthesized_funlhs_chirho(self.pos_chirho)
            .is_some()
        {
            self.parse_parenthesized_funlhs_chirho();
            while self.can_start_fun_arg_pat_chirho() {
                let before_chirho = self.pos_chirho;
                self.parse_fun_arg_pat_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    break;
                }
            }
        } else {
            self.parse_lpat_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_backticked_operator_chirho();
            } else {
                self.bump_chirho(); // the operator
            }
            self.eat_trivia_chirho();
            self.parse_lpat_chirho();
            self.eat_trivia_chirho();
        }
        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho(); // )
            self.eat_trivia_chirho();
        }
    }

    fn starts_infix_fun_bind_chirho(&self) -> bool {
        let Some(current_kind_chirho) = self.current_kind_chirho() else {
            return false;
        };
        let starts_with_name_head_chirho = matches!(
            current_kind_chirho,
            RawTokenKindChirho::VarIdChirho | RawTokenKindChirho::ConIdChirho
        );
        let mut lookahead_idx_chirho = match self.peek_after_fun_arg_pat_chirho(self.pos_chirho) {
            Some(idx_chirho) => idx_chirho,
            None => return false,
        };
        lookahead_idx_chirho = self.skip_trivia_idx_chirho(lookahead_idx_chirho);
        if lookahead_idx_chirho >= self.tokens_chirho.len() {
            return false;
        }
        if starts_with_name_head_chirho
            && self.starts_prefix_pattern_operator_chirho(lookahead_idx_chirho)
        {
            let head_continues_as_pattern_chirho = self
                .tokens_chirho
                .get(self.skip_trivia_idx_chirho(self.pos_chirho + 1))
                .is_some_and(|token_chirho| {
                    token_chirho.kind_chirho == RawTokenKindChirho::AtChirho
                });
            if !head_continues_as_pattern_chirho {
                // A prefix `!x` or `-1` after the head is an argument pattern
                // (`f !x !y = ...`, `g -1 y = ...`); a loose, suffix or tight
                // `x ! y`, `a-b` defines the operator (GHC's whitespace rule).
                return false;
            }
        }
        matches!(
            self.tokens_chirho[lookahead_idx_chirho].kind_chirho,
            RawTokenKindChirho::BacktickChirho | RawTokenKindChirho::VarSymChirho
        )
    }

    fn peek_after_fun_arg_pat_chirho(&self, start_idx_chirho: usize) -> Option<usize> {
        let current_kind_chirho = self.tokens_chirho.get(start_idx_chirho)?.kind_chirho;
        match current_kind_chirho {
            RawTokenKindChirho::VarIdChirho => {
                let mut idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                if self.tokens_chirho.get(idx_chirho)?.kind_chirho == RawTokenKindChirho::AtChirho {
                    idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
                    self.peek_after_apat_chirho(idx_chirho)
                } else {
                    Some(idx_chirho)
                }
            }
            RawTokenKindChirho::ConIdChirho | RawTokenKindChirho::QualifiedIdChirho => {
                let mut idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                if self
                    .tokens_chirho
                    .get(idx_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::LeftBraceChirho
                    })
                {
                    return self.peek_after_balanced_group_chirho(
                        idx_chirho,
                        RawTokenKindChirho::LeftBraceChirho,
                        RawTokenKindChirho::RightBraceChirho,
                    );
                }

                while self.can_start_fun_arg_pat_idx_chirho(idx_chirho) {
                    let after_arg_chirho = self.peek_after_fun_arg_pat_chirho(idx_chirho)?;
                    if after_arg_chirho == idx_chirho {
                        break;
                    }
                    idx_chirho = self.skip_trivia_idx_chirho(after_arg_chirho);
                }
                Some(idx_chirho)
            }
            RawTokenKindChirho::VarSymChirho => {
                let token_text_chirho =
                    self.token_text_chirho(&self.tokens_chirho[start_idx_chirho]);
                if token_text_chirho == "-" || token_text_chirho == "!" {
                    let idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                    self.peek_after_apat_chirho(idx_chirho)
                } else {
                    None
                }
            }
            RawTokenKindChirho::TildeChirho => {
                let idx_chirho = self.skip_trivia_idx_chirho(start_idx_chirho + 1);
                self.peek_after_apat_chirho(idx_chirho)
            }
            _ => self.peek_after_apat_chirho(start_idx_chirho),
        }
    }

    pub(super) fn peek_after_apat_chirho(&self, start_idx_chirho: usize) -> Option<usize> {
        let current_kind_chirho = self.tokens_chirho.get(start_idx_chirho)?.kind_chirho;
        match current_kind_chirho {
            RawTokenKindChirho::VarIdChirho
            | RawTokenKindChirho::ConIdChirho
            | RawTokenKindChirho::QualifiedIdChirho
            | RawTokenKindChirho::UnderscoreChirho
            | RawTokenKindChirho::IntLitChirho
            | RawTokenKindChirho::FloatLitChirho
            | RawTokenKindChirho::CharLitChirho
            | RawTokenKindChirho::StringLitChirho => Some(start_idx_chirho + 1),
            RawTokenKindChirho::LeftParenChirho => {
                self.peek_after_parenthesized_apat_chirho(start_idx_chirho)
            }
            RawTokenKindChirho::LeftBracketChirho => self.peek_after_balanced_group_chirho(
                start_idx_chirho,
                RawTokenKindChirho::LeftBracketChirho,
                RawTokenKindChirho::RightBracketChirho,
            ),
            _ => None,
        }
    }

    fn peek_after_parenthesized_apat_chirho(&self, start_idx_chirho: usize) -> Option<usize> {
        self.peek_after_balanced_group_chirho(
            start_idx_chirho,
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::RightParenChirho,
        )
    }

    fn peek_after_balanced_group_chirho(
        &self,
        start_idx_chirho: usize,
        left_kind_chirho: RawTokenKindChirho,
        right_kind_chirho: RawTokenKindChirho,
    ) -> Option<usize> {
        if self.tokens_chirho.get(start_idx_chirho)?.kind_chirho != left_kind_chirho {
            return None;
        }

        let mut depth_chirho = 0usize;
        let mut idx_chirho = start_idx_chirho;
        while idx_chirho < self.tokens_chirho.len() {
            match self.tokens_chirho[idx_chirho].kind_chirho {
                kind_chirho if kind_chirho == left_kind_chirho => {
                    depth_chirho += 1;
                }
                kind_chirho if kind_chirho == right_kind_chirho => {
                    depth_chirho = depth_chirho.saturating_sub(1);
                    if depth_chirho == 0 {
                        return Some(idx_chirho + 1);
                    }
                }
                _ => {}
            }
            idx_chirho += 1;
        }

        None
    }

    fn parse_guarded_rhs_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::GuardedRhsChirho);

        while self.at_chirho(RawTokenKindChirho::PipeChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::GuardChirho);

            self.bump_chirho(); // |
            self.eat_trivia_chirho();

            // Guard qualifiers until `=`. Supports:
            //   | guardExpr = rhs
            //   | pat <- expr, guardExpr = rhs
            //   | let binds, guardExpr = rhs
            while !self.at_chirho(RawTokenKindChirho::EqualsChirho)
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
                    self.parse_guard_let_binds_chirho(RawTokenKindChirho::EqualsChirho);
                    self.builder_chirho.finish_node_chirho();
                    self.eat_trivia_chirho();
                } else if self.scan_for_guard_bind_arrow_chirho() {
                    self.parse_pat_chirho();
                    self.eat_trivia_chirho();
                    if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
                        self.bump_chirho(); // <-
                        self.eat_trivia_chirho();
                        self.parse_expr_chirho();
                        self.eat_trivia_chirho();
                    }
                } else {
                    self.parse_guard_expr_chirho();
                }

                if self.pos_chirho == before_chirho {
                    break;
                }
            }

            if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
                self.bump_chirho(); // =
                self.eat_trivia_chirho();
                self.parse_expr_chirho();
            }

            self.builder_chirho.finish_node_chirho(); // Guard
            self.eat_trivia_chirho();
        }

        self.builder_chirho.finish_node_chirho(); // GuardedRhs
    }

    /// Parse a guard expression (the part between | and =).
    fn parse_guard_expr_chirho(&mut self) {
        self.parse_expr_chirho();
        self.eat_trivia_chirho();
    }

    /// Scan ahead to see whether the current guard qualifier contains a
    /// top-level `<-` before the next comma or `=`.
    fn scan_for_guard_bind_arrow_chirho(&self) -> bool {
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
                RawTokenKindChirho::CommaChirho | RawTokenKindChirho::EqualsChirho
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

    pub(super) fn parse_where_block_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::WhereClauseChirho);

        self.expect_chirho(RawTokenKindChirho::WhereChirho);
        self.eat_trivia_chirho();

        // Eat the layout block
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
                // A leaked top-level declaration starter here means the
                // surrounding layout block should have ended already. Stop
                // instead of swallowing sibling decls into this where-block.
                if matches!(
                    self.current_kind_chirho(),
                    Some(RawTokenKindChirho::InstanceChirho)
                        | Some(RawTokenKindChirho::ClassChirho)
                        | Some(RawTokenKindChirho::ImportChirho)
                        | Some(RawTokenKindChirho::ModuleChirho)
                ) {
                    break;
                }
                let before_chirho = self.pos_chirho;
                self.parse_decl_chirho();
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

        self.builder_chirho.finish_node_chirho();
    }

    pub(super) fn current_starts_unseparated_value_decl_chirho(&self) -> bool {
        if self.current_kind_chirho() != Some(RawTokenKindChirho::VarIdChirho) {
            return false;
        }

        let mut prev_idx_chirho = self.pos_chirho;
        let mut saw_newline_trivia_chirho = false;
        while prev_idx_chirho > 0 {
            prev_idx_chirho -= 1;
            let token_chirho = &self.tokens_chirho[prev_idx_chirho];
            match token_chirho.kind_chirho {
                RawTokenKindChirho::WhitespaceChirho
                | RawTokenKindChirho::LineCommentChirho
                | RawTokenKindChirho::BlockCommentChirho => {
                    if self.token_text_chirho(token_chirho).contains('\n') {
                        saw_newline_trivia_chirho = true;
                    }
                    continue;
                }
                RawTokenKindChirho::VirtualSemicolonChirho
                | RawTokenKindChirho::SemicolonChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho
                | RawTokenKindChirho::LeftBraceChirho => break,
                _ if saw_newline_trivia_chirho => break,
                _ => return false,
            }
        }

        let next_idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho + 1);
        self.tokens_chirho
            .get(next_idx_chirho)
            .is_some_and(|token_chirho| {
                matches!(
                    token_chirho.kind_chirho,
                    RawTokenKindChirho::ColonColonChirho | RawTokenKindChirho::EqualsChirho
                )
            })
    }

    /// Lookahead: is this a type signature? (name :: ...)
    fn is_type_sig_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho;

        loop {
            // Skip one name — could be `(+!)` for operator type sigs
            if i_chirho < self.tokens_chirho.len() {
                let k_chirho = self.tokens_chirho[i_chirho].kind_chirho;
                if k_chirho == RawTokenKindChirho::VarIdChirho
                    || k_chirho == RawTokenKindChirho::ConIdChirho
                {
                    i_chirho += 1;
                } else if k_chirho == RawTokenKindChirho::LeftParenChirho {
                    // Operator in parens: (+!)
                    i_chirho += 1;
                    while i_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[i_chirho].kind_chirho
                            != RawTokenKindChirho::RightParenChirho
                    {
                        i_chirho += 1;
                    }
                    if i_chirho < self.tokens_chirho.len() {
                        i_chirho += 1; // skip )
                    }
                }
            }

            // Skip trivia after the current name.
            while i_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
            {
                i_chirho += 1;
            }

            if i_chirho < self.tokens_chirho.len()
                && self.tokens_chirho[i_chirho].kind_chirho == RawTokenKindChirho::CommaChirho
            {
                i_chirho += 1;
                while i_chirho < self.tokens_chirho.len()
                    && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
                {
                    i_chirho += 1;
                }
                continue;
            }
            break;
        }

        // Check for ::
        i_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[i_chirho].kind_chirho == RawTokenKindChirho::ColonColonChirho
    }

    pub(super) fn can_start_fun_arg_pat_chirho(&self) -> bool {
        self.can_start_fun_arg_pat_idx_chirho(self.pos_chirho)
    }

    fn can_start_fun_arg_pat_idx_chirho(&self, idx_chirho: usize) -> bool {
        matches!(
            self.tokens_chirho
                .get(idx_chirho)
                .map(|token_chirho| token_chirho.kind_chirho),
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
                | Some(RawTokenKindChirho::VarSymChirho)
        ) && !self
            .tokens_chirho
            .get(idx_chirho)
            .is_some_and(|token_chirho| {
                token_chirho.kind_chirho == RawTokenKindChirho::VarSymChirho
                    && !self.starts_prefix_pattern_operator_chirho(idx_chirho)
            })
    }

    /// GHC's whitespace rule for `!` and `-` after a pattern. Only a PREFIX occurrence starts an
    /// argument pattern: a bang pattern (`f !x`), or a negative literal (`g -1`, read as
    /// NegativeLiterals reads it). A loose, suffix or tight-infix occurrence (`x ! y`, `a! b`,
    /// `a-b`), or a prefix `-` before anything but a number (`a -b`), is the operator an infix
    /// definition defines.
    /// workflow: language-features-chirho/pattern-matching-chirho
    fn starts_prefix_pattern_operator_chirho(&self, idx_chirho: usize) -> bool {
        let Some(token_chirho) = self.tokens_chirho.get(idx_chirho) else {
            return false;
        };
        if token_chirho.kind_chirho != RawTokenKindChirho::VarSymChirho
            || !self.is_prefix_occurrence_chirho(idx_chirho)
        {
            return false;
        }
        match self.token_text_chirho(token_chirho) {
            "!" => true,
            "-" => self
                .tokens_chirho
                .get(idx_chirho + 1)
                .is_some_and(|next_chirho| {
                    matches!(
                        next_chirho.kind_chirho,
                        RawTokenKindChirho::IntLitChirho | RawTokenKindChirho::FloatLitChirho
                    )
                }),
            _ => false,
        }
    }

    /// GHC's lexer (`varsym_prefix` in `Lexer.x`): an operator occurrence is PREFIX when the
    /// character before it does not close a token and the character after it opens one.
    fn is_prefix_occurrence_chirho(&self, idx_chirho: usize) -> bool {
        let Some(token_chirho) = self.tokens_chirho.get(idx_chirho) else {
            return false;
        };
        let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
        let end_chirho = token_chirho.span_chirho.end_chirho().as_usize_chirho();
        !self
            .source_chirho
            .get(..start_chirho)
            .is_some_and(closes_token_chirho)
            && self
                .source_chirho
                .get(end_chirho..)
                .is_some_and(opens_token_chirho)
    }
}

/// GHC's `precededByClosingToken`: the text before an operator ends an identifier, a literal or
/// a closing bracket, so the operator touches what precedes it. A comment's `-}` does not count.
fn closes_token_chirho(before_chirho: &str) -> bool {
    let mut chars_chirho = before_chirho.chars().rev();
    match chars_chirho.next() {
        Some('}') => chars_chirho.next() != Some('-'),
        Some(')' | ']' | '"' | '\'' | '_' | '⟧' | '⦈') => true,
        Some(char_chirho) => char_chirho.is_alphanumeric(),
        None => false,
    }
}

/// GHC's `followedByOpeningToken`: the text after an operator starts an identifier, a literal or
/// an opening bracket. A comment's `{-` does not count.
fn opens_token_chirho(after_chirho: &str) -> bool {
    let mut chars_chirho = after_chirho.chars();
    match chars_chirho.next() {
        Some('{') => chars_chirho.next() != Some('-'),
        Some('(' | '[' | '"' | '\'' | '_' | '⟦' | '⦇') => true,
        Some(char_chirho) => char_chirho.is_alphanumeric(),
        None => false,
    }
}
