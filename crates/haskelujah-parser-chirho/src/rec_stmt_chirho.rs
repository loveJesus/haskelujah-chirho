// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Frontend-only RecursiveDo statement transformation.
//!
//! The CST preserves explicit `rec` groups, but the typed AST and later phases continue to see
//! ordinary do statements. Each recursive group becomes one `mfix` knot returning its bound
//! names as a tuple. Core lowering supplies genuinely lazy tuple projections.
//!
//! workflow: recursive-do-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::{PatChirho, PatFieldChirho};
use haskelujah_span_chirho::SpanChirho;

/// A do statement or an explicit recursive statement group before AST normalization.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DoSegmentChirho {
    StatementChirho(StmtChirho),
    RecursiveChirho {
        stmts_chirho: Vec<StmtChirho>,
        span_chirho: SpanChirho,
    },
}

/// Normalize explicit `rec` groups and `mdo` into ordinary AST do statements.
pub(crate) fn transform_recursive_do_chirho(
    segments_chirho: Vec<DoSegmentChirho>,
    is_mdo_chirho: bool,
    span_chirho: SpanChirho,
) -> Vec<StmtChirho> {
    if is_mdo_chirho {
        let mut stmts_chirho = flatten_segments_chirho(segments_chirho);
        if stmts_chirho.len() <= 1 {
            return stmts_chirho;
        }
        let final_stmt_chirho = stmts_chirho.pop().expect("mdo statement count was checked");
        if !statements_need_recursive_knot_chirho(&stmts_chirho) {
            stmts_chirho.push(final_stmt_chirho);
            return stmts_chirho;
        }
        let mut transformed_chirho = transform_recursive_group_chirho(stmts_chirho, span_chirho);
        transformed_chirho.push(final_stmt_chirho);
        return transformed_chirho;
    }

    let mut transformed_chirho = Vec::new();
    for segment_chirho in segments_chirho {
        match segment_chirho {
            DoSegmentChirho::StatementChirho(stmt_chirho) => {
                transformed_chirho.push(stmt_chirho);
            }
            DoSegmentChirho::RecursiveChirho {
                stmts_chirho,
                span_chirho,
            } => transformed_chirho
                .extend(transform_recursive_group_chirho(stmts_chirho, span_chirho)),
        }
    }
    transformed_chirho
}

fn flatten_segments_chirho(segments_chirho: Vec<DoSegmentChirho>) -> Vec<StmtChirho> {
    let mut stmts_chirho = Vec::new();
    for segment_chirho in segments_chirho {
        match segment_chirho {
            DoSegmentChirho::StatementChirho(stmt_chirho) => stmts_chirho.push(stmt_chirho),
            DoSegmentChirho::RecursiveChirho {
                stmts_chirho: nested_stmts_chirho,
                ..
            } => stmts_chirho.extend(nested_stmts_chirho),
        }
    }
    stmts_chirho
}

fn transform_recursive_group_chirho(
    mut stmts_chirho: Vec<StmtChirho>,
    span_chirho: SpanChirho,
) -> Vec<StmtChirho> {
    let bound_names_chirho = collect_stmt_bound_names_chirho(&stmts_chirho);
    if bound_names_chirho.is_empty() {
        return stmts_chirho;
    }

    let result_expr_chirho = tuple_expr_for_names_chirho(&bound_names_chirho, span_chirho);
    stmts_chirho.push(StmtChirho::ExprChirho(app_expr_chirho(
        raw_var_chirho("return", span_chirho),
        result_expr_chirho,
        span_chirho,
    )));

    let knot_pat_chirho = tuple_pat_for_names_chirho(&bound_names_chirho, span_chirho);
    let lazy_knot_pat_chirho = if bound_names_chirho.len() == 1 {
        knot_pat_chirho.clone()
    } else {
        PatChirho::LazyChirho {
            inner_chirho: Box::new(knot_pat_chirho.clone()),
            span_chirho,
        }
    };
    let knot_body_chirho = ExprChirho::DoChirho {
        stmts_chirho,
        span_chirho,
    };
    let knot_function_chirho = ExprChirho::LamChirho {
        pats_chirho: vec![lazy_knot_pat_chirho],
        body_chirho: Box::new(knot_body_chirho),
        span_chirho,
    };
    let mfix_expr_chirho = app_expr_chirho(
        raw_var_chirho("mfix", span_chirho),
        knot_function_chirho,
        span_chirho,
    );

    vec![StmtChirho::BindChirho {
        pat_chirho: knot_pat_chirho,
        expr_chirho: mfix_expr_chirho,
        span_chirho,
    }]
}

