// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Body-backed operations over `Ordering`. The four `Ordering -> Bool` tests serve
//! the compare route (row 551). The `Enum Ordering` rows preserve GHC's constructor
//! order and its endpoint failures (row 602). The two `Bounded Ordering` rows
//! expose LT and GT as GHC's finite endpoints (row 605).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

/// Each ordering primop the desugarer emits, its test binding, the constructor the
/// test decides on, and whether that constructor answers True.
pub(in crate::dict_chirho) const ORDERING_TESTS_CHIRHO: [(&str, &str, &str, bool); 4] = [
    ("<#", "$ordering_lt_chirho", "LT", true),
    ("<=#", "$ordering_le_chirho", "GT", false),
    (">#", "$ordering_gt_chirho", "GT", true),
    (">=#", "$ordering_ge_chirho", "LT", false),
];

fn bool_chirho(value_chirho: bool) -> CoreExprChirho {
    CoreExprChirho::ConAppChirho {
        con_name_chirho: if value_chirho { "True" } else { "False" }.to_string(),
        args_chirho: vec![],
    }
}

fn ordering_chirho(name_chirho: &str) -> CoreExprChirho {
    CoreExprChirho::ConAppChirho {
        con_name_chirho: name_chirho.to_string(),
        args_chirho: vec![],
    }
}

fn int_chirho(value_chirho: i64) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(value_chirho))
}

fn error_chirho(message_chirho: &str) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: "error".to_string(),
        args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
            message_chirho.to_string(),
        ))],
    }
}

impl DictPassCtxChirho {
    /// Generate the four Ordering tests: `\o -> case o of { C -> v; _ -> not v }`.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_ordering_tests_prelude_chirho(&mut self) {
        let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());
        let bool_ty_chirho = TyChirho::ConChirho("Bool".to_string());
        for (_, name_chirho, con_chirho, accepts_chirho) in ORDERING_TESTS_CHIRHO {
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let ordering_chirho = self.fresh_binder_chirho("o", ordering_ty_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", ordering_ty_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ordering_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: bool_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_chirho.to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: bool_chirho(accepts_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: bool_chirho(!accepts_chirho),
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![ordering_ty_chirho.clone()],
                        bool_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: ordering_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate the four body-backed `Enum Ordering` methods.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_ordering_enum_rows_chirho(&mut self) {
        let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());
        let int_ty_chirho = TyChirho::int_chirho();

        let tag_chirho = self.fresh_binder_chirho("tag", int_ty_chirho.clone());
        let tag_case_chirho = self.fresh_binder_chirho("$ordering_enum_tag", int_ty_chirho.clone());
        let to_enum_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(tag_chirho.id_chirho)),
            bind_chirho: tag_case_chirho,
            result_ty_chirho: ordering_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(0)),
                    binders_chirho: vec![],
                    rhs_chirho: ordering_chirho("LT"),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                    binders_chirho: vec![],
                    rhs_chirho: ordering_chirho("EQ"),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(2)),
                    binders_chirho: vec![],
                    rhs_chirho: ordering_chirho("GT"),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: error_chirho("Prelude.Enum.Ordering.toEnum: bad argument"),
                },
            ],
        };
        self.push_ordering_enum_row_chirho(
            "$prim_Enum_toEnum_Ordering",
            TyChirho::fun_chirho(int_ty_chirho.clone(), ordering_ty_chirho.clone()),
            tag_chirho,
            to_enum_chirho,
        );

        let ordering_value_chirho =
            self.fresh_binder_chirho("ordering", ordering_ty_chirho.clone());
        let from_enum_chirho = self.ordering_case_chirho(
            &ordering_value_chirho,
            int_ty_chirho.clone(),
            [int_chirho(0), int_chirho(1), int_chirho(2)],
        );
        self.push_ordering_enum_row_chirho(
            "$prim_Enum_fromEnum_Ordering",
            TyChirho::fun_chirho(ordering_ty_chirho.clone(), int_ty_chirho),
            ordering_value_chirho,
            from_enum_chirho,
        );

        let ordering_value_chirho =
            self.fresh_binder_chirho("ordering", ordering_ty_chirho.clone());
        let succ_chirho = self.ordering_case_chirho(
            &ordering_value_chirho,
            ordering_ty_chirho.clone(),
            [
                ordering_chirho("EQ"),
                ordering_chirho("GT"),
                error_chirho("Prelude.Enum.Ordering.succ: bad argument"),
            ],
        );
        self.push_ordering_enum_row_chirho(
            "$prim_Enum_succ_Ordering",
            TyChirho::fun_chirho(ordering_ty_chirho.clone(), ordering_ty_chirho.clone()),
            ordering_value_chirho,
            succ_chirho,
        );

        let ordering_value_chirho =
            self.fresh_binder_chirho("ordering", ordering_ty_chirho.clone());
        let pred_chirho = self.ordering_case_chirho(
            &ordering_value_chirho,
            ordering_ty_chirho.clone(),
            [
                error_chirho("Prelude.Enum.Ordering.pred: bad argument"),
                ordering_chirho("LT"),
                ordering_chirho("EQ"),
            ],
        );
        self.push_ordering_enum_row_chirho(
            "$prim_Enum_pred_Ordering",
            TyChirho::fun_chirho(ordering_ty_chirho.clone(), ordering_ty_chirho),
            ordering_value_chirho,
            pred_chirho,
        );
    }

    /// Generate the two body-backed `Bounded Ordering` methods.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_ordering_bounded_rows_chirho(&mut self) {
        let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());
        for (name_chirho, constructor_chirho) in [
            ("$prim_Bounded_minBound_Ordering", "LT"),
            ("$prim_Bounded_maxBound_Ordering", "GT"),
        ] {
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: ordering_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: ordering_chirho(constructor_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    fn ordering_case_chirho(
        &mut self,
        value_chirho: &BinderChirho,
        result_ty_chirho: TyChirho,
        results_chirho: [CoreExprChirho; 3],
    ) -> CoreExprChirho {
        let case_chirho =
            self.fresh_binder_chirho("$ordering_enum_case", value_chirho.ty_chirho.clone());
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(value_chirho.id_chirho)),
            bind_chirho: case_chirho,
            result_ty_chirho,
            alts_chirho: ["LT", "EQ", "GT"]
                .into_iter()
                .zip(results_chirho)
                .map(|(constructor_chirho, rhs_chirho)| CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(constructor_chirho.to_string()),
                    binders_chirho: vec![],
                    rhs_chirho,
                })
                .collect(),
        }
    }

    fn push_ordering_enum_row_chirho(
        &mut self,
        name_chirho: &str,
        ty_chirho: TyChirho,
        parameter_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
    ) {
        let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
