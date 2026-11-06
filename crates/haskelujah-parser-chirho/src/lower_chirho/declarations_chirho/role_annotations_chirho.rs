// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `type role T r1 .. rn` annotations (RoleAnnotations, row 694, #31844).
//!
//! The CST keeps a role annotation as a type signature node. Lowering reads it here into its
//! own declaration, `DeclChirho::RoleAnnotationDeclChirho`: the annotated type's name (a
//! constructor, qualified or not, or an operator in parentheses, `type role (:-) nominal
//! nominal`) and every role word exactly as written, each with its span. Lowering never judges the words or their
//! count and never rewrites one. Declaration checking judges them (an unknown word is
//! GHC-09009, a wrong count GHC-54298, a written role below the final one GHC-29178), and role
//! inference reads a well-formed annotation as its parameters' floors.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::LowerCtxChirho;

impl LowerCtxChirho {
    /// The role annotation `node_chirho` writes, as a declaration spanning `span_chirho`, when
    /// it is `type role T r1 .. rn`.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn lower_role_annotation_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Option<DeclChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut tokens_chirho =
            children_chirho
                .iter()
                .filter_map(|child_chirho| match child_chirho.element_chirho {
                    GreenElementChirho::TokenChirho(token_chirho) => {
                        Some((child_chirho, token_chirho))
                    }
                    GreenElementChirho::NodeChirho(_) => None,
                });
        let (_, keyword_chirho) = tokens_chirho.next()?;
        if keyword_chirho.kind_chirho() != TokenKindChirho::TypeKeywordChirho {
            return None;
        }
        let (_, role_chirho) = tokens_chirho.next()?;
        if role_chirho.text_chirho() != "role" {
            return None;
        }
        let (mut target_child_chirho, mut target_chirho) = tokens_chirho.next()?;
        match target_chirho.kind_chirho() {
            TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {}
            // An operator type constructor is written in parentheses.
            TokenKindChirho::LeftParenChirho => {
                (target_child_chirho, target_chirho) = tokens_chirho.next()?;
                if !matches!(
                    target_chirho.kind_chirho(),
                    TokenKindChirho::ConSymChirho
                        | TokenKindChirho::VarSymChirho
                        | TokenKindChirho::QualifiedConSymChirho
                        | TokenKindChirho::QualifiedVarSymChirho
                ) || tokens_chirho.next()?.1.kind_chirho() != TokenKindChirho::RightParenChirho
                {
                    return None;
                }
            }
            _ => return None,
        }
        let type_name_chirho = self.name_from_token_chirho(
            target_chirho,
            self.span_chirho(
                target_child_chirho.start_chirho,
                target_child_chirho.end_chirho,
            ),
        );
        let roles_chirho = tokens_chirho
            .map(|(child_chirho, token_chirho)| {
                let word_span_chirho =
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                self.name_from_token_chirho(token_chirho, word_span_chirho)
            })
            .collect();
        Some(DeclChirho::RoleAnnotationDeclChirho {
            type_name_chirho,
            roles_chirho,
            span_chirho,
        })
    }
}
