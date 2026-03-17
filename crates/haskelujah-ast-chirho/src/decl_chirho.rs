// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Top-level declarations

use std::ops::Deref;

use haskelujah_span_chirho::SpanChirho;

use crate::expr_chirho::{LocalBindChirho, MatchArmChirho, RhsChirho};
use crate::name_chirho::NameChirho;
use crate::pat_chirho::PatChirho;
use crate::ty_chirho::{ConstraintChirho, TypeChirho};

/// AST-level kind annotation (written in source code with KindSignatures).
///
/// Represents kind expressions like `*`, `* -> *`, `(* -> *) -> *`.
#[derive(Debug, Clone, PartialEq)]
pub enum AstKindChirho {
    /// `*` or `Type` — the kind of types.
    StarChirho,
    /// `k1 -> k2` — arrow kind (type constructor kind).
    ArrowChirho(Box<AstKindChirho>, Box<AstKindChirho>),
    /// `Constraint` — the kind of typeclass constraints (ConstraintKinds).
    ConstraintChirho,
    /// Kind variable (PolyKinds): `k` in `(a :: k)`.
    VarChirho(String),
}

/// One equation in a closed type family:
/// `F Int = Bool` or `F [a] = a`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeFamilyEquationChirho {
    pub lhs_types_chirho: Vec<TypeChirho>,
    pub rhs_chirho: TypeChirho,
    pub span_chirho: SpanChirho,
}

/// A type variable, optionally annotated with a kind signature.
///
/// Without KindSignatures: `data Foo a = ...` → `TyVarChirho { name: a, kind: None }`
/// With KindSignatures: `data Foo (a :: *) = ...` → `TyVarChirho { name: a, kind: Some(Star) }`
#[derive(Debug, Clone, PartialEq)]
pub struct TyVarChirho {
    pub name_chirho: NameChirho,
    pub kind_annotation_chirho: Option<AstKindChirho>,
}

impl TyVarChirho {
    /// Create an unannotated type variable (no kind signature).
    pub fn plain_chirho(name_chirho: NameChirho) -> Self {
        Self {
            name_chirho,
            kind_annotation_chirho: None,
        }
    }

    /// Create a type variable with a kind annotation.
    pub fn annotated_chirho(name_chirho: NameChirho, kind_chirho: AstKindChirho) -> Self {
        Self {
            name_chirho,
            kind_annotation_chirho: Some(kind_chirho),
        }
    }
}

impl From<NameChirho> for TyVarChirho {
    fn from(name_chirho: NameChirho) -> Self {
        Self::plain_chirho(name_chirho)
    }
}

impl Deref for TyVarChirho {
    type Target = NameChirho;

