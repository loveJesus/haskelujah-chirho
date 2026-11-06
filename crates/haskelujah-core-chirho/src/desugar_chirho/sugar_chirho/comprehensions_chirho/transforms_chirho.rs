// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Transform the complete prefix stream and restore its columns for the suffix.
//! User expressions remain borrowed AST nodes; generated list loops do not
//! resolve source-spelled `map` or `concatMap` bindings.
//! Workflow: language-features-chirho/parallel-list-comprehensions-chirho.

use super::*;

impl DesugarCtxChirho {
    pub(super) fn desugar_comprehension_transform_chirho(
        &mut self,
        result_chirho: &mut dyn FnMut(&mut Self) -> CoreExprChirho,
        qualifiers_chirho: &[ComprehensionQualifierChirho],
        index_chirho: usize,
        continuation_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let ComprehensionQualifierChirho::TransformChirho(transform_chirho) =
            &qualifiers_chirho[index_chirho]
        else {
            unreachable!("selected transform qualifier")
        };
        let using_chirho = transform_chirho
            .using_chirho
            .as_ref()
            .expect("public Core input preflight rejects missing transformer");
        // The transformer belongs to the comprehension's OUTER scope. In
        // particular, a preceding generator with its name cannot capture it.
        let mut function_chirho = self.desugar_expr_chirho(using_chirho);
        let names_chirho = self.comprehension_columns_chirho(&qualifiers_chirho[..index_chirho]);

        if let Some(key_chirho) = &transform_chirho.by_chirho {
            self.push_scope_chirho();
            let binder_chirho = self.comprehension_binder_chirho("$transform_key_row_chirho");
            let key_core_chirho = self.comprehension_unpack_chirho(
                &names_chirho,
                binder_chirho.id_chirho,
                &mut |context_chirho| context_chirho.desugar_expr_chirho(key_chirho),
            );
            self.pop_scope_chirho();
            function_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(function_chirho),
                arg_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho,
                    body_chirho: Box::new(key_core_chirho),
                }),
            };
        }

        // Capture actual lexical IDs in the prefix match/let success scope.
        // No user reference is reconstructed through a raw spelling.
        let prefix_chirho = self.desugar_comprehension_result_chirho(
            &qualifiers_chirho[..index_chirho],
            &mut |context_chirho| context_chirho.comprehension_pack_chirho(&names_chirho),
            Self::comprehension_nil_chirho(),
        );
        let transformed_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(function_chirho),
            arg_chirho: Box::new(prefix_chirho),
        };
        self.comprehension_rows_chirho(
            transformed_chirho,
            continuation_chirho,
            &mut |context_chirho, row_chirho, tail_chirho| {
                if transform_chirho.group_chirho {
                    // Each prefix column becomes its own list. Mapping the
                    // whole tuple into every name would disconnect columns.
                    let mut bindings_chirho = Vec::with_capacity(names_chirho.len());
                    for name_chirho in &names_chirho {
                        let column_chirho = context_chirho.comprehension_rows_chirho(
                            CoreExprChirho::VarChirho(row_chirho),
                            Self::comprehension_nil_chirho(),
                            &mut |context_chirho, element_chirho, tail_chirho| {
                                context_chirho.comprehension_unpack_chirho(
                                    &names_chirho,
                                    element_chirho,
                                    &mut |context_chirho| {
                                        let id_chirho = context_chirho
                                            .lookup_scope_chirho(name_chirho)
                                            .expect("packed prefix column is bound by its pattern");
                                        CoreExprChirho::ConAppChirho {
                                            con_name_chirho: ":".to_string(),
                                            args_chirho: vec![
                                                CoreExprChirho::VarChirho(id_chirho),
                                                tail_chirho.clone(),
                                            ],
                                        }
                                    },
                                )
                            },
                        );
                        bindings_chirho.push((name_chirho, column_chirho));
                    }
                    context_chirho.push_scope_chirho();
                    let bindings_chirho = bindings_chirho
                        .into_iter()
                        .map(|(name_chirho, column_chirho)| {
                            let binder_chirho =
                                context_chirho.comprehension_binder_chirho(name_chirho);
                            context_chirho
                                .bind_in_scope_chirho(name_chirho, binder_chirho.id_chirho);
                            (binder_chirho, column_chirho)
                        })
                        .collect();
                    let suffix_chirho = context_chirho.desugar_comprehension_result_chirho(
                        &qualifiers_chirho[index_chirho + 1..],
                        result_chirho,
                        tail_chirho,
                    );
                    context_chirho.pop_scope_chirho();
                    CoreExprChirho::LetChirho {
                        rec_chirho: false,
                        binds_chirho: bindings_chirho,
                        body_chirho: Box::new(suffix_chirho),
                    }
                } else {
                    context_chirho.comprehension_unpack_chirho(
                        &names_chirho,
                        row_chirho,
                        &mut |context_chirho| {
                            context_chirho.desugar_comprehension_result_chirho(
                                &qualifiers_chirho[index_chirho + 1..],
                                result_chirho,
                                tail_chirho.clone(),
                            )
                        },
                    )
                }
            },
        )
    }

    pub(crate) fn comprehension_columns_chirho(
        &self,
        qualifiers_chirho: &[ComprehensionQualifierChirho],
    ) -> Vec<String> {
        let mut columns_chirho = Vec::new();
        let mut seen_chirho = HashSet::new();
        for qualifier_chirho in qualifiers_chirho {
            let ComprehensionQualifierChirho::StatementChirho(statement_chirho) = qualifier_chirho
            else {
                continue;
            };
            let names_chirho = match statement_chirho.as_ref() {
                haskelujah_ast_chirho::expr_chirho::StmtChirho::BindChirho {
                    pat_chirho, ..
                } => self.pattern_variables_chirho(pat_chirho),
                haskelujah_ast_chirho::expr_chirho::StmtChirho::LetChirho {
                    binds_chirho, ..
                } => binds_chirho
                    .iter()
                    .flat_map(|binding_chirho| match binding_chirho {
                        LocalBindChirho::FunBindChirho { name_chirho, .. } => {
                            vec![name_chirho.text_chirho().to_string()]
                        }
                        LocalBindChirho::PatBindChirho { pat_chirho, .. } => {
                            self.pattern_variables_chirho(pat_chirho)
                        }
                        LocalBindChirho::TypeSigChirho { .. } => vec![],
                    })
                    .collect(),
                haskelujah_ast_chirho::expr_chirho::StmtChirho::ExprChirho { .. } => vec![],
            };
            for name_chirho in names_chirho {
                if seen_chirho.insert(name_chirho.clone()) {
                    columns_chirho.push(name_chirho);
                }
            }
        }
        columns_chirho
    }

    pub(super) fn comprehension_pack_chirho(&self, names_chirho: &[String]) -> CoreExprChirho {
        let columns_chirho: Vec<_> = names_chirho
            .iter()
            .map(|name_chirho| {
                CoreExprChirho::VarChirho(
                    self.lookup_scope_chirho(name_chirho)
                        .expect("prefix column is bound in its match/let success scope"),
                )
            })
            .collect();
        match columns_chirho.as_slice() {
            [column_chirho] => column_chirho.clone(),
            _ => CoreExprChirho::ConAppChirho {
                con_name_chirho: format!("$tuple{}", columns_chirho.len()),
                args_chirho: columns_chirho,
            },
        }
    }

    /// Bind a packed row directly through existing Core IDs. Tuple matching
    /// introduces only these columns, never a spelling-resolved user value.
    pub(super) fn comprehension_unpack_chirho(
        &mut self,
        names_chirho: &[String],
        row_chirho: CoreIdChirho,
        result_chirho: &mut dyn FnMut(&mut Self) -> CoreExprChirho,
    ) -> CoreExprChirho {
        self.push_scope_chirho();
        if let [name_chirho] = names_chirho {
            self.bind_in_scope_chirho(name_chirho, row_chirho);
            let result_chirho = result_chirho(self);
            self.pop_scope_chirho();
            return result_chirho;
        }
        let binders_chirho: Vec<_> = names_chirho
            .iter()
            .map(|name_chirho| {
                let binder_chirho = self.comprehension_binder_chirho(name_chirho);
                self.bind_in_scope_chirho(name_chirho, binder_chirho.id_chirho);
                binder_chirho
            })
            .collect();
        let body_chirho = result_chirho(self);
        let wildcard_chirho = self.comprehension_binder_chirho("$transform_unpack_chirho");
        self.pop_scope_chirho();
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(row_chirho)),
            result_ty_chirho: wildcard_chirho.ty_chirho.clone(),
            bind_chirho: wildcard_chirho,
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(format!("$tuple{}", names_chirho.len())),
                binders_chirho,
                rhs_chirho: body_chirho,
            }],
        }
    }

    pub(super) fn comprehension_nil_chirho() -> CoreExprChirho {
        CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        }
    }

    pub(super) fn comprehension_binder_chirho(&mut self, name_chirho: &str) -> BinderChirho {
        self.fresh_binder_chirho(
            name_chirho,
            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                self.next_id_chirho,
            )),
            SpanChirho::DUMMY_CHIRHO,
        )
    }

    /// Inline concatMap with its exhaustion continuation. This is the same
    /// list/constructor vocabulary used by ordinary comprehension lowering.
    pub(super) fn comprehension_rows_chirho(
        &mut self,
        stream_chirho: CoreExprChirho,
        continuation_chirho: CoreExprChirho,
        row_chirho: &mut dyn FnMut(&mut Self, CoreIdChirho, CoreExprChirho) -> CoreExprChirho,
    ) -> CoreExprChirho {
        self.push_scope_chirho();
        let loop_chirho = self.comprehension_binder_chirho("$transform_loop_chirho");
        let list_chirho = self.comprehension_binder_chirho("$transform_list_chirho");
        let wildcard_chirho = self.comprehension_binder_chirho("$transform_case_chirho");
        let element_chirho = self.comprehension_binder_chirho("$transform_element_chirho");
        let tail_chirho = self.comprehension_binder_chirho("$transform_tail_chirho");
        let recursive_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(loop_chirho.id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(tail_chirho.id_chirho)),
        };
        let result_chirho = row_chirho(self, element_chirho.id_chirho, recursive_chirho);
        let loop_body_chirho = CoreExprChirho::LamChirho {
            binder_chirho: list_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(list_chirho.id_chirho)),
                bind_chirho: wildcard_chirho,
                result_ty_chirho: loop_chirho.ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: continuation_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![element_chirho, tail_chirho],
                        rhs_chirho: result_chirho,
                    },
                ],
            }),
        };
        self.pop_scope_chirho();
        CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![(loop_chirho.clone(), loop_body_chirho)],
            body_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(loop_chirho.id_chirho)),
                arg_chirho: Box::new(stream_chirho),
            }),
        }
    }
}