fn collect_stmt_bound_names_chirho(stmts_chirho: &[StmtChirho]) -> Vec<NameChirho> {
    let mut names_chirho = Vec::new();
    for stmt_chirho in stmts_chirho {
        collect_one_stmt_bound_names_chirho(stmt_chirho, &mut names_chirho);
    }
    names_chirho
}

fn collect_one_stmt_bound_names_chirho(
    stmt_chirho: &StmtChirho,
    names_chirho: &mut Vec<NameChirho>,
) {
    match stmt_chirho {
        StmtChirho::BindChirho { pat_chirho, .. } => {
            collect_pat_bound_names_chirho(pat_chirho, names_chirho);
        }
        StmtChirho::LetChirho { binds_chirho, .. } => {
            for bind_chirho in binds_chirho {
                match bind_chirho {
                    LocalBindChirho::FunBindChirho { name_chirho, .. } => {
                        push_unique_name_chirho(names_chirho, name_chirho);
                    }
                    LocalBindChirho::PatBindChirho { pat_chirho, .. } => {
                        collect_pat_bound_names_chirho(pat_chirho, names_chirho);
                    }
                    LocalBindChirho::TypeSigChirho { .. } => {}
                }
            }
        }
        StmtChirho::ExprChirho(_) => {}
    }
}

/// GHC segments `mdo` blocks so statements with only backward dependencies stay sequential.
///
/// This first segmentation boundary is deliberately conservative: any reference to a name
/// bound by a later statement requires the knot, as does a monadic bind that references its
/// own binder. Ordinary `let` groups already provide their own recursive scope.
fn statements_need_recursive_knot_chirho(stmts_chirho: &[StmtChirho]) -> bool {
    let mut binding_positions_chirho = HashMap::new();
    for (stmt_index_chirho, stmt_chirho) in stmts_chirho.iter().enumerate() {
        let mut names_chirho = Vec::new();
        collect_one_stmt_bound_names_chirho(stmt_chirho, &mut names_chirho);
        for name_chirho in names_chirho {
            binding_positions_chirho
                .entry(name_chirho.text_chirho().to_string())
                .or_insert(stmt_index_chirho);
        }
    }

    for (stmt_index_chirho, stmt_chirho) in stmts_chirho.iter().enumerate() {
        let mut references_chirho = Vec::new();
        collect_stmt_references_chirho(stmt_chirho, &mut references_chirho);
        for reference_chirho in references_chirho {
            let Some(bound_index_chirho) = binding_positions_chirho.get(&reference_chirho) else {
                continue;
            };
            if *bound_index_chirho > stmt_index_chirho
                || (*bound_index_chirho == stmt_index_chirho
                    && matches!(stmt_chirho, StmtChirho::BindChirho { .. }))
            {
                return true;
            }
        }
    }
    false
}

fn collect_stmt_references_chirho(stmt_chirho: &StmtChirho, names_chirho: &mut Vec<String>) {
    match stmt_chirho {
        StmtChirho::ExprChirho(expr_chirho) | StmtChirho::BindChirho { expr_chirho, .. } => {
            collect_expr_references_chirho(expr_chirho, names_chirho);
        }
        StmtChirho::LetChirho { binds_chirho, .. } => {
            for bind_chirho in binds_chirho {
                collect_local_bind_references_chirho(bind_chirho, names_chirho);
            }
        }
    }
}

fn collect_local_bind_references_chirho(
    bind_chirho: &LocalBindChirho,
    names_chirho: &mut Vec<String>,
) {
    match bind_chirho {
        LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
            for match_chirho in matches_chirho {
                collect_rhs_references_chirho(&match_chirho.rhs_chirho, names_chirho);
                for where_bind_chirho in &match_chirho.where_binds_chirho {
                    collect_local_bind_references_chirho(where_bind_chirho, names_chirho);
                }
            }
        }
        LocalBindChirho::PatBindChirho { rhs_chirho, .. } => {
            collect_rhs_references_chirho(rhs_chirho, names_chirho);
        }
        LocalBindChirho::TypeSigChirho { .. } => {}
    }
}

fn collect_rhs_references_chirho(rhs_chirho: &RhsChirho, names_chirho: &mut Vec<String>) {
    match rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => {
            collect_expr_references_chirho(expr_chirho, names_chirho);
        }
        RhsChirho::GuardedChirho(guards_chirho) => {
            for guard_chirho in guards_chirho {
                collect_expr_references_chirho(&guard_chirho.guard_chirho, names_chirho);
                collect_expr_references_chirho(&guard_chirho.body_chirho, names_chirho);
            }
        }
    }
}

