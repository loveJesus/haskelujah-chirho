// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Case alternatives share their failure continuations rather than repeatedly
//! expanding remaining pattern rows. Workflow: testing-chirho/execution-oracles-chirho.

use super::{
    AltConChirho, CoreAltChirho, CoreExprChirho, CoreLitChirho, DesugarCtxChirho, SpanChirho,
    TyChirho,
};
use haskelujah_ast_chirho::expr_chirho::AltChirho;
use haskelujah_typing_chirho::ty_chirho::TyVarChirho;

impl DesugarCtxChirho {
    pub(super) fn desugar_case_expr_with_scrutinee_chirho(
        &mut self,
        scrut_chirho: CoreExprChirho,
        alts_chirho: &[haskelujah_ast_chirho::expr_chirho::AltChirho],
    ) -> CoreExprChirho {
        if alts_chirho.is_empty() {
            let error_id_chirho = self.fresh_id_chirho("error");
            let msg_chirho = CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                "Non-exhaustive case alternatives".to_string(),
            ));
            return CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(error_id_chirho)),
                arg_chirho: Box::new(msg_chirho),
            };
        }

        let wild_chirho = self.fresh_binder_chirho(
            "wild",
            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                self.next_id_chirho,
            )),
            SpanChirho::DUMMY_CHIRHO,
        );

        if self.should_desugar_string_case_as_eq_chain_chirho(alts_chirho) {
            let scrut_binder_chirho = self.fresh_binder_chirho(
                "str_case_scrut",
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                    self.next_id_chirho,
                )),
                SpanChirho::DUMMY_CHIRHO,
            );
            let body_chirho = self
                .desugar_string_case_eq_chain_chirho(scrut_binder_chirho.id_chirho, alts_chirho);
            return CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(scrut_binder_chirho, scrut_chirho)],
                body_chirho: Box::new(body_chirho),
            };
        }

        // A failure below the outer constructor must try the remaining source
        // alternatives. Re-desugaring every suffix here grows exponentially.
        if alts_chirho.iter().any(|alt_chirho| {
            let expanded_chirho = self.expand_record_wildcard_pat_chirho(&alt_chirho.pat_chirho);
            Self::pat_nested_cases_can_use_fallback_chirho(
                expanded_chirho.as_ref().unwrap_or(&alt_chirho.pat_chirho),
            )
        }) {
            return self.desugar_nested_case_chain_chirho(scrut_chirho, alts_chirho);
        }

        let mut core_alts_chirho = Vec::with_capacity(alts_chirho.len());
        for alt_chirho in alts_chirho {
            let expanded_pat_chirho =
                self.expand_record_wildcard_pat_chirho(&alt_chirho.pat_chirho);
            let pat_ref_chirho = expanded_pat_chirho
                .as_ref()
                .unwrap_or(&alt_chirho.pat_chirho);
            let con_chirho = self.pat_to_alt_con_chirho(pat_ref_chirho);
            let binders_chirho = self.pat_to_binders_chirho(pat_ref_chirho);
            let rhs_final_chirho = self.desugar_case_alt_rhs_with_scrutinee_chirho(
                alt_chirho,
                pat_ref_chirho,
                &binders_chirho,
                wild_chirho.id_chirho,
                None,
            );
            core_alts_chirho.push(CoreAltChirho {
                con_chirho,
                binders_chirho,
                rhs_chirho: rhs_final_chirho,
            });
        }

        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(scrut_chirho),
            bind_chirho: wild_chirho,
            result_ty_chirho: TyChirho::VarChirho(
                haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
            ),
            alts_chirho: core_alts_chirho,
        }
    }

    /// Compile each alternative once. Failures refer to a shared lazy suffix,
    /// not a cloned tree. RHS desugaring stays in source order because method
    /// evidence joins occurrence ids by their source-order ordinal.
    fn desugar_nested_case_chain_chirho(
        &mut self,
        scrutinee_chirho: CoreExprChirho,
        alternatives_chirho: &[AltChirho],
    ) -> CoreExprChirho {
        let scrutinee_binder_chirho = self.fresh_binder_chirho(
            "match_scrutinee_chirho",
            TyChirho::VarChirho(TyVarChirho(self.next_id_chirho)),
            SpanChirho::DUMMY_CHIRHO,
        );
        let result_ty_chirho = TyChirho::VarChirho(TyVarChirho(self.next_id_chirho));
        let continuations_chirho: Vec<_> = (0..=alternatives_chirho.len())
            .map(|_| {
                self.fresh_binder_chirho(
                    "match_next_chirho",
                    result_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                )
            })
            .collect();
        let mut bindings_chirho = Vec::with_capacity(continuations_chirho.len());
        for (index_chirho, alternative_chirho) in alternatives_chirho.iter().enumerate() {
            let expanded_chirho =
                self.expand_record_wildcard_pat_chirho(&alternative_chirho.pat_chirho);
            let pattern_chirho = expanded_chirho
                .as_ref()
                .unwrap_or(&alternative_chirho.pat_chirho);
            let constructor_chirho = self.pat_to_alt_con_chirho(pattern_chirho);
            let binders_chirho = self.pat_to_binders_chirho(pattern_chirho);
            let next_chirho =
                CoreExprChirho::VarChirho(continuations_chirho[index_chirho + 1].id_chirho);
            let rhs_chirho = self.desugar_case_alt_rhs_with_scrutinee_chirho(
                alternative_chirho,
                pattern_chirho,
                &binders_chirho,
                scrutinee_binder_chirho.id_chirho,
                Some(&next_chirho),
            );
            let row_chirho = if constructor_chirho == AltConChirho::DefaultChirho {
                rhs_chirho
            } else {
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        scrutinee_binder_chirho.id_chirho,
                    )),
                    bind_chirho: self.fresh_binder_chirho(
                        "match_wild_chirho",
                        scrutinee_binder_chirho.ty_chirho.clone(),
                        SpanChirho::DUMMY_CHIRHO,
                    ),
                    result_ty_chirho: result_ty_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: constructor_chirho,
                            binders_chirho,
                            rhs_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: next_chirho,
                        },
                    ],
                }
            };
            bindings_chirho.push((continuations_chirho[index_chirho].clone(), row_chirho));
        }
        let failure_chirho = self.desugar_case_expr_with_scrutinee_chirho(
            CoreExprChirho::VarChirho(scrutinee_binder_chirho.id_chirho),
            &[],
        );
        bindings_chirho.push((
            continuations_chirho
                .last()
                .expect("failure continuation")
                .clone(),
            failure_chirho,
        ));
        let mut body_chirho = CoreExprChirho::VarChirho(continuations_chirho[0].id_chirho);
        // Each suffix is outside the row that refers to it. Singleton non-recursive
        // lets make lexical dependencies explicit to every subsequent pass.
        for binding_chirho in bindings_chirho {
            body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![binding_chirho],
                body_chirho: Box::new(body_chirho),
            };
        }
        CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(scrutinee_binder_chirho, scrutinee_chirho)],
            body_chirho: Box::new(body_chirho),
        }
    }
}
