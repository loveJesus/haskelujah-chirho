// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Green Tree — immutable, identity-free syntax tree nodes
//!
//! The green tree is the immutable backbone of the lossless CST. Each green
//! node stores its kind, text length, and children. Green nodes are cheap to
//! share (they use `Arc` internally) and can be reused across incremental
//! reparses.
//!
//! ## Architecture
//!
//! - `GreenTokenChirho` — a leaf node holding a token kind and its source text.
//! - `GreenNodeChirho` — an internal node holding a syntax kind and child elements.
//! - `GreenElementChirho` — either a token or a node (the children of a green node).
//! - `GreenBuilderChirho` — a builder for constructing green trees bottom-up.

use std::sync::Arc;

use crate::cst_chirho::SyntaxKindChirho;
use crate::token_chirho::TokenKindChirho;

// ---------------------------------------------------------------------------
// GreenTokenChirho — leaf nodes
// ---------------------------------------------------------------------------

/// A leaf node in the green tree, representing a single token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GreenTokenChirho {
    kind_chirho: TokenKindChirho,
    text_chirho: Arc<str>,
}

impl GreenTokenChirho {
    pub fn new_chirho(kind_chirho: TokenKindChirho, text_chirho: impl Into<Arc<str>>) -> Self {
        Self {
            kind_chirho,
            text_chirho: text_chirho.into(),
        }
    }

    pub fn kind_chirho(&self) -> TokenKindChirho {
        self.kind_chirho
    }

    pub fn text_chirho(&self) -> &str {
        &self.text_chirho
    }

    pub fn text_len_chirho(&self) -> usize {
        self.text_chirho.len()
    }
}

// ---------------------------------------------------------------------------
// GreenNodeChirho — internal nodes
// ---------------------------------------------------------------------------

/// An internal node in the green tree, with a syntax kind and children.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GreenNodeChirho {
    kind_chirho: SyntaxKindChirho,
    children_chirho: Vec<GreenElementChirho>,
    text_len_chirho: usize,
}

impl GreenNodeChirho {
    pub fn new_chirho(
        kind_chirho: SyntaxKindChirho,
        children_chirho: Vec<GreenElementChirho>,
    ) -> Self {
        let text_len_chirho = children_chirho
            .iter()
            .map(|c_chirho| c_chirho.text_len_chirho())
            .sum();
        Self {
            kind_chirho,
            children_chirho,
            text_len_chirho,
        }
    }

    pub fn kind_chirho(&self) -> SyntaxKindChirho {
        self.kind_chirho
    }

    pub fn children_chirho(&self) -> &[GreenElementChirho] {
        &self.children_chirho
    }

    pub fn text_len_chirho(&self) -> usize {
        self.text_len_chirho
    }

    pub fn child_count_chirho(&self) -> usize {
        self.children_chirho.len()
    }
}

// ---------------------------------------------------------------------------
// GreenElementChirho — a child of a green node
// ---------------------------------------------------------------------------

/// A child element of a green node — either a token leaf or a sub-node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GreenElementChirho {
    TokenChirho(GreenTokenChirho),
    NodeChirho(Arc<GreenNodeChirho>),
}

impl GreenElementChirho {
    pub fn text_len_chirho(&self) -> usize {
        match self {
            Self::TokenChirho(t_chirho) => t_chirho.text_len_chirho(),
            Self::NodeChirho(n_chirho) => n_chirho.text_len_chirho(),
        }
    }

    pub fn kind_display_chirho(&self) -> &'static str {
        match self {
            Self::TokenChirho(_) => "Token",
            Self::NodeChirho(_) => "Node",
        }
    }
}

impl From<GreenTokenChirho> for GreenElementChirho {
    fn from(token_chirho: GreenTokenChirho) -> Self {
        Self::TokenChirho(token_chirho)
    }
}

impl From<Arc<GreenNodeChirho>> for GreenElementChirho {
    fn from(node_chirho: Arc<GreenNodeChirho>) -> Self {
        Self::NodeChirho(node_chirho)
    }
}

// ---------------------------------------------------------------------------
// GreenBuilderChirho — incremental tree builder
// ---------------------------------------------------------------------------

/// A checkpoint in the builder that records the current position.
/// Used to retroactively wrap previously-parsed content in a new node.
#[derive(Debug, Clone, Copy)]
pub struct CheckpointChirho {
    stack_depth_chirho: usize,
    children_count_chirho: usize,
}

