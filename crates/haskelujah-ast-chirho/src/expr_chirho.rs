// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Expression syntax

use haskelujah_span_chirho::SpanChirho;

use crate::lit_chirho::LitChirho;
use crate::name_chirho::NameChirho;
use crate::pat_chirho::PatChirho;
use crate::ty_chirho::TypeChirho;

/// A Haskell expression.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprChirho {
    /// Variable or function reference (`x`, `putStrLn`).
    VarChirho(NameChirho),
    /// Constructor reference (`Just`, `Left`).
    ConChirho(NameChirho),
    /// Literal value (`42`, `"hello"`).
    LitChirho(LitChirho),
    /// Function application (`f x`).
    AppChirho {
        /// Function expression being applied.
        fun_chirho: Box<ExprChirho>,
        /// Argument expression supplied to the function.
        arg_chirho: Box<ExprChirho>,
        /// Span covering the whole application.
        span_chirho: SpanChirho,
    },
    /// Type application (`f @Int`, `read @Bool "True"`).
    TypeAppChirho {
        /// Expression receiving an explicit type argument.
        expr_chirho: Box<ExprChirho>,
        /// Type argument supplied with visible type application.
        ty_chirho: TypeChirho,
        /// Span covering the whole type application.
        span_chirho: SpanChirho,
    },
    /// Infix application (`a + b`).
    InfixChirho {
        /// Left operand expression.
        left_chirho: Box<ExprChirho>,
        /// Operator applied between the operands.
        op_chirho: NameChirho,
        /// Right operand expression.
        right_chirho: Box<ExprChirho>,
        /// Span covering the whole infix application.
        span_chirho: SpanChirho,
    },
    /// Negation (`-x`).
    NegChirho {
        /// Expression being negated.
        expr_chirho: Box<ExprChirho>,
        /// Span covering the whole negation.
        span_chirho: SpanChirho,
    },
    /// Lambda expression (`\x y -> body`).
    LamChirho {
        /// Parameters introduced by the lambda.
        pats_chirho: Vec<PatChirho>,
        /// Lambda body expression.
        body_chirho: Box<ExprChirho>,
        /// Span covering the whole lambda.
        span_chirho: SpanChirho,
    },
    /// Let expression (`let binds in body`).
    LetChirho {
        /// Local bindings visible in the body.
        binds_chirho: Vec<LocalBindChirho>,
        /// Body evaluated with the local bindings in scope.
        body_chirho: Box<ExprChirho>,
        /// Span covering the whole let expression.
        span_chirho: SpanChirho,
    },
    /// If expression (`if c then t else e`).
    IfChirho {
        /// Condition deciding which branch to evaluate.
        cond_chirho: Box<ExprChirho>,
        /// Branch evaluated when the condition is true.
        then_chirho: Box<ExprChirho>,
        /// Branch evaluated when the condition is false.
        else_chirho: Box<ExprChirho>,
        /// Span covering the whole conditional expression.
        span_chirho: SpanChirho,
    },
    /// Case expression (`case e of { alts }`).
    CaseChirho {
        /// Expression being scrutinized.
        scrutinee_chirho: Box<ExprChirho>,
        /// Alternatives matched against the scrutinee.
        alts_chirho: Vec<AltChirho>,
        /// Span covering the whole case expression.
        span_chirho: SpanChirho,
    },
    /// Do expression (`do { stmts }` or `Module.do { stmts }`).
    DoChirho {
        /// QualifiedDo module qualifier selecting the sequencing methods.
        qualifier_chirho: Option<String>,
        /// Statements executed in sequence.
        stmts_chirho: Vec<StmtChirho>,
        /// Span covering the whole do block.
        span_chirho: SpanChirho,
    },
    /// Tuple expression (`(a, b, c)`).
    TupleChirho {
        /// Tuple elements in source order.
        elements_chirho: Vec<ExprChirho>,
        /// Span covering the whole tuple expression.
        span_chirho: SpanChirho,
    },
    /// List expression (`[a, b, c]`).
    ListChirho {
        /// List elements in source order.
        elements_chirho: Vec<ExprChirho>,
        /// Span covering the whole list expression.
        span_chirho: SpanChirho,
    },
    /// Arithmetic sequence (`[1..10]`, `[1,3..10]`).
    ArithSeqChirho {
        /// First element in the arithmetic progression.
        from_chirho: Box<ExprChirho>,
        /// Optional second element establishing the step size.
        then_chirho: Option<Box<ExprChirho>>,
        /// Optional inclusive upper bound.
        to_chirho: Option<Box<ExprChirho>>,
        /// Span covering the whole arithmetic sequence.
        span_chirho: SpanChirho,
    },
    /// List comprehension (`[e | quals]`).
    ListCompChirho {
        /// Result expression produced for each successful qualifier path.
        body_chirho: Box<ExprChirho>,
        /// Sequential qualifiers driving generator, guard, and let semantics.
        quals_chirho: Vec<StmtChirho>,
        /// Parallel qualifier branches evaluated independently and zipped
        /// together before the result expression is evaluated.
        parallel_quals_chirho: Vec<Vec<StmtChirho>>,
        /// Span covering the whole list comprehension.
        span_chirho: SpanChirho,
    },
    /// Left section (`(+ 1)`).
    LeftSectionChirho {
        /// Operator waiting for a left operand.
        op_chirho: NameChirho,
        /// Right operand fixed by the section.
        arg_chirho: Box<ExprChirho>,
        /// Span covering the whole left section.
        span_chirho: SpanChirho,
    },
    /// Right section (`(1 +)`).
    RightSectionChirho {
        /// Left operand fixed by the section.
        arg_chirho: Box<ExprChirho>,
        /// Operator waiting for a right operand.
        op_chirho: NameChirho,
        /// Span covering the whole right section.
        span_chirho: SpanChirho,
    },
    /// Type annotation (`expr :: Type`).
    AnnChirho {
        /// Expression receiving the type annotation.
        expr_chirho: Box<ExprChirho>,
        /// Annotated type attached to the expression.
        ty_chirho: TypeChirho,
        /// Span covering the whole annotation.
        span_chirho: SpanChirho,
    },
    /// Parenthesized expression (`(expr)`).
    ParenChirho {
        /// Inner expression wrapped in parentheses.
        inner_chirho: Box<ExprChirho>,
        /// Span covering the whole parenthesized expression.
        span_chirho: SpanChirho,
    },
    /// Record construction (`Con { f1 = e1, f2 = e2 }` or `Con { f1, .. }`).
    RecordConChirho {
        /// Constructor being built.
        con_chirho: NameChirho,
        /// Explicit field assignments provided in the record literal.
        fields_chirho: Vec<FieldAssignChirho>,
        /// Whether `..` was present (RecordWildCards).
        has_wildcard_chirho: bool,
        /// Span covering the whole record construction.
        span_chirho: SpanChirho,
    },
    /// Record update (`expr { f1 = e1 }`).
    RecordUpdateChirho {
        /// Original record expression being copied and updated.
        expr_chirho: Box<ExprChirho>,
        /// Replacement field assignments applied to the record.
        fields_chirho: Vec<FieldAssignChirho>,
        /// Span covering the whole record update.
        span_chirho: SpanChirho,
    },
    /// Template Haskell splice expression (`$(expr)` or `$name`).
    SpliceChirho {
        /// Expression spliced into the surrounding AST.
        expr_chirho: Box<ExprChirho>,
        /// Span covering the whole splice expression.
        span_chirho: SpanChirho,
    },
    /// Template Haskell typed splice expression (`$$(expr)` or `$$name`).
    TypedSpliceChirho {
        /// Typed splice expression evaluated at compile time.
        expr_chirho: Box<ExprChirho>,
        /// Span covering the whole typed splice.
        span_chirho: SpanChirho,
    },
    /// Template Haskell expression quotation (`[| expr |]` or `[e| expr |]`).
    QuoteExprChirho {
        /// Quoted expression AST payload.
        expr_chirho: Box<ExprChirho>,
        /// Span covering the whole expression quote.
        span_chirho: SpanChirho,
    },
    /// Template Haskell declaration quotation (`[d| decls |]`).
    QuoteDeclChirho {
        /// Declarations captured inside the quotation.
        decls_chirho: Vec<crate::decl_chirho::DeclChirho>,
        /// Span covering the whole declaration quote.
        span_chirho: SpanChirho,
    },
    /// Template Haskell type quotation (`[t| type |]`).
    QuoteTypeChirho {
        /// Quoted type AST payload.
        ty_chirho: TypeChirho,
        /// Span covering the whole type quote.
        span_chirho: SpanChirho,
    },
    /// Template Haskell pattern quotation (`[p| pat |]`).
    QuotePatChirho {
        /// Quoted pattern AST payload.
        pat_chirho: PatChirho,
        /// Span covering the whole pattern quote.
        span_chirho: SpanChirho,
    },
}

