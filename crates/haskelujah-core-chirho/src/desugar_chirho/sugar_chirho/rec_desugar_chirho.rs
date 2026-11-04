// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lazy tuple projection lowering used by RecursiveDo's generated `mfix` lambda.
//!
//! A lazy tuple pattern cannot become a strict Core case around the lambda body. Each named
//! field is instead a non-recursive let-bound selector thunk. The selector case runs only when
//! that field is demanded, allowing `mfix` to tie the tuple knot.
//!
//! workflow: recursive-do-chirho

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use crate::expr_chirho::{AltConChirho, CoreAltChirho, CoreExprChirho, CoreIdChirho};

use super::DesugarCtxChirho;

impl DesugarCtxChirho {
    /// Lower `\ ~(x, y, ...) -> body` to lazy tuple selectors.
    ///
    /// Returns `None` for every other pattern shape so the established general lambda
    /// desugarer remains authoritative outside this narrow irrefutable-tuple case.
    pub(super) fn try_desugar_lazy_tuple_lambda_chirho(
        &mut self,
        pats_chirho: &[PatChirho],
        body_chirho: &ExprChirho,
    ) -> Option<CoreExprChirho> {
        let [PatChirho::LazyChirho { inner_chirho, .. }] = pats_chirho else {
            return None;
        };
        let PatChirho::TupleChirho {
            elements_chirho, ..
        } = inner_chirho.as_ref()
        else {
            return None;
        };

        let mut field_names_chirho = Vec::with_capacity(elements_chirho.len());
        for element_chirho in elements_chirho {
            field_names_chirho.push(lazy_field_name_chirho(element_chirho)?);
        }

        self.push_scope_chirho();
        let tuple_binder_chirho = self.fresh_binder_chirho(
            "_recursive_tuple_chirho",
            self.fresh_lazy_projection_ty_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );
        let mut projection_specs_chirho = Vec::new();
        for (field_index_chirho, field_name_chirho) in field_names_chirho.into_iter().enumerate() {
            let Some(field_name_chirho) = field_name_chirho else {
                continue;
            };
            let projection_binder_chirho = self.fresh_binder_chirho(
                &field_name_chirho,
                self.fresh_lazy_projection_ty_chirho(),
                SpanChirho::DUMMY_CHIRHO,
            );
            self.bind_in_scope_chirho(&field_name_chirho, projection_binder_chirho.id_chirho);
            projection_specs_chirho.push((field_index_chirho, projection_binder_chirho));
        }

        let body_core_chirho = self.desugar_expr_chirho(body_chirho);
        let arity_chirho = elements_chirho.len();
        let projection_binds_chirho = projection_specs_chirho
            .into_iter()
            .map(|(field_index_chirho, projection_binder_chirho)| {
                let projection_rhs_chirho = self.lazy_tuple_projection_chirho(
                    tuple_binder_chirho.id_chirho,
                    arity_chirho,
                    field_index_chirho,
                );
                (projection_binder_chirho, projection_rhs_chirho)
            })
            .collect::<Vec<_>>();
        self.pop_scope_chirho();

        let projected_body_chirho = if projection_binds_chirho.is_empty() {
            body_core_chirho
        } else {
            CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: projection_binds_chirho,
                body_chirho: Box::new(body_core_chirho),
            }
        };
        Some(CoreExprChirho::LamChirho {
            binder_chirho: tuple_binder_chirho,
            body_chirho: Box::new(projected_body_chirho),
        })
    }

    fn lazy_tuple_projection_chirho(
        &mut self,
        tuple_id_chirho: CoreIdChirho,
        arity_chirho: usize,
        selected_index_chirho: usize,
    ) -> CoreExprChirho {
        let field_binders_chirho = (0..arity_chirho)
            .map(|field_index_chirho| {
                self.fresh_binder_chirho(
                    &format!("_recursive_field_{field_index_chirho}_chirho"),
                    self.fresh_lazy_projection_ty_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                )
            })
            .collect::<Vec<_>>();
        let selected_id_chirho = field_binders_chirho[selected_index_chirho].id_chirho;
        let case_binder_chirho = self.fresh_binder_chirho(
            "_recursive_tuple_case_chirho",
            self.fresh_lazy_projection_ty_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );

        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(tuple_id_chirho)),
            bind_chirho: case_binder_chirho,
            result_ty_chirho: self.fresh_lazy_projection_ty_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(format!("$tuple{arity_chirho}")),
                binders_chirho: field_binders_chirho,
                rhs_chirho: CoreExprChirho::VarChirho(selected_id_chirho),
            }],
        }
    }

    fn fresh_lazy_projection_ty_chirho(&self) -> TyChirho {
        TyChirho::VarChirho(TyVarChirho(self.next_id_chirho))
    }
}

fn lazy_field_name_chirho(pat_chirho: &PatChirho) -> Option<Option<String>> {
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => Some(Some(name_chirho.text_chirho().to_string())),
        PatChirho::WildcardChirho(_) => Some(None),
        PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. } => lazy_field_name_chirho(inner_chirho),
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => lazy_field_name_chirho(pat_chirho),
        _ => None,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn variable_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn lazy_tuple_lambda_uses_delayed_projection_lets_chirho() {
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::LazyChirho {
                inner_chirho: Box::new(PatChirho::TupleChirho {
                    elements_chirho: vec![
                        PatChirho::VarChirho(variable_name_chirho("leftChirho")),
                        PatChirho::VarChirho(variable_name_chirho("rightChirho")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(ExprChirho::TupleChirho {
                elements_chirho: vec![
                    ExprChirho::VarChirho(variable_name_chirho("leftChirho")),
                    ExprChirho::VarChirho(variable_name_chirho("rightChirho")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

        let CoreExprChirho::LamChirho {
            binder_chirho: tuple_binder_chirho,
            body_chirho,
        } = core_chirho
        else {
            panic!("expected one tuple lambda");
        };
        let CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho,
            body_chirho: projected_body_chirho,
        } = body_chirho.as_ref()
        else {
            panic!("expected lazy selector lets under the lambda");
        };
        assert_eq!(binds_chirho.len(), 2);
        for (field_index_chirho, (_, selector_chirho)) in binds_chirho.iter().enumerate() {
            assert!(matches!(
                selector_chirho,
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho,
                    alts_chirho,
                    ..
                } if matches!(
                    scrutinee_chirho.as_ref(),
                    CoreExprChirho::VarChirho(id_chirho)
                        if *id_chirho == tuple_binder_chirho.id_chirho
                ) && matches!(
                    alts_chirho.as_slice(),
                    [CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_name_chirho),
                        binders_chirho,
                        rhs_chirho: CoreExprChirho::VarChirho(selected_id_chirho),
                    }] if con_name_chirho == "$tuple2"
                        && binders_chirho[field_index_chirho].id_chirho == *selected_id_chirho
                )
            ));
        }
        assert!(matches!(
            projected_body_chirho.as_ref(),
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } if con_name_chirho == "$tuple2" && args_chirho.len() == 2
        ));
    }
}