/// Builds a green tree bottom-up using a stack of in-progress nodes.
///
/// Usage:
/// ```ignore
/// let mut b = GreenBuilderChirho::new_chirho();
/// b.start_node_chirho(SyntaxKindChirho::SourceFileChirho);
///   b.token_chirho(TokenKindChirho::ModuleKeywordChirho, "module");
///   b.token_chirho(TokenKindChirho::WhitespaceTriviaChirho, " ");
///   // ...
/// b.finish_node_chirho();
/// let root = b.finish_chirho();
/// ```
pub struct GreenBuilderChirho {
    /// Stack of (kind, children) for nodes being built.
    stack_chirho: Vec<(SyntaxKindChirho, Vec<GreenElementChirho>)>,
}

impl GreenBuilderChirho {
    pub fn new_chirho() -> Self {
        Self {
            stack_chirho: Vec::new(),
        }
    }

    /// Start building a new node of the given kind.
    pub fn start_node_chirho(&mut self, kind_chirho: SyntaxKindChirho) {
        self.stack_chirho.push((kind_chirho, Vec::new()));
    }

    /// Create a checkpoint at the current builder position.
    /// Later, `start_node_at_chirho()` can retroactively wrap everything
    /// added since this checkpoint in a new node.
    pub fn checkpoint_chirho(&self) -> CheckpointChirho {
        let children_count_chirho = self
            .stack_chirho
            .last()
            .map_or(0, |(_, children_chirho)| children_chirho.len());
        CheckpointChirho {
            stack_depth_chirho: self.stack_chirho.len(),
            children_count_chirho,
        }
    }

    /// Retroactively start a node at the checkpoint position.
    /// Everything added to the current parent since the checkpoint
    /// becomes the initial children of this new node.
    pub fn start_node_at_chirho(
        &mut self,
        checkpoint_chirho: CheckpointChirho,
        kind_chirho: SyntaxKindChirho,
    ) {
        // If stack depth diverged from the checkpoint (e.g. due to error
        // recovery or malformed input), unwind back to the checkpoint depth
        // rather than panicking.
        while self.stack_chirho.len() > checkpoint_chirho.stack_depth_chirho {
            self.finish_node_chirho();
        }
        if self.stack_chirho.len() < checkpoint_chirho.stack_depth_chirho {
            // Can't recover — checkpoint was deeper than current stack.
            // Skip the wrap and return silently.
            return;
        }
        let parent_chirho = self
            .stack_chirho
            .last_mut()
            .expect("stack should not be empty");
        let children_count_chirho = checkpoint_chirho.children_count_chirho
            .min(parent_chirho.1.len());
        let wrapped_children_chirho =
            parent_chirho.1.split_off(children_count_chirho);
        self.stack_chirho
            .push((kind_chirho, wrapped_children_chirho));
    }

    /// Add a token leaf to the current node.
    pub fn token_chirho(&mut self, kind_chirho: TokenKindChirho, text_chirho: &str) {
        let token_chirho = GreenTokenChirho::new_chirho(kind_chirho, text_chirho);
        let element_chirho = GreenElementChirho::TokenChirho(token_chirho);
        if let Some(top_chirho) = self.stack_chirho.last_mut() {
            top_chirho.1.push(element_chirho);
        }
    }

    /// Finish the current node, pushing it as a child of the parent node
    /// (or making it the root).
    pub fn finish_node_chirho(&mut self) {
        let (kind_chirho, children_chirho) = self
            .stack_chirho
            .pop()
            .expect("finish_node_chirho called without matching start_node_chirho");
        let node_chirho = Arc::new(GreenNodeChirho::new_chirho(kind_chirho, children_chirho));
        let element_chirho = GreenElementChirho::NodeChirho(node_chirho);

        if let Some(parent_chirho) = self.stack_chirho.last_mut() {
            parent_chirho.1.push(element_chirho);
        } else {
            // This was the root — push back as a single-child frame so
            // finish_chirho() can extract it.
            self.stack_chirho
                .push((kind_chirho, vec![element_chirho]));
        }
    }

