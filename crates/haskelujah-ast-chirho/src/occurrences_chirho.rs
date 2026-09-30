// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Where occurrences live in the AST, stated once.
//!
//! An occurrence is a use site whose evidence the checker and the desugarer must
//! agree on. Lowering stamps each one as it builds it. Code generated after
//! lowering (derived instances, GND, spliced declarations) and code a producer
//! DUPLICATES as new code are reminted here: every occurrence in that subtree
//! takes a fresh origin, replacing whatever it carried. Moving or keeping an
//! occurrence never comes here, because it keeps its identity.
//!
//! Every match below is exhaustive with no fallback arm, so a new expression,
//! pattern, statement or declaration form fails to compile until it is
//! classified: an occurrence cannot be overlooked silently.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use crate::decl_chirho::{ClassMethodChirho, DeclChirho, PatSynDirChirho};
use crate::expr_chirho::{
    AltChirho, ExprChirho, GuardedExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho,
    StmtChirho,
};
use crate::lit_chirho::LitChirho;
use crate::name_chirho::NameChirho;
use crate::pat_chirho::PatChirho;
use crate::provenance_chirho::{OriginIdChirho, OriginSupplyChirho};
use crate::stmt_operation_chirho::SelectedOperationChirho;

/// One occurrence, reached for reading or restamping.
pub enum OccurrenceMutChirho<'a> {
    /// A variable or constructor reference (a constructor can carry a class
    /// context), or the operator of an infix application or section.
    ReferenceChirho(&'a mut NameChirho),
    /// A literal, in an expression or in a pattern: an overloaded literal's
    /// conversion, and a literal pattern's comparison.
    LiteralChirho(&'a mut LitChirho),
}

impl OccurrenceMutChirho<'_> {
    /// The origin this occurrence carries, if any.
    pub fn origin_chirho(&self) -> Option<OriginIdChirho> {
        match self {
            Self::ReferenceChirho(name_chirho) => name_chirho.origin_chirho(),
            Self::LiteralChirho(lit_chirho) => lit_chirho.origin_chirho(),
        }
    }

    /// Give this occurrence `origin_chirho`, replacing any it carried.
    pub fn set_origin_chirho(&mut self, origin_chirho: OriginIdChirho) {
        match self {
            Self::ReferenceChirho(name_chirho) => {
                name_chirho.set_origin_chirho(Some(origin_chirho))
            }
            Self::LiteralChirho(lit_chirho) => lit_chirho.set_origin_chirho(Some(origin_chirho)),
        }
    }
}

/// Give every occurrence in these declarations a fresh origin. For a producer's
/// own fresh output only, never for declarations that already existed.
pub fn remint_decls_chirho(
    decls_chirho: &mut [DeclChirho],
    supply_chirho: &mut OriginSupplyChirho,
) {
    for decl_chirho in decls_chirho {
        visit_decl_chirho(decl_chirho, &mut |mut occurrence_chirho| {
            occurrence_chirho.set_origin_chirho(supply_chirho.fresh_chirho());
        });
    }
}

/// Give every occurrence in this expression a fresh origin: the expression is
/// being duplicated as new code.
pub fn remint_expr_chirho(expr_chirho: &mut ExprChirho, supply_chirho: &mut OriginSupplyChirho) {
    visit_expr_chirho(expr_chirho, &mut |mut occurrence_chirho| {
        occurrence_chirho.set_origin_chirho(supply_chirho.fresh_chirho());
    });
}

/// The visitor every traversal below threads through.
pub type OccurrenceVisitorChirho<'v> = dyn FnMut(OccurrenceMutChirho<'_>) + 'v;

/// A statement, reached for reading or rewriting. A statement is NOT an
/// occurrence.
pub type StatementVisitorChirho<'v> = dyn FnMut(&mut StmtChirho) + 'v;

