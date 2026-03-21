// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Template Haskell AST types
//!
//! Mirrors `Language.Haskell.TH.Syntax` — the types that TH splice functions
//! produce and consume. These are distinct from the compiler's own AST types
//! (`haskelujah-ast-chirho`) because TH is a stable API surface.

use std::fmt;

// ---------------------------------------------------------------------------
// Names
// ---------------------------------------------------------------------------

/// How a TH `Name` was introduced.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThNameFlavourChirho {
    /// Global name from a known module: `Name "foo" (NameG VarName "Module.Name")`.
    GlobalChirho {
        namespace_chirho: ThNameSpaceChirho,
        module_chirho: String,
        package_chirho: Option<String>,
    },
    /// Unique/fresh name from `newName`: `Name "x" (NameU 42)`.
    UniqueChirho(u64),
    /// Local/captured name from a quotation: `Name "x" NameL`.
    LocalChirho,
    /// String-based name from `mkName`: `Name "foo" NameS`.
    StringChirho,
}

/// Whether a name refers to a value-level or type-level entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThNameSpaceChirho {
    /// Functions, variables, data constructors.
    VarNameChirho,
    /// Data constructors specifically.
    DataNameChirho,
    /// Type constructors, type classes.
    TcClsNameChirho,
}

/// A Template Haskell name — the central identifier type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThNameChirho {
    pub occ_chirho: String,
    pub flavour_chirho: ThNameFlavourChirho,
}

impl ThNameChirho {
    /// Create a simple string-based name (from `mkName`).
    pub fn mk_name_chirho(s_chirho: &str) -> Self {
        Self {
            occ_chirho: s_chirho.to_string(),
            flavour_chirho: ThNameFlavourChirho::StringChirho,
        }
    }

    /// Create a global name with module qualification.
    pub fn global_chirho(
        occ_chirho: &str,
        module_chirho: &str,
        ns_chirho: ThNameSpaceChirho,
    ) -> Self {
        Self {
            occ_chirho: occ_chirho.to_string(),
            flavour_chirho: ThNameFlavourChirho::GlobalChirho {
                namespace_chirho: ns_chirho,
                module_chirho: module_chirho.to_string(),
                package_chirho: None,
            },
        }
    }

    /// Create a unique name (from `newName`).
    pub fn unique_chirho(occ_chirho: &str, id_chirho: u64) -> Self {
        Self {
            occ_chirho: occ_chirho.to_string(),
            flavour_chirho: ThNameFlavourChirho::UniqueChirho(id_chirho),
        }
    }
}

impl fmt::Display for ThNameChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.flavour_chirho {
            ThNameFlavourChirho::GlobalChirho { module_chirho, .. } => {
                write!(f_chirho, "{}.{}", module_chirho, self.occ_chirho)
            }
            ThNameFlavourChirho::UniqueChirho(id_chirho) => {
                write!(f_chirho, "{}_{}", self.occ_chirho, id_chirho)
            }
            _ => write!(f_chirho, "{}", self.occ_chirho),
        }
    }
}

// ---------------------------------------------------------------------------
// Literals
// ---------------------------------------------------------------------------

/// A TH literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum ThLitChirho {
    IntegerLChirho(i64),
    RationalLChirho(f64),
    CharLChirho(char),
    StringLChirho(String),
    IntPrimLChirho(i64),
    WordPrimLChirho(u64),
    FloatPrimLChirho(f64),
    DoublePrimLChirho(f64),
    StringPrimLChirho(Vec<u8>),
    BytesPrimLChirho(Vec<u8>),
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

