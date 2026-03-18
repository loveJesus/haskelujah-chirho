// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Pattern syntax

use haskelujah_span_chirho::SpanChirho;

use crate::lit_chirho::LitChirho;
use crate::name_chirho::NameChirho;

/// A Haskell pattern.
#[derive(Debug, Clone, PartialEq)]
pub enum PatChirho {
    /// Variable pattern (`x`).
    VarChirho(NameChirho),
    /// Constructor pattern (`Just x`, `Left a`).
    ConChirho {
        /// Constructor name being matched.
        con_chirho: NameChirho,
        /// Positional subpatterns matched against constructor fields.
        args_chirho: Vec<PatChirho>,
        /// Span covering the whole constructor pattern.
        span_chirho: SpanChirho,
    },
    /// Literal pattern (`42`, `'a'`, `"hello"`).
    LitChirho(LitChirho),
    /// Wildcard pattern (`_`).
    WildcardChirho(SpanChirho),
    /// As pattern (`x@pat`).
    AsChirho {
        /// Variable name bound to the entire matched value.
        name_chirho: NameChirho,
        /// Nested pattern matched after the alias binding.
        pattern_chirho: Box<PatChirho>,
        /// Span covering the whole as-pattern.
        span_chirho: SpanChirho,
    },
    /// Tuple pattern (`(a, b, c)`).
    TupleChirho {
        /// Tuple element patterns in source order.
        elements_chirho: Vec<PatChirho>,
        /// Span covering the whole tuple pattern.
        span_chirho: SpanChirho,
    },
    /// List pattern (`[a, b, c]`).
    ListChirho {
        /// Element patterns matched against the list.
        elements_chirho: Vec<PatChirho>,
        /// Span covering the whole list pattern.
        span_chirho: SpanChirho,
    },
    /// Parenthesized pattern (`(pat)`).
    ParenChirho {
        /// Inner pattern wrapped in parentheses.
        inner_chirho: Box<PatChirho>,
        /// Span covering the whole parenthesized pattern.
        span_chirho: SpanChirho,
    },
    /// Negated literal pattern (`-42`).
    NegChirho {
        /// Literal being matched after unary negation.
        lit_chirho: LitChirho,
        /// Span covering the whole negated literal pattern.
        span_chirho: SpanChirho,
    },
    /// Lazy (irrefutable) pattern (`~pat`).
    LazyChirho {
        /// Pattern matched lazily.
        inner_chirho: Box<PatChirho>,
        /// Span covering the whole lazy pattern.
        span_chirho: SpanChirho,
    },
    /// Bang pattern (`!pat`).
    BangChirho {
        /// Pattern forced before matching proceeds.
        inner_chirho: Box<PatChirho>,
        /// Span covering the whole bang pattern.
        span_chirho: SpanChirho,
    },
    /// Infix constructor pattern (`x : xs`).
    InfixConChirho {
        /// Left operand pattern.
        left_chirho: Box<PatChirho>,
        /// Infix constructor being matched.
        op_chirho: NameChirho,
        /// Right operand pattern.
        right_chirho: Box<PatChirho>,
        /// Span covering the whole infix constructor pattern.
        span_chirho: SpanChirho,
    },
    /// Record pattern (`Foo { bar = baz }` or `Foo { bar, .. }`).
    RecordChirho {
        /// Record constructor being matched.
        con_chirho: NameChirho,
        /// Field subpatterns explicitly listed in the record pattern.
        fields_chirho: Vec<PatFieldChirho>,
        /// Whether `..` was present (RecordWildCards).
        has_wildcard_chirho: bool,
        /// Span covering the whole record pattern.
        span_chirho: SpanChirho,
    },
    /// View pattern (`expr -> pat`), requires ViewPatterns extension.
    ViewChirho {
        /// Expression applied to the scrutinee before matching.
        expr_chirho: Box<super::expr_chirho::ExprChirho>,
        /// Pattern matched against the view result.
        pat_chirho: Box<PatChirho>,
        /// Span covering the whole view pattern.
        span_chirho: SpanChirho,
    },
    /// Type-annotated pattern (`(x :: Int)`), requires ScopedTypeVariables.
    TypeAnnotChirho {
        /// Underlying pattern carrying the annotation.
        pat_chirho: Box<PatChirho>,
        /// Type annotation attached to the pattern.
        ty_chirho: super::ty_chirho::TypeChirho,
        /// Span covering the whole typed pattern.
        span_chirho: SpanChirho,
    },
}

/// A field in a record pattern (`field = pat`).
#[derive(Debug, Clone, PartialEq)]
pub struct PatFieldChirho {
    /// The record field name being matched.
    pub name_chirho: NameChirho,
    /// The pattern bound to the field.
    pub pattern_chirho: PatChirho,
    /// Span covering the whole field binding.
    pub span_chirho: SpanChirho,
}

impl PatChirho {
    /// Return the source span covering this pattern.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::VarChirho(n_chirho) => n_chirho.span_chirho(),
            Self::LitChirho(l_chirho) => l_chirho.span_chirho(),
            Self::WildcardChirho(s_chirho) => *s_chirho,
            Self::ConChirho { span_chirho, .. }
            | Self::AsChirho { span_chirho, .. }
            | Self::TupleChirho { span_chirho, .. }
            | Self::ListChirho { span_chirho, .. }
            | Self::ParenChirho { span_chirho, .. }
            | Self::NegChirho { span_chirho, .. }
            | Self::LazyChirho { span_chirho, .. }
            | Self::BangChirho { span_chirho, .. }
            | Self::InfixConChirho { span_chirho, .. }
            | Self::RecordChirho { span_chirho, .. }
            | Self::ViewChirho { span_chirho, .. }
            | Self::TypeAnnotChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
