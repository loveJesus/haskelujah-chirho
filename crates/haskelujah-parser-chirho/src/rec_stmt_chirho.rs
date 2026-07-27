// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Frontend-only RecursiveDo statement transformation.
//!
//! The CST preserves explicit `rec` groups, but the typed AST and later phases continue to see
//! ordinary do statements. Each recursive group becomes one `mfix` knot returning its bound
//! names as a tuple. Core lowering supplies genuinely lazy tuple projections.
//!
//! workflow: recursive-do-chirho

use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, StmtChirho};
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
        match stmt_chirho {
            StmtChirho::BindChirho { pat_chirho, .. } => {
                collect_pat_bound_names_chirho(pat_chirho, &mut names_chirho);
            }
            StmtChirho::LetChirho { binds_chirho, .. } => {
                for bind_chirho in binds_chirho {
                    match bind_chirho {
                        LocalBindChirho::FunBindChirho { name_chirho, .. } => {
                            push_unique_name_chirho(&mut names_chirho, name_chirho);
                        }
                        LocalBindChirho::PatBindChirho { pat_chirho, .. } => {
                            collect_pat_bound_names_chirho(pat_chirho, &mut names_chirho);
                        }
                        LocalBindChirho::TypeSigChirho { .. } => {}
                    }
                }
            }
            StmtChirho::ExprChirho(_) => {}
        }
    }
    names_chirho
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

        let [StmtChirho::BindChirho {
            pat_chirho,
            expr_chirho:
                ExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                },
            ..
        }] = transformed_chirho.as_slice()
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
    fn mdo_wraps_every_statement_before_the_final_expression_chirho() {
        let final_stmt_chirho =
            StmtChirho::ExprChirho(raw_var_chirho("finishChirho", SpanChirho::DUMMY_CHIRHO));
        let transformed_chirho = transform_recursive_do_chirho(
            vec![
                DoSegmentChirho::StatementChirho(bind_stmt_chirho("xChirho")),
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
