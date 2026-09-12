// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Top-level declarations

use std::borrow::Cow;
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
    /// A nominal kind constructor, retaining qualification and source span.
    /// It is not a lexical variable and must never be implicitly quantified.
    ConChirho(NameChirho),
    /// Kind application: `TYPE representation` in `(a :: TYPE representation)`.
    AppChirho(Box<AstKindChirho>, Box<AstKindChirho>),
    /// Explicit invisible application, distinct from an ordinary kind argument.
    KindAppChirho(Box<AstKindChirho>, Box<AstKindChirho>),
    /// Lexically quantified kind, as in `(f :: forall k. k -> Type)`.
    ForallChirho {
        vars_chirho: Vec<TyVarChirho>,
        body_chirho: Box<AstKindChirho>,
        span_chirho: SpanChirho,
    },
    /// Required quantification (`forall k -> ...`), not invisible instantiation.
    RequiredForallChirho {
        vars_chirho: Vec<TyVarChirho>,
        body_chirho: Box<AstKindChirho>,
        span_chirho: SpanChirho,
    },
}

/// One equation in a closed type family:
/// `F Int = Bool` or `F [a] = a`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeFamilyEquationChirho {
    /// The family arguments on the left-hand side of the equation.
    pub lhs_types_chirho: Vec<TypeChirho>,
    /// The reduced result type on the right-hand side.
    pub rhs_chirho: TypeChirho,
    /// Span covering the whole equation.
    pub span_chirho: SpanChirho,
}

/// Written family result contract. Naming a result does not itself promise
/// injectivity; equation validation must establish that separate annotation.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TypeFamilyResultChirho {
    /// Full head and inline result contracts have distinct scopes.
    pub kind_sig_chirho: Option<DeclKindSigChirho>,
    pub binder_chirho: Option<NameChirho>,
    pub injectivity_chirho: Option<TypeFamilyInjectivityChirho>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFamilyInjectivityChirho {
    pub result_chirho: NameChirho,
    pub parameters_chirho: Vec<NameChirho>,
    pub span_chirho: SpanChirho,
}

/// Whether a declaration-head binder consumes an ordinary type argument.
/// This is not the specified/inferred distinction on invisible forall binders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TyVarVisibilityChirho {
    /// An ordinary head parameter, written `a` or `(a :: k)`.
    VisibleChirho,
    /// A scoped invisible parameter, written `@a` or `@(a :: k)`.
    InvisibleChirho,
}

/// Whether an invisible forall binder can be selected with visible `@` syntax.
/// This is independent of whether a declaration-head argument is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TyVarSpecificityChirho {
    SpecifiedChirho,
    InferredChirho,
}

/// A type variable, optionally annotated with a kind signature.
///
/// Without KindSignatures: `data Foo a = ...` → `TyVarChirho { name: a, kind: None }`
/// With KindSignatures: `data Foo (a :: *) = ...` → `TyVarChirho { name: a, kind: Some(Star) }`
#[derive(Debug, Clone, PartialEq)]
pub struct TyVarChirho {
    /// The source name of the type variable.
    pub name_chirho: NameChirho,
    /// Optional kind annotation supplied in source.
    pub kind_annotation_chirho: Option<AstKindChirho>,
    /// Head binders remain in lexical scope even when they are invisible.
    pub visibility_chirho: TyVarVisibilityChirho,
    /// Braced forall binders (`forall {k}.`) are inferred, never selected by `@`.
    pub specificity_chirho: TyVarSpecificityChirho,
}

impl TyVarChirho {
    /// Create an unannotated type variable (no kind signature).
    pub fn plain_chirho(name_chirho: NameChirho) -> Self {
        Self {
            name_chirho,
            kind_annotation_chirho: None,
            visibility_chirho: TyVarVisibilityChirho::VisibleChirho,
            specificity_chirho: TyVarSpecificityChirho::SpecifiedChirho,
        }
    }

    /// Create a type variable with a kind annotation.
    pub fn annotated_chirho(name_chirho: NameChirho, kind_chirho: AstKindChirho) -> Self {
        Self {
            name_chirho,
            kind_annotation_chirho: Some(kind_chirho),
            visibility_chirho: TyVarVisibilityChirho::VisibleChirho,
            specificity_chirho: TyVarSpecificityChirho::SpecifiedChirho,
        }
    }

    /// Whether this binder belongs in an ordinary type-constructor application.
    pub fn is_visible_chirho(&self) -> bool {
        self.visibility_chirho == TyVarVisibilityChirho::VisibleChirho
    }