/// What one walk carries. The two hooks are deliberately separate kinds
/// (gpt_chirho #25088): an OCCURRENCE is an evidence-bearing use and may be
/// restamped, while a STATEMENT is not an occurrence at all. Visiting statements
/// therefore cannot mint, remint, or change how many occurrences a walk sees -
/// a statement never reaches `set_origin_chirho`, because it is not an
/// `OccurrenceMutChirho` and cannot be made into one.
///
/// Every walk that existed before this hook passes no statement visitor, so
/// remint, clone and passthrough keep their exact previous behaviour.
pub struct WalkChirho<'v> {
    on_occurrence_chirho: &'v mut OccurrenceVisitorChirho<'v>,
    on_statement_chirho: Option<&'v mut StatementVisitorChirho<'v>>,
}

impl<'v> WalkChirho<'v> {
    /// A walk that sees occurrences only, as every caller did before.
    pub fn occurrences_chirho(on_occurrence_chirho: &'v mut OccurrenceVisitorChirho<'v>) -> Self {
        Self {
            on_occurrence_chirho,
            on_statement_chirho: None,
        }
    }

    /// A walk that also sees every statement, in source order.
    pub fn with_statements_chirho(
        on_occurrence_chirho: &'v mut OccurrenceVisitorChirho<'v>,
        on_statement_chirho: &'v mut StatementVisitorChirho<'v>,
    ) -> Self {
        Self {
            on_occurrence_chirho,
            on_statement_chirho: Some(on_statement_chirho),
        }
    }

    fn occurrence_chirho(&mut self, occurrence_chirho: OccurrenceMutChirho<'_>) {
        (self.on_occurrence_chirho)(occurrence_chirho);
    }

    fn statement_chirho(&mut self, stmt_chirho: &mut StmtChirho) {
        if let Some(on_statement_chirho) = self.on_statement_chirho.as_mut() {
            on_statement_chirho(stmt_chirho);
        }
    }
}

/// Visit every occurrence in one declaration, in source order. The signature
/// every caller had before the statement hook existed, and it still carries no
/// statement visitor.
pub fn visit_decl_chirho(decl_chirho: &mut DeclChirho, visit_chirho: &mut OccurrenceVisitorChirho) {
    visit_decl_walk_chirho(
        decl_chirho,
        &mut WalkChirho::occurrences_chirho(visit_chirho),
    );
}

/// Visit every occurrence AND every statement in one declaration, in source
/// order. The statement hook exists for passes that rewrite a statement without
/// touching occurrence identity, such as clearing a `fail` a pattern cannot
/// need.
pub fn visit_decl_and_statements_chirho(
    decl_chirho: &mut DeclChirho,
    visit_chirho: &mut OccurrenceVisitorChirho,
    statement_chirho: &mut StatementVisitorChirho,
) {
    visit_decl_walk_chirho(
        decl_chirho,
        &mut WalkChirho::with_statements_chirho(visit_chirho, statement_chirho),
    );
}

/// The one traversal both entry points share.
pub fn visit_decl_walk_chirho(decl_chirho: &mut DeclChirho, walk_chirho: &mut WalkChirho<'_>) {
    match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => {
            visit_match_arms_chirho(matches_chirho, walk_chirho);
        }
        DeclChirho::PatBindChirho {
            pat_chirho,
            rhs_chirho,
            ..
        } => {
            visit_pat_chirho(pat_chirho, walk_chirho);
            visit_rhs_chirho(rhs_chirho, walk_chirho);
        }
        DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
            for ClassMethodChirho { default_chirho, .. } in methods_chirho {
                if let Some(arms_chirho) = default_chirho {
                    visit_match_arms_chirho(arms_chirho, walk_chirho);
                }
            }
        }
        DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
            visit_local_binds_chirho(methods_chirho, walk_chirho);
        }
        DeclChirho::PatSynDeclChirho {
            dir_chirho,
            pat_chirho,
            ..
        } => {
            visit_pat_chirho(pat_chirho, walk_chirho);
            match dir_chirho {
                PatSynDirChirho::ExplBidirChirho {
                    builder_binds_chirho,
                } => visit_local_binds_chirho(builder_binds_chirho, walk_chirho),
                PatSynDirChirho::ImplBidirChirho | PatSynDirChirho::UnidirChirho => {}
            }
        }
        DeclChirho::SpliceDeclChirho { expr_chirho, .. } => {
            visit_expr_walk_chirho(expr_chirho, walk_chirho);
        }
        // Declarations that hold types, names and kinds but no expression.
        DeclChirho::TypeSigChirho { .. }
        | DeclChirho::DataDeclChirho { .. }
        | DeclChirho::NewtypeDeclChirho { .. }
        | DeclChirho::TypeAliasDeclChirho { .. }
        | DeclChirho::TypeFamilyDeclChirho { .. }
        | DeclChirho::TypeFamilyInstanceDeclChirho { .. }
        | DeclChirho::FixityDeclChirho { .. }
        | DeclChirho::DefaultDeclChirho { .. }
        | DeclChirho::ForeignDeclChirho { .. }
        | DeclChirho::StandaloneDerivingDeclChirho { .. } => {}
    }
}

