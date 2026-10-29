// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The lazy arithmetic-sequence bodies the integer-represented Enum keys and Char share,
//! moved byte-identical out of `prelude_chirho.rs` (row 671). They are Int's four
//! enumeration rows, `$prim_Enum_<method>_Int`; every other key reaches them by CoreId through
//! its own row (`builtin_enum_slots_chirho.rs`). No body is bound under a source name such as
//! `enumFromTo`, so an Enum use whose dictionary is unresolved fails by name instead of
//! running Int's arithmetic.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

impl DictPassCtxChirho {
    /// Generate lazy recursive Core IR functions for arithmetic sequences.
    ///
    /// These replace the eager primops (which built entire lists synchronously)
    /// with recursive functions whose cons tails are thunked — giving true
    /// Haskell lazy evaluation for infinite lists.
    ///
    /// ```text
    /// enumFrom n       = n : enumFrom (n +# 1)
    /// enumFromThen a b = a : enumFromThen b (2*b - a)
    /// enumFromTo n to  = case n ># to of { True -> []; False -> n : enumFromTo (enumStepChirho# n 1) to }
    /// enumFromThenTo a b to = let step = b -# a in go a step to
    ///   where go n s to | s >= 0    = if n > to then [] else n : go (n+s) s to
    ///                   | otherwise = if n < to then [] else n : go (n+s) s to
    /// ```
    ///
    /// Each is bound as Int's row, `$prim_Enum_enumFrom_Int` and its three siblings (row 671).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_enum_sequence_prelude_chirho(&mut self) {
        let int_chirho = TyChirho::int_chirho();
        let list_int_chirho = TyChirho::ListChirho(Box::new(int_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };

        // Helper: build a binary primop expression
        let prim_bin_chirho =
            |op_chirho: &str, l_chirho: CoreExprChirho, r_chirho: CoreExprChirho| {
                CoreExprChirho::PrimOpChirho {
                    name_chirho: op_chirho.to_string(),
                    args_chirho: vec![l_chirho, r_chirho],
                }
            };

        // ── enumFrom :: Int -> [Int] ──────────────────────────────────
        // enumFrom n = n : enumFrom (n +# 1)
        {
            let ef_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Enum_enumFrom_Int");
            let n_chirho = self.fresh_binder_chirho("n", int_chirho.clone());

            let n_plus_1_chirho = prim_bin_chirho(
                "+#",
                CoreExprChirho::VarChirho(n_chirho.id_chirho),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
            );
            let recurse_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(ef_id_chirho)),
                arg_chirho: Box::new(n_plus_1_chirho),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    recurse_chirho,
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(cons_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ef_id_chirho,
                    name_chirho: "$prim_Enum_enumFrom_Int".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_chirho.clone(), list_int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── enumFromThen :: Int -> Int -> [Int] ──────────────────────
        // enumFromThen a b = a : enumFromThen b (2*b - a)
        {
            let eft_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Enum_enumFromThen_Int");
            let a_chirho = self.fresh_binder_chirho("a", int_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_chirho.clone());

            // 2*b - a
            let two_b_chirho = prim_bin_chirho(
                "*#",
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                CoreExprChirho::VarChirho(b_chirho.id_chirho),
            );
            let next_b_chirho = prim_bin_chirho(
                "-#",
                two_b_chirho,
                CoreExprChirho::VarChirho(a_chirho.id_chirho),
            );

            let recurse_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(eft_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(next_b_chirho),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    recurse_chirho,
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(cons_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: eft_id_chirho,
                    name_chirho: "$prim_Enum_enumFromThen_Int".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        int_chirho.clone(),
                        TyChirho::fun_chirho(int_chirho.clone(), list_int_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── enumFromTo :: Int -> Int -> [Int] ────────────────────────
        // enumFromTo n to = case n ># to of
        //   True  -> []
        //   False -> n : enumFromTo (n +# 1) to
        {
            let efto_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Enum_enumFromTo_Int");
            let n_chirho = self.fresh_binder_chirho("n", int_chirho.clone());
            let to_chirho = self.fresh_binder_chirho("to", int_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());

            let n_gt_to_chirho = prim_bin_chirho(
                ">#",
                CoreExprChirho::VarChirho(n_chirho.id_chirho),
                CoreExprChirho::VarChirho(to_chirho.id_chirho),
            );

            let n_plus_1_chirho = prim_bin_chirho(
                "enumStepChirho#",
                CoreExprChirho::VarChirho(n_chirho.id_chirho),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
            );
            let recurse_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(efto_id_chirho)),
                    arg_chirho: Box::new(n_plus_1_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(to_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    recurse_chirho,
                ],
            };

            let case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(n_gt_to_chirho),
                bind_chirho: w_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: to_chirho.clone(),
                    body_chirho: Box::new(case_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: efto_id_chirho,
                    name_chirho: "$prim_Enum_enumFromTo_Int".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        int_chirho.clone(),
                        TyChirho::fun_chirho(int_chirho.clone(), list_int_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── enumFromThenTo :: Int -> Int -> Int -> [Int] ─────────────
        // Ascending or descending based on step sign.
        // enumFromThenTo a b to =
        //   let step = b -# a in
        //   case step >=# 0 of
        //     True  -> goUp a step to      -- ascending
        //     False -> goDown a step to    -- descending
        //
        // But since we can't easily introduce a local helper, we embed the
        // logic directly: compare n to bound each step.
        //
        // enumFromThenTo a b to =
        //   let step = b -# a in
        //   case step >=# 0 of
        //     True  -> case a ># to of
        //                True -> []
        //                _    -> a : enumFromThenTo (a +# step) (a +# 2*step) to
        //     False -> case a <# to of
        //                True -> []
        //                _    -> a : enumFromThenTo (a +# step) (a +# 2*step) to
        {
            let eftt_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Enum_enumFromThenTo_Int");
            let a_chirho = self.fresh_binder_chirho("a", int_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_chirho.clone());
            let to_chirho = self.fresh_binder_chirho("to", int_chirho.clone());
            let w1_chirho = self.fresh_binder_chirho("$w1", TyChirho::bool_chirho());
            let w2_chirho = self.fresh_binder_chirho("$w2", TyChirho::bool_chirho());
            let w3_chirho = self.fresh_binder_chirho("$w3", TyChirho::bool_chirho());

            let step_chirho = prim_bin_chirho(
                "-#",
                CoreExprChirho::VarChirho(b_chirho.id_chirho),
                CoreExprChirho::VarChirho(a_chirho.id_chirho),
            );

            // a +# step = a +# (b -# a) = b
            let next_a_chirho = CoreExprChirho::VarChirho(b_chirho.id_chirho);
            // b + step preserves the current Enum representation: Int stays
            // Int, while Char stays Char in the interpreter. Native values are
            // scalar words, so every backend lowers this as integer addition.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            let next_b_chirho = prim_bin_chirho(
                "enumStepChirho#",
                CoreExprChirho::VarChirho(b_chirho.id_chirho),
                step_chirho.clone(),
            );

            let recurse_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(eftt_id_chirho)),
                        arg_chirho: Box::new(next_a_chirho.clone()),
                    }),
                    arg_chirho: Box::new(next_b_chirho.clone()),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(to_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    recurse_chirho,
                ],
            };

            // Ascending: case a ># to
            let asc_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(prim_bin_chirho(
                    ">#",
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    CoreExprChirho::VarChirho(to_chirho.id_chirho),
                )),
                bind_chirho: w2_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho.clone(),
                    },
                ],
            };

            // Descending: case a <# to
            let desc_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(prim_bin_chirho(
                    "<#",
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    CoreExprChirho::VarChirho(to_chirho.id_chirho),
                )),
                bind_chirho: w3_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            // Outer: case step >=# 0
            let step_ge_zero_chirho = prim_bin_chirho(
                ">=#",
                step_chirho,
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            );
            let outer_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(step_ge_zero_chirho),
                bind_chirho: w1_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: asc_case_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: desc_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: to_chirho.clone(),
                        body_chirho: Box::new(outer_case_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: eftt_id_chirho,
                    name_chirho: "$prim_Enum_enumFromThenTo_Int".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        int_chirho.clone(),
                        TyChirho::fun_chirho(
                            int_chirho.clone(),
                            TyChirho::fun_chirho(int_chirho.clone(), list_int_chirho.clone()),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }
}