    /// Adapt parameter-only consumers without copying ordinary declarations.
    /// Lexical consumers must retain the original complete binder sequence.
    pub fn visible_binders_chirho(binders_chirho: &[Self]) -> Cow<'_, [Self]> {
        if binders_chirho.iter().all(Self::is_visible_chirho) {
            Cow::Borrowed(binders_chirho)
        } else {
            Cow::Owned(
                binders_chirho
                    .iter()
                    .filter(|binder_chirho| binder_chirho.is_visible_chirho())
                    .cloned()
                    .collect(),
            )
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

/// Written kind contracts on a declaration. A result annotation
/// follows the declaration's binders; a standalone signature describes the
/// complete constructor kind in an independent lexical scope. Keep both when
/// both are written. See language-features-chirho/declaration-kinds-chirho.
#[derive(Debug, Clone, PartialEq)]
pub enum DeclKindSigChirho {
    /// `data T a :: K where`: only the kind remaining after `a`.
    ResultChirho(TypeChirho),
    /// `type T :: K`: the complete kind, optionally accompanied by an inline tail.
    StandaloneChirho {
        /// Complete constructor kind in the standalone signature's scope.
        signature_chirho: TypeChirho,
        /// Separately written tail in the declaration-head scope, if any.
        result_chirho: Option<TypeChirho>,
    },
}

impl DeclKindSigChirho {
    /// The explicitly written inline result kind, never a synthesized tail.
    pub fn result_chirho(&self) -> Option<&TypeChirho> {
        match self {
            Self::ResultChirho(result_chirho) => Some(result_chirho),
            Self::StandaloneChirho { result_chirho, .. } => result_chirho.as_ref(),
        }
    }

    /// The complete standalone signature, never an inline annotation.
    pub fn standalone_chirho(&self) -> Option<&TypeChirho> {
        match self {
            Self::ResultChirho(_) => None,
            Self::StandaloneChirho {
                signature_chirho, ..
            } => Some(signature_chirho),
        }
    }
}

/// A top-level declaration in a Haskell module.
#[derive(Debug, Clone, PartialEq)]
pub enum DeclChirho {
    /// Type signature (`foo :: Type`).
    TypeSigChirho {
        /// Name whose type is being declared.
        name_chirho: NameChirho,
        /// Declared type for the binding.
        ty_chirho: TypeChirho,
        /// Span covering the whole type signature.
        span_chirho: SpanChirho,
    },
    /// Function binding (`foo x y = ...` with possibly multiple equations).
    FunBindChirho {
        /// Bound top-level function name.
        name_chirho: NameChirho,
        /// Clauses implementing the function.
        matches_chirho: Vec<MatchArmChirho>,
        /// Span covering the whole binding group.
        span_chirho: SpanChirho,
    },
    /// Pattern binding (`(a, b) = expr`).
    PatBindChirho {
        /// Pattern introduced by the binding.
        pat_chirho: PatChirho,
        /// Right-hand side assigned to the pattern.
        rhs_chirho: RhsChirho,
        /// Span covering the whole pattern binding.
        span_chirho: SpanChirho,
    },
    /// Data type declaration (`data T a = C1 | C2`).
    DataDeclChirho {
        /// Declared type constructor name.
        name_chirho: NameChirho,
        /// Type parameters introduced by the declaration.
        type_vars_chirho: Vec<TyVarChirho>,
        /// Constructors belonging to the data type.
        constructors_chirho: Vec<ConDeclChirho>,
        /// Classes listed in the deriving clause.
        deriving_chirho: Vec<NameChirho>,
        /// Optional inline result kind and/or complete standalone kind signature.
        kind_sig_chirho: Option<DeclKindSigChirho>,
        /// Span covering the whole data declaration.
        span_chirho: SpanChirho,
    },
    /// Newtype declaration (`newtype T a = Con Type`).
    NewtypeDeclChirho {
        /// Declared newtype constructor name.
        name_chirho: NameChirho,
        /// Type parameters introduced by the declaration.
        type_vars_chirho: Vec<TyVarChirho>,
        /// The single runtime constructor carried by the newtype.
        constructor_chirho: ConDeclChirho,
        /// Classes listed in the deriving clause.
        deriving_chirho: Vec<NameChirho>,
        /// Optional inline result kind and/or complete standalone kind signature.
        kind_sig_chirho: Option<DeclKindSigChirho>,
        /// Span covering the whole newtype declaration.
        span_chirho: SpanChirho,
    },
    /// Type alias (`type Name = Type`).
    TypeAliasDeclChirho {
        /// Alias name being introduced.
        name_chirho: NameChirho,
        /// Type parameters accepted by the alias.
        type_vars_chirho: Vec<TyVarChirho>,
        /// Aliased right-hand-side type.
        rhs_chirho: TypeChirho,
        /// Span covering the whole type alias.
        span_chirho: SpanChirho,
    },
    /// Type family declaration (open or closed).
    /// Open: `type family F a :: *`
    /// Closed: `type family F a where { F Int = Bool; ... }`
    TypeFamilyDeclChirho {
        /// Family name being declared.
        name_chirho: NameChirho,
        /// Family parameters introduced by the declaration.
        type_vars_chirho: Vec<TyVarChirho>,
        /// Result kind, optional named binder and written injectivity contract.
        result_chirho: TypeFamilyResultChirho,
        /// `where {}` is closed even when it contains no equations.
        closed_chirho: bool,
        /// Equations for closed families; empty for open families.
        equations_chirho: Vec<TypeFamilyEquationChirho>,
        /// Span covering the whole family declaration.
        span_chirho: SpanChirho,
    },
    /// Open type family instance (`type instance F Int = Bool`).
    TypeFamilyInstanceDeclChirho {
        /// Family being instantiated.
        family_name_chirho: NameChirho,
        /// Left-hand-side instance arguments.
        lhs_types_chirho: Vec<TypeChirho>,
        /// Reduced result type for the instance.
        rhs_chirho: TypeChirho,
        /// Span covering the whole family instance declaration.
        span_chirho: SpanChirho,
    },
    /// Type class declaration.
    ClassDeclChirho {
        /// Class context required by the declaration head.
        context_chirho: Vec<ConstraintChirho>,
        /// Class name being introduced.
        name_chirho: NameChirho,
        /// Class type parameters.
        type_vars_chirho: Vec<TyVarChirho>,
        /// Method signatures and optional defaults declared in the class body.
        methods_chirho: Vec<ClassMethodChirho>,
        /// Associated type families declared inside the class.
        associated_tfs_chirho: Vec<AssocTypeFamilyChirho>,
        /// Functional dependencies: `| a -> b, c -> d`.
        /// Each pair `(from_vars, to_vars)` means the from-vars determine the to-vars.
        fundeps_chirho: Vec<(Vec<String>, Vec<String>)>,
        /// Span covering the whole class declaration.
        span_chirho: SpanChirho,
    },
    /// Instance declaration.
    InstanceDeclChirho {
        /// Constraints required by the instance head.
        context_chirho: Vec<ConstraintChirho>,
        /// Class implemented by the instance.
        class_chirho: NameChirho,
        /// Concrete instance head arguments.
        types_chirho: Vec<TypeChirho>,
        /// Method implementations provided by the instance body.
        methods_chirho: Vec<LocalBindChirho>,
        /// Associated type family instances: `type FamName ConcreteType = ResultType`.
        assoc_tf_instances_chirho: Vec<AssocTfInstanceChirho>,
        /// Span covering the whole instance declaration.
        span_chirho: SpanChirho,
    },
    /// Fixity declaration (`infixl 6 +`).
    FixityDeclChirho {
        /// Associativity declared for the operators.
        fixity_chirho: FixityChirho,
        /// Optional precedence level.
        precedence_chirho: Option<u8>,
        /// Operators whose fixity is being declared.
        ops_chirho: Vec<NameChirho>,
        /// Span covering the whole fixity declaration.
        span_chirho: SpanChirho,
    },
    /// Default declaration (`default (Int, Double)`).
    DefaultDeclChirho {
        /// Fallback defaulted types used by ambiguous numeric inference.
        types_chirho: Vec<TypeChirho>,
        /// Span covering the whole default declaration.
        span_chirho: SpanChirho,
    },
    /// Foreign declaration (`foreign import`/`foreign export`).
    ForeignDeclChirho {
        /// Whether the declaration imports or exports a symbol.
        direction_chirho: ForeignDirectionChirho,
        /// Local Haskell binding name.
        name_chirho: NameChirho,
        /// Haskell type attached to the foreign binding.
        ty_chirho: TypeChirho,
        /// Calling convention such as `ccall` or `stdcall`.
        calling_conv_chirho: String,
        /// Optional safety annotation (`safe`, `unsafe`, `interruptible`).
        safety_chirho: Option<String>,
        /// Optional foreign symbol name when it differs from the local binding name.
        foreign_name_chirho: Option<String>,
        /// Span covering the whole foreign declaration.
        span_chirho: SpanChirho,
    },
    /// Pattern synonym declaration.
    PatSynDeclChirho {
        /// Pattern synonym name being introduced.
        name_chirho: NameChirho,
        /// Pattern variables bound by this synonym.
        args_chirho: Vec<NameChirho>,
        /// Directionality (unidirectional, implicitly/explicitly bidirectional).
        dir_chirho: PatSynDirChirho,
        /// The pattern this synonym expands to in pattern position.
        pat_chirho: PatChirho,
        /// Span covering the whole pattern synonym declaration.
        span_chirho: SpanChirho,
    },
    /// Template Haskell splice at declaration level (`$(makeLenses ''Foo)`).
    SpliceDeclChirho {
        /// Splice expression to execute during compilation.
        expr_chirho: crate::expr_chirho::ExprChirho,
        /// Span covering the whole splice declaration.
        span_chirho: SpanChirho,
    },
    /// Standalone deriving declaration (`deriving instance Show Foo`).
    StandaloneDerivingDeclChirho {
        /// Constraints attached to the deriving instance head.
        context_chirho: Vec<ConstraintChirho>,
        /// Class being derived.
        class_chirho: NameChirho,
        /// Instance head types receiving the derived instance.
        types_chirho: Vec<TypeChirho>,
        /// Span covering the whole standalone deriving declaration.
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
        /// Builder-side bindings used when the synonym appears in expression position.
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
        /// Constructor name.
        name_chirho: NameChirho,
        /// Positional constructor fields and their strictness.
        fields_chirho: Vec<(StrictnessChirho, TypeChirho)>,
        /// Span covering the whole constructor declaration.
        span_chirho: SpanChirho,
    },
    /// Record constructor (`Con { field1 :: Type1, field2 :: Type2 }`).
    RecordChirho {
        /// Constructor name.
        name_chirho: NameChirho,
        /// Named record fields declared by the constructor.
        fields_chirho: Vec<FieldDeclChirho>,
        /// Span covering the whole constructor declaration.
        span_chirho: SpanChirho,
    },
    /// GADT constructor (`Con :: forall a. Ctx => Arg -> ... -> T Int a`).
    /// Preserves the full type signature including return type for type refinement.
    GadtChirho {
        /// Constructor name.
        name_chirho: NameChirho,
        /// The full type signature after `::`.
        ty_chirho: TypeChirho,
        /// Span covering the whole constructor declaration.
        span_chirho: SpanChirho,
    },
}

/// A record field declaration (`fieldName :: !Type`).
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDeclChirho {
    /// Field names that share the same type declaration.
    pub names_chirho: Vec<NameChirho>,
    /// The declared field type.
    pub ty_chirho: TypeChirho,
    /// Strictness annotation attached to the field.
    pub strictness_chirho: StrictnessChirho,
    /// Span covering the whole field declaration.
    pub span_chirho: SpanChirho,
}

/// A method in a class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethodChirho {
    /// Method name.
    pub name_chirho: NameChirho,
    /// Declared method type.
    pub ty_chirho: TypeChirho,
    /// Optional default implementation clauses.
    pub default_chirho: Option<Vec<MatchArmChirho>>,
    /// Default signature from `{-# LANGUAGE DefaultSignatures #-}`:
    /// `default methodName :: MoreConstrained => Type`.
    /// Stores the raw type text for the more-constrained default method type.
    pub default_sig_chirho: Option<String>,
    /// Span covering the whole method declaration.
    pub span_chirho: SpanChirho,
}

/// An associated type family declaration inside a class:
/// `type FamilyName a :: *` or `type FamilyName a` (with optional default).
#[derive(Debug, Clone, PartialEq)]
pub struct AssocTypeFamilyChirho {
    /// Name of the associated type family.
    pub name_chirho: NameChirho,
    /// Family parameters declared in the class body.
    pub type_vars_chirho: Vec<NameChirho>,
    /// Optional default: `type FamilyName a = DefaultType`.
    pub default_rhs_chirho: Option<TypeChirho>,
    /// The binders the default equation was written with (`type F a x = …`
    /// may name them differently from the family declaration); empty when
    /// they coincide with `type_vars_chirho`.
    pub default_params_chirho: Vec<NameChirho>,
    /// Span covering the whole associated type declaration.
    pub span_chirho: SpanChirho,
}

/// An associated type family instance inside an instance declaration:
/// `type FamilyName ConcreteType = ResultType`.
#[derive(Debug, Clone, PartialEq)]
pub struct AssocTfInstanceChirho {
    /// Associated family name being instantiated.
    pub family_name_chirho: NameChirho,
    /// Concrete left-hand-side arguments for the instance.
    pub lhs_types_chirho: Vec<TypeChirho>,
    /// Result type produced by this associated family instance.
    pub rhs_chirho: TypeChirho,
    /// Span covering the whole associated type instance.
    pub span_chirho: SpanChirho,
}

/// Fixity direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixityChirho {
    /// Non-associative infix operator.
    InfixChirho,
    /// Left-associative infix operator.
    InfixlChirho,
    /// Right-associative infix operator.
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
    /// Return the source span covering this top-level declaration.
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