fn visit_match_arms_chirho(arms_chirho: &mut [MatchArmChirho], walk_chirho: &mut WalkChirho<'_>) {
    for arm_chirho in arms_chirho {
        for pat_chirho in &mut arm_chirho.pats_chirho {
            visit_pat_chirho(pat_chirho, walk_chirho);
        }
        visit_rhs_chirho(&mut arm_chirho.rhs_chirho, walk_chirho);
        visit_local_binds_chirho(&mut arm_chirho.where_binds_chirho, walk_chirho);
    }
}

fn visit_local_binds_chirho(
    binds_chirho: &mut [LocalBindChirho],
    walk_chirho: &mut WalkChirho<'_>,
) {
    for bind_chirho in binds_chirho {
        match bind_chirho {
            LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
                visit_match_arms_chirho(matches_chirho, walk_chirho);
            }
            LocalBindChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                ..
            } => {
                visit_pat_chirho(pat_chirho, walk_chirho);
                visit_rhs_chirho(rhs_chirho, walk_chirho);
            }
            LocalBindChirho::TypeSigChirho { .. } => {}
        }
    }
}

fn visit_rhs_chirho(rhs_chirho: &mut RhsChirho, walk_chirho: &mut WalkChirho<'_>) {
    match rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => visit_expr_walk_chirho(expr_chirho, walk_chirho),
        RhsChirho::GuardedChirho(guarded_chirho) => {
            for GuardedExprChirho {
                guard_chirho,
                body_chirho,
                ..
            } in guarded_chirho
            {
                visit_expr_walk_chirho(guard_chirho, walk_chirho);
                visit_expr_walk_chirho(body_chirho, walk_chirho);
            }
        }
    }
}

/// The selected operation of a statement, if it selected one. A statement that
/// selects nothing has no occurrence here, so nothing is stamped for it.
fn visit_operation_chirho(
    operation_chirho: &mut Option<SelectedOperationChirho>,
    walk_chirho: &mut WalkChirho<'_>,
) {
    if let Some(operation_chirho) = operation_chirho {
        walk_chirho.occurrence_chirho(OccurrenceMutChirho::ReferenceChirho(
            &mut operation_chirho.name_chirho,
        ));
    }
}

fn visit_stmts_chirho(stmts_chirho: &mut [StmtChirho], walk_chirho: &mut WalkChirho<'_>) {
    for stmt_chirho in stmts_chirho {
        // Occurrences FIRST, then the statement. A pass that rewrites a
        // statement - clearing a `fail` its pattern cannot need - therefore
        // cannot change what this same walk sees, so the occurrences a walk
        // yields are exactly the ones it would have yielded with no statement
        // hook at all (gpt_chirho #25088).
        match stmt_chirho {
            StmtChirho::ExprChirho {
                expr_chirho,
                then_chirho,
            } => {
                visit_expr_walk_chirho(expr_chirho, walk_chirho);
                // The `>>` the statement selects is a use like any other: it is
                // reminted with the statement when a producer duplicates it.
                visit_operation_chirho(then_chirho, walk_chirho);
            }
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                bind_chirho,
                fail_chirho,
                ..
            } => {
                visit_pat_chirho(pat_chirho, walk_chirho);
                visit_expr_walk_chirho(expr_chirho, walk_chirho);
                visit_operation_chirho(bind_chirho, walk_chirho);
                visit_operation_chirho(fail_chirho, walk_chirho);
            }
            StmtChirho::LetChirho { binds_chirho, .. } => {
                visit_local_binds_chirho(binds_chirho, walk_chirho);
            }
        }
        walk_chirho.statement_chirho(stmt_chirho);
    }
}

