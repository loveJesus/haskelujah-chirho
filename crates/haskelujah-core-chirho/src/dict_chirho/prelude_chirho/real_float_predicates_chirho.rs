// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Body-backed `RealFloat Double` predicates and representation properties.
//! They are ordinary Core built from constants, ordered Double comparisons, and
//! division every engine already implements, so all engines share one definition.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

fn var_chirho(binder_chirho: &BinderChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(binder_chirho.id_chirho)
}

fn float_chirho(value_chirho: f64) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(value_chirho))
}

fn bool_chirho(value_chirho: bool) -> CoreExprChirho {
    CoreExprChirho::ConAppChirho {
        con_name_chirho: if value_chirho { "True" } else { "False" }.to_string(),
        args_chirho: vec![],
    }
}

fn primop_chirho(name_chirho: &str, args_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: name_chirho.to_string(),
        args_chirho,
    }
}

impl DictPassCtxChirho {
    /// Generate the five `RealFloat Double` predicate rows and guarded Prelude aliases.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_real_float_predicates_chirho(&mut self) {
        let double_ty_chirho = TyChirho::double_chirho();
        let bool_ty_chirho = TyChirho::bool_chirho();

        for predicate_chirho in [
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
        ] {
            let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho.clone());
            let body_chirho = match predicate_chirho {
                "isNaN" => primop_chirho(
                    "not#",
                    vec![primop_chirho(
                        "eqFloat#",
                        vec![var_chirho(&value_chirho), var_chirho(&value_chirho)],
                    )],
                ),
                "isInfinite" => {
                    let positive_chirho = primop_chirho(
                        ">.#",
                        vec![var_chirho(&value_chirho), float_chirho(f64::MAX)],
                    );
                    let negative_chirho = primop_chirho(
                        "<.#",
                        vec![var_chirho(&value_chirho), float_chirho(-f64::MAX)],
                    );
                    self.real_float_bool_case_chirho(
                        positive_chirho,
                        bool_chirho(true),
                        negative_chirho,
                    )
                }
                "isDenormalized" => {
                    let is_zero_chirho = primop_chirho(
                        "eqFloat#",
                        vec![var_chirho(&value_chirho), float_chirho(0.0)],
                    );
                    let below_normal_chirho = primop_chirho(
                        "<.#",
                        vec![var_chirho(&value_chirho), float_chirho(f64::MIN_POSITIVE)],
                    );
                    let above_negative_normal_chirho = primop_chirho(
                        ">.#",
                        vec![var_chirho(&value_chirho), float_chirho(-f64::MIN_POSITIVE)],
                    );
                    let within_normal_bounds_chirho = self.real_float_bool_case_chirho(
                        below_normal_chirho,
                        above_negative_normal_chirho,
                        bool_chirho(false),
                    );
                    self.real_float_bool_case_chirho(
                        is_zero_chirho,
                        bool_chirho(false),
                        within_normal_bounds_chirho,
                    )
                }
                "isNegativeZero" => {
                    let is_zero_chirho = primop_chirho(
                        "eqFloat#",
                        vec![var_chirho(&value_chirho), float_chirho(0.0)],
                    );
                    let reciprocal_chirho =
                        primop_chirho("/.#", vec![float_chirho(1.0), var_chirho(&value_chirho)]);
                    let reciprocal_is_negative_chirho =
                        primop_chirho("<.#", vec![reciprocal_chirho, float_chirho(0.0)]);
                    self.real_float_bool_case_chirho(
                        is_zero_chirho,
                        reciprocal_is_negative_chirho,
                        bool_chirho(false),
                    )
                }
                "isIEEE" => bool_chirho(true),
                _ => unreachable!(),
            };
            self.push_real_float_row_chirho(
                predicate_chirho,
                bool_ty_chirho.clone(),
                value_chirho,
                body_chirho,
            );
        }
    }

    /// Generate the three constant properties of the concrete Double representation.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_real_float_properties_chirho(&mut self) {
        let double_ty_chirho = TyChirho::double_chirho();
        let int_ty_chirho = TyChirho::int_chirho();

        let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho.clone());
        self.push_real_float_row_chirho(
            "floatRadix",
            TyChirho::ConChirho("Integer".to_string()),
            value_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
        );

        let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho.clone());
        self.push_real_float_row_chirho(
            "floatDigits",
            int_ty_chirho.clone(),
            value_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(53)),
        );

        let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho);
        self.push_real_float_row_chirho(
            "floatRange",
            TyChirho::TupleChirho(vec![int_ty_chirho.clone(), int_ty_chirho]),
            value_chirho,
            CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(-1021)),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1024)),
                ],
            },
        );
    }

    fn push_real_float_row_chirho(
        &mut self,
        method_chirho: &str,
        result_ty_chirho: TyChirho,
        value_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
    ) {
        let prim_name_chirho = format!("$prim_RealFloat_{method_chirho}_Double");
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: prim_id_chirho,
                name_chirho: prim_name_chirho.clone(),
                ty_chirho: TyChirho::fun_chirho(TyChirho::double_chirho(), result_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: value_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho(method_chirho, &prim_name_chirho);
    }

    fn real_float_bool_case_chirho(
        &mut self,
        scrutinee_chirho: CoreExprChirho,
        true_chirho: CoreExprChirho,
        false_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let case_chirho =
            self.fresh_binder_chirho("$real_float_case_chirho", TyChirho::bool_chirho());
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(scrutinee_chirho),
            bind_chirho: case_chirho,
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_chirho,
                },
            ],
        }
    }
}
