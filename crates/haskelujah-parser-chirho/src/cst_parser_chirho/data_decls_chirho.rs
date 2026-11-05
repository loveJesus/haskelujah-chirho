// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `data` and `newtype` declarations: constructors, GADT blocks, records and deriving clauses.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Data type declarations
    // -----------------------------------------------------------------------

    pub(super) fn parse_data_decl_chirho(&mut self) {
        // Check for `data family` or `data instance` by peeking ahead
        let mut look_chirho = self.pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[look_chirho].kind_chirho,
                RawTokenKindChirho::WhitespaceChirho
                    | RawTokenKindChirho::LineCommentChirho
                    | RawTokenKindChirho::BlockCommentChirho
            )
        {
            look_chirho += 1;
        }
        let next_text_chirho = if look_chirho < self.tokens_chirho.len() {
            self.token_text_chirho(&self.tokens_chirho[look_chirho])
        } else {
            ""
        };
        if next_text_chirho == "family" {
            // The AST currently represents open type and data-family headers
            // with the same family declaration shape. Preserve this header so
            // later namespace and kind passes can bind the family tycon.
            self.parse_type_family_decl_chirho();
            return;
        }
        if next_text_chirho == "instance" {
            // Data-family instances need their own constructor-bearing AST
            // shape; keep skipping them rather than fabricating a data type,
            // but stop at the end of their own layout block so the next
            // top-level declaration survives.
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::DataDeclChirho);
            self.eat_until_unrepresented_instance_end_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DataDeclChirho);

        self.expect_chirho(RawTokenKindChirho::DataChirho);
        self.eat_trivia_chirho();

        // Type name and type variables until = or where or deriving
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::DerivingChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        // = Constructor | Constructor ... (ADT syntax)
        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();

            self.parse_con_decl_chirho();
            self.eat_trivia_chirho();

            while self.at_chirho(RawTokenKindChirho::PipeChirho) {
                self.bump_chirho(); // |
                self.eat_trivia_chirho();
                self.parse_con_decl_chirho();
                self.eat_trivia_chirho();
            }
        } else if self.at_chirho(RawTokenKindChirho::WhereChirho)
            || matches!(
                self.previous_non_trivia_kind_chirho(),
                Some(RawTokenKindChirho::WhereChirho)
            )
            || self.starts_gadt_con_decl_chirho()
        {
            self.parse_gadt_constructor_block_chirho();
        }

        self.parse_deriving_clauses_chirho();

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse the shared GADT constructor block used by `data` and `newtype`.
    fn parse_gadt_constructor_block_chirho(&mut self) {
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.bump_chirho(); // where
            self.eat_trivia_chirho();
        }

        let has_brace_chirho = self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
            || self.at_chirho(RawTokenKindChirho::LeftBraceChirho);
        if has_brace_chirho {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        loop {
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.bump_chirho();
                break;
            }
            if self.at_eof_chirho() || self.at_chirho(RawTokenKindChirho::DerivingChirho) {
                break;
            }
            if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
                continue;
            }

            if self.starts_gadt_con_decl_chirho() {
                self.parse_gadt_con_decl_chirho();
            } else {
                let before_chirho = self.pos_chirho;
                self.bump_chirho();
                self.eat_trivia_chirho();
                if self.pos_chirho == before_chirho {
                    break;
                }
            }
            self.eat_trivia_chirho();
        }
    }

    fn parse_con_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ConDeclChirho);

        // ExistentialQuantification: `forall a. Ctx => Con ...`
        if self.at_chirho(RawTokenKindChirho::ForallChirho) {
            self.bump_chirho(); // forall
            self.eat_trivia_chirho();
            // Eat type variables until dot
            while self.at_chirho(RawTokenKindChirho::VarIdChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            // Expect '.'
            if self.at_dot_chirho() {
                self.bump_chirho(); // .
                self.eat_trivia_chirho();
            }
            // Optional context: `Show a =>`
            // Check for `ConId VarId ... =>` pattern
            let _save_pos_chirho = self.pos_chirho;
            let mut found_arrow_chirho = false;
            let mut lookahead_chirho = self.pos_chirho;
            let mut delimiter_depth_chirho = 0usize;
            while lookahead_chirho < self.tokens_chirho.len() {
                let tk_chirho = self.tokens_chirho[lookahead_chirho].kind_chirho;
                match tk_chirho {
                    RawTokenKindChirho::LeftParenChirho
                    | RawTokenKindChirho::LeftBracketChirho
                    | RawTokenKindChirho::LeftBraceChirho => delimiter_depth_chirho += 1,
                    RawTokenKindChirho::RightParenChirho
                    | RawTokenKindChirho::RightBracketChirho
                    | RawTokenKindChirho::RightBraceChirho => {
                        delimiter_depth_chirho = delimiter_depth_chirho.saturating_sub(1);
                    }
                    RawTokenKindChirho::FatArrowChirho if delimiter_depth_chirho == 0 => {
                        found_arrow_chirho = true;
                        break;
                    }
                    RawTokenKindChirho::RightArrowChirho
                    | RawTokenKindChirho::VirtualSemicolonChirho
                    | RawTokenKindChirho::PipeChirho
                        if delimiter_depth_chirho == 0 =>
                    {
                        break;
                    }
                    _ => {}
                }
                lookahead_chirho += 1;
            }
            if found_arrow_chirho {
                // Parse constraint(s) before =>
                while !self.at_chirho(RawTokenKindChirho::FatArrowChirho) && !self.at_eof_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                    self.bump_chirho(); // =>
                    self.eat_trivia_chirho();
                }
            }
        }

        let starts_infix_con_decl_chirho = self.starts_infix_con_decl_chirho();

        if starts_infix_con_decl_chirho {
            while self.can_start_atype_chirho() || self.at_strictness_prefix_chirho() {
                let before_chirho = self.pos_chirho;
                if self.at_strictness_prefix_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.can_start_atype_chirho() {
                    self.parse_atype_chirho();
                    self.eat_trivia_chirho();
                }
                let lookahead_idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho);
                if self
                    .tokens_chirho
                    .get(lookahead_idx_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::ConSymChirho
                    })
                {
                    break;
                }
                if self.pos_chirho == before_chirho {
                    break;
                }
            }

            if self.at_chirho(RawTokenKindChirho::ConSymChirho) {
                self.bump_chirho(); // constructor operator
                self.eat_trivia_chirho();
                while self.can_start_atype_chirho() || self.at_strictness_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    if self.at_strictness_prefix_chirho() {
                        self.bump_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.can_start_atype_chirho() {
                        self.parse_atype_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.pos_chirho == before_chirho {
                        break;
                    }
                }
            }
        // Prefix constructor name
        } else if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();

            // Record syntax: { field :: Type, ... }
            if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
                self.parse_record_fields_chirho();
            } else {
                // Ordinary constructor: parse atomic types as fields
                // Handle strictness annotations: `!` or `~` before a field type
                while self.can_start_atype_chirho() || self.at_strictness_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    // Consume a `!` or `~` strictness annotation if present
                    if self.at_strictness_prefix_chirho() {
                        self.bump_chirho(); // ! or ~
                        self.eat_trivia_chirho();
                    }
                    if self.can_start_atype_chirho() {
                        self.parse_atype_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.pos_chirho == before_chirho {
                        break;
                    }
                }
            }
        } else if self.can_start_atype_chirho() || self.at_strictness_prefix_chirho() {
            // Infix constructor declaration: `!a :*: !b`
            let before_left_chirho = self.pos_chirho;
            if self.at_strictness_prefix_chirho() {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.can_start_atype_chirho() {
                self.parse_atype_chirho();
                self.eat_trivia_chirho();
            }

            if self.pos_chirho != before_left_chirho
                && self.at_chirho(RawTokenKindChirho::ConSymChirho)
            {
                self.bump_chirho(); // constructor operator
                self.eat_trivia_chirho();
                if self.at_strictness_prefix_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                // Right operand may be a type application (e.g. `Seq a`
                // in `a :< Seq a`), so parse all atomic types, not just one.
                while self.can_start_atype_chirho() || self.at_strictness_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    if self.at_strictness_prefix_chirho() {
                        self.bump_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.can_start_atype_chirho() {
                        self.parse_atype_chirho();
                        self.eat_trivia_chirho();
                    }
                    if self.pos_chirho == before_chirho {
                        break;
                    }
                }
            }
        } else {
            // Fallback: eat until boundary
            self.eat_until_any_chirho(&[
                RawTokenKindChirho::PipeChirho,
                RawTokenKindChirho::DerivingChirho,
                RawTokenKindChirho::VirtualSemicolonChirho,
                RawTokenKindChirho::VirtualRightBraceChirho,
                RawTokenKindChirho::SemicolonChirho,
                RawTokenKindChirho::RightBraceChirho,
            ]);
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn starts_infix_con_decl_chirho(&self) -> bool {
        let mut idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho);

        loop {
            if self
                .tokens_chirho
                .get(idx_chirho)
                .is_some_and(|token_chirho| {
                    token_chirho.kind_chirho == RawTokenKindChirho::VarSymChirho
                        && self.token_text_chirho(token_chirho) == "!"
                })
            {
                idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
            }

            let Some(after_atype_chirho) = self.peek_after_apat_chirho(idx_chirho) else {
                return false;
            };

            let lookahead_idx_chirho = self.skip_trivia_idx_chirho(after_atype_chirho);
            let Some(lookahead_token_chirho) = self.tokens_chirho.get(lookahead_idx_chirho) else {
                return false;
            };

            if lookahead_token_chirho.kind_chirho == RawTokenKindChirho::ConSymChirho {
                return true;
            }

            if !self.can_start_atype_idx_chirho(lookahead_idx_chirho) {
                return false;
            }

            idx_chirho = lookahead_idx_chirho;
        }
    }

    /// Parse a GADT constructor declaration: `Con :: forall a. Ctx => Arg -> ... -> T a`
    fn parse_gadt_con_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::GadtConDeclChirho);

        // Constructor name(s) — could be `C1, C2 :: Type`
        if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
        } else if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::ConSymChirho)
                || self.at_chirho(RawTokenKindChirho::ConIdChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
        }
        self.eat_trivia_chirho();

        // Expect ::
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
        }

        // GADT record syntax: `MkT :: { f :: Int, x :: Char } -> T`. The braces are
        // NOT a type, so `parse_type_chirho` cannot read them — it meets `{` in type
        // position and the damage cascades past the declaration, taking the rest of
        // the module's bindings with it. Parse the fields with the same routine the
        // ordinary record path uses, then the `->` and the result type.
        if self.at_chirho(RawTokenKindChirho::LeftBraceChirho) {
            self.parse_record_fields_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
                self.parse_type_chirho();
            }
        } else {
            // Parse the type signature
            self.parse_type_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_record_fields_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::RecordFieldsChirho);

        self.expect_chirho(RawTokenKindChirho::LeftBraceChirho);
        self.eat_trivia_chirho();

        while !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            && !self.at_decl_boundary_chirho()
            && !self.at_eof_chirho()
        {
            let outer_before_chirho = self.pos_chirho;
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::FieldDeclChirho);
            let mut paren_depth_chirho = 0usize;
            let mut bracket_depth_chirho = 0usize;
            let mut brace_depth_chirho = 0usize;

            // field names, ::, type
            while !self.at_decl_boundary_chirho() && !self.at_eof_chirho() {
                self.eat_trivia_chirho();
                if self.at_decl_boundary_chirho() || self.at_eof_chirho() {
                    break;
                }

                let should_end_field_chirho =
                    match self.current_chirho().map(|t_chirho| t_chirho.kind_chirho) {
                        Some(RawTokenKindChirho::CommaChirho) => {
                            paren_depth_chirho == 0
                                && bracket_depth_chirho == 0
                                && brace_depth_chirho == 0
                        }
                        Some(RawTokenKindChirho::RightBraceChirho) => {
                            paren_depth_chirho == 0
                                && bracket_depth_chirho == 0
                                && brace_depth_chirho == 0
                        }
                        _ => false,
                    };
                if should_end_field_chirho {
                    break;
                }

                match self.current_chirho().map(|t_chirho| t_chirho.kind_chirho) {
                    Some(RawTokenKindChirho::LeftParenChirho) => paren_depth_chirho += 1,
                    Some(RawTokenKindChirho::RightParenChirho) => {
                        paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                    }
                    Some(RawTokenKindChirho::LeftBracketChirho) => bracket_depth_chirho += 1,
                    Some(RawTokenKindChirho::RightBracketChirho) => {
                        bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                    }
                    Some(RawTokenKindChirho::LeftBraceChirho) => brace_depth_chirho += 1,
                    Some(RawTokenKindChirho::RightBraceChirho) => {
                        brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                    }
                    _ => {}
                }

                self.bump_chirho();
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

        self.builder_chirho.finish_node_chirho();
    }

    /// Every deriving clause of one `data` or `newtype` declaration: GHC allows any number,
    /// each with its own strategy (`deriving stock Eq` then `deriving newtype Num`).
    /// workflow: language-features-chirho/deriving-strategies-chirho
    fn parse_deriving_clauses_chirho(&mut self) {
        self.eat_trivia_chirho();
        while self.at_chirho(RawTokenKindChirho::DerivingChirho) {
            self.parse_deriving_clause_chirho();
            // Trivia between two clauses belongs to the declaration, like the trivia
            // before the first one; trivia after the last clause stays outside it.
            let next_chirho = self.skip_trivia_idx_chirho(self.pos_chirho);
            let another_clause_chirho =
                self.tokens_chirho
                    .get(next_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::DerivingChirho
                    });
            if !another_clause_chirho {
                break;
            }
            self.eat_trivia_chirho();
        }
    }

    /// Whether the current token is a `DerivingStrategies` strategy keyword. `stock` and
    /// `anyclass` lex as variable identifiers, but `newtype` is a reserved word and always
    /// arrives as its keyword token.
    pub(super) fn at_deriving_strategy_chirho(&self) -> bool {
        self.at_varid_text_chirho("stock")
            || self.at_varid_text_chirho("anyclass")
            || self.at_chirho(RawTokenKindChirho::NewtypeChirho)
    }

    fn parse_deriving_clause_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DerivingClauseChirho);

        self.expect_chirho(RawTokenKindChirho::DerivingChirho);
        self.eat_trivia_chirho();

        // DerivingStrategies: an unconsumed `newtype` would end this clause and start a
        // separate `newtype` declaration named after the class (row 694).
        if self.at_deriving_strategy_chirho() {
            self.bump_chirho(); // strategy keyword
            self.eat_trivia_chirho();
        }

        // deriving (Show, C (Maybe Int)) or deriving Show. A class may take parenthesized
        // arguments, so the list ends at the parenthesis that closes it. A layout token
        // ends the clause even inside an unclosed list, so a malformed clause cannot
        // swallow the next declaration.
        if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            self.bump_chirho();
            let mut depth_chirho = 1usize;
            while depth_chirho > 0 {
                self.eat_trivia_chirho();
                if self.at_eof_chirho()
                    || self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                {
                    break;
                }
                if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                    depth_chirho += 1;
                } else if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    depth_chirho -= 1;
                }
                self.bump_chirho();
            }
        } else if self.at_chirho(RawTokenKindChirho::ConIdChirho)
            || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
        {
            self.bump_chirho();
        }

        // DerivingVia: `deriving (Class) via Type`
        // Check for `via` keyword after the class list.
        self.eat_trivia_chirho();
        if self.at_varid_text_chirho("via") {
            self.bump_chirho(); // consume "via"
            self.eat_trivia_chirho();
            // Parse the via type (could be a ConId, or parenthesized, etc.)
            self.parse_atype_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Newtype declarations
    // -----------------------------------------------------------------------

    pub(super) fn parse_newtype_decl_chirho(&mut self) {
        // Check for `newtype instance` — data family instance with newtype
        let mut look_chirho = self.pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[look_chirho].kind_chirho,
                RawTokenKindChirho::WhitespaceChirho
                    | RawTokenKindChirho::LineCommentChirho
                    | RawTokenKindChirho::BlockCommentChirho
            )
        {
            look_chirho += 1;
        }
        if look_chirho < self.tokens_chirho.len()
            && self.token_text_chirho(&self.tokens_chirho[look_chirho]) == "instance"
        {
            // newtype instance — parse as a skipped decl without consuming a
            // following top-level declaration.
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::NewtypeDeclChirho);
            self.eat_until_unrepresented_instance_end_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::NewtypeDeclChirho);

        self.expect_chirho(RawTokenKindChirho::NewtypeChirho);
        self.eat_trivia_chirho();

        // Type name and variables until an ADT or GADT constructor form.
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_con_decl_chirho();
        } else if self.at_chirho(RawTokenKindChirho::WhereChirho)
            || matches!(
                self.previous_non_trivia_kind_chirho(),
                Some(RawTokenKindChirho::WhereChirho)
            )
            || self.starts_gadt_con_decl_chirho()
        {
            self.parse_gadt_constructor_block_chirho();
        }

        self.parse_deriving_clauses_chirho();

        self.builder_chirho.finish_node_chirho();
    }

    fn starts_gadt_con_decl_chirho(&self) -> bool {
        let mut idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho);
        if idx_chirho >= self.tokens_chirho.len() {
            return false;
        }

        if self.tokens_chirho[idx_chirho].kind_chirho == RawTokenKindChirho::WhereChirho {
            idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
        }

        if idx_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[idx_chirho].kind_chirho,
                RawTokenKindChirho::VirtualLeftBraceChirho | RawTokenKindChirho::LeftBraceChirho
            )
        {
            idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
        }

        while idx_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[idx_chirho].kind_chirho,
                RawTokenKindChirho::VirtualSemicolonChirho | RawTokenKindChirho::SemicolonChirho
            )
        {
            idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
        }

        let Some(token_chirho) = self.tokens_chirho.get(idx_chirho) else {
            return false;
        };

        match token_chirho.kind_chirho {
            RawTokenKindChirho::ConIdChirho => {
                let next_idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
                self.tokens_chirho
                    .get(next_idx_chirho)
                    .is_some_and(|next_token_chirho| {
                        next_token_chirho.kind_chirho == RawTokenKindChirho::ColonColonChirho
                    })
            }
            RawTokenKindChirho::LeftParenChirho => {
                let sym_idx_chirho = self.skip_trivia_idx_chirho(idx_chirho + 1);
                let close_idx_chirho = self.skip_trivia_idx_chirho(sym_idx_chirho + 1);
                let colon_idx_chirho = self.skip_trivia_idx_chirho(close_idx_chirho + 1);
                self.tokens_chirho
                    .get(sym_idx_chirho)
                    .is_some_and(|sym_token_chirho| {
                        matches!(
                            sym_token_chirho.kind_chirho,
                            RawTokenKindChirho::ConSymChirho | RawTokenKindChirho::ConIdChirho
                        )
                    })
                    && self
                        .tokens_chirho
                        .get(close_idx_chirho)
                        .is_some_and(|close_token_chirho| {
                            close_token_chirho.kind_chirho == RawTokenKindChirho::RightParenChirho
                        })
                    && self
                        .tokens_chirho
                        .get(colon_idx_chirho)
                        .is_some_and(|colon_token_chirho| {
                            colon_token_chirho.kind_chirho == RawTokenKindChirho::ColonColonChirho
                        })
            }
            _ => false,
        }
    }
}
