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
    /// Written multiplicity type, including variables and family applications.
    /// Preserve its name and span for lexical scope and kind validation.
    ExpressionChirho(Box<TypeChirho>),
}

impl MultiplicityChirho {
    /// Recognize fixed linear syntax after name/kind validation. Unquoted
    /// aliases and family applications require elaboration, not a spelling guess.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub fn is_explicit_one_chirho(&self) -> bool {
        match self {
            Self::OneChirho => true,
            Self::ManyChirho => false,
            Self::ExpressionChirho(expression_chirho) => {
                let mut inner_chirho = expression_chirho.as_ref();
                while let TypeChirho::ParenChirho {
                    inner_chirho: next_chirho,
                    ..
                } = inner_chirho
                {
                    inner_chirho = next_chirho;
                }
                matches!(inner_chirho, TypeChirho::PromotedConChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "One")
            }
        }
    }
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
    /// An invisible argument supplied explicitly (`Proxy @Bool`). It does not
    /// consume an ordinary arrow in the constructor's kind and is retained
    /// separately until kind elaboration chooses its quantified binder.
    KindAppChirho {
        fun_chirho: Box<TypeChirho>,
        arg_chirho: Box<TypeChirho>,
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
    /// Required forall quantifier (`forall a -> Type`).
    RequiredForallChirho {
        /// Type variables supplied without `@` at call sites.
        vars_chirho: Vec<TyVarChirho>,
        /// Body type made available after the required arguments.
        body_chirho: Box<TypeChirho>,
        /// Span covering the whole required forall type.
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
    /// Type-level literal (DataKinds: numeric `42`, string `"hello"`, char `'x'`).
    LitChirho {
        /// The literal value as a string representation.
        value_chirho: String,
        /// Span covering the type-level literal.
        span_chirho: SpanChirho,
    },
}

/// A constraint in a type context.
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintChirho {
    /// A simple class constraint such as `Eq a` or `Show (Maybe a)`.
    ClassChirho {
        class_chirho: NameChirho,
        args_chirho: Vec<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// A quantified constraint such as `forall a. Eq a => Show (f a)`.
    QuantifiedChirho {
        vars_chirho: Vec<TyVarChirho>,
        context_chirho: Vec<ConstraintChirho>,
        body_chirho: Box<ConstraintChirho>,
        span_chirho: SpanChirho,
    },
}

impl ConstraintChirho {
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::ClassChirho { span_chirho, .. } | Self::QuantifiedChirho { span_chirho, .. } => {
                *span_chirho
            }
        }
    }

    pub fn simple_class_chirho(&self) -> Option<&NameChirho> {
        match self {
            Self::ClassChirho { class_chirho, .. } => Some(class_chirho),
            Self::QuantifiedChirho { .. } => None,
        }
    }

    pub fn simple_args_chirho(&self) -> Option<&[TypeChirho]> {
        match self {
            Self::ClassChirho { args_chirho, .. } => Some(args_chirho.as_slice()),
            Self::QuantifiedChirho { .. } => None,
        }
    }
}

impl TypeChirho {
    /// Return the source span covering this type expression.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::VarChirho(n_chirho) => n_chirho.span_chirho(),
            Self::ConChirho(n_chirho) => n_chirho.span_chirho(),
            Self::AppChirho { span_chirho, .. }
            | Self::KindAppChirho { span_chirho, .. }
            | Self::FunChirho { span_chirho, .. }
            | Self::TupleChirho { span_chirho, .. }
            | Self::ListChirho { span_chirho, .. }
            | Self::ParenChirho { span_chirho, .. }
            | Self::QualChirho { span_chirho, .. }
            | Self::ForallChirho { span_chirho, .. }
            | Self::RequiredForallChirho { span_chirho, .. }
            | Self::PromotedConChirho { span_chirho, .. }
            | Self::PromotedListChirho { span_chirho, .. }
            | Self::WildcardChirho { span_chirho }
            | Self::LitChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