/// A TH expression — mirrors `Language.Haskell.TH.Syntax.Exp`.
#[derive(Debug, Clone, PartialEq)]
pub enum ThExpChirho {
    /// Variable: `x`, `Data.Map.insert`.
    VarEChirho(ThNameChirho),
    /// Data constructor: `Just`, `Left`.
    ConEChirho(ThNameChirho),
    /// Literal: `42`, `"hello"`.
    LitEChirho(ThLitChirho),
    /// Application: `f x`.
    AppEChirho(Box<ThExpChirho>, Box<ThExpChirho>),
    /// Type application: `f @Int`.
    AppTypeEChirho(Box<ThExpChirho>, Box<ThTypeChirho>),
    /// Infix application: `x + y` as `InfixE (Just x) (+) (Just y)`.
    InfixEChirho(
        Option<Box<ThExpChirho>>,
        Box<ThExpChirho>,
        Option<Box<ThExpChirho>>,
    ),
    /// Unboxed infix application.
    UnboundVarEChirho(ThNameChirho),
    /// Lambda: `\p1 p2 -> body`.
    LamEChirho(Vec<ThPatChirho>, Box<ThExpChirho>),
    /// Lambda-case: `\case { p1 -> e1; p2 -> e2 }`.
    LamCaseEChirho(Vec<ThMatchChirho>),
    /// Tuple: `(a, b, c)`.
    TupEChirho(Vec<Option<ThExpChirho>>),
    /// Unboxed tuple.
    UnboxedTupEChirho(Vec<Option<ThExpChirho>>),
    /// Conditional: `if c then t else e`.
    CondEChirho(Box<ThExpChirho>, Box<ThExpChirho>, Box<ThExpChirho>),
    /// Multi-way if: `if | g1 -> e1 | g2 -> e2`.
    MultiIfEChirho(Vec<(ThGuardChirho, ThExpChirho)>),
    /// Let: `let decs in exp`.
    LetEChirho(Vec<ThDecChirho>, Box<ThExpChirho>),
    /// Case: `case scrut of { matches }`.
    CaseEChirho(Box<ThExpChirho>, Vec<ThMatchChirho>),
    /// Do expression: `do { stmts }`.
    DoEChirho(Option<String>, Vec<ThStmtChirho>),
    /// Monad comprehension.
    MDoEChirho(Option<String>, Vec<ThStmtChirho>),
    /// Comprehension: `[body | quals]`.
    CompEChirho(Vec<ThStmtChirho>),
    /// Arithmetic sequence: `[from..to]`.
    ArithSeqEChirho(Box<ThRangeChirho>),
    /// List literal: `[1, 2, 3]`.
    ListEChirho(Vec<ThExpChirho>),
    /// Type signature: `expr :: type`.
    SigEChirho(Box<ThExpChirho>, Box<ThTypeChirho>),
    /// Record construction: `Con { f1 = e1 }`.
    RecConEChirho(ThNameChirho, Vec<ThFieldExpChirho>),
    /// Record update: `expr { f1 = e1 }`.
    RecUpdEChirho(Box<ThExpChirho>, Vec<ThFieldExpChirho>),
    /// Static pointer: `static expr`.
    StaticEChirho(Box<ThExpChirho>),
    /// Parenthesized for pretty-printing.
    ParensEChirho(Box<ThExpChirho>),
    /// Type annotation in expression position: `(e :: t)`.
    GetFieldEChirho(Box<ThExpChirho>, String),
    /// Projection: `(.fieldName)`.
    ProjectionEChirho(Vec<String>),
}

/// A field expression: `fieldName = expr`.
#[derive(Debug, Clone, PartialEq)]
pub struct ThFieldExpChirho {
    pub name_chirho: ThNameChirho,
    pub expr_chirho: ThExpChirho,
}

/// A range for arithmetic sequences.
#[derive(Debug, Clone, PartialEq)]
pub enum ThRangeChirho {
    FromRChirho(ThExpChirho),
    FromThenRChirho(ThExpChirho, ThExpChirho),
    FromToRChirho(ThExpChirho, ThExpChirho),
    FromThenToRChirho(ThExpChirho, ThExpChirho, ThExpChirho),
}

