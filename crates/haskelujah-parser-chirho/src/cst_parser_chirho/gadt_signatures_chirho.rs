// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor-only field syntax on a GADT signature's outer arrow spine.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

impl ParserChirho<'_> {
    /// Retain a complete kind-annotated forall binder without restricting its
    /// kind grammar to the subset AstKind can currently represent.
    pub(super) fn has_kind_annotated_forall_binder_chirho(&self) -> bool {
        if !self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
            return false;
        }
        let variable_chirho = self.skip_trivia_idx_chirho(self.pos_chirho + 1);
        let colon_chirho = self.skip_trivia_idx_chirho(variable_chirho + 1);
        if self
            .tokens_chirho
            .get(variable_chirho)
            .map(|token_chirho| token_chirho.kind_chirho)
            != Some(RawTokenKindChirho::VarIdChirho)
            || self
                .tokens_chirho
                .get(colon_chirho)
                .map(|token_chirho| token_chirho.kind_chirho)
                != Some(RawTokenKindChirho::ColonColonChirho)
        {
            return false;
        }
        let mut depth_chirho = 1_usize;
        for token_chirho in &self.tokens_chirho[colon_chirho + 1..] {
            match token_chirho.kind_chirho {
                RawTokenKindChirho::LeftParenChirho => depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    depth_chirho -= 1;
                    if depth_chirho == 0 {
                        return true;
                    }
                }
                RawTokenKindChirho::EofChirho
                | RawTokenKindChirho::VirtualSemicolonChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => return false,
                _ => {}
            }
        }
        false
    }

    pub(super) fn parse_gadt_field_type_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::GadtFieldTypeChirho);
        self.bump_chirho(); // !
        self.eat_trivia_chirho();
        self.parse_btype_chirho();
        self.builder_chirho.finish_node_chirho();
    }
}
