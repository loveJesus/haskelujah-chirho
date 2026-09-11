// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family syntax owns its head, result contract and equations separately.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;

impl<'source_chirho> ParserChirho<'source_chirho> {
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
