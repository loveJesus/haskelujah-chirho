// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Expression syntax

use rhasky_span_chirho::SpanChirho;

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
        fun_chirho: Box<ExprChirho>,
        arg_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Type application (`f @Int`, `read @Bool "True"`).
    TypeAppChirho {
        expr_chirho: Box<ExprChirho>,
        ty_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
    /// Infix application (`a + b`).
    InfixChirho {
        left_chirho: Box<ExprChirho>,
        op_chirho: NameChirho,
        right_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Negation (`-x`).
    NegChirho {
        expr_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Lambda expression (`\x y -> body`).
    LamChirho {
        pats_chirho: Vec<PatChirho>,
        body_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Let expression (`let binds in body`).
    LetChirho {
        binds_chirho: Vec<LocalBindChirho>,
        body_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// If expression (`if c then t else e`).
    IfChirho {
        cond_chirho: Box<ExprChirho>,
        then_chirho: Box<ExprChirho>,
        else_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Case expression (`case e of { alts }`).
    CaseChirho {
        scrutinee_chirho: Box<ExprChirho>,
        alts_chirho: Vec<AltChirho>,
        span_chirho: SpanChirho,
    },
    /// Do expression (`do { stmts }`).
    DoChirho {
        stmts_chirho: Vec<StmtChirho>,
        span_chirho: SpanChirho,
    },
    /// Tuple expression (`(a, b, c)`).
    TupleChirho {
        elements_chirho: Vec<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// List expression (`[a, b, c]`).
    ListChirho {
        elements_chirho: Vec<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Arithmetic sequence (`[1..10]`, `[1,3..10]`).
    ArithSeqChirho {
        from_chirho: Box<ExprChirho>,
        then_chirho: Option<Box<ExprChirho>>,
        to_chirho: Option<Box<ExprChirho>>,
        span_chirho: SpanChirho,
    },
    /// List comprehension (`[e | quals]`).
    ListCompChirho {
        body_chirho: Box<ExprChirho>,
        quals_chirho: Vec<StmtChirho>,
        span_chirho: SpanChirho,
    },
    /// Left section (`(+ 1)`).
    LeftSectionChirho {
        op_chirho: NameChirho,
        arg_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Right section (`(1 +)`).
    RightSectionChirho {
        arg_chirho: Box<ExprChirho>,
        op_chirho: NameChirho,
        span_chirho: SpanChirho,
    },
    /// Type annotation (`expr :: Type`).
    AnnChirho {
        expr_chirho: Box<ExprChirho>,
        ty_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
    /// Parenthesized expression (`(expr)`).
    ParenChirho {
        inner_chirho: Box<ExprChirho>,
        span_chirho: SpanChirho,
    },
    /// Record construction (`Con { f1 = e1, f2 = e2 }`).
    RecordConChirho {
        con_chirho: NameChirho,
        fields_chirho: Vec<FieldAssignChirho>,
        span_chirho: SpanChirho,
    },
    /// Record update (`expr { f1 = e1 }`).
    RecordUpdateChirho {
        expr_chirho: Box<ExprChirho>,
        fields_chirho: Vec<FieldAssignChirho>,
        span_chirho: SpanChirho,
    },
}

/// A field assignment in a record expression.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldAssignChirho {
    pub name_chirho: NameChirho,
    pub value_chirho: ExprChirho,
    pub span_chirho: SpanChirho,
}

/// A case alternative (`pattern -> expr` or `pattern | guard -> expr`).
#[derive(Debug, Clone, PartialEq)]
pub struct AltChirho {
    pub pat_chirho: PatChirho,
    pub rhs_chirho: RhsChirho,
    pub where_binds_chirho: Vec<LocalBindChirho>,
    pub span_chirho: SpanChirho,
}

/// A right-hand side — either unguarded or guarded.
#[derive(Debug, Clone, PartialEq)]
pub enum RhsChirho {
    UnguardedChirho(ExprChirho),
    GuardedChirho(Vec<GuardedExprChirho>),
}

/// A guarded expression (`| guard = expr`).
#[derive(Debug, Clone, PartialEq)]
pub struct GuardedExprChirho {
    pub guard_chirho: ExprChirho,
    pub body_chirho: ExprChirho,
    pub span_chirho: SpanChirho,
}

/// A statement in a do block or list comprehension.
#[derive(Debug, Clone, PartialEq)]
pub enum StmtChirho {
    /// Expression statement (`expr`).
    ExprChirho(ExprChirho),
    /// Bind statement (`pat <- expr`).
    BindChirho {
        pat_chirho: PatChirho,
        expr_chirho: ExprChirho,
        span_chirho: SpanChirho,
    },
    /// Let statement in do (`let binds`).
    LetChirho {
        binds_chirho: Vec<LocalBindChirho>,
        span_chirho: SpanChirho,
    },
}

/// A local binding in a let or where clause.
#[derive(Debug, Clone, PartialEq)]
pub enum LocalBindChirho {
    /// A function/value binding.
    FunBindChirho {
        name_chirho: NameChirho,
        matches_chirho: Vec<MatchArmChirho>,
        span_chirho: SpanChirho,
    },
    /// A pattern binding (`(a, b) = expr`).
    PatBindChirho {
        pat_chirho: PatChirho,
        rhs_chirho: RhsChirho,
        span_chirho: SpanChirho,
    },
    /// A type signature.
    TypeSigChirho {
        name_chirho: NameChirho,
        ty_chirho: TypeChirho,
        span_chirho: SpanChirho,
    },
}

/// One equation/clause of a function binding (`f p1 p2 = body where ...`).
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArmChirho {
    pub pats_chirho: Vec<PatChirho>,
    pub rhs_chirho: RhsChirho,
    pub where_binds_chirho: Vec<LocalBindChirho>,
    pub span_chirho: SpanChirho,
}

impl ExprChirho {
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
            | Self::RecordUpdateChirho { span_chirho, .. } => *span_chirho,
        }
    }
}
