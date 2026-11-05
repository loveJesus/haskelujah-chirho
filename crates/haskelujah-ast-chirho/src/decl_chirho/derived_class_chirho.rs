// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The classes a `deriving` clause requests, each with its clause's strategy (row 694).
//!
//! `newtype Age = Age Int deriving stock Eq deriving newtype (Show, Num)` derives Eq with
//! the stock strategy and Show and Num with the newtype strategy, so the strategy is kept
//! per class, not per declaration. A clause without a strategy keyword leaves the choice to
//! the deriving pass's legacy policy. A class keeps the arguments written before the derived
//! type, so `deriving anyclass (Pair Int)` asks for `Pair Int T`, never `Pair T`.
//! `deriving C via T` clauses keep their own module-level record
//! (`ModuleChirho::deriving_via_chirho`) and never appear here.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use haskelujah_span_chirho::SpanChirho;

use crate::name_chirho::{NameChirho, RawNameChirho};
use crate::ty_chirho::TypeChirho;

/// An explicit `stock`, `newtype` or `anyclass` strategy, as written.
///
/// The keyword is kept as a name so a diagnostic can point at it. Only those three
/// keywords construct one, so the three predicates are exhaustive.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivingStrategyChirho {
    keyword_chirho: NameChirho,
}

impl DerivingStrategyChirho {
    /// The stock strategy's keyword: the class's own deriving code.
    pub const STOCK_CHIRHO: &'static str = "stock";
    /// The newtype strategy's keyword: the representation type's instance (GND).
    pub const NEWTYPE_CHIRHO: &'static str = "newtype";
    /// The anyclass strategy's keyword: an empty instance using the class defaults.
    pub const ANYCLASS_CHIRHO: &'static str = "anyclass";

    /// The strategy `keyword_chirho` spells, or `None` for any other word.
    pub fn from_keyword_chirho(keyword_chirho: NameChirho) -> Option<Self> {
        matches!(
            keyword_chirho.text_chirho(),
            Self::STOCK_CHIRHO | Self::NEWTYPE_CHIRHO | Self::ANYCLASS_CHIRHO
        )
        .then_some(Self { keyword_chirho })
    }

    /// The stock strategy, written at `span_chirho`.
    pub fn stock_at_chirho(span_chirho: SpanChirho) -> Self {
        Self::at_chirho(Self::STOCK_CHIRHO, span_chirho)
    }

    /// The newtype strategy, written at `span_chirho`.
    pub fn newtype_at_chirho(span_chirho: SpanChirho) -> Self {
        Self::at_chirho(Self::NEWTYPE_CHIRHO, span_chirho)
    }

    /// The anyclass strategy, written at `span_chirho`.
    pub fn anyclass_at_chirho(span_chirho: SpanChirho) -> Self {
        Self::at_chirho(Self::ANYCLASS_CHIRHO, span_chirho)
    }

    fn at_chirho(keyword_text_chirho: &str, span_chirho: SpanChirho) -> Self {
        Self {
            keyword_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                keyword_text_chirho,
                span_chirho,
            )),
        }
    }

    /// The written keyword, with its span.
    pub fn keyword_chirho(&self) -> &NameChirho {
        &self.keyword_chirho
    }

    /// Whether this is `stock`.
    pub fn is_stock_chirho(&self) -> bool {
        self.keyword_chirho.text_chirho() == Self::STOCK_CHIRHO
    }

    /// Whether this is `newtype`.
    pub fn is_newtype_chirho(&self) -> bool {
        self.keyword_chirho.text_chirho() == Self::NEWTYPE_CHIRHO
    }

    /// Whether this is `anyclass`.
    pub fn is_anyclass_chirho(&self) -> bool {
        self.keyword_chirho.text_chirho() == Self::ANYCLASS_CHIRHO
    }
}

/// One class a deriving clause requests.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedClassChirho {
    /// The class, as written (`Show`, `P.Show`).
    pub class_chirho: NameChirho,
    /// The arguments written after the class, before the derived type, in order: `Int` in
    /// `deriving anyclass (Pair Int)`. Empty for a bare class.
    pub class_arguments_chirho: Vec<TypeChirho>,
    /// The clause's explicit strategy; `None` when the clause names none.
    pub strategy_chirho: Option<DerivingStrategyChirho>,
}

impl DerivedClassChirho {
    /// A bare class requested with `strategy_chirho`.
    pub fn new_chirho(
        class_chirho: NameChirho,
        strategy_chirho: Option<DerivingStrategyChirho>,
    ) -> Self {
        Self::applied_chirho(class_chirho, Vec::new(), strategy_chirho)
    }

    /// A class applied to `class_arguments_chirho`, requested with `strategy_chirho`.
    pub fn applied_chirho(
        class_chirho: NameChirho,
        class_arguments_chirho: Vec<TypeChirho>,
        strategy_chirho: Option<DerivingStrategyChirho>,
    ) -> Self {
        Self {
            class_chirho,
            class_arguments_chirho,
            strategy_chirho,
        }
    }

    /// A class from a clause that names no strategy.
    pub fn unspecified_chirho(class_chirho: NameChirho) -> Self {
        Self::new_chirho(class_chirho, None)
    }

    /// The class as written.
    pub fn class_text_chirho(&self) -> &str {
        self.class_chirho.text_chirho()
    }

    /// Whether the class is applied to arguments before the derived type.
    pub fn has_class_arguments_chirho(&self) -> bool {
        !self.class_arguments_chirho.is_empty()
    }

    /// Whether the entry's strategy lets the class's own stock code derive it: none is
    /// written, or `stock` is. Which stock class the entry names is the deriving pass's
    /// judgment, not a spelling.
    pub fn allows_stock_chirho(&self) -> bool {
        self.strategy_chirho
            .as_ref()
            .is_none_or(DerivingStrategyChirho::is_stock_chirho)
    }
}

#[cfg(test)]
#[path = "derived_class_tests_chirho.rs"]
mod tests_chirho;
