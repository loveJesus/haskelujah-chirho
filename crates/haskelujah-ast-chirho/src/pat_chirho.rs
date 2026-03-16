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
        con_chirho: NameChirho,
        args_chirho: Vec<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// Literal pattern (`42`, `'a'`, `"hello"`).
    LitChirho(LitChirho),
    /// Wildcard pattern (`_`).
    WildcardChirho(SpanChirho),
    /// As pattern (`x@pat`).
    AsChirho {
        name_chirho: NameChirho,
        pattern_chirho: Box<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// Tuple pattern (`(a, b, c)`).
    TupleChirho {
        elements_chirho: Vec<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// List pattern (`[a, b, c]`).
    ListChirho {
        elements_chirho: Vec<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// Parenthesized pattern (`(pat)`).
    ParenChirho {
        inner_chirho: Box<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// Negated literal pattern (`-42`).
    NegChirho {
        lit_chirho: LitChirho,
        span_chirho: SpanChirho,
    },
    /// Lazy (irrefutable) pattern (`~pat`).
    LazyChirho {
        inner_chirho: Box<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// Bang pattern (`!pat`).
    BangChirho {
        inner_chirho: Box<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// Infix constructor pattern (`x : xs`).
    InfixConChirho {
        left_chirho: Box<PatChirho>,
        op_chirho: NameChirho,
        right_chirho: Box<PatChirho>,
        span_chirho: SpanChirho,
    },
    /// Record pattern (`Foo { bar = baz }` or `Foo { bar, .. }`).
    RecordChirho {
        con_chirho: NameChirho,
        fields_chirho: Vec<PatFieldChirho>,
        /// Whether `..` was present (RecordWildCards).
        has_wildcard_chirho: bool,
        span_chirho: SpanChirho,
    },
    /// View pattern (`expr -> pat`), requires ViewPatterns extension.
    ViewChirho {
        expr_chirho: Box<super::expr_chirho::ExprChirho>,
        pat_chirho: Box<PatChirho>,
        span_chirho: SpanChirho,
    },
}

/// A field in a record pattern (`field = pat`).
#[derive(Debug, Clone, PartialEq)]
pub struct PatFieldChirho {
    pub name_chirho: NameChirho,
    pub pattern_chirho: PatChirho,
    pub span_chirho: SpanChirho,
}

impl PatChirho {
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
            | Self::ViewChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
