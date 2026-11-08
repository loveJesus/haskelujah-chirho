// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type syntax

use haskelujah_span_chirho::SpanChirho;

use crate::decl_chirho::TyVarChirho;
use crate::name_chirho::NameChirho;

pub mod inference_variables_chirho;

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
    /// A retained written kind annotation (`a :: k`).
    KindAnnotationChirho {
        ty_chirho: Box<TypeChirho>,
        kind_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// A kind argument, written with `@` or elaborated from a declaration's
    /// telescope. It never counts as an ordinary written type argument.
    KindApplicationChirho {
        fun_chirho: Box<TypeChirho>,
        arg_chirho: Box<TypeChirho>,
        written_chirho: bool,
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
    /// A written sort annotation on a complete constraint expression.
    KindAnnotationChirho {
        constraint_chirho: Box<ConstraintChirho>,
        kind_chirho: Box<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// A simple class constraint such as `Eq a` or `Show (Maybe a)`.
    ClassChirho {
        /// The complete stamped class name; boxing changes only its storage.
        // Workflow: spec-chirho/workflows-chirho/type-identity-chirho/ast-owned-payloads-chirho.md
        class_chirho: Box<NameChirho>,
        args_chirho: Vec<TypeChirho>,
        /// Hidden kind arguments retain their own prefix, never primary/extra
        /// ordinary class arguments.
        kind_args_chirho: Vec<TypeChirho>,
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
    /// An annotation preserves its check and source span, while consumers of
    /// the predicate itself share one transparent structural view.
    pub fn unannotated_chirho(&self) -> &Self {
        let mut current_chirho = self;
        while let Self::KindAnnotationChirho {
            constraint_chirho, ..
        } = current_chirho
        {
            current_chirho = constraint_chirho;
        }
        current_chirho
    }

    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::ClassChirho { span_chirho, .. }
            | Self::QuantifiedChirho { span_chirho, .. }
            | Self::KindAnnotationChirho { span_chirho, .. } => *span_chirho,
        }
    }

    pub fn simple_class_chirho(&self) -> Option<&NameChirho> {
        match self {
            Self::ClassChirho { class_chirho, .. } => Some(class_chirho.as_ref()),
            Self::QuantifiedChirho { .. } => None,
            Self::KindAnnotationChirho {
                constraint_chirho, ..
            } => constraint_chirho.simple_class_chirho(),
        }
    }

    pub fn simple_args_chirho(&self) -> Option<&[TypeChirho]> {
        match self {
            Self::ClassChirho { args_chirho, .. } => Some(args_chirho.as_slice()),
            Self::QuantifiedChirho { .. } => None,
            Self::KindAnnotationChirho {
                constraint_chirho, ..
            } => constraint_chirho.simple_args_chirho(),
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
            | Self::KindAnnotationChirho { span_chirho, .. }
            | Self::KindApplicationChirho { span_chirho, .. }
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

/// The former restricted kind syntax is a compatibility construction view.
/// Written producers retain TypeChirho directly and never project through it.
impl From<crate::decl_chirho::AstKindChirho> for TypeChirho {
    fn from(kind_chirho: crate::decl_chirho::AstKindChirho) -> Self {
        use crate::decl_chirho::AstKindChirho;
        use crate::name_chirho::{
            CompilerTypeConstructorChirho, RawNameChirho, TypeConstructorDeclarationChirho,
        };
        let name_chirho = |text_chirho: &str| {
            NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                text_chirho,
                SpanChirho::DUMMY_CHIRHO,
            ))
        };
        match kind_chirho {
            AstKindChirho::StarChirho => {
                let mut written_chirho = name_chirho("*");
                written_chirho.set_type_constructor_chirho(Some(
                    TypeConstructorDeclarationChirho::compiler_data_chirho(
                        CompilerTypeConstructorChirho::TypeUniverseChirho,
                    ),
                ));
                Self::ConChirho(written_chirho)
            }
            AstKindChirho::ConstraintChirho => {
                let mut written_chirho = name_chirho("Constraint");
                written_chirho.set_type_constructor_chirho(Some(
                    TypeConstructorDeclarationChirho::compiler_data_chirho(
                        CompilerTypeConstructorChirho::ConstraintUniverseChirho,
                    ),
                ));
                Self::ConChirho(written_chirho)
            }
            AstKindChirho::VarChirho(text_chirho) => {
                let written_chirho = name_chirho(&text_chirho);
                if text_chirho.chars().next().is_some_and(char::is_uppercase) {
                    Self::ConChirho(written_chirho)
                } else {
                    Self::VarChirho(written_chirho)
                }
            }
            AstKindChirho::AppChirho(fun_chirho, arg_chirho) => Self::AppChirho {
                fun_chirho: Box::new((*fun_chirho).into()),
                arg_chirho: Box::new((*arg_chirho).into()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            AstKindChirho::ArrowChirho(arg_chirho, result_chirho) => Self::FunChirho {
                arg_chirho: Box::new((*arg_chirho).into()),
                mult_chirho: None,
                result_chirho: Box::new((*result_chirho).into()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        }
    }
}
