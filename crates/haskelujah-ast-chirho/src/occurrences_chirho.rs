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

/// Visit every occurrence in one declaration, in source order.
pub fn visit_decl_chirho(decl_chirho: &mut DeclChirho, visit_chirho: &mut OccurrenceVisitorChirho) {
    match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => {
            visit_match_arms_chirho(matches_chirho, visit_chirho);
        }
        DeclChirho::PatBindChirho {
            pat_chirho,
            rhs_chirho,
            ..
        } => {
            visit_pat_chirho(pat_chirho, visit_chirho);
            visit_rhs_chirho(rhs_chirho, visit_chirho);
        }
        DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
            for ClassMethodChirho { default_chirho, .. } in methods_chirho {
                if let Some(arms_chirho) = default_chirho {
                    visit_match_arms_chirho(arms_chirho, visit_chirho);
                }
            }
        }
        DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
            visit_local_binds_chirho(methods_chirho, visit_chirho);
        }
        DeclChirho::PatSynDeclChirho {
            dir_chirho,
            pat_chirho,
            ..
        } => {
            visit_pat_chirho(pat_chirho, visit_chirho);
            match dir_chirho {
                PatSynDirChirho::ExplBidirChirho {
                    builder_binds_chirho,
                } => visit_local_binds_chirho(builder_binds_chirho, visit_chirho),
                PatSynDirChirho::ImplBidirChirho | PatSynDirChirho::UnidirChirho => {}
            }
        }
        DeclChirho::SpliceDeclChirho { expr_chirho, .. } => {
            visit_expr_chirho(expr_chirho, visit_chirho);
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

fn visit_match_arms_chirho(
    arms_chirho: &mut [MatchArmChirho],
    visit_chirho: &mut OccurrenceVisitorChirho,
) {
    for arm_chirho in arms_chirho {
        for pat_chirho in &mut arm_chirho.pats_chirho {
            visit_pat_chirho(pat_chirho, visit_chirho);
        }
        visit_rhs_chirho(&mut arm_chirho.rhs_chirho, visit_chirho);
        visit_local_binds_chirho(&mut arm_chirho.where_binds_chirho, visit_chirho);
    }
}

fn visit_local_binds_chirho(
    binds_chirho: &mut [LocalBindChirho],
    visit_chirho: &mut OccurrenceVisitorChirho,
) {
    for bind_chirho in binds_chirho {
        match bind_chirho {
            LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
                visit_match_arms_chirho(matches_chirho, visit_chirho);
            }
            LocalBindChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                ..
            } => {
                visit_pat_chirho(pat_chirho, visit_chirho);
                visit_rhs_chirho(rhs_chirho, visit_chirho);
            }
            LocalBindChirho::TypeSigChirho { .. } => {}
        }
    }
}

fn visit_rhs_chirho(rhs_chirho: &mut RhsChirho, visit_chirho: &mut OccurrenceVisitorChirho) {
    match rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => visit_expr_chirho(expr_chirho, visit_chirho),
        RhsChirho::GuardedChirho(guarded_chirho) => {
            for GuardedExprChirho {
                guard_chirho,
                body_chirho,
                ..
            } in guarded_chirho
            {
                visit_expr_chirho(guard_chirho, visit_chirho);
                visit_expr_chirho(body_chirho, visit_chirho);
            }
        }
    }
}

fn visit_stmts_chirho(stmts_chirho: &mut [StmtChirho], visit_chirho: &mut OccurrenceVisitorChirho) {
    for stmt_chirho in stmts_chirho {
        match stmt_chirho {
            StmtChirho::ExprChirho(expr_chirho) => visit_expr_chirho(expr_chirho, visit_chirho),
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                visit_pat_chirho(pat_chirho, visit_chirho);
                visit_expr_chirho(expr_chirho, visit_chirho);
            }
            StmtChirho::LetChirho { binds_chirho, .. } => {
                visit_local_binds_chirho(binds_chirho, visit_chirho);
            }
        }
    }
}

