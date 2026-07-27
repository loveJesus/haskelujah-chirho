// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Linearity checking for LinearTypes
//!
//! After type inference, this pass walks the typed AST and verifies
//! that variables bound by linear function arrows (`%1 ->` or `⊸`) are used
//! exactly once. Violations produce warnings (not errors, for now).

use std::collections::HashMap;

use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, StmtChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

/// A linearity violation.
#[derive(Debug, Clone, PartialEq)]
pub enum LinearityViolationChirho {
    /// A linear variable was used more than once.
    UsedMultipleChirho {
        name_chirho: String,
        count_chirho: usize,
        span_chirho: SpanChirho,
    },
    /// A linear variable was never used.
    UnusedLinearChirho {
        name_chirho: String,
        span_chirho: SpanChirho,
    },
}

/// Count how many times each variable name appears in an expression.
pub fn count_var_uses_chirho(expr_chirho: &ExprChirho) -> HashMap<String, usize> {
    let mut counts_chirho = HashMap::new();
    walk_expr_chirho(expr_chirho, &mut counts_chirho);
    counts_chirho
}

fn walk_expr_chirho(expr_chirho: &ExprChirho, counts_chirho: &mut HashMap<String, usize>) {
    match expr_chirho {
        ExprChirho::VarChirho(name_chirho) => {
            *counts_chirho
                .entry(name_chirho.text_chirho().to_string())
                .or_insert(0) += 1;
        }
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            walk_expr_chirho(fun_chirho, counts_chirho);
            walk_expr_chirho(arg_chirho, counts_chirho);
        }
        ExprChirho::LamChirho { body_chirho, .. } => {
            walk_expr_chirho(body_chirho, counts_chirho);
        }
        ExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for bind_chirho in binds_chirho {
                walk_local_bind_chirho(bind_chirho, counts_chirho);
            }
            walk_expr_chirho(body_chirho, counts_chirho);
        }
        ExprChirho::IfChirho {
            cond_chirho,
            then_chirho,
            else_chirho,
            ..
        } => {
            walk_expr_chirho(cond_chirho, counts_chirho);
            walk_expr_chirho(then_chirho, counts_chirho);
            walk_expr_chirho(else_chirho, counts_chirho);
        }
        ExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            walk_expr_chirho(scrutinee_chirho, counts_chirho);
            for alt_chirho in alts_chirho {
                walk_rhs_chirho(&alt_chirho.rhs_chirho, counts_chirho);
            }
        }
        ExprChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                walk_expr_chirho(elem_chirho, counts_chirho);
            }
        }
        ExprChirho::ListChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                walk_expr_chirho(elem_chirho, counts_chirho);
            }
        }
        ExprChirho::NegChirho {
            expr_chirho: inner_chirho,
            ..
        } => {
            walk_expr_chirho(inner_chirho, counts_chirho);
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => {
            for stmt_chirho in stmts_chirho {
                match stmt_chirho {
                    StmtChirho::ExprChirho(e_chirho) => {
                        walk_expr_chirho(e_chirho, counts_chirho);
                    }
                    StmtChirho::BindChirho {
                        expr_chirho: e_chirho,
                        ..
                    } => {
                        walk_expr_chirho(e_chirho, counts_chirho);
                    }
                    StmtChirho::LetChirho { binds_chirho, .. } => {
                        for bind_chirho in binds_chirho {
                            walk_local_bind_chirho(bind_chirho, counts_chirho);
                        }
                    }
                }
            }
        }
        ExprChirho::InfixChirho {
            left_chirho,
            right_chirho,
            op_chirho,
            ..
        } => {
            walk_expr_chirho(left_chirho, counts_chirho);
            *counts_chirho
                .entry(op_chirho.text_chirho().to_string())
                .or_insert(0) += 1;
            walk_expr_chirho(right_chirho, counts_chirho);
        }
        ExprChirho::TypeAppChirho {
            expr_chirho: inner_chirho,
            ..
        } => {
            walk_expr_chirho(inner_chirho, counts_chirho);
        }
        ExprChirho::ArithSeqChirho {
            from_chirho,
            then_chirho,
            to_chirho,
            ..
        } => {
            walk_expr_chirho(from_chirho, counts_chirho);
            if let Some(t_chirho) = then_chirho {
                walk_expr_chirho(t_chirho, counts_chirho);
            }
            if let Some(t_chirho) = to_chirho {
                walk_expr_chirho(t_chirho, counts_chirho);
            }
        }
        ExprChirho::ListCompChirho {
            body_chirho,
            quals_chirho,
            parallel_quals_chirho,
            ..
        } => {
            walk_expr_chirho(body_chirho, counts_chirho);
            for stmt_chirho in quals_chirho
                .iter()
                .chain(parallel_quals_chirho.iter().flatten())
            {
                match stmt_chirho {
                    StmtChirho::ExprChirho(expr_chirho)
                    | StmtChirho::BindChirho { expr_chirho, .. } => {
                        walk_expr_chirho(expr_chirho, counts_chirho);
                    }
                    StmtChirho::LetChirho { binds_chirho, .. } => {
                        for bind_chirho in binds_chirho {
                            walk_local_bind_chirho(bind_chirho, counts_chirho);
                        }
                    }
                }
            }
        }
        // Literals, constructors, wildcards — no variable usage
        _ => {}
    }
}