/// Visit every occurrence in one expression, in source order.
pub fn visit_expr_chirho(expr_chirho: &mut ExprChirho, visit_chirho: &mut OccurrenceVisitorChirho) {
    visit_expr_walk_chirho(
        expr_chirho,
        &mut WalkChirho::occurrences_chirho(visit_chirho),
    );
}

/// The expression traversal both entry points share.
pub fn visit_expr_walk_chirho(expr_chirho: &mut ExprChirho, walk_chirho: &mut WalkChirho<'_>) {
    match expr_chirho {
        // A constructor is a reference too: the checker captures the class
        // context a constructor can carry, exactly as it does for a variable.
        ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => {
            walk_chirho.occurrence_chirho(OccurrenceMutChirho::ReferenceChirho(name_chirho));
        }
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            visit_expr_walk_chirho(left_chirho, walk_chirho);
            walk_chirho.occurrence_chirho(OccurrenceMutChirho::ReferenceChirho(op_chirho));
            visit_expr_walk_chirho(right_chirho, walk_chirho);
        }
        ExprChirho::LeftSectionChirho {
            op_chirho,
            arg_chirho,
            ..
        } => {
            walk_chirho.occurrence_chirho(OccurrenceMutChirho::ReferenceChirho(op_chirho));
            visit_expr_walk_chirho(arg_chirho, walk_chirho);
        }
        ExprChirho::RightSectionChirho {
            arg_chirho,
            op_chirho,
            ..
        } => {
            visit_expr_walk_chirho(arg_chirho, walk_chirho);
            walk_chirho.occurrence_chirho(OccurrenceMutChirho::ReferenceChirho(op_chirho));
        }
        ExprChirho::LitChirho(lit_chirho) => {
            walk_chirho.occurrence_chirho(OccurrenceMutChirho::LiteralChirho(lit_chirho));
        }
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            visit_expr_walk_chirho(fun_chirho, walk_chirho);
            visit_expr_walk_chirho(arg_chirho, walk_chirho);
        }
        ExprChirho::TypeAppChirho { expr_chirho, .. }
        | ExprChirho::NegChirho { expr_chirho, .. }
        | ExprChirho::AnnChirho { expr_chirho, .. }
        | ExprChirho::SpliceChirho { expr_chirho, .. }
        | ExprChirho::TypedSpliceChirho { expr_chirho, .. }
        | ExprChirho::QuoteExprChirho { expr_chirho, .. } => {
            visit_expr_walk_chirho(expr_chirho, walk_chirho);
        }
        ExprChirho::ParenChirho { inner_chirho, .. } => {
            visit_expr_walk_chirho(inner_chirho, walk_chirho)
        }
        ExprChirho::LamChirho {
            pats_chirho,
            body_chirho,
            ..
        } => {
            for pat_chirho in pats_chirho {
                visit_pat_chirho(pat_chirho, walk_chirho);
            }
            visit_expr_walk_chirho(body_chirho, walk_chirho);
        }
        ExprChirho::TypeLamChirho { body_chirho, .. } => {
            visit_expr_walk_chirho(body_chirho, walk_chirho)
        }
        ExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            visit_local_binds_chirho(binds_chirho, walk_chirho);
            visit_expr_walk_chirho(body_chirho, walk_chirho);
        }
        ExprChirho::IfChirho {
            cond_chirho,
            then_chirho,
            else_chirho,
            ..
        } => {
            visit_expr_walk_chirho(cond_chirho, walk_chirho);
            visit_expr_walk_chirho(then_chirho, walk_chirho);
            visit_expr_walk_chirho(else_chirho, walk_chirho);
        }
        ExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            visit_expr_walk_chirho(scrutinee_chirho, walk_chirho);
            for AltChirho {
                pat_chirho,
                rhs_chirho,
                where_binds_chirho,
                ..
            } in alts_chirho
            {
                visit_pat_chirho(pat_chirho, walk_chirho);
                visit_rhs_chirho(rhs_chirho, walk_chirho);
                visit_local_binds_chirho(where_binds_chirho, walk_chirho);
            }
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => visit_stmts_chirho(stmts_chirho, walk_chirho),
        ExprChirho::TupleChirho {
            elements_chirho, ..
        }
        | ExprChirho::ListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                visit_expr_walk_chirho(element_chirho, walk_chirho);
            }
        }
        ExprChirho::ArithSeqChirho {
            from_chirho,
            then_chirho,
            to_chirho,
            ..
        } => {
            visit_expr_walk_chirho(from_chirho, walk_chirho);
            if let Some(then_chirho) = then_chirho {
                visit_expr_walk_chirho(then_chirho, walk_chirho);
            }
            if let Some(to_chirho) = to_chirho {
                visit_expr_walk_chirho(to_chirho, walk_chirho);
            }
        }
        ExprChirho::ListCompChirho {
            body_chirho,
            quals_chirho,
            parallel_quals_chirho,
            ..
        } => {
            visit_expr_walk_chirho(body_chirho, walk_chirho);
            visit_stmts_chirho(quals_chirho, walk_chirho);
            for branch_chirho in parallel_quals_chirho {
                visit_stmts_chirho(branch_chirho, walk_chirho);
            }
        }
        ExprChirho::RecordConChirho {
            con_chirho,
            fields_chirho,
            ..
        } => {
            walk_chirho.occurrence_chirho(OccurrenceMutChirho::ReferenceChirho(con_chirho));
            for field_chirho in fields_chirho {
                visit_expr_walk_chirho(&mut field_chirho.value_chirho, walk_chirho);
            }
        }
        ExprChirho::RecordUpdateChirho {
            expr_chirho,
            fields_chirho,
            ..
        } => {
            visit_expr_walk_chirho(expr_chirho, walk_chirho);
            for field_chirho in fields_chirho {
                visit_expr_walk_chirho(&mut field_chirho.value_chirho, walk_chirho);
            }
        }
        ExprChirho::QuoteDeclChirho { decls_chirho, .. } => {
            for decl_chirho in decls_chirho {
                visit_decl_walk_chirho(decl_chirho, walk_chirho);
            }
        }
        ExprChirho::QuotePatChirho { pat_chirho, .. } => visit_pat_chirho(pat_chirho, walk_chirho),
        ExprChirho::QuoteTypeChirho { .. } => {}
    }
}

