// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # TH-to-AST conversion — lowering TH splice output back to haskelujah AST
//!
//! After a TH splice is evaluated, the result is a `Vec<ThDecChirho>` (for
//! declaration splices) or a `ThExpChirho` (for expression splices). This
//! module converts those TH types back into `haskelujah-ast-chirho` types so
//! they can be inserted into the module's AST and continue through the
//! normal compilation pipeline.

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, FieldDeclChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, FieldAssignChirho, LocalBindChirho, MatchArmChirho, RhsChirho,
    StmtChirho,
};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::th_ast_chirho::*;

/// The span used for all TH-generated code (synthetic, not from source).
const TH_SPAN_CHIRHO: SpanChirho = SpanChirho::DUMMY_CHIRHO;

/// Helper: create a raw unqualified NameChirho from a string.
fn mk_ast_name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        TH_SPAN_CHIRHO,
    ))
}

// ---------------------------------------------------------------------------
// Names
// ---------------------------------------------------------------------------

/// Convert a TH name to a haskelujah AST name.
pub fn th_name_to_ast_chirho(name_chirho: &ThNameChirho) -> NameChirho {
    mk_ast_name_chirho(&name_chirho.occ_chirho)
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Convert a TH type to a haskelujah AST type.
pub fn th_type_to_ast_chirho(ty_chirho: &ThTypeChirho) -> TypeChirho {
    match ty_chirho {
        ThTypeChirho::ConTChirho(name_chirho) => {
            TypeChirho::ConChirho(th_name_to_ast_chirho(name_chirho))
        }
        ThTypeChirho::VarTChirho(name_chirho) => {
            TypeChirho::VarChirho(th_name_to_ast_chirho(name_chirho))
        }
        ThTypeChirho::AppTChirho(fun_chirho, arg_chirho) => TypeChirho::AppChirho {
            fun_chirho: Box::new(th_type_to_ast_chirho(fun_chirho)),
            arg_chirho: Box::new(th_type_to_ast_chirho(arg_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThTypeChirho::ArrowTChirho => TypeChirho::ConChirho(mk_ast_name_chirho("->")),
        ThTypeChirho::ListTChirho => TypeChirho::ConChirho(mk_ast_name_chirho("[]")),
        ThTypeChirho::TupleTChirho(n_chirho) => {
            let name_chirho = match n_chirho {
                0 => "()",
                2 => "(,)",
                3 => "(,,)",
                4 => "(,,,)",
                _ => "(,)",
            };
            TypeChirho::ConChirho(mk_ast_name_chirho(name_chirho))
        }
        ThTypeChirho::ForallTChirho(bndrs_chirho, cxt_chirho, ty_chirho) => {
            let inner_chirho = th_type_to_ast_chirho(ty_chirho);
            let with_context_chirho = if cxt_chirho.is_empty() {
                inner_chirho
            } else {
                let constraints_chirho: Vec<ConstraintChirho> = cxt_chirho
                    .iter()
                    .filter_map(|c_chirho| th_type_to_constraint_chirho(c_chirho))
                    .collect();
                TypeChirho::QualChirho {
                    context_chirho: constraints_chirho,
                    body_chirho: Box::new(inner_chirho),
                    span_chirho: TH_SPAN_CHIRHO,
                }
            };
            if bndrs_chirho.is_empty() {
                with_context_chirho
            } else {
                let vars_chirho: Vec<TyVarChirho> = bndrs_chirho
                    .iter()
                    .map(|b_chirho| match b_chirho {
                        ThTyVarBndrChirho::PlainTVChirho(n_chirho) => {
                            TyVarChirho::plain_chirho(th_name_to_ast_chirho(n_chirho))
                        }
                        ThTyVarBndrChirho::KindedTVChirho(n_chirho, _) => {
                            TyVarChirho::plain_chirho(th_name_to_ast_chirho(n_chirho))
                        }
                    })
                    .collect();
                TypeChirho::ForallChirho {
                    vars_chirho,
                    body_chirho: Box::new(with_context_chirho),
                    span_chirho: TH_SPAN_CHIRHO,
                }
            }
        }
        ThTypeChirho::SigTChirho(ty_chirho, _kind_chirho) => {
            // No KindSig variant in AST, just return the type
            th_type_to_ast_chirho(ty_chirho)
        }
        ThTypeChirho::WildCardTChirho => {
            // No Wildcard variant in TypeChirho, use a placeholder
            TypeChirho::VarChirho(mk_ast_name_chirho("_"))
        }
        ThTypeChirho::StarTChirho => TypeChirho::ConChirho(mk_ast_name_chirho("Type")),
        ThTypeChirho::ConstraintTChirho => TypeChirho::ConChirho(mk_ast_name_chirho("Constraint")),
        ThTypeChirho::PromotedTChirho(name_chirho) => TypeChirho::PromotedConChirho {
            name_chirho: th_name_to_ast_chirho(name_chirho),
            span_chirho: TH_SPAN_CHIRHO,
        },
        // For unsupported TH type forms, produce a placeholder
        _ => TypeChirho::ConChirho(mk_ast_name_chirho("__TH_UNSUPPORTED__")),
    }
}

/// Try to interpret a TH type as a typeclass constraint.
fn th_type_to_constraint_chirho(ty_chirho: &ThTypeChirho) -> Option<ConstraintChirho> {
    // Peel off AppT applications to find the class name and arguments
    let mut args_chirho = Vec::new();
    let mut current_chirho = ty_chirho;
    loop {
        match current_chirho {
            ThTypeChirho::AppTChirho(fun_chirho, arg_chirho) => {
                args_chirho.push(th_type_to_ast_chirho(arg_chirho));
                current_chirho = fun_chirho;
            }
            ThTypeChirho::ConTChirho(name_chirho) => {
                args_chirho.reverse();
                return Some(ConstraintChirho::ClassChirho {
                    class_chirho: th_name_to_ast_chirho(name_chirho),
                    args_chirho,
                    span_chirho: TH_SPAN_CHIRHO,
                });
            }
            _ => return None,
        }
    }
}

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

/// Convert a TH pattern to a haskelujah AST pattern.
pub fn th_pat_to_ast_chirho(pat_chirho: &ThPatChirho) -> PatChirho {
    match pat_chirho {
        ThPatChirho::VarPChirho(name_chirho) => {
            PatChirho::VarChirho(th_name_to_ast_chirho(name_chirho))
        }
        ThPatChirho::LitPChirho(lit_chirho) => {
            PatChirho::LitChirho(th_lit_to_ast_chirho(lit_chirho))
        }
        ThPatChirho::WildPChirho => PatChirho::WildcardChirho(TH_SPAN_CHIRHO),
        ThPatChirho::ConPChirho(name_chirho, _tys, pats_chirho) => PatChirho::ConChirho {
            con_chirho: th_name_to_ast_chirho(name_chirho),
            args_chirho: pats_chirho
                .iter()
                .map(|p_chirho| th_pat_to_ast_chirho(p_chirho))
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThPatChirho::TupPChirho(pats_chirho) => PatChirho::TupleChirho {
            elements_chirho: pats_chirho
                .iter()
                .map(|p_chirho| th_pat_to_ast_chirho(p_chirho))
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThPatChirho::ListPChirho(pats_chirho) => PatChirho::ListChirho {
            elements_chirho: pats_chirho
                .iter()
                .map(|p_chirho| th_pat_to_ast_chirho(p_chirho))
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThPatChirho::AsPChirho(name_chirho, pat_inner_chirho) => PatChirho::AsChirho {
            name_chirho: th_name_to_ast_chirho(name_chirho),
            pattern_chirho: Box::new(th_pat_to_ast_chirho(pat_inner_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThPatChirho::BangPChirho(inner_chirho) => PatChirho::BangChirho {
            inner_chirho: Box::new(th_pat_to_ast_chirho(inner_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThPatChirho::TildePChirho(inner_chirho) => PatChirho::LazyChirho {
            inner_chirho: Box::new(th_pat_to_ast_chirho(inner_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        _ => PatChirho::WildcardChirho(TH_SPAN_CHIRHO), // Fallback for unsupported patterns
    }
}

// ---------------------------------------------------------------------------
// Literals
// ---------------------------------------------------------------------------

/// Convert a TH literal to a haskelujah AST literal.
pub fn th_lit_to_ast_chirho(lit_chirho: &ThLitChirho) -> LitChirho {
    match lit_chirho {
        ThLitChirho::IntegerLChirho(n_chirho) => LitChirho::IntChirho(*n_chirho, TH_SPAN_CHIRHO),
        ThLitChirho::RationalLChirho(f_chirho) => LitChirho::FloatChirho(*f_chirho, TH_SPAN_CHIRHO),
        ThLitChirho::CharLChirho(c_chirho) => LitChirho::CharChirho(*c_chirho, TH_SPAN_CHIRHO),
        ThLitChirho::StringLChirho(s_chirho) => {
            LitChirho::StringChirho(s_chirho.clone(), TH_SPAN_CHIRHO)
        }
        ThLitChirho::IntPrimLChirho(n_chirho) => LitChirho::IntChirho(*n_chirho, TH_SPAN_CHIRHO),
        ThLitChirho::FloatPrimLChirho(f_chirho) => {
            LitChirho::FloatChirho(*f_chirho, TH_SPAN_CHIRHO)
        }
        ThLitChirho::DoublePrimLChirho(f_chirho) => {
            LitChirho::FloatChirho(*f_chirho, TH_SPAN_CHIRHO)
        }
        _ => LitChirho::IntChirho(0, TH_SPAN_CHIRHO), // Fallback
    }
}

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

/// Convert a TH expression to a haskelujah AST expression.
pub fn th_exp_to_ast_chirho(exp_chirho: &ThExpChirho) -> ExprChirho {
    match exp_chirho {
        ThExpChirho::VarEChirho(name_chirho) => {
            ExprChirho::VarChirho(th_name_to_ast_chirho(name_chirho))
        }
        ThExpChirho::ConEChirho(name_chirho) => {
            ExprChirho::ConChirho(th_name_to_ast_chirho(name_chirho))
        }
        ThExpChirho::LitEChirho(lit_chirho) => {
            ExprChirho::LitChirho(th_lit_to_ast_chirho(lit_chirho))
        }
        ThExpChirho::AppEChirho(fun_chirho, arg_chirho) => ExprChirho::AppChirho {
            fun_chirho: Box::new(th_exp_to_ast_chirho(fun_chirho)),
            arg_chirho: Box::new(th_exp_to_ast_chirho(arg_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::LamEChirho(pats_chirho, body_chirho) => ExprChirho::LamChirho {
            pats_chirho: pats_chirho
                .iter()
                .map(|p_chirho| th_pat_to_ast_chirho(p_chirho))
                .collect(),
            body_chirho: Box::new(th_exp_to_ast_chirho(body_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::TupEChirho(elts_chirho) => ExprChirho::TupleChirho {
            elements_chirho: elts_chirho
                .iter()
                .filter_map(|e_chirho| {
                    e_chirho
                        .as_ref()
                        .map(|e_chirho| th_exp_to_ast_chirho(e_chirho))
                })
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::ListEChirho(elts_chirho) => ExprChirho::ListChirho {
            elements_chirho: elts_chirho
                .iter()
                .map(|e_chirho| th_exp_to_ast_chirho(e_chirho))
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::CondEChirho(cond_chirho, then_chirho, else_chirho) => ExprChirho::IfChirho {
            cond_chirho: Box::new(th_exp_to_ast_chirho(cond_chirho)),
            then_chirho: Box::new(th_exp_to_ast_chirho(then_chirho)),
            else_chirho: Box::new(th_exp_to_ast_chirho(else_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::CaseEChirho(scrut_chirho, matches_chirho) => ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(th_exp_to_ast_chirho(scrut_chirho)),
            alts_chirho: matches_chirho
                .iter()
                .map(|m_chirho| th_match_to_alt_chirho(m_chirho))
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::LetEChirho(decs_chirho, body_chirho) => {
            let binds_chirho: Vec<LocalBindChirho> = decs_chirho
                .iter()
                .filter_map(|d_chirho| th_dec_to_local_bind_chirho(d_chirho))
                .collect();
            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho: Box::new(th_exp_to_ast_chirho(body_chirho)),
                span_chirho: TH_SPAN_CHIRHO,
            }
        }
        ThExpChirho::DoEChirho(_, stmts_chirho) => ExprChirho::DoChirho {
            qualifier_chirho: None,
            stmts_chirho: stmts_chirho
                .iter()
                .map(|s_chirho| th_stmt_to_ast_chirho(s_chirho))
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::InfixEChirho(left_chirho, op_chirho, right_chirho) => {
            let op_name_chirho = match op_chirho.as_ref() {
                ThExpChirho::VarEChirho(n_chirho) => th_name_to_ast_chirho(n_chirho),
                ThExpChirho::ConEChirho(n_chirho) => th_name_to_ast_chirho(n_chirho),
                _ => mk_ast_name_chirho("__op__"),
            };
            match (left_chirho, right_chirho) {
                (Some(l_chirho), Some(r_chirho)) => ExprChirho::InfixChirho {
                    left_chirho: Box::new(th_exp_to_ast_chirho(l_chirho)),
                    op_chirho: op_name_chirho,
                    right_chirho: Box::new(th_exp_to_ast_chirho(r_chirho)),
                    span_chirho: TH_SPAN_CHIRHO,
                },
                (None, Some(r_chirho)) => ExprChirho::LeftSectionChirho {
                    op_chirho: op_name_chirho,
                    arg_chirho: Box::new(th_exp_to_ast_chirho(r_chirho)),
                    span_chirho: TH_SPAN_CHIRHO,
                },
                (Some(l_chirho), None) => ExprChirho::RightSectionChirho {
                    arg_chirho: Box::new(th_exp_to_ast_chirho(l_chirho)),
                    op_chirho: op_name_chirho,
                    span_chirho: TH_SPAN_CHIRHO,
                },
                (None, None) => ExprChirho::VarChirho(op_name_chirho),
            }
        }
        ThExpChirho::SigEChirho(expr_chirho, ty_chirho) => ExprChirho::AnnChirho {
            expr_chirho: Box::new(th_exp_to_ast_chirho(expr_chirho)),
            ty_chirho: th_type_to_ast_chirho(ty_chirho),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::RecConEChirho(name_chirho, fields_chirho) => ExprChirho::RecordConChirho {
            con_chirho: th_name_to_ast_chirho(name_chirho),
            has_wildcard_chirho: false,
            fields_chirho: fields_chirho
                .iter()
                .map(|f_chirho| FieldAssignChirho {
                    name_chirho: th_name_to_ast_chirho(&f_chirho.name_chirho),
                    value_chirho: th_exp_to_ast_chirho(&f_chirho.expr_chirho),
                    span_chirho: TH_SPAN_CHIRHO,
                })
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::RecUpdEChirho(expr_chirho, fields_chirho) => ExprChirho::RecordUpdateChirho {
            expr_chirho: Box::new(th_exp_to_ast_chirho(expr_chirho)),
            fields_chirho: fields_chirho
                .iter()
                .map(|f_chirho| FieldAssignChirho {
                    name_chirho: th_name_to_ast_chirho(&f_chirho.name_chirho),
                    value_chirho: th_exp_to_ast_chirho(&f_chirho.expr_chirho),
                    span_chirho: TH_SPAN_CHIRHO,
                })
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThExpChirho::ParensEChirho(inner_chirho) => ExprChirho::ParenChirho {
            inner_chirho: Box::new(th_exp_to_ast_chirho(inner_chirho)),
            span_chirho: TH_SPAN_CHIRHO,
        },
        // Fallback for unsupported expression forms
        _ => ExprChirho::VarChirho(mk_ast_name_chirho("__TH_UNSUPPORTED__")),
    }
}

/// Convert a TH match to a haskelujah case alternative.
fn th_match_to_alt_chirho(match_chirho: &ThMatchChirho) -> AltChirho {
    AltChirho {
        pat_chirho: th_pat_to_ast_chirho(&match_chirho.pat_chirho),
        rhs_chirho: th_body_to_rhs_chirho(&match_chirho.body_chirho),
        where_binds_chirho: match_chirho
            .decs_chirho
            .iter()
            .filter_map(|d_chirho| th_dec_to_local_bind_chirho(d_chirho))
            .collect(),
        span_chirho: TH_SPAN_CHIRHO,
    }
}

/// Convert a TH body to a haskelujah RHS.
fn th_body_to_rhs_chirho(body_chirho: &ThBodyChirho) -> RhsChirho {
    match body_chirho {
        ThBodyChirho::NormalBChirho(expr_chirho) => {
            RhsChirho::UnguardedChirho(th_exp_to_ast_chirho(expr_chirho))
        }
        ThBodyChirho::GuardedBChirho(guards_chirho) => RhsChirho::GuardedChirho(
            guards_chirho
                .iter()
                .map(|(g_chirho, e_chirho)| {
                    let guard_expr_chirho = match g_chirho {
                        ThGuardChirho::NormalGChirho(ge_chirho) => th_exp_to_ast_chirho(ge_chirho),
                        ThGuardChirho::PatGChirho(_, ge_chirho) => th_exp_to_ast_chirho(ge_chirho),
                    };
                    haskelujah_ast_chirho::expr_chirho::GuardedExprChirho {
                        guard_chirho: guard_expr_chirho,
                        body_chirho: th_exp_to_ast_chirho(e_chirho),
                        span_chirho: TH_SPAN_CHIRHO,
                    }
                })
                .collect(),
        ),
    }
}

/// Convert a TH statement to a haskelujah statement.
fn th_stmt_to_ast_chirho(stmt_chirho: &ThStmtChirho) -> StmtChirho {
    match stmt_chirho {
        ThStmtChirho::BindSChirho(pat_chirho, expr_chirho) => StmtChirho::BindChirho {
            pat_chirho: th_pat_to_ast_chirho(pat_chirho),
            expr_chirho: th_exp_to_ast_chirho(expr_chirho),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThStmtChirho::LetSChirho(decs_chirho) => StmtChirho::LetChirho {
            binds_chirho: decs_chirho
                .iter()
                .filter_map(|d_chirho| th_dec_to_local_bind_chirho(d_chirho))
                .collect(),
            span_chirho: TH_SPAN_CHIRHO,
        },
        ThStmtChirho::NoBindSChirho(expr_chirho) => {
            StmtChirho::ExprChirho(th_exp_to_ast_chirho(expr_chirho))
        }
        _ => StmtChirho::ExprChirho(ExprChirho::VarChirho(mk_ast_name_chirho(
            "__TH_UNSUPPORTED__",
        ))),
    }
}

/// Convert a TH declaration to a local binding (for let/where).
fn th_dec_to_local_bind_chirho(dec_chirho: &ThDecChirho) -> Option<LocalBindChirho> {
    match dec_chirho {
        ThDecChirho::FunDChirho(name_chirho, clauses_chirho) => {
            let matches_chirho: Vec<MatchArmChirho> = clauses_chirho
                .iter()
                .map(|c_chirho| MatchArmChirho {
                    pats_chirho: c_chirho
                        .pats_chirho
                        .iter()
                        .map(|p_chirho| th_pat_to_ast_chirho(p_chirho))
                        .collect(),
                    rhs_chirho: th_body_to_rhs_chirho(&c_chirho.body_chirho),
                    where_binds_chirho: c_chirho
                        .decs_chirho
                        .iter()
                        .filter_map(|d_chirho| th_dec_to_local_bind_chirho(d_chirho))
                        .collect(),
                    span_chirho: TH_SPAN_CHIRHO,
                })
                .collect();
            Some(LocalBindChirho::FunBindChirho {
                name_chirho: th_name_to_ast_chirho(name_chirho),
                matches_chirho,
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        ThDecChirho::ValDChirho(pat_chirho, body_chirho, _decs) => {
            Some(LocalBindChirho::PatBindChirho {
                pat_chirho: th_pat_to_ast_chirho(pat_chirho),
                rhs_chirho: th_body_to_rhs_chirho(body_chirho),
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Declarations
// ---------------------------------------------------------------------------

/// Convert a TH declaration to a haskelujah top-level declaration.
pub fn th_dec_to_ast_chirho(dec_chirho: &ThDecChirho) -> Option<DeclChirho> {
    match dec_chirho {
        ThDecChirho::FunDChirho(name_chirho, clauses_chirho) => {
            let matches_chirho: Vec<MatchArmChirho> = clauses_chirho
                .iter()
                .map(|c_chirho| MatchArmChirho {
                    pats_chirho: c_chirho
                        .pats_chirho
                        .iter()
                        .map(|p_chirho| th_pat_to_ast_chirho(p_chirho))
                        .collect(),
                    rhs_chirho: th_body_to_rhs_chirho(&c_chirho.body_chirho),
                    where_binds_chirho: c_chirho
                        .decs_chirho
                        .iter()
                        .filter_map(|d_chirho| th_dec_to_local_bind_chirho(d_chirho))
                        .collect(),
                    span_chirho: TH_SPAN_CHIRHO,
                })
                .collect();
            Some(DeclChirho::FunBindChirho {
                name_chirho: th_name_to_ast_chirho(name_chirho),
                matches_chirho,
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        ThDecChirho::SigDChirho(name_chirho, ty_chirho) => Some(DeclChirho::TypeSigChirho {
            name_chirho: th_name_to_ast_chirho(name_chirho),
            ty_chirho: th_type_to_ast_chirho(ty_chirho),
            span_chirho: TH_SPAN_CHIRHO,
        }),
        ThDecChirho::DataDChirho(
            _cxt_chirho,
            name_chirho,
            tyvars_chirho,
            _kind,
            cons_chirho,
            derivs_chirho,
        ) => {
            let ast_tyvars_chirho: Vec<TyVarChirho> = tyvars_chirho
                .iter()
                .map(|tv_chirho| match tv_chirho {
                    ThTyVarBndrChirho::PlainTVChirho(n_chirho) => {
                        TyVarChirho::plain_chirho(th_name_to_ast_chirho(n_chirho))
                    }
                    ThTyVarBndrChirho::KindedTVChirho(n_chirho, _) => {
                        TyVarChirho::plain_chirho(th_name_to_ast_chirho(n_chirho))
                    }
                })
                .collect();
            let ast_cons_chirho: Vec<ConDeclChirho> = cons_chirho
                .iter()
                .filter_map(|c_chirho| th_con_to_ast_chirho(c_chirho))
                .collect();
            let deriving_names_chirho: Vec<NameChirho> = derivs_chirho
                .iter()
                .flat_map(|dc_chirho| {
                    dc_chirho.classes_chirho.iter().filter_map(|t_chirho| {
                        if let ThTypeChirho::ConTChirho(n_chirho) = t_chirho {
                            Some(th_name_to_ast_chirho(n_chirho))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            Some(DeclChirho::DataDeclChirho {
                name_chirho: th_name_to_ast_chirho(name_chirho),
                type_vars_chirho: ast_tyvars_chirho,
                constructors_chirho: ast_cons_chirho,
                deriving_chirho: deriving_names_chirho,
                kind_sig_chirho: None,
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        ThDecChirho::TySynDChirho(name_chirho, tyvars_chirho, rhs_chirho) => {
            let ast_tyvars_chirho: Vec<TyVarChirho> = tyvars_chirho
                .iter()
                .map(|tv_chirho| match tv_chirho {
                    ThTyVarBndrChirho::PlainTVChirho(n_chirho) => {
                        TyVarChirho::plain_chirho(th_name_to_ast_chirho(n_chirho))
                    }
                    ThTyVarBndrChirho::KindedTVChirho(n_chirho, _) => {
                        TyVarChirho::plain_chirho(th_name_to_ast_chirho(n_chirho))
                    }
                })
                .collect();
            Some(DeclChirho::TypeAliasDeclChirho {
                name_chirho: th_name_to_ast_chirho(name_chirho),
                type_vars_chirho: ast_tyvars_chirho,
                rhs_chirho: th_type_to_ast_chirho(rhs_chirho),
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        ThDecChirho::InstanceDChirho(_overlap, cxt_chirho, ty_chirho, methods_chirho) => {
            // Extract class name and type arguments from the instance type
            let (class_name_chirho, type_args_chirho) = decompose_instance_type_chirho(ty_chirho);
            let constraints_chirho: Vec<ConstraintChirho> = cxt_chirho
                .iter()
                .filter_map(|c_chirho| th_type_to_constraint_chirho(c_chirho))
                .collect();
            let method_binds_chirho: Vec<LocalBindChirho> = methods_chirho
                .iter()
                .filter_map(|d_chirho| th_dec_to_local_bind_chirho(d_chirho))
                .collect();
            Some(DeclChirho::InstanceDeclChirho {
                context_chirho: constraints_chirho,
                class_chirho: class_name_chirho,
                types_chirho: type_args_chirho,
                methods_chirho: method_binds_chirho,
                assoc_tf_instances_chirho: vec![],
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        ThDecChirho::ValDChirho(pat_chirho, body_chirho, _decs) => {
            Some(DeclChirho::PatBindChirho {
                pat_chirho: th_pat_to_ast_chirho(pat_chirho),
                rhs_chirho: th_body_to_rhs_chirho(body_chirho),
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        _ => None, // Other declaration types not yet supported
    }
}

/// Decompose an instance type like `AppT (ConT "Show") (ConT "Int")` into class name + args.
fn decompose_instance_type_chirho(ty_chirho: &ThTypeChirho) -> (NameChirho, Vec<TypeChirho>) {
    let mut args_chirho = Vec::new();
    let mut current_chirho = ty_chirho;
    loop {
        match current_chirho {
            ThTypeChirho::AppTChirho(fun_chirho, arg_chirho) => {
                args_chirho.push(th_type_to_ast_chirho(arg_chirho));
                current_chirho = fun_chirho;
            }
            ThTypeChirho::ConTChirho(name_chirho) => {
                args_chirho.reverse();
                return (th_name_to_ast_chirho(name_chirho), args_chirho);
            }
            _ => {
                args_chirho.reverse();
                return (mk_ast_name_chirho("__UNKNOWN_CLASS__"), args_chirho);
            }
        }
    }
}

/// Convert a TH constructor to a haskelujah constructor declaration.
fn th_con_to_ast_chirho(con_chirho: &ThConChirho) -> Option<ConDeclChirho> {
    match con_chirho {
        ThConChirho::NormalCChirho(name_chirho, bang_types_chirho) => {
            Some(ConDeclChirho::OrdinaryChirho {
                name_chirho: th_name_to_ast_chirho(name_chirho),
                fields_chirho: bang_types_chirho
                    .iter()
                    .map(|bt_chirho| {
                        (
                            StrictnessChirho::LazyChirho,
                            th_type_to_ast_chirho(&bt_chirho.ty_chirho),
                        )
                    })
                    .collect(),
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        ThConChirho::RecCChirho(name_chirho, var_bang_types_chirho) => {
            Some(ConDeclChirho::RecordChirho {
                name_chirho: th_name_to_ast_chirho(name_chirho),
                fields_chirho: var_bang_types_chirho
                    .iter()
                    .map(|vbt_chirho| FieldDeclChirho {
                        names_chirho: vec![th_name_to_ast_chirho(&vbt_chirho.name_chirho)],
                        ty_chirho: th_type_to_ast_chirho(&vbt_chirho.ty_chirho),
                        strictness_chirho: StrictnessChirho::LazyChirho,
                        span_chirho: TH_SPAN_CHIRHO,
                    })
                    .collect(),
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        ThConChirho::GadtCChirho(names_chirho, bang_types_chirho, ret_ty_chirho) => {
            // Use the first name for the GADT constructor.
            let con_name_chirho = names_chirho
                .first()
                .map(|n_chirho| th_name_to_ast_chirho(n_chirho))
                .unwrap_or_else(|| mk_ast_name_chirho("__UNKNOWN_GADT__"));
            // Build the full type: arg1 -> arg2 -> ... -> ret_ty
            let mut full_ty_chirho = th_type_to_ast_chirho(ret_ty_chirho);
            for bt_chirho in bang_types_chirho.iter().rev() {
                full_ty_chirho = TypeChirho::FunChirho {
                    arg_chirho: Box::new(th_type_to_ast_chirho(&bt_chirho.ty_chirho)),
                    mult_chirho: None,
                    result_chirho: Box::new(full_ty_chirho),
                    span_chirho: TH_SPAN_CHIRHO,
                };
            }
            Some(ConDeclChirho::GadtChirho {
                name_chirho: con_name_chirho,
                ty_chirho: full_ty_chirho,
                span_chirho: TH_SPAN_CHIRHO,
            })
        }
        _ => None, // Infix/RecGadtC constructors not yet supported
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn roundtrip_simple_type_chirho() {
        // Int -> Bool should roundtrip
        let th_ty_chirho = ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::AppTChirho(
                Box::new(ThTypeChirho::ArrowTChirho),
                Box::new(ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                    "Int",
                ))),
            )),
            Box::new(ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                "Bool",
            ))),
        );

        let ast_ty_chirho = th_type_to_ast_chirho(&th_ty_chirho);
        match ast_ty_chirho {
            TypeChirho::AppChirho { arg_chirho, .. } => match *arg_chirho {
                TypeChirho::ConChirho(n_chirho) => assert_eq!(n_chirho.text_chirho(), "Bool"),
                _ => panic!("expected Con Bool"),
            },
            _ => panic!("expected AppT"),
        }
    }

    #[test]
    fn convert_th_fun_dec_chirho() {
        // FunD "f" [Clause [VarP "x"] (NormalB (VarE "x")) []]
        let th_dec_chirho = ThDecChirho::FunDChirho(
            ThNameChirho::mk_name_chirho("f"),
            vec![ThClauseChirho {
                pats_chirho: vec![ThPatChirho::VarPChirho(ThNameChirho::mk_name_chirho("x"))],
                body_chirho: ThBodyChirho::NormalBChirho(ThExpChirho::VarEChirho(
                    ThNameChirho::mk_name_chirho("x"),
                )),
                decs_chirho: vec![],
            }],
        );

        let ast_dec_chirho = th_dec_to_ast_chirho(&th_dec_chirho).unwrap();
        match ast_dec_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), "f");
                assert_eq!(matches_chirho.len(), 1);
                assert_eq!(matches_chirho[0].pats_chirho.len(), 1);
            }
            _ => panic!("expected FunBind"),
        }
    }

    #[test]
    fn convert_th_data_dec_chirho() {
        // DataD [] "Foo" [PlainTV "a"] Nothing [NormalC "MkFoo" [(Bang NoSourceUnpackedness NoSourceStrictness, VarT "a")]] []
        let th_dec_chirho = ThDecChirho::DataDChirho(
            vec![],
            ThNameChirho::mk_name_chirho("Foo"),
            vec![ThTyVarBndrChirho::PlainTVChirho(
                ThNameChirho::mk_name_chirho("a"),
            )],
            None,
            vec![ThConChirho::NormalCChirho(
                ThNameChirho::mk_name_chirho("MkFoo"),
                vec![ThBangTypeChirho {
                    bang_chirho: ThBangChirho::default_bang_chirho(),
                    ty_chirho: ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho("a")),
                }],
            )],
            vec![],
        );

        let ast_dec_chirho = th_dec_to_ast_chirho(&th_dec_chirho).unwrap();
        match ast_dec_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                ..
            } => {
                assert_eq!(name_chirho.text_chirho(), "Foo");
                assert_eq!(type_vars_chirho.len(), 1);
                assert_eq!(constructors_chirho.len(), 1);
            }
            _ => panic!("expected DataDecl"),
        }
    }
}
