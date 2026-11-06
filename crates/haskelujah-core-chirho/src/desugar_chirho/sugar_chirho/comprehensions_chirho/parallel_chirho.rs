// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Parallel branches transport lexical Core IDs, with lockstep truncation.
//! No copied qualifier AST or user-spelled map/zip mediates these rows.
//! Workflow: language-features-chirho/parallel-list-comprehensions-chirho.
use super::*;

impl DesugarCtxChirho {
    pub(in crate::desugar_chirho) fn desugar_parallel_list_comp_chirho(
        &mut self,
        body_chirho: &ExprChirho,
        branches_chirho: &[Vec<ComprehensionQualifierChirho>],
        _span_chirho: SpanChirho,
    ) -> CoreExprChirho {
        let columns_chirho: Vec<_> = branches_chirho
            .iter()
            .map(|branch_chirho| self.comprehension_columns_chirho(branch_chirho))
            .collect();
        let mut streams_chirho =
            branches_chirho
                .iter()
                .zip(&columns_chirho)
                .map(|(branch_chirho, names_chirho)| {
                    self.desugar_comprehension_result_chirho(
                        branch_chirho,
                        &mut |context_chirho| {
                            context_chirho.comprehension_pack_chirho(names_chirho)
                        },
                        Self::comprehension_nil_chirho(),
                    )
                });
        let Some(first_chirho) = streams_chirho.next() else {
            return self.desugar_list_comp_chirho(body_chirho, &[]);
        };
        let streams_chirho: Vec<_> = streams_chirho.collect();
        let mut zipped_chirho = first_chirho;
        for stream_chirho in streams_chirho {
            zipped_chirho = self.comprehension_zip_chirho(zipped_chirho, stream_chirho);
        }
        self.comprehension_rows_chirho(
            zipped_chirho,
            Self::comprehension_nil_chirho(),
            &mut |context_chirho, row_chirho, tail_chirho| {
                context_chirho.comprehension_parallel_unpack_chirho(
                    &columns_chirho,
                    row_chirho,
                    &mut |context_chirho| CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            context_chirho.desugar_expr_chirho(body_chirho),
                            tail_chirho.clone(),
                        ],
                    },
                )
            },
        )
    }

    fn comprehension_parallel_unpack_chirho(
        &mut self,
        columns_chirho: &[Vec<String>],
        row_chirho: CoreIdChirho,
        result_chirho: &mut dyn FnMut(&mut Self) -> CoreExprChirho,
    ) -> CoreExprChirho {
        let Some((last_chirho, prefix_chirho)) = columns_chirho.split_last() else {
            return result_chirho(self);
        };
        if prefix_chirho.is_empty() {
            return self.comprehension_unpack_chirho(last_chirho, row_chirho, result_chirho);
        }
        let left_chirho = self.comprehension_binder_chirho("$parallel_left_chirho");
        let right_chirho = self.comprehension_binder_chirho("$parallel_right_chirho");
        let wildcard_chirho = self.comprehension_binder_chirho("$parallel_unpack_chirho");
        let body_chirho = self.comprehension_parallel_unpack_chirho(
            prefix_chirho,
            left_chirho.id_chirho,
            &mut |context_chirho| {
                context_chirho.comprehension_unpack_chirho(
                    last_chirho,
                    right_chirho.id_chirho,
                    result_chirho,
                )
            },
        );
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(row_chirho)),
            result_ty_chirho: wildcard_chirho.ty_chirho.clone(),
            bind_chirho: wildcard_chirho,
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                binders_chirho: vec![left_chirho, right_chirho],
                rhs_chirho: body_chirho,
            }],
        }
    }

    fn comprehension_zip_chirho(
        &mut self,
        left_stream_chirho: CoreExprChirho,
        right_stream_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let loop_chirho = self.comprehension_binder_chirho("$parallel_zip_chirho");
        let left_chirho = self.comprehension_binder_chirho("$parallel_xs_chirho");
        let right_chirho = self.comprehension_binder_chirho("$parallel_ys_chirho");
        let left_head_chirho = self.comprehension_binder_chirho("$parallel_x_chirho");
        let left_tail_chirho = self.comprehension_binder_chirho("$parallel_xt_chirho");
        let right_head_chirho = self.comprehension_binder_chirho("$parallel_y_chirho");
        let right_tail_chirho = self.comprehension_binder_chirho("$parallel_yt_chirho");
        let left_case_chirho = self.comprehension_binder_chirho("$parallel_xcase_chirho");
        let right_case_chirho = self.comprehension_binder_chirho("$parallel_ycase_chirho");
        let recursive_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(loop_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(left_tail_chirho.id_chirho)),
            }),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(right_tail_chirho.id_chirho)),
        };
        let paired_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: ":".to_string(),
            args_chirho: vec![
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: "$tuple2".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(left_head_chirho.id_chirho),
                        CoreExprChirho::VarChirho(right_head_chirho.id_chirho),
                    ],
                },
                recursive_chirho,
            ],
        };
        let right_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(right_chirho.id_chirho)),
            result_ty_chirho: right_case_chirho.ty_chirho.clone(),
            bind_chirho: right_case_chirho,
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".into()),
                    binders_chirho: vec![],
                    rhs_chirho: Self::comprehension_nil_chirho(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".into()),
                    binders_chirho: vec![right_head_chirho, right_tail_chirho],
                    rhs_chirho: paired_chirho,
                },
            ],
        };
        let left_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(left_chirho.id_chirho)),
            result_ty_chirho: left_case_chirho.ty_chirho.clone(),
            bind_chirho: left_case_chirho,
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".into()),
                    binders_chirho: vec![],
                    rhs_chirho: Self::comprehension_nil_chirho(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".into()),
                    binders_chirho: vec![left_head_chirho, left_tail_chirho],
                    rhs_chirho: right_body_chirho,
                },
            ],
        };
        CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![(
                loop_chirho.clone(),
                CoreExprChirho::LamChirho {
                    binder_chirho: left_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: right_chirho,
                        body_chirho: Box::new(left_body_chirho),
                    }),
                },
            )],
            body_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(loop_chirho.id_chirho)),
                    arg_chirho: Box::new(left_stream_chirho),
                }),
                arg_chirho: Box::new(right_stream_chirho),
            }),
        }
    }
}