/// Patterns bind names; a binder is not an occurrence. A view pattern's
/// function is an expression, so its occurrences are visited.
fn visit_pat_chirho(pat_chirho: &mut PatChirho, walk_chirho: &mut WalkChirho<'_>) {
    match pat_chirho {
        PatChirho::ViewChirho {
            expr_chirho,
            pat_chirho,
            ..
        } => {
            visit_expr_walk_chirho(expr_chirho, walk_chirho);
            visit_pat_chirho(pat_chirho, walk_chirho);
        }
        PatChirho::ConChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                visit_pat_chirho(arg_chirho, walk_chirho);
            }
        }
        PatChirho::TupleChirho {
            elements_chirho, ..
        }
        | PatChirho::ListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                visit_pat_chirho(element_chirho, walk_chirho);
            }
        }
        PatChirho::AsChirho { pattern_chirho, .. } => visit_pat_chirho(pattern_chirho, walk_chirho),
        PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::BangChirho { inner_chirho, .. } => visit_pat_chirho(inner_chirho, walk_chirho),
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            visit_pat_chirho(left_chirho, walk_chirho);
            visit_pat_chirho(right_chirho, walk_chirho);
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                visit_pat_chirho(&mut field_chirho.pattern_chirho, walk_chirho);
            }
        }
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => visit_pat_chirho(pat_chirho, walk_chirho),
        // A literal pattern is a comparison, and a use of its literal.
        PatChirho::LitChirho(lit_chirho) | PatChirho::NegChirho { lit_chirho, .. } => {
            walk_chirho.occurrence_chirho(OccurrenceMutChirho::LiteralChirho(lit_chirho));
        }
        PatChirho::VarChirho(_) | PatChirho::WildcardChirho(_) => {}
    }
}