/// A case match: `pat -> body where decs`.
#[derive(Debug, Clone, PartialEq)]
pub struct ThMatchChirho {
    pub pat_chirho: ThPatChirho,
    pub body_chirho: ThBodyChirho,
    pub decs_chirho: Vec<ThDecChirho>,
}

/// A guard expression.
#[derive(Debug, Clone, PartialEq)]
pub enum ThGuardChirho {
    NormalGChirho(ThExpChirho),
    PatGChirho(ThPatChirho, ThExpChirho),
}

/// A statement in do-notation or list comprehension.
#[derive(Debug, Clone, PartialEq)]
pub enum ThStmtChirho {
    /// `pat <- expr`.
    BindSChirho(ThPatChirho, ThExpChirho),
    /// `let decs`.
    LetSChirho(Vec<ThDecChirho>),
    /// Bare expression.
    NoBindSChirho(ThExpChirho),
    /// Parallel list comprehension.
    ParSChirho(Vec<Vec<ThStmtChirho>>),
    /// `rec { stmts }` (recursive do).
    RecSChirho(Vec<ThStmtChirho>),
}

/// A body — either unguarded or guarded.
#[derive(Debug, Clone, PartialEq)]
pub enum ThBodyChirho {
    /// Unguarded: `= expr`.
    NormalBChirho(ThExpChirho),
    /// Guarded: `| g1 = e1 | g2 = e2`.
    GuardedBChirho(Vec<(ThGuardChirho, ThExpChirho)>),
}

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

/// A TH pattern.
#[derive(Debug, Clone, PartialEq)]
pub enum ThPatChirho {
    /// Literal pattern: `42`.
    LitPChirho(ThLitChirho),
    /// Variable pattern: `x`.
    VarPChirho(ThNameChirho),
    /// Tuple pattern: `(p1, p2)`.
    TupPChirho(Vec<ThPatChirho>),
    /// Unboxed tuple pattern.
    UnboxedTupPChirho(Vec<ThPatChirho>),
    /// Unboxed sum pattern.
    UnboxedSumPChirho(Box<ThPatChirho>, i32, i32),
    /// Constructor application: `Just x`.
    ConPChirho(ThNameChirho, Vec<ThTypeChirho>, Vec<ThPatChirho>),
    /// Infix constructor: `x : xs`.
    InfixPChirho(Box<ThPatChirho>, ThNameChirho, Box<ThPatChirho>),
    /// Tilde (lazy) pattern: `~pat`.
    TildePChirho(Box<ThPatChirho>),
    /// Bang pattern: `!pat`.
    BangPChirho(Box<ThPatChirho>),
    /// As-pattern: `x@pat`.
    AsPChirho(ThNameChirho, Box<ThPatChirho>),
    /// Wildcard: `_`.
    WildPChirho,
    /// Record pattern: `Con { f1 = p1 }`.
    RecPChirho(ThNameChirho, Vec<ThFieldPatChirho>),
    /// List pattern: `[p1, p2]`.
    ListPChirho(Vec<ThPatChirho>),
    /// Type signature pattern: `(pat :: type)`.
    SigPChirho(Box<ThPatChirho>, Box<ThTypeChirho>),
    /// View pattern: `(f -> pat)`.
    ViewPChirho(Box<ThExpChirho>, Box<ThPatChirho>),
}