    /// Extract the finished root node. Panics if the tree is incomplete.
    pub fn finish_chirho(mut self) -> Arc<GreenNodeChirho> {
        assert_eq!(
            self.stack_chirho.len(),
            1,
            "unfinished nodes remain on the builder stack"
        );
        let (_, mut children_chirho) = self.stack_chirho.pop().unwrap();
        assert_eq!(
            children_chirho.len(),
            1,
            "root should have exactly one child (the root node)"
        );
        match children_chirho.pop().unwrap() {
            GreenElementChirho::NodeChirho(n_chirho) => n_chirho,
            _ => panic!("root child should be a node"),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn build_simple_tree_chirho() {
        let mut builder_chirho = GreenBuilderChirho::new_chirho();

        builder_chirho.start_node_chirho(SyntaxKindChirho::SourceFileChirho);
        builder_chirho.start_node_chirho(SyntaxKindChirho::ModuleHeaderChirho);
        builder_chirho.token_chirho(TokenKindChirho::ModuleKeywordChirho, "module");
        builder_chirho.token_chirho(TokenKindChirho::WhitespaceTriviaChirho, " ");
        builder_chirho.token_chirho(TokenKindChirho::ConIdChirho, "Main");
        builder_chirho.token_chirho(TokenKindChirho::WhitespaceTriviaChirho, " ");
        builder_chirho.token_chirho(TokenKindChirho::WhereKeywordChirho, "where");
        builder_chirho.finish_node_chirho();
        builder_chirho.finish_node_chirho();

        let root_chirho = builder_chirho.finish_chirho();

        assert_eq!(root_chirho.kind_chirho(), SyntaxKindChirho::SourceFileChirho);
        assert_eq!(root_chirho.text_len_chirho(), "module Main where".len());
        assert_eq!(root_chirho.child_count_chirho(), 1); // ModuleHeader
    }

    #[test]
    fn green_token_basics_chirho() {
        let tok_chirho =
            GreenTokenChirho::new_chirho(TokenKindChirho::IntegerLiteralChirho, "42");
        assert_eq!(tok_chirho.kind_chirho(), TokenKindChirho::IntegerLiteralChirho);
        assert_eq!(tok_chirho.text_chirho(), "42");
        assert_eq!(tok_chirho.text_len_chirho(), 2);
    }

    #[test]
    fn checkpoint_wraps_retroactively_chirho() {
        let mut builder_chirho = GreenBuilderChirho::new_chirho();
        builder_chirho.start_node_chirho(SyntaxKindChirho::SourceFileChirho);

        // Parse "Int -> String" — checkpoint before Int, wrap in FunType after ->
        let cp_chirho = builder_chirho.checkpoint_chirho();
        builder_chirho.token_chirho(TokenKindChirho::ConIdChirho, "Int");
        builder_chirho.token_chirho(TokenKindChirho::WhitespaceTriviaChirho, " ");

        // Retroactively wrap "Int " in a FunType node
        builder_chirho.start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
        builder_chirho.token_chirho(TokenKindChirho::VarSymChirho, "->");
        builder_chirho.token_chirho(TokenKindChirho::WhitespaceTriviaChirho, " ");
        builder_chirho.token_chirho(TokenKindChirho::ConIdChirho, "String");
        builder_chirho.finish_node_chirho(); // FunType

        builder_chirho.finish_node_chirho(); // SourceFile

        let root_chirho = builder_chirho.finish_chirho();
        assert_eq!(root_chirho.text_len_chirho(), "Int -> String".len());
        assert_eq!(root_chirho.child_count_chirho(), 1); // FunType
        if let GreenElementChirho::NodeChirho(fun_chirho) = &root_chirho.children_chirho()[0] {
            assert_eq!(fun_chirho.kind_chirho(), SyntaxKindChirho::FunTypeChirho);
            // Children: "Int", " ", "->", " ", "String"
            assert_eq!(fun_chirho.child_count_chirho(), 5);
        } else {
            panic!("expected FunType node");
        }
    }

    #[test]
    fn text_len_accumulates_chirho() {
        let children_chirho = vec![
            GreenElementChirho::TokenChirho(GreenTokenChirho::new_chirho(
                TokenKindChirho::VarIdChirho,
                "hello",
            )),
            GreenElementChirho::TokenChirho(GreenTokenChirho::new_chirho(
                TokenKindChirho::WhitespaceTriviaChirho,
                " ",
            )),
            GreenElementChirho::TokenChirho(GreenTokenChirho::new_chirho(
                TokenKindChirho::VarIdChirho,
                "world",
            )),
        ];
        let node_chirho = GreenNodeChirho::new_chirho(
            SyntaxKindChirho::AppExprChirho,
            children_chirho,
        );
        assert_eq!(node_chirho.text_len_chirho(), 11);
    }
}