/// A field assignment in a record expression.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldAssignChirho {
    /// The field being assigned.
    pub name_chirho: NameChirho,
    /// The expression stored in the field.
    pub value_chirho: ExprChirho,
    /// Span covering the whole field assignment.
    pub span_chirho: SpanChirho,
}

/// A case alternative (`pattern -> expr` or `pattern | guard -> expr`).
#[derive(Debug, Clone, PartialEq)]
pub struct AltChirho {
    /// The pattern matched by this alternative.
    pub pat_chirho: PatChirho,
    /// The right-hand side for the alternative.
    pub rhs_chirho: RhsChirho,
    /// Local bindings introduced by an alternative-level `where`.
    pub where_binds_chirho: Vec<LocalBindChirho>,
    /// Span covering the whole alternative.
    pub span_chirho: SpanChirho,
}

/// A right-hand side — either unguarded or guarded.
#[derive(Debug, Clone, PartialEq)]
pub enum RhsChirho {
    /// An unguarded right-hand side (`= expr`).
    UnguardedChirho(ExprChirho),
    /// One or more guarded right-hand sides (`| guard = expr`).
    GuardedChirho(Vec<GuardedExprChirho>),
}

/// A guarded expression (`| guard = expr`).
#[derive(Debug, Clone, PartialEq)]
pub struct GuardedExprChirho {
    /// The boolean guard expression.
    pub guard_chirho: ExprChirho,
    /// The body to evaluate when the guard succeeds.
    pub body_chirho: ExprChirho,
    /// Span covering the whole guarded RHS arm.
    pub span_chirho: SpanChirho,
}