    fn deref(&self) -> &NameChirho {
        &self.name_chirho
    }
}

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
        type_vars_chirho: Vec<TyVarChirho>,
        constructors_chirho: Vec<ConDeclChirho>,
        deriving_chirho: Vec<NameChirho>,
        /// Standalone kind signature for GADT-style: `data T :: K1 -> K2 -> Type where`
        kind_sig_chirho: Option<crate::ty_chirho::TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Newtype declaration (`newtype T a = Con Type`).
    NewtypeDeclChirho {
        name_chirho: NameChirho,
        type_vars_chirho: Vec<TyVarChirho>,
        constructor_chirho: ConDeclChirho,
        deriving_chirho: Vec<NameChirho>,
        /// Standalone kind signature for GADT-style newtype.
        kind_sig_chirho: Option<crate::ty_chirho::TypeChirho>,
        span_chirho: SpanChirho,
    },
    /// Type alias (`type Name = Type`).
    TypeAliasDeclChirho {
        name_chirho: NameChirho,
        type_vars_chirho: Vec<TyVarChirho>,
        rhs_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
    /// Type family declaration (open or closed).
    /// Open: `type family F a :: *`
    /// Closed: `type family F a where { F Int = Bool; ... }`
    TypeFamilyDeclChirho {
        name_chirho: NameChirho,
        type_vars_chirho: Vec<TyVarChirho>,
        result_kind_chirho: Option<TypeChirho>,
        /// Equations for closed families; empty for open families.
        equations_chirho: Vec<TypeFamilyEquationChirho>,
        span_chirho: SpanChirho,
    },
    /// Open type family instance (`type instance F Int = Bool`).
    TypeFamilyInstanceDeclChirho {
        family_name_chirho: NameChirho,
        lhs_types_chirho: Vec<TypeChirho>,
        rhs_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
    /// Type class declaration.
    ClassDeclChirho {
        context_chirho: Vec<ConstraintChirho>,
        name_chirho: NameChirho,
        type_vars_chirho: Vec<TyVarChirho>,
        methods_chirho: Vec<ClassMethodChirho>,
        /// Associated type families declared inside the class.
        associated_tfs_chirho: Vec<AssocTypeFamilyChirho>,
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
        /// Associated type family instances: `type FamName ConcreteType = ResultType`.
        assoc_tf_instances_chirho: Vec<AssocTfInstanceChirho>,
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
    /// Pattern synonym declaration.
    PatSynDeclChirho {
        name_chirho: NameChirho,
        /// Pattern variables bound by this synonym.
        args_chirho: Vec<NameChirho>,
        /// Directionality (unidirectional, implicitly/explicitly bidirectional).
        dir_chirho: PatSynDirChirho,
        /// The pattern this synonym expands to in pattern position.
        pat_chirho: PatChirho,
        span_chirho: SpanChirho,
    },
    /// Template Haskell splice at declaration level (`$(makeLenses ''Foo)`).
    SpliceDeclChirho {
        expr_chirho: crate::expr_chirho::ExprChirho,
        span_chirho: SpanChirho,
    },
    /// Standalone deriving declaration (`deriving instance Show Foo`).
    StandaloneDerivingDeclChirho {
        context_chirho: Vec<ConstraintChirho>,
        class_chirho: NameChirho,
        types_chirho: Vec<TypeChirho>,
        span_chirho: SpanChirho,
    },
}

/// Directionality of a pattern synonym.
#[derive(Debug, Clone, PartialEq)]
pub enum PatSynDirChirho {
    /// `pattern P x = pat` — usable as both pattern and expression.
    ImplBidirChirho,
    /// `pattern P x <- pat` — usable only in pattern position.
    UnidirChirho,
    /// `pattern P x <- pat where P x = expr` — separate match and builder.
    ExplBidirChirho {
        builder_binds_chirho: Vec<LocalBindChirho>,
    },
}

/// Field strictness annotation for data constructor fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrictnessChirho {
    /// Default (lazy) field.
    LazyChirho,
    /// Strict field (`!Type`).
    StrictChirho,
    /// Strict and unpacked field (`{-# UNPACK #-} !Type`).
    UnpackChirho,
}

/// A data constructor declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum ConDeclChirho {
    /// Ordinary constructor (`Con Type1 !Type2`).
    OrdinaryChirho {
        name_chirho: NameChirho,
        fields_chirho: Vec<(StrictnessChirho, TypeChirho)>,
        span_chirho: SpanChirho,
    },
    /// Record constructor (`Con { field1 :: Type1, field2 :: Type2 }`).
    RecordChirho {
        name_chirho: NameChirho,
        fields_chirho: Vec<FieldDeclChirho>,
        span_chirho: SpanChirho,
    },
    /// GADT constructor (`Con :: forall a. Ctx => Arg -> ... -> T Int a`).
    /// Preserves the full type signature including return type for type refinement.
    GadtChirho {
        name_chirho: NameChirho,
        /// The full type signature after `::`.
        ty_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
}

/// A record field declaration (`fieldName :: !Type`).
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDeclChirho {
    pub names_chirho: Vec<NameChirho>,
    pub ty_chirho: TypeChirho,
    pub strictness_chirho: StrictnessChirho,
    pub span_chirho: SpanChirho,
}

/// A method in a class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethodChirho {
    pub name_chirho: NameChirho,
    pub ty_chirho: TypeChirho,
    pub default_chirho: Option<Vec<MatchArmChirho>>,
    /// Default signature from `{-# LANGUAGE DefaultSignatures #-}`:
    /// `default methodName :: MoreConstrained => Type`.
    /// Stores the raw type text for the more-constrained default method type.
    pub default_sig_chirho: Option<String>,
    pub span_chirho: SpanChirho,
}

/// An associated type family declaration inside a class:
/// `type FamilyName a :: *` or `type FamilyName a` (with optional default).
#[derive(Debug, Clone, PartialEq)]
pub struct AssocTypeFamilyChirho {
    pub name_chirho: NameChirho,
    pub type_vars_chirho: Vec<NameChirho>,
    /// Optional default: `type FamilyName a = DefaultType`.
    pub default_rhs_chirho: Option<TypeChirho>,
    pub span_chirho: SpanChirho,
}

/// An associated type family instance inside an instance declaration:
/// `type FamilyName ConcreteType = ResultType`.
#[derive(Debug, Clone, PartialEq)]
pub struct AssocTfInstanceChirho {
    pub family_name_chirho: NameChirho,
    pub lhs_types_chirho: Vec<TypeChirho>,
    pub rhs_chirho: TypeChirho,
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
            | Self::ForeignDeclChirho { span_chirho, .. }
            | Self::PatSynDeclChirho { span_chirho, .. }
            | Self::TypeFamilyDeclChirho { span_chirho, .. }
            | Self::TypeFamilyInstanceDeclChirho { span_chirho, .. }
            | Self::SpliceDeclChirho { span_chirho, .. }
            | Self::StandaloneDerivingDeclChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