fn collect_expr_references_chirho(expr_chirho: &ExprChirho, names_chirho: &mut Vec<String>) {
    match expr_chirho {
        ExprChirho::VarChirho(name_chirho) => {
            names_chirho.push(name_chirho.text_chirho().to_string());
        }
        ExprChirho::ConChirho(_) | ExprChirho::LitChirho(_) => {}
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            collect_expr_references_chirho(fun_chirho, names_chirho);
            collect_expr_references_chirho(arg_chirho, names_chirho);
        }
        ExprChirho::TypeAppChirho { expr_chirho, .. }
        | ExprChirho::NegChirho { expr_chirho, .. }
        | ExprChirho::AnnChirho { expr_chirho, .. }
        | ExprChirho::SpliceChirho { expr_chirho, .. }
        | ExprChirho::TypedSpliceChirho { expr_chirho, .. } => {
            collect_expr_references_chirho(expr_chirho, names_chirho);
        }
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            collect_expr_references_chirho(left_chirho, names_chirho);
            names_chirho.push(op_chirho.text_chirho().to_string());
            collect_expr_references_chirho(right_chirho, names_chirho);
        }
        ExprChirho::LamChirho { body_chirho, .. }
        | ExprChirho::ParenChirho {
            inner_chirho: body_chirho,
            ..
        } => {
            collect_expr_references_chirho(body_chirho, names_chirho);
        }
        ExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for bind_chirho in binds_chirho {
                collect_local_bind_references_chirho(bind_chirho, names_chirho);
            }
            collect_expr_references_chirho(body_chirho, names_chirho);
        }
        ExprChirho::IfChirho {
            cond_chirho,
            then_chirho,
            else_chirho,
            ..
        } => {
            collect_expr_references_chirho(cond_chirho, names_chirho);
            collect_expr_references_chirho(then_chirho, names_chirho);
            collect_expr_references_chirho(else_chirho, names_chirho);
        }
        ExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            collect_expr_references_chirho(scrutinee_chirho, names_chirho);
            for alt_chirho in alts_chirho {
                collect_rhs_references_chirho(&alt_chirho.rhs_chirho, names_chirho);
                for bind_chirho in &alt_chirho.where_binds_chirho {
                    collect_local_bind_references_chirho(bind_chirho, names_chirho);
                }
            }
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => {
            for stmt_chirho in stmts_chirho {
                collect_stmt_references_chirho(stmt_chirho, names_chirho);
            }
        }
        ExprChirho::TupleChirho {
            elements_chirho, ..
        }
        | ExprChirho::ListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                collect_expr_references_chirho(element_chirho, names_chirho);
            }
        }
        ExprChirho::ArithSeqChirho {
            from_chirho,
            then_chirho,
            to_chirho,
            ..
        } => {
            collect_expr_references_chirho(from_chirho, names_chirho);
            if let Some(then_chirho) = then_chirho {
                collect_expr_references_chirho(then_chirho, names_chirho);
            }
            if let Some(to_chirho) = to_chirho {
                collect_expr_references_chirho(to_chirho, names_chirho);
            }
        }
        ExprChirho::ListCompChirho {
            body_chirho,
            quals_chirho,
            ..
        } => {
            collect_expr_references_chirho(body_chirho, names_chirho);
            for qual_chirho in quals_chirho {
                collect_stmt_references_chirho(qual_chirho, names_chirho);
            }
        }
        ExprChirho::LeftSectionChirho {
            op_chirho,
            arg_chirho,
            ..
        }
        | ExprChirho::RightSectionChirho {
            op_chirho,
            arg_chirho,
            ..
        } => {
            names_chirho.push(op_chirho.text_chirho().to_string());
            collect_expr_references_chirho(arg_chirho, names_chirho);
        }
        ExprChirho::RecordConChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                collect_expr_references_chirho(&field_chirho.value_chirho, names_chirho);
            }
        }
        ExprChirho::RecordUpdateChirho {
            expr_chirho,
            fields_chirho,
            ..
        } => {
            collect_expr_references_chirho(expr_chirho, names_chirho);
            for field_chirho in fields_chirho {
                collect_expr_references_chirho(&field_chirho.value_chirho, names_chirho);
            }
        }
        ExprChirho::QuoteExprChirho { .. }
        | ExprChirho::QuoteDeclChirho { .. }
        | ExprChirho::QuoteTypeChirho { .. }
        | ExprChirho::QuotePatChirho { .. } => {}
    }
}

