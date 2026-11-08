// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC9.14.1's four transform forms share ordinary expression parsing.
//! Effective lexical by/using boundaries own delimiters; tokens never mutate.
use super::*;

impl ParserChirho<'_> {
    pub(super) fn parse_comprehension_transform_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ComprehensionTransformChirho);
        self.bump_chirho(); // actual then
        self.eat_trivia_chirho();
        let after_group_chirho = self.skip_trivia_idx_chirho(self.pos_chirho.saturating_add(1));
        let group_chirho = self.at_varid_text_chirho("group")
            && self
                .tokens_chirho
                .get(after_group_chirho)
                .is_some_and(|token_chirho| {
                    matches!(
                        token_chirho.kind_chirho,
                        RawTokenKindChirho::TransformByChirho
                            | RawTokenKindChirho::TransformUsingChirho
                    )
                });
        if group_chirho {
            self.bump_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::TransformByChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
                self.parse_comprehension_component_chirho(SyntaxKindChirho::ComprehensionByChirho);
                self.eat_trivia_chirho();
            }
            self.expect_chirho(RawTokenKindChirho::TransformUsingChirho);
            self.eat_trivia_chirho();
            self.parse_comprehension_component_chirho(SyntaxKindChirho::ComprehensionUsingChirho);
        } else {
            // group is also a genuine ordinary identifier in this alternative.
            self.parse_comprehension_component_chirho(SyntaxKindChirho::ComprehensionUsingChirho);
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::TransformByChirho) {
                self.bump_chirho();
                self.eat_trivia_chirho();
                self.parse_comprehension_component_chirho(SyntaxKindChirho::ComprehensionByChirho);
            }
        }
        self.builder_chirho.finish_node_chirho();
    }

    fn parse_comprehension_component_chirho(&mut self, kind_chirho: SyntaxKindChirho) {
        self.builder_chirho.start_node_chirho(kind_chirho);
        let start_chirho = self.pos_chirho;
        if self.can_start_aexp_chirho()
            || matches!(
                self.current_kind_chirho(),
                Some(
                    RawTokenKindChirho::BackslashChirho
                        | RawTokenKindChirho::IfChirho
                        | RawTokenKindChirho::LetChirho
                        | RawTokenKindChirho::CaseChirho
                        | RawTokenKindChirho::DoChirho
                )
            )
        {
            self.parse_expr_chirho();
        }
        if self.pos_chirho == start_chirho {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
            self.builder_chirho.finish_node_chirho();
        }
        self.builder_chirho.finish_node_chirho();
    }
}