fn walk_local_bind_chirho(
    bind_chirho: &LocalBindChirho,
    counts_chirho: &mut HashMap<String, usize>,
) {
    match bind_chirho {
        LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
            for arm_chirho in matches_chirho {
                walk_rhs_chirho(&arm_chirho.rhs_chirho, counts_chirho);
            }
        }
        LocalBindChirho::PatBindChirho { rhs_chirho, .. } => {
            walk_rhs_chirho(rhs_chirho, counts_chirho);
        }
        LocalBindChirho::TypeSigChirho { .. } => {}
    }
}

fn walk_rhs_chirho(
    rhs_chirho: &haskelujah_ast_chirho::expr_chirho::RhsChirho,
    counts_chirho: &mut HashMap<String, usize>,
) {
    match rhs_chirho {
        haskelujah_ast_chirho::expr_chirho::RhsChirho::UnguardedChirho(e_chirho) => {
            walk_expr_chirho(e_chirho, counts_chirho);
        }
        haskelujah_ast_chirho::expr_chirho::RhsChirho::GuardedChirho(guards_chirho) => {
            for ge_chirho in guards_chirho {
                walk_expr_chirho(&ge_chirho.guard_chirho, counts_chirho);
                walk_expr_chirho(&ge_chirho.body_chirho, counts_chirho);
            }
        }
    }
}

/// Extract variable names bound by a pattern.
pub fn pat_bound_names_chirho(pat_chirho: &PatChirho) -> Vec<String> {
    let mut names_chirho = Vec::new();
    collect_pat_names_chirho(pat_chirho, &mut names_chirho);
    names_chirho
}

fn collect_pat_names_chirho(pat_chirho: &PatChirho, out_chirho: &mut Vec<String>) {
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => {
            out_chirho.push(name_chirho.text_chirho().to_string());
        }
        PatChirho::ConChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                collect_pat_names_chirho(arg_chirho, out_chirho);
            }
        }
        PatChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                collect_pat_names_chirho(elem_chirho, out_chirho);
            }
        }
        PatChirho::ListChirho {
            elements_chirho, ..
        } => {
            for elem_chirho in elements_chirho {
                collect_pat_names_chirho(elem_chirho, out_chirho);
            }
        }
        PatChirho::AsChirho {
            name_chirho,
            pattern_chirho,
            ..
        } => {
            out_chirho.push(name_chirho.text_chirho().to_string());
            collect_pat_names_chirho(pattern_chirho, out_chirho);
        }
        PatChirho::ParenChirho { inner_chirho, .. } => {
            collect_pat_names_chirho(inner_chirho, out_chirho);
        }
        PatChirho::BangChirho { inner_chirho, .. } => {
            collect_pat_names_chirho(inner_chirho, out_chirho);
        }
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            collect_pat_names_chirho(left_chirho, out_chirho);
            collect_pat_names_chirho(right_chirho, out_chirho);
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                collect_pat_names_chirho(&field_chirho.pattern_chirho, out_chirho);
            }
        }
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
            collect_pat_names_chirho(pat_chirho, out_chirho);
        }
        PatChirho::LazyChirho { inner_chirho, .. } => {
            collect_pat_names_chirho(inner_chirho, out_chirho);
        }
        PatChirho::ViewChirho { pat_chirho, .. } => {
            collect_pat_names_chirho(pat_chirho, out_chirho);
        }
        _ => {}
    }
}

