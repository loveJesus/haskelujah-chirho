// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The concrete `Double ^^ Int` helper selected by the checker’s two predicate proofs
//! (row 577). GHC defines `x ^^ n` as `x ^ n` for a nonnegative exponent and
//! `recip (x ^ negate n)` otherwise. Row 570’s `^.#` already follows GHC.Real’s exact
//! repeated-squaring order, so this helper adds only the sign branch and the evidenced
//! `Fractional./` row (`recip x` is `1 / x`; the native backends intentionally have no
//! standalone `recip#` lowering).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

/// The helper a `(^^)` occurrence proved at `Fractional Double, Integral Int` becomes,
/// after application to the body-backed `Fractional./ Double` row.
pub(in crate::dict_chirho) const FRACTIONAL_POWER_DOUBLE_INT_CHIRHO: &str =
    "$fractional_power_Double_Int_chirho";

fn var_chirho(binder_chirho: &BinderChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(binder_chirho.id_chirho)
}

impl DictPassCtxChirho {
    /// Generate `\divide x n -> if n < 0 then 1 / (x ^ negate n) else x ^ n`.
    /// The occurrence rewrite supplies the concrete division function and selects this
    /// helper only when typing proved both of `(^^)`'s predicates.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_fractional_power_prelude_chirho(&mut self) {
        let helper_id_chirho = self.resolve_or_fresh_id_chirho(FRACTIONAL_POWER_DOUBLE_INT_CHIRHO);
        let double_ty_chirho = TyChirho::ConChirho("Double".to_string());
        let int_ty_chirho = TyChirho::int_chirho();
        let divide_chirho = self.fresh_binder_chirho(
            "divide_chirho",
            TyChirho::fun_n_chirho(
                vec![double_ty_chirho.clone(), double_ty_chirho.clone()],
                double_ty_chirho.clone(),
            ),
        );
        let base_chirho = self.fresh_binder_chirho("base_chirho", double_ty_chirho.clone());
        let exponent_chirho = self.fresh_binder_chirho("exponent_chirho", int_ty_chirho.clone());
        let comparison_chirho =
            self.fresh_binder_chirho("$comparison_chirho", TyChirho::bool_chirho());

        let power_chirho = |exponent_expr_chirho: CoreExprChirho| CoreExprChirho::PrimOpChirho {
            name_chirho: "^.#".to_string(),
            args_chirho: vec![var_chirho(&base_chirho), exponent_expr_chirho],
        };
        let negative_power_chirho = power_chirho(CoreExprChirho::PrimOpChirho {
            name_chirho: "negate#".to_string(),
            args_chirho: vec![var_chirho(&exponent_chirho)],
        });
        let reciprocal_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(var_chirho(&divide_chirho)),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(1.0))),
            }),
            arg_chirho: Box::new(negative_power_chirho),
        };
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "<#".to_string(),
                args_chirho: vec![
                    var_chirho(&exponent_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            }),
            bind_chirho: comparison_chirho,
            result_ty_chirho: double_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: reciprocal_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("False".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: power_chirho(var_chirho(&exponent_chirho)),
                },
            ],
        };
        let rhs_chirho = [divide_chirho, base_chirho, exponent_chirho]
            .into_iter()
            .rev()
            .fold(body_chirho, |body_chirho, binder_chirho| {
                CoreExprChirho::LamChirho {
                    binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            });
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: helper_id_chirho,
                name_chirho: FRACTIONAL_POWER_DOUBLE_INT_CHIRHO.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![double_ty_chirho.clone(), double_ty_chirho.clone()],
                            double_ty_chirho.clone(),
                        ),
                        double_ty_chirho.clone(),
                        int_ty_chirho,
                    ],
                    double_ty_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