/// A statement in a do block or list comprehension.
#[derive(Debug, Clone, PartialEq)]
pub enum StmtChirho {
    /// Expression statement (`expr`).
    ExprChirho(ExprChirho),
    /// Bind statement (`pat <- expr`).
    BindChirho {
        /// Pattern bound by the statement.
        pat_chirho: PatChirho,
        /// Expression producing the bound value.
        expr_chirho: ExprChirho,
        /// Span covering the whole bind statement.
        span_chirho: SpanChirho,
    },
    /// Let statement in do (`let binds`).
    LetChirho {
        /// Local bindings introduced by the statement.
        binds_chirho: Vec<LocalBindChirho>,
        /// Span covering the whole let statement.
        span_chirho: SpanChirho,
    },
}

/// A local binding in a let or where clause.
#[derive(Debug, Clone, PartialEq)]
pub enum LocalBindChirho {
    /// A function/value binding.
    FunBindChirho {
        /// Bound local name.
        name_chirho: NameChirho,
        /// Clauses implementing the binding.
        matches_chirho: Vec<MatchArmChirho>,
        /// Span covering the whole local binding.
        span_chirho: SpanChirho,
    },
    /// A pattern binding (`(a, b) = expr`).
    PatBindChirho {
        /// Pattern introduced by the binding.
        pat_chirho: PatChirho,
        /// Right-hand side producing the bound value.
        rhs_chirho: RhsChirho,
        /// Span covering the whole local binding.
        span_chirho: SpanChirho,
    },
    /// A type signature.
    TypeSigChirho {
        /// Name whose local type is annotated.
        name_chirho: NameChirho,
        /// Declared local type.
        ty_chirho: TypeChirho,
        /// Span covering the whole local type signature.
        span_chirho: SpanChirho,
    },
}

/// One equation/clause of a function binding (`f p1 p2 = body where ...`).
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArmChirho {
    /// Argument patterns for this clause.
    pub pats_chirho: Vec<PatChirho>,
    /// The clause right-hand side.
    pub rhs_chirho: RhsChirho,
    /// Local `where` bindings attached to this clause.
    pub where_binds_chirho: Vec<LocalBindChirho>,
    /// Span covering the whole clause.
    pub span_chirho: SpanChirho,
}

impl ExprChirho {
    /// Return the source span covering this expression.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::VarChirho(n_chirho) | Self::ConChirho(n_chirho) => n_chirho.span_chirho(),
            Self::LitChirho(l_chirho) => l_chirho.span_chirho(),
            Self::AppChirho { span_chirho, .. }
            | Self::InfixChirho { span_chirho, .. }
            | Self::NegChirho { span_chirho, .. }
            | Self::LamChirho { span_chirho, .. }
            | Self::LetChirho { span_chirho, .. }
            | Self::IfChirho { span_chirho, .. }
            | Self::CaseChirho { span_chirho, .. }
            | Self::DoChirho { span_chirho, .. }
            | Self::TupleChirho { span_chirho, .. }
            | Self::ListChirho { span_chirho, .. }
            | Self::ArithSeqChirho { span_chirho, .. }
            | Self::ListCompChirho { span_chirho, .. }
            | Self::LeftSectionChirho { span_chirho, .. }
            | Self::RightSectionChirho { span_chirho, .. }
            | Self::TypeAppChirho { span_chirho, .. }
            | Self::AnnChirho { span_chirho, .. }
            | Self::ParenChirho { span_chirho, .. }
            | Self::RecordConChirho { span_chirho, .. }
            | Self::RecordUpdateChirho { span_chirho, .. }
            | Self::SpliceChirho { span_chirho, .. }
            | Self::TypedSpliceChirho { span_chirho, .. }
            | Self::QuoteExprChirho { span_chirho, .. }
            | Self::QuoteDeclChirho { span_chirho, .. }
            | Self::QuoteTypeChirho { span_chirho, .. }
            | Self::QuotePatChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