/// A field pattern: `fieldName = pat`.
#[derive(Debug, Clone, PartialEq)]
pub struct ThFieldPatChirho {
    pub name_chirho: ThNameChirho,
    pub pat_chirho: ThPatChirho,
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A TH type.
#[derive(Debug, Clone, PartialEq)]
pub enum ThTypeChirho {
    /// Forall quantifier: `forall a b. ctx => ty`.
    ForallTChirho(Vec<ThTyVarBndrChirho>, ThCxtChirho, Box<ThTypeChirho>),
    /// Forall with visibility.
    ForallVisTChirho(Vec<ThTyVarBndrChirho>, Box<ThTypeChirho>),
    /// Type application: `Maybe Int`.
    AppTChirho(Box<ThTypeChirho>, Box<ThTypeChirho>),
    /// Kind application: `t @k`.
    AppKindTChirho(Box<ThTypeChirho>, Box<ThTypeChirho>),
    /// Type signature: `ty :: kind`.
    SigTChirho(Box<ThTypeChirho>, Box<ThTypeChirho>),
    /// Type variable: `a`.
    VarTChirho(ThNameChirho),
    /// Type constructor: `Int`, `Maybe`.
    ConTChirho(ThNameChirho),
    /// Promoted data constructor: `'True`.
    PromotedTChirho(ThNameChirho),
    /// Infix type application: `a :+: b`.
    InfixTChirho(Box<ThTypeChirho>, ThNameChirho, Box<ThTypeChirho>),
    /// Equality constraint: `a ~ b`.
    EqualityTChirho,
    /// Arrow type constructor: `(->)`.
    ArrowTChirho,
    /// Multiplicity arrow for LinearTypes.
    MulArrowTChirho,
    /// List type constructor: `[]`.
    ListTChirho,
    /// Tuple type constructor: `(,)`, `(,,)`, etc.
    TupleTChirho(i32),
    /// Unboxed tuple type constructor.
    UnboxedTupleTChirho(i32),
    /// Unboxed sum type constructor.
    UnboxedSumTChirho(i32),
    /// `*` / `Type` kind.
    StarTChirho,
    /// `Constraint` kind.
    ConstraintTChirho,
    /// Promoted list: `'[Int, Bool]`.
    PromotedListTChirho(Vec<ThTypeChirho>),
    /// Promoted tuple: `'(Int, Bool)`.
    PromotedTupleTChirho(Vec<ThTypeChirho>),
    /// Promoted nil: `'[]`.
    PromotedNilTChirho,
    /// Promoted cons: `(':)`.
    PromotedConsTChirho,
    /// Wildcard: `_`.
    WildCardTChirho,
    /// Implicit parameter: `?x :: Int`.
    ImplicitParamTChirho(String, Box<ThTypeChirho>),
}

/// A type variable binder.
#[derive(Debug, Clone, PartialEq)]
pub enum ThTyVarBndrChirho {
    /// Plain type variable: `a`.
    PlainTVChirho(ThNameChirho),
    /// Kinded type variable: `(a :: k)`.
    KindedTVChirho(ThNameChirho, Box<ThTypeChirho>),
}

/// A type context (list of constraints).
pub type ThCxtChirho = Vec<ThTypeChirho>;

// ---------------------------------------------------------------------------
// Declarations
// ---------------------------------------------------------------------------

/// A TH declaration — mirrors `Language.Haskell.TH.Syntax.Dec`.
#[derive(Debug, Clone, PartialEq)]
pub enum ThDecChirho {
    /// Function definition: `f p1 p2 = body`.
    FunDChirho(ThNameChirho, Vec<ThClauseChirho>),
    /// Value binding: `pat = body`.
    ValDChirho(ThPatChirho, ThBodyChirho, Vec<ThDecChirho>),
    /// Type signature: `f :: Type`.
    SigDChirho(ThNameChirho, Box<ThTypeChirho>),
    /// Data declaration: `data T a = C1 | C2`.
    DataDChirho(
        ThCxtChirho,
        ThNameChirho,
        Vec<ThTyVarBndrChirho>,
        Option<Box<ThTypeChirho>>,
        Vec<ThConChirho>,
        Vec<ThDerivClauseChirho>,
    ),
    /// Newtype: `newtype T a = C Type`.
    NewtypeDChirho(
        ThCxtChirho,
        ThNameChirho,
        Vec<ThTyVarBndrChirho>,
        Option<Box<ThTypeChirho>>,
        ThConChirho,
        Vec<ThDerivClauseChirho>,
    ),
    /// Type alias: `type Name = Type`.
    TySynDChirho(ThNameChirho, Vec<ThTyVarBndrChirho>, Box<ThTypeChirho>),
    /// Class declaration.
    ClassDChirho(
        ThCxtChirho,
        ThNameChirho,
        Vec<ThTyVarBndrChirho>,
        Vec<ThFunDepChirho>,
        Vec<ThDecChirho>,
    ),
    /// Instance declaration.
    InstanceDChirho(
        Option<ThOverlapChirho>,
        ThCxtChirho,
        Box<ThTypeChirho>,
        Vec<ThDecChirho>,
    ),
    /// Standalone deriving: `deriving instance C T`.
    StandaloneDerivDChirho(
        Option<ThDerivStrategyChirho>,
        ThCxtChirho,
        Box<ThTypeChirho>,
    ),
    /// Foreign import.
    ForeignDChirho(ThForeignChirho),
    /// Inline pragma.
    InlineDChirho(
        ThNameChirho,
        ThInlineChirho,
        ThRuleBangChirho,
        ThPhasesChirho,
    ),
    /// Specialise pragma.
    SpecialiseDChirho(ThNameChirho, Box<ThTypeChirho>, ThPhasesChirho),
    /// Specialise instance pragma.
    SpecialiseInstDChirho(Box<ThTypeChirho>),
    /// Rule pragma: `{-# RULES "name" forall x. f x = g x #-}`.
    RuleDChirho(
        String,
        Vec<ThRuleBndrChirho>,
        Box<ThExpChirho>,
        Box<ThExpChirho>,
        ThPhasesChirho,
    ),
    /// ANN pragma.
    AnnDChirho(ThAnnTargetChirho, Box<ThExpChirho>),
    /// Fixity declaration.
    InfixDChirho(ThFixityChirho, ThNameChirho),
    /// Default declaration.
    DefaultDChirho(Vec<ThTypeChirho>),
    /// Open type family.
    OpenTypeFamilyDChirho(ThTypeFamilyHeadChirho),
    /// Closed type family.
    ClosedTypeFamilyDChirho(ThTypeFamilyHeadChirho, Vec<ThTySynEqnChirho>),
    /// Data family declaration.
    DataFamilyDChirho(
        ThNameChirho,
        Vec<ThTyVarBndrChirho>,
        Option<Box<ThTypeChirho>>,
    ),
    /// Data family instance.
    DataInstDChirho(
        ThCxtChirho,
        Box<ThTypeChirho>,
        Option<Box<ThTypeChirho>>,
        Vec<ThConChirho>,
        Vec<ThDerivClauseChirho>,
    ),
    /// Newtype family instance.
    NewtypeInstDChirho(
        ThCxtChirho,
        Box<ThTypeChirho>,
        Option<Box<ThTypeChirho>>,
        ThConChirho,
        Vec<ThDerivClauseChirho>,
    ),
    /// Type family instance: `type instance F Int = Bool`.
    TySynInstDChirho(ThTySynEqnChirho),
    /// Role annotation.
    RoleAnnotDChirho(ThNameChirho, Vec<ThRoleChirho>),
    /// Pattern synonym.
    PatSynDChirho(
        ThNameChirho,
        ThPatSynArgsChirho,
        ThPatSynDirChirho,
        ThPatChirho,
    ),
    /// Pattern synonym type signature.
    PatSynSigDChirho(
        ThNameChirho,
        Vec<ThTyVarBndrChirho>,
        ThCxtChirho,
        Vec<ThTyVarBndrChirho>,
        ThCxtChirho,
        Box<ThTypeChirho>,
    ),
    /// COMPLETE pragma.
    CompleteDChirho(Vec<ThNameChirho>, Option<ThNameChirho>),
    /// Implicit parameter binding.
    ImplicitParamBindDChirho(String, Box<ThExpChirho>),
}

/// A function clause: `p1 p2 -> body where decs`.
#[derive(Debug, Clone, PartialEq)]
pub struct ThClauseChirho {
    pub pats_chirho: Vec<ThPatChirho>,
    pub body_chirho: ThBodyChirho,
    pub decs_chirho: Vec<ThDecChirho>,
}

/// A data constructor.
#[derive(Debug, Clone, PartialEq)]
pub enum ThConChirho {
    /// Ordinary: `C Int Bool`.
    NormalCChirho(ThNameChirho, Vec<ThBangTypeChirho>),
    /// Record: `C { f1 :: Int, f2 :: Bool }`.
    RecCChirho(ThNameChirho, Vec<ThVarBangTypeChirho>),
    /// Infix: `Int :+: Bool`.
    InfixCChirho(ThBangTypeChirho, ThNameChirho, ThBangTypeChirho),
    /// GADT: `C :: Int -> Bool -> T a`.
    GadtCChirho(Vec<ThNameChirho>, Vec<ThBangTypeChirho>, Box<ThTypeChirho>),
    /// Record GADT: `C :: { f :: Int } -> T a`.
    RecGadtCChirho(
        Vec<ThNameChirho>,
        Vec<ThVarBangTypeChirho>,
        Box<ThTypeChirho>,
    ),
    /// Forall-qualified constructor.
    ForallCChirho(Vec<ThTyVarBndrChirho>, ThCxtChirho, Box<ThConChirho>),
}

/// A banged type: `(Bang, Type)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ThBangTypeChirho {
    pub bang_chirho: ThBangChirho,
    pub ty_chirho: ThTypeChirho,
}

