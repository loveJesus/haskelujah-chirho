// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Parenthesized operators are complete alternatives, not speculative prefixes.
//! Workflow: language-features-chirho/flat-type-syntax-chirho.
use super::{ParserChirho, RawTokenKindChirho, SyntaxKindChirho};

impl<'source_chirho> ParserChirho<'source_chirho> {
    /// Preserve every operand when a group starts with a symbolic type atom.
    /// In `(* -> *)`, the first star belongs to the ordinary type grammar;
    /// only a single operator followed by `)` is a constructor-only group.
    pub(super) fn parse_paren_type_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenTypeChirho);
        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        let single_operator_chirho = matches!(
            self.current_kind_chirho(),
            Some(RawTokenKindChirho::RightArrowChirho)
                | Some(RawTokenKindChirho::VarSymChirho)
                | Some(RawTokenKindChirho::ConSymChirho)
        ) && self
            .tokens_chirho
            .get(self.skip_trivia_idx_chirho(self.pos_chirho + 1))
            .is_some_and(|token_chirho| {
                token_chirho.kind_chirho == RawTokenKindChirho::RightParenChirho
            });
        if single_operator_chirho {
            self.bump_chirho();
            self.eat_trivia_chirho();
            self.bump_chirho(); // )
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_type_with_ascription_chirho();
        while self.at_chirho(RawTokenKindChirho::CommaChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            self.parse_type_chirho();
            self.eat_trivia_chirho();
        }
        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }
        self.builder_chirho.finish_node_chirho();
    }
}