/// Check linearity of a lambda body: given parameter names and their
/// expected multiplicities (true = linear), verify usage counts.
pub fn check_linearity_chirho(
    param_names_chirho: &[(String, bool)],
    body_chirho: &ExprChirho,
    span_chirho: SpanChirho,
) -> Vec<LinearityViolationChirho> {
    let counts_chirho = count_var_uses_chirho(body_chirho);
    let mut violations_chirho = Vec::new();

    for (name_chirho, is_linear_chirho) in param_names_chirho {
        if !is_linear_chirho {
            continue;
        }
        let count_chirho = counts_chirho.get(name_chirho).copied().unwrap_or(0);
        if count_chirho == 0 {
            violations_chirho.push(LinearityViolationChirho::UnusedLinearChirho {
                name_chirho: name_chirho.clone(),
                span_chirho,
            });
        } else if count_chirho > 1 {
            violations_chirho.push(LinearityViolationChirho::UsedMultipleChirho {
                name_chirho: name_chirho.clone(),
                count_chirho,
                span_chirho,
            });
        }
    }

    violations_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::lit_chirho::LitChirho;
    use haskelujah_ast_chirho::name_chirho::NameChirho;

    fn mk_name_chirho(s_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(haskelujah_ast_chirho::name_chirho::RawNameChirho {
            text_chirho: s_chirho.to_string(),
            qualifier_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        })
    }

    #[test]
    fn linear_used_once_ok_chirho() {
        // \x -> x (used once = OK for linear)
        let body_chirho = ExprChirho::VarChirho(mk_name_chirho("x"));
        let params_chirho = vec![("x".to_string(), true)];
        let violations_chirho =
            check_linearity_chirho(&params_chirho, &body_chirho, SpanChirho::DUMMY_CHIRHO);
        assert!(violations_chirho.is_empty());
    }

    #[test]
    fn linear_unused_violation_chirho() {
        // \x -> 42 (x not used = violation)
        let body_chirho = ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO));
        let params_chirho = vec![("x".to_string(), true)];
        let violations_chirho =
            check_linearity_chirho(&params_chirho, &body_chirho, SpanChirho::DUMMY_CHIRHO);
        assert_eq!(violations_chirho.len(), 1);
        assert!(matches!(
            &violations_chirho[0],
            LinearityViolationChirho::UnusedLinearChirho { name_chirho, .. } if name_chirho == "x"
        ));
    }

    #[test]
    fn linear_used_twice_violation_chirho() {
        // \x -> (x, x) — used twice = violation
        let body_chirho = ExprChirho::TupleChirho {
            elements_chirho: vec![
                ExprChirho::VarChirho(mk_name_chirho("x")),
                ExprChirho::VarChirho(mk_name_chirho("x")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let params_chirho = vec![("x".to_string(), true)];
        let violations_chirho =
            check_linearity_chirho(&params_chirho, &body_chirho, SpanChirho::DUMMY_CHIRHO);
        assert_eq!(violations_chirho.len(), 1);
        assert!(matches!(
            &violations_chirho[0],
            LinearityViolationChirho::UsedMultipleChirho { name_chirho, count_chirho: 2, .. } if name_chirho == "x"
        ));
    }

    #[test]
    fn unrestricted_unused_ok_chirho() {
        // \x -> 42 with unrestricted x — no violation
        let body_chirho = ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO));
        let params_chirho = vec![("x".to_string(), false)];
        let violations_chirho =
            check_linearity_chirho(&params_chirho, &body_chirho, SpanChirho::DUMMY_CHIRHO);
        assert!(violations_chirho.is_empty());
    }

    #[test]
    fn count_var_uses_basic_chirho() {
        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(mk_name_chirho("f"))),
            arg_chirho: Box::new(ExprChirho::VarChirho(mk_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let counts_chirho = count_var_uses_chirho(&expr_chirho);
        assert_eq!(counts_chirho.get("f"), Some(&1));
        assert_eq!(counts_chirho.get("x"), Some(&1));
    }
}
