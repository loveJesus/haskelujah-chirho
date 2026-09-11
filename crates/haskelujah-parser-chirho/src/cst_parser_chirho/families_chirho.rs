// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family syntax owns its head, result contract and equations separately.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;

impl<'source_chirho> ParserChirho<'source_chirho> {
    /// Classify the declaration by its complete name, not a one-token guess.
    /// The lookahead consumes only a name and `::`; it cannot mistake a
    /// parameter annotation or a later declaration's signature for this one.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn parse_type_or_family_decl_chirho(&mut self) {
        let name_index_chirho = self.skip_trivia_idx_chirho(self.pos_chirho + 1);
        let next_text_chirho = self
            .tokens_chirho
            .get(name_index_chirho)
            .map(|token_chirho| self.token_text_chirho(token_chirho))
            .unwrap_or("");
        match next_text_chirho {
            "family" => self.parse_type_family_decl_chirho(),
            "instance" => self.parse_type_family_instance_decl_chirho(),
            _ if next_text_chirho == "role"
                || self.at_standalone_kind_head_chirho(name_index_chirho) =>
            {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);
                self.eat_until_decl_end_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => self.parse_type_alias_decl_chirho(),
        }
    }

    fn at_standalone_kind_head_chirho(&self, name_index_chirho: usize) -> bool {
        let Some(name_chirho) = self.tokens_chirho.get(name_index_chirho) else {
            return false;
        };
        let after_name_chirho = match name_chirho.kind_chirho {
            RawTokenKindChirho::ConIdChirho => name_index_chirho + 1,
            RawTokenKindChirho::LeftParenChirho => {
                let operator_index_chirho = self.skip_trivia_idx_chirho(name_index_chirho + 1);
                if !self
                    .tokens_chirho
                    .get(operator_index_chirho)
                    .is_some_and(|token_chirho| {
                        matches!(
                            token_chirho.kind_chirho,
                            RawTokenKindChirho::VarSymChirho
                                | RawTokenKindChirho::ConSymChirho
                                | RawTokenKindChirho::TildeChirho
                        )
                    })
                {
                    return false;
                }
                let close_index_chirho = self.skip_trivia_idx_chirho(operator_index_chirho + 1);
                if !self
                    .tokens_chirho
                    .get(close_index_chirho)
                    .is_some_and(|token_chirho| {
                        token_chirho.kind_chirho == RawTokenKindChirho::RightParenChirho
                    })
                {
                    return false;
                }
                close_index_chirho + 1
            }
            _ => return false,
        };
        self.tokens_chirho
            .get(self.skip_trivia_idx_chirho(after_name_chirho))
            .is_some_and(|token_chirho| {
                token_chirho.kind_chirho == RawTokenKindChirho::ColonColonChirho
            })
    }

    pub(super) fn parse_type_family_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyDeclChirho);

        if self.at_chirho(RawTokenKindChirho::TypeChirho) {
            self.bump_chirho(); // type
        } else {
            self.expect_chirho(RawTokenKindChirho::DataChirho); // data
        }
        self.eat_trivia_chirho();
        self.bump_chirho(); // family
        self.eat_trivia_chirho();

        // Family name and type variables until `where`, `::`, or end of decl
        self.eat_until_any_top_level_chirho(&[
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::ColonColonChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        // Optional result kind annotation `:: *`
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // kind
        }

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.parse_family_result_chirho();
        }

        // Check for `where` (closed type family)
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.bump_chirho(); // where
            self.eat_trivia_chirho();
            // Parse equations: each is `F lhs_types = rhs_type`
            if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
                || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
            {
                self.bump_chirho();
            }
            while !self.at_eof_chirho()
                && !self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
                {
                    self.bump_chirho();
                    continue;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                    || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                {
                    break;
                }
                // Patterns use the same type grammar as their RHS. Retain
                // promoted lists, literals and grouped applications as nodes;
                // lowering must not reconstruct a second grammar from tokens.
                // Workflow: language-features-chirho/declaration-kinds-chirho.
                let before_chirho = self.pos_chirho;
                self.parse_type_chirho(); // complete family application
                if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
                    self.bump_chirho(); // =
                    self.eat_trivia_chirho();
                    self.parse_type_chirho(); // rhs
                }
                if self.pos_chirho == before_chirho {
                    // A malformed pattern may start with a token that cannot
                    // begin a type. Keep recovery bounded and visible.
                    self.builder_chirho
                        .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                    self.bump_chirho();
                    self.builder_chirho.finish_node_chirho();
                }
            }
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.bump_chirho();
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    pub(super) fn parse_type_family_instance_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyInstanceDeclChirho);

        self.expect_chirho(RawTokenKindChirho::TypeChirho); // type
        self.eat_trivia_chirho();
        self.bump_chirho(); // instance
        self.eat_trivia_chirho();

        // Open instances and closed equations share the type grammar.
        self.parse_type_chirho(); // complete family application

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // RHS type
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_family_result_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyResultChirho);
        self.bump_chirho(); // =
        self.eat_trivia_chirho();
        let parenthesized_chirho = self.at_chirho(RawTokenKindChirho::LeftParenChirho);
        if parenthesized_chirho {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }
        self.expect_chirho(RawTokenKindChirho::VarIdChirho);
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            self.parse_type_chirho();
        }
        if parenthesized_chirho {
            self.expect_chirho(RawTokenKindChirho::RightParenChirho);
            self.eat_trivia_chirho();
        }
        if self.at_chirho(RawTokenKindChirho::PipeChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            self.expect_chirho(RawTokenKindChirho::VarIdChirho);
            self.eat_trivia_chirho();
            self.expect_chirho(RawTokenKindChirho::RightArrowChirho);
            self.eat_trivia_chirho();
            self.expect_chirho(RawTokenKindChirho::VarIdChirho);
            self.eat_trivia_chirho();
            while self.at_chirho(RawTokenKindChirho::VarIdChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
        }
        self.builder_chirho.finish_node_chirho();
    }
}
