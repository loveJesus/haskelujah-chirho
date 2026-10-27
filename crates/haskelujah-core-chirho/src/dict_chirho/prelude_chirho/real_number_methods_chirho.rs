// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Body-backed `RealFrac Double` and `RealFloat Double` methods and properties.
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
    /// Generate `properFraction :: Double -> (Int, Double)` behind a guarded
    /// Prelude alias. The checker retains the standard polymorphic scheme; the
    /// concrete generated row serves the Double/Int representation this compiler
    /// can execute today.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_real_frac_proper_fraction_chirho(&mut self) {
        let double_ty_chirho = TyChirho::double_chirho();
        let int_ty_chirho = TyChirho::int_chirho();
        let result_ty_chirho =
            TyChirho::TupleChirho(vec![int_ty_chirho.clone(), double_ty_chirho.clone()]);
        let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho.clone());

        let zero_pair_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "$tuple2".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                float_chirho(0.0),
            ],
        };
        let integral_chirho =
            self.fresh_binder_chirho("$proper_fraction_integral_chirho", int_ty_chirho.clone());
        let truncated_chirho = primop_chirho("truncate#", vec![var_chirho(&value_chirho)]);
        let regular_pair_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(integral_chirho.clone(), truncated_chirho.clone())],
            body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    var_chirho(&integral_chirho),
                    primop_chirho(
                        "-.#",
                        vec![
                            var_chirho(&value_chirho),
                            primop_chirho("fromIntegral#", vec![var_chirho(&integral_chirho)]),
                        ],
                    ),
                ],
            }),
        };
        let out_of_range_pair_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "$tuple2".to_string(),
            args_chirho: vec![truncated_chirho, float_chirho(0.0)],
        };

        let zero_or_regular_chirho = self.real_float_case_chirho(
            primop_chirho(
                "eqFloat#",
                vec![var_chirho(&value_chirho), float_chirho(0.0)],
            ),
            result_ty_chirho.clone(),
            zero_pair_chirho(),
            regular_pair_chirho,
        );
        let below_int_chirho = self.real_float_case_chirho(
            primop_chirho(
                "<.#",
                vec![var_chirho(&value_chirho), float_chirho(i64::MIN as f64)],
            ),
            result_ty_chirho.clone(),
            out_of_range_pair_chirho.clone(),
            zero_or_regular_chirho,
        );
        let int_range_chirho = self.real_float_case_chirho(
            primop_chirho(
                ">=.#",
                vec![var_chirho(&value_chirho), float_chirho(-(i64::MIN as f64))],
            ),
            result_ty_chirho.clone(),
            out_of_range_pair_chirho,
            below_int_chirho,
        );
        let not_negative_infinity_chirho = self.real_float_case_chirho(
            primop_chirho(
                "<.#",
                vec![var_chirho(&value_chirho), float_chirho(-f64::MAX)],
            ),
            result_ty_chirho.clone(),
            zero_pair_chirho(),
            int_range_chirho,
        );
        let finite_chirho = self.real_float_case_chirho(
            primop_chirho(
                ">.#",
                vec![var_chirho(&value_chirho), float_chirho(f64::MAX)],
            ),
            result_ty_chirho.clone(),
            zero_pair_chirho(),
            not_negative_infinity_chirho,
        );
        let body_chirho = self.real_float_case_chirho(
            primop_chirho(
                "eqFloat#",
                vec![var_chirho(&value_chirho), var_chirho(&value_chirho)],
            ),
            result_ty_chirho.clone(),
            finite_chirho,
            zero_pair_chirho(),
        );

        let prim_name_chirho = "$prim_RealFrac_properFraction_Double_Int";
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: prim_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_chirho(double_ty_chirho, result_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: value_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho("properFraction", prim_name_chirho);
    }

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

    /// Generate `exponent` and `significand` for the concrete Double representation
    /// through the RTS routines every engine shares (row 585).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_real_float_components_chirho(&mut self) {
        let double_ty_chirho = TyChirho::double_chirho();
        let int_ty_chirho = TyChirho::int_chirho();

        for (method_chirho, primop_name_chirho, result_ty_chirho) in [
            ("exponent", "exponentFloatChirho#", int_ty_chirho.clone()),
            (
                "significand",
                "significandFloatChirho#",
                double_ty_chirho.clone(),
            ),
        ] {
            let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho.clone());
            let body_chirho = primop_chirho(primop_name_chirho, vec![var_chirho(&value_chirho)]);
            self.push_real_float_row_chirho(
                method_chirho,
                result_ty_chirho,
                value_chirho,
                body_chirho,
            );
        }
    }

    /// Generate `scaleFloat :: Int -> Double -> Double` with its two argument
    /// types explicit. The rewrite traversal keeps the exponent at Int (row 587).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_real_float_scale_chirho(&mut self) {
        let int_ty_chirho = TyChirho::int_chirho();
        let double_ty_chirho = TyChirho::double_chirho();
        let exponent_chirho = self.fresh_binder_chirho("exponent_chirho", int_ty_chirho.clone());
        let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho.clone());
        let prim_name_chirho = "$prim_RealFloat_scaleFloat_Double";
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        let body_chirho = primop_chirho(
            "scaleFloatChirho#",
            vec![var_chirho(&exponent_chirho), var_chirho(&value_chirho)],
        );
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: prim_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    [int_ty_chirho, double_ty_chirho.clone()],
                    double_ty_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: exponent_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: value_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho("scaleFloat", prim_name_chirho);
    }

    /// Generate the concrete `Double` codec rows from the representation
    /// operations shared by all engines. The compiler currently represents
    /// `Integer` in one machine word; `decodeFloat`'s 53-bit mantissa is exact,
    /// while `encodeFloat` follows that existing representable-domain boundary.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_real_float_codec_chirho(&mut self) {
        let integer_ty_chirho = TyChirho::ConChirho("Integer".to_string());
        let int_ty_chirho = TyChirho::int_chirho();
        let double_ty_chirho = TyChirho::double_chirho();
        let decoded_ty_chirho =
            TyChirho::TupleChirho(vec![integer_ty_chirho.clone(), int_ty_chirho.clone()]);

        let value_chirho = self.fresh_binder_chirho("value_chirho", double_ty_chirho.clone());
        let mantissa_chirho =
            self.fresh_binder_chirho("$decoded_mantissa_chirho", integer_ty_chirho.clone());
        let scaled_significand_chirho = primop_chirho(
            "scaleFloatChirho#",
            vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(53)),
                primop_chirho("significandFloatChirho#", vec![var_chirho(&value_chirho)]),
            ],
        );
        let decoded_nonzero_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                mantissa_chirho.clone(),
                primop_chirho("truncate#", vec![scaled_significand_chirho]),
            )],
            body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    var_chirho(&mantissa_chirho),
                    primop_chirho(
                        "-#",
                        vec![
                            primop_chirho("exponentFloatChirho#", vec![var_chirho(&value_chirho)]),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(53)),
                        ],
                    ),
                ],
            }),
        };
        let decoded_zero_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "$tuple2".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            ],
        };
        let decode_body_chirho = self.real_float_case_chirho(
            primop_chirho(
                "eqFloat#",
                vec![var_chirho(&value_chirho), float_chirho(0.0)],
            ),
            decoded_ty_chirho.clone(),
            decoded_zero_chirho,
            decoded_nonzero_chirho,
        );
        let decode_name_chirho = "$prim_RealFloat_decodeFloat_Double";
        let decode_id_chirho = self.resolve_or_fresh_id_chirho(decode_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: decode_id_chirho,
                name_chirho: decode_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_chirho(double_ty_chirho.clone(), decoded_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: value_chirho,
                body_chirho: Box::new(decode_body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho("decodeFloat", decode_name_chirho);

        let encoded_mantissa_chirho =
            self.fresh_binder_chirho("mantissa_chirho", integer_ty_chirho.clone());
        let encoded_exponent_chirho =
            self.fresh_binder_chirho("exponent_chirho", int_ty_chirho.clone());
        let encode_name_chirho = "$prim_RealFloat_encodeFloat_Double";
        let encode_id_chirho = self.resolve_or_fresh_id_chirho(encode_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: encode_id_chirho,
                name_chirho: encode_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    [integer_ty_chirho, int_ty_chirho],
                    double_ty_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: encoded_mantissa_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: encoded_exponent_chirho.clone(),
                    body_chirho: Box::new(primop_chirho(
                        "scaleFloatChirho#",
                        vec![
                            var_chirho(&encoded_exponent_chirho),
                            primop_chirho(
                                "fromIntegral#",
                                vec![var_chirho(&encoded_mantissa_chirho)],
                            ),
                        ],
                    )),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho("encodeFloat", encode_name_chirho);
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
        self.real_float_case_chirho(
            scrutinee_chirho,
            TyChirho::bool_chirho(),
            true_chirho,
            false_chirho,
        )
    }

    fn real_float_case_chirho(
        &mut self,
        scrutinee_chirho: CoreExprChirho,
        result_ty_chirho: TyChirho,
        true_chirho: CoreExprChirho,
        false_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let case_chirho =
            self.fresh_binder_chirho("$real_float_case_chirho", TyChirho::bool_chirho());
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(scrutinee_chirho),
            bind_chirho: case_chirho,
            result_ty_chirho,
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

#[cfg(test)]
mod tests_chirho {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn proper_fraction_has_one_concrete_row_and_guarded_alias_chirho() {
        let mut context_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
        context_chirho.generate_real_frac_proper_fraction_chirho();

        let [row_chirho] = context_chirho
            .generated_bindings_chirho
            .iter()
            .filter(|binding_chirho| {
                binding_chirho.binder_chirho.name_chirho
                    == "$prim_RealFrac_properFraction_Double_Int"
            })
            .collect::<Vec<_>>()[..]
        else {
            panic!("one properFraction row");
        };
        assert_eq!(
            row_chirho.binder_chirho.ty_chirho,
            TyChirho::fun_chirho(
                TyChirho::double_chirho(),
                TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::double_chirho()]),
            )
        );
        assert!(matches!(
            row_chirho.rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));

        let [alias_chirho] = context_chirho
            .generated_bindings_chirho
            .iter()
            .filter(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "properFraction")
            .collect::<Vec<_>>()[..]
        else {
            panic!("one properFraction alias");
        };
        assert!(matches!(
            alias_chirho.rhs_chirho,
            CoreExprChirho::VarChirho(id_chirho)
                if id_chirho == row_chirho.binder_chirho.id_chirho
        ));
    }

    #[test]
    fn real_float_codec_has_two_concrete_rows_and_guarded_aliases_chirho() {
        let mut context_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
        context_chirho.generate_real_float_codec_chirho();

        let decode_name_chirho = "$prim_RealFloat_decodeFloat_Double";
        let encode_name_chirho = "$prim_RealFloat_encodeFloat_Double";
        let decode_chirho = context_chirho
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == decode_name_chirho)
            .expect("decodeFloat row");
        let encode_chirho = context_chirho
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == encode_name_chirho)
            .expect("encodeFloat row");

        assert_eq!(
            decode_chirho.binder_chirho.ty_chirho,
            TyChirho::fun_chirho(
                TyChirho::double_chirho(),
                TyChirho::TupleChirho(vec![
                    TyChirho::ConChirho("Integer".to_string()),
                    TyChirho::int_chirho(),
                ]),
            )
        );
        assert_eq!(
            encode_chirho.binder_chirho.ty_chirho,
            TyChirho::fun_n_chirho(
                [
                    TyChirho::ConChirho("Integer".to_string()),
                    TyChirho::int_chirho(),
                ],
                TyChirho::double_chirho(),
            )
        );
        assert!(matches!(
            decode_chirho.rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
        assert!(matches!(
            encode_chirho.rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));

        for (alias_name_chirho, row_chirho) in [
            ("decodeFloat", decode_chirho),
            ("encodeFloat", encode_chirho),
        ] {
            let alias_chirho = context_chirho
                .generated_bindings_chirho
                .iter()
                .find(|binding_chirho| {
                    binding_chirho.binder_chirho.name_chirho == alias_name_chirho
                })
                .expect("codec alias");
            assert!(matches!(
                alias_chirho.rhs_chirho,
                CoreExprChirho::VarChirho(id_chirho)
                    if id_chirho == row_chirho.binder_chirho.id_chirho
            ));
        }
    }
}
