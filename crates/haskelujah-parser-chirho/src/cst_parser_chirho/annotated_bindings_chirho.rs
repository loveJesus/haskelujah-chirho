// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Distinguish a pattern annotation with a right-hand side from a standalone type signature.
//! `where t :: [a] = xs` binds `t`; treating the prefix as a signature loses that binding.
//! Pattern and type parsing, including the annotation's lowering, remain shared.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    /// Look past one atomic pattern and its annotation for a binding's `=` or guard. A bare
    /// function head followed by argument patterns is deliberately excluded: result type
    /// signatures such as `f x :: T = x` are not pattern annotations.
    pub(super) fn starts_annotated_pat_bind_chirho(&self) -> bool {
        let Some(first_chirho) = self.tokens_chirho.get(self.pos_chirho) else {
            return false;
        };
        let mut idx_chirho = self.pos_chirho + 1;
        match first_chirho.kind_chirho {
            RawTokenKindChirho::VarIdChirho | RawTokenKindChirho::ConIdChirho => {}
            RawTokenKindChirho::LeftParenChirho | RawTokenKindChirho::LeftBracketChirho => {
                let mut depth_chirho = 1usize;
                while let Some(token_chirho) = self.tokens_chirho.get(idx_chirho) {
                    match token_chirho.kind_chirho {
                        RawTokenKindChirho::LeftParenChirho
                        | RawTokenKindChirho::LeftBracketChirho => depth_chirho += 1,
                        RawTokenKindChirho::RightParenChirho
                        | RawTokenKindChirho::RightBracketChirho => depth_chirho -= 1,
                        RawTokenKindChirho::EofChirho => return false,
                        _ => {}
                    }
                    idx_chirho += 1;
                    if depth_chirho == 0 {
                        break;
                    }
                }
                if depth_chirho != 0 {
                    return false;
                }
            }
            _ => return false,
        }
        idx_chirho = self.skip_trivia_idx_chirho(idx_chirho);
        if !self
            .tokens_chirho
            .get(idx_chirho)
            .is_some_and(|token_chirho| {
                token_chirho.kind_chirho == RawTokenKindChirho::ColonColonChirho
            })
        {
            return false;
        }
        idx_chirho += 1;

        let mut depth_chirho = 0usize;
        while let Some(token_chirho) = self.tokens_chirho.get(idx_chirho) {
            match token_chirho.kind_chirho {
                RawTokenKindChirho::EqualsChirho | RawTokenKindChirho::PipeChirho
                    if depth_chirho == 0 =>
                {
                    return true;
                }
                RawTokenKindChirho::VirtualSemicolonChirho
                | RawTokenKindChirho::SemicolonChirho
                | RawTokenKindChirho::VirtualRightBraceChirho
                | RawTokenKindChirho::EofChirho => return false,
                RawTokenKindChirho::LeftParenChirho
                | RawTokenKindChirho::LeftBracketChirho
                | RawTokenKindChirho::LeftBraceChirho => depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho
                | RawTokenKindChirho::RightBracketChirho
                | RawTokenKindChirho::RightBraceChirho => {
                    if depth_chirho == 0 {
                        return false;
                    }
                    depth_chirho -= 1;
                }
                _ => {}
            }
            idx_chirho += 1;
        }
        false
    }
}
