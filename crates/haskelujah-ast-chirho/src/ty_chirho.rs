// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type syntax

use haskelujah_span_chirho::SpanChirho;

use crate::decl_chirho::TyVarChirho;
use crate::name_chirho::NameChirho;

/// Multiplicity annotation for LinearTypes (`a %1 -> b`, `a %Many -> b`).
#[derive(Debug, Clone, PartialEq)]
pub enum MultiplicityChirho {
    /// Linear: exactly one use (`%1` or `⊸`).
    OneChirho,
    /// Unrestricted: any number of uses (`%Many` or plain `->` without annotation).
    ManyChirho,
    /// Multiplicity variable (`%m`): polymorphic over linearity.
    MultVarChirho(NameChirho),
}

/// A Haskell type expression.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeChirho {
    /// A type variable (`a`, `b`).
    VarChirho(NameChirho),
    /// A type constructor (`Int`, `Maybe`, `IO`).
    ConChirho(NameChirho),
    /// Type application (`Maybe Int`, `Either String Int`).
    AppChirho {
        /// Type constructor or higher-order type being applied.
        fun_chirho: Box<TypeChirho>,
        /// Type argument supplied to the function type.
        arg_chirho: Box<TypeChirho>,
        /// Span covering the whole type application.
        span_chirho: SpanChirho,
    },
    /// Function type (`a -> b`, `a %1 -> b`, `a ⊸ b`).
    FunChirho {
        /// Argument type accepted by the function.
        arg_chirho: Box<TypeChirho>,
        /// Optional linearity annotation on the arrow.
        mult_chirho: Option<MultiplicityChirho>,
        /// Result type produced by the function.
        result_chirho: Box<TypeChirho>,
        /// Span covering the whole function type.
        span_chirho: SpanChirho,
    },
    /// Tuple type (`(a, b, c)`).
    TupleChirho {
        /// Tuple element types in source order.
        elements_chirho: Vec<TypeChirho>,
        /// Span covering the whole tuple type.
        span_chirho: SpanChirho,
    },
    /// List type (`[a]`).
    ListChirho {
        /// Element type stored in the list.
        element_chirho: Box<TypeChirho>,
        /// Span covering the whole list type.
        span_chirho: SpanChirho,
    },
    /// Parenthesized type (`(Type)`).
    ParenChirho {
        /// Inner type wrapped in parentheses.
        inner_chirho: Box<TypeChirho>,
        /// Span covering the whole parenthesized type.
        span_chirho: SpanChirho,
    },
    /// Qualified type with context (`Eq a => a -> a -> Bool`).
    QualChirho {
        /// Constraints required before the body type is available.
        context_chirho: Vec<ConstraintChirho>,
        /// Type to the right of the `=>`.
        body_chirho: Box<TypeChirho>,
        /// Span covering the whole qualified type.
        span_chirho: SpanChirho,
    },
    /// Forall quantifier (`forall a b. Type`).
    ForallChirho {
        /// Universally quantified type variables.
        vars_chirho: Vec<TyVarChirho>,
        /// Body type quantified by the forall.
        body_chirho: Box<TypeChirho>,
        /// Span covering the whole forall type.
        span_chirho: SpanChirho,
    },
    /// DataKinds promoted constructor (`'True`, `'Just`, `'Nothing`).
    PromotedConChirho {
        /// Promoted data constructor name.
        name_chirho: NameChirho,
        /// Span covering the whole promoted constructor.
        span_chirho: SpanChirho,
    },
    /// DataKinds promoted list type (`'[Int, Bool]`, `'[]`).
    PromotedListChirho {
        /// Promoted list element types in source order.
        elements_chirho: Vec<TypeChirho>,
        /// Span covering the whole promoted list type.
        span_chirho: SpanChirho,
    },
    /// PartialTypeSignatures wildcard type (`_`). Lowered to a fresh
    /// unification variable during type inference.
    WildcardChirho { span_chirho: SpanChirho },
}

/// A class constraint in a type context (e.g. `Eq a`, `Show (Maybe a)`).
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintChirho {
    /// The class name being applied.
    pub class_chirho: NameChirho,
    /// Type arguments supplied to the class.
    pub args_chirho: Vec<TypeChirho>,
    /// Span covering the whole constraint.
    pub span_chirho: SpanChirho,
}

impl TypeChirho {
    /// Return the source span covering this type expression.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::VarChirho(n_chirho) => n_chirho.span_chirho(),
            Self::ConChirho(n_chirho) => n_chirho.span_chirho(),
            Self::AppChirho { span_chirho, .. }
            | Self::FunChirho { span_chirho, .. }
            | Self::TupleChirho { span_chirho, .. }
            | Self::ListChirho { span_chirho, .. }
            | Self::ParenChirho { span_chirho, .. }
            | Self::QualChirho { span_chirho, .. }
            | Self::ForallChirho { span_chirho, .. }
            | Self::PromotedConChirho { span_chirho, .. }
            | Self::PromotedListChirho { span_chirho, .. }
            | Self::WildcardChirho { span_chirho } => *span_chirho,
        }
    }
}
