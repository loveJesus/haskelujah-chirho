// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Promotion owns its delimiters and every operand. Workflow: flat-type-syntax-chirho.
use super::*;

impl<'source_chirho> ParserChirho<'source_chirho> {
    pub(super) fn parse_promoted_type_chirho(&mut self) {
        match self.peek_after_tick_chirho() {
            Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho)
            | Some(RawTokenKindChirho::ConSymChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedConTypeChirho);
                self.bump_chirho();
                self.eat_trivia_chirho();
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedListTypeChirho);
                self.bump_chirho();
                self.eat_trivia_chirho();
                self.bump_chirho();
                self.eat_trivia_chirho();
                if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.parse_type_chirho();
                    self.eat_trivia_chirho();
                    while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                        self.bump_chirho();
                        self.eat_trivia_chirho();
                        self.parse_type_chirho();
                        self.eat_trivia_chirho();
                    }
                }
                if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => self.parse_promoted_parenthesized_chirho(),
            _ => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    fn parse_promoted_parenthesized_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::PromotedConTypeChirho);
        self.bump_chirho();
        self.eat_trivia_chirho();
        self.bump_chirho();
        self.eat_trivia_chirho();
        if self.current_chirho().is_some_and(|token_chirho| {
            matches!(
                map_token_kind_chirho(token_chirho.kind_chirho, self.current_text_chirho()),
                TokenKindChirho::ConSymChirho | TokenKindChirho::QualifiedConSymChirho
            )
        }) && self.is_operator_section_chirho()
        {
            // '(:) and '(Module.:*) are promoted symbols, not unary tuples.
            self.bump_chirho();
            self.eat_trivia_chirho();
        } else if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // A constructor section contains separators, not missing operands.
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
        } else if !self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            let first_chirho = self.builder_chirho.checkpoint_chirho();
            self.parse_type_with_ascription_chirho();
            self.eat_trivia_chirho();
            if !self.at_chirho(RawTokenKindChirho::CommaChirho) {
                // '(a) is neither unit nor a tuple; retain a visible CST error.
                self.builder_chirho
                    .start_node_at_chirho(first_chirho, SyntaxKindChirho::ErrorNodeChirho);
                self.builder_chirho.finish_node_chirho();
            }
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                let comma_chirho = self.builder_chirho.checkpoint_chirho();
                self.bump_chirho();
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.builder_chirho
                        .start_node_at_chirho(comma_chirho, SyntaxKindChirho::ErrorNodeChirho);
                    self.builder_chirho.finish_node_chirho();
                    break;
                }
                let before_chirho = self.pos_chirho;
                self.parse_type_with_ascription_chirho();
                self.eat_trivia_chirho();
                if before_chirho == self.pos_chirho {
                    break;
                }
            }
        }
        self.expect_chirho(RawTokenKindChirho::RightParenChirho);
        self.builder_chirho.finish_node_chirho();
    }
}