fn collect_pat_bound_names_chirho(pat_chirho: &PatChirho, names_chirho: &mut Vec<NameChirho>) {
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => {
            push_unique_name_chirho(names_chirho, name_chirho);
        }
        PatChirho::AsChirho {
            name_chirho,
            pattern_chirho,
            ..
        } => {
            push_unique_name_chirho(names_chirho, name_chirho);
            collect_pat_bound_names_chirho(pattern_chirho, names_chirho);
        }
        PatChirho::ConChirho { args_chirho, .. }
        | PatChirho::TupleChirho {
            elements_chirho: args_chirho,
            ..
        }
        | PatChirho::ListChirho {
            elements_chirho: args_chirho,
            ..
        } => {
            for arg_chirho in args_chirho {
                collect_pat_bound_names_chirho(arg_chirho, names_chirho);
            }
        }
        PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::BangChirho { inner_chirho, .. } => {
            collect_pat_bound_names_chirho(inner_chirho, names_chirho)
        }
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            collect_pat_bound_names_chirho(left_chirho, names_chirho);
            collect_pat_bound_names_chirho(right_chirho, names_chirho);
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for PatFieldChirho { pattern_chirho, .. } in fields_chirho {
                collect_pat_bound_names_chirho(pattern_chirho, names_chirho);
            }
        }
        PatChirho::ViewChirho { pat_chirho, .. }
        | PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
            collect_pat_bound_names_chirho(pat_chirho, names_chirho);
        }
        PatChirho::LitChirho(_) | PatChirho::WildcardChirho(_) | PatChirho::NegChirho { .. } => {}
    }
}

fn push_unique_name_chirho(names_chirho: &mut Vec<NameChirho>, name_chirho: &NameChirho) {
    if !names_chirho
        .iter()
        .any(|existing_chirho| existing_chirho.text_chirho() == name_chirho.text_chirho())
    {
        names_chirho.push(name_chirho.clone());
    }
}

fn tuple_pat_for_names_chirho(names_chirho: &[NameChirho], span_chirho: SpanChirho) -> PatChirho {
    if let [name_chirho] = names_chirho {
        return PatChirho::VarChirho(name_chirho.clone());
    }
    PatChirho::TupleChirho {
        elements_chirho: names_chirho
            .iter()
            .cloned()
            .map(PatChirho::VarChirho)
            .collect(),
        span_chirho,
    }
}

fn tuple_expr_for_names_chirho(names_chirho: &[NameChirho], span_chirho: SpanChirho) -> ExprChirho {
    if let [name_chirho] = names_chirho {
        return ExprChirho::VarChirho(name_chirho.clone());
    }
    ExprChirho::TupleChirho {
        elements_chirho: names_chirho
            .iter()
            .cloned()
            .map(ExprChirho::VarChirho)
            .collect(),
        span_chirho,
    }
}

fn raw_var_chirho(text_chirho: &str, span_chirho: SpanChirho) -> ExprChirho {
    ExprChirho::VarChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        span_chirho,
    )))
}

