// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Concrete Syntax Tree (CST) Node Kinds
//!
//! Defines the kinds of nodes that appear in the lossless concrete syntax tree.
//! The CST preserves all source information (whitespace, comments, punctuation)
//! so it can round-trip back to identical source text.
//!
//! Architecture follows the red-green tree approach (Roslyn / rust-analyzer):
//! - Green nodes are immutable, interned, identity-free, and cheap to share.
//! - Red nodes wrap green nodes with parent pointers and absolute offsets.
//!
//! This module defines the node *kinds*; the actual tree structure will be
//! built in a separate module.

/// Kinds of nodes in the concrete syntax tree.
///
/// Every node in the CST is either a *token* (leaf) or a *composite* node
/// (internal). Token nodes use `TokenKindChirho` from the `token_chirho` module;
/// composite nodes use the variants below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyntaxKindChirho {
    // -- Top-level structures --
    /// The root node for an entire source file.
    SourceFileChirho,
    /// `module Name where` header.
    ModuleHeaderChirho,
    /// `module Name (export1, export2) where` — the export list.
    ExportListChirho,
    /// A single export spec item.
    ExportSpecChirho,

    // -- Import declarations --
    /// A single `import` declaration.
    ImportDeclChirho,
    /// The import spec list (`(foo, Bar(..))`).
    ImportSpecListChirho,
    /// A single import spec item.
    ImportSpecChirho,

    // -- Type-level declarations --
    /// `data T a = C1 | C2` — data type declaration.
    DataDeclChirho,
    /// A single data constructor (`C a b`).
    ConDeclChirho,
    /// A GADT constructor (`C :: Type -> ... -> T a`).
    GadtConDeclChirho,
    /// Record fields in a constructor (`{ field :: Type }`).
    RecordFieldsChirho,
    /// A single record field declaration.
    FieldDeclChirho,
    /// `type Name = Type` — type alias.
    TypeAliasDeclChirho,
    /// `type family F a :: *` or `type family F a where ...` — type family.
    TypeFamilyDeclChirho,
    /// `type instance F Int = Bool` — open type family instance.
    TypeFamilyInstanceDeclChirho,
    /// `newtype Name = Con Type`.
    NewtypeDeclChirho,
    /// `class Ctx => Name a where` — type class declaration.
    ClassDeclChirho,
    /// `instance Ctx => Name Type where` — instance declaration.
    InstanceDeclChirho,
    /// `deriving (Class1, Class2)` clause.
    DerivingClauseChirho,
    /// `deriving instance Show Foo` — standalone deriving declaration.
    StandaloneDerivingDeclChirho,
    /// `default (Type1, Type2)` declaration.
    DefaultDeclChirho,
    /// `foreign import ccall ...` declaration.
    ForeignDeclChirho,
    /// Pattern synonym declaration (`pattern Head x <- x:_`).
    PatSynDeclChirho,
    /// Fixity declaration (`infixl 6 +!`).
    FixityDeclChirho,

    // -- Value-level declarations --
    /// A type signature (`foo :: Type`).
    TypeSigDeclChirho,
    /// A function or value binding (`foo x = expr` or `foo = expr`).
    FunBindChirho,
    /// A pattern binding (`(a, b) = expr`).
    PatBindChirho,
    /// A single equation in a function binding (for multi-clause defs).
    MatchChirho,
    /// A where clause attached to a binding.
    WhereClauseChirho,
    /// Guard clause (`| cond = expr`).
    GuardChirho,
    /// A group of guards in a guarded right-hand side.
    GuardedRhsChirho,

    // -- Expressions --
    /// Function application (`f x`).
    AppExprChirho,
    /// Type application (`f @Int`, `read @Bool`).
    TypeAppExprChirho,
    /// Infix application (`a + b`).
    InfixExprChirho,
    /// Lambda expression (`\x -> expr`).
    LambdaExprChirho,
    /// Lambda-case expression (`\case { alts }`) — LambdaCase extension.
    LambdaCaseExprChirho,
    /// Let expression (`let binds in expr`).
    LetExprChirho,
    /// If expression (`if c then t else f`).
    IfExprChirho,
    /// Multi-way if expression (`if | g1 -> e1 | g2 -> e2`) — MultiWayIf extension.
    MultiWayIfExprChirho,
    /// Case expression (`case e of { alts }`).
    CaseExprChirho,
    /// A single case alternative.
    CaseAltChirho,
    /// Do expression (`do { stmts }`).
    DoExprChirho,
    /// A single statement in a do block.
    DoStmtChirho,
    /// Bind statement in do (`pat <- expr`).
    BindStmtChirho,
    /// Let statement in do (`let binds`).
    LetStmtChirho,
    /// Parenthesized expression (`(expr)`).
    ParenExprChirho,
    /// Left operator section (`(+ 1)`).
    LeftSectionExprChirho,
    /// Right operator section (`(1 +)`).
    RightSectionExprChirho,
    /// Tuple expression (`(a, b, c)`).
    TupleExprChirho,
    /// List expression (`[a, b, c]`).
    ListExprChirho,
    /// Arithmetic sequence (`[1..10]`, `[1,3..10]`).
    ArithSeqExprChirho,
    /// List comprehension (`[expr | quals]`).
    ListCompExprChirho,
    /// Section (`(+ 1)`, `(1 +)`).
    SectionExprChirho,
    /// Type annotation expression (`expr :: Type`).
    TypeAnnotExprChirho,
    /// Negation expression (`-expr`).
    NegateExprChirho,
    /// Record construction (`Con { field = val }`).
    RecordConExprChirho,
    /// Record update (`expr { field = val }`).
    RecordUpdateExprChirho,
    /// A field assignment in a record expression.
    FieldAssignChirho,
    /// A literal expression (int, float, char, string).
    LiteralExprChirho,
    /// A name expression (variable or constructor reference).
    NameExprChirho,

    // -- Patterns --
    /// Constructor pattern (`Con p1 p2`).
    ConPatChirho,
    /// Variable pattern (`x`).
    VarPatChirho,
    /// Literal pattern (`42`, `'a'`).
    LitPatChirho,
    /// Wildcard pattern (`_`).
    WildcardPatChirho,
    /// As pattern (`x@pat`).
    AsPatChirho,
    /// Parenthesized pattern (`(pat)`).
    ParenPatChirho,
    /// Tuple pattern (`(p1, p2)`).
    TuplePatChirho,
    /// List pattern (`[p1, p2]`).
    ListPatChirho,
    /// Negated pattern (`-42`).
    NegPatChirho,
    /// Lazy (irrefutable) pattern (`~pat`).
    LazyPatChirho,
    /// Bang pattern (`!pat`) — extension, but common.
    BangPatChirho,
    /// Record pattern (`Con { field = pat }`).
    RecordPatChirho,
    /// Infix constructor pattern (`p1 :+: p2`).
    InfixConPatChirho,
    /// View pattern (`expr -> pat`), requires ViewPatterns extension.
    ViewPatChirho,

    // -- Types --
    /// Function type (`a -> b`).
    FunTypeChirho,
    /// Type application (`Maybe Int`).
    AppTypeChirho,
    /// Parenthesized type (`(Type)`).
    ParenTypeChirho,
    /// Tuple type (`(a, b)`).
    TupleTypeChirho,
    /// List type (`[a]`).
    ListTypeChirho,
    /// Type variable.
    VarTypeChirho,
    /// Type constructor.
    ConTypeChirho,
    /// Qualified type with context (`Eq a => a -> a`).
    QualTypeChirho,
    /// Context (class constraints before `=>`).
    ContextChirho,
    /// Forall quantifier (`forall a. ...`) — extension.
    ForallTypeChirho,
    /// Kind annotation (`Type :: Kind`).
    KindAnnotTypeChirho,
    /// DataKinds promoted constructor type (`'True`, `'Just`).
    PromotedConTypeChirho,
    /// DataKinds promoted list type (`'[Int, Bool]`).
    PromotedListTypeChirho,
    /// Infix type operator (`a :+: b`, `a `Either` b`).
    InfixTypeChirho,

    // -- Template Haskell --
    /// A splice expression (`$(expr)` or `$name`).
    SpliceExprChirho,
    /// A typed splice expression (`$$(expr)` or `$$name`).
    TypedSpliceExprChirho,
    /// A top-level splice declaration (`$(expr)` at declaration level).
    SpliceDeclChirho,
    /// An expression quotation (`[| expr |]` or `[e| expr |]`).
    QuoteExprChirho,
    /// A declaration quotation (`[d| decls |]`).
    QuoteDeclChirho,
    /// A type quotation (`[t| type |]`).
    QuoteTypeChirho,
    /// A pattern quotation (`[p| pat |]`).
    QuotePatChirho,
    /// A typed expression quotation (`[|| expr ||]`).
    TypedQuoteExprChirho,

    // -- Layout / trivia wrappers --
    /// A layout block (the body between virtual `{` and `}`).
    LayoutBlockChirho,
    /// Error recovery node — wraps tokens that could not be parsed.
    ErrorNodeChirho,
}

