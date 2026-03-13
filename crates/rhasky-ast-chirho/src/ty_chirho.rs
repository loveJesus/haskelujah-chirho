// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type syntax

use rhasky_span_chirho::SpanChirho;

use crate::name_chirho::NameChirho;

/// A Haskell type expression.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeChirho {
    /// A type variable (`a`, `b`).
    VarChirho(NameChirho),
    /// A type constructor (`Int`, `Maybe`, `IO`).
    ConChirho(NameChirho),
    /// Type application (`Maybe Int`, `Either String Int`).
    AppChirho {
        fun_chirho: Box<TypeChirho>,
        arg_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Function type (`a -> b`).
    FunChirho {
        arg_chirho: Box<TypeChirho>,
        result_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Tuple type (`(a, b, c)`).
    TupleChirho {
        elements_chirho: Vec<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// List type (`[a]`).
    ListChirho {
        element_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Parenthesized type (`(Type)`).
    ParenChirho {
        inner_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Qualified type with context (`Eq a => a -> a -> Bool`).
    QualChirho {
        context_chirho: Vec<ConstraintChirho>,
        body_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Forall quantifier (`forall a b. Type`).
    ForallChirho {
        vars_chirho: Vec<NameChirho>,
        body_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
}

/// A class constraint in a type context (e.g. `Eq a`, `Show (Maybe a)`).
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintChirho {
    pub class_chirho: NameChirho,
    pub args_chirho: Vec<TypeChirho>,
    pub span_chirho: SpanChirho,
}

impl TypeChirho {
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
            | Self::ForallChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
