// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deriving clauses and standalone deriving strategies (row 694).
//! Part of CST-to-AST lowering (`lower_chirho.rs`).
//!
//! Each clause lowers to the classes it lists, every one carrying the clause's written
//! strategy. Each comma-separated entry lowers through the flat type lowering record fields
//! use, and splits into its class and the arguments written after it: `C (Maybe Int)` is the
//! class `C` applied to `Maybe Int`, never two classes, and the arguments are never dropped.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::{DerivedClassChirho, DerivingStrategyChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    /// The classes one deriving clause lists, each with the clause's strategy and its own
    /// arguments. A `deriving C via T` clause yields nothing: the module's via record
    /// derives it.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn lower_deriving_clause_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<DerivedClassChirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut strategy_chirho = None;
        let mut entries_chirho: Vec<Vec<&ChildChirho>> = Vec::new();
        let mut entry_chirho: Vec<&ChildChirho> = Vec::new();
        // The clause's own list is depth 1; a bare `deriving C` is depth 0.
        let mut depth_chirho = 0usize;
        let mut saw_class_list_chirho = false;
        for child_chirho in &children_chirho {
            let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho else {
                if depth_chirho > 0 {
                    entry_chirho.push(child_chirho);
                }
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
                TokenKindChirho::LeftParenChirho if depth_chirho == 0 => {
                    saw_class_list_chirho = true;
                    depth_chirho = 1;
                }
                TokenKindChirho::RightParenChirho | TokenKindChirho::CommaChirho
                    if depth_chirho == 1 =>
                {
                    entries_chirho.push(std::mem::take(&mut entry_chirho));
                    if kind_chirho == TokenKindChirho::RightParenChirho {
                        depth_chirho = 0;
                    }
                }
                _ if depth_chirho > 0 => {
                    match kind_chirho {
                        TokenKindChirho::LeftParenChirho => depth_chirho += 1,
                        TokenKindChirho::RightParenChirho => depth_chirho -= 1,
                        _ => {}
                    }
                    entry_chirho.push(child_chirho);
                }
                TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho
                    if !saw_class_list_chirho =>
                {
                    saw_class_list_chirho = true;
                    entries_chirho.push(vec![child_chirho]);
                }
                _ => saw_class_list_chirho = true,
            }
        }
        // A list a layout token cut short keeps the entry it was reading.
        entries_chirho.push(entry_chirho);
        entries_chirho
            .into_iter()
            .filter_map(|entry_chirho| {
                let (first_chirho, last_chirho) = (entry_chirho.first()?, entry_chirho.last()?);
                let span_chirho =
                    self.span_chirho(first_chirho.start_chirho, last_chirho.end_chirho);
                let (class_chirho, arguments_chirho) = class_application_chirho(
                    self.type_from_flat_children_chirho(&entry_chirho, span_chirho),
                )?;
                Some(DerivedClassChirho::applied_chirho(
                    class_chirho,
                    arguments_chirho,
                    strategy_chirho.clone(),
                ))
            })
            .collect()
    }

    /// The strategy a standalone deriving declaration writes before `instance`
    /// (`deriving newtype instance Show Age`). `via` is not a strategy here: a standalone
    /// `deriving via` keeps its existing route.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
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

/// A deriving entry's class and the arguments written after it, in order: `C (Maybe Int) Bool`
/// is `C` applied to `Maybe Int` and `Bool`. `None` when the entry's head names no class.
fn class_application_chirho(type_chirho: TypeChirho) -> Option<(NameChirho, Vec<TypeChirho>)> {
    let mut arguments_chirho = Vec::new();
    let mut head_chirho = type_chirho;
    loop {
        match head_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                arguments_chirho.push(*arg_chirho);
                head_chirho = *fun_chirho;
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => head_chirho = *inner_chirho,
            TypeChirho::ConChirho(class_chirho) => {
                arguments_chirho.reverse();
                return Some((class_chirho, arguments_chirho));
            }
            _ => return None,
        }
    }
}
