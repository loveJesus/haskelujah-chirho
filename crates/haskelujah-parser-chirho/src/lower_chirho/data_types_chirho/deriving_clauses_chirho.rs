// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deriving clauses and standalone deriving strategies (row 694).
//! Part of CST-to-AST lowering (`lower_chirho.rs`).
//!
//! Each clause lowers to the classes it lists, every one carrying the clause's written
//! strategy. A class is the first constructor name of each comma-separated entry, so the
//! arguments of `deriving (C (Maybe Int))` are never read as classes of their own.
//! workflow: language-features-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::{DerivedClassChirho, DerivingStrategyChirho};
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    /// The classes one deriving clause lists, each with the clause's strategy. A
    /// `deriving C via T` clause yields nothing: the module's via record derives it.
    /// workflow: language-features-chirho/deriving-strategies-chirho
    pub(super) fn lower_deriving_clause_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<DerivedClassChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut strategy_chirho = None;
        let mut classes_chirho = Vec::new();
        // The clause's own list is depth 1; a bare `deriving C` is depth 0.
        let mut depth_chirho = 0usize;
        let mut expecting_class_chirho = true;
        let mut saw_class_list_chirho = false;
        for child_chirho in &children_chirho {
            let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho else {
                continue;
            };
            let kind_chirho = tok_chirho.kind_chirho();
            if kind_chirho == TokenKindChirho::VarIdChirho && tok_chirho.text_chirho() == "via" {
                return Vec::new();
            }
            match kind_chirho {
                TokenKindChirho::DerivingKeywordChirho => {}
                TokenKindChirho::NewtypeKeywordChirho | TokenKindChirho::VarIdChirho
                    if !saw_class_list_chirho && strategy_chirho.is_none() =>
                {
                    let span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    strategy_chirho = DerivingStrategyChirho::from_keyword_chirho(
                        self.name_from_token_chirho(tok_chirho, span_chirho),
                    );
                }
                TokenKindChirho::LeftParenChirho => {
                    saw_class_list_chirho = true;
                    depth_chirho += 1;
                }
                TokenKindChirho::RightParenChirho => {
                    depth_chirho = depth_chirho.saturating_sub(1);
                }
                TokenKindChirho::CommaChirho if depth_chirho <= 1 => {
                    expecting_class_chirho = true;
                }
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
                    if expecting_class_chirho && depth_chirho <= 1 =>
                {
                    saw_class_list_chirho = true;
                    expecting_class_chirho = false;
                    let span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    classes_chirho.push(self.name_from_token_chirho(tok_chirho, span_chirho));
                }
                _ => saw_class_list_chirho = true,
            }
        }
        classes_chirho
            .into_iter()
            .map(|class_chirho| {
                DerivedClassChirho::new_chirho(class_chirho, strategy_chirho.clone())
            })
            .collect()
    }

    /// The strategy a standalone deriving declaration writes before `instance`
    /// (`deriving newtype instance Show Age`). `via` is not a strategy here: a standalone
    /// `deriving via` keeps its existing route.
    /// workflow: language-features-chirho/deriving-strategies-chirho
    pub(super) fn standalone_deriving_strategy_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
    ) -> Option<DerivingStrategyChirho> {
        for child_chirho in children_chirho {
            let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho else {
                continue;
            };
            match tok_chirho.kind_chirho() {
                TokenKindChirho::DerivingKeywordChirho => {}
                TokenKindChirho::NewtypeKeywordChirho | TokenKindChirho::VarIdChirho => {
                    let span_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    return DerivingStrategyChirho::from_keyword_chirho(
                        self.name_from_token_chirho(tok_chirho, span_chirho),
                    );
                }
                _ => return None,
            }
        }
        None
    }
}