/// A named banged type: `(Name, Bang, Type)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ThVarBangTypeChirho {
    pub name_chirho: ThNameChirho,
    pub bang_chirho: ThBangChirho,
    pub ty_chirho: ThTypeChirho,
}

/// Strictness annotation.
#[derive(Debug, Clone, PartialEq)]
pub struct ThBangChirho {
    pub src_unpackedness_chirho: ThSourceUnpackednessChirho,
    pub src_strictness_chirho: ThSourceStrictnessChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThSourceUnpackednessChirho {
    NoSourceUnpackednessChirho,
    SourceNoUnpackChirho,
    SourceUnpackChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThSourceStrictnessChirho {
    NoSourceStrictnessChirho,
    SourceLazyChirho,
    SourceStrictChirho,
}

impl ThBangChirho {
    /// Default bang: no unpackedness, no strictness.
    pub fn default_bang_chirho() -> Self {
        Self {
            src_unpackedness_chirho: ThSourceUnpackednessChirho::NoSourceUnpackednessChirho,
            src_strictness_chirho: ThSourceStrictnessChirho::NoSourceStrictnessChirho,
        }
    }
}

// ---------------------------------------------------------------------------
// Deriving, overlap, fixity, and other auxiliary types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct ThDerivClauseChirho {
    pub strategy_chirho: Option<ThDerivStrategyChirho>,
    pub classes_chirho: Vec<ThTypeChirho>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThDerivStrategyChirho {
    StockStrategyChirho,
    AnyclassStrategyChirho,
    NewtypeStrategyChirho,
    ViaStrategyChirho(Box<ThTypeChirho>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThOverlapChirho {
    OverlapChirho,
    OverlappableChirho,
    OverlappingChirho,
    OverlapsChirho,
    IncoherentChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThFunDepChirho {
    pub from_chirho: Vec<ThNameChirho>,
    pub to_chirho: Vec<ThNameChirho>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThFixityChirho {
    pub prec_chirho: i32,
    pub dir_chirho: ThFixityDirectionChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThFixityDirectionChirho {
    InfixLChirho,
    InfixRChirho,
    InfixNChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThForeignChirho {
    ImportFChirho(
        ThCallconvChirho,
        ThSafetyChirho,
        String,
        ThNameChirho,
        Box<ThTypeChirho>,
    ),
    ExportFChirho(ThCallconvChirho, String, ThNameChirho, Box<ThTypeChirho>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThCallconvChirho {
    CCallChirho,
    StdCallChirho,
    CApiChirho,
    PrimChirho,
    JavaScriptChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThSafetyChirho {
    UnsafeChirho,
    SafeChirho,
    InterruptibleChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThInlineChirho {
    InlineChirho,
    NoInlineChirho,
    InlinableChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThRuleBangChirho {
    ConLikeChirho,
    FunLikeChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThPhasesChirho {
    AllPhasesChirho,
    FromPhaseChirho(i32),
    BeforePhaseChirho(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThRuleBndrChirho {
    RuleBChirho(ThNameChirho),
    TypedRuleBChirho(ThNameChirho, Box<ThTypeChirho>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThAnnTargetChirho {
    ModuleAnnotationChirho,
    TypeAnnotationChirho(ThNameChirho),
    ValueAnnotationChirho(ThNameChirho),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThTypeFamilyHeadChirho {
    pub name_chirho: ThNameChirho,
    pub ty_vars_chirho: Vec<ThTyVarBndrChirho>,
    pub result_sig_chirho: Option<ThFamilyResultSigChirho>,
    pub injectivity_ann_chirho: Option<ThInjectivityAnnChirho>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThFamilyResultSigChirho {
    NoSigChirho,
    KindSigChirho(Box<ThTypeChirho>),
    TyVarSigChirho(ThTyVarBndrChirho),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThInjectivityAnnChirho {
    pub lhs_chirho: ThNameChirho,
    pub rhs_chirho: Vec<ThNameChirho>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThTySynEqnChirho {
    pub ty_vars_chirho: Option<Vec<ThTyVarBndrChirho>>,
    pub lhs_chirho: Box<ThTypeChirho>,
    pub rhs_chirho: Box<ThTypeChirho>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThRoleChirho {
    NominalRChirho,
    RepresentationalRChirho,
    PhantomRChirho,
    InferRChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThPatSynArgsChirho {
    PrefixPatSynChirho(Vec<ThNameChirho>),
    InfixPatSynChirho(ThNameChirho, ThNameChirho),
    RecordPatSynChirho(Vec<ThNameChirho>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThPatSynDirChirho {
    UnidirPatSynChirho,
    ImplBidirPatSynChirho,
    ExplBidirPatSynChirho(Vec<ThClauseChirho>),
}

// ---------------------------------------------------------------------------
// Reification info
// ---------------------------------------------------------------------------

/// Result of `reify` — information about a name known at compile time.
#[derive(Debug, Clone, PartialEq)]
pub enum ThInfoChirho {
    /// A class: its declaration and all known instances.
    ClassIChirho(ThDecChirho, Vec<ThDecChirho>),
    /// A class method: its name, type, and parent class.
    ClassOpIChirho(ThNameChirho, Box<ThTypeChirho>, ThNameChirho),
    /// A type constructor: its declaration.
    TyConIChirho(ThDecChirho),
    /// A type family instance.
    FamilyIChirho(ThDecChirho, Vec<ThTySynEqnChirho>),
    /// A primitive type constructor.
    PrimTyConIChirho(ThNameChirho, i32, bool),
    /// A data constructor: name, type, parent type name.
    DataConIChirho(ThNameChirho, Box<ThTypeChirho>, ThNameChirho),
    /// A pattern synonym.
    PatSynIChirho(ThDecChirho),
    /// A value: name, type, optional declaration.
    VarIChirho(ThNameChirho, Box<ThTypeChirho>, Option<ThDecChirho>),
    /// A type variable in scope.
    TyVarIChirho(ThNameChirho, Box<ThTypeChirho>),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn th_name_display_chirho() {
        let name_chirho = ThNameChirho::mk_name_chirho("foo");
        assert_eq!(format!("{}", name_chirho), "foo");

        let global_chirho =
            ThNameChirho::global_chirho("insert", "Data.Map", ThNameSpaceChirho::VarNameChirho);
        assert_eq!(format!("{}", global_chirho), "Data.Map.insert");

        let unique_chirho = ThNameChirho::unique_chirho("x", 42);
        assert_eq!(format!("{}", unique_chirho), "x_42");
    }

    #[test]
    fn th_make_lenses_output_chirho() {
        // Simulate what makeLenses would produce for:
        // data Person = Person { _name :: String, _age :: Int }
        // => name :: Lens' Person String
        //    name f (Person n a) = fmap (\n' -> Person n' a) (f n)
        let person_name_chirho = ThNameChirho::mk_name_chirho("Person");
        let lens_sig_chirho = ThDecChirho::SigDChirho(
            ThNameChirho::mk_name_chirho("name"),
            Box::new(ThTypeChirho::AppTChirho(
                Box::new(ThTypeChirho::AppTChirho(
                    Box::new(ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                        "Lens'",
                    ))),
                    Box::new(ThTypeChirho::ConTChirho(person_name_chirho.clone())),
                )),
                Box::new(ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                    "String",
                ))),
            )),
        );

        // The actual function body would be more complex, but this shows
        // that the TH types can represent lens output.
        let lens_fun_chirho = ThDecChirho::FunDChirho(
            ThNameChirho::mk_name_chirho("name"),
            vec![ThClauseChirho {
                pats_chirho: vec![ThPatChirho::VarPChirho(ThNameChirho::mk_name_chirho("f"))],
                body_chirho: ThBodyChirho::NormalBChirho(ThExpChirho::LamEChirho(
                    vec![ThPatChirho::ConPChirho(
                        person_name_chirho,
                        vec![],
                        vec![
                            ThPatChirho::VarPChirho(ThNameChirho::mk_name_chirho("n")),
                            ThPatChirho::VarPChirho(ThNameChirho::mk_name_chirho("a")),
                        ],
                    )],
                    Box::new(ThExpChirho::VarEChirho(ThNameChirho::mk_name_chirho(
                        "placeholder",
                    ))),
                )),
                decs_chirho: vec![],
            }],
        );

        // Just verify the types constructed without panic
        assert!(matches!(lens_sig_chirho, ThDecChirho::SigDChirho(..)));
        assert!(matches!(lens_fun_chirho, ThDecChirho::FunDChirho(..)));
    }

    #[test]
    fn th_simple_splice_output_chirho() {
        // Simulate what a simple TH splice produces:
        // $([d| x = 42 |]) should produce [ValD (VarP "x") (NormalB (LitE (IntegerL 42))) []]
        let result_chirho = vec![ThDecChirho::ValDChirho(
            ThPatChirho::VarPChirho(ThNameChirho::mk_name_chirho("x")),
            ThBodyChirho::NormalBChirho(ThExpChirho::LitEChirho(ThLitChirho::IntegerLChirho(42))),
            vec![],
        )];

        assert_eq!(result_chirho.len(), 1);
        match &result_chirho[0] {
            ThDecChirho::ValDChirho(pat_chirho, body_chirho, decs_chirho) => {
                assert!(matches!(pat_chirho, ThPatChirho::VarPChirho(n) if n.occ_chirho == "x"));
                assert!(matches!(
                    body_chirho,
                    ThBodyChirho::NormalBChirho(ThExpChirho::LitEChirho(
                        ThLitChirho::IntegerLChirho(42)
                    ))
                ));
                assert!(decs_chirho.is_empty());
            }
            _ => panic!("expected ValD"),
        }
    }

    #[test]
    fn th_bang_default_chirho() {
        let bang_chirho = ThBangChirho::default_bang_chirho();
        assert_eq!(
            bang_chirho.src_unpackedness_chirho,
            ThSourceUnpackednessChirho::NoSourceUnpackednessChirho
        );
        assert_eq!(
            bang_chirho.src_strictness_chirho,
            ThSourceStrictnessChirho::NoSourceStrictnessChirho
        );
    }
}