impl SyntaxKindChirho {
    /// Whether this node kind represents a declaration.
    pub fn is_decl_chirho(self) -> bool {
        matches!(
            self,
            Self::DataDeclChirho
                | Self::TypeAliasDeclChirho
                | Self::NewtypeDeclChirho
                | Self::ClassDeclChirho
                | Self::InstanceDeclChirho
                | Self::DefaultDeclChirho
                | Self::ForeignDeclChirho
                | Self::PatSynDeclChirho
                | Self::FixityDeclChirho
                | Self::TypeSigDeclChirho
                | Self::FunBindChirho
                | Self::PatBindChirho
                | Self::ImportDeclChirho
                | Self::TypeFamilyDeclChirho
                | Self::TypeFamilyInstanceDeclChirho
                | Self::SpliceDeclChirho
        )
    }

    /// Whether this node kind represents an expression.
    pub fn is_expr_chirho(self) -> bool {
        matches!(
            self,
            Self::AppExprChirho
                | Self::TypeAppExprChirho
                | Self::InfixExprChirho
                | Self::LambdaExprChirho
                | Self::LambdaCaseExprChirho
                | Self::LetExprChirho
                | Self::IfExprChirho
                | Self::MultiWayIfExprChirho
                | Self::CaseExprChirho
                | Self::DoExprChirho
                | Self::ParenExprChirho
                | Self::LeftSectionExprChirho
                | Self::RightSectionExprChirho
                | Self::TupleExprChirho
                | Self::ListExprChirho
                | Self::ArithSeqExprChirho
                | Self::ListCompExprChirho
                | Self::SectionExprChirho
                | Self::TypeAnnotExprChirho
                | Self::NegateExprChirho
                | Self::RecordConExprChirho
                | Self::RecordUpdateExprChirho
                | Self::LiteralExprChirho
                | Self::NameExprChirho
                | Self::SpliceExprChirho
                | Self::TypedSpliceExprChirho
                | Self::QuoteExprChirho
                | Self::TypedQuoteExprChirho
        )
    }

    /// Whether this node kind represents a pattern.
    pub fn is_pat_chirho(self) -> bool {
        matches!(
            self,
            Self::ConPatChirho
                | Self::VarPatChirho
                | Self::LitPatChirho
                | Self::WildcardPatChirho
                | Self::AsPatChirho
                | Self::ParenPatChirho
                | Self::TuplePatChirho
                | Self::ListPatChirho
                | Self::NegPatChirho
                | Self::LazyPatChirho
                | Self::BangPatChirho
                | Self::RecordPatChirho
                | Self::InfixConPatChirho
                | Self::ViewPatChirho
        )
    }

    /// Whether this node kind represents a type.
    pub fn is_type_chirho(self) -> bool {
        matches!(
            self,
            Self::FunTypeChirho
                | Self::AppTypeChirho
                | Self::ParenTypeChirho
                | Self::TupleTypeChirho
                | Self::ListTypeChirho
                | Self::VarTypeChirho
                | Self::ConTypeChirho
                | Self::QualTypeChirho
                | Self::ContextChirho
                | Self::ForallTypeChirho
                | Self::KindAnnotTypeChirho
                | Self::PromotedConTypeChirho
                | Self::PromotedListTypeChirho
                | Self::InfixTypeChirho
        )
    }
}
