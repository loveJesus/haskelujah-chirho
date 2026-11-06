// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Application and atomic expressions: parentheses and sections, lists and comprehensions,
//! records, quoted names, and layout blocks.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

mod comprehension_chirho;

use super::ParserChirho;
use super::cursor_chirho::{qualified_local_text_chirho, qualified_name_is_operator_chirho};
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    /// Parse a function application: f x y z
    pub(super) fn parse_fexp_chirho(&mut self) {
        if !self.can_start_aexp_chirho() {
            return;
        }

        let cp_chirho = self.builder_chirho.checkpoint_chirho();
        self.parse_aexp_chirho();
        self.eat_trivia_chirho();

        let mut count_chirho = 1u32;
        loop {
            // TypeApplications: @Type after an expression
            if self.at_chirho(RawTokenKindChirho::AtChirho) {
                // Close any open AppExpr node first
                if count_chirho > 1 {
                    self.builder_chirho.finish_node_chirho(); // AppExpr
                    count_chirho = 1; // reset
                }
                // Wrap the completed term-application spine, not the still-open
                // AppExpr builder. Reversing this order leaks later declarations
                // into a malformed TypeAppExpr (`f x @T`).
                // workflow: language-features-chirho/rank-n-visible-type-application-chirho
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::TypeAppExprChirho);
                self.bump_chirho(); // consume @
                self.eat_trivia_chirho();
                self.parse_atype_chirho();
                self.builder_chirho.finish_node_chirho(); // TypeAppExpr
                self.eat_trivia_chirho();
                continue;
            }

            if !self.can_start_aexp_chirho() {
                break;
            }
            let before_chirho = self.pos_chirho;
            if count_chirho == 1 {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AppExprChirho);
            }
            count_chirho += 1;
            self.parse_aexp_chirho();
            self.eat_trivia_chirho();
            // Safety: break if no progress to avoid infinite loop
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if count_chirho > 1 {
            self.builder_chirho.finish_node_chirho(); // AppExpr
        }
    }

    /// Parse an atomic expression: literal, variable, constructor,
    /// parenthesized, tuple, list, section.
    fn parse_aexp_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NameExprChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordUpdateExprChirho);
                    self.parse_record_expr_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::TickChirho) => {
                self.parse_quoted_name_expr_chirho();
            }
            Some(RawTokenKindChirho::ConIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NameExprChirho);
                self.bump_chirho();
                // Check for record construction: Con { field = val }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    // Could be record construction — but in a CST we
                    // need to be careful. This might also be a layout
                    // thing. Only treat as record if it's an explicit brace.
                    self.parse_record_expr_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::QualifiedIdChirho) => {
                let current_text_chirho = self.current_text_chirho().to_string();
                let local_text_chirho = qualified_local_text_chirho(&current_text_chirho);
                let is_constructor_like_chirho = local_text_chirho
                    .chars()
                    .next()
                    .is_some_and(|ch_chirho| ch_chirho.is_ascii_uppercase());
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NameExprChirho);
                self.bump_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    if is_constructor_like_chirho {
                        self.parse_record_expr_chirho();
                        self.builder_chirho.finish_node_chirho();
                    } else {
                        self.builder_chirho.finish_node_chirho();
                        self.builder_chirho.start_node_at_chirho(
                            cp_chirho,
                            SyntaxKindChirho::RecordUpdateExprChirho,
                        );
                        self.parse_record_expr_chirho();
                        self.builder_chirho.finish_node_chirho();
                    }
                } else {
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::FloatLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LiteralExprChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            // Typed holes: `_` in expression position (GHC's TypedHoles)
            Some(RawTokenKindChirho::UnderscoreChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::NameExprChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                let cp_chirho = self.builder_chirho.checkpoint_chirho();
                self.parse_paren_expr_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(cp_chirho, SyntaxKindChirho::RecordUpdateExprChirho);
                    self.parse_record_expr_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.parse_list_expr_chirho();
            }
            // BlockArguments: block-like expressions valid as function arguments
            Some(RawTokenKindChirho::BackslashChirho) => {
                self.parse_lambda_chirho();
            }
            Some(RawTokenKindChirho::DoChirho) => {
                self.parse_do_expr_chirho();
            }
            Some(RawTokenKindChirho::CaseChirho) => {
                self.parse_case_expr_chirho();
            }
            Some(RawTokenKindChirho::IfChirho) => {
                self.parse_if_expr_chirho();
            }
            Some(RawTokenKindChirho::LetChirho) => {
                self.parse_let_expr_chirho();
            }
            // Template Haskell splice: $name or $(expr)
            Some(RawTokenKindChirho::ThSpliceChirho) => {
                self.parse_splice_expr_chirho();
            }
            // Template Haskell typed splice: $$name or $$(expr)
            Some(RawTokenKindChirho::ThTypedSpliceChirho) => {
                self.parse_typed_splice_expr_chirho();
            }
            // Template Haskell expression quotation: [| expr |] or [e| expr |]
            Some(RawTokenKindChirho::ThOpenExpQuoteChirho)
            | Some(RawTokenKindChirho::ThOpenExpExplicitQuoteChirho) => {
                self.parse_quote_expr_chirho();
            }
            // Template Haskell declaration quotation: [d| decls |]
            Some(RawTokenKindChirho::ThOpenDecQuoteChirho) => {
                self.parse_quote_decl_chirho();
            }
            // Template Haskell type quotation: [t| type |]
            Some(RawTokenKindChirho::ThOpenTypeQuoteChirho) => {
                self.parse_quote_type_chirho();
            }
            // Template Haskell pattern quotation: [p| pat |]
            Some(RawTokenKindChirho::ThOpenPatQuoteChirho) => {
                self.parse_quote_pat_chirho();
            }
            // Template Haskell typed expression quotation: [|| expr ||]
            Some(RawTokenKindChirho::ThOpenTypedExpQuoteChirho) => {
                self.parse_typed_quote_expr_chirho();
            }
            _ => {
                // Unexpected token — wrap in error
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    fn parse_quoted_name_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::QuotedNameExprChirho);

        while self.at_chirho(RawTokenKindChirho::TickChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho)
            | Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                self.bump_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.bump_chirho();
                self.eat_trivia_chirho();
                while !self.at_chirho(RawTokenKindChirho::RightParenChirho) && !self.at_eof_chirho()
                {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.bump_chirho();
                }
            }
            _ => {
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a parenthesized expression, tuple, section, or unit.
    fn parse_paren_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenExprChirho);

        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            // Unit ()
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Detect bare operator section: (op) where op is an operator
        // symbol immediately followed by ).  Wrap the operator in a
        // NameExprChirho so it lowers to a VarChirho reference.
        if self.is_operator_section_chirho() {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NameExprChirho);
            self.bump_chirho(); // the operator token
            self.builder_chirho.finish_node_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for left section: (op expr) — operator NOT immediately
        // followed by ')'.  Emit operator token and then the expression
        // inside the ParenExprChirho; the CST→AST lowerer will detect
        // the leading operator token and produce LeftSectionChirho.
        // IMPORTANT: exclude `-` (minus) because `(-e)` is always negation
        // in Haskell, never a section.
        let is_op_chirho = matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarSymChirho) | Some(RawTokenKindChirho::ConSymChirho)
        ) || self.current_kind_chirho()
            == Some(RawTokenKindChirho::QualifiedIdChirho)
            && qualified_name_is_operator_chirho(self.current_text_chirho());
        let is_minus_chirho = is_op_chirho && self.current_text_chirho() == "-";
        let is_backticked_left_section_chirho = self.is_backticked_left_section_start_chirho();
        if (is_op_chirho && !is_minus_chirho && !self.is_operator_section_chirho())
            || is_backticked_left_section_chirho
        {
            if is_backticked_left_section_chirho {
                self.bump_backticked_operator_chirho();
            } else {
                self.bump_chirho(); // the operator
            }
            self.eat_trivia_chirho();
            self.parse_expr_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
            }
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // TupleSections: if current token is a comma, first position is a gap
        if !self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // Could be a right section: (expr op) or just (expr) or (expr, expr, ...)
            self.parse_expr_chirho();
            self.eat_trivia_chirho();

            // Check for right section: after expr, we have op followed by ')'
            let is_right_op_chirho = matches!(
                self.current_kind_chirho(),
                Some(RawTokenKindChirho::VarSymChirho) | Some(RawTokenKindChirho::ConSymChirho)
            ) || self.current_kind_chirho()
                == Some(RawTokenKindChirho::QualifiedIdChirho)
                && qualified_name_is_operator_chirho(self.current_text_chirho());
            let is_backticked_right_section_chirho = self.is_backticked_right_section_chirho();
            if is_right_op_chirho || is_backticked_right_section_chirho {
                // Look ahead past operator + trivia for ')'
                let is_right_section_chirho = if is_backticked_right_section_chirho {
                    true
                } else {
                    let mut look_chirho = self.pos_chirho + 1;
                    while look_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[look_chirho]
                            .kind_chirho
                            .is_trivia_chirho()
                    {
                        look_chirho += 1;
                    }
                    look_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[look_chirho].kind_chirho
                            == RawTokenKindChirho::RightParenChirho
                };

                if is_right_section_chirho {
                    // Right section: emit the operator token; lowerer detects it
                    if is_backticked_right_section_chirho {
                        self.bump_backticked_operator_chirho();
                    } else {
                        self.bump_chirho(); // the operator
                    }
                    self.eat_trivia_chirho();
                    if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                        self.bump_chirho();
                    }
                    self.builder_chirho.finish_node_chirho();
                    return;
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // Tuple (possibly with sections/gaps)
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightParenChirho)
                    && !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_eof_chirho()
                {
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a list expression, arithmetic sequence, or list comprehension.
    fn parse_list_expr_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ListExprChirho);

        self.bump_chirho(); // [
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            // Empty list
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_expr_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            // List comprehension: [expr | quals]
            self.bump_chirho(); // |
            self.eat_trivia_chirho();
            // Parse qualifiers until ]
            // Each qualifier is either:
            //   - generator: pat <- expr
            //   - guard: expr
            //   - let: let binds
            // Separated by commas. A transform consumes a preceding statement
            // group; even a zero-column prefix needs an actual guard/let/bind.
            let mut preceding_qualifier_chirho = false;
            while !self.at_chirho(RawTokenKindChirho::RightBracketChirho)
                && !self.at_eof_chirho()
                && (!self.at_decl_boundary_chirho() || self.at_list_comp_let_layout_close_chirho())
            {
                let before_chirho = self.pos_chirho;
                if self.at_list_comp_let_layout_close_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    continue;
                }
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::PipeChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                    preceding_qualifier_chirho = false;
                }
                if !self.at_chirho(RawTokenKindChirho::RightBracketChirho)
                    && !self.at_eof_chirho()
                    && !self.at_decl_boundary_chirho()
                {
                    if self.at_chirho(RawTokenKindChirho::ThenChirho) {
                        if !preceding_qualifier_chirho {
                            self.builder_chirho
                                .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                            self.parse_comprehension_transform_chirho();
                            self.builder_chirho.finish_node_chirho();
                        } else {
                            self.parse_comprehension_transform_chirho();
                        }
                    } else if self.at_chirho(RawTokenKindChirho::LetChirho) {
                        self.builder_chirho
                            .start_node_chirho(SyntaxKindChirho::LetStmtChirho);
                        self.bump_chirho(); // let
                        self.eat_trivia_chirho();
                        self.parse_comprehension_let_binds_chirho();
                        self.builder_chirho.finish_node_chirho();
                        self.eat_trivia_chirho();
                    } else if self.scan_for_list_comp_bind_arrow_chirho() {
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
                }
                if self.pos_chirho == before_chirho {
                    break;
                }
                preceding_qualifier_chirho = true;
            }
        } else if self.at_chirho(RawTokenKindChirho::DotDotChirho) {
            // Arithmetic sequence: [a..], [a..b], [a,b..], [a,b..c]
            self.bump_chirho(); // ..
            self.eat_trivia_chirho();
            if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
            }
        } else if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // List or arithmetic sequence with step
            self.bump_chirho(); // ,
            self.eat_trivia_chirho();

            if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                self.parse_expr_chirho();
                self.eat_trivia_chirho();
            }

            if self.at_chirho(RawTokenKindChirho::DotDotChirho) {
                // [a, b .. c]
                self.bump_chirho();
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            } else {
                // Regular list: more elements
                while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho(); // ,
                    self.eat_trivia_chirho();
                    if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                        self.parse_expr_chirho();
                        self.eat_trivia_chirho();
                    }
                }
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Scan ahead to see whether the current list-comprehension qualifier
    /// contains a top-level `<-` before the next comma or `]`.
    fn scan_for_list_comp_bind_arrow_chirho(&self) -> bool {
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
                RawTokenKindChirho::CommaChirho
                | RawTokenKindChirho::PipeChirho
                | RawTokenKindChirho::RightBracketChirho
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
                _ => {}
            }
            i_chirho += 1;
        }

        false
    }

    /// workflow: parallel-list-comprehensions-chirho
    fn at_list_comp_let_layout_close_chirho(&self) -> bool {
        if !self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho) {
            return false;
        }
        let next_idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho + 1);
        self.tokens_chirho
            .get(next_idx_chirho)
            .is_some_and(|token_chirho| {
                matches!(
                    token_chirho.kind_chirho,
                    RawTokenKindChirho::CommaChirho
                        | RawTokenKindChirho::PipeChirho
                        | RawTokenKindChirho::RightBracketChirho
                )
            })
    }

    /// Parse record expression fields: { field = expr, ... }
    fn parse_record_expr_chirho(&mut self) {
        // We're inside a NameExpr that started with ConId.
        // Just eat the record fields.
        self.bump_chirho(); // {
        self.eat_trivia_chirho();

        while !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            && !self.at_eof_chirho()
            && !self.at_decl_boundary_chirho()
        {
            let outer_before_chirho = self.pos_chirho;
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::FieldAssignChirho);

            // Parse field name
            if self.at_chirho(RawTokenKindChirho::VarIdChirho)
                || self.at_chirho(RawTokenKindChirho::ConIdChirho)
                || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
            {
                self.bump_chirho(); // field name
                self.eat_trivia_chirho();
            }

            // Parse = and value expression
            if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
                self.bump_chirho(); // =
                self.eat_trivia_chirho();
                // Parse the value as a proper expression so lambdas,
                // let-expressions etc. get correct AST nodes.
                if self.can_start_aexp_chirho()
                    || self.at_chirho(RawTokenKindChirho::BackslashChirho)
                    || self.at_chirho(RawTokenKindChirho::LetChirho)
                    || self.at_chirho(RawTokenKindChirho::IfChirho)
                    || self.at_chirho(RawTokenKindChirho::CaseChirho)
                    || self.at_chirho(RawTokenKindChirho::DoChirho)
                {
                    self.parse_expr_chirho();
                    self.eat_trivia_chirho();
                }
            } else {
                // NamedFieldPuns or other tokens — consume until , or }
                while !self.at_chirho(RawTokenKindChirho::CommaChirho)
                    && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                    && !self.at_eof_chirho()
                    && !self.at_decl_boundary_chirho()
                {
                    self.eat_trivia_chirho();
                    if !self.at_chirho(RawTokenKindChirho::CommaChirho)
                        && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                        && !self.at_decl_boundary_chirho()
                        && !self.at_eof_chirho()
                    {
                        self.bump_chirho();
                    }
                }
            }

            self.builder_chirho.finish_node_chirho();

            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.pos_chirho == outer_before_chirho {
                break;
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightBraceChirho) {
            self.bump_chirho();
        }
    }

    /// Parse a layout block (shared by let, do, where, etc.)
    pub(super) fn parse_layout_block_chirho(&mut self) {
        if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
        {
            self.bump_chirho(); // {
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
                self.parse_decl_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    if !self.at_eof_chirho() {
                        self.builder_chirho
                            .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                        self.bump_chirho();
                        self.builder_chirho.finish_node_chirho();
                    } else {
                        break;
                    }
                }
            }
        }
    }

    /// Can the current token start an atomic expression?
    fn can_start_aexp_chirho(&self) -> bool {
        matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::TickChirho)
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::FloatLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
                // BlockArguments: block-like expressions as function arguments
                | Some(RawTokenKindChirho::BackslashChirho)
                | Some(RawTokenKindChirho::DoChirho)
                | Some(RawTokenKindChirho::CaseChirho)
                | Some(RawTokenKindChirho::IfChirho)
                | Some(RawTokenKindChirho::LetChirho)
                // Template Haskell splice and quotation forms
                | Some(RawTokenKindChirho::ThSpliceChirho)
                | Some(RawTokenKindChirho::ThTypedSpliceChirho)
                | Some(RawTokenKindChirho::ThOpenExpQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenExpExplicitQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenDecQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenTypeQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenPatQuoteChirho)
                | Some(RawTokenKindChirho::ThOpenTypedExpQuoteChirho)
                // TypedHoles: `_` in expression position
                | Some(RawTokenKindChirho::UnderscoreChirho)
        ) && !self.current_is_qualified_operator_chirho()
    }

    /// Is the current token a field strictness annotation: `!` (strict) or `~`
    /// (explicitly lazy, which GHC allows under `StrictData`; validity judges
    /// that)? Constructor declarations read it before a field type.
    pub(super) fn at_strictness_prefix_chirho(&self) -> bool {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarSymChirho) => self.current_text_chirho() == "!",
            Some(RawTokenKindChirho::TildeChirho) => true,
            _ => false,
        }
    }
}
