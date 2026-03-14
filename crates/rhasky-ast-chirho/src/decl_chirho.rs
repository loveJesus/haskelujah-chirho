// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Top-level declarations

use rhasky_span_chirho::SpanChirho;

use crate::expr_chirho::{LocalBindChirho, MatchArmChirho, RhsChirho};
use crate::name_chirho::NameChirho;
use crate::pat_chirho::PatChirho;
use crate::ty_chirho::{ConstraintChirho, TypeChirho};

/// A top-level declaration in a Haskell module.
#[derive(Debug, Clone, PartialEq)]
pub enum DeclChirho {
    /// Type signature (`foo :: Type`).
    TypeSigChirho {
        name_chirho: NameChirho,
        ty_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
    /// Function binding (`foo x y = ...` with possibly multiple equations).
    FunBindChirho {
        name_chirho: NameChirho,
        matches_chirho: Vec<MatchArmChirho>,
        span_chirho: SpanChirho,
    },
    /// Pattern binding (`(a, b) = expr`).
    PatBindChirho {
        pat_chirho: PatChirho,
        rhs_chirho: RhsChirho,
        span_chirho: SpanChirho,
    },
    /// Data type declaration (`data T a = C1 | C2`).
    DataDeclChirho {
        name_chirho: NameChirho,
        type_vars_chirho: Vec<NameChirho>,
        constructors_chirho: Vec<ConDeclChirho>,
        deriving_chirho: Vec<NameChirho>,
        span_chirho: SpanChirho,
    },
    /// Newtype declaration (`newtype T a = Con Type`).
    NewtypeDeclChirho {
        name_chirho: NameChirho,
        type_vars_chirho: Vec<NameChirho>,
        constructor_chirho: ConDeclChirho,
        deriving_chirho: Vec<NameChirho>,
        span_chirho: SpanChirho,
    },
    /// Type alias (`type Name = Type`).
    TypeAliasDeclChirho {
        name_chirho: NameChirho,
        type_vars_chirho: Vec<NameChirho>,
        rhs_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
    /// Type class declaration.
    ClassDeclChirho {
        context_chirho: Vec<ConstraintChirho>,
        name_chirho: NameChirho,
        type_vars_chirho: Vec<NameChirho>,
        methods_chirho: Vec<ClassMethodChirho>,
        /// Functional dependencies: `| a -> b, c -> d`.
        /// Each pair `(from_vars, to_vars)` means the from-vars determine the to-vars.
        fundeps_chirho: Vec<(Vec<String>, Vec<String>)>,
        span_chirho: SpanChirho,
    },
    /// Instance declaration.
    InstanceDeclChirho {
        context_chirho: Vec<ConstraintChirho>,
        class_chirho: NameChirho,
        types_chirho: Vec<TypeChirho>,
        methods_chirho: Vec<LocalBindChirho>,
        span_chirho: SpanChirho,
    },
    /// Fixity declaration (`infixl 6 +`).
    FixityDeclChirho {
        fixity_chirho: FixityChirho,
        precedence_chirho: Option<u8>,
        ops_chirho: Vec<NameChirho>,
        span_chirho: SpanChirho,
    },
    /// Default declaration (`default (Int, Double)`).
    DefaultDeclChirho {
        types_chirho: Vec<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Foreign declaration (`foreign import`/`foreign export`).
    ForeignDeclChirho {
        direction_chirho: ForeignDirectionChirho,
        name_chirho: NameChirho,
        ty_chirho: TypeChirho,
        calling_conv_chirho: String,
        safety_chirho: Option<String>,
        foreign_name_chirho: Option<String>,
        span_chirho: SpanChirho,
    },
}

/// A data constructor declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum ConDeclChirho {
    /// Ordinary constructor (`Con Type1 Type2`).
    OrdinaryChirho {
        name_chirho: NameChirho,
        fields_chirho: Vec<TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Record constructor (`Con { field1 :: Type1, field2 :: Type2 }`).
    RecordChirho {
        name_chirho: NameChirho,
        fields_chirho: Vec<FieldDeclChirho>,
        span_chirho: SpanChirho,
    },
}

/// A record field declaration (`fieldName :: Type`).
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDeclChirho {
    pub names_chirho: Vec<NameChirho>,
    pub ty_chirho: TypeChirho,
    pub span_chirho: SpanChirho,
}

/// A method in a class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethodChirho {
    pub name_chirho: NameChirho,
    pub ty_chirho: TypeChirho,
    pub default_chirho: Option<Vec<MatchArmChirho>>,
    pub span_chirho: SpanChirho,
}

/// Fixity direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixityChirho {
    InfixChirho,
    InfixlChirho,
    InfixrChirho,
}

/// Foreign declaration direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForeignDirectionChirho {
    /// `foreign import` — bring a C function into Haskell.
    ImportChirho,
    /// `foreign export` — expose a Haskell function to C.
    ExportChirho,
}

impl DeclChirho {
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::TypeSigChirho { span_chirho, .. }
            | Self::FunBindChirho { span_chirho, .. }
            | Self::PatBindChirho { span_chirho, .. }
            | Self::DataDeclChirho { span_chirho, .. }
            | Self::NewtypeDeclChirho { span_chirho, .. }
            | Self::TypeAliasDeclChirho { span_chirho, .. }
            | Self::ClassDeclChirho { span_chirho, .. }
            | Self::InstanceDeclChirho { span_chirho, .. }
            | Self::FixityDeclChirho { span_chirho, .. }
            | Self::DefaultDeclChirho { span_chirho, .. }
            | Self::ForeignDeclChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