/// Visit every occurrence in one expression, in source order.
pub fn visit_expr_chirho(expr_chirho: &mut ExprChirho, visit_chirho: &mut OccurrenceVisitorChirho) {
    match expr_chirho {
        // A constructor is a reference too: the checker captures the class
        // context a constructor can carry, exactly as it does for a variable.
        ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => {
            visit_chirho(OccurrenceMutChirho::ReferenceChirho(name_chirho));
        }
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            visit_expr_chirho(left_chirho, visit_chirho);
            visit_chirho(OccurrenceMutChirho::ReferenceChirho(op_chirho));
            visit_expr_chirho(right_chirho, visit_chirho);
        }
        ExprChirho::LeftSectionChirho {
            op_chirho,
            arg_chirho,
            ..
        } => {
            visit_chirho(OccurrenceMutChirho::ReferenceChirho(op_chirho));
            visit_expr_chirho(arg_chirho, visit_chirho);
        }
        ExprChirho::RightSectionChirho {
            arg_chirho,
            op_chirho,
            ..
        } => {
            visit_expr_chirho(arg_chirho, visit_chirho);
            visit_chirho(OccurrenceMutChirho::ReferenceChirho(op_chirho));
        }
        ExprChirho::LitChirho(lit_chirho) => {
            visit_chirho(OccurrenceMutChirho::LiteralChirho(lit_chirho));
        }
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            visit_expr_chirho(fun_chirho, visit_chirho);
            visit_expr_chirho(arg_chirho, visit_chirho);
        }
        ExprChirho::TypeAppChirho { expr_chirho, .. }
        | ExprChirho::NegChirho { expr_chirho, .. }
        | ExprChirho::AnnChirho { expr_chirho, .. }
        | ExprChirho::SpliceChirho { expr_chirho, .. }
        | ExprChirho::TypedSpliceChirho { expr_chirho, .. }
        | ExprChirho::QuoteExprChirho { expr_chirho, .. } => {
            visit_expr_chirho(expr_chirho, visit_chirho);
        }
        ExprChirho::ParenChirho { inner_chirho, .. } => {
            visit_expr_chirho(inner_chirho, visit_chirho)
        }
        ExprChirho::LamChirho {
            pats_chirho,
            body_chirho,
            ..
        } => {
            for pat_chirho in pats_chirho {
                visit_pat_chirho(pat_chirho, visit_chirho);
            }
            visit_expr_chirho(body_chirho, visit_chirho);
        }
        ExprChirho::TypeLamChirho { body_chirho, .. } => {
            visit_expr_chirho(body_chirho, visit_chirho)
        }
        ExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            visit_local_binds_chirho(binds_chirho, visit_chirho);
            visit_expr_chirho(body_chirho, visit_chirho);
        }
        ExprChirho::IfChirho {
            cond_chirho,
            then_chirho,
            else_chirho,
            ..
        } => {
            visit_expr_chirho(cond_chirho, visit_chirho);
            visit_expr_chirho(then_chirho, visit_chirho);
            visit_expr_chirho(else_chirho, visit_chirho);
        }
        ExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            visit_expr_chirho(scrutinee_chirho, visit_chirho);
            for AltChirho {
                pat_chirho,
                rhs_chirho,
                where_binds_chirho,
                ..
            } in alts_chirho
            {
                visit_pat_chirho(pat_chirho, visit_chirho);
                visit_rhs_chirho(rhs_chirho, visit_chirho);
                visit_local_binds_chirho(where_binds_chirho, visit_chirho);
            }
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => visit_stmts_chirho(stmts_chirho, visit_chirho),
        ExprChirho::TupleChirho {
            elements_chirho, ..
        }
        | ExprChirho::ListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                visit_expr_chirho(element_chirho, visit_chirho);
            }
        }
        ExprChirho::ArithSeqChirho {
            from_chirho,
            then_chirho,
            to_chirho,
            ..
        } => {
            visit_expr_chirho(from_chirho, visit_chirho);
            if let Some(then_chirho) = then_chirho {
                visit_expr_chirho(then_chirho, visit_chirho);
            }
            if let Some(to_chirho) = to_chirho {
                visit_expr_chirho(to_chirho, visit_chirho);
            }
        }
        ExprChirho::ListCompChirho {
            body_chirho,
            quals_chirho,
            parallel_quals_chirho,
            ..
        } => {
            visit_expr_chirho(body_chirho, visit_chirho);
            visit_stmts_chirho(quals_chirho, visit_chirho);
            for branch_chirho in parallel_quals_chirho {
                visit_stmts_chirho(branch_chirho, visit_chirho);
            }
        }
        ExprChirho::RecordConChirho {
            con_chirho,
            fields_chirho,
            ..
        } => {
            visit_chirho(OccurrenceMutChirho::ReferenceChirho(con_chirho));
            for field_chirho in fields_chirho {
                visit_expr_chirho(&mut field_chirho.value_chirho, visit_chirho);
            }
        }
        ExprChirho::RecordUpdateChirho {
            expr_chirho,
            fields_chirho,
            ..
        } => {
            visit_expr_chirho(expr_chirho, visit_chirho);
            for field_chirho in fields_chirho {
                visit_expr_chirho(&mut field_chirho.value_chirho, visit_chirho);
            }
        }
        ExprChirho::QuoteDeclChirho { decls_chirho, .. } => {
            for decl_chirho in decls_chirho {
                visit_decl_chirho(decl_chirho, visit_chirho);
            }
        }
        ExprChirho::QuotePatChirho { pat_chirho, .. } => visit_pat_chirho(pat_chirho, visit_chirho),
        ExprChirho::QuoteTypeChirho { .. } => {}
    }
}

/// Patterns bind names; a binder is not an occurrence. A view pattern's
/// function is an expression, so its occurrences are visited.
fn visit_pat_chirho(pat_chirho: &mut PatChirho, visit_chirho: &mut OccurrenceVisitorChirho) {
    match pat_chirho {
        PatChirho::ViewChirho {
            expr_chirho,
            pat_chirho,
            ..
        } => {
            visit_expr_chirho(expr_chirho, visit_chirho);
            visit_pat_chirho(pat_chirho, visit_chirho);
        }
        PatChirho::ConChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                visit_pat_chirho(arg_chirho, visit_chirho);
            }
        }
        PatChirho::TupleChirho {
            elements_chirho, ..
        }
        | PatChirho::ListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                visit_pat_chirho(element_chirho, visit_chirho);
            }
        }
        PatChirho::AsChirho { pattern_chirho, .. } => {
            visit_pat_chirho(pattern_chirho, visit_chirho)
        }
        PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::BangChirho { inner_chirho, .. } => {
            visit_pat_chirho(inner_chirho, visit_chirho)
        }
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            visit_pat_chirho(left_chirho, visit_chirho);
            visit_pat_chirho(right_chirho, visit_chirho);
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                visit_pat_chirho(&mut field_chirho.pattern_chirho, visit_chirho);
            }
        }
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => visit_pat_chirho(pat_chirho, visit_chirho),
        // A literal pattern is a comparison, and a use of its literal.
        PatChirho::LitChirho(lit_chirho) | PatChirho::NegChirho { lit_chirho, .. } => {
            visit_chirho(OccurrenceMutChirho::LiteralChirho(lit_chirho));
        }
        PatChirho::VarChirho(_) | PatChirho::WildcardChirho(_) => {}
    }
}