fn app_expr_chirho(
    fun_chirho: ExprChirho,
    arg_chirho: ExprChirho,
    span_chirho: SpanChirho,
) -> ExprChirho {
    ExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::expr_chirho::RhsChirho;

    fn name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn bind_stmt_chirho(name_text_chirho: &str) -> StmtChirho {
        StmtChirho::BindChirho {
            pat_chirho: PatChirho::VarChirho(name_chirho(name_text_chirho)),
            expr_chirho: raw_var_chirho("actionChirho", SpanChirho::DUMMY_CHIRHO),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn explicit_rec_becomes_one_lazy_tuple_mfix_knot_chirho() {
        let transformed_chirho = transform_recursive_do_chirho(
            vec![DoSegmentChirho::RecursiveChirho {
                stmts_chirho: vec![bind_stmt_chirho("xsChirho"), bind_stmt_chirho("ysChirho")],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            false,
            SpanChirho::DUMMY_CHIRHO,
        );

        let [
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho:
                    ExprChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                        ..
                    },
                ..
            },
        ] = transformed_chirho.as_slice()
        else {
            panic!("expected one generated mfix bind: {transformed_chirho:?}");
        };
        assert!(matches!(
            pat_chirho,
            PatChirho::TupleChirho { elements_chirho, .. } if elements_chirho.len() == 2
        ));
        assert!(matches!(
            fun_chirho.as_ref(),
            ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "mfix"
        ));
        assert!(matches!(
            arg_chirho.as_ref(),
            ExprChirho::LamChirho { pats_chirho, .. }
                if matches!(
                    pats_chirho.as_slice(),
                    [PatChirho::LazyChirho { inner_chirho, .. }]
                        if matches!(
                            inner_chirho.as_ref(),
                            PatChirho::TupleChirho { elements_chirho, .. }
                                if elements_chirho.len() == 2
                        )
                )
        ));
    }

    #[test]
    fn mdo_wraps_forward_dependent_statements_before_the_final_expression_chirho() {
        let final_stmt_chirho =
            StmtChirho::ExprChirho(raw_var_chirho("finishChirho", SpanChirho::DUMMY_CHIRHO));
        let forward_bind_chirho = StmtChirho::BindChirho {
            pat_chirho: PatChirho::VarChirho(name_chirho("xChirho")),
            expr_chirho: raw_var_chirho("yChirho", SpanChirho::DUMMY_CHIRHO),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let transformed_chirho = transform_recursive_do_chirho(
            vec![
                DoSegmentChirho::StatementChirho(forward_bind_chirho),
                DoSegmentChirho::StatementChirho(bind_stmt_chirho("yChirho")),
                DoSegmentChirho::StatementChirho(final_stmt_chirho.clone()),
            ],
            true,
            SpanChirho::DUMMY_CHIRHO,
        );

        assert_eq!(transformed_chirho.len(), 2);
        assert!(matches!(
            transformed_chirho.first(),
            Some(StmtChirho::BindChirho {
                pat_chirho: PatChirho::TupleChirho { elements_chirho, .. },
                ..
            }) if elements_chirho.len() == 2
        ));
        assert_eq!(transformed_chirho.last(), Some(&final_stmt_chirho));
    }

    #[test]
    fn mdo_without_forward_dependency_stays_sequential_chirho() {
        let first_bind_chirho = bind_stmt_chirho("xChirho");
        let second_bind_chirho = StmtChirho::BindChirho {
            pat_chirho: PatChirho::VarChirho(name_chirho("yChirho")),
            expr_chirho: raw_var_chirho("xChirho", SpanChirho::DUMMY_CHIRHO),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let final_stmt_chirho =
            StmtChirho::ExprChirho(raw_var_chirho("finishChirho", SpanChirho::DUMMY_CHIRHO));
        let source_stmts_chirho = vec![first_bind_chirho, second_bind_chirho, final_stmt_chirho];
        let transformed_chirho = transform_recursive_do_chirho(
            source_stmts_chirho
                .iter()
                .cloned()
                .map(DoSegmentChirho::StatementChirho)
                .collect(),
            true,
            SpanChirho::DUMMY_CHIRHO,
        );

        assert_eq!(transformed_chirho, source_stmts_chirho);
    }

    #[test]
    fn binder_free_rec_group_stays_sequential_chirho() {
        let stmt_chirho =
            StmtChirho::ExprChirho(raw_var_chirho("effectChirho", SpanChirho::DUMMY_CHIRHO));
        assert_eq!(
            transform_recursive_do_chirho(
                vec![DoSegmentChirho::RecursiveChirho {
                    stmts_chirho: vec![stmt_chirho.clone()],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                false,
                SpanChirho::DUMMY_CHIRHO,
            ),
            vec![stmt_chirho]
        );
    }

    #[test]
    fn pattern_and_let_binders_keep_source_order_without_duplicates_chirho() {
        let tuple_pat_chirho = PatChirho::TupleChirho {
            elements_chirho: vec![
                PatChirho::VarChirho(name_chirho("leftChirho")),
                PatChirho::VarChirho(name_chirho("rightChirho")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let stmts_chirho = vec![
            StmtChirho::BindChirho {
                pat_chirho: tuple_pat_chirho,
                expr_chirho: raw_var_chirho("actionChirho", SpanChirho::DUMMY_CHIRHO),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            StmtChirho::LetChirho {
                binds_chirho: vec![LocalBindChirho::PatBindChirho {
                    pat_chirho: PatChirho::VarChirho(name_chirho("leftChirho")),
                    rhs_chirho: RhsChirho::UnguardedChirho(raw_var_chirho(
                        "valueChirho",
                        SpanChirho::DUMMY_CHIRHO,
                    )),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ];
        let names_chirho = collect_stmt_bound_names_chirho(&stmts_chirho);
        assert_eq!(
            names_chirho
                .iter()
                .map(NameChirho::text_chirho)
                .collect::<Vec<_>>(),
            vec!["leftChirho", "rightChirho"]
        );
    }
}
