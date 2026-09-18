// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The enclosing declaration determines whether `type` starts a head or an equation.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;

impl<'source_chirho> ParserChirho<'source_chirho> {
    pub(super) fn parse_where_block_context_chirho(&mut self, parse_type_chirho: fn(&mut Self)) {
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
                if self.at_chirho(RawTokenKindChirho::TypeChirho) {
                    parse_type_chirho(self);
                } else {
                    self.parse_decl_chirho();
                }
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
}
