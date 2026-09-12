// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor headers, fields and GADT syntax share one declaration boundary.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;

impl<'source_chirho> ParserChirho<'source_chirho> {
    pub(super) fn parse_con_decl_chirho(&mut self) {
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
        }

        self.parse_optional_constructor_context_chirho();

        let starts_infix_con_decl_chirho = self.starts_infix_con_decl_chirho();

        if starts_infix_con_decl_chirho {
            while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                let before_chirho = self.pos_chirho;
                if self.at_strict_prefix_chirho() {
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
                while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    if self.at_strict_prefix_chirho() {
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
                // Handle strictness annotations: `!` before a field type
                while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    // Consume `!` strictness annotation if present
                    if self.at_strict_prefix_chirho() {
                        self.bump_chirho(); // !
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
        } else if self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
            // Infix constructor declaration: `!a :*: !b`
            let before_left_chirho = self.pos_chirho;
            if self.at_strict_prefix_chirho() {
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
                if self.at_strict_prefix_chirho() {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                // Right operand may be a type application (e.g. `Seq a`
                // in `a :< Seq a`), so parse all atomic types, not just one.
                while self.can_start_atype_chirho() || self.at_strict_prefix_chirho() {
                    let before_chirho = self.pos_chirho;
                    if self.at_strict_prefix_chirho() {
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
    pub(super) fn parse_gadt_con_decl_chirho(&mut self) {
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

    pub(super) fn parse_record_fields_chirho(&mut self) {
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

    /// Locate only a top-level constructor context. Looking ahead is bounded by
    /// this declaration; a => inside a rank-n field is not the header separator.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    fn parse_optional_constructor_context_chirho(&mut self) {
        let mut depth_chirho = 0usize;
        let mut arrow_chirho = None;
        for index_chirho in self.pos_chirho..self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[index_chirho].kind_chirho;
            match kind_chirho {
                RawTokenKindChirho::FatArrowChirho if depth_chirho == 0 => {
                    arrow_chirho = Some(index_chirho);
                    break;
                }
                RawTokenKindChirho::RightArrowChirho
                | RawTokenKindChirho::PipeChirho
                | RawTokenKindChirho::DerivingChirho
                | RawTokenKindChirho::SemicolonChirho
                | RawTokenKindChirho::VirtualSemicolonChirho
                | RawTokenKindChirho::VirtualRightBraceChirho
                | RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::LeftBraceChirho
                    if depth_chirho == 0 =>
                {
                    break;
                }
                RawTokenKindChirho::LeftParenChirho | RawTokenKindChirho::LeftBracketChirho => {
                    depth_chirho += 1
                }
                RawTokenKindChirho::RightParenChirho | RawTokenKindChirho::RightBracketChirho => {
                    let Some(outer_chirho) = depth_chirho.checked_sub(1) else {
                        break;
                    };
                    depth_chirho = outer_chirho;
                }
                _ => {}
            }
        }
        if let Some(arrow_chirho) = arrow_chirho {
            while self.pos_chirho <= arrow_chirho && !self.at_eof_chirho() {
                self.bump_chirho();
            }
            self.eat_trivia_chirho();
        }
    }
}
